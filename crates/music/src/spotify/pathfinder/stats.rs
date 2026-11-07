use std::collections::HashMap;

use anyhow::{Context as _, Result, ensure};
use librespot_core::Session;
use serde::Deserialize;

use crate::spotify::{collection, descriptors};
use crate::{Album, ListeningStats, Ranked, SavedArtist, StatsPeriod};

/// Reads the same personal rankings as Spotify's profile, without the Web API.
pub(crate) async fn stats(session: &Session, period: StatsPeriod) -> Result<ListeningStats> {
    let range = match period {
        StatsPeriod::Weeks => "SHORT_TERM",
        StatsPeriod::Months => "MID_TERM",
        StatsPeriod::Year => "LONG_TERM",
        _ => anyhow::bail!("unsupported Spotify stats period"),
    };
    let input =
        serde_json::json!({"offset": 0, "limit": 50, "sortBy": "AFFINITY", "timeRange": range});
    let data: Data = super::query(
        session,
        "userTopContent",
        serde_json::json!({
            "includeTopArtists": true, "topArtistsInput": input,
            "includeTopTracks": true, "topTracksInput": input,
        }),
    )
    .await?;
    let profile = data
        .me
        .context("stats response has no account")?
        .profile
        .context("stats response has no profile")?;
    ensure!(
        profile.tracks.kind == "TrackPageV2" && profile.artists.kind == "ArtistPageV2",
        "Spotify did not return personal rankings"
    );
    let uris: Vec<_> = profile
        .tracks
        .items
        .into_iter()
        .filter_map(|hit| hit.data)
        .filter(|item| item.kind == "Track")
        .map(|item| item.uri)
        .collect();
    let (metadata, tags) = tokio::join!(
        collection::metadata(session, &uris),
        descriptors::genres(session, &uris)
    );
    let mut metadata = metadata?;
    let mut tags = tags.unwrap_or_else(|error| {
        log::warn!("stats: cannot read Spotify track genres: {error:#}");
        HashMap::new()
    });
    let mut descriptors: HashMap<String, usize> = HashMap::new();
    let tracks: Vec<_> = uris
        .into_iter()
        .filter_map(|uri| {
            let mut track = metadata.remove(&uri)?;
            track.tags = tags.remove(&uri).unwrap_or_default();
            for tag in &track.tags {
                *descriptors.entry(tag.clone()).or_default() += 1;
            }
            Some(track)
        })
        .map(|item| Ranked { item, count: None })
        .collect();
    let mut album_map = HashMap::<String, (Album, usize, usize)>::new();
    for (rank, ranked) in tracks.iter().enumerate() {
        let track = &ranked.item;
        if track.album.is_empty() {
            continue;
        }
        let id = track
            .album_id
            .clone()
            .unwrap_or_else(|| track.album.clone());
        let entry = album_map.entry(id.clone()).or_insert_with(|| {
            (
                Album {
                    id,
                    name: track.album.clone(),
                    artists: track.artists.clone(),
                    artist_refs: track.artist_refs.clone(),
                    cover: track.cover.clone(),
                    ..Default::default()
                },
                0,
                rank,
            )
        });
        entry.1 += 1;
    }
    let mut albums: Vec<_> = album_map.into_values().collect();
    albums.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.2.cmp(&b.2)));
    let albums = albums
        .into_iter()
        .map(|(item, _, _)| Ranked { item, count: None })
        .collect();
    let artists = profile
        .artists
        .items
        .into_iter()
        .filter_map(|hit| hit.data)
        .filter(|item| item.kind == "Artist")
        .filter_map(|item| {
            let id = item.uri.strip_prefix("spotify:artist:")?.to_owned();
            let name = item.profile?.name;
            let cover = item
                .visuals
                .and_then(|v| v.avatar_image)
                .and_then(|image| image.sources.into_iter().max_by_key(|s| s.height))
                .map(|source| source.url);
            Some(Ranked {
                item: SavedArtist {
                    id,
                    name,
                    cover,
                    ..Default::default()
                },
                count: None,
            })
        })
        .collect();
    let mut descriptors: Vec<_> = descriptors
        .into_iter()
        .map(|(item, count)| Ranked {
            item,
            count: Some(count),
        })
        .collect();
    descriptors.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.item.cmp(&b.item)));
    Ok(ListeningStats {
        tracks,
        artists,
        albums,
        descriptors,
        plays: None,
        ..Default::default()
    })
}

#[derive(Deserialize)]
struct Data {
    me: Option<Account>,
}
#[derive(Deserialize)]
struct Account {
    profile: Option<Profile>,
}
#[derive(Deserialize)]
struct Profile {
    #[serde(rename = "topTracks")]
    tracks: Page,
    #[serde(rename = "topArtists")]
    artists: Page,
}
#[derive(Deserialize)]
struct Page {
    #[serde(rename = "__typename")]
    kind: String,
    #[serde(default)]
    items: Vec<Hit>,
}
#[derive(Deserialize)]
struct Hit {
    data: Option<Item>,
}
#[derive(Deserialize)]
struct Item {
    #[serde(rename = "__typename")]
    kind: String,
    #[serde(default)]
    uri: String,
    profile: Option<Name>,
    visuals: Option<Visuals>,
}
#[derive(Deserialize)]
struct Name {
    name: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Visuals {
    avatar_image: Option<Image>,
}
#[derive(Deserialize)]
struct Image {
    sources: Vec<Source>,
}
#[derive(Deserialize)]
struct Source {
    url: String,
    height: Option<u32>,
}
