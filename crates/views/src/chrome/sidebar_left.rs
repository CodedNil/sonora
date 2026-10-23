use std::rc::Rc;
use std::time::Instant;

use ui::{
    ActiveTheme as _, Button, Card, Deck, DraggedPin, Edge, MenuItem, Panel, Picker, Pin,
    Pinnable as _, SNUG, Scroller, Side, Spot, Tabs, Text, Vacancy, drop_gap, drop_marker,
};

use gpui::prelude::*;
use gpui::{
    AnyElement, App, Context, DragMoveEvent, ElementId, Entity, Hsla, MouseButton, MouseDownEvent,
    MouseUpEvent, Pixels, Render, ScrollHandle, SharedString, SpringConfig, SpringState, svg,
};
use gpui::{Window, div, px};
use router::{Destination, NavEntry, Navigation, Place, navigate, open_settings};
use state::{AppSettings, Library, Origin, PinSort, Pins, Playback, Session, Shelf, Sonora};

use super::LIBRARY_TABS;
use super::UpNext;
use super::sidebar_motion::{FLING, HELD, KNOCK, POP, POP_SPRING, rubber, settle};
use crate::shared::menus::{CardMenu, Item};
use crate::shared::nav::nav_row;

/// The one drag list the pinned section keeps, so a pin dropped anywhere in it lands in order.
const PINS: &str = "sidebar-pins";

const MIN_WIDTH: Pixels = px(160.);
const MAX_WIDTH: Pixels = px(400.);
const HINT_HEIGHT: Pixels = px(42.);
const VACANCY_HEIGHT: Pixels = px(88.);
/// How far the pin mark on a library row falls back from the accent.
const PIN_MARK: f32 = 0.7;
/// How much larger a cover is on the rail than in the full list.
const RAIL_ART: f32 = 1.5;
/// The fraction of a compact pin slot occupied by playlist artwork.
const COMPACT_PLAYLIST_ART: f32 = 0.8;
/// How tall a navigation entry on the rail is, in field heights: roomier than a row in the
/// full list, but well short of the square slot a pin takes.
const RAIL_ROW: f32 = 1.2;

/// The two navigation entries that expand into tabs rather than navigate.
#[derive(Clone, Copy, PartialEq)]
enum Group {
    Library,
    Local,
}

impl Group {
    fn of(destination: &Destination) -> Option<Self> {
        match destination {
            Destination::Library(_) => Some(Self::Library),
            Destination::Local(_) => Some(Self::Local),
            _ => None,
        }
    }
}

/// A drag of the edge in progress.
#[derive(Clone, Copy)]
struct Hold {
    /// What a fold partway through the drag added to the width the pointer asks for, so the
    /// drag carries on from the folded edge rather than from where it started.
    shift: Pixels,
    /// Whether the pull has gone past the limit and not come back.
    strained: bool,
}

pub(crate) struct SidebarLeft {
    up_next: Entity<UpNext>,
    settings: Entity<AppSettings>,
    session: Entity<Session>,
    trail: Entity<Navigation>,
    at: Destination,
    width: Pixels,
    open: bool,
    /// A close preview remains reversible until the resize drag is released.
    closing: bool,
    /// Folded down to a rail of icons rather than the full list, by choice.
    compact: bool,
    /// Folded down to the rail for now, because the window has no room for the full list but
    /// does for the rail. This is never saved, so a wider window brings the full list back.
    squeezed: bool,
    /// Whether a frame has been drawn yet, so the first one places the edge without sliding it.
    drawn: bool,
    /// How far the edge is drawn from where it rests, sprung so a pull past a limit gives and
    /// wiggles, and a fold bounces into place.
    give: SpringState,
    /// Where the give is heading: the stretch under the pointer while held, nothing once let go.
    pull: Pixels,
    spring: SpringConfig,
    hold: Option<Hold>,
    ticked: Instant,
    /// The width drawn this frame, give included.
    shown: Pixels,
    /// The side of one square slot on the rail, a cover and the inset round it, which every
    /// icon on the rail fills too.
    slot: Pixels,
    cramped: bool,
    forced: Option<bool>,
    library_open: bool,
    local_open: bool,
    pinned_open: bool,
    dropping: bool,
    drop_gap: Option<usize>,
    pin_drag: Option<usize>,
    pin_motion: ui::ReorderMotion,
    playback: Entity<Playback>,
    pins: Entity<Pins>,
    library: Entity<Library>,
    scrollbar: Entity<ui::Scrollbar>,
    popovers: ui::Popovers,
}

