//! Optional Spotify playback, with a key, download and pairing flow.

use gpui::prelude::*;
use gpui::{AnyElement, Context, div};
use i18n::t;
use music::companion::CompanionState;
use state::Sonora;
use ui::{Button, Picker};

use super::{Row, SettingsView};

const KEYS: &str = "https://developer.spotify.com/dashboard/soloist";
const GUIDE: &str = "https://developer.spotify.com/documentation/soloist";

impl SettingsView {
    pub(super) fn soloist_row(&self, cx: &mut Context<Self>) -> AnyElement {
        let devices = Sonora::global(cx).devices.clone();
        let player = devices.read(cx);
        let selected = player.companion_selected();
        let state = player.companion_state();
        let name = player.companion_name().to_owned();
        let build = player.companion_build();
        let busy = matches!(
            state,
            CompanionState::Downloading | CompanionState::Starting
        );
        let status = match state {
            CompanionState::Off => "settings-soloist-off",
            CompanionState::Downloading => "settings-soloist-downloading",
            CompanionState::Starting => "settings-soloist-starting",
            CompanionState::Pairing => "settings-soloist-pairing",
            CompanionState::Verifying => "settings-soloist-verifying",
            CompanionState::Ready => "settings-soloist-ready",
            CompanionState::Expired => "settings-soloist-expired",
            CompanionState::Failed => "settings-soloist-failed",
        };
        let action = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_2()
            .child(
                Button::new("soloist-guide")
                    .label(t!("settings-soloist-guide"))
                    .small()
                    .ghost()
                    .on_click(|_, _, cx| cx.open_url(GUIDE)),
            )
            .when(selected && matches!(state, CompanionState::Failed), |row| {
                row.child(
                    Button::new("soloist-retry")
                        .label(t!("settings-soloist-retry"))
                        .small()
                        .outline()
                        .on_click(cx.listener(|_, _, _, cx| {
                            Sonora::global(cx)
                                .devices
                                .clone()
                                .update(cx, |devices, cx| devices.retry_companion(cx));
                        })),
                )
            })
            .when(selected || state != CompanionState::Off, |row| {
                row.child(
                    Button::new("soloist-disable")
                        .label(t!("settings-soloist-builtin"))
                        .small()
                        .outline()
                        .on_click(cx.listener(|_, _, _, cx| {
                            Sonora::global(cx)
                                .devices
                                .clone()
                                .update(cx, |devices, cx| devices.disable_companion(cx));
                        })),
                )
            });

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                Row::new(t!("settings-soloist"), action.into_any_element())
                    .detail(i18n::lookup(status, None)),
            )
            .when_some(build, |column, build| {
                column.child(ui::spoken(
                    "soloist-build",
                    t!(
                        "settings-soloist-build",
                        version = build.version.as_str(),
                        count = build.days_left()
                    ),
                ))
            })
            .when(!selected, |column| {
                column
                    .child(
                        Row::new(
                            t!("settings-soloist-key"),
                            Button::new("soloist-get-key")
                                .label(t!("settings-soloist-get-key"))
                                .small()
                                .outline()
                                .on_click(|_, _, cx| cx.open_url(KEYS))
                                .into_any_element(),
                        )
                        .detail(t!("settings-soloist-detail")),
                    )
                    .child(
                        Row::new(
                            t!("settings-soloist-personal-key"),
                            div()
                                .w(Picker::WIDE)
                                .child(self.soloist_key.clone())
                                .into_any_element(),
                        )
                        .yielding(),
                    )
            })
            .child(
                Row::new(
                    t!("settings-soloist-download"),
                    Button::new("soloist-download")
                        .label(match selected {
                            true => t!("settings-soloist-update"),
                            false => t!("settings-soloist-install"),
                        })
                        .small()
                        .outline()
                        .disabled(busy)
                        .on_click(cx.listener(|this, _, _, cx| {
                            let key = this.soloist_key.read(cx).text().to_string();
                            this.soloist_key
                                .update(cx, |input, cx| input.set_text("", cx));
                            Sonora::global(cx)
                                .devices
                                .clone()
                                .update(cx, |devices, cx| devices.install_companion(key, cx));
                        }))
                        .into_any_element(),
                )
                .detail(t!("settings-soloist-download-detail")),
            )
            .when(
                selected && matches!(state, CompanionState::Pairing | CompanionState::Verifying),
                |column| {
                    column.child(
                        Row::new(
                            t!("settings-soloist-pair"),
                            Button::new("soloist-open-spotify")
                                .label(t!("settings-soloist-open-spotify"))
                                .small()
                                .outline()
                                .on_click(|_, _, cx| cx.open_url("spotify:"))
                                .into_any_element(),
                        )
                        .detail(t!("settings-soloist-pair-detail", name = name.as_str())),
                    )
                },
            )
            .when(selected, |column| {
                column.child(ui::spoken(
                    "soloist-limitations",
                    t!("settings-soloist-limitations"),
                ))
            })
            .into_any_element()
    }
}
