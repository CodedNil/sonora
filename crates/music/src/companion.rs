//! Optional provider-owned playback programs, installed separately from Sonora.

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedSender;

/// The installed player version and Spotify's build timestamp, in Unix seconds.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CompanionBuild {
    pub version: String,
    pub built_at: u64,
}

impl CompanionBuild {
    /// Remaining whole days before Spotify's 90-day build lifetime ends.
    pub fn days_left(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        (self.built_at.saturating_add(90 * 86400)).saturating_sub(now) / 86400
    }
}

/// The lifecycle of an optional playback program. Ready also requires its device to be
/// present in the signed-in account's Connect roster; authentication alone is insufficient.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CompanionState {
    #[default]
    Off,
    Downloading,
    Starting,
    Pairing,
    Verifying,
    Ready,
    Expired,
    Failed,
}

/// A provider's optional external player. All asynchronous methods run on the IO runtime.
/// Dropping `run` must terminate the owned process, including during account changes.
#[async_trait]
pub trait Companion: Send + Sync {
    fn enabled(&self) -> bool;
    fn supported(&self) -> bool;
    fn device_name(&self) -> &str;
    fn build(&self) -> Option<CompanionBuild>;
    /// Downloads the official build and stores the personal key privately. An empty key
    /// reuses the saved key. Does not enable the backend until installation succeeds.
    async fn install(&self, key: String) -> Result<()>;
    async fn disable(&self) -> Result<()>;
    /// Runs until stopped or failed, forwarding authentication and process status changes.
    async fn run(&self, events: UnboundedSender<CompanionState>) -> Result<()>;
}
