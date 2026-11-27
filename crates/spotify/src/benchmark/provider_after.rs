use std::{
    fs,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result};
use music::{LyricsProvider as _, LyricsQuery, MusicApi as _, TrackKey};
use spotify::{AuthConfig, SpotifyClient, SpotifyLyrics, auth};
use spotifyapis::{
    Session,
    access_point::AudioKeys,
    dealer::{Dealer, Event},
    id::raw_id,
    proto::extended_metadata::{ExtensionKind, Track},
};
use std::sync::Arc;

const TRACK: &str = "4cOdK2wGLETKBW3PvgPWqT";
const AUDIO_TRACK: &str = "4PTG3Z6ehGkBFwjybzWkR8";
const ARTIST: &str = "0gxyHStUsqpMadRV0Di1Qt";
const ALBUM: &str = "4aawyAB9vmqN3uQ7FjRGTy";
const PLAYLIST: &str = "37i9dQZF1DWY4lFlS4Pnso";
const POP: &str = "0JQ5DAqbMKFEC4WFtoNRpw";
const MADE: &str = "0JQ5DAt0tbjZptfcdMSKl3";

struct Measure {
    label: &'static str,
    start: Instant,
    traffic: (u64, u64),
}

impl Measure {
    fn start(label: &'static str) -> Self {
        println!("START {label}");
        Self {
            label,
            start: Instant::now(),
            traffic: bytes(),
        }
    }

    fn first(&self) {
        println!("FIRST {} {}", self.label, self.start.elapsed().as_millis());
    }

    fn finish(self, result: Result<usize>) {
        let (down, up) = bytes();
        let ms = self.start.elapsed().as_millis();
        let (down, up) = (down - self.traffic.0, up - self.traffic.1);
        match result {
            Ok(n) => println!("END {} {ms} {down} {up} {n}", self.label),
            Err(_) => println!("FAIL {} {ms} {down} {up}", self.label),
        }
    }
}

fn bytes() -> (u64, u64) {
    let data = fs::read(std::env::var_os("SONORA_PROFILE_NETWORK").unwrap()).unwrap();
    (
        u64::from_ne_bytes(data[..8].try_into().unwrap()),
        u64::from_ne_bytes(data[8..16].try_into().unwrap()),
    )
}