impl SidebarLeft {
    pub fn new(up_next: Entity<UpNext>, cx: &mut Context<Self>) -> Self {
        let settings = Sonora::global(cx).settings.clone();
        let session = Sonora::global(cx).session.clone();
        let playback = Sonora::global(cx).playback.clone();
        let pins = Sonora::global(cx).pins.clone();
        let library = Sonora::global(cx).library.clone();
        cx.observe(&library, |_, _, cx| cx.notify()).detach();
        cx.observe(&pins, |_, _, cx| cx.notify()).detach();
        cx.observe(&playback, |_, _, cx| cx.notify()).detach();
        cx.observe(&up_next, |_, _, cx| cx.notify()).detach();
        let chrome = super::Chrome::entity(cx);
        cx.observe(&chrome, |_, _, cx| cx.notify()).detach();
        let me = cx.entity_id();
        let scrollbar = cx.new(|_| ui::Scrollbar::new(ScrollHandle::new()).watching(me));
        let width = sidebar_width(settings.read(cx));
        let open = settings.read(cx).sidebar_open();
        let compact = settings.read(cx).sidebar_compact();
        let pinned_open = settings.read(cx).sidebar_pinned_open();
        let trail = router::trail(cx);

        cx.observe(&session, |_, _, cx| cx.notify()).detach();
        cx.observe(&settings, |_, _, cx| cx.notify()).detach();
        cx.observe(&trail, |_, _, cx| cx.notify()).detach();

        let at = trail.read(cx).current();
        let library_open = matches!(at, Destination::Library(_));
        let local_open = matches!(at, Destination::Local(_));

        Self {
            up_next,
            settings,
            session,
            trail,
            at,
            width,
            open,
            closing: false,
            compact,
            squeezed: false,
            drawn: false,
            give: SpringState::default(),
            pull: Pixels::ZERO,
            spring: HELD,
            hold: None,
            ticked: Instant::now(),
            shown: width,
            slot: Pixels::ZERO,
            forced: None,
            cramped: false,
            library_open,
            local_open,
            pinned_open,
            dropping: false,
            drop_gap: None,
            pin_drag: None,
            pin_motion: ui::ReorderMotion::default(),
            playback,
            pins,
            library,
            scrollbar,
            popovers: ui::Popovers::default(),
        }
    }

    fn follow(&mut self, current: &Destination) {
        if self.at == *current {
            return;
        }
        self.at = current.clone();
        let (library, local) = expanded(current);
        self.library_open |= library;
        self.local_open |= local;
    }

    /// Whether the sidebar is shown. A window too narrow even for the rail hides it until it is
    /// toggled open for now, and then it shows as the rail beside the page, never over it.
    pub fn is_open(&self) -> bool {
        self.forced.unwrap_or(self.open && !self.cramped)
    }

    pub fn occupied_width(&self) -> Pixels {
        match self.cramped && !self.is_open() {
            true => Pixels::ZERO,
            false => self.shown,
        }
    }

    /// Whether the normal-width sidebar holds the next-track preview above its bottom inset.
    pub(crate) fn embeds_up_next(&self) -> bool {
        self.is_open() && !self.folded()
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        let drawn = self.shown;
        self.closing = false;
        match self.cramped {
            true => self.forced = Some(!self.is_open()),
            false => {
                self.open = !self.open;
                self.persist(cx);
            }
        }
        self.snap(drawn, cx);
        cx.notify();
    }

    /// Starts an open or close snap from the edge currently on screen.
    fn snap(&mut self, drawn: Pixels, cx: &App) {
        self.give = SpringState {
            position: (drawn - self.rest(cx)).as_f32(),
            velocity: match self.is_open() && !self.closing {
                true => FLING,
                false => -FLING,
            },
        };
        self.pull = Pixels::ZERO;
        self.spring = POP_SPRING;
    }

    fn ceiling(&self, window: &Window, cx: &Context<Self>) -> Pixels {
        let reserved = SNUG + super::Chrome::reserved_right(cx);
        super::cap(MIN_WIDTH, MAX_WIDTH, reserved, window)
    }

