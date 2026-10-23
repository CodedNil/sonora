use std::time::Instant;

use gpui::prelude::*;
use gpui::{
    Context, Entity, MouseButton, MouseUpEvent, Pixels, Render, SpringState, Window, div, px,
};
use state::{AppSettings, Cards, Playback, Queue, SideTab, Sonora};
use ui::{ActiveTheme as _, MIN_CONTENT, Panel, Room, Side};

use super::sidebar_motion::{FLING, HELD, KNOCK, POP_SPRING, rubber, settle};
use crate::chrome::Aside;

const MIN_WIDTH: Pixels = px(240.);
const MAX_WIDTH: Pixels = px(560.);

fn fills_content(width: Pixels) -> bool {
    !Room::of(width).fits(Room::Wide)
}

pub(crate) struct SidebarRight {
    aside: Entity<Aside>,
    settings: Entity<AppSettings>,
    width: Pixels,
    open: bool,
    closing: bool,
    shown: Pixels,
    give: SpringState,
    pull: Pixels,
    strained: bool,
    ticked: Instant,
}

impl SidebarRight {
    pub(crate) fn new(
        queue: Entity<Queue>,
        playback: Entity<Playback>,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings = Sonora::global(cx).settings.clone();
        let width = px(settings.read(cx).sidebar_right_width()).clamp(MIN_WIDTH, MAX_WIDTH);
        let open = settings.read(cx).sidebar_right_open();
        let aside = cx.new(|cx| Aside::new(queue, playback, cx));

        Self {
            aside,
            settings,
            width,
            open,
            closing: false,
            shown: match open {
                true => width,
                false => Pixels::ZERO,
            },
            give: SpringState::default(),
            pull: Pixels::ZERO,
            strained: false,
            ticked: Instant::now(),
        }
    }

    pub(crate) fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn available(window: &Window) -> bool {
        !fills_content(window.viewport_size().width)
    }

    pub(crate) fn covers_content(&self, _window: &Window) -> bool {
        false
    }

    pub(crate) fn occupied_width(&self, window: &Window) -> Pixels {
        match Self::available(window) {
            false => Pixels::ZERO,
            true => self.shown,
        }
    }

    /// The width the left sidebar has to leave free for this one. It counts an open sidebar
    /// even while the window is too narrow to show it, so the left sidebar collapses first and
    /// stays collapsed when this one hides in turn.
    pub(crate) fn reserved_width(&self) -> Pixels {
        match self.open {
            false => self.shown,
            true => self.width,
        }
    }

    pub(crate) fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.open {
            return self.close(cx);
        }
        self.open = true;
        self.closing = false;
        self.snap(true);
        self.remember(cx);
        cx.notify();
    }

    /// Lets `tab`'s card fill the sidebar, opening it if it is closed, or puts the sidebar back to
    /// all its cards if that card already fills it.
    pub(crate) fn show(&mut self, tab: SideTab, cx: &mut Context<Self>) {
        let cards = self.aside.read(cx).cards(cx);
        let cards = match self.open && cards.expanded == Some(tab) {
            true => cards.expand(tab),
            false => Cards {
                expanded: Some(tab),
                ..cards.showing(tab, true)
            },
        };
        self.aside
            .update(cx, |aside, cx| aside.set_cards(cards, cx));
        if !self.open {
            self.open = true;
            self.closing = false;
            self.snap(true);
            self.remember(cx);
        }
        cx.notify();
    }

    pub(crate) fn close(&mut self, cx: &mut Context<Self>) {
        self.aside.update(cx, |aside, cx| aside.dismiss(cx));
        if !self.open {
            return;
        }
        self.open = false;
        self.closing = false;
        self.snap(false);
        self.remember(cx);
        cx.notify();
    }

    /// Shares the left sidebar's snap tuning while preserving the width to reopen at.
    fn snap(&mut self, open: bool) {
        let rest = match open {
            true => self.width,
            false => Pixels::ZERO,
        };
        self.give = SpringState {
            position: (self.shown - rest).as_f32(),
            velocity: match open {
                true => FLING,
                false => -FLING,
            },
        };
        self.pull = Pixels::ZERO;
        self.strained = false;
    }

    /// Resolves the animated edge before the workspace reserves space for the sidebar.
    pub(crate) fn adapt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !cx.has_active_drag() {
            if self.closing {
                self.closing = false;
                self.snap(true);
            }
            self.release(cx);
        }
        let spring = match self.strained {
            true => HELD,
            false => POP_SPRING,
        };
        let give = settle(
            &mut self.give,
            &mut self.ticked,
            self.pull,
            spring,
            window,
            cx,
        );
        let rest = match self.open && !self.closing {
            true => self.width,
            false => Pixels::ZERO,
        };
        let minimum = match self.closing {
            true => cx.theme().metrics.pad * 2.,
            false => Pixels::ZERO,
        };
        self.shown = ui::snapped((rest + give).max(minimum), window);
    }

    /// Resizes freely above the minimum, gives below it, and snaps shut at the window edge.
    fn stretch(&mut self, asked: Pixels, window: &Window, cx: &mut Context<Self>) {
        if !self.open {
            return;
        }
        let closing = asked <= cx.theme().metrics.pad * 2.;
        if closing != self.closing {
            self.closing = closing;
            self.snap(!closing);
        }
        if closing {
            cx.notify();
            return;
        }
        let width = ui::snapped(
            asked.clamp(
                MIN_WIDTH,
                super::cap(MIN_WIDTH, MAX_WIDTH, MIN_CONTENT, window),
            ),
            window,
        );
        if width != self.width {
            self.width = width;
            self.persist(cx);
        }
        let over = (MIN_WIDTH - asked).max(Pixels::ZERO);
        if over > Pixels::ZERO && !self.strained {
            self.give.velocity -= KNOCK;
        }
        self.strained = over > Pixels::ZERO;
        self.pull = -rubber(over);
        cx.notify();
    }

    /// Commits a close preview only after the pointer is released.
    fn release(&mut self, cx: &mut Context<Self>) {
        if self.closing {
            self.close(cx);
        }
        let moved = self.pull != Pixels::ZERO || self.strained;
        self.pull = Pixels::ZERO;
        self.strained = false;
        if moved {
            cx.notify();
        }
    }

    fn remember(&self, cx: &mut Context<Self>) {
        let open = self.open;
        self.settings
            .update(cx, |settings, cx| settings.set_sidebar_right_open(open, cx));
    }

    fn persist(&self, cx: &mut Context<Self>) {
        let width = self.width / px(1.);
        self.settings.update(cx, |settings, cx| {
            settings.set_sidebar_right_width(width, cx)
        });
    }
}

impl Render for SidebarRight {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.shown <= Pixels::ZERO || !Self::available(window) {
            self.aside.update(cx, |aside, cx| aside.pause_clip(cx));
            return div().into_any_element();
        }

        Panel::new("sidebar-right", Side::Right, self.shown)
            .content_width(self.width)
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, cx| this.release(cx)),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, cx| this.release(cx)),
            )
            .overflow_hidden()
            .grip_above(super::Chrome::bottom(cx))
            .when(self.open, |panel| {
                panel.on_resize(cx.listener(|this, width: &Pixels, window, cx| {
                    this.stretch(*width, window, cx);
                }))
            })
            .island()
            .child(self.aside.clone())
            .into_any_element()
    }
}