macro_rules! measure {
    ($label:literal, $expr:expr) => {{
        let measure = Measure::start($label);
        measure.finish(async { $expr }.await);
    }};
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let sign_in = Measure::start("sign_in");
    let session = auth::restore(&AuthConfig::from_env())
        .await?
        .context("no stored Spotify sign-in")?;
    let client = SpotifyClient::new(session);
    let (api, device) = client.benchmark_session();
    sign_in.finish(Ok(1));
    let profile_read = Measure::start("own_profile");
    let profile = client.profile().await?;
    profile_read.finish(Ok(1));

    let home = Measure::start("home");
    let mut pages = client.home_paged().await?;
    let mut count = None;
    while let Some(page) = pages.recv().await {
        let feed = page?;
        if count.is_none() {
            home.first();
        }
        count = Some(
            feed.sections.iter().map(|s| s.items.len()).sum::<usize>() + feed.listen_again.len(),
        );
    }
    home.finish(count.context("no Home feed"));
    measure!("search", {
        let result = client.search_all("Rick Astley").await;
        Ok(result.tracks?.len()
            + result.albums?.len()
            + result.playlists?.len()
            + result.artists?.len())
    });
    let landing = Measure::start("browse");
    let genres = client.genres().await;
    let titles: std::collections::HashMap<_, _> = genres
        .iter()
        .flatten()
        .map(|genre| (genre.id.clone(), genre.name.clone()))
        .collect();
    landing.finish(genres.map(|x| x.len()));
    // A category opens from its Browse card, which hands over its title, and shows a preview
    // first, as the app does.
    for (label, id) in [("genre", POP), ("made_for_you", MADE)] {
        let read = Measure::start(label);
        let (preview, mut previews) = tokio::sync::mpsc::channel(1);
        let (result, ()) = tokio::join!(
            client.genre_with_preview(id, titles.get(id).map(String::as_str), preview),
            async {
                if previews.recv().await.is_some() {
                    read.first();
                }
            }
        );
        read.finish(result.map(|x| x.sections.iter().map(|s| s.items.len()).sum()));
    }
    measure!("home_revisit", client.home().await.map(|x| x.sections.iter().map(|s|s.items.len()).sum::<usize>() + x.listen_again.len()));

    measure!(
        "playlist",
        client.playlist(PLAYLIST).await.map(|x| x.tracks.len())
    );
    measure!(
        "playlist_covers",
        client.playlist_covers(PLAYLIST, 4).await.map(|x| x.len())
    );
    measure!(
        "playlist_membership",
        Ok(client
            .playlist_memberships(&[(PLAYLIST.into(), 4)])
            .await
            .pop()
            .context("missing playlist result")??
            .tracks
            .len())
    );
    measure!("playlist_tracks", client.playlist_tracks(PLAYLIST).await.map(|x| x.len()));
    let library_read = Measure::start("library_playlists");
    let library = client.playlists().await?;
    library_read.finish(Ok(library.len()));
    let summary_targets: Vec<_> = library
        .iter()
        .filter(|playlist| playlist.owned)
        .take(4)
        .map(|playlist| playlist.id.clone())
        .collect();
    measure!(
        "playlist_summary_batch",
        Ok(client
            .playlist_snapshots(summary_targets)
            .await
            .context("summary reads unavailable")?
            .len())
    );
    measure!(
        "pins",
        client
            .pin_targets()
            .await
            .map(|x| x.map_or(0, |pins| pins.len()))
    );
    let mixed = Measure::start("saved_collection");
    let pages = client
        .saved_collection_paged()
        .await
        .context("mixed collection unavailable")?;
    let (tracks, albums) = tokio::try_join!(
        count_pages(pages.tracks, Some(&mixed)),
        count_pages(pages.albums, None)
    )?;
    mixed.finish(Ok(tracks + albums));
    measure!(
        "saved_artists",
        client.saved_artists().await.map(|x| x.len())
    );

    measure!("album", client.album(ALBUM).await.map(|x| x.tracks.len()));
    measure!("album_tracks", client.album_tracks(ALBUM).await.map(|x| x.len()));
    measure!(
        "album_catalogue",
        client
            .album_catalogue(ALBUM, Some("0TnOYISbd1XYRBk9myaseg"))
            .await
            .map(|x| x.also_like.len() + x.similar.len())
    );
    measure!(
        "artist",
        client
            .artist(ARTIST)
            .await
            .map(|x| x.top_tracks.len() + x.albums.len())
    );
    measure!(
        "artist_catalogue",
        client
            .artist_catalogue(ARTIST, &[])
            .await
            .map(|x| x.albums.len())
    );
    measure!(
        "artist_tracks",
        client.artist_tracks(ARTIST).await.map(|x| x.len())
    );
    measure!(
        "artist_profile",
        client.artist_profile(ARTIST).await.map(|_| 1)
    );
    measure!(
        "artist_images",
        client
            .artist_images(vec![ARTIST.into(), "6eUKZXaKkcviH0Ku9w2n3V".into()])
            .await
            .map(|x| x.len())
    );
    let track_read = Measure::start("track");
    let track = client.track(TRACK).await?;
    track_read.finish(Ok(1));
    measure!("artwork", {
        let cover = track.cover.context("track has no artwork")?;
        println!("HTTP artwork");
        let response = spotifyapis::media_client()?
            .get(cover)
            .send()
            .await?
            .error_for_status()?;
        Ok(response.bytes().await?.len())
    });
    measure!(
        "public_profile",
        client.user("spotify").await.map(|x| x.playlists.len())
    );
    measure!(
        "account_profile",
        client.user(&profile.id).await.map(|x| x.playlists.len())
    );
    measure!(
        "track_radio",
        client.track_radio(TRACK, None).await.map(|x| x.0.len())
    );
    let context_id = library
        .iter()
        .max_by_key(|item| item.track_count)
        .map_or(PLAYLIST, |item| item.id.as_str());
    measure!(
        "playlist_context",
        api.resolve_sequence(&format!("spotify:playlist:{context_id}"), device)
            .await
            .map(|x| x.tracks.len())
    );
    measure!(
        "collection_context",
        api.resolve_sequence(&format!("spotify:user:{}:collection", profile.id), device)
            .await
            .map(|x| x.tracks.len())
    );
    measure!("short_link", {
        let fixture = std::env::var("SONORA_BENCHMARK_SHORT_LINK");
        let link = fixture
            .as_deref()
            .unwrap_or("https://spotify.link/sonora-benchmark-missing");
        let result = music::link::follow(link)
            .await
            .context("short link request failed")?;
        let resolved = result.address.starts_with("https://open.spotify.com/");
        anyhow::ensure!(
            fixture.is_err() || resolved,
            "short link did not resolve to Spotify"
        );
        Ok(usize::from(resolved))
    });

    let lyrics = SpotifyLyrics::from_env();
    let query = LyricsQuery {
        title: "Never Gonna Give You Up".into(),
        artist: "Rick Astley".into(),
        duration: Duration::from_secs(213),
        ..Default::default()
    };
    let identified = LyricsQuery {
        track: Some(TrackKey {
            provider: "spotify".into(),
            id: TRACK.into(),
        }),
        ..query.clone()
    };
    measure!(
        "lyrics_id",
        lyrics.search(&identified).await.map(|x| x.len())
    );
    measure!("lyrics_text", lyrics.search(&query).await.map(|x| x.len()));

    // The lyrics source above keeps a grant of its own, and a rotation it makes leaves this one
    // stale. The app's account restores again when that happens, so the probe does too.
    let client = SpotifyClient::new(
        auth::restore(&AuthConfig::from_env())
            .await?
            .context("no stored Spotify sign-in")?,
    );
    let (api, device) = client.benchmark_session();

    let dealer_read = Measure::start("dealer");
    let mut dealer = Dealer::start(api.clone());
    let connection = tokio::time::timeout(Duration::from_secs(15), async {
        while let Some(event) = dealer.next().await {
            if let Event::Connected(id) = event {
                return Ok(id);
            }
        }
        anyhow::bail!("Dealer closed")
    })
    .await
    .context("Dealer announcement timed out")??;
    dealer_read.finish(Ok(1));
    // No participant writes are needed to cover the invitation lookup.
    let invite = std::env::var("SONORA_BENCHMARK_JAM_TOKEN")
        .unwrap_or_else(|_| "00000000000000000000000000000000".into());
    measure!(
        "jam_invite",
        jam_result(api.jam_invite(&invite, &connection).await)
    );
    drop(dealer);

    measure!("audio_read", audio_read(api.clone(), device).await);
    Ok(())
}

