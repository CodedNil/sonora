//! A provider's remote-control surface: what the app reports to the service as playing, what
//! the service's other apps ask of it, and the account's other devices. Nothing here names a
//! provider, so `state` and `views` use the same channel while each provider owns its worker.

use std::hash::{DefaultHasher, Hash as _, Hasher as _};
use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, SystemTime};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

/// The collection a track plays from, as far as the remote end needs to know it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Collection {
    Album(String),
    Playlist(String),
    /// The listener's saved tracks.
    Saved,
    /// The radio of a track, which goes on with similar songs once the track ends.
    Radio(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Repeat {
    #[default]
    Off,
    All,
    One,
}

/// How the app is named on the account's device list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Naming {
    /// The app's name alone.
    App,
    /// The app's name and the computer's.
    #[default]
    AppOnComputer,
    /// The computer's name alone.
    Computer,
    /// A name the listener chose. An empty one falls back to the app's name and the computer's.
    Custom(String),
}

/// A particular occurrence of a provider track in the queue, stable as it moves through playback.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QueuedTrack {
    pub id: String,
    pub uid: String,
    pub queued: bool,
    /// Whether the provider or the app picked the track itself, to follow what the listener
    /// chose once that runs out.
    pub suggested: bool,
    pub added_by: Option<String>,
}

impl QueuedTrack {
    pub fn new(id: String, queued: bool) -> Self {
        Self {
            id,
            queued,
            suggested: false,
            uid: String::new(),
            added_by: None,
        }
    }
}

/// What the app is playing, including the recent and upcoming queue windows. Track IDs belong
/// to the provider; history is chronological and each occurrence keeps its own provenance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot {
    pub track: Option<QueuedTrack>,
    /// Recent tracks in chronological order, with the most recent last.
    pub previous: Vec<QueuedTrack>,
    pub upcoming: Vec<QueuedTrack>,
    pub context: Option<Collection>,
    /// Where `track` sits in `context`, counted from 0, when that is known.
    pub index: Option<usize>,
    pub transport: Transport,
    /// From 0 to 1.
    pub volume: f32,
}

impl Snapshot {
    pub fn track_id(&self) -> Option<&str> {
        self.track.as_ref().map(|track| track.id.as_str())
    }

    /// The reported queue in playback order, including the current track.
    pub fn tracks(&self) -> impl Iterator<Item = &QueuedTrack> + Clone {
        self.previous
            .iter()
            .chain(&self.track)
            .chain(&self.upcoming)
    }

    /// Fingerprints the queue, including occurrence identifiers and provenance.
    pub fn queue_fingerprint(&self) -> u64 {
        let mut hash = DefaultHasher::new();
        (&self.track, &self.previous, &self.upcoming).hash(&mut hash);
        hash.finish()
    }

    /// Whether both reports describe the same track occurrences and order.
    pub fn same_queue(&self, other: &Self) -> bool {
        self.track == other.track
            && self.previous == other.previous
            && self.upcoming == other.upcoming
    }

    /// Compares reports at the same clock time, ignoring volume.
    pub fn playback_changed(&self, before: &Self) -> bool {
        let now = self.transport;
        let old = before.transport;
        let time = now.stamp.max(old.stamp).unwrap_or(SystemTime::UNIX_EPOCH);
        !self.same_queue(before)
            || self.context != before.context
            || self.index != before.index
            || now.playing != old.playing
            || now.duration != old.duration
            || now.shuffle != old.shuffle
            || now.repeat != old.repeat
            || if now.playing {
                now.position_at(time).abs_diff(old.position_at(time)) > Duration::from_secs(2)
            } else {
                now.position != old.position
            }
    }
}

/// The timing and options needed to apply a report after its queue loads.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Transport {
    pub playing: bool,
    pub position: Duration,
    pub duration: Duration,
    /// When position was measured. Play commands may leave it unset.
    pub stamp: Option<SystemTime>,
    pub shuffle: Option<bool>,
    pub repeat: Option<Repeat>,
}

impl Transport {
    /// Projects the device's last measured position to the current time.
    pub fn live_position(&self) -> Duration {
        self.position_at(SystemTime::now())
    }

    fn position_at(&self, time: SystemTime) -> Duration {
        let moved = match self.playing {
            true => self
                .stamp
                .and_then(|stamp| time.duration_since(stamp).ok())
                .unwrap_or_default(),
            false => Duration::ZERO,
        };
        match self.duration.is_zero() {
            true => self.position + moved,
            false => (self.position + moved).min(self.duration),
        }
    }
}

/// A play request or exact transfer, both using the same playback snapshot as device reports.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Start {
    pub state: std::sync::Arc<Snapshot>,
    /// Uses the supplied history and upcoming order instead of reconstructing the collection.
    pub preserve_queue: bool,
}

impl Start {
    /// Applies the reported history and upcoming order exactly as supplied.
    pub fn window(state: impl Into<Arc<Snapshot>>) -> Self {
        Self {
            state: state.into(),
            preserve_queue: true,
        }
    }
}