    /// Fits the sidebar to the room the window leaves once `right` pixels are
    /// held back for the right sidebar: the full list where it fits, the rail
    /// where only that fits, and nothing where not even the rail does. It also
    /// steps the edge's spring to the width this frame draws. This runs inside a
    /// render, where a notify schedules nothing, so a flip asks for a full window
    /// refresh instead. That effect lands once the draw is over.
    pub fn adapt(&mut self, right: Pixels, window: &mut Window, cx: &mut Context<Self>) {
        self.width = ui::snapped(self.width, window);
        let metrics = cx.theme().metrics;
        self.slot = ui::snapped((metrics.list_row - metrics.pad * 2.) * RAIL_ART, window)
            + metrics.pad * 2.;
        if self.hold.is_some() && !cx.has_active_drag() {
            if self.closing {
                self.closing = false;
                self.snap(self.shown, cx);
            }
            self.release(cx);
        }

        let room = window.viewport_size().width - right;
        let cramped = room - self.rail(cx) < SNUG;
        let squeezed = !self.compact && !cramped && room - self.width < SNUG;
        if squeezed != self.squeezed {
            let before = self.rest(cx);
            self.squeezed = squeezed;
            // Slide from the old width to the new one rather than jump, unless the sidebar
            // is not on screen to see it.
            if self.drawn && !cramped && !self.cramped {
                let after = self.rest(cx);
                self.give.position += (before - after).as_f32();
                self.spring = POP_SPRING;
            }
            cx.refresh_windows();
        }
        if cramped != self.cramped {
            self.cramped = cramped;
            self.forced = None;
            cx.refresh_windows();
        }

        let rest = self.rest(cx);
        let give = settle(
            &mut self.give,
            &mut self.ticked,
            self.pull,
            self.spring,
            window,
            cx,
        );
        let minimum = match self.closing {
            true => cx.theme().metrics.pad * 2.,
            false => Pixels::ZERO,
        };
        self.shown = ui::snapped((rest + give).max(minimum), window);
        self.drawn = true;
    }

    /// Whether the sidebar shows as the rail, by choice or for lack of room.
    fn folded(&self) -> bool {
        self.compact || self.forced_fold()
    }

    /// Whether the window, not the user, keeps the sidebar on the rail.
    fn forced_fold(&self) -> bool {
        self.squeezed || self.cramped
    }

    /// The width of the rail: one slot, the padding either side of it inside the island, and
    /// the island's inset either side of that.
    fn rail(&self, cx: &App) -> Pixels {
        self.slot + cx.theme().metrics.pad * 4.
    }

    /// Where the edge sits when nothing pulls on it.
    fn rest(&self, cx: &App) -> Pixels {
        if !self.is_open() || self.closing {
            return Pixels::ZERO;
        }
        match self.folded() {
            true => self.rail(cx),
            false => self.width,
        }
    }

    /// Follows the edge under the pointer. In full the edge resizes freely above the minimum
    /// and gives grudgingly below it; on the rail it only gives. A pull `POP` past either limit
    /// folds or unfolds the sidebar, and the drag carries on from the new edge. A rail the
    /// window forced only gives, since the full list has no room to unfold into.
    fn stretch(&mut self, asked: Pixels, window: &Window, cx: &mut Context<Self>) {
        if !self.is_open() {
            return;
        }
        let closing = asked <= cx.theme().metrics.pad * 2.;
        if closing != self.closing {
            let drawn = self.shown;
            self.closing = closing;
            self.snap(drawn, cx);
        }
        if closing {
            self.hold.get_or_insert(Hold {
                shift: Pixels::ZERO,
                strained: false,
            });
            cx.notify();
            return;
        }
        let mut hold = self.hold.unwrap_or(Hold {
            shift: Pixels::ZERO,
            strained: false,
        });
        let asked = asked + hold.shift;
        let rest = self.rest(cx);
        let (over, inward) = match self.folded() {
            false => (MIN_WIDTH - asked, -1.),
            true => match asked < rest {
                true => (rest - asked, -1.),
                false => (asked - rest, 1.),
            },
        };

        if over >= POP && !self.forced_fold() && (!self.folded() || inward > 0.) {
            self.fold(!self.compact, asked, &mut hold, cx);
            self.hold = Some(hold);
            cx.notify();
            return;
        }

        if !self.folded() {
            let width = ui::snapped(asked.clamp(MIN_WIDTH, self.ceiling(window, cx)), window);
            if width != self.width {
                self.width = width;
                self.persist(cx);
            }
        }

        let over = over.max(Pixels::ZERO);
        let strained = over > Pixels::ZERO;
        if strained && !hold.strained {
            self.spring = HELD;
            self.give.velocity += KNOCK * inward;
        }
        hold.strained = strained;
        self.pull = rubber(over) * inward;
        self.hold = Some(hold);
        cx.notify();
    }

    /// Folds the sidebar down to its rail or out to its minimum, keeping the edge where it was
    /// drawn and flinging it towards its new place so it overshoots and settles there.
    fn fold(&mut self, compact: bool, asked: Pixels, hold: &mut Hold, cx: &mut Context<Self>) {
        let drawn = self.rest(cx) + px(self.give.position);
        self.compact = compact;
        if !compact {
            self.width = MIN_WIDTH;
        }
        let rest = self.rest(cx);
        hold.shift += rest - asked;
        hold.strained = false;
        self.give = SpringState {
            position: (drawn - rest).as_f32(),
            velocity: match compact {
                true => -FLING,
                false => FLING,
            },
        };
        self.pull = Pixels::ZERO;
        self.spring = POP_SPRING;
        self.settings
            .update(cx, |settings, cx| settings.set_sidebar_compact(compact, cx));
        self.persist(cx);
    }

