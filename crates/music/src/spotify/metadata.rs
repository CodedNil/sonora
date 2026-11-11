use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context as _, Result};
use librespot_core::Session;
use librespot_protocol::entity_extension_data::EntityExtensionData;
use librespot_protocol::extended_metadata::{
    BatchedEntityRequest, EntityExtensionDataArray, EntityRequest, ExtensionQuery,
};
use librespot_protocol::extension_kind::ExtensionKind;
use librespot_protocol::metadata::image::Size as ImageSize;
use librespot_protocol::metadata::{
    Album as AlbumMessage, Artist as ArtistMessage, Track as TrackMessage,
};
use protobuf::{EnumOrUnknown, Message as _};
use serde::Deserialize;

use crate::spotify::{pace::paced, pb, wire};
use crate::{ArtistRef, AudioFeatures, Credit, Track};

const TRACK_PREFIX: &str = "spotify:track:";
const UNKNOWN: &str = "Unknown";
const BATCH: usize = 500;
pub(super) const FEATURES_AT_ONCE: usize = 100;

/// Enriches tracks supplied by Pathfinder, which does not include descriptor words or audio features.
pub(super) async fn enrich(session: &Session, tracks: &mut [Track]) {
    let uris: Vec<_> = tracks
        .iter()
        .filter_map(|track| track.id.as_ref())
        .map(|id| format!("{TRACK_PREFIX}{id}"))
        .collect();
    if uris.is_empty() {
        return;
    }
    let (words, sound) = tokio::join!(
        extended(session, &uris, ExtensionKind::TRACK_DESCRIPTOR),
        features(session, &uris)
    );
    apply(tracks.iter_mut(), words, sound);
}

/// Applies both optional sources to the same track metadata. A failed source leaves the other intact.
fn apply<'a>(
    tracks: impl Iterator<Item = &'a mut Track>,
    words: Result<Vec<EntityExtensionData>>,
    sound: HashMap<String, Features>,
) {
    let words: HashMap<_, _> = words
        .unwrap_or_else(|error| {
            log::warn!("metadata: cannot read Spotify descriptor words: {error:#}");
            Vec::new()
        })
        .into_iter()
        .filter_map(|entity| match concepts(&entity.extension_data.value) {
            Ok(found) => Some((entity.entity_uri, found)),
            Err(error) => {
                log::warn!("metadata: skipped {}: {error:#}", entity.entity_uri);
                None
            }
        })
        .collect();
    for track in tracks {
        let Some(id) = &track.id else { continue };
        if let Some(found) = words.get(&format!("{TRACK_PREFIX}{id}")) {
            track.genres = found.0.clone();
            track.tags = found.1.clone();
        }
        if let Some(features) = sound.get(id) {
            track.audio_features = Some(features.audio);
            track.bpm = features.tempo.and_then(crate::rounded_bpm);
        }
    }
}

/// Reads descriptor text and its repeated type enums: type 1 is a genre, and any other word is a
/// tag.
fn concepts(bytes: &[u8]) -> Result<(Vec<String>, Vec<String>)> {
    let mut reader = pb::Reader::new(bytes);
    let mut genres = Vec::new();
    let mut tags = Vec::new();
    while let Some((field, value)) = reader.field()? {
        let (1, pb::Value::Bytes(descriptor)) = (field, value) else {
            continue;
        };
        let mut descriptor = pb::Reader::new(descriptor);
        let mut text = None;
        let mut genre = false;
        while let Some((field, value)) = descriptor.field()? {
            match (field, value) {
                (1, pb::Value::Bytes(value)) => text = Some(pb::text(value)?),
                (3, pb::Value::Varint(value)) => genre |= value == 1,
                (3, pb::Value::Bytes(values)) => genre |= pb::varints(values)?.contains(&1),
                _ => {}
            }
        }
        let Some(text) = text else { continue };
        let text = text.trim().to_lowercase();
        let text = text
            .strip_prefix("spotify:concept:")
            .unwrap_or(&text)
            .replace('_', " ");
        if text.is_empty() || text == "track" {
            continue;
        }
        let list = match genre {
            true => &mut genres,
            false => &mut tags,
        };
        if !list.contains(&text) {
            list.push(text);
        }
    }
    Ok((genres, tags))
}