/// A request that crosses the wire in either direction: asked of this app by a remote end, or
/// asked of another device through `Request::Control`.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Play,
    Pause,
    Next,
    Previous,
    /// Selects a specific occurrence in the locally published queue.
    Pick(String),
    Seek(Duration),
    /// From 0 to 1.
    Volume(f32),
    Shuffle(bool),
    Repeat(Repeat),
    /// Adds a track to the end of the hand-made queue, with the id of whoever queued it when
    /// the provider says who that is. Sent to another device, `added_by` is left out.
    Enqueue {
        id: String,
        added_by: Option<String>,
    },
    /// Plays a collection or a track list, from this app's side only.
    Start(Start),
    /// Replaces what plays after the current track without restarting playback. History is not
    /// part of an edit.
    SetQueue {
        base: u64,
        upcoming: Vec<QueuedTrack>,
    },
    /// Another device took playback over, so this app has to let go of it.
    Released,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DeviceKind {
    Computer,
    Phone,
    Tablet,
    Speaker,
    Tv,
    Console,
    Car,
    #[default]
    Other,
}

/// One of the account's devices.
#[derive(Clone, Debug, PartialEq)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub kind: DeviceKind,
    /// From 0 to 1.
    pub volume: f32,
}

/// Devices reported by the connection and the one with active playback, if any.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Roster {
    pub devices: Vec<Device>,
    pub active: Option<Device>,
    /// Playback reported by providers whose device worker owns playback state.
    pub playback: Option<Arc<Snapshot>>,
}

/// One person in a group session.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JamMember {
    /// What the provider goes by to name the person, as removing them does.
    pub id: String,
    /// The account name, which songs they queue name them by when it differs from `id`.
    pub username: String,
    pub name: String,
    /// The address of their picture, when they have one.
    pub image: Option<String>,
    /// Whether they own the session.
    pub host: bool,
    /// Whether they are the listener.
    pub you: bool,
}

/// A group listening session the listener has joined, in which the host's playback plays here
/// too.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Jam {
    /// Everyone in it, the listener too, in the order the provider lists them.
    pub members: Vec<JamMember>,
    /// Whether the listener owns the session, which leaving ends for everyone.
    pub owner: bool,
    /// The link that invites others, when the listener owns the session.
    pub link: Option<String>,
}

/// A group session a link names, read before the listener joins it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invite {
    /// The link to join it by, which needs no lookup of its own.
    pub link: String,
    pub jam: Jam,
}

pub enum Event {
    Command(Command),
    Roster(Roster),
    /// Saved collections or the account's playlist list changed in another client.
    LibraryChanged,
    /// A playlist's contents or attributes changed in another client.
    PlaylistChanged(String),
    /// One of the account's apps liked a track, or took the like back. `track` is a provider
    /// track id.
    Liked {
        track: String,
        liked: bool,
    },
    /// The group session changed: joined, updated, or `None` once it ended or was left.
    Jam(Option<Jam>),
    /// What `Request::Peek` found, or `None` when the link names no group session.
    Invite(Option<Invite>),
    /// Starting or joining a group session failed.
    JamFailed,
    /// Leaving or ending the group session failed. The session as it stands follows as a `Jam`.
    LeaveFailed,
}

/// Requests sent to the provider's worker; replies arrive through `Event`.
pub enum Request {
    Enable(bool),
    /// Enables account discovery using a hidden connection while a separate player owns playback.
    Observe,
    Rename(Naming),
    /// Publishing claims playback, so only listener-initiated playback should be reported.
    Publish(Option<std::sync::Arc<Snapshot>>),
    Transfer(String),
    /// Updates history and upcoming tracks on the local playback engine's own device.
    PublishQueue(String, Arc<Snapshot>),
    /// A refused start returns as a command to play locally.
    Control(String, Command),
    Join(String),
    Peek(String),
    Start,
    Leave,
    Kick(String),
}

/// Spotify's Connect queue windows, shared by both playback engines.
pub const UPCOMING: usize = 80;
pub const PREVIOUS: usize = 10;

/// A provider-independent channel to the device worker. Dropping the last handle ends it.
pub struct Connect {
    device: String,
    native: Mutex<Weak<Mutex<Option<String>>>>,
    /// Whether the network has group sessions to start and join.
    jams: bool,
    requests: UnboundedSender<Request>,
    events: Mutex<Option<UnboundedReceiver<Event>>>,
}

impl Connect {
    /// Creates the app handle and the worker's request receiver and event sender.
    pub fn new(device: String) -> (Self, UnboundedReceiver<Request>, UnboundedSender<Event>) {
        let (requests, incoming) = unbounded_channel();
        let (outgoing, events) = unbounded_channel();
        (
            Self {
                device,
                native: Mutex::default(),
                jams: true,
                requests,
                events: Mutex::new(Some(events)),
            },
            incoming,
            outgoing,
        )
    }

    pub fn send(&self, request: Request) {
        self.requests.send(request).ok();
    }

    /// Taken once by the state entity that owns the device list.
    pub fn events(&self) -> Option<UnboundedReceiver<Event>> {
        self.events.lock().ok()?.take()
    }

    pub fn device(&self) -> &str {
        &self.device
    }

    /// Gives a new native player a place to register its device for account discovery.
    pub(crate) fn register_native(&self) -> Arc<Mutex<Option<String>>> {
        let device = Arc::default();
        *self
            .native
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Arc::downgrade(&device);
        device
    }

    /// Reads the device id of the currently registered native player.
    pub fn native_device(&self) -> Option<String> {
        self.native.lock().ok()?.upgrade()?.lock().ok()?.clone()
    }

    /// Marks a network that has no group sessions, so the app offers none.
    pub fn without_jams(mut self) -> Self {
        self.jams = false;
        self
    }

    pub fn jams(&self) -> bool {
        self.jams
    }
}
