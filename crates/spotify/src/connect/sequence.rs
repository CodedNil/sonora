use anyhow::{Context as _, Result};
use spotifyapis::id::TRACK_PREFIX;

use crate::{collection, session::Session};
use spotifyapis::proto::sequence_proxy::ResolvedSequence;

const RADIO_TRACKS: usize = 50;

pub(crate) async fn resolve(session: &Session, uri: &str) -> Result<ResolvedSequence> {
    session
        .call(|api| api.resolve_sequence(uri, session.device_id()))
        .await
}

pub(crate) fn radio_kind(seed: music::RadioSeed) -> &'static str {
    match seed {
        music::RadioSeed::Album => "album",
        music::RadioSeed::Playlist => "playlist",
        music::RadioSeed::Artist => "artist",
    }
}

/// Resolves and hydrates a station while preserving Spotify's order.
pub(crate) async fn radio(session: &Session, kind: &str, id: &str) -> Result<Vec<music::Track>> {
    let station = format!("spotify:station:{kind}:{id}");
    let uris: Vec<_> = resolve(session, &station)
        .await
        .context("cannot resolve Spotify radio")?
        .tracks
        .into_iter()
        .filter_map(|track| track.uri)
        .filter(|uri| uri.starts_with(TRACK_PREFIX))
        .take(RADIO_TRACKS)
        .collect();
    collection::ordered_tracks(session, &uris, collection::TRACK_DETAILS).await
}
