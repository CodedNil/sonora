use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use gpui::{Context, Entity, Task};
use music::{MusicApi, Track};

use crate::catalog::CatalogSource;
use crate::{Io, Session, SessionEvent, join};

/// Album requests are shared by their tracks, with a small bound on concurrent provider work.
const CONCURRENT: usize = 4;

type Counts = Vec<(String, Option<u64>)>;

/// A provider request for one album's counts, or one track when it has no album catalogue.
struct Request {
    key: String,
    id: String,
    album: Option<(String, Arc<CatalogSource>)>,
    client: Arc<dyn MusicApi>,
}

/// Session-scoped play counts for table rows. Tables ask only for visible rows unless sorting
/// by count, share album requests, and never replace an unavailable count with zero.
pub struct TrackCounts {
    session: Entity<Session>,
    io: Io,
    counts: HashMap<String, Option<u64>>,
    asked: HashSet<String>,
    pending: VecDeque<Request>,
    loads: HashMap<String, Task<()>>,
    generation: u64,
}

impl TrackCounts {
    pub fn new(session: Entity<Session>, io: Io, cx: &mut Context<Self>) -> Self {
        cx.subscribe(&session, |this, _, event, cx| {
            if let SessionEvent::SignedOut = event {
                this.generation += 1;
                this.loads.clear();
                this.pending.clear();
                this.counts.clear();
                this.asked.clear();
                cx.notify();
            }
        })
        .detach();
        Self {
            session,
            io,
            counts: HashMap::new(),
            asked: HashSet::new(),
            pending: VecDeque::new(),
            loads: HashMap::new(),
            generation: 0,
        }
    }

    /// A cached provider count; missing and unavailable counts both stay blank in the table.
    pub fn get(&self, id: &str) -> Option<u64> {
        self.counts.get(id).copied().flatten()
    }

    /// Queues missing counts, coalescing tracks from the same album and requests already in flight.
    pub fn request(&mut self, tracks: &[Track], cx: &mut Context<Self>) {
        let session = self.session.read(cx);
        for track in tracks {
            let Some(id) = track.id.as_ref() else {
                continue;
            };
            if track.playcount.is_some() || self.counts.contains_key(id) {
                continue;
            }
            let client = match music::is_local_id(id) {
                true => session.local_client(),
                false => session.client(),
            };
            let Some(client) = client else {
                continue;
            };
            let album = track.album_id.clone().zip(session.catalog(id));
            let key = match &album {
                Some((id, _)) => format!("album:{id}"),
                None => format!("track:{id}"),
            };
            if !self.asked.insert(key.clone()) {
                continue;
            }
            self.pending.push_back(Request {
                key,
                id: id.clone(),
                album,
                client,
            });
        }
        self.load(cx);
    }

    /// Starts queued work on Tokio and retains each task until its result is applied.
    fn load(&mut self, cx: &mut Context<Self>) {
        while self.loads.len() < CONCURRENT {
            let Some(request) = self.pending.pop_front() else {
                break;
            };
            let key = request.key.clone();
            let held = key.clone();
            let io = self.io.clone();
            let generation = self.generation;
            let task = cx.spawn(async move |this, cx| {
                let found = join(io.spawn(fetch(request))).await;
                this.update(cx, |this, cx| {
                    if this.generation != generation {
                        return;
                    }
                    this.loads.remove(&key);
                    match found {
                        Ok(counts) => this.counts.extend(counts),
                        Err(error) => log::debug!("track counts: cannot load {key}: {error:#}"),
                    }
                    this.load(cx);
                    cx.notify();
                })
                .ok();
            });
            self.loads.insert(held, task);
        }
    }
}

/// Reads counts through the provider's shared catalogue, falling back to its single-track API.
async fn fetch(request: Request) -> anyhow::Result<Counts> {
    if let Some((id, catalog)) = request.album {
        let album = catalog.album(&id).await?;
        return Ok(album
            .tracks
            .iter()
            .filter_map(|track| Some((track.id.clone()?, track.playcount)))
            .collect());
    }
    let count = request.client.track_playcount(&request.id).await?;
    Ok(vec![(request.id, count)])
}