    /// Lets go of the edge, which springs back from any give to where it rests.
    fn release(&mut self, cx: &mut Context<Self>) {
        if self.closing {
            self.closing = false;
            self.open = false;
            self.forced = self.forced.map(|_| false);
            self.hold = None;
            self.snap(self.shown, cx);
            self.persist(cx);
            cx.notify();
            return;
        }
        if self.hold.take().is_some_and(|hold| hold.strained) {
            self.spring = HELD;
        }
        self.pull = Pixels::ZERO;
        cx.notify();
    }

    /// The navigation entries, each followed by its tabs while its group is open.
    fn navigation(&self, window: &Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        // A shelf with rows on it belongs in the sidebar even before the session has restored,
        // since a snapshot fills it while the provider is still answering.
        let stocked = self.session.read(cx).authenticated()
            || self.library.read(cx).stocked(Shelf::Streaming);
        let mut rows = Vec::new();
        for entry in super::ordered_navigation(cx) {
            if super::place(entry, cx) != Place::Sidebar {
                continue;
            }
            if entry == NavEntry::Pins {
                rows.extend(self.pins(window, cx));
                continue;
            }
            let Some(destination) = entry.destination() else {
                continue;
            };
            let group = Group::of(&destination);
            if group == Some(Group::Library) && !stocked {
                continue;
            }
            match group.filter(|_| self.folded()) {
                Some(group) => rows.push(self.compact_tabs(group, cx)),
                None => rows.push(self.nav(entry, cx)),
            }
            if let Some(group) = group.filter(|group| !self.folded() && self.opened(*group)) {
                rows.push(self.tabs(group, cx));
            }
        }
        rows
    }

    /// The pinned section: its header, and its entries once it is expanded. The entries are a
    /// `Deck`, so only the ones on screen are ever built.
    fn pins(&self, window: &Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        if super::place(NavEntry::Pins, cx) != Place::Sidebar {
            return Vec::new();
        }

        let mut rows = match self.folded() {
            true => Vec::new(),
            false => vec![self.pins_header(cx)],
        };
        if !self.pinned_open && !self.folded() {
            return rows;
        }

        let pins = self.pins.read(cx);
        let pinned = pins.entries(cx);
        let rest = match self.settings.read(cx).sidebar_full_library() {
            true => pins.library(cx),
            false => Rc::new(Vec::new()),
        };
        if pinned.is_empty() && rest.is_empty() {
            if self.folded() {
                return rows;
            }
            rows.push(match self.dropping {
                true => hint(cx),
                false => vacancy(),
            });
            return rows;
        }

        let held = pinned.len();
        let count = held + rest.len();
        let gap = cx.theme().metrics.pad;
        let heights = pinned
            .iter()
            .chain(rest.iter())
            .map(|pin| match self.folded() {
                true => self.pin_art(pin),
                false => ui::snapped(cx.theme().metrics.list_row, window),
            })
            .collect::<Vec<_>>();
        let mut order = (0..count).collect::<Vec<_>>();
        order.sort_by_key(|&index| match index < held {
            true => ui::reorder_index(index, self.pin_drag, self.drop_gap),
            false => index,
        });
        let mut targets = vec![Pixels::ZERO; count];
        let mut cursor = Pixels::ZERO;
        for index in order {
            targets[index] = cursor;
            cursor += heights[index] + gap;
        }
        let mut cursor = Pixels::ZERO;
        let positions = heights
            .iter()
            .map(|height| {
                let position = cursor;
                cursor += *height + gap;
                position
            })
            .collect::<Vec<_>>();
        self.pin_motion.animate(
            pinned
                .iter()
                .chain(rest.iter())
                .enumerate()
                .map(|(index, pin)| {
                    let key = SharedString::from(format!("{}:{}", pin.kind.key(), pin.id));
                    (key, targets[index], false)
                }),
            window,
            cx,
        );
        rows.push(
            Deck::new("sidebar-pins-deck")
                .overflow_rows()
                .rows(heights)
                .gap(gap)
                .draw(cx.processor(move |this, index: usize, _, cx| {
                    let pin = match index < held {
                        true => pinned.get(index),
                        false => rest.get(index - held),
                    };
                    match pin {
                        Some(pin) => this.pin_row(index, pin.clone(), held, positions[index], cx),
                        None => div().into_any_element(),
                    }
                }))
                .into_any_element(),
        );
        rows
    }

    fn opened(&self, group: Group) -> bool {
        match group {
            Group::Library => self.library_open,
            Group::Local => self.local_open,
        }
    }

    fn flip(&mut self, group: Group) {
        let open = match group {
            Group::Library => &mut self.library_open,
            Group::Local => &mut self.local_open,
        };
        *open = !*open;
    }

