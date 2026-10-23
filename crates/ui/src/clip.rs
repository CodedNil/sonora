use std::sync::Arc;
use std::sync::mpsc::{SyncSender, TryRecvError, sync_channel};
use std::time::{Duration, Instant};

use gpui::prelude::*;
use gpui::{
    Context, ImageSource, ObjectFit, Render, RenderImage, SharedString, Task, Window, div, img,
};
use image::{Frame, RgbaImage};

use crate::theme::ActiveTheme as _;

/// How many decoded frames wait ahead of the one on screen. Each is a full picture, so this
/// stays small.
const AHEAD: usize = 2;
/// How long the player waits before looking again when the decoder has fallen behind.
const BEHIND: Duration = Duration::from_millis(4);
/// How often a paused clip looks to see whether it should play again.
const PAUSED: Duration = Duration::from_millis(100);

/// A short silent clip that loops for as long as it is on screen, such as a song's Canvas. It
/// is built from the clip's MP4 bytes and fills whatever box it is given, cropping to cover it.
/// A thread decodes a couple of frames ahead and stops once the player is dropped, and a clip
/// that cannot be decoded draws nothing, so the caller's own backdrop shows through.
pub struct ClipPlayer {
    shown: Option<Arc<RenderImage>>,
    /// Held on the frame it was showing, with the decoder idle, while nothing can see it.
    paused: bool,
    /// Frames taken off the screen, freed from the atlas one draw later so the frame that last
    /// painted them has finished with them.
    retired: Vec<Arc<RenderImage>>,
    _playing: Task<()>,
}

impl ClipPlayer {
    /// Starts decoding `bytes` and showing each frame as it comes due. `name` labels the
    /// decoding thread and its log lines.
    pub fn new(bytes: Arc<[u8]>, name: SharedString, cx: &mut Context<Self>) -> Self {
        let (frames, decoded) = sync_channel(AHEAD);
        let spawned = std::thread::Builder::new()
            .name("sonora-clip".into())
            .spawn(move || decode(bytes, frames, &name));
        if let Err(error) = spawned {
            log::warn!("clip: cannot start the decoder thread: {error}");
        }

        // Keep source timing while frames are ready. Rebase after starvation or a pause so a
        // slow decoder does not spend successive half-second stretches discarding fresh frames.
        let playing = cx.spawn(async move |this, cx| {
            let mut due = Instant::now();
            loop {
                if this.read_with(cx, |this, _| this.paused).unwrap_or(true) {
                    cx.background_executor().timer(PAUSED).await;
                    due = Instant::now();
                    continue;
                }
                let (frame, held) = match decoded.try_recv() {
                    Ok(next) => next,
                    Err(TryRecvError::Empty) => {
                        cx.background_executor().timer(BEHIND).await;
                        due = Instant::now();
                        continue;
                    }
                    Err(TryRecvError::Disconnected) => return,
                };
                let now = Instant::now();
                let ends = due.max(now) + held;
                due = ends;
                let shown = this.update(cx, |this, cx| {
                    if let Some(old) = this.shown.replace(frame) {
                        this.retired.push(old);
                    }
                    cx.notify();
                });
                if shown.is_err() {
                    return;
                }
                cx.background_executor()
                    .timer(ends.saturating_duration_since(Instant::now()))
                    .await;
            }
        });

        Self {
            shown: None,
            paused: false,
            retired: Vec::new(),
            _playing: playing,
        }
    }

    /// Holds the clip on its current frame, or plays it on from there. A clip that is out of
    /// sight is paused rather than dropped, so it shows again at once.
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
}

impl Render for ClipPlayer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The newest retiree may still be in the frame being replaced, so keep it one more draw.
        let keep = self.retired.pop();
        for old in self.retired.drain(..) {
            _ = window.drop_image(old);
        }
        self.retired.extend(keep);

        div().size_full().children(self.shown.clone().map(|frame| {
            img(ImageSource::Render(frame))
                .size_full()
                .object_fit(ObjectFit::Cover)
                .rounded(cx.theme().radius)
        }))
    }
}

/// Decodes the clip over and over, handing each frame and how long it stays up to the player,
/// until the player goes away.
fn decode(bytes: Arc<[u8]>, frames: SyncSender<(Arc<RenderImage>, Duration)>, name: &str) {
    let clip = match video::Clip::open(&bytes) {
        Ok(clip) => clip,
        Err(error) => return log::warn!("clip: cannot open {name}: {error:#}"),
    };
    loop {
        let pass = match clip.frames() {
            Ok(pass) => pass,
            Err(error) => return log::warn!("clip: cannot play {name}: {error:#}"),
        };
        let mut any = false;
        for frame in pass {
            let frame = match frame {
                Ok(frame) => frame,
                Err(error) => return log::warn!("clip: cannot decode {name}: {error:#}"),
            };
            let Some(pixels) = RgbaImage::from_raw(frame.width, frame.height, frame.bgra) else {
                return log::warn!("clip: a frame of {name} has the wrong size");
            };
            let image = Arc::new(RenderImage::new(vec![Frame::new(pixels)]));
            if frames.send((image, frame.duration)).is_err() {
                return;
            }
            any = true;
        }
        if !any {
            return log::warn!("clip: {name} decoded to no frames");
        }
    }
}
