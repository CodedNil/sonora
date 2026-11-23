use anyhow::{Context as _, Result};

use crate::spotify::{auth::sign_in_error, collection, session::Session};
use spotifyapis::proto::sequence_proxy::ResolvedSequence;

const RADIO_TRACKS: usize = 50;

pub(in crate::spotify) async fn resolve(session: &Session, uri: &str) -> Result<ResolvedSequence> {
    session
        .api()
        .resolve_sequence(uri, session.device_id())
        .await
        .map_err(sign_in_error)
}

/// Resolves and hydrates a track station in Spotify's playback order.
pub(in crate::spotify) async fn track_radio(
    session: &Session,
    id: &str,
) -> Result<Vec<crate::Track>> {
    let station = format!("spotify:station:track:{id}");
    let uris: Vec<_> = resolve(session, &station)
        .await
        .context("cannot resolve Spotify track radio")?
        .tracks
        .into_iter()
        .filter_map(|track| track.uri)
        .filter(|uri| uri.starts_with("spotify:track:"))
        .take(RADIO_TRACKS)
        .collect();
    collection::ordered_tracks(session, &uris, collection::TRACK_DETAILS).await
}
