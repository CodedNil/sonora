//! Runs the Soloist daemon and translates its WebSocket reports into playback events.
//! The account connection publishes the queue while this bridge controls the audio engine.

use std::ffi::OsStr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Result, ensure};
use futures::{SinkExt as _, StreamExt as _};
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use super::daemon::{RUNNING, follow, launch, stop};
use super::wire::{self, Event, Playback, Status};
use super::{QUALITIES, binary, data, key, paired_account, prefs, recheck};
use crate::connect::Repeat;
use crate::{Normalisation, PlaybackConfig, PlaybackEvent, Quality, TrackSignal};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;
type Outgoing = (&'static str, Value);

const TRACK_PREFIX: &str = "spotify:track:";
/// The most audio the daemon keeps on disk, in megabytes. It keeps everything without a limit.
const CACHE_MB: &str = "512";
/// The files the daemon writes its WebSocket address into once it listens.
const WS_ADDR: &str = "ws.addr";
const WS_PORT: &str = "ws.port";
/// What the daemon decodes every track to.
const RATE: u32 = 44_100;
const CHANNELS: u16 = 2;

/// How long the daemon gets to open its WebSocket.
const STARTUP: Duration = Duration::from_secs(30);
const POLL: Duration = Duration::from_millis(100);
/// How many sign-ins in a row Spotify may turn down before the setup is taken to be broken. One
/// refusal can be a passing fault the next start does not repeat.
const REFUSALS: u8 = 2;

/// What the daemon prints when Spotify turns its sign-in down.
const REFUSED: &str = "login failed";
/// What it prints while it has no sign-in to restore, and also while it is still restoring one.
const UNPAIRED: &str = "waiting for login";

/// Transport control of a Soloist daemon. Dropping it stops the daemon.
pub(crate) struct Engine {
    commands: UnboundedSender<Command>,
}

impl Engine {
    /// Plays a track from `at`, or with `play` off keeps it ready there for `play`.
    pub(crate) fn load(&self, track_id: &str, at: Duration, play: bool) {
        self.ask(Command::Load {
            id: track_id.to_owned(),
            at,
            play,
        });
    }

    pub(crate) fn ask(&self, command: Command) {
        self.commands.send(command).ok();
    }
}

/// Builds the engine and its daemon task for the provider's Tokio runtime.
pub(crate) fn start(
    config: &PlaybackConfig,
    device: Arc<Mutex<Option<String>>>,
) -> (
    Engine,
    UnboundedReceiver<PlaybackEvent>,
    impl Future<Output = ()> + Send + use<>,
) {
    let setup = Setup {
        normalise: config.normalisation,
        gapless: config.gapless,
        quality: config.quality,
    };
    let (commands, asked) = unbounded_channel();
    let (events, reported) = unbounded_channel();
    let brain = Brain {
        setup,
        events,
        device,
        link: None,
        signed_in: false,
        volume: percent(config.gain),
        pending: None,
        current: None,
        opening_seek: None,
        seeking: false,
        position: Duration::ZERO,
        active: false,
        repeat: Repeat::Off,
        refusals: 0,
        refreshed: false,
        dead: false,
    };
    (Engine { commands }, reported, brain.run(asked))
}

/// A transport command sent to the Soloist task.
pub(crate) enum Command {
    Load {
        id: String,
        at: Duration,
        play: bool,
    },
    Play,
    Pause,
    Seek(Duration),
    Gain(f32),
    Repeat(Repeat),
    Skip(bool),
    Shuffle(bool),
}

/// What a daemon is started with, beyond the key.
#[derive(Clone, Copy)]
struct Setup {
    normalise: bool,
    gapless: bool,
    quality: Quality,
}

/// A selection waits for sign-in or an explicit resume before taking the device.
struct Selection {
    id: String,
    position: Duration,
    play: bool,
}

/// The engine's own loop: commands in, the daemon's reports in, events out.
struct Brain {
    setup: Setup,
    events: UnboundedSender<PlaybackEvent>,
    device: Arc<Mutex<Option<String>>>,
    link: Option<Link>,
    /// Whether the daemon is signed in, before which it takes no command.
    signed_in: bool,
    /// Sonora's volume, from 0 to 100.
    volume: u8,
    pending: Option<Selection>,
    current: Option<String>,
    opening_seek: Option<(String, Duration)>,
    seeking: bool,
    position: Duration,
    active: bool,
    repeat: Repeat,
    /// Sign-ins Spotify turned down since the last one it took.
    refusals: u8,
    /// Whether a too-old build was already replaced once, so it is not fetched in a loop.
    refreshed: bool,
    /// Whether the setup proved broken. Nothing is started again until a new engine.
    dead: bool,
}

impl Brain {
    async fn run(mut self, mut commands: UnboundedReceiver<Command>) {
        self.wake();
        loop {
            tokio::select! {
                command = commands.recv() => match command {
                    Some(command) => self.command(command),
                    None => break,
                },
                incoming = next(&mut self.link) => match incoming {
                    Incoming::Event(event) => self.event(event),
                    Incoming::Line(line) => self.line(&line),
                    Incoming::Gone(end) => self.gone(end),
                },
            }
        }
        if let Some(link) = self.link.take() {
            link.close().await;
        }
    }

    fn command(&mut self, command: Command) {
        match command {
            Command::Load { id, at, play } => {
                if self.dead {
                    return self.report(PlaybackEvent::Refused);
                }
                self.report(PlaybackEvent::Signal {
                    id: id.clone(),
                    track: self.signal(),
                });
                self.report(match play {
                    true => PlaybackEvent::Loading {
                        id: Some(id.clone()),
                        at,
                    },
                    false => PlaybackEvent::Paused {
                        id: Some(id.clone()),
                        at,
                    },
                });
                self.pending = Some(Selection {
                    id,
                    position: at,
                    play,
                });
                self.opening_seek = None;
                self.seeking = false;
                self.wake();
                self.start_pending();
            }
            Command::Play => {
                if let Some(pending) = &mut self.pending {
                    pending.play = true;
                    self.start_pending();
                } else {
                    self.send("play", json!({}));
                }
            }
            Command::Pause => {
                if let Some(pending) = &mut self.pending {
                    pending.play = false;
                }
                if self.active {
                    self.send("pause", json!({}));
                }
            }
            Command::Seek(at) => {
                if let Some(pending) = &mut self.pending {
                    pending.position = at;
                    let id = Some(pending.id.clone());
                    self.report(PlaybackEvent::Seeked { id, at });
                } else {
                    self.seeking = true;
                    self.send("seek", json!({ "position_ms": millis(at) }));
                }
            }
            Command::Gain(gain) => {
                self.volume = percent(gain);
                self.send("set_volume", json!({ "volume": self.volume }));
            }
            Command::Repeat(mode) => {
                self.repeat = mode;
                if self.active {
                    self.send_repeat();
                }
            }
            Command::Skip(forward) if self.active => {
                self.send(if forward { "skip_next" } else { "skip_prev" }, json!({}));
            }
            Command::Shuffle(enabled) if self.active => {
                self.send("set_shuffle", json!({ "enabled": enabled }))
            }
            Command::Skip(_) | Command::Shuffle(_) => {}
        }
    }

    /// A URI starts a new selection. Ordinary transport commands keep the daemon's queue.
    fn start_pending(&mut self) {
        if !self.signed_in {
            return;
        }
        let Some(selection) = self.pending.take_if(|selection| selection.play) else {
            return;
        };
        if !self.active {
            self.send("activate", json!({}));
        }
        self.send(
            "play",
            json!({ "uri": format!("{TRACK_PREFIX}{}", selection.id) }),
        );
        self.current = None;
        self.position = Duration::ZERO;
        self.opening_seek = Some((selection.id, selection.position));
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::AuthState {
                logged_in,
                is_active,
            } => {
                self.active = is_active;
                if logged_in && !self.signed_in {
                    self.refusals = 0;
                    self.signed_in = true;
                    self.start_pending();
                }
                self.signed_in = logged_in;
            }
            Event::PlaybackState(state) | Event::PlaybackChanged(state) => self.playback(state),
            Event::TrackChanged { item: Some(item) } => {
                self.current = item.uri.strip_prefix(TRACK_PREFIX).map(str::to_owned);
                self.position = Duration::ZERO;
            }
            Event::PositionSync { position } if self.active && self.current.is_some() => {
                self.position = position.at();
                let id = self.current.clone();
                let at = self.position;
                let seeking = std::mem::take(&mut self.seeking);
                self.report(if seeking {
                    PlaybackEvent::Seeked { id, at }
                } else {
                    PlaybackEvent::Position { id, at }
                });
            }
            Event::VolumeChanged { volume } if self.active && self.volume != volume => {
                self.volume = volume;
                self.report(PlaybackEvent::RemoteGain(
                    (f32::from(volume) / 100.0).powi(3),
                ));
            }
            Event::DeviceChanged { is_active } => self.active = is_active,
            Event::Error { message } => log::warn!("soloist: {message}"),
            _ => {}
        }
    }

    /// Forward transport transitions for the daemon's current track.
    fn playback(&mut self, state: Playback) {
        if let Some(active) = state.is_active {
            self.active = active;
        }
        if !self.active {
            return;
        }
        if let Some(item) = state.item {
            self.current = item.uri.strip_prefix(TRACK_PREFIX).map(str::to_owned);
        }
        if let Some(position) = state.position {
            self.position = position.at();
        }
        let Some(current) = &self.current else { return };
        let id = Some(current.clone());
        let at = self.position;
        match state.status {
            Some(Status::Playing) => {
                if let Some((_, target)) = self
                    .opening_seek
                    .take_if(|(track, _)| Some(track.as_str()) == id.as_deref())
                {
                    self.send_repeat();
                    if !target.is_zero() {
                        self.seeking = true;
                        self.send("seek", json!({"position_ms": millis(target)}));
                    }
                }
                self.report(PlaybackEvent::Playing { id, at });
            }
            Some(Status::Paused) => self.report(PlaybackEvent::Paused { id, at }),
            Some(Status::Idle) if self.opening_seek.is_none() => {
                self.current = None;
                self.report(PlaybackEvent::Ended { id });
            }
            _ if state.position.is_some() => self.report(PlaybackEvent::Seeked { id, at }),
            _ => {}
        }
    }

    fn line(&mut self, line: &str) {
        if let Some((_, id)) = line
            .split_once("ready, device_name=")
            .and_then(|(_, device)| device.rsplit_once(" device_id="))
            && id.len() == 36
            && id
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() || byte == b'-')
        {
            *self
                .device
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(id.to_owned());
        }
        if line.contains(REFUSED) {
            self.refusals += 1;
            log::warn!("soloist: spotify turned the sign-in down");
            match self.refusals >= REFUSALS {
                true => self.refuse(),
                false => {
                    self.hang_up();
                    self.fail();
                }
            }
        } else if line.contains(UNPAIRED) && paired_account().is_none() {
            log::warn!("soloist: the daemon has no sign-in left and has to be paired again");
            self.refuse();
        }
    }

    fn gone(&mut self, end: End) {
        self.link = None;
        self.signed_in = false;
        self.active = false;
        match end {
            End::Closed => {}
            End::Expired if !self.refreshed => {
                log::info!("soloist: the build is too old, fetching the current one");
                self.refreshed = true;
                self.link = Some(Link::open(self.setup, self.volume, true));
            }
            End::Unready => {
                recheck();
                self.refuse();
            }
            End::Expired | End::Failed => self.fail(),
        }
    }

    /// Gives up on the wanted track. The next load starts a new daemon.
    fn fail(&mut self) {
        if let Some(id) = self
            .pending
            .take()
            .map(|s| s.id)
            .or_else(|| self.opening_seek.take().map(|(id, _)| id))
            .or_else(|| self.current.take())
        {
            self.report(PlaybackEvent::Unavailable { id: Some(id) });
        }
    }

    /// Reports that the setup no longer works and stops for good.
    fn refuse(&mut self) {
        self.dead = true;
        self.pending = None;
        self.current = None;
        self.hang_up();
        self.report(PlaybackEvent::Refused);
    }

    /// Starts a daemon when none runs.
    fn wake(&mut self) {
        if self.link.is_none() && !self.dead {
            self.link = Some(Link::open(self.setup, self.volume, false));
        }
    }

    /// Stops the daemon without waiting for it.
    fn hang_up(&mut self) {
        self.signed_in = false;
        self.active = false;
        if let Some(link) = self.link.take() {
            tokio::spawn(link.close());
        }
    }

    /// What every track played here is: the level asked for, decoded by the daemon to 44.1 kHz
    /// stereo and normalised there. The daemon never says what it fetched, so the level is the
    /// ceiling Spotify's own apps show too.
    fn signal(&self) -> TrackSignal {
        TrackSignal {
            encoding: self.setup.quality.offered_encoding(QUALITIES),
            rate: RATE,
            channels: CHANNELS,
            normalisation: match self.setup.normalise {
                true => Normalisation::Limited,
                false => Normalisation::Off,
            },
            direct: true,
        }
    }

    fn send_repeat(&self) {
        for (name, fields) in repeat_commands(self.repeat) {
            self.send(name, fields);
        }
    }

    /// Sends a command, which the daemon only takes once it is signed in.
    fn send(&self, name: &'static str, fields: Value) {
        if let Some(link) = self.link.as_ref().filter(|_| self.signed_in) {
            link.outgoing.send((name, fields)).ok();
        }
    }

    fn report(&self, event: PlaybackEvent) {
        self.events.send(event).ok();
    }
}

