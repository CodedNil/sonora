//! Spotify's separately downloaded player. Audio and listen reporting belong to Soloist;
//! Sonora addresses its verified device through the existing Connect implementation.

use std::io::{Cursor, Read as _};
use std::path::{Component, Path, PathBuf};
use std::process::Stdio;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use flate2::read::GzDecoder;
use futures::{SinkExt as _, StreamExt as _};
use librespot_core::Session;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::companion::{Companion, CompanionBuild, CompanionState};
use crate::credentials;

const CDN: &str = "https://soloist-builds.spotifycdn.com";
const MAX_ARCHIVE: usize = 256 * 1024 * 1024;
const MAX_EXPANDED: u64 = 512 * 1024 * 1024;
const START_WAIT: Duration = Duration::from_secs(20);
const POLL: Duration = Duration::from_millis(250);
const SOCKET_WAIT: Duration = Duration::from_secs(30);
const MAX_ENTRIES: usize = 8192;

/// An account's private installation and remembered opt-in. The key is never part of app
/// settings, logs, or a debug representation.
pub(super) struct Soloist {
    folder: PathBuf,
    name: String,
    enabled: AtomicBool,
    build: RwLock<Option<CompanionBuild>>,
}

/// The private key, selected backend and path to an independently downloaded build.
#[derive(Default, Deserialize, Serialize)]
struct Config {
    key: String,
    binary: PathBuf,
    enabled: bool,
    #[serde(default)]
    build: Option<CompanionBuild>,
}

impl Soloist {
    pub(super) fn new(session: &Session) -> Self {
        let username = match session.username().is_empty() {
            true => session
                .cache()
                .and_then(|cache| cache.credentials())
                .and_then(|credentials| credentials.username)
                .unwrap_or_default(),
            false => session.username(),
        };
        let account = ring::digest::digest(&ring::digest::SHA256, username.as_bytes());
        let account: String = account
            .as_ref()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let folder = credentials::dir("spotify").join("soloist").join(account);
        let config = read(&folder).unwrap_or_default();
        Self {
            folder,
            name: format!(
                "Sonora Soloist ({})",
                session.device_id().chars().take(8).collect::<String>()
            ),
            enabled: AtomicBool::new(config.enabled),
            build: RwLock::new(config.build),
        }
    }
}

