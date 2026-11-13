//! Native PipeWire discovery complements CPAL's playback devices with hardware routes and
//! negotiated formats. ALSA remains available for opening hardware directly.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use cpal::traits::DeviceTrait as _;
use pipewire as pw;
use pw::proxy::{Listener, ProxyT};
use pw::spa::param::{
    ParamType,
    audio::{AudioFormat, AudioInfoRaw},
};
use pw::spa::pod::{Pod, Value, ValueArray, deserialize::PodDeserializer};
use tracing::debug;

use super::{DeviceOption, Format, Port, Sample, Sink, output_alsa};

const DISCOVERY_LIMIT: Duration = Duration::from_millis(250);
const CACHE_LIFETIME: Duration = Duration::from_millis(500);
const POLL: Duration = Duration::from_millis(10);
static SNAPSHOT: Mutex<Option<(Instant, Snapshot)>> = Mutex::new(None);

#[derive(Clone, Default)]
struct Snapshot {
    sinks: HashMap<String, Sink>,
    default: Option<String>,
}

/// A hardware node belongs to one profile device on its parent card.
struct OutputNode {
    name: String,
    device: Option<u32>,
    profile: Option<i32>,
    format: Option<Format>,
}

/// EnumRoute describes every connector; Route identifies the connector currently selected.
struct Route {
    devices: Vec<i32>,
    port: Port,
    active: bool,
}

/// CPAL's PipeWire host snapshots its registry. Recreate it to include hotplugged devices and
/// fall back to ALSA when a native PipeWire connection cannot be established.
pub(super) fn host() -> Arc<cpal::Host> {
    Arc::new(
        cpal::available_hosts()
            .contains(&cpal::HostId::PipeWire)
            .then(|| cpal::host_from_id(cpal::HostId::PipeWire).ok())
            .flatten()
            .unwrap_or_else(|| {
                cpal::host_from_id(cpal::HostId::Alsa).expect("ALSA host unavailable")
            }),
    )
}

pub(super) fn is_server(host: &cpal::Host) -> bool {
    host.id() == cpal::HostId::PipeWire
}

pub(super) fn served(device: &cpal::Device) -> bool {
    device
        .id()
        .is_ok_and(|id| id.host() == cpal::HostId::PipeWire)
}

/// Hide CPAL's synthetic default aliases and tidy ALSA's configured PCM list.
pub(super) fn listed(device: &cpal::Device, option: DeviceOption) -> Option<DeviceOption> {
    let id = device.id().ok()?;
    match id.host() {
        cpal::HostId::Alsa => output_alsa::tidy(id.id(), option),
        cpal::HostId::PipeWire if matches!(id.id(), "sink_default" | "output_default") => None,
        cpal::HostId::PipeWire => {
            // CPAL also enumerates other applications' playback streams; only sink nodes are
            // selectable outputs. If discovery failed, keep CPAL's list as a fallback.
            let sinks = snapshot().sinks;
            (sinks.is_empty() || sinks.contains_key(&option.id)).then_some(option)
        }
    }
}

/// The actual default node, rather than CPAL's synthetic default device.
pub(super) fn system_default(host: &cpal::Host) -> Option<String> {
    is_server(host)
        .then(snapshot)?
        .default
        .map(|name| sink_id(&name))
}

pub(super) fn sinks() -> HashMap<String, Sink> {
    snapshot().sinks
}

pub(super) fn sink(device: &str) -> Option<Sink> {
    let id: cpal::DeviceId = device.parse().ok()?;
    if id.host() != cpal::HostId::PipeWire {
        return None;
    }
    let mut snapshot = snapshot();
    let key = match id.id() {
        "sink_default" | "output_default" => sink_id(snapshot.default.as_deref()?),
        name => sink_id(name),
    };
    snapshot.sinks.remove(&key)
}

/// Call off the UI and audio threads. Discovery has a deadline, and adjacent format/device
/// requests share a short-lived snapshot; disconnected servers are retried after it expires.
fn snapshot() -> Snapshot {
    if let Some((at, snapshot)) = SNAPSHOT
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .as_ref()
        && at.elapsed() < CACHE_LIFETIME
    {
        return snapshot.clone();
    }
    let snapshot = discover().unwrap_or_else(|error| {
        debug!("output: cannot inspect PipeWire outputs: {error}");
        Snapshot::default()
    });
    *SNAPSHOT.lock().unwrap_or_else(PoisonError::into_inner) =
        Some((Instant::now(), snapshot.clone()));
    snapshot
}

