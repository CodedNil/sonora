use gpui::prelude::*;
use gpui::{AnyElement, App, Context};
use i18n::t;
use state::{DiscordName, Sonora};
use ui::{Button, MenuItem, Picker, TabBar};

use super::{Group, SettingsView, Slot};

const DISCORD_NAME: &str = "discord-name";

impl SettingsView {
    /// The Discord page. While the status is off only the master switch and the local files
    /// row stand, since the rest shapes a status nobody sees.
    pub(super) fn discord_groups(&self, cx: &App) -> Vec<Group> {
        let settings = self.settings.read(cx);
        let mut groups = vec![Group::untitled().rows([Slot::Discord])];
        if settings.discord_presence() {
            // the choice only means something while Spotify is signed in and listed as a device
            let connect =
                settings.spotify_connect() && Sonora::global(cx).devices.read(cx).supported();
            groups.push(
                Group::new("settings-group-status", "icons/radio.svg")
                    .rows([
                        Slot::DiscordName,
                        Slot::DiscordShowPaused,
                        Slot::DiscordBadge,
                        Slot::DiscordAnonymous,
                    ])
                    .rows(connect.then_some(Slot::DiscordHideOnConnect)),
            );
            groups.push(
                Group::new("settings-discord-buttons", "icons/link.svg")
                    .rows([Slot::DiscordButtons]),
            );
        }
        groups.push(
            Group::new("settings-group-local-files", "icons/file-music.svg")
                .rows([Slot::ArtworkForLocal]),
        );
        groups
    }

    pub(super) fn discord_name_row(&self, cx: &mut Context<Self>) -> AnyElement {
        let chosen = self.settings.read(cx).discord_name();

        let picker = Picker::new(
            DISCORD_NAME,
            &self.popovers,
            i18n::lookup(chosen.key(), None),
        )
        .width(Picker::NARROW)
        .items(DiscordName::ALL.map(|name| {
            MenuItem::new(name.id(), i18n::lookup(name.key(), None))
                .selected(name == chosen)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.settings
                        .update(cx, |settings, cx| settings.set_discord_name(name, cx));
                    cx.notify();
                }))
        }));

        self.row(t!("settings-discord-name"), picker.into_any_element())
    }

    /// The buttons under the status, each a segment that turns on and off by itself.
    pub(super) fn discord_buttons_row(&self, cx: &mut Context<Self>) -> AnyElement {
        let settings = self.settings.read(cx);
        let sonora = settings.discord_sonora_button();
        let provider = settings.discord_provider_button();

        let buttons = TabBar::new("discord-buttons").flex_none().items([
            Button::new("discord-button-provider")
                .label(t!("settings-discord-name-provider"))
                .small()
                .ghost()
                .selected(provider)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.settings.update(cx, |settings, cx| {
                        settings.set_discord_provider_button(!provider, cx)
                    });
                    cx.notify();
                })),
            Button::new("discord-button-sonora")
                .label(t!("settings-discord-name-sonora"))
                .small()
                .ghost()
                .selected(sonora)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.settings.update(cx, |settings, cx| {
                        settings.set_discord_sonora_button(!sonora, cx)
                    });
                    cx.notify();
                })),
        ]);

        self.row(t!("settings-discord-buttons"), buttons.into_any_element())
    }
}