    fn pins_header(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = *cx.theme();
        let open = self.pinned_open;

        div()
            .id("sidebar-pins-header")
            .flex()
            .flex_none()
            .items_center()
            .justify_between()
            .w_full()
            .min_w_0()
            .h(theme.metrics.control_small)
            .pl_2()
            // Narrower on the right than on the left, so the sort glyph falls on the same
            // line as the chevrons of the rows above.
            .pr_1()
            .mt(theme.metrics.pad)
            .mb(px(2.))
            .child(
                div()
                    .id("sidebar-pins-toggle")
                    .flex()
                    .flex_1()
                    .min_w_0()
                    .items_center()
                    .gap_1()
                    .cursor_pointer()
                    .child(
                        svg()
                            .path(icons::path(chevron(open)))
                            .flex_none()
                            .size(theme.text(Text::Small))
                            .text_color(theme.muted_foreground),
                    )
                    .child(
                        ui::eyebrow(i18n::lookup("nav-pinned", None), cx)
                            .min_w_0()
                            .truncate(),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.pinned_open = !this.pinned_open;
                        let open = this.pinned_open;
                        this.settings.update(cx, |settings, cx| {
                            settings.set_sidebar_pinned_open(open, cx)
                        });
                        cx.notify();
                    })),
            )
            .when(open, |header| header.child(self.sort_picker(cx)))
            // Dragging over the header aims at the top of the list, which is otherwise
            // out of reach above the first entry.
            .on_drag_move(
                cx.listener(|this, event: &DragMoveEvent<DraggedPin>, _, cx| {
                    if !event.bounds.contains(&event.event.position) || this.drop_gap == Some(0) {
                        return;
                    }
                    this.drop_gap = Some(0);
                    cx.notify();
                }),
            )
            .into_any_element()
    }

    fn sort_picker(&self, cx: &App) -> AnyElement {
        let chosen = self.pins.read(cx).sort(cx);
        let reversed = self.pins.read(cx).reversed(cx);
        let full = self.settings.read(cx).sidebar_full_library();
        let arrow = match reversed {
            true => "icons/chevron-down.svg",
            false => "icons/chevron-up.svg",
        };
        let pins = self.pins.clone();

        Picker::icon(
            "sidebar-pins-sort",
            &self.popovers,
            "icons/arrow-up-down.svg",
        )
        .tooltip("tool-sort")
        .sticky()
        .width(Picker::NARROW)
        .tint(match self.pins.read(cx).sorted(cx) {
            true => cx.theme().primary,
            false => cx.theme().muted_foreground,
        })
        .items(PinSort::ALL.into_iter().map(move |sort| {
            let pins = pins.clone();

            MenuItem::new(sort.id(), i18n::lookup(sort.key(), None))
                .selected(Some(sort) == chosen)
                .when(Some(sort) == chosen, |item| item.icon(arrow))
                .on_click(move |_, _, cx| {
                    pins.update(cx, |pins, cx| pins.choose(sort, cx));
                })
        }))
        .item(MenuItem::separator("sidebar-pins-scope"))
        .item(
            MenuItem::new(
                "sidebar-full-library",
                i18n::lookup("nav-show-full-library", None),
            )
            .checked(full)
            .on_click({
                let settings = self.settings.clone();
                move |_, _, cx| {
                    settings.update(cx, |settings, cx| {
                        settings.set_sidebar_full_library(!full, cx)
                    });
                }
            }),
        )
        .into_any_element()
    }

    /// The compact artwork height also determines its row height, leaving one gap between covers.
    fn pin_art(&self, pin: &Pin) -> Pixels {
        match pin.kind {
            ui::PinKind::Playlist => self.slot * COMPACT_PLAYLIST_ART,
            _ => self.slot,
        }
    }

    /// One entry. The first `count` of them are the pins, which carry the drop slots. The rest
    /// is the library underneath, which can be dragged up into the pins but holds no slot.
    fn pin_row(
        &self,
        index: usize,
        pin: Pin,
        count: usize,
        position: Pixels,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let art = self.pin_art(&pin);
        let theme = *cx.theme();
        let accent = theme.sidebar_accent;
        let destination = Destination::from(&pin);
        let active = destination == self.trail.read(cx).current();
        let opened = pin.clone();
        let held = index < count;
        // Only worth marking when the library sits alongside, since otherwise every row is a pin.
        let marked = held && self.settings.read(cx).sidebar_full_library();
        let edge = match self.drop_gap {
            Some(gap) if held && gap == index => Some(Edge::Above),
            Some(gap) if gap == count && index + 1 == count => Some(Edge::Below),
            _ => None,
        };

        let origin = Origin::from(&pin);
        let playing = self.playback.read(cx).playing_from(&origin, cx) == Some(true);

        let card = Card::new(("pinned", index), pin.label())
            .cover(pin.cover.clone())
            .fallback(pin.kind.icon())
            .when(pin.kind.round(), Card::circle)
            // On the rail the artwork is all there is to click, so it opens the pin rather than
            // carrying a play button over the whole of it.
            .when(!self.folded(), |card| {
                card.play(
                    playing,
                    cx.listener(move |this, _, _, cx| {
                        this.playback
                            .update(cx, |playback, cx| playback.toggle_origin(&origin, cx));
                    }),
                )
            })
            .tint(match active {
                true => theme.foreground,
                false => theme.muted_foreground,
            })
            .meta(caption(pin.kind.key(), marked, cx))
            .when(self.folded(), |card| {
                card.bare_art().flat().art(art).flex_none().w_full().h(art)
            })
            .when(!self.folded(), |card| {
                card.when(active, |card| card.bg(accent))
                    .hover(move |style| style.bg(accent))
            })
            .press(move |_, _, cx| navigate(destination.clone(), cx))
            .menu(cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                CardMenu::open(
                    Item::Pin(opened.clone()),
                    this.playback.clone(),
                    event.position,
                    cx,
                );
            }))
            .when_else(
                held,
                |card| card.pin_from(pin.clone(), Spot::new(PINS, index)),
                |card| card.pin(pin.clone()),
            );

        let key = format!("{}:{}", pin.kind.key(), pin.id);
        let offset = self.pin_motion.position(&key, position) - position;
        div()
            .id(("pinned-slot", index))
            .relative()
            .flex_none()
            .w_full()
            .min_w_0()
            .child(card.relative().top(offset))
            .on_drag_move(
                cx.listener(move |this, event: &DragMoveEvent<DraggedPin>, _, cx| {
                    if !event.bounds.contains(&event.event.position) {
                        return;
                    }
                    let Some(gap) = drop_gap(event.bounds, event.event.position, index) else {
                        return;
                    };
                    let gap = Some(match held {
                        true => gap,
                        false => count,
                    });
                    if this.drop_gap != gap {
                        this.drop_gap = gap;
                        cx.notify();
                    }
                }),
            )
            .when_some(edge, |this, edge| this.child(drop_marker(edge, cx)))
            .into_any_element()
    }

    /// One navigation entry. An expandable one flips its group open and shut, any other one
    /// navigates and is lit while its section is current.
    fn nav(&self, entry: NavEntry, cx: &mut Context<Self>) -> AnyElement {
        let theme = *cx.theme();
        let accent = theme.sidebar_accent;
        let index = entry as usize;
        let (key, icon) = (entry.key(), entry.icon());
        let Some(destination) = entry.destination() else {
            return div().into_any_element();
        };
        let current = self.trail.read(cx).current();
        let group = Group::of(&destination);
        let active = match group {
            Some(group) => Group::of(&current) == Some(group),
            None => destination.same_section(&current),
        };
        let tint = match active {
            true => theme.foreground,
            false => theme.muted_foreground,
        };
        if self.folded() {
            return rail_row(index, key, tint, accent)
                .icon(icon)
                .icon_size(theme.text(Text::Title))
                .h((theme.metrics.field * RAIL_ROW).round())
                .when(active, |button| button.bg(accent))
                .on_click(move |_, _, cx| navigate(destination.clone(), cx))
                .into_any_element();
        }
        let row = nav_row(index, key, tint, accent).icon(icon);

        match group {
            Some(group) => row
                .trailing(chevron(self.opened(group)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.flip(group);
                    cx.notify();
                })),
            None => row
                .when(active, |button| button.bg(accent))
                .on_click(move |_, _, cx| match destination.in_settings() {
                    true => open_settings(cx),
                    false => navigate(destination.clone(), cx),
                }),
        }
        .into_any_element()
    }

    /// The compact library's four destinations, always reachable without expanding a group.
    fn compact_tabs(&self, group: Group, cx: &mut Context<Self>) -> AnyElement {
        let theme = *cx.theme();
        let current = self.trail.read(cx).current();
        let stocked = self.session.read(cx).authenticated()
            || self.library.read(cx).stocked(Shelf::Streaming);
        let show_group_labels = stocked
            && super::place(NavEntry::Library, cx) == Place::Sidebar
            && super::place(NavEntry::Local, cx) == Place::Sidebar;
        let icons = [
            "icons/music.svg",
            "icons/disc-3.svg",
            "icons/user-round.svg",
            "icons/list-music.svg",
        ];
        let labels = match group {
            Group::Library => [
                "nav-library-songs",
                "nav-library-albums",
                "nav-library-artists",
                "nav-library-playlists",
            ],
            Group::Local => [
                "nav-local-songs",
                "nav-local-albums",
                "nav-local-artists",
                "nav-local-playlists",
            ],
        };
        div()
            .flex()
            .flex_col()
            .gap_1()
            .when(show_group_labels, |this| {
                this.child(
                    ui::eyebrow(
                        i18n::lookup(
                            match group {
                                Group::Library => "nav-compact-library",
                                Group::Local => "nav-compact-local",
                            },
                            None,
                        ),
                        cx,
                    )
                    .w_full()
                    .text_center(),
                )
            })
            .children(LIBRARY_TABS.chunks(2).enumerate().map(|(row, tabs)| {
                div()
                    .flex()
                    .gap_1()
                    .children(tabs.iter().enumerate().map(|(column, &(_, tab))| {
                        let index = row * 2 + column;
                        let destination = match group {
                            Group::Library => Destination::Library(tab),
                            Group::Local => Destination::Local(tab),
                        };
                        let active = current == destination;
                        let tint = match active {
                            true => theme.foreground,
                            false => theme.muted_foreground,
                        };
                        rail_row(
                            (
                                match group {
                                    Group::Library => "compact-library",
                                    Group::Local => "compact-local",
                                },
                                index,
                            ),
                            labels[index],
                            tint,
                            theme.sidebar_accent,
                        )
                        .icon(icons[index])
                        .flex_1()
                        .min_w_0()
                        .w_auto()
                        .px_0()
                        .h(theme.metrics.control)
                        .when(active, |button| button.bg(theme.sidebar_accent))
                        .on_click(move |_, _, cx| navigate(destination.clone(), cx))
                    }))
            }))
            .into_any_element()
    }

    /// The tabs under an expanded group.
    fn tabs(&self, group: Group, cx: &mut Context<Self>) -> AnyElement {
        let theme = *cx.theme();
        let accent = theme.sidebar_accent;
        let current = self.trail.read(cx).current();
        let tab = |id: ElementId, key: &'static str, destination: Destination| {
            let chosen = destination == current;
            let tint = match chosen {
                true => theme.foreground,
                false => theme.muted_foreground,
            };

            nav_row(id, key, tint, accent)
                .flex_1()
                .when(chosen, |button| button.bg(accent))
                .on_click(move |_, _, cx| navigate(destination.clone(), cx))
        };

        Tabs::new()
            .items(
                LIBRARY_TABS
                    .into_iter()
                    .enumerate()
                    .map(|(slot, (name, tab_id))| {
                        let (id, destination) = match group {
                            Group::Library => (name.into(), Destination::Library(tab_id)),
                            Group::Local => {
                                (("local-tab", slot).into(), Destination::Local(tab_id))
                            }
                        };
                        tab(id, name, destination)
                    }),
            )
            .into_any_element()
    }

    fn persist(&self, cx: &mut Context<Self>) {
        let width = self.width / px(1.);
        let open = self.open;
        self.settings
            .update(cx, |settings, cx| settings.set_sidebar(width, open, cx));
    }
}