/// Keep all proxies on this calling thread, and finish two sync barriers: registry discovery
/// first, then the node/device parameters requested by the registry callbacks.
fn discover() -> anyhow::Result<Snapshot> {
    pw::init();
    let main_loop = pw::main_loop::MainLoopRc::new(None)?;
    let context = pw::context::ContextRc::new(&main_loop, None)?;
    let core = context.connect_rc(None)?;
    let registry = core.get_registry_rc()?;
    let nodes = Rc::new(RefCell::new(HashMap::<u32, OutputNode>::new()));
    let routes = Rc::new(RefCell::new(HashMap::<u32, Vec<Route>>::new()));
    let default = Rc::new(RefCell::new(None::<String>));
    let proxies = Rc::new(RefCell::new(
        Vec::<(Box<dyn ProxyT>, Box<dyn Listener>)>::new(),
    ));
    let registry_weak = registry.downgrade();
    let _registry_listener = registry
        .add_listener_local()
        .global({
            let (nodes, routes, default, proxies) = (
                nodes.clone(),
                routes.clone(),
                default.clone(),
                proxies.clone(),
            );
            move |global| {
                let Some(registry) = registry_weak.upgrade() else {
                    return;
                };
                let Some(props) = global.props.as_ref() else {
                    return;
                };
                let id = global.id;
                match global.type_ {
                    pw::types::ObjectType::Node
                        if props.get("media.class") == Some("Audio/Sink") =>
                    {
                        let Some(name) = props.get("node.name") else {
                            return;
                        };
                        let Ok(node) = registry.bind::<pw::node::Node, _>(global) else {
                            return;
                        };
                        nodes.borrow_mut().insert(
                            id,
                            OutputNode {
                                name: name.to_owned(),
                                device: props.get("device.id").and_then(|value| value.parse().ok()),
                                profile: props
                                    .get("card.profile.device")
                                    .and_then(|value| value.parse().ok()),
                                format: None,
                            },
                        );
                        let node_info = nodes.clone();
                        let nodes = nodes.clone();
                        let listener = node
                            .add_listener_local()
                            .info(move |info| {
                                if let Some(props) = info.props()
                                    && let Some(node) = node_info.borrow_mut().get_mut(&id)
                                {
                                    node.device = props
                                        .get("device.id")
                                        .and_then(|value| value.parse().ok())
                                        .or(node.device);
                                    node.profile = props
                                        .get("card.profile.device")
                                        .and_then(|value| value.parse().ok())
                                        .or(node.profile);
                                }
                            })
                            .param(move |_, param, _, _, pod| {
                                if param == ParamType::Format
                                    && let Some(node) = nodes.borrow_mut().get_mut(&id)
                                {
                                    node.format = pod.and_then(raw_format);
                                }
                            })
                            .register();
                        node.enum_params(0, Some(ParamType::Format), 0, u32::MAX);
                        proxies
                            .borrow_mut()
                            .push((Box::new(node), Box::new(listener)));
                    }
                    pw::types::ObjectType::Device => {
                        let Ok(device) = registry.bind::<pw::device::Device, _>(global) else {
                            return;
                        };
                        let routes = routes.clone();
                        let listener = device
                            .add_listener_local()
                            .param(move |_, param, _, _, pod| {
                                if let Some(route) =
                                    pod.and_then(|pod| route(pod, param == ParamType::Route))
                                {
                                    routes.borrow_mut().entry(id).or_default().push(route);
                                }
                            })
                            .register();
                        device.enum_params(0, Some(ParamType::EnumRoute), 0, u32::MAX);
                        device.enum_params(0, Some(ParamType::Route), 0, u32::MAX);
                        proxies
                            .borrow_mut()
                            .push((Box::new(device), Box::new(listener)));
                    }
                    pw::types::ObjectType::Metadata
                        if props.get("metadata.name") == Some("default") =>
                    {
                        let Ok(metadata) = registry.bind::<pw::metadata::Metadata, _>(global)
                        else {
                            return;
                        };
                        let default = default.clone();
                        let listener = metadata
                            .add_listener_local()
                            .property(move |_, key, _, value| {
                                if key == Some("default.audio.sink") {
                                    *default.borrow_mut() = value
                                        .and_then(|json| {
                                            serde_json::from_str::<serde_json::Value>(json).ok()
                                        })
                                        .and_then(|value| {
                                            value.get("name")?.as_str().map(str::to_owned)
                                        });
                                }
                                0
                            })
                            .register();
                        proxies
                            .borrow_mut()
                            .push((Box::new(metadata), Box::new(listener)));
                    }
                    _ => {}
                }
            }
        })
        .register();
    let completed = Rc::new(Cell::new(None));
    let failed = Rc::new(Cell::new(false));
    let _core_listener = core
        .add_listener_local()
        .done({
            let completed = completed.clone();
            move |_, seq| completed.set(Some(seq))
        })
        .error({
            let failed = failed.clone();
            move |id, _, _, _| {
                if id == pw::core::PW_ID_CORE {
                    failed.set(true);
                }
            }
        })
        .register();
    let deadline = Instant::now() + DISCOVERY_LIMIT;
    for round in 0..2 {
        let seq = core.sync(round)?;
        while completed.get() != Some(seq) {
            anyhow::ensure!(!failed.get(), "PipeWire connection closed");
            let remaining = deadline.saturating_duration_since(Instant::now());
            anyhow::ensure!(!remaining.is_zero(), "PipeWire discovery timed out");
            main_loop
                .loop_()
                .iterate(pw::loop_::Timeout::Finite(POLL.min(remaining)));
        }
    }
    let routes = routes.borrow();
    let sinks = nodes
        .borrow()
        .values()
        .map(|node| {
            let matching = node.device.and_then(|id| routes.get(&id));
            let applies = |route: &&Route| {
                route.devices.is_empty()
                    || node
                        .profile
                        .is_some_and(|profile| route.devices.contains(&profile))
            };
            let ports: Vec<_> = matching
                .into_iter()
                .flatten()
                .filter(applies)
                .filter(|route| !route.active)
                .map(|route| route.port.clone())
                .collect();
            let active = matching
                .into_iter()
                .flatten()
                .filter(applies)
                .find(|route| route.active)
                .and_then(|route| ports.iter().position(|port| port.id == route.port.id));
            (
                sink_id(&node.name),
                Sink {
                    ports,
                    active,
                    format: node.format,
                },
            )
        })
        .collect();
    let default = default.borrow().clone();
    Ok(Snapshot { sinks, default })
}