/// The identity and tempo surrounding Spotify's provider-neutral audio measurements.
#[derive(Deserialize)]
struct Features {
    id: String,
    tempo: Option<f32>,
    #[serde(flatten)]
    audio: AudioFeatures,
}

/// Reads how each of the tracks sounds by id, a hundred to a request, which is what Spotify's own
/// audio analysis answers for. A track it does not know is left out.
async fn features(session: &Session, uris: &[String]) -> HashMap<String, Features> {
    #[derive(Deserialize)]
    struct Answer {
        audio_features: Vec<Option<Features>>,
    }
    let ids: Vec<_> = uris
        .iter()
        .filter_map(|uri| uri.strip_prefix(TRACK_PREFIX))
        .collect();
    let mut found = HashMap::new();
    for ids in ids.chunks(FEATURES_AT_ONCE) {
        let path = format!("/audio-attributes/v1/audio-features?ids={}", ids.join(","));
        let answer: Result<Answer> = async {
            let body = session
                .spclient()
                .request_as_json(&http::Method::GET, &path, None, None)
                .await
                .context("cannot request audio features")?;
            serde_json::from_slice(&body).context("cannot decode audio features")
        }
        .await;
        match answer {
            Ok(answer) => found.extend(
                answer
                    .audio_features
                    .into_iter()
                    .flatten()
                    .map(|features| (features.id.clone(), features)),
            ),
            Err(error) => {
                log::warn!("metadata: cannot read an audio feature batch: {error:#}");
            }
        }
    }
    found
}

pub async fn track(session: &Session, track_id: &str) -> Result<Track> {
    let uri = format!("{TRACK_PREFIX}{track_id}");
    tracks(session, std::slice::from_ref(&uri))
        .await?
        .remove(&uri)
        .context("track metadata is missing")
}

pub(crate) async fn tracks(session: &Session, uris: &[String]) -> Result<HashMap<String, Track>> {
    let mut tracks = HashMap::with_capacity(uris.len());
    for uris in uris.chunks(BATCH) {
        let (response, sound) = tokio::join!(
            extensions(
                session,
                uris,
                &[ExtensionKind::TRACK_V4, ExtensionKind::TRACK_DESCRIPTOR]
            ),
            features(session, uris),
        );
        let response = match response {
            Ok(response) => response,
            Err(error) => {
                log::warn!("metadata: cannot read combined Spotify metadata: {error:#}");
                extensions(session, uris, &[ExtensionKind::TRACK_V4])
                    .await
                    .context("cannot read track metadata")?
            }
        };
        let mut batch = HashMap::with_capacity(uris.len());
        let mut words = Vec::new();
        for array in response {
            match array.extension_kind.enum_value() {
                Ok(ExtensionKind::TRACK_V4) => {
                    for entity in array.extension_data {
                        if let Ok(message) =
                            TrackMessage::parse_from_bytes(&entity.extension_data.value)
                        {
                            batch.insert(
                                entity.entity_uri.clone(),
                                track_from(&entity.entity_uri, &message),
                            );
                        }
                    }
                }
                Ok(ExtensionKind::TRACK_DESCRIPTOR) => words.extend(array.extension_data),
                _ => {}
            }
        }
        apply(batch.values_mut(), Ok(words), sound);
        tracks.extend(batch);
    }
    Ok(tracks)
}

pub(crate) async fn extended(
    session: &Session,
    uris: &[String],
    kind: ExtensionKind,
) -> Result<Vec<EntityExtensionData>> {
    let mut entities = Vec::with_capacity(uris.len());
    batched(session, uris, kind, BATCH, |batch| entities.extend(batch)).await?;

    Ok(entities)
}

/// Reads extended metadata a batch at a time and hands each batch to `take` before asking
/// for the next one. A caller that turns the entities into its own models here never holds
/// more than one batch of protobufs, which is what keeps a thousand-release discography from
/// costing hundreds of megabytes.
pub(crate) async fn batched(
    session: &Session,
    uris: &[String],
    kind: ExtensionKind,
    batch: usize,
    mut take: impl FnMut(Vec<EntityExtensionData>),
) -> Result<()> {
    for uris in uris.chunks(batch.max(1)) {
        take(
            extensions(session, uris, &[kind])
                .await?
                .into_iter()
                .flat_map(|array| array.extension_data)
                .collect(),
        );
    }
    Ok(())
}

