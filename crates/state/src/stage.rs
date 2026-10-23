use std::collections::HashSet;
use std::sync::Arc;

use anyhow::{Context as _, Result, bail};
use gpui::{Context, Entity, Task};
use music::{AlbumDetail, Artist, Canvas, Credit, SavedArtist};

use crate::{Devices, Io, Playback, Session, SessionEvent, join};

/// The most a Canvas file is allowed to weigh. The clips are a few seconds long, so anything
/// past this is not one, and is never held in memory.
const LARGEST_CLIP: usize = 16 * 1024 * 1024;

/// What the right sidebar shows about the song playing beyond the song itself: its Canvas and
/// the clip it plays, its play count, its album, everyone credited on it, its lead artist and
/// the artists its listeners also like. It follows playback, loads
/// each part independently and publishes metadata together, and clears on sign-out. A part
/// the provider has no answer for stays empty, and the sidebar leaves it out.
pub struct Stage {
    session: Entity<Session>,
    playback: Entity<Playback>,
    io: Io,
    http: reqwest::Client,
    /// The song all of this is about.
    following: Option<String>,
    shown: Details,
    incoming: Option<Details>,
    loads: Vec<Task<()>>,
    pending: HashSet<&'static str>,
}

/// One complete set of song details. Pending loads update a separate set, published together.
#[derive(Default)]
struct Details {
    canvas: Option<Canvas>,
    /// The clip's file, once downloaded, shared with whatever plays it.
    clip: Option<Arc<[u8]>>,
    credits: Vec<Credit>,
    artist: Option<(String, Arc<Artist>)>,
    plays: Option<u64>,
    album: Option<Arc<AlbumDetail>>,
    similar: Vec<SavedArtist>,
}

impl Stage {
    pub fn new(
        session: Entity<Session>,
        playback: Entity<Playback>,
        devices: Entity<Devices>,
        io: Io,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&playback, |this, _, cx| this.follow(cx))
            .detach();
        cx.observe(&devices, |this, _, cx| this.follow(cx)).detach();
        cx.subscribe(&session, |this, _, event, cx| {
            if let SessionEvent::SignedOut = event {
                this.follow_song(None, cx);
            }
        })
        .detach();

