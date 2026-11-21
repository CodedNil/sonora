use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use anyhow::Result;
use futures::{StreamExt as _, stream};
use gpui::{App, Context, Entity, Task};
use music::connect::{
    Collection, Command, Connect, Device, Event, Invite, Jam, Naming, PREVIOUS, QueuedTrack,
    Request, Roster, Snapshot, Start, Transport, UPCOMING,
};
use music::{Contributor, MusicApi, Track};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::queue::{PlaybackOwner, QueueStart, Reach, RemoteWindow};
use crate::{
    AppSettings, ConnectName, Io, Library, Origin, Outcome, Playback, Queue, QueueKind, Session,
    SessionEvent, Toasts, Whence,
};

/// Resolving a queue should not fan out into a request for every track at once.
const QUEUE_READS: usize = 8;
const ADOPTION_RETRY: Duration = Duration::from_secs(3);

/// The owner of playback while its queue is being resolved.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Adoption {
    Native(u64),
    /// Keep the in-flight load when the phone repeats the same queue report.
    Remote(u64),
}

/// The provider's device network as the app sees it: this app listed as a device, playback
/// reported to the account's other apps, their commands carried out, and the account's other
/// devices to hand playback to. Only a provider that has such a network gives it anything to do.
pub struct Devices {
    playback: Entity<Playback>,
    queue: Entity<Queue>,
    library: Entity<Library>,
    settings: Entity<AppSettings>,
    session: Entity<Session>,
    io: Io,
    owner: Rc<Cell<PlaybackOwner>>,
    events: Option<Task<()>>,
    asked: Option<String>,
    network: DeviceNetwork,
}

/// State and in-flight operations scoped to one provider connection.
#[derive(Default)]
struct DeviceNetwork {
    link: Option<Arc<Connect>>,
    /// Whether this app is on the device list, as far as the link was told.
    shown: bool,
    observing: bool,
    /// The track whose existing Sonora queue is being handed to Soloist.
    local_start: Option<String>,
    /// The name last handed to the link.
    named: Option<Naming>,
    roster: Roster,
    /// The group session the listener joined, if any.
    jam: Option<Jam>,
    /// The group session a link named, which the listener is being asked to join.
    invite: Option<Invite>,
    /// Whether the listener started a Jam that is not up yet, so its code shows once it is.
    awaiting_code: bool,
    /// Whether the code that opens the Jam's link on a phone is on show.
    code: bool,
    /// Whether the dialog that takes a Jam link to join is open.
    joining: bool,
    /// Whether a leave is on its way. The Jam is off screen already, so updates to it that come
    /// in before the leave is through are dropped.
    leaving: bool,
    /// People who queued songs and are not in the Jam's member list, read from their profiles,
    /// by the id the queue names them by.
    people: HashMap<String, Arc<Contributor>>,
    /// Whether this app took playback, which only the listener pressing play does. A track
    /// restored paused at launch must not pull playback off another device.
    claimed: bool,
    sent: Option<Arc<Snapshot>>,
    changing: Option<(Task<()>, Option<Adoption>)>,
    retry_adoption_at: Option<Instant>,
    queuing: HashMap<String, Task<()>>,
    liking: HashMap<String, Task<()>>,
}

impl Devices {
    pub fn new(
        playback: Entity<Playback>,
        queue: Entity<Queue>,
        library: Entity<Library>,
        settings: Entity<AppSettings>,
        session: Entity<Session>,
        io: Io,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&session, |this, _, event, cx| match event {
            SessionEvent::LocalChanged => {}
            SessionEvent::SignedIn
            | SessionEvent::SignedOut
            | SessionEvent::Reconnected
            | SessionEvent::WentOffline => this.relink(cx),
        })
        .detach();
        cx.observe(&playback, |this, _, cx| this.publish(cx))
            .detach();
        // the report to the other devices also follows the position as it moves
        let ticks = playback.read(cx).ticks().clone();
        cx.observe(&ticks, |this, _, cx| {
            if !this.network.observing {
                this.publish(cx);
            }
        })
        .detach();
        cx.observe(&queue, |this, _, cx| this.publish(cx)).detach();
        cx.observe(&settings, |this, _, cx| this.apply(cx)).detach();