/// What a daemon sends the brain.
enum Incoming {
    Event(Event),
    /// A line the daemon printed.
    Line(String),
    /// The daemon is gone, which is always the last thing sent.
    Gone(End),
}

/// How a daemon came to its end.
enum End {
    /// The brain let go of it.
    Closed,
    /// Its build is too old for Spotify's servers.
    Expired,
    /// The key or the program is missing, so the setup has to be done again.
    Unready,
    /// It could not start or stopped by itself.
    Failed,
}

/// One running daemon: commands go in as JSON text, its reports come back.
struct Link {
    outgoing: UnboundedSender<Outgoing>,
    incoming: UnboundedReceiver<Incoming>,
    task: JoinHandle<()>,
}

impl Link {
    /// Starts a daemon, after fetching the current build when `refresh` asks.
    fn open(setup: Setup, volume: u8, refresh: bool) -> Self {
        let (outgoing, asked) = unbounded_channel();
        let (reports, incoming) = unbounded_channel();
        let task = tokio::spawn(serve(setup, volume, refresh, asked, reports));
        Self {
            outgoing,
            incoming,
            task,
        }
    }

    /// Stops the daemon and waits until it has let go of the data folder.
    async fn close(self) {
        drop(self.outgoing);
        drop(self.incoming);
        self.task.await.ok();
    }
}