impl Render for SidebarLeft {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.theme();

        let current = self.trail.read(cx).current();
        self.follow(&current);
        self.adapt(super::Chrome::reserved_right(cx), window, cx);
        let pad = theme.metrics.pad;
        let up_next = match self.embeds_up_next() {
            true => self.up_next.read(cx).inline(window, cx),
            false => None,
        };

        if !cx.has_active_drag() {
            self.dropping = false;
            self.drop_gap = None;
            self.pin_drag = None;
        }

        let rows = self.navigation(window, cx);

        // The panel is given no limits, so its grip reports the width the pointer asks for and
        // `stretch` decides how much of it the edge follows.
        Panel::new("sidebar-left", Side::Left, self.shown)
            .content_width(match self.folded() {
                true => self.rail(cx),
                false => self.width,
            })
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, cx| this.release(cx)),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, cx| this.release(cx)),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|_, event: &MouseDownEvent, window, cx| {
                    window.prevent_default();
                    cx.stop_propagation();
                    CardMenu::dismiss(cx);
                    super::LayoutEditor::open(event.position, cx);
                    cx.notify();
                }),
            )
            .clears_scrollbar()
            .when(self.is_open(), |panel| {
                panel.on_resize(cx.listener(|this, width: &Pixels, window, cx| {
                    this.stretch(*width, window, cx);
                }))
            })
            .on_drag_move(
                cx.listener(|this, event: &DragMoveEvent<DraggedPin>, _, cx| {
                    let from = event.drag(cx).spot(PINS).map(|spot| spot.index);
                    if this.pin_drag != from {
                        this.pin_drag = from;
                        cx.notify();
                    }
                    if this.dropping {
                        return;
                    }
                    this.dropping = true;
                    // A drag has nowhere to land while the section is closed, so open it.
                    if !this.pinned_open {
                        this.pinned_open = true;
                        this.settings.update(cx, |settings, cx| {
                            settings.set_sidebar_pinned_open(true, cx)
                        });
                    }
                    cx.notify();
                }),
            )
            .on_drop(cx.listener(|this, dragged: &DraggedPin, _, cx| {
                let gap = this.drop_gap.take();
                this.dropping = false;
                let pin = dragged.pin.clone();
                this.pins.update(cx, |pins, cx| pins.place(pin, gap, cx));
                cx.notify();
            }))
            .when(self.shown <= Pixels::ZERO, |this| this.hidden())
            .overflow_hidden()
            .island()
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h_0()
                    // GPUI clips to rectangles, so keep the scroll viewport inside the
                    // island’s rounded ends even when its first row has scrolled away.
                    .pt(pad)
                    .overflow_hidden()
                    .child(
                        Scroller::new("sidebar-left-rows", &self.scrollbar)
                            .flex_1()
                            .min_h_0()
                            .w_full()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .w_full()
                                    .px(pad)
                                    .pb(pad)
                                    .children(rows.into_iter().enumerate().map(|(index, row)| {
                                        self.scrollbar
                                            .read(cx)
                                            .anchors()
                                            .mark(("navigation", index), row)
                                    })),
                            ),
                    )
                    .children(ui::return_top("sidebar-return-top", &self.scrollbar, cx))
                    .children(up_next),
            )
    }
}