#[async_trait]
impl Companion for Soloist {
    fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }

    fn supported(&self) -> bool {
        archive().is_some()
    }

    fn device_name(&self) -> &str {
        &self.name
    }

    fn build(&self) -> Option<CompanionBuild> {
        self.build.read().ok()?.clone()
    }

    async fn install(&self, key: String) -> Result<()> {
        let archive = archive().context("cannot run Soloist on this platform")?;
        let mut config = read(&self.folder).unwrap_or_default();
        if !key.trim().is_empty() {
            config.key = key.trim().to_owned();
        }
        if config.key.is_empty() {
            bail!("cannot install Soloist without a personal API key");
        }
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(180))
            .build()?;
        let mut response = client
            .get(format!("{CDN}/{archive}"))
            .send()
            .await?
            .error_for_status()?;
        if response
            .content_length()
            .is_some_and(|size| size > MAX_ARCHIVE as u64)
        {
            bail!("cannot download an oversized Soloist archive");
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if bytes.len().saturating_add(chunk.len()) > MAX_ARCHIVE {
                bail!("cannot download an oversized Soloist archive");
            }
            bytes.extend_from_slice(&chunk);
        }
        let folder = self.folder.clone();
        let (staged, binary) =
            tokio::task::spawn_blocking(move || install_archive(&folder, archive, bytes)).await??;
        let build = inspect(&binary).await?;
        config.binary = binary;
        config.build = Some(build.clone());
        config.enabled = true;
        credentials::write(
            &self.folder.join("config.json"),
            &serde_json::to_vec(&config)?,
        )?;
        let _ = staged.keep();
        if let Ok(mut installed) = self.build.write() {
            *installed = Some(build);
        }
        self.enabled.store(true, Ordering::Release);
        Ok(())
    }

    async fn disable(&self) -> Result<()> {
        let mut config = read(&self.folder).unwrap_or_default();
        config.enabled = false;
        credentials::write(
            &self.folder.join("config.json"),
            &serde_json::to_vec(&config)?,
        )?;
        self.enabled.store(false, Ordering::Release);
        Ok(())
    }

    async fn run(&self, events: UnboundedSender<CompanionState>) -> Result<()> {
        let config = read(&self.folder)?;
        let data = self.folder.join("data");
        // Also sets owner-only directory permissions before Spotify writes its session here.
        credentials::write(&data.join(".private"), b"")?;
        let _ = std::fs::remove_file(data.join("ws.port"));
        let _ = std::fs::remove_file(data.join("ws.addr"));
        let mut child = tokio::process::Command::new(&config.binary)
            .arg("--device-name")
            .arg(&self.name)
            .arg("--api-key")
            .arg(&config.key)
            .arg("--data-dir")
            .arg(&data)
            .arg("--cache-dir")
            .arg(credentials::root().join("soloist"))
            .arg("--cache-size")
            .arg("512")
            .arg("--ws")
            .arg("127.0.0.1:0")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .context("cannot start Soloist")?;
        events.send(CompanionState::Starting).ok();
        let started = tokio::time::Instant::now();
        let mut stream = loop {
            if let Some(status) = child.try_wait()? {
                if status.code() == Some(10) {
                    events.send(CompanionState::Expired).ok();
                    return Ok(());
                }
                bail!("cannot start Soloist: process exited");
            }
            if started.elapsed() > START_WAIT {
                bail!("cannot reach Soloist's local control socket");
            }
            let port = std::fs::read_to_string(data.join("ws.port"))
                .ok()
                .and_then(|port| port.trim().parse::<u16>().ok());
            if let Some(port) = port.filter(|port| *port != 0)
                && let Ok((stream, _)) =
                    tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{port}")).await
            {
                break stream;
            }
            tokio::time::sleep(POLL).await;
        };
        loop {
            tokio::select! {
                status = child.wait() => {
                    if status?.code() == Some(10) {
                        events.send(CompanionState::Expired).ok();
                        return Ok(());
                    }
                    bail!("cannot keep Soloist running: process exited");
                }
                message = tokio::time::timeout(SOCKET_WAIT, stream.next()) => {
                    let message = match message {
                        Ok(Some(message)) => message?,
                        Ok(None) => bail!("cannot reach Soloist: control socket closed"),
                        // No events while paused is normal. Check the process again.
                        Err(_) => continue,
                    };
                    if message.is_close() {
                        bail!("cannot reach Soloist: control socket closed");
                    }
                    if let tokio_tungstenite::tungstenite::Message::Ping(payload) = message {
                        stream.send(tokio_tungstenite::tungstenite::Message::Pong(payload)).await?;
                        continue;
                    }
                    if !message.is_text() { continue; }
                    let value: serde_json::Value = serde_json::from_str(message.to_text()?)?;
                    if value["type"] == "auth_state" {
                        let state = match value["logged_in"].as_bool() == Some(true) {
                            true => CompanionState::Verifying,
                            false => CompanionState::Pairing,
                        };
                        events.send(state).ok();
                    }
                }
            }
        }
    }
}

fn read(folder: &Path) -> Result<Config> {
    let contents =
        std::fs::read(folder.join("config.json")).context("cannot read Soloist settings")?;
    serde_json::from_slice(&contents).context("cannot read Soloist settings")
}

/// The filenames published by Spotify; no binary is bundled or mirrored by Sonora.
fn archive() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Some("soloist_release_x86_64.tar.gz"),
        ("linux", "aarch64") => Some("soloist_release_arm64.tar.gz"),
        ("linux", "arm") => Some("soloist_release_arm32.tar.gz"),
        ("macos", "aarch64") => Some("soloist_release_osx_arm64.tar.gz"),
        ("windows", "x86_64") => Some("soloist_release_windows_x86_64.zip"),
        _ => None,
    }
}

/// Extracts regular files only into a fresh private directory, bounding both compressed and
/// expanded sizes. Keeping the whole archive preserves Spotify's accompanying license files.
fn install_archive(
    folder: &Path,
    archive: &str,
    bytes: Vec<u8>,
) -> Result<(tempfile::TempDir, PathBuf)> {
    credentials::write(&folder.join(".private"), b"")?;
    let staged = tempfile::Builder::new()
        .prefix("build-")
        .tempdir_in(folder)?;
    let mut total = 0u64;
    if archive.ends_with(".zip") {
        let mut zipped = zip::ZipArchive::new(Cursor::new(bytes))?;
        if zipped.len() > MAX_ENTRIES {
            bail!("cannot extract too many Soloist files");
        }
        for index in 0..zipped.len() {
            let mut entry = zipped.by_index(index)?;
            let relative = entry
                .enclosed_name()
                .context("cannot extract an unsafe Soloist path")?;
            if entry.is_symlink() {
                bail!("cannot extract a Soloist archive link");
            }
            if entry.is_dir() {
                continue;
            }
            total = total.saturating_add(entry.size());
            if total > MAX_EXPANDED {
                bail!("cannot extract an oversized Soloist archive");
            }
            extract(staged.path(), &relative, &mut entry)?;
        }
    } else {
        let mut tar = tar::Archive::new(GzDecoder::new(Cursor::new(bytes)));
        for (index, entry) in tar.entries()?.enumerate() {
            if index >= MAX_ENTRIES {
                bail!("cannot extract too many Soloist files");
            }
            let mut entry = entry?;
            if entry.header().entry_type().is_dir() {
                continue;
            }
            if !entry.header().entry_type().is_file() {
                bail!("cannot extract a Soloist archive link");
            }
            let relative = entry.path()?.into_owned();
            total = total.saturating_add(entry.size());
            if total > MAX_EXPANDED {
                bail!("cannot extract an oversized Soloist archive");
            }
            extract(staged.path(), &relative, &mut entry)?;
        }
    }
    let name = if cfg!(windows) {
        "soloist.exe"
    } else {
        "soloist"
    };
    let binary =
        find_binary(staged.path(), name)?.context("cannot find Soloist in the download")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&binary, std::fs::Permissions::from_mode(0o700))?;
    }
    Ok((staged, binary))
}