/// The next thing the daemon sends, or never when none runs.
async fn next(link: &mut Option<Link>) -> Incoming {
    match link {
        Some(link) => link
            .incoming
            .recv()
            .await
            .unwrap_or(Incoming::Gone(End::Failed)),
        None => std::future::pending().await,
    }
}

/// Runs one daemon from start to end, holding the data folder for as long as it lives.
async fn serve(
    setup: Setup,
    volume: u8,
    refresh: bool,
    mut asked: UnboundedReceiver<Outgoing>,
    reports: UnboundedSender<Incoming>,
) {
    let _running = tokio::select! {
        running = RUNNING.lock() => running,
        () = dropped(&mut asked) => return,
    };
    let end = daemon(setup, volume, refresh, &mut asked, &reports).await;
    reports.send(Incoming::Gone(end)).ok();
}

async fn daemon(
    setup: Setup,
    volume: u8,
    refresh: bool,
    asked: &mut UnboundedReceiver<Outgoing>,
    reports: &UnboundedSender<Incoming>,
) -> End {
    if refresh {
        let fetched = tokio::select! {
            fetched = binary::fetch() => fetched,
            () = dropped(asked) => return End::Closed,
        };
        if let Err(error) = fetched {
            log::warn!("soloist: cannot fetch the current build: {error:#}");
            return End::Failed;
        }
    }
    let prepared = tokio::task::spawn_blocking(move || prepare(setup)).await;
    let Ok(Some(key)) = prepared else {
        log::warn!("soloist: no api key is stored");
        return End::Unready;
    };
    let volume = volume.to_string();
    let flags = [
        "--cache-size",
        CACHE_MB,
        "--initial-volume",
        volume.as_str(),
        "--ws",
        "127.0.0.1:0",
    ]
    .map(OsStr::new);
    let mut child = match launch(&key, &data(), &flags) {
        Ok(child) => child,
        Err(error) => {
            log::error!("soloist: {error:#}");
            return End::Unready;
        }
    };
    let mut lines = follow(&mut child, &key);
    let mut printing = true;

    let connecting = connect();
    tokio::pin!(connecting);
    let mut socket: Option<Socket> = None;
    loop {
        let connected = socket.is_some();
        tokio::select! {
            result = &mut connecting, if !connected => match result {
                Ok(open) => socket = Some(open),
                Err(error) => {
                    log::warn!("soloist: {error:#}");
                    stop(&mut child).await;
                    return End::Failed;
                }
            },
            frame = async { socket.as_mut().expect("connected socket").next().await }, if connected => match frame {
                Some(Ok(Message::Text(text))) => match serde_json::from_str(text.as_str()) {
                    Ok(event) => {
                        reports.send(Incoming::Event(event)).ok();
                    }
                    Err(error) => log::debug!("soloist: cannot read a message: {error}"),
                },
                Some(Ok(_)) => {}
                Some(Err(_)) | None => {
                    // the socket goes when the daemon does
                    stop(&mut child).await;
                    return ended(child.wait().await);
                }
            },
            command = asked.recv() => match command {
                Some((name, fields)) => {
                    if let Some(socket) = &mut socket
                        && let Err(error) = socket.send(Message::text(wire::command(name, fields))).await
                    {
                        log::warn!("soloist: cannot send a command: {error}");
                    }
                }
                None => {
                    stop(&mut child).await;
                    return End::Closed;
                }
            },
            line = lines.recv(), if printing => match line {
                Some(line) => {
                    reports.send(Incoming::Line(line)).ok();
                }
                None => printing = false,
            },
            status = child.wait() => return ended(status),
        }
    }
}