        let mut this = Self {
            session,
            playback,
            io,
            http: reqwest::Client::new(),
            following: None,
            shown: Details::default(),
            incoming: None,
            loads: Vec::new(),
            pending: HashSet::new(),
        };
        this.follow(cx);
        this
    }

    /// The song's Canvas, if it has one. A still has a `url` to show as an image.
    pub fn canvas(&self) -> Option<&Canvas> {
        self.shown.canvas.as_ref()
    }

    /// The Canvas clip's file, once it has downloaded.
    pub fn clip(&self) -> Option<Arc<[u8]>> {
        self.shown.clip.clone()
    }

    /// Everyone credited on the song: the full credits where the provider has them, and the
    /// track's own credits otherwise.
    pub fn credits(&self) -> &[Credit] {
        &self.shown.credits
    }

    /// The song's lead artist, with its id, once it has loaded.
    pub fn artist(&self) -> Option<(&str, &Artist)> {
        self.shown
            .artist
            .as_ref()
            .map(|(id, artist)| (id.as_str(), artist.as_ref()))
    }

    /// How many times the song has been played, where the provider counts.
    pub fn plays(&self) -> Option<u64> {
        self.incoming
            .is_none()
            .then_some(self.shown.plays)
            .flatten()
    }

    /// The album the song is on, once it has loaded.
    pub fn album(&self) -> Option<&AlbumDetail> {
        self.shown.album.as_deref()
    }

    /// Artists the song's listeners also like.
    pub fn similar(&self) -> &[SavedArtist] {
        &self.shown.similar
    }

    /// Whether a named part is still loading for the current song. Failures finish the load too.
    pub fn loading(&self, part: &str) -> bool {
        self.pending.contains(part)
    }

    fn follow(&mut self, cx: &mut Context<Self>) {
        let track = self.playback.read(cx).shown_track(cx);
        let id = track.as_ref().and_then(|track| track.id.clone());
        if id.is_none() && track.is_some() || id == self.following {
            return;
        }
        let lead = track.as_ref().and_then(|track| {
            track
                .artist_refs
                .iter()
                .find_map(|artist| artist.id.as_deref())
        });
        let album_id = track.as_ref().and_then(|track| track.album_id.as_deref());
        let artist = self
            .shown
            .artist
            .clone()
            .filter(|(id, _)| Some(id.as_str()) == lead);
        let album = self
            .shown
            .album
            .clone()
            .filter(|album| Some(album.album.id.as_str()) == album_id);
        let similar = match album.is_some() {
            true => self.shown.similar.clone(),
            false => Vec::new(),
        };
        self.follow_song(id, cx);
        let (Some(track), Some(id)) = (track, self.following.clone()) else {
            return;
        };
        let incoming = self
            .incoming
            .as_mut()
            .expect("song details are being loaded");
        incoming.artist = artist;
        incoming.album = album;
        incoming.similar = similar;
        incoming.credits = track.credits;
        let session = self.session.read(cx);
        let client = match music::is_local_id(&id) {
            true => session.local_client(),
            false => session.client(),
        };
        let catalog = session.catalog(&id);
        let lead = track.artist_refs.into_iter().find_map(|artist| artist.id);
        let album = track.album_id;

        if let Some(client) = client {
            let (asked, http) = (id.clone(), self.http.clone());
            let canvas = client.clone();
            self.load(
                "canvas",
                async move {
                    let Some(canvas) = canvas.canvas(&asked).await? else {
                        return Ok((None, None));
                    };
                    let clip = match canvas.video {
                        true => download(&http, &canvas.url)
                            .await
                            .inspect_err(|error| log::warn!("stage: no clip: {error:#}"))
                            .ok(),
                        false => None,
                    };
                    Ok((Some(canvas), clip))
                },
                |this, (canvas, clip)| (this.canvas, this.clip) = (canvas, clip),
                cx,
            );
            let (asked, plays) = (id.clone(), client.clone());
            self.load(
                "play count",
                async move { plays.track_playcount(&asked).await },
                |this, plays| this.plays = plays,
                cx,
            );
            let asked = id.clone();
            self.load(
                "credits",
                async move { client.credits(&asked).await },
                |this, credits| {
                    if !credits.is_empty() {
                        this.credits = credits;
                    }
                },
                cx,
            );
        }
        if let (Some(catalog), Some(album)) = (catalog.clone(), album) {
            if self
                .incoming
                .as_ref()
                .is_some_and(|details| details.album.is_none())
            {
                let (shelf, asked) = (catalog.clone(), album.clone());
                self.load(
                    "album",
                    async move { shelf.album(&asked).await },
                    |this, album| this.album = Some(album),
                    cx,
                );
            }
            if self
                .incoming
                .as_ref()
                .is_some_and(|details| details.similar.is_empty())
            {
                let artist = lead.clone();
                self.load(
                    "similar artists",
                    async move { catalog.album_catalogue(&album, artist.as_deref()).await },
                    |this, related| this.similar = related.similar.clone(),
                    cx,
                );
            }
        }
        if let (Some(catalog), Some(lead)) = (
            catalog,
            lead.filter(|_| {
                self.incoming
                    .as_ref()
                    .is_some_and(|details| details.artist.is_none())
            }),
        ) {
            let asked = lead.clone();
            self.load(
                "artist",
                async move { catalog.artist(&asked).await },
                |this, artist| this.artist = Some((lead, artist)),
                cx,
            );
        }
        self.publish();
    }

    /// Publishes metadata together; a slower Canvas download can replace just the background later.
    fn publish(&mut self) {
        if self.pending.iter().all(|part| *part == "canvas")
            && let Some(details) = self.incoming.take()
        {
            self.shown = details;
        }
    }

    /// Cancels previous loads, retaining the visible details until the replacement is ready.
    fn follow_song(&mut self, song: Option<String>, cx: &mut Context<Self>) {
        self.following = song.clone();
        self.incoming = song.as_ref().map(|_| Details::default());
        if song.is_none() {
            self.shown = Details::default();
        }
        self.loads.clear();
        self.pending.clear();
        cx.notify();
    }

    /// Runs `fetch` on tokio and hands its answer to `apply`, if the song it was for is still
    /// playing by then. A provider with no answer only logs, and the part stays empty.
    fn load<T: Send + 'static>(
        &mut self,
        part: &'static str,
        fetch: impl Future<Output = Result<T>> + Send + 'static,
        apply: impl FnOnce(&mut Details, T) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.pending.insert(part);
        let (io, song) = (self.io.clone(), self.following.clone());
        self.loads.push(cx.spawn(async move |this, cx| {
            let found = join(io.spawn(fetch)).await;
            this.update(cx, |this, cx| {
                if this.following != song {
                    return;
                }
                this.pending.remove(part);
                match found {
                    Ok(value) => {
                        apply(this.incoming.as_mut().unwrap_or(&mut this.shown), value);
                    }
                    Err(error) => log::debug!("stage: no {part}: {error:#}"),
                }
                this.publish();
                cx.notify();
            })
            .ok();
        }));
    }
}

/// Downloads a Canvas clip, refusing one too large to be a clip.
async fn download(http: &reqwest::Client, url: &str) -> Result<Arc<[u8]>> {
    let response = http
        .get(url)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .context("cannot fetch the canvas clip")?;
    if response
        .content_length()
        .is_some_and(|length| length as usize > LARGEST_CLIP)
    {
        bail!("the canvas clip is too large");
    }
    let bytes = response
        .bytes()
        .await
        .context("cannot read the canvas clip")?;
    if bytes.len() > LARGEST_CLIP {
        bail!("the canvas clip is too large");
    }
    Ok(Arc::from(bytes.as_ref()))
}