/// How wide the left sidebar stands from its saved width, which the settings rail matches.
pub(crate) fn sidebar_width(settings: &AppSettings) -> Pixels {
    px(settings.sidebar_width()).clamp(MIN_WIDTH, MAX_WIDTH)
}

/// The kind of an entry, behind a dimmed pin when it is one of the pinned ones.
fn caption(kind: &'static str, marked: bool, cx: &App) -> impl IntoElement {
    let theme = *cx.theme();

    div()
        .flex()
        .items_center()
        .gap(theme.metrics.pad / 4.)
        .min_w_0()
        .when(marked, |row| {
            row.child(
                svg()
                    .path(icons::path("icons/pin.svg"))
                    .flex_none()
                    .size(theme.text(Text::Tiny))
                    .text_color(theme.primary.opacity(PIN_MARK)),
            )
        })
        .child(div().min_w_0().truncate().child(i18n::lookup(kind, None)))
}

fn hint(cx: &App) -> AnyElement {
    let theme = *cx.theme();

    div()
        .flex()
        .flex_none()
        .min_w_0()
        .items_center()
        .justify_center()
        // A narrow sidebar wraps the line, so the box grows rather than the text spilling out.
        .min_h(HINT_HEIGHT)
        .mx_2()
        .px_2()
        .py_1()
        .rounded(theme.radius)
        .border_1()
        .border_dashed()
        .border_color(theme.sidebar_border)
        .text_size(theme.text(Text::Small))
        .text_color(theme.muted_foreground)
        .text_center()
        .child(div().min_w_0().child(i18n::lookup("nav-pin-hint", None)))
        .into_any_element()
}

