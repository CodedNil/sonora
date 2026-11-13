//! Canvas: the short looping clip Spotify plays behind a song's artwork. It is read from the
//! canvaz cache with a hand-built protobuf, since librespot carries only the response half.

use anyhow::{Context as _, Result};
use http::header::{ACCEPT, CONTENT_TYPE};
use http::{HeaderMap, HeaderValue, Method};
use librespot_core::Session;
use librespot_protocol::canvaz::{Type, entity_canvaz_response::Canvaz};
use protobuf::Message as _;

use crate::Canvas;
use crate::spotify::pb::{Reader, Value, Writer};

const ENDPOINT: &str = "/canvaz-cache/v0/canvases";
const CONTENT: &str = "application/x-protobuf";
const TRACK_PREFIX: &str = "spotify:track:";

// CanvasRequest { repeated Track tracks = 1 } and Track { string track_uri = 1 }.
const REQUEST_TRACKS: u32 = 1;
const REQUEST_TRACK_URI: u32 = 1;
// CanvasResponse { repeated Canvaz canvases = 1 }, and the Canvaz fields read here.
const RESPONSE_CANVASES: u32 = 1;

/// The song's Canvas, or `None` when the artist gave it none.
pub(crate) async fn canvas(session: &Session, track_id: &str) -> Result<Option<Canvas>> {
    let uri = format!("{TRACK_PREFIX}{track_id}");
    let mut track = Writer::default();
    track.string(REQUEST_TRACK_URI, &uri);
    let mut request = Writer::default();
    request.message(REQUEST_TRACKS, &track.finish());
    let body = request.finish();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(CONTENT));
    headers.insert(ACCEPT, HeaderValue::from_static(CONTENT));
    let raw = session
        .spclient()
        .request(&Method::POST, ENDPOINT, Some(headers), Some(&body))
        .await
        .context("cannot read the canvas")?;
    answer(&raw, &uri)
}

/// Picks the canvas for `uri` out of a CanvasResponse.
fn answer(raw: &[u8], uri: &str) -> Result<Option<Canvas>> {
    let mut response = Reader::new(raw);
    while let Some((field, value)) = response.field()? {
        let (RESPONSE_CANVASES, Value::Bytes(bytes)) = (field, value) else {
            continue;
        };
        let canvas = Canvaz::parse_from_bytes(bytes).context("cannot decode the canvas")?;
        if canvas.url.is_empty() || (!canvas.entity_uri.is_empty() && canvas.entity_uri != uri) {
            continue;
        }
        return Ok(Some(Canvas {
            url: canvas.url,
            video: canvas.type_.value() != Type::IMAGE as i32,
        }));
    }
    Ok(None)
}