/// Checks that Spotify's executable runs on this machine and that the downloaded build has
/// not already expired. Only `--version` output is read; child logs and keys are never logged.
async fn inspect(binary: &Path) -> Result<CompanionBuild> {
    let output = tokio::time::timeout(
        Duration::from_secs(10),
        tokio::process::Command::new(binary)
            .arg("--version")
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .output(),
    )
    .await??;
    if !output.status.success() {
        bail!("cannot run the downloaded Soloist build");
    }
    let text = std::str::from_utf8(&output.stdout)?;
    let words: Vec<_> = text.split_whitespace().collect();
    if words.first() != Some(&"soloist") {
        bail!("cannot recognize the Soloist build");
    }
    let version = words
        .get(1)
        .context("cannot read the Soloist version")?
        .to_string();
    let index = words
        .iter()
        .position(|word| *word == "build")
        .context("cannot find the Soloist build timestamp")?;
    let built_at: u64 = words
        .get(index + 1)
        .context("cannot read the Soloist build timestamp")?
        .parse()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    if built_at > now.saturating_add(86400) || now >= built_at.saturating_add(90 * 86400) {
        bail!("cannot use an expired or incorrectly dated Soloist build");
    }
    Ok(CompanionBuild { version, built_at })
}

fn extract(folder: &Path, relative: &Path, reader: &mut dyn std::io::Read) -> Result<()> {
    if relative.components().count() > 32 {
        bail!("cannot extract an excessively nested Soloist path");
    }
    if relative
        .components()
        .any(|part| !matches!(part, Component::Normal(_) | Component::CurDir))
    {
        bail!("cannot extract an unsafe Soloist path");
    }
    let path = folder.join(relative);
    std::fs::create_dir_all(
        path.parent()
            .context("cannot find Soloist archive folder")?,
    )?;
    let mut output = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    let copied = std::io::copy(&mut reader.take(MAX_EXPANDED + 1), &mut output)?;
    if copied > MAX_EXPANDED {
        bail!("cannot extract an oversized Soloist file");
    }
    Ok(())
}

fn find_binary(folder: &Path, name: &str) -> Result<Option<PathBuf>> {
    for entry in std::fs::read_dir(folder)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(path) = find_binary(&entry.path(), name)? {
                return Ok(Some(path));
            }
        } else if entry.file_name() == name {
            return Ok(Some(entry.path()));
        }
    }
    Ok(None)
}

/// A selected companion must never fall back to decoding through librespot, even if a
/// reconnect or a remote command reaches the local engine before setup has finished.
pub(super) fn guarded() -> (Box<dyn crate::Player>, Box<dyn crate::PlaybackEvents>) {
    (Box::new(Guarded), Box::new(Guarded))
}

/// Keeps the dormant built-in engine from making playback or download requests.
struct Guarded;

impl crate::Player for Guarded {
    fn load(&self, _: &str, _: Duration, _: bool) -> Result<()> {
        bail!("cannot play through the built-in engine while Soloist is selected")
    }
    fn load_paused_at(&self, _: &str, _: Duration) -> Result<()> {
        Ok(())
    }
    fn preload(&self, _: &str, _: bool) -> Result<()> {
        Ok(())
    }
    fn play(&self) {}
    fn pause(&self) {}
    fn seek(&self, _: Duration) {}
    fn set_gain(&self, _: f32) {}
}

#[async_trait]
impl crate::PlaybackEvents for Guarded {
    async fn next(&mut self) -> Option<crate::PlaybackEvent> {
        std::future::pending().await
    }
}