fn vacancy() -> AnyElement {
    Vacancy::new(i18n::lookup("nav-nothing-pinned", None))
        .icon("icons/pin.svg")
        .compact()
        .flex_none()
        .h(VACANCY_HEIGHT)
        .px_2()
        .into_any_element()
}

/// Which groups the route opens: Your Library and Local Music, in that order.
fn expanded(current: &Destination) -> (bool, bool) {
    (
        matches!(current, Destination::Library(_)),
        matches!(current, Destination::Local(_)),
    )
}

fn chevron(open: bool) -> &'static str {
    match open {
        true => "icons/chevron-down.svg",
        false => "icons/chevron-right.svg",
    }
}

/// One icon on the rail, filling its width, with its name in a tooltip.
fn rail_row(id: impl Into<ElementId>, key: &'static str, tint: Hsla, accent: Hsla) -> Button {
    Button::new(id)
        .ghost()
        .tooltip(key)
        .tint(tint)
        .w_full()
        .hover(move |style| style.bg(accent))
        .active(move |style| style.bg(accent))
}

#[cfg(test)]
mod tests {
    use router::{Destination, LibraryTab, SettingsPage};

    use super::expanded;

    #[test]
    fn a_section_expands_only_where_it_leads() {
        assert_eq!(
            expanded(&Destination::Library(LibraryTab::Albums)),
            (true, false)
        );
        assert_eq!(
            expanded(&Destination::Local(LibraryTab::Albums)),
            (false, true)
        );
        assert_eq!(
            expanded(&Destination::Settings(SettingsPage::Providers)),
            (false, false)
        );
    }

    #[test]
    fn content_belongs_to_neither_section() {
        let away = [
            Destination::Home,
            Destination::Search,
            Destination::Album("id".into()),
            Destination::Playlist("id".into()),
            Destination::Artist("id".into()),
            Destination::Song("id".into()),
        ];

        for destination in away {
            assert_eq!(expanded(&destination), (false, false), "{destination:?}");
        }
    }
}
