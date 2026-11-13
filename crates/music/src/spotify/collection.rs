use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

use anyhow::{Context as _, Result};
use http::header::{ACCEPT, CONTENT_TYPE};
use http::{HeaderMap, HeaderValue, Method};
use librespot_core::Session;

use crate::spotify::pb::{Reader, Value, Writer, text};
use crate::spotify::{metadata, pace::paced};
use crate::{Page, Pages, Track};

const TRACK_PREFIX: &str = "spotify:track:";
const PAGING: &str = "/collection/v2/paging";
const WRITE: &str = "/collection/v2/write";
const CONTENT: &str = "application/vnd.collection-v2.spotify.proto";
pub(crate) const COLLECTION: &str = "collection";
pub(crate) const ARTISTS: &str = "artist";
const PAGE: i32 = 300;

static UPDATE: AtomicU64 = AtomicU64::new(0);

const REQUEST_USERNAME: u32 = 1;
const REQUEST_SET: u32 = 2;
const REQUEST_TOKEN: u32 = 3;
const REQUEST_LIMIT: u32 = 4;

const RESPONSE_ITEMS: u32 = 1;
const RESPONSE_TOKEN: u32 = 2;

const ITEM_URI: u32 = 1;
const ITEM_ADDED_AT: u32 = 2;
const ITEM_REMOVED: u32 = 3;

const WRITE_USERNAME: u32 = 1;
const WRITE_SET: u32 = 2;
const WRITE_ITEMS: u32 = 3;
const WRITE_UPDATE_ID: u32 = 4;

struct CollectionPage {
    items: Vec<SavedItem>,
    next: String,
}

pub(crate) struct SavedItem {
    pub(crate) uri: String,
    pub(crate) added_at: Option<crate::Timestamp>,
}

pub async fn saved_tracks(session: &Session) -> Result<Vec<Track>> {
    let mut pages = saved_tracks_paged(session).await?;
    let mut tracks = Vec::new();
    while let Some(page) = pages.recv().await {
        tracks.extend(page?.items);
    }
    Ok(tracks)
}

/// Streams enriched favorites in bounded batches, so a large library does not delay its first rows.
/// Dropping the receiver cancels the in-flight batch and stops further requests.
pub async fn saved_tracks_paged(session: &Session) -> Result<Pages<Track>> {
    let items = saved_items(session, COLLECTION, TRACK_PREFIX).await?;
    let total = Some(items.len());
    let session = session.clone();
    Ok(crate::pages(1, move |sink| async move {
        for items in items.chunks(metadata::FEATURES_AT_ONCE) {
            let uris: Vec<_> = items.iter().map(|item| item.uri.clone()).collect();
            let mut known = metadata::tracks(&session, &uris).await?;
            let items = items
                .iter()
                .filter_map(|item| {
                    let mut track = known.remove(&item.uri)?;
                    track.added_at = item.added_at;
                    Some(track)
                })
                .collect();
            if sink.send(Ok(Page { total, items })).await.is_err() {
                break;
            }
        }
        Ok(())
    }))
}

/// Stars or unstars a track, album or artist in its corresponding collection set.
pub async fn set_saved(session: &Session, kind: &str, id: &str, saved: bool) -> Result<()> {
    let uri = format!("spotify:{kind}:{id}");
    let set = match kind {
        "artist" => ARTISTS,
        _ => COLLECTION,
    };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("cannot read the current time")?;
    let added_at = match saved {
        true => now
            .as_secs()
            .try_into()
            .context("current time exceeds the collection timestamp range")?,
        false => 0,
    };

    let mut item = Writer::default();
    item.string(ITEM_URI, &uri);
    item.int32(ITEM_ADDED_AT, added_at);
    item.bool(ITEM_REMOVED, !saved);

    let sequence = UPDATE.fetch_add(1, Ordering::Relaxed);
    let update_id = format!(
        "sonora-{}-{}-{}-{sequence}",
        now.as_secs(),
        now.subsec_nanos(),
        std::process::id()
    );
    let mut request = Writer::default();
    request.string(WRITE_USERNAME, &session.username());
    request.string(WRITE_SET, set);
    request.message(WRITE_ITEMS, &item.finish());
    request.string(WRITE_UPDATE_ID, &update_id);
    let body = request.finish();

    paced(|| {
        session
            .spclient()
            .request(&Method::POST, WRITE, Some(headers()), Some(&body))
    })
    .await
    .context("cannot update the saved collection")?;
    Ok(())
}

