use anyhow::{Context as _, Result};
use log::{LevelFilter, Log, Metadata, Record};
use music::{LyricsProvider as _, LyricsQuery, MusicApi as _, TrackKey};
use spotify::{AuthConfig, LibrespotClient, SpotifyLyrics, auth};
use std::{
    fs,
    time::{Duration, Instant},
};
const TRACK: &str = "4cOdK2wGLETKBW3PvgPWqT";
const ARTIST: &str = "0gxyHStUsqpMadRV0Di1Qt";
const ALBUM: &str = "4aawyAB9vmqN3uQ7FjRGTy";
const PLAYLIST: &str = "37i9dQZF1DWY4lFlS4Pnso";
const POP: &str = "0JQ5DAqbMKFEC4WFtoNRpw";
const MADE: &str = "0JQ5DAt0tbjZptfcdMSKl3";
struct Requests;
impl Log for Requests {
    fn enabled(&self, meta: &Metadata) -> bool {
        meta.target() == "librespot_core::http_client"
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = record.args().to_string();
            if let Some(url) = message.strip_prefix("Requesting ") {
                println!(
                    "HTTP {}",
                    url.split('?')
                        .next()
                        .unwrap_or("")
                        .split('#')
                        .next()
                        .unwrap_or("")
                );
            } else if message.contains("retrying in") {
                println!("HTTP_RETRY");
            }
        }
    }
    fn flush(&self) {}
}
static LOG: Requests = Requests;
fn bytes() -> (u64, u64) {
    let path = std::env::var_os("SONORA_PROFILE_NETWORK").unwrap();
    let data = fs::read(path).unwrap();
    (
        u64::from_ne_bytes(data[..8].try_into().unwrap()),
        u64::from_ne_bytes(data[8..16].try_into().unwrap()),
    )
}
fn end(label: &str, start: Instant, (rx, tx): (u64, u64), result: Result<usize>) {
    let (a, b) = bytes();
    match result {
        Ok(n) => println!(
            "END {label} {} {} {} {n}",
            start.elapsed().as_millis(),
            a - rx,
            b - tx
        ),
        Err(_) => println!(
            "FAIL {label} {} {} {}",
            start.elapsed().as_millis(),
            a - rx,
            b - tx
        ),
    }
}
macro_rules! workload {
    ($label:literal, $expr:expr) => {{
        println!("START {}", $label);
        let start = Instant::now();
        let traffic = bytes();
        let result: Result<usize> = async { $expr }.await;
        end($label, start, traffic, result);
    }};
}
async fn search(client: &LibrespotClient, query: &str) -> Result<usize> {
    let found = client.search_all(query).await;
    Ok(found.tracks?.len() + found.albums?.len() + found.artists?.len() + found.playlists?.len())
}
#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();
    log::set_logger(&LOG).ok();
    log::set_max_level(LevelFilter::Debug);
    println!("START sign_in");
    let start = Instant::now();
    let traffic = bytes();
    let session = auth::restore(&AuthConfig::from_env())
        .await?
        .context("no stored Spotify sign-in")?;
    let client = LibrespotClient::new(session);
    end("sign_in", start, traffic, Ok(1));
    println!("START own_profile");
    let start = Instant::now();
    let traffic = bytes();
    let profile = client.profile().await?;
    end("own_profile", start, traffic, Ok(1));
    workload!("home", client.home().await.map(|x|x.sections.iter().map(|s|s.items.len()).sum::<usize>()+x.listen_again.len()));
    workload!("search", search(&client, "Rick Astley").await);
    workload!(
        "playlist",
        client.playlist(PLAYLIST).await.map(|x| x.tracks.len())
    );
    workload!("album", client.album(ALBUM).await.map(|x| x.tracks.len()));
    workload!(
        "artist",
        client
            .artist(ARTIST)
            .await
            .map(|x| x.top_tracks.len() + x.albums.len())
    );
    workload!("browse", browse(&client).await);
    workload!(
        "genre",
        client
            .genre(POP)
            .await
            .map(|x| x.sections.iter().map(|s| s.items.len()).sum())
    );
    workload!(
        "made_for_you",
        client
            .genre(MADE)
            .await
            .map(|x| x.sections.iter().map(|s| s.items.len()).sum())
    );
    workload!(
        "track",
        tokio::try_join!(client.track(TRACK), client.track_playcount(TRACK)).map(|_| 1)
    );
    workload!(
        "artist_catalogue",
        client
            .artist_catalogue(ARTIST, &[])
            .await
            .map(|x| x.albums.len())
    );
    workload!(
        "artist_tracks",
        client.artist_tracks(ARTIST).await.map(|x| x.len())
    );
    workload!(
        "artist_profile",
        client.artist_profile(ARTIST).await.map(|_| 1)
    );
    workload!(
        "artist_images",
        client
            .artist_images(vec![ARTIST.into(), "6eUKZXaKkcviH0Ku9w2n3V".into()])
            .await
            .map(|x| x.len())
    );
    workload!(
        "album_catalogue",
        client
            .album_catalogue(ALBUM, Some("0TnOYISbd1XYRBk9myaseg"))
            .await
            .map(|x| x.also_like.len() + x.similar.len())
    );
    workload!(
        "album_tracks",
        client.album_tracks(ALBUM).await.map(|x| x.len())
    );
    workload!(
        "playlist_tracks",
        client.playlist_tracks(PLAYLIST).await.map(|x| x.len())
    );
    println!("START library_playlists");
    let start = Instant::now();
    let traffic = bytes();
    let library = client.playlists().await?;
    end("library_playlists", start, traffic, Ok(library.len()));
    let summary_targets: std::collections::HashSet<_> = library
        .iter()
        .filter(|playlist| playlist.owned)
        .take(4)
        .map(|playlist| playlist.id.as_str())
        .collect();
    workload!(
        "playlist_covers",
        client.playlist_covers(PLAYLIST, 4).await.map(|x| x.len())
    );
    workload!(
        "playlist_membership",
        client.playlist_tracks(PLAYLIST).await.map(|x| x
            .into_iter()
            .filter_map(|t| t.id)
            .collect::<std::collections::HashSet<_>>()
            .len())
    );
    workload!(
        "playlist_summary_batch",
        client.playlists().await.map(|rows| rows.iter()
            .filter(|playlist| summary_targets.contains(playlist.id.as_str())).count())
    );
    workload!("saved_collection", {
        tokio::try_join!(client.saved_tracks(), client.saved_albums())
            .map(|(tracks, albums)| tracks.len() + albums.len())
    });
    workload!(
        "saved_artists",
        client.saved_artists().await.map(|x| x.len())
    );
    workload!(
        "pins",
        client.pin_targets().await.map(|x| x.map_or(0, |p| p.len()))
    );
    workload!(
        "public_profile",
        client.user("spotify").await.map(|x| x.playlists.len())
    );
    workload!(
        "account_profile",
        client.user(&profile.id).await.map(|x| x.playlists.len())
    );
    workload!(
        "track_radio",
        client.track_radio(TRACK, None).await.map(|x| x.0.len())
    );
    let lyrics = SpotifyLyrics::from_env();
    let query = LyricsQuery {
        title: "Never Gonna Give You Up".into(),
        artist: "Rick Astley".into(),
        duration: Duration::from_secs(213),
        ..Default::default()
    };
    let query_with_id = LyricsQuery {
        track: Some(TrackKey {
            provider: "spotify".into(),
            id: TRACK.into(),
        }),
        ..query.clone()
    };
    workload!(
        "lyrics_id",
        lyrics.search(&query_with_id).await.map(|x| x.len())
    );
    workload!("lyrics_text", lyrics.search(&query).await.map(|x| x.len()));
    workload!("home_revisit",client.home().await.map(|x|x.sections.iter().map(|s|s.items.len()).sum::<usize>()+x.listen_again.len()));
    Ok(())
}

async fn browse(client: &LibrespotClient) -> Result<usize> {
    Ok(client.genres().await?.len())
}
