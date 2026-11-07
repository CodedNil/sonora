//! Lifecycle and setup of a provider's separately installed player.

use futures::future::{AbortHandle, Abortable};
use gpui::{Context, Task};
use music::companion::CompanionState;
use tokio::sync::mpsc::unbounded_channel;

use super::Devices;
use crate::{Outcome, Toasts};

/// Cancels work on the IO runtime when its owning session or setup task goes away.
pub(super) struct CompanionJob {
    abort: AbortHandle,
    _task: Task<()>,
}

impl Drop for CompanionJob {
    fn drop(&mut self) {
        self.abort.abort();
    }
}

impl Devices {
    pub fn companion_supported(&self) -> bool {
        self.companion
            .as_ref()
            .is_some_and(|player| player.supported())
    }

    pub fn companion_selected(&self) -> bool {
        self.companion_selected
    }

    pub fn companion_state(&self) -> CompanionState {
        self.companion_state
    }

    pub fn companion_build(&self) -> Option<music::companion::CompanionBuild> {
        self.companion.as_ref()?.build()
    }

    pub fn companion_name(&self) -> &str {
        self.companion
            .as_ref()
            .map_or("", |player| player.device_name())
    }

    /// Whether a Connect device is the managed player on this computer.
    pub fn is_companion(&self, id: &str) -> bool {
        self.companion_selected
            && self
                .companion_device
                .as_ref()
                .is_some_and(|device| device.id == id)
    }

    /// Downloads or updates the official player, then starts the pairing step. A failed
    /// download preserves the existing backend choice and private installation.
    pub fn install_companion(&mut self, key: String, cx: &mut Context<Self>) {
        if self.companion_install.is_some() {
            return;
        }
        let Some(player) = self.companion.clone() else {
            return;
        };
        self.companion_state = CompanionState::Downloading;
        let (abort, registration) = AbortHandle::new_pair();
        let handle = self.io.spawn(Abortable::new(
            async move { player.install(key).await },
            registration,
        ));
        let task = cx.spawn(async move |this, cx| {
            let result = handle.await;
            this.update(cx, |this, cx| {
                this.companion_install = None;
                match result {
                    Ok(Ok(Ok(()))) => {
                        this.here(cx, |playback, cx| playback.change_backend(cx));
                        this.companion_selected = true;
                        this.claimed = false;
                        if let Some(link) = &this.link {
                            link.send(music::connect::Request::Publish(None));
                        }
                        this.sent = None;
                        this.start_companion(cx);
                        this.apply(cx);
                    }
                    _ => {
                        this.companion_state = CompanionState::Failed;
                        Toasts::show(Outcome::Info, "toast-soloist-install-failed", cx);
                    }
                }
                cx.notify();
            })
            .ok();
        });
        self.companion_install = Some(CompanionJob { abort, _task: task });
        cx.notify();
    }

    /// Stops the owned process and remembers the built-in backend, retaining the key and
    /// pairing for the next opt-in. Cancelling setup also cancels any download in progress.
    pub fn disable_companion(&mut self, cx: &mut Context<Self>) {
        self.companion_install = None;
        self.companion_job = None;
        self.companion_updates = None;
        self.companion_device = None;
        self.companion_authenticated = false;
        let Some(player) = self.companion.clone() else {
            return;
        };
        let (abort, registration) = AbortHandle::new_pair();
        let handle = self.io.spawn(Abortable::new(
            async move { player.disable().await },
            registration,
        ));
        let task = cx.spawn(async move |this, cx| {
            let result = handle.await;
            this.update(cx, |this, cx| {
                this.companion_install = None;
                match result {
                    Ok(Ok(Ok(()))) => {
                        this.companion_selected = false;
                        this.here(cx, |playback, cx| playback.change_backend(cx));
                        this.companion_state = CompanionState::Off;
                        this.roster.elsewhere = None;
                        this.claimed = false;
                        this.apply(cx);
                    }
                    _ => this.companion_state = CompanionState::Failed,
                }
                this.redraw(cx);
                cx.notify();
            })
            .ok();
        });
        self.companion_install = Some(CompanionJob { abort, _task: task });
        cx.notify();
    }

    /// Starts a saved installation without downloading again. Pairing is retained by Spotify.
    pub fn retry_companion(&mut self, cx: &mut Context<Self>) {
        if self.companion_selected {
            self.start_companion(cx);
        }
    }

    pub(super) fn start_companion(&mut self, cx: &mut Context<Self>) {
        self.companion_job = None;
        self.companion_updates = None;
        self.companion_authenticated = false;
        self.companion_device = None;
        self.companion_state = CompanionState::Starting;
        let Some(player) = self.companion.clone() else {
            return;
        };
        let (send, mut receive) = unbounded_channel();
        let failed = send.clone();
        let (abort, registration) = AbortHandle::new_pair();
        let handle = self.io.spawn(Abortable::new(
            async move {
                if let Err(error) = player.run(send).await {
                    log::warn!("soloist: player stopped: {error:#}");
                    failed.send(CompanionState::Failed).ok();
                }
            },
            registration,
        ));
        let task = cx.spawn(async move |_, _| {
            handle.await.ok();
        });
        self.companion_job = Some(CompanionJob { abort, _task: task });
        self.companion_updates = Some(cx.spawn(async move |this, cx| {
            while let Some(state) = receive.recv().await {
                if this
                    .update(cx, |this, cx| {
                        this.companion_state = state;
                        this.companion_authenticated = state == CompanionState::Verifying;
                        if !this.companion_authenticated {
                            this.companion_device = None;
                        }
                        this.verify_companion();
                        if matches!(state, CompanionState::Failed | CompanionState::Expired) {
                            this.companion_device = None;
                            Toasts::show(Outcome::Info, "toast-soloist-stopped", cx);
                        }
                        this.redraw(cx);
                        cx.notify();
                    })
                    .is_err()
                {
                    break;
                }
            }
        }));
        cx.notify();
    }

    /// A local authenticated socket is not proof of the Spotify account. Only accept the
    /// uniquely named device after the signed-in account's Connect roster lists it.
    pub(super) fn verify_companion(&mut self) {
        if !self.companion_selected || !self.companion_authenticated {
            return;
        }
        if let Some(device) = self
            .roster
            .devices
            .iter()
            .find(|device| device.name == self.companion_name())
        {
            self.companion_device = Some(device.clone());
            self.companion_state = CompanionState::Ready;
        }
    }
}