/// Reads the key, clears the address a dead daemon left behind and writes the audio settings
/// the daemon reads at startup. `None` without a key. Blocking.
fn prepare(setup: Setup) -> Option<String> {
    let key = key()?;
    let data = data();
    for file in [WS_ADDR, WS_PORT] {
        std::fs::remove_file(data.join(file)).ok();
    }
    if let Err(error) = prefs::write(&data, setup.normalise, setup.gapless, setup.quality) {
        log::warn!("soloist: cannot write the audio settings: {error:#}");
    }
    Some(key)
}

/// Waits for the daemon to write down where its WebSocket listens and connects to it.
async fn connect() -> Result<Socket> {
    let data = data();
    let deadline = Instant::now() + STARTUP;
    loop {
        if let Some(address) = endpoint(&data)
            && let Ok((socket, _)) = tokio_tungstenite::connect_async(address).await
        {
            return Ok(socket);
        }
        ensure!(
            Instant::now() < deadline,
            "the daemon did not open its websocket"
        );
        tokio::time::sleep(POLL).await;
    }
}

fn endpoint(data: &Path) -> Option<String> {
    let address = std::fs::read_to_string(data.join(WS_ADDR)).ok()?;
    let port = std::fs::read_to_string(data.join(WS_PORT)).ok()?;
    let port: u16 = port.trim().parse().ok().filter(|port| *port != 0)?;
    Some(format!("ws://{}:{port}", address.trim()))
}