pub(crate) async fn saved_items(
    session: &Session,
    set: &str,
    prefix: &str,
) -> Result<Vec<SavedItem>> {
    let username = session.username();
    let mut found = Vec::new();
    let mut token = String::new();

    loop {
        let body = request(&username, set, &token);
        let raw = paced(|| {
            session
                .spclient()
                .request(&Method::POST, PAGING, Some(headers()), Some(&body))
        })
        .await
        .context("cannot read the saved collection")?;

        let page = response(&raw).context("cannot decode the collection page")?;

        found.extend(
            page.items
                .into_iter()
                .filter(|item| item.uri.starts_with(prefix)),
        );

        token = page.next;
        if token.is_empty() {
            break;
        }
    }

    Ok(found)
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(CONTENT));
    headers.insert(ACCEPT, HeaderValue::from_static(CONTENT));
    headers
}

fn request(username: &str, set: &str, token: &str) -> Vec<u8> {
    let mut writer = Writer::default();
    writer.string(REQUEST_USERNAME, username);
    writer.string(REQUEST_SET, set);
    writer.string(REQUEST_TOKEN, token);
    writer.int32(REQUEST_LIMIT, PAGE);
    writer.finish()
}

fn response(bytes: &[u8]) -> Result<CollectionPage> {
    let mut page = CollectionPage {
        items: Vec::new(),
        next: String::new(),
    };
    let mut reader = Reader::new(bytes);

    while let Some((field, value)) = reader.field()? {
        match (field, value) {
            (RESPONSE_ITEMS, Value::Bytes(item)) => page.items.extend(kept(item)?),
            (RESPONSE_TOKEN, Value::Bytes(token)) => page.next = text(token)?,
            _ => (),
        }
    }

    Ok(page)
}

fn kept(bytes: &[u8]) -> Result<Option<SavedItem>> {
    let mut uri = None;
    let mut added_at = None;
    let mut removed = false;
    let mut reader = Reader::new(bytes);

    while let Some((field, value)) = reader.field()? {
        match (field, value) {
            (ITEM_URI, Value::Bytes(raw)) => uri = Some(text(raw)?),
            (ITEM_ADDED_AT, Value::Varint(raw) | Value::Fixed(raw)) => {
                added_at = crate::Timestamp::from_second(raw as u32 as i32 as i64).ok();
            }
            (ITEM_REMOVED, Value::Varint(flag)) => removed = flag != 0,
            _ => (),
        }
    }

    if added_at.is_none() && uri.is_some() && !removed {
        debug!(
            "collection: saved item has no date, fields: {}",
            fields(bytes)
        );
    }

    Ok(uri
        .filter(|_| !removed)
        .map(|uri| SavedItem { uri, added_at }))
}

fn fields(bytes: &[u8]) -> String {
    let mut reader = Reader::new(bytes);
    let mut seen = Vec::new();
    while let Ok(Some((field, value))) = reader.field() {
        seen.push(match value {
            Value::Varint(raw) => format!("{field}=varint({raw})"),
            Value::Fixed(raw) => format!("{field}=fixed({raw})"),
            Value::Bytes(raw) => format!("{field}=bytes({})", raw.len()),
        });
    }
    seen.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_saved_item_with_date() {
        let mut writer = Writer::default();
        writer.string(ITEM_URI, "spotify:track:abc");
        writer.int32(ITEM_ADDED_AT, 1_725_552_000);

        let item = kept(&writer.finish()).unwrap().unwrap();

        assert_eq!(item.uri, "spotify:track:abc");
        assert_eq!(
            item.added_at.map(|stamp| stamp.as_second()),
            Some(1_725_552_000)
        );
    }

    #[test]
    fn drops_removed_item() {
        let mut writer = Writer::default();
        writer.string(ITEM_URI, "spotify:track:abc");
        writer.int32(ITEM_REMOVED, 1);

        assert!(kept(&writer.finish()).unwrap().is_none());
    }
}