async fn count_pages<T>(mut pages: music::Pages<T>, measure: Option<&Measure>) -> Result<usize> {
    let mut count = 0;
    let mut reported = false;
    while let Some(page) = pages.recv().await {
        count += page?.items.len();
        if !reported
            && count != 0
            && let Some(measure) = measure
        {
            measure.first();
            reported = true;
        }
    }
    Ok(count)
}

fn jam_result(result: Result<Option<spotifyapis::proto::social_connect::SocialSession>>) -> Result<usize> {
    match result {
        Ok(session) => Ok(usize::from(session.is_some())),
        Err(error) if spotifyapis::http_status(&error) == Some(http::StatusCode::NOT_FOUND) => {
            Ok(0)
        }
        Err(error) => Err(error),
    }
}

async fn audio_read(api: Arc<Session>, device: &str) -> Result<usize> {
    let key = (ExtensionKind::TrackV4, spotifyapis::id::track_uri(AUDIO_TRACK));
    let track: Track = api
        .extended_metadata([(key.1.as_str(), key.0)])
        .await?
        .remove(&key)
        .context("missing audio metadata")?
        .decode()?;
    let file: [u8; 20] = [1, 2, 0]
        .into_iter()
        .find_map(|format| {
            track
                .file
                .iter()
                .find(|file| file.format == Some(format))
                .and_then(|file| file.file_id().try_into().ok())
        })
        .context("missing Vorbis file")?;
    let file_id = file
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let url = api
        .audio_urls(&file_id)
        .await?
        .into_iter()
        .next()
        .context("missing CDN URL")?;
    println!("HTTP signed CDN range");
    let response = spotifyapis::media_client()?
        .get(url)
        .header(http::header::RANGE, "bytes=0-16383")
        .send()
        .await?
        .error_for_status()?;
    anyhow::ensure!(
        response.status() == http::StatusCode::PARTIAL_CONTENT,
        "CDN did not serve a range"
    );
    let bytes = response.bytes().await?;
    AudioKeys::new(api, device.to_owned())
        .get(raw_id(AUDIO_TRACK).context("invalid track ID")?, file)
        .await?;
    Ok(bytes.len())
}