fn ended(status: std::io::Result<std::process::ExitStatus>) -> End {
    match status.ok().and_then(|status| status.code()) {
        Some(binary::EXPIRED) => End::Expired,
        code => {
            log::warn!("soloist: the daemon stopped with code {code:?}");
            End::Failed
        }
    }
}

/// Discards commands while the daemon cannot start and returns when its owner goes away.
async fn dropped(asked: &mut UnboundedReceiver<Outgoing>) {
    while asked.recv().await.is_some() {}
}

/// Clears the other repeat mode before enabling the requested one.
fn repeat_commands(mode: Repeat) -> [Outgoing; 2] {
    let (clear, set) = match mode {
        Repeat::One => ("set_repeat_context", "set_repeat_track"),
        _ => ("set_repeat_track", "set_repeat_context"),
    };
    [
        (clear, json!({ "enabled": false })),
        (set, json!({ "enabled": mode != Repeat::Off })),
    ]
}

/// The daemon's volume for a linear gain. It attenuates on Spotify's cubic curve, so the cube
/// root gives back the gain Sonora asked for.
fn percent(gain: f32) -> u8 {
    (gain.clamp(0., 1.).cbrt() * 100.).round() as u8
}

fn millis(position: Duration) -> u64 {
    u64::try_from(position.as_millis()).unwrap_or(u64::MAX)
}