fn sink_id(name: &str) -> String {
    cpal::DeviceId::new(cpal::HostId::PipeWire, name).to_string()
}

/// Only report an active raw format, never an advertised capability or graph-rate guess.
fn raw_format(pod: &Pod) -> Option<Format> {
    let mut info = AudioInfoRaw::new();
    info.parse(pod).ok()?;
    let sample = match info.format() {
        AudioFormat::U8 | AudioFormat::U8P => Sample::Integer(8),
        AudioFormat::S16LE | AudioFormat::S16BE | AudioFormat::S16P => Sample::Integer(16),
        AudioFormat::S24LE
        | AudioFormat::S24BE
        | AudioFormat::S24_32LE
        | AudioFormat::S24_32BE
        | AudioFormat::S24P
        | AudioFormat::S24_32P => Sample::Integer(24),
        AudioFormat::S32LE | AudioFormat::S32BE | AudioFormat::S32P => Sample::Integer(32),
        AudioFormat::F32LE | AudioFormat::F32BE | AudioFormat::F32P => Sample::Float(32),
        AudioFormat::F64LE | AudioFormat::F64BE | AudioFormat::F64P => Sample::Float(64),
        _ => Sample::Other,
    };
    (info.rate() > 0 && info.channels() > 0).then_some(Format {
        rate: info.rate(),
        channels: u16::try_from(info.channels()).ok()?,
        sample,
    })
}

/// Decode SPA route objects without depending on struct layouts or an external pw-dump tool.
fn route(pod: &Pod, active: bool) -> Option<Route> {
    use pw::spa::sys as spa;
    let (_, Value::Object(object)) =
        PodDeserializer::deserialize_from::<Value>(pod.as_bytes()).ok()?
    else {
        return None;
    };
    let value = |key| {
        object
            .properties
            .iter()
            .find(|property| property.key == key)
            .map(|property| &property.value)
    };
    if !matches!(value(spa::SPA_PARAM_ROUTE_direction), Some(Value::Id(id)) if id.0 == spa::SPA_DIRECTION_OUTPUT)
    {
        return None;
    }
    let Some(Value::String(name)) = value(spa::SPA_PARAM_ROUTE_name) else {
        return None;
    };
    let description = match value(spa::SPA_PARAM_ROUTE_description) {
        Some(Value::String(description)) => description.clone(),
        _ => name.clone(),
    };
    let devices = match value(spa::SPA_PARAM_ROUTE_devices) {
        Some(Value::ValueArray(ValueArray::Int(devices))) => devices.clone(),
        _ => match value(spa::SPA_PARAM_ROUTE_device) {
            Some(Value::Int(device)) => vec![*device],
            _ => Vec::new(),
        },
    };
    let available = !matches!(value(spa::SPA_PARAM_ROUTE_available), Some(Value::Id(id)) if id.0 == spa::SPA_PARAM_AVAILABILITY_no);
    Some(Route {
        devices,
        port: Port {
            id: name.clone(),
            name: description,
            available,
        },
        active,
    })
}
