use gpui::prelude::*;
use gpui::{App, Context, Entity, Global, MouseDownEvent, Pixels, Point, Window, div};
use state::Playback;
use ui::{Popup, Scrollbar};

use crate::shared::menus::{Item, ItemMenu};

/// The one context menu every album, playlist, artist and track card opens, drawn once at the
/// root of either shell. A card factory attaches it, so no screen has to keep menu state of
/// its own for a card to have one.
pub(crate) struct CardMenu {
    menus: ItemMenu,
    open: Option<Opened>,
}

/// The item a menu is open on, the playback its entries act through, and where it was asked for.
struct Opened {
    item: Item,
    playback: Entity<Playback>,
    at: Point<Pixels>,
}

struct Installed(Entity<CardMenu>);

impl Global for Installed {}

impl CardMenu {
    pub fn entity(cx: &mut App) -> Entity<Self> {
        if cx.try_global::<Installed>().is_none() {
            let menu = cx.new(|cx: &mut Context<Self>| {
                let host = cx.entity_id();
                let scrollbar = cx.new(|_| Scrollbar::inset().watching(host));
                let session = state::Sonora::global(cx).session.clone();
                cx.subscribe(&session, |this: &mut Self, _, _, cx| {
                    this.open = None;
                    cx.notify();
                })
                .detach();
                let trail = router::trail(cx);
                cx.subscribe(
                    &trail,
                    |this: &mut Self, _, _: &router::NavigationEvent, cx| {
                        this.open = None;
                        cx.notify();
                    },
                )
                .detach();
                let library = state::Sonora::global(cx).library.clone();
                cx.observe(&library, |_, _, cx| cx.notify()).detach();
                let sonora = state::Sonora::global(cx);
                let settings = sonora.settings.clone();
                let downloads = sonora.downloads.clone();
                let pins = sonora.pins.clone();
                cx.observe(&settings, |_, _, cx| cx.notify()).detach();
                cx.observe(&downloads, |_, _, cx| cx.notify()).detach();
                cx.observe(&pins, |_, _, cx| cx.notify()).detach();
                Self {
                    menus: ItemMenu::new(scrollbar, cx),
                    open: None,
                }
            });
            cx.set_global(Installed(menu));
        }
        cx.global::<Installed>().0.clone()
    }

    /// Opens the shared media menu, resetting submenu state from the last item.
    pub(crate) fn open(item: Item, playback: Entity<Playback>, at: Point<Pixels>, cx: &mut App) {
        Self::entity(cx).update(cx, |this, cx| {
            this.menus.reset(cx);
            this.open = Some(Opened { item, playback, at });
            cx.notify();
        });
    }

    /// Closes the shared popup before another chrome menu or surface takes over.
    pub(crate) fn dismiss(cx: &mut App) {
        if let Some(menu) = cx
            .try_global::<Installed>()
            .map(|installed| installed.0.clone())
        {
            menu.update(cx, |this, cx| {
                this.open = None;
                cx.notify();
            });
        }
    }

    /// Keeps fullscreen controls awake while the shared menu is being used.
    pub(crate) fn is_open(cx: &App) -> bool {
        cx.try_global::<Installed>()
            .is_some_and(|menu| menu.0.read(cx).open.is_some())
    }

    /// Opens on the shown song at click time, shared by player-bar and fullscreen artwork and titles.
    pub(crate) fn playing(event: &MouseDownEvent, window: &mut Window, cx: &mut App) {
        let playback = state::Sonora::global(cx).playback.clone();
        let Some(track) = playback.read(cx).shown_track(cx) else {
            return;
        };
        Self::opener(Item::Track(track), playback)(event, window, cx);
    }

    /// The right-click handler a card takes, which opens the menu of `item` where the pointer is.
    pub(crate) fn opener(
        item: Item,
        playback: Entity<Playback>,
    ) -> impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static {
        move |event, window, cx| {
            window.prevent_default();
            cx.stop_propagation();
            Self::open(item.clone(), playback.clone(), event.position, cx);
        }
    }
}

impl Render for CardMenu {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(open) = &self.open else {
            return div();
        };
        let menu = open.item.menu(&self.menus, open.playback.clone(), cx);

        div().child(
            Popup::new(open.at, menu).on_close(cx.listener(|this, _, _, cx| {
                this.open = None;
                cx.notify();
            })),
        )
    }
}