/// Requests several metadata extensions per URI in one spclient lookup.
async fn extensions(
    session: &Session,
    uris: &[String],
    kinds: &[ExtensionKind],
) -> Result<Vec<EntityExtensionDataArray>> {
    let request = BatchedEntityRequest {
        entity_request: uris
            .iter()
            .map(|uri| EntityRequest {
                entity_uri: uri.clone(),
                query: kinds
                    .iter()
                    .map(|kind| ExtensionQuery {
                        extension_kind: EnumOrUnknown::new(*kind),
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    };
    Ok(
        paced(|| session.spclient().get_extended_metadata(request.clone()))
            .await?
            .extended_metadata,
    )
}

fn track_from(uri: &str, track: &TrackMessage) -> Track {
    let (artists, artist_refs) = artists_from(&track.artist);

    Track {
        id: uri.strip_prefix(TRACK_PREFIX).map(str::to_owned),
        playable: !track.file.is_empty() || !track.alternative.is_empty(),
        name: non_empty(track.name.as_deref())
            .unwrap_or(UNKNOWN)
            .to_owned(),
        artists,
        artist_refs,
        album: track
            .album
            .as_ref()
            .and_then(|album| non_empty(album.name.as_deref()))
            .unwrap_or_default()
            .to_owned(),
        album_id: track.album.as_ref().and_then(|album| base62(album.gid())),
        cover: track.album.as_ref().and_then(cover_url),
        duration: track
            .duration
            .and_then(|value| u64::try_from(value).ok())
            .map(Duration::from_millis),
        release_date: track
            .album
            .as_ref()
            .and_then(|album| album.date.as_ref())
            .and_then(super::wire::release_date),
        popularity: track.popularity.map(|value| value.clamp(0, 100) as u8),
        explicit: track.explicit.unwrap_or_default(),
        track_number: track.number.unwrap_or_default().max(0) as u32,
        disc_number: track.disc_number.unwrap_or_default().max(0) as u32,
        languages: track.language_of_performance.clone(),
        credits: track
            .artist_with_role
            .iter()
            .filter_map(|credit| {
                let name = non_empty(credit.artist_name.as_deref())?.to_owned();
                let role = match credit.role.map(|role| role.value()) {
                    Some(1) => "Main artist",
                    Some(2) => "Featured artist",
                    Some(3) => "Remixer",
                    Some(4) => "Actor",
                    Some(5) => "Composer",
                    Some(6) => "Conductor",
                    Some(7) => "Orchestra",
                    _ => "Performer",
                };
                Some(Credit {
                    name,
                    role: role.to_owned(),
                    id: base62(credit.artist_gid()),
                })
            })
            .collect(),
        ..Default::default()
    }
}

pub(crate) fn artists_from(artists: &[ArtistMessage]) -> (String, Vec<ArtistRef>) {
    let refs: Vec<_> = artists
        .iter()
        .filter_map(|artist| {
            let name = non_empty(artist.name.as_deref())?.to_owned();
            Some(ArtistRef {
                name,
                id: base62(artist.gid()),
            })
        })
        .collect();
    let names = match refs.is_empty() {
        true => UNKNOWN.to_owned(),
        false => ArtistRef::names(&refs),
    };
    (names, refs)
}

pub(crate) fn base62(gid: &[u8]) -> Option<String> {
    librespot_core::SpotifyId::from_raw(gid)
        .ok()
        .map(|id| id.to_base62())
}

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.filter(|value| !value.is_empty())
}

fn cover_url(album: &AlbumMessage) -> Option<String> {
    let smallest = album
        .cover_group
        .as_ref()?
        .image
        .iter()
        .filter(|image| image.has_file_id())
        .min_by_key(|image| match image.size() {
            ImageSize::SMALL => 0,
            ImageSize::DEFAULT => 1,
            ImageSize::LARGE => 2,
            ImageSize::XLARGE => 3,
        })?;

    wire::image_url(smallest.file_id())
}