        let this = cx.weak_entity();
        playback.update(cx, |playback, _| playback.set_devices(this));
        let owner = queue.read(cx).owner();
        let mut devices = Self {
            playback,
            queue,
            library,
            settings,
            session,
            io,
            owner,
            events: None,
            asked: None,
            network: DeviceNetwork::default(),
        };
        devices.relink(cx);
        devices
    }

    /// Whether the signed-in provider has a device network, wanted or not.
    pub fn supported(&self) -> bool {
        self.network.link.is_some()
    }

    /// Whether the signed-in provider has a device network and the listener wants it.
    pub fn available(&self) -> bool {
        self.network.link.is_some() && self.network.shown
    }

    /// Whether the device network has group sessions to start and join.
    pub fn jams(&self) -> bool {
        !self.network.observing && self.network.link.as_ref().is_some_and(|link| link.jams())
    }

    /// Whether this app's playback is what the account's other devices are being told about.
    pub fn publishing(&self) -> bool {
        self.available() && self.network.sent.is_some()
    }

    /// The account's other devices.
    pub fn devices(&self) -> &[Device] {
        &self.network.roster.devices
    }

    /// The other device that has playback, if one does. `None` once this app took playback, even
    /// before the network's next roster stops naming the device it came from.
    pub fn elsewhere(&self) -> Option<&Device> {
        self.network.roster.active.as_ref().filter(|device| {
            let native = self.is_native(device);
            (self.network.shown || native)
                && !self.network.claimed
                && if native {
                    self.adopting()
                } else {
                    !self.owner.get().native
                }
        })
    }

    pub(crate) fn is_native(&self, device: &Device) -> bool {
        self.network.observing
            && self
                .network
                .link
                .as_ref()
                .and_then(|link| link.native_device())
                .as_deref()
                == Some(device.id.as_str())
    }

    /// Whether the current queue load projects an active device into Sonora.
    fn adopting(&self) -> bool {
        matches!(self.network.changing, Some((_, Some(_))))
    }

    /// The group session the listener is in, if any.
    pub fn jam(&self) -> Option<&Jam> {
        self.network.jam.as_ref()
    }

    /// The group session a link named, while the listener is asked whether to join it.
    pub fn invite(&self) -> Option<&Invite> {
        self.network.invite.as_ref()
    }

    /// Looks up the group session a shared link names. It shows up on `invite()` to be accepted
    /// or dismissed. A link asked about before the device network is up waits for it, and one
    /// asked about while the listener keeps the app off the device list only gets a toast.
    pub fn peek(&mut self, link: &str, cx: &mut Context<Self>) {
        let Some(link_to) = &self.network.link else {
            self.asked = Some(link.to_owned());
            return;
        };
        if !self.network.shown {
            return Toasts::show(Outcome::Info, "toast-jam-needs-connect", cx);
        }
        link_to.send(Request::Peek(link.to_owned()));
    }

    /// Joins the group session the invite names.
    pub fn accept(&mut self, cx: &mut Context<Self>) {
        if let (Some(invite), Some(link)) = (self.network.invite.take(), &self.network.link) {
            link.send(Request::Join(invite.link));
            self.network.leaving = false;
        }
        cx.notify();
    }

    /// Drops the invite without joining.
    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.network.invite = None;
        cx.notify();
    }

    /// Removes a member, by `JamMember::id`, from the group session the listener owns.
    pub fn kick(&self, member: &str) {
        if let Some(link) = &self.network.link {
            link.send(Request::Kick(member.to_owned()));
        }
    }

    /// Starts a group session on this app. Its link to share is on `jam()` once it is up, and its
    /// code is shown then.
    pub fn start_jam(&mut self) {
        if let Some(link) = &self.network.link {
            link.send(Request::Start);
            self.network.awaiting_code = true;
            self.network.leaving = false;
        }
    }

    /// Whether the code of the Jam's link is on show, which only a Jam with a link can have.
    pub fn code_shown(&self) -> bool {
        self.network.code
            && self
                .network
                .jam
                .as_ref()
                .is_some_and(|jam| jam.link.is_some())
    }

    /// Shows or hides the code of the Jam's link.
    pub fn show_code(&mut self, shown: bool, cx: &mut Context<Self>) {
        self.network.code = shown;
        cx.notify();
    }

    /// Whether the dialog that takes a Jam link to join is open.
    pub fn join_shown(&self) -> bool {
        self.network.joining
    }

    /// Opens or closes the dialog that takes a Jam link to join.
    pub fn show_join(&mut self, shown: bool, cx: &mut Context<Self>) {
        self.network.joining = shown;
        cx.notify();
    }

    /// Leaves the group session, or ends it when the listener started it.
    /// Takes the Jam off screen at once, says it is over, and asks the provider to follow. A
    /// refusal brings it back with a toast.
    pub fn leave(&mut self, cx: &mut Context<Self>) {
        let Some(link) = &self.network.link else {
            return;
        };
        link.send(Request::Leave);
        let owned = self.network.jam.as_ref().is_some_and(|jam| jam.owner);
        let said = match owned {
            true => "toast-jam-ended",
            false => "toast-jam-left",
        };
        Toasts::show(Outcome::Done, said, cx);
        self.network.leaving = true;
        self.network.jam = None;
        self.network.code = false;
        cx.notify();
    }

    /// The listener as the Jam's members list them, so a song they queue names them.
    pub(crate) fn you(&self) -> Option<Arc<Contributor>> {
        let member = self.jam()?.members.iter().find(|member| member.you)?;
        self.known(&member.id)
    }

    /// The person a provider id names, by the Jam's members or a profile already read, when
    /// either knows them.
    fn known(&self, id: &str) -> Option<Arc<Contributor>> {
        let member = self
            .network
            .jam
            .iter()
            .flat_map(|jam| &jam.members)
            .find(|member| member.id == id || member.username == id);
        match member {
            Some(member) => Some(Arc::new(Contributor {
                id: member.id.clone(),
                name: member.name.clone(),
                avatar: member.image.clone(),
            })),
            None => self.network.people.get(id).cloned(),
        }
    }

    /// Moves playback onto this app from whichever device has it.
    pub fn transfer_here(&self) {
        let Some(link) = &self.network.link else {
            return;
        };
        let target = match self.network.observing {
            true => link.native_device(),
            false => Some(link.device().to_owned()),
        };
        if let Some(target) = target {
            link.send(Request::Transfer(target));
        }
    }

    /// Moves playback onto another device.
    pub fn transfer(&self, device: &str) {
        if let Some(link) = &self.network.link {
            link.send(Request::Transfer(device.to_owned()));
        }
    }

    /// Sends a command to another device.
    pub fn control(&self, device: &str, command: Command) {
        if let Some(link) = &self.network.link {
            link.send(Request::Control(device.to_owned(), command));
        }
    }

    /// Changes the device's volume and shows the requested level until its next report arrives.
    /// The caller notifies playback, since playback may already be updating when this runs.
    pub(crate) fn set_volume(&mut self, device: &str, level: f32, cx: &mut Context<Self>) {
        self.control(device, Command::Volume(level));
        if let Some(elsewhere) = self
            .network
            .roster
            .active
            .as_mut()
            .filter(|e| e.id == device)
        {
            elsewhere.volume = level;
        }
        cx.notify();
    }

    /// The id of the song that plays or loads here.
    fn current_id<'a>(&self, cx: &'a App) -> Option<&'a str> {
        self.queue
            .read(cx)
            .current()
            .and_then(|track| track.id.as_deref())
    }

    /// Has playback act on this app's own engine, as another device asked, rather than steer
    /// whichever device has playback.
    fn here(
        &self,
        cx: &mut Context<Self>,
        act: impl FnOnce(&mut Playback, &mut Context<Playback>),
    ) {
        self.playback
            .update(cx, |playback, cx| playback.locally(cx, act));
    }

    /// Takes the device network of the signed-in provider, or lets go of the last one when
    /// there is none or the session was replaced.
    fn relink(&mut self, cx: &mut Context<Self>) {
        let link = self
            .session
            .read(cx)
            .client()
            .and_then(|client| client.connect());
        let same = match (&self.network.link, &link) {
            (Some(old), Some(new)) => Arc::ptr_eq(old, new),
            (None, None) => true,
            _ => false,
        };
        if same {
            return;
        }

        self.here(cx, |playback, cx| playback.release_external(cx));
        self.events = None;
        self.reset(link);
        if let Some(events) = self.network.link.as_ref().and_then(|link| link.events()) {
            self.events = Some(self.listen(events, cx));
        }
        self.apply(cx);
        if let Some(asked) = self.asked.take()
            && self.network.link.is_some()
        {
            self.peek(&asked, cx);
        }
        cx.notify();
    }

    /// Drops everything owned by the previous provider connection.
    fn reset(&mut self, link: Option<Arc<Connect>>) {
        self.owner.set(PlaybackOwner {
            native: false,
            ..self.owner.get()
        });
        self.network = DeviceNetwork {
            link,
            ..Default::default()
        };
    }

    /// Keeps account discovery available while Soloist owns the visible Connect device.
    pub(crate) fn apply(&mut self, cx: &mut Context<Self>) {
        let Some(link) = self.network.link.clone() else {
            return;
        };
        let settings = self.settings.read(cx);
        let wanted = settings.spotify_connect();
        let observing = self.session.read(cx).offers_soloist() && music::soloist::ready();
        let naming = match settings.spotify_connect_name() {
            ConnectName::Sonora => Naming::App,
            ConnectName::Both => Naming::AppOnComputer,
            ConnectName::Computer => Naming::Computer,
            ConnectName::Custom => {
                Naming::Custom(settings.spotify_connect_custom_name().to_owned())
            }
        };
        if self.network.named.as_ref() != Some(&naming) {
            link.send(Request::Rename(naming.clone()));
            self.network.named = Some(naming);
        }
        if wanted == self.network.shown && observing == self.network.observing {
            return;
        }
        self.network.shown = wanted;
        self.network.observing = observing;
        self.network.retry_adoption_at = None;
        if observing {
            self.network.sent = None;

            self.network.claimed = false;
        }
        if !wanted
            && !(observing
                && (self.owner.get().native
                    || self
                        .network
                        .roster
                        .active
                        .as_ref()
                        .is_some_and(|active| self.is_native(active))))
        {
            if self.adopting() {
                self.network.changing = None;
            }
            self.here(cx, |playback, cx| playback.release_external(cx));
        }
        link.send(match observing {
            true => Request::Observe,
            false => Request::Enable(wanted),
        });
        if wanted || observing {
            self.publish(cx);
        } else {
            let named = self.network.named.take();
            self.reset(Some(link));
            self.network.named = named;
        }
        cx.notify();
    }

    fn listen(&mut self, mut events: UnboundedReceiver<Event>, cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async move |this, cx| {
            while let Some(event) = events.recv().await {
                if this
                    .update(cx, |this, cx| {
                        let queued = std::iter::once(event)
                            .chain(std::iter::from_fn(|| events.try_recv().ok()));
                        this.on_events(queued, cx);
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
    }

    /// Applies already-delivered events together without delaying commands or scheduling a timer.
    fn on_events(&mut self, events: impl Iterator<Item = Event>, cx: &mut Context<Self>) {
        let mut listed = false;
        let mut playlists = HashSet::new();
        for event in events {
            match event {
                Event::Roster(roster) => self.set_roster(roster, cx),
                Event::Command(command) => self.run(command, cx),
                Event::Liked { track, liked } => self.liked(track, liked, cx),
                Event::LibraryChanged => listed = true,
                Event::PlaylistChanged(id) => {
                    playlists.insert(id);
                }
                Event::Jam(mut jam) => {
                    if self.network.leaving {
                        if jam.is_some() {
                            continue;
                        }
                        self.network.leaving = false;
                    }
                    // an update to a Jam the listener owns can come without its link or its owner,
                    // which would take the code off screen and bring it back with its entrance played
                    // again, so both carry over from what was known
                    if let (Some(jam), Some(known)) = (jam.as_mut(), self.network.jam.as_ref())
                        && known.owner
                        && jam.link.is_none()
                    {
                        jam.owner = true;
                        jam.link = known.link.clone();
                    }
                    // a Jam the listener started shows its code once, as it comes up
                    if self.network.awaiting_code
                        && jam.as_ref().is_some_and(|jam| jam.link.is_some())
                    {
                        self.network.awaiting_code = false;
                        self.network.code = true;
                    }
                    if jam.is_none() {
                        self.network.code = false;
                    }
                    self.network.jam = jam;
                    cx.notify();
                }
                Event::Invite(invite) => {
                    if invite.is_none() {
                        Toasts::show(Outcome::Failed, "toast-jam-unreadable", cx);
                    }
                    self.network.invite = invite;
                    cx.notify();
                }
                Event::LeaveFailed => {
                    self.network.leaving = false;
                    Toasts::show(Outcome::Failed, "toast-jam-leave-failed", cx);
                }
                Event::JamFailed => {
                    self.network.awaiting_code = false;
                    Toasts::show(Outcome::Failed, "toast-jam-failed", cx);
                }
            }
        }
        if listed || !playlists.is_empty() {
            self.library.update(cx, |library, cx| {
                library.changed_elsewhere(playlists, listed, cx)
            });
        }
    }

    fn set_roster(&mut self, roster: Roster, cx: &mut Context<Self>) {
        let before = std::mem::replace(&mut self.network.roster, roster);
        let active = self.network.roster.active.as_ref();
        let native = active.is_some_and(|device| self.is_native(device));
        let changed =
            before.active.as_ref().map(|device| &device.id) != active.map(|device| &device.id);
        if changed || native != self.owner.get().native {
            self.owner.set(PlaybackOwner {
                native,
                ..self.owner.get()
            });
            if self.adopting() {
                self.network.changing = None;
            }
            if self.playback.read(cx).is_external() {
                self.queue.update(cx, |queue, _| queue.invalidate_undo());
            }
            self.network.retry_adoption_at = None;
            self.network.sent = None;
            if !native {
                self.network.local_start = None;
            }
        } else if let (Some(before), Some(active)) = (&before.active, active)
            && before.volume != active.volume
        {
            self.playback.update(cx, |playback, _| {
                playback.reported_steered_volume(&active.id, active.volume)
            });
        }
        if let Some(state) = self.network.roster.playback.clone() {
            if !native
                && self.network.claimed
                && self.network.sent.is_none()
                && state.transport.playing
                && (changed || !before.playback.is_some_and(|state| state.transport.playing))
            {
                self.network.claimed = false;
                self.here(cx, |playback, cx| playback.pause(cx));
            }

            self.apply_report(state, native, cx);
        } else if self.network.roster.active.is_none() {
            self.here(cx, |playback, cx| playback.release_external(cx));
        }
        self.redraw(cx);
    }

    /// A desktop selection can reset Soloist's context. Publish the desktop queue for this load.
    pub(crate) fn local_start(&mut self, id: String) {
        if self.network.observing {
            self.network.local_start = Some(id);
            self.network.sent = None;
        }
    }

    /// Applies transport immediately and reconciles full reports against the active queue.
    fn apply_report(&mut self, mut state: Arc<Snapshot>, native: bool, cx: &mut Context<Self>) {
        if !native && self.elsewhere().is_none() {
            return;
        }
        let id = state.track_id();
        let playback = self.playback.read(cx);
        let current = playback.track().and_then(|track| track.id.as_deref());
        if native
            && ((current.is_some_and(music::is_local_id) && !state.transport.playing)
                || (playback.is_loading()
                    && self
                        .network
                        .local_start
                        .as_deref()
                        .is_some_and(|wanted| Some(wanted) != id)))
        {
            return;
        }
        let context_changed = state.context.is_some() && self.context(cx) != state.context;
        let moved = current != id;
        let was_external = playback.is_external();
        let queue_aligned = self.current_id(cx) == id;
        self.here(cx, |playback, cx| {
            playback.set_external(!native, cx);
            if moved
                && !context_changed
                && let Some(id) = id
            {
                playback.adopt_queue_position(id, &state.transport, native, cx);
            }
            if !native || was_external || queue_aligned {
                playback.sync_device(&state.transport, id, cx);
            }
        });
        let Some(id) = id else {
            if !self.queue.read(cx).sections.is_empty() {
                self.queue
                    .update(cx, |queue, cx| queue.clear_projection(cx));
            }
            return;
        };
        let local = self.snapshot(cx);
        let aligned = local
            .as_ref()
            .is_some_and(|local| local.track_id() == Some(id));
        let same_order = aligned
            && local
                .as_ref()
                .is_some_and(|local| same_window(local, &state));
        if native {
            if self.network.local_start.as_deref() == Some(id) && aligned {
                if same_order {
                    self.network.local_start = None;
                }
                self.publish(cx);
                return;
            }
            self.network.local_start = None;
        }
        let restart = !aligned || context_changed || (!native && !was_external);
        if same_order && !restart {
            if self.adopting() {
                self.network.changing = None;
            }
            if native {
                self.network.sent = local.map(Arc::new);
            }
            return;
        }
        let key = state.queue_fingerprint();
        let load = match native {
            true => Adoption::Native(key),
            false => Adoption::Remote(key),
        };
        if matches!(self.network.changing, Some((_, Some(pending))) if pending == load)
            || self
                .network
                .retry_adoption_at
                .is_some_and(|at| Instant::now() < at)
        {
            return;
        }
        if native
            && !restart
            && let Some(local) = local
        {
            Arc::make_mut(&mut state).track = local.track;
        }
        self.load_start(
            Start {
                state,
                preserve_queue: true,
            },
            restart,
            Some(load),
            cx,
        );
    }

    /// Tells the views, and everything that follows playback such as the lyrics, that what the
    /// other device plays has changed or moved on.
    fn redraw(&self, cx: &mut Context<Self>) {
        cx.notify();
        self.playback.update(cx, |_, cx| cx.notify());
    }

    /// Carries out what another device asked of this one.
    fn run(&mut self, command: Command, cx: &mut Context<Self>) {
        match command {
            Command::Play => self.here(cx, |playback, cx| playback.resume(cx)),
            Command::Pause => self.here(cx, |playback, cx| playback.pause(cx)),
            Command::Next => self.here(cx, |playback, cx| playback.next(cx)),
            Command::Previous => self.here(cx, |playback, cx| playback.previous_track(cx)),
            Command::Pick(uid) => {
                if !self.playback.read(cx).is_external()
                    && self.queue.read(cx).current_uid() != Some(&uid)
                {
                    self.here(cx, |playback, cx| playback.play_queue(&uid, cx));
                }
            }
            Command::Seek(at) => self.here(cx, |playback, cx| playback.seek(at, cx)),
            Command::Volume(level) => self.here(cx, |playback, cx| playback.set_volume(level, cx)),
            Command::Shuffle(on) => self.queue.update(cx, |queue, cx| queue.set_shuffle(on, cx)),
            Command::Repeat(mode) => self.here(cx, |playback, cx| playback.set_repeat(mode, cx)),
            Command::Enqueue { id, added_by } => self.enqueue(id, added_by, cx),
            Command::Start(start) => self.load_start(start, true, None, cx),
            Command::SetQueue { base, upcoming } => self.set_queue(base, upcoming, cx),
            Command::Released => {
                // another device took over, so this one stops without calling playback back
                self.network.changing = None;
                self.network.claimed = false;
                self.here(cx, |playback, cx| playback.pause(cx));
                self.publish(cx);
            }
        }
    }

    fn enqueue(&mut self, id: String, added_by: Option<String>, cx: &mut Context<Self>) {
        let Some(client) = self.session.read(cx).client() else {
            return;
        };
        let key = id.clone();
        let slot = id.clone();
        let stranger = added_by.clone().filter(|adder| self.known(adder).is_none());
        let task = self.io.load(
            async move {
                let track = client.track(&id).await?;
                let adder = match stranger {
                    Some(id) => person(client.as_ref(), id).await,
                    None => None,
                };
                Ok::<_, anyhow::Error>((track, adder))
            },
            move |this, found, cx| {
                this.network.queuing.remove(&key);
                match found {
                    Ok((mut track, person)) => {
                        this.network.people.extend(person);
                        track.added_by = added_by.as_deref().and_then(|id| this.known(id));
                        this.here(cx, |playback, cx| playback.enqueue(track, cx))
                    }
                    Err(error) => log::warn!("connect: cannot queue {key}: {error:#}"),
                }
            },
            cx,
        );
        self.network.queuing.insert(slot, task);
    }

    /// Shows a like made in another of the account's apps, reading the track in first when it
    /// was liked rather than unliked.
    fn liked(&mut self, id: String, liked: bool, cx: &mut Context<Self>) {
        // an unlike also calls off a like of the same track still being read in
        self.network.liking.remove(&id);
        if !liked {
            return self
                .library
                .update(cx, |library, cx| library.liked_elsewhere(&id, None, cx));
        }
        // a like this app made comes back the same way, with nothing left to show
        if self.library.read(cx).saved(&id) {
            return;
        }
        let Some(client) = self.session.read(cx).client() else {
            return;
        };
        let slot = id.clone();
        let wanted = id.clone();
        let task = self.io.load(
            async move { client.track(&wanted).await },
            move |this, found, cx| {
                this.network.liking.remove(&id);
                match found {
                    Ok(track) => this.library.update(cx, |library, cx| {
                        library.liked_elsewhere(&id, Some(track), cx)
                    }),
                    Err(error) => {
                        log::warn!("connect: cannot read the liked track {id}: {error:#}")
                    }
                }
            },
            cx,
        );
        self.network.liking.insert(slot, task);
    }

    /// Accepts queue edits made in another Spotify client without resetting the current song.
    /// The history stays as it is here, since an edit only ever changes what is still to play.
    fn set_queue(&mut self, base: u64, upcoming: Vec<QueuedTrack>, cx: &mut Context<Self>) {
        let Some(state) = self.snapshot(cx) else {
            return;
        };
        // the edit was made on the last report, and one made on an older queue would undo what
        // changed here since, so it gives way to the report of what this queue holds now
        if base != state.queue_fingerprint() {
            log::debug!("connect: dropping a queue edit made on an older report");
            self.network.sent = None;
            self.publish(cx);
            return;
        }
        self.load_start(
            Start::window(Snapshot { upcoming, ..state }),
            false,
            None,
            cx,
        );
    }

    /// Resolves one Connect snapshot, either to start playback or to adopt the engine's playback.
    fn load_start(
        &mut self,
        start: Start,
        restart: bool,
        adoption: Option<Adoption>,
        cx: &mut Context<Self>,
    ) {
        let adopt = adoption.is_some();
        let native = matches!(adoption, Some(Adoption::Native(_)));
        let external = matches!(adoption, Some(Adoption::Remote(_)));
        if native {
            self.network.claimed = false;
        }
        let Some(client) = self.session.read(cx).client() else {
            return;
        };
        let state = start.state.transport;
        let reach = match restart {
            true => Reach::Handed,
            false => Reach::Reported {
                past: PREVIOUS.max(start.state.previous.len()),
                upcoming: UPCOMING,
            },
        };
        let queue = self.queue.read(cx);
        let known = queue
            .tracks()
            .filter_map(|track| Some((track.id.clone()?, track.clone())))
            .collect();
        let revision = self.queue.read(cx).revision();
        let people = start
            .state
            .tracks()
            .filter_map(|track| track.added_by.as_ref())
            .filter_map(|id| self.known(id).map(|person| (id.clone(), person)))
            .collect();
        let task = self.io.load(
            load(client, start, known, people, adopt, restart),
            move |this, loaded, cx| {
                this.network.changing = None;
                if this.queue.read(cx).revision() != revision {
                    if adopt {
                        this.redraw(cx);
                    }
                    this.publish(cx);
                    return;
                }
                let (start, origin, window) = match loaded {
                    Ok(loaded) => loaded,
                    Err(error) => {
                        log::warn!("connect: cannot apply playback change: {error:#}");
                        if external {
                            this.network.retry_adoption_at = Some(Instant::now() + ADOPTION_RETRY);
                        }
                        this.network.sent = None;

                        match adopt {
                            true => this.network.sent = this.snapshot(cx).map(Arc::new),
                            false => this.publish(cx),
                        }
                        cx.notify();
                        return;
                    }
                };
                this.network.retry_adoption_at = None;
                let retained = if restart && native && !this.playback.read(cx).is_external() {
                    this.queue
                        .read(cx)
                        .upcoming()
                        .filter(|track| track.id.as_deref().is_some_and(music::is_local_id))
                        .cloned()
                        .collect()
                } else {
                    Vec::new()
                };
                let install = |queue: &mut Queue, cx: &mut Context<Queue>| {
                    queue.install_device_queue(start, origin, window, reach, retained, cx)
                };
                if restart {
                    this.network.claimed = !adopt;
                    let queue = this.queue.clone();
                    this.here(cx, |playback, cx| {
                        playback.set_external(external, cx);
                        if native {
                            playback.report_modes(state.shuffle, state.repeat, cx);
                        } else if !external {
                            if let Some(shuffle) = state.shuffle {
                                queue.update(cx, |queue, cx| queue.set_shuffle(shuffle, cx));
                            }
                            if let Some(repeat) = state.repeat {
                                playback.set_repeat(repeat, cx);
                            }
                        }
                        if let Some(track) = queue.update(cx, install) {
                            playback.play_installed(
                                track,
                                state.live_position(),
                                adopt.then_some(&state),
                                cx,
                            );
                        }
                        if !adopt && !state.playing {
                            playback.pause(cx);
                        }
                    });
                } else {
                    this.queue.update(cx, install);
                }
                if adopt {
                    this.network.sent = native.then(|| this.snapshot(cx)).flatten().map(Arc::new);
                }
                cx.notify();
            },
            cx,
        );
        self.network.changing = Some((task, adoption));
    }

    /// Publishes one local snapshot through either the built-in player or Soloist's device.
    fn publish(&mut self, cx: &mut Context<Self>) {
        if self.network.changing.is_some() || !(self.network.shown || self.network.observing) {
            return;
        }
        let Some(link) = self.network.link.clone() else {
            return;
        };
        let playback = self.playback.read(cx);
        let current = self.current_id(cx);
        let playing = playback.track().and_then(|track| track.id.as_deref());
        // Queue navigation and engine loads notify separately. Wait for a coherent pair
        // before publishing, since withdrawing here would end the Connect session.
        if !playback.is_external() && current.is_some() && playing != current {
            return;
        }
        let id = current.filter(|_| !playback.is_external());
        if let Some(id) = id
            && !self.network.observing
            && !self.network.claimed
            && playback.wants_playing()
        {
            self.network.claimed = true;
            // A local file has no Connect player to claim playback through.
            if music::is_local_id(id)
                && let Some(remote) = self.network.roster.active.as_ref().filter(|_| {
                    self.network
                        .roster
                        .playback
                        .as_ref()
                        .is_some_and(|state| state.transport.playing)
                })
            {
                self.control(&remote.id, Command::Pause);
            }
        }
        let Some(now) = id
            .filter(|id| {
                !music::is_local_id(id) && (self.network.observing || self.network.claimed)
            })
            .and_then(|_| self.snapshot(cx))
        else {
            if self.network.observing {
                self.network.sent = None;
            } else if self.network.sent.take().is_some() {
                link.send(Request::Publish(None));
                cx.notify();
            }
            return;
        };
        if self.network.observing {
            let roster = &self.network.roster;
            let Some(remote) = roster
                .active
                .as_ref()
                .filter(|device| self.is_native(device))
            else {
                return;
            };
            let Some(reported) = roster
                .playback
                .as_ref()
                .filter(|state| state.track_id() == now.track_id())
            else {
                return;
            };
            if self
                .network
                .sent
                .as_ref()
                .is_some_and(|last| last.same_queue(&now))
            {
                return;
            }
            let changed = !same_window(reported, &now);
            let now = Arc::new(now);
            self.network.sent = Some(now.clone());
            if changed {
                link.send(Request::PublishQueue(remote.id.clone(), now));
            }
        } else if self
            .network
            .sent
            .as_ref()
            .is_none_or(|sent| now.volume != sent.volume || now.playback_changed(sent))
        {
            let now = Arc::new(now);
            self.network.sent = Some(now.clone());
            link.send(Request::Publish(Some(now)));
            cx.notify();
        }
    }

    fn context(&self, cx: &App) -> Option<Collection> {
        let queue = self.queue.read(cx);
        queue
            .context_origin()
            .or_else(|| queue.origin())
            .and_then(collection_of)
    }

    /// The local queue and playback as a snapshot, regardless of who has playback.
    fn snapshot(&self, cx: &App) -> Option<Snapshot> {
        let playback = self.playback.read(cx);
        let queue = self.queue.read(cx);
        let track = queue.current()?;
        let context = self.context(cx);
        let mut previous: Vec<_> = queue.connect_previous().rev().take(PREVIOUS).collect();
        previous.reverse();
        Some(Snapshot {
            track: queue.connect_current(),
            previous,
            upcoming: queue
                .connect_upcoming(self.network.observing)
                .take(UPCOMING)
                .collect(),
            index: context.is_some().then(|| queue.place()).flatten(),
            context,
            volume: playback.volume(),
            transport: Transport {
                playing: playback.wants_playing(),
                position: playback.live_position(),
                duration: track.duration,
                shuffle: Some(queue.shuffle()),
                repeat: Some(playback.repeat()),
                stamp: Some(SystemTime::now()),
            },
        })
    }
}

/// The collection a track was queued from, as the device network names it. `None` for one it has
/// no name for, such as an artist or a radio.
pub(crate) fn collection_of(origin: &Origin) -> Option<Collection> {
    match origin.whence {
        Whence::Album => Some(Collection::Album(origin.id.clone())),
        Whence::Playlist => Some(Collection::Playlist(origin.id.clone())),
        Whence::Saved => Some(Collection::Saved),
        Whence::Radio => Some(Collection::Radio(origin.id.clone())),
        _ => None,
    }
}

/// The profile of someone a queue names by an id the Jam's members do not list, keyed by that id.
async fn person(client: &dyn MusicApi, id: String) -> Option<(String, Arc<Contributor>)> {
    match client.user(&id).await {
        Ok(user) => Some((
            id,
            Arc::new(Contributor {
                id: user.id,
                name: user.name,
                avatar: user.avatar,
            }),
        )),
        Err(error) => {
            log::warn!("connect: cannot read who queued a track: {error:#}");
            None
        }
    }
}

/// Reads in what a start names: the collection if it has one the track is in, otherwise the
/// track and the tracks that follow it.
async fn load(
    client: Arc<dyn MusicApi>,
    start: Start,
    mut known: HashMap<String, Track>,
    people: HashMap<String, Arc<Contributor>>,
    adopt: bool,
    restart: bool,
) -> Result<(Option<QueueStart>, Option<Origin>, Option<RemoteWindow>)> {
    let Start {
        state: start,
        preserve_queue,
    } = start;
    let context = start.context.as_ref().filter(|_| restart);
    let origin = context.map(|context| match context {
        Collection::Album(id) => Origin::album(id),
        Collection::Playlist(id) => Origin::playlist(id),
        Collection::Saved => Origin::saved(),
        Collection::Radio(id) => Origin::radio(id),
    });
    let tracks = match context.filter(|_| !adopt) {
        Some(Collection::Album(id)) => client.album_tracks(id).await?,
        Some(Collection::Playlist(id)) => client.playlist_tracks(id).await?,
        Some(Collection::Saved) => client.saved_tracks().await?,
        Some(Collection::Radio(id)) => {
            let (mut tracks, _) = client.track_radio(id, None).await.unwrap_or_default();
            tracks.retain(|track| track.playable && track.id.as_deref() != Some(id));
            tracks.insert(0, client.track(id).await?);
            tracks
        }
        None => Vec::new(),
    };

    known.extend(
        tracks
            .iter()
            .filter_map(|track| Some((track.id.clone()?, track.clone()))),
    );
    let accepts = |track: &Track| {
        start
            .track
            .as_ref()
            .is_none_or(|listed| track.id.as_ref() == Some(&listed.id))
    };
    let index = start
        .index
        .filter(|at| tracks.get(*at).is_some_and(accepts))
        .or_else(|| tracks.iter().position(accepts));
    let resolved = if preserve_queue || index.is_none() {
        resolve_queue(&*client, &mut known, people, &start, adopt && restart).await
    } else {
        Vec::new()
    };
    let mut rows = resolved.into_iter();
    let past: Vec<_> = rows.by_ref().take(start.previous.len()).flatten().collect();
    let current = start.track.as_ref().and_then(|_| rows.next().flatten());
    let upcoming: Vec<_> = rows.flatten().collect();
    let kind = start.track.as_ref().map_or(QueueKind::Context, kind_of);
    let start = match (restart, index) {
        (false, _) => None,
        (true, Some(index)) => Some(QueueStart::Collection(tracks, index)),
        (true, None) => {
            let (current, _) = current
                .ok_or_else(|| anyhow::anyhow!("cannot load the transferred current track"))?;
            return Ok((
                Some(QueueStart::Reported(
                    Box::new(current),
                    (past, kind, upcoming),
                )),
                origin,
                None,
            ));
        }
    };
    let window = preserve_queue.then_some((past, kind, upcoming));

    Ok((start, origin, window))
}

/// Fetches each missing song once, concurrently, then reconstructs the requested order.
/// Repeated occurrences share metadata; an unavailable song remains an explicit gap.
async fn resolve_queue(
    client: &dyn MusicApi,
    known: &mut HashMap<String, Track>,
    mut people: HashMap<String, Arc<Contributor>>,
    snapshot: &Snapshot,
    retry_current: bool,
) -> Vec<Option<(Track, QueueKind)>> {
    let rows = snapshot.tracks();
    let mut requested = HashSet::new();
    let missing: Vec<_> = rows
        .clone()
        .map(|row| &row.id)
        .filter(|id| !known.contains_key(*id) && requested.insert(*id))
        .cloned()
        .collect();
    let mut reads = stream::iter(missing)
        .map(|id| async move {
            let mut result = client.track(&id).await;
            if retry_current && result.is_err() && snapshot.track_id() == Some(id.as_str()) {
                tokio::time::sleep(ADOPTION_RETRY).await;
                result = client.track(&id).await;
            }
            (id, result)
        })
        .buffer_unordered(QUEUE_READS);
    while let Some((id, result)) = reads.next().await {
        match result {
            Ok(track) if track.id.as_ref() == Some(&id) => {
                known.insert(id, track);
            }
            result => log::warn!("connect: cannot resolve queued track {id}: {result:?}"),
        }
    }
    let strangers: HashSet<_> = rows
        .clone()
        .filter_map(|row| row.added_by.as_ref())
        .filter(|id| !people.contains_key(*id))
        .cloned()
        .collect();
    let mut profiles = stream::iter(strangers)
        .map(|id| person(client, id))
        .buffer_unordered(QUEUE_READS);
    while let Some(profile) = profiles.next().await {
        people.extend(profile);
    }
    rows.map(|row| {
        let mut track = known.get(&row.id)?.clone();
        track.added_by = row.added_by.as_ref().and_then(|id| people.get(id)).cloned();
        Some((track, kind_of(row)))
    })
    .collect()
}

/// Why a track another device lists is in its play order: queued by hand, picked by the
/// provider to follow the collection, or part of the collection itself.
fn kind_of(row: &QueuedTrack) -> QueueKind {
    match (row.queued, row.suggested) {
        (true, _) => QueueKind::Queued,
        (false, true) => QueueKind::Suggested,
        (false, false) => QueueKind::Context,
    }
}

/// Local and provider occurrence IDs can differ. Compare both sides of the playhead by provenance.
fn same_window(a: &Snapshot, b: &Snapshot) -> bool {
    let same = |a: &QueuedTrack, b: &QueuedTrack| {
        a.id == b.id
            && a.queued == b.queued
            && a.suggested == b.suggested
            && a.added_by == b.added_by
    };
    a.previous.len() == b.previous.len()
        && a.upcoming.len() == b.upcoming.len()
        && a.previous
            .iter()
            .chain(&a.upcoming)
            .zip(b.previous.iter().chain(&b.upcoming))
            .all(|(a, b)| same(a, b))
}
