use std::cell::Cell;
use std::rc::Rc;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use gpui::Decorations;
use gpui::accesskit::Role;
use gpui::prelude::*;
use gpui::{AnyElement, App, Window, div};
use gpui::{
    Bounds, Context, Entity, EventEmitter, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Render,
};
use input::SEARCH_CONTEXT;
use ui::TrafficLightControls;
use ui::WindowControls;
use ui::{
    ActiveTheme as _, Button, Input, Menu, MenuItem, Picker, Popover, Popovers, SelectNext, Spin,
    Text,
};

use crate::chrome::SidebarRight;
use input::Refresh;
use router::{Destination, NavEntry, Navigation, Place, navigate, open_settings};
use state::{AppSettings, Shelf, Sonora};

const SYSTEM_ZOOMS: bool = cfg!(target_os = "windows");
/// How wide the search field grows, in field heights, before the bar leaves the rest as space.
const SEARCH_SPAN: f32 = 12.;
/// Tells whether the page a refresh reloaded is still waiting for its data.
pub(crate) type Busy = Box<dyn Fn(&App) -> bool>;

/// The height of AppKit's traffic-light buttons, which the bar centres them by.
#[cfg(target_os = "macos")]
const TRAFFIC_LIGHTS_SIZE: f32 = 14.;
/// The extra room round corners leave before the traffic lights, since their wider curve
/// otherwise crowds the close button.
#[cfg(target_os = "macos")]
const TRAFFIC_LIGHTS_ROUND_PAD: f32 = 4.;
/// The bar's bottom border, which sits inside its height and so out of the centring.
#[cfg(target_os = "macos")]
const TITLE_BAR_BORDER: f32 = 1.;
/// Where hidden traffic lights go: past the window's left edge, where nothing draws or hits them.
#[cfg(target_os = "macos")]
const TRAFFIC_LIGHTS_HIDDEN: f32 = -200.;
/// The width of the three traffic lights and the gap the title bar leaves after them.
#[cfg(target_os = "macos")]
const TRAFFIC_LIGHTS_SPAN: f32 = 65.;

#[derive(Clone, PartialEq)]
pub(crate) struct TitleBarOptions {
    pub navigation: bool,
    pub sidebar_open: bool,
    pub sidebar_right: Option<bool>,
    /// Whether the bar holds anything in its middle, which makes it taller.
    pub tall: bool,
    pub border: bool,
    /// Lets the fullscreen ambient background show through behind the controls.
    pub transparent: bool,
}

impl Default for TitleBarOptions {
    fn default() -> Self {
        Self {
            navigation: false,
            sidebar_open: false,
            sidebar_right: None,
            tall: false,
            border: true,
            transparent: false,
        }
    }
}

impl TitleBarOptions {
    /// How tall the bar is drawn: taller once its middle holds something.
    pub fn height(&self, window: &Window, cx: &App) -> Pixels {
        let metrics = cx.theme().metrics;
        ui::snapped(
            match self.tall {
                true => metrics.title_bar_tall,
                false => metrics.title_bar,
            },
            window,
        )
    }
}

/// Whether the user placed any navigation or control group in the centered toolbar region.
pub(crate) fn crowded(cx: &App) -> bool {
    NavEntry::ALL
        .into_iter()
        .any(|entry| super::place(entry, cx) == Place::Top)
}

pub(crate) enum TitleBarEvent {
    ToggleSidebar,
    ToggleSidebarRight,
    /// The search field was pressed, so the search screen should open.
    Search,
    /// Down was pressed in the search field, so the cursor should move into the results.
    EnterResults,
}

pub(crate) struct TitleBar {
    navigation: Entity<Navigation>,
    popovers: Popovers,
    search: Entity<Input>,
    settings: Entity<AppSettings>,
    options: TitleBarOptions,
    grabbed: bool,
    /// Set by a refresh and dropped once the page has its data or the user moves on, so the
    /// refresh button turns for exactly that long.
    busy: Option<Busy>,
    /// The refresh icon's turn, which springs back to rest once `busy` is gone.
    spin: Spin,
    /// Search’s horizontal position, measured before the containing group is balanced.
    search_center: Rc<Cell<Pixels>>,
    center_bias: Pixels,
    left_width: Pixels,
    right_width: Pixels,
    short_search: bool,
}

impl EventEmitter<TitleBarEvent> for TitleBar {}

impl TitleBar {
    /// Builds the bar around `search`, the field the search screen reads its query from.
    pub fn new(search: Entity<Input>, cx: &mut Context<Self>) -> Self {
        let navigation = router::trail(cx);
        let settings = Sonora::global(cx).settings.clone();

        cx.observe(&navigation, |this, _, cx| {
            this.busy = None;
            this.popovers.close();
            cx.notify();
        })
        .detach();
        cx.observe(&settings, |_, _, cx| cx.notify()).detach();
        let chrome = super::Chrome::entity(cx);
        cx.observe(&chrome, |_, _, cx| cx.notify()).detach();
        // Your Library only joins the bar once there is a library to show.
        cx.observe(&Sonora::global(cx).session.clone(), |_, _, cx| cx.notify())
            .detach();
        cx.observe(&Sonora::global(cx).library.clone(), |_, _, cx| cx.notify())
            .detach();
        // The bar is a cached view, and the field's own notify does not reach it there, so the
        // bar redraws itself whenever the field changes or the typing would show only later.
        cx.observe(&search, |_, _, cx| cx.notify()).detach();
        Self {
            navigation,
            popovers: Popovers::default(),
            search,
            settings,
            options: TitleBarOptions::default(),
            grabbed: false,
            busy: None,
            spin: Spin::default(),
            search_center: Rc::new(Cell::new(Pixels::ZERO)),
            center_bias: Pixels::ZERO,
            left_width: Pixels::ZERO,
            right_width: Pixels::ZERO,
            short_search: false,
        }
    }

    /// Turns the refresh button until `busy` says the reloaded page has its data.
    pub fn refreshing(&mut self, busy: Busy, cx: &mut Context<Self>) {
        self.busy = Some(busy);
        cx.notify();
    }

    pub fn set_options(&mut self, options: TitleBarOptions, cx: &mut Context<Self>) {
        if self.options == options {
            return;
        }
        self.options = options;
        cx.notify();
    }

    fn history(&mut self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.busy.as_ref().is_some_and(|busy| !busy(cx)) {
            self.busy = None;
        }
        match self.busy.is_some() {
            true => self.spin.start(),
            false => self.spin.stop(),
        }
        let turn = self.spin.turn(window, cx);
        let hover = cx.theme().sidebar_accent;
        let navigation = self.navigation.read(cx);
        let (can_back, can_forward) = (navigation.can_go_back(), navigation.can_go_forward());
        let refreshes = navigation.current().refreshes();
        let back = self.navigation.clone();
        let forward = self.navigation.clone();

        div()
            .id("titlebar-history")
            .role(Role::Toolbar)
            .aria_label(i18n::lookup("nav-history", None))
            .flex()
            .flex_none()
            .items_center()
            .gap_1()
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                Button::new("history-back")
                    .ghost()
                    .icon("icons/chevron-left.svg")
                    .tooltip("nav-back")
                    .disabled(!can_back)
                    .size_8()
                    .px_0()
                    .when(can_back, |button| {
                        button
                            .hover(move |style| style.bg(hover))
                            .active(move |style| style.bg(hover))
                    })
                    .on_click(move |_, _, cx| {
                        back.update(cx, |navigation, cx| navigation.back(cx))
                    }),
            )
            .child(
                Button::new("history-forward")
                    .ghost()
                    .icon("icons/chevron-right.svg")
                    .tooltip("nav-forward")
                    .disabled(!can_forward)
                    .size_8()
                    .px_0()
                    .when(can_forward, |button| {
                        button
                            .hover(move |style| style.bg(hover))
                            .active(move |style| style.bg(hover))
                    })
                    .on_click(move |_, _, cx| {
                        forward.update(cx, |navigation, cx| navigation.forward(cx))
                    }),
            )
            .when(refreshes, |history| {
                history.child(
                    Button::new("history-refresh")
                        .ghost()
                        .icon("icons/refresh-cw.svg")
                        .turned(turn)
                        .tooltip("app-refresh")
                        .size_8()
                        .px_0()
                        .hover(move |style| style.bg(hover))
                        .active(move |style| style.bg(hover))
                        .on_click(|_, window, cx| window.dispatch_action(Box::new(Refresh), cx)),
                )
            })
    }

    /// The search field, centred in the bar. Pressing it opens the search screen, and Down
    /// moves from it into the results.
    fn search(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let width = cx.theme().metrics.field * SEARCH_SPAN;
        let center = self.search_center.clone();
        let measured = cx.weak_entity();
        let threshold = width - cx.theme().metrics.field * 2.;

        div()
            .flex()
            .min_w_0()
            .w(width)
            .items_center()
            .occlude()
            .key_context(SEARCH_CONTEXT)
            .on_action(cx.listener(|_, _: &SelectNext, _, cx| cx.emit(TitleBarEvent::EnterResults)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    if this.navigation.read(cx).current() != Destination::Search {
                        cx.emit(TitleBarEvent::Search);
                    }
                }),
            )
            .on_children_prepainted(move |bounds, _, cx| {
                if let Some(bounds) = bounds.first() {
                    center.set(bounds.center().x);
                    let short = bounds.size.width < threshold;
                    measured
                        .update(cx, |this, cx| {
                            if this.short_search != short {
                                this.short_search = short;
                                this.search.update(cx, |input, cx| {
                                    input.set_hint(
                                        match short {
                                            true => "common-search",
                                            false => "search-placeholder",
                                        },
                                        cx,
                                    )
                                });
                            }
                        })
                        .ok();
                }
            })
            .child(self.search.clone())
    }

    /// Renders one saved toolbar region, keeping page-specific controls together as one item.
    fn group(
        &mut self,
        place: Place,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let sonora = Sonora::global(cx);
        let stocked = sonora.session.read(cx).authenticated()
            || sonora.library.read(cx).stocked(Shelf::Streaming);
        let current = self.navigation.read(cx).current();
        super::ordered_navigation(cx)
            .into_iter()
            .filter(|entry| super::place(*entry, cx) == place)
            .filter(|entry| *entry != NavEntry::Library || stocked)
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|entry| match entry {
                NavEntry::Transport => Some(self.history(window, cx).into_any_element()),
                NavEntry::Search => Some(self.search(cx).into_any_element()),
                NavEntry::Library | NavEntry::Local => {
                    let key = match entry {
                        NavEntry::Library => "title-library",
                        _ => "title-local",
                    };
                    let destination = |tab| match entry {
                        NavEntry::Library => Destination::Library(tab),
                        _ => Destination::Local(tab),
                    };
                    let active = destination(router::LibraryTab::Songs).same_section(&current);
                    let items =
                        super::LIBRARY_TABS
                            .iter()
                            .enumerate()
                            .map(|(index, (label, tab))| {
                                let to = destination(*tab);
                                MenuItem::new((key, index), i18n::lookup(label, None))
                                    .selected(current == to)
                                    .on_click(move |_, _, cx| navigate(to.clone(), cx))
                            });
                    Some(
                        Popover::new(key, self.popovers.clone())
                            .button(section(entry, active, cx))
                            .selected(active)
                            .menu(
                                Menu::new(key)
                                    .w(Picker::REGULAR)
                                    .top(cx.theme().metrics.field + cx.theme().metrics.pad)
                                    .when_else(
                                        place == Place::TopLeft,
                                        |menu| menu.left_0(),
                                        |menu| menu.right_0(),
                                    )
                                    .items(items),
                            )
                            .commands()
                            .into_any_element(),
                    )
                }
                NavEntry::Settings => Some(
                    section(
                        entry,
                        current.same_section(&Destination::Settings(router::SettingsPage::General)),
                        cx,
                    )
                    .on_click(|_, _, cx| open_settings(cx))
                    .into_any_element(),
                ),
                _ => entry.destination().map(|destination| {
                    let active = destination.same_section(&current);
                    section(entry, active, cx)
                        .on_click(move |_, _, cx| navigate(destination.clone(), cx))
                        .into_any_element()
                }),
            })
            .collect()
    }

    /// Measures a side group's intrinsic width, clearing its reservation when the group empties.
    fn measure_side(
        &self,
        side: Place,
        cx: &Context<Self>,
    ) -> impl Fn(Vec<Bounds<Pixels>>, &mut Window, &mut App) + 'static {
        let measured = cx.weak_entity();
        let padding = cx.theme().metrics.pad * 2.;
        move |bounds, _, cx| {
            let width = bounds
                .first()
                .zip(bounds.last())
                .map_or(Pixels::ZERO, |(first, last)| {
                    last.right() - first.left() + padding
                });
            measured
                .update(cx, |this, cx| {
                    let held = match side {
                        Place::TopLeft => &mut this.left_width,
                        _ => &mut this.right_width,
                    };
                    if (*held - width).abs() >= gpui::px(0.5) {
                        *held = width;
                        cx.notify();
                    }
                })
                .ok();
        }
    }

    /// A sidebar toggle that keeps its pointer events out of the window's drag area.
    fn toggle(&self, right: bool, open: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let (id, opened, closed, tooltip) = match right {
            true => (
                "sidebar-right-toggle",
                "icons/panel-right-close.svg",
                "icons/panel-right-open.svg",
                "nav-sidebar-right",
            ),
            false => (
                "sidebar-toggle",
                "icons/panel-left-close.svg",
                "icons/panel-left-open.svg",
                "nav-sidebar",
            ),
        };
        let icon = match open {
            true => opened,
            false => closed,
        };
        div()
            .flex()
            .flex_none()
            .items_center()
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                Button::new(id)
                    .ghost()
                    .small()
                    .icon(icon)
                    .tooltip(tooltip)
                    .on_click(cx.listener(move |_, _, _, cx| {
                        cx.emit(match right {
                            true => TitleBarEvent::ToggleSidebarRight,
                            false => TitleBarEvent::ToggleSidebar,
                        })
                    })),
            )
    }
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.theme();
        let height = self.options.height(window, cx);
        let navigation = self.options.navigation;
        let window_fullscreen = window.is_fullscreen();
        let settings = self.settings.read(cx);
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        let controls =
            matches!(window.window_decorations(), Decorations::Client { .. }) && !window_fullscreen;
        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
        let controls = settings.window_controls() && !window_fullscreen;
        let inset = traffic_lights_inset(settings, height);
        let decorated = cfg!(not(target_os = "macos")) && controls;
        let leading = decorated && settings.controls_on_left();
        #[cfg(not(target_os = "macos"))]
        let traffic_light = settings.traffic_light_controls();
        #[cfg(target_os = "macos")]
        let traffic_light = false;
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        let radius = crate::chrome::window_radius(settings, cx, window);
        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
        let radius: Option<Pixels> = None;

        let (left, center, right) = match navigation {
            true => (
                self.group(Place::TopLeft, window, cx),
                self.group(Place::Top, window, cx),
                self.group(Place::TopRight, window, cx),
            ),
            false => (Vec::new(), Vec::new(), Vec::new()),
        };

        let search_centered = navigation && super::place(NavEntry::Search, cx) == Place::Top;
        let sidebar = match navigation {
            true => super::Chrome::get(cx).sidebar_left,
            false => Pixels::ZERO,
        };
        let balance = sidebar + self.center_bias * 2.;
        let search_center = self.search_center.clone();
        let measured = cx.weak_entity();

        div()
            .id("title-bar")
            .role(Role::Banner)
            .flex()
            .items_center()
            .w_full()
            .h(height)
            .flex_none()
            .when_some(radius, |this, radius| this.rounded_t(radius))
            .when(!self.options.transparent && !theme.transparent, |this| {
                this.bg(theme.background)
            })
            // The height is a border box, so a bar that drops its border centres its controls
            // one pixel lower than one that keeps it. The edge is always an edge; only its
            // colour goes, and fullscreen's controls land where the workspace's do.
            .border_b_1()
            .border_color(match self.options.border {
                true => theme.title_bar_border,
                false => gpui::transparent_black(),
            })
            .window_control_area(gpui::WindowControlArea::Drag)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(
                    |this, event: &MouseDownEvent, window, _| match event.click_count {
                        1 => this.grabbed = true,
                        2 if !SYSTEM_ZOOMS => titlebar_double_click(window),
                        _ => {}
                    },
                ),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, _| this.grabbed = false),
            )
            .on_mouse_down_out(cx.listener(|this, _: &MouseDownEvent, _, _| this.grabbed = false))
            .on_mouse_move(cx.listener(|this, _: &MouseMoveEvent, window, _| {
                if this.grabbed {
                    this.grabbed = false;
                    window.start_window_move();
                }
            }))
            .on_mouse_down(MouseButton::Right, move |event, window, cx| {
                if navigation {
                    window.prevent_default();
                    cx.stop_propagation();
                    super::LayoutEditor::open(event.position, cx);
                }
            })
            .child(
                div()
                    .flex()
                    .flex_1()
                    .flex_basis((-balance).max(Pixels::ZERO))
                    .min_w(self.left_width)
                    .on_children_prepainted(self.measure_side(Place::TopLeft, cx))
                    .h_full()
                    .items_center()
                    .gap_1()
                    .map(|this| match (leading, inset) {
                        (true, _) => this.pl_2(),
                        (false, Some(inset)) => this.pl(inset),
                        (false, None) => this.pl_3(),
                    })
                    .pr_3()
                    .when(leading, |this| {
                        this.child(window_controls(true, traffic_light))
                    })
                    .when(navigation && !self.options.sidebar_open, |this| {
                        this.child(self.toggle(false, false, cx))
                    })
                    .children(left),
            )
            .child(
                div()
                    .flex()
                    .min_w_0()
                    .items_center()
                    .overflow_hidden()
                    .gap_2()
                    .on_children_prepainted(move |bounds, window, cx| {
                        if navigation
                            && let (Some(first), Some(last)) = (bounds.first(), bounds.last())
                        {
                            let anchor = match search_centered {
                                true => search_center.get(),
                                false => (first.left() + last.right()) / 2.,
                            };
                            let error = anchor - window.viewport_size().width / 2.;
                            let space = (window.viewport_size().width
                                - sidebar
                                - (last.right() - first.left()))
                            .max(Pixels::ZERO);
                            let next = ((balance + error * 2.).clamp(-space, space) - sidebar) / 2.;
                            measured
                                .update(cx, |this, cx| {
                                    if (this.center_bias - next).abs() >= gpui::px(0.5) {
                                        this.center_bias = next;
                                        cx.notify();
                                    }
                                })
                                .ok();
                        }
                    })
                    .children(center),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .flex_basis(balance.max(Pixels::ZERO))
                    .min_w(self.right_width)
                    .on_children_prepainted(self.measure_side(Place::TopRight, cx))
                    .h_full()
                    .items_center()
                    .justify_end()
                    .gap_1()
                    .px_3()
                    .children(right)
                    .when(navigation, |this| {
                        this.when_some(
                            self.options
                                .sidebar_right
                                .filter(|open| !open && SidebarRight::available(window)),
                            |this, open| this.child(self.toggle(true, open, cx)),
                        )
                    })
                    .when(decorated && !leading, |this| {
                        this.child(
                            div()
                                .flex_none()
                                .when(cfg!(target_os = "windows") && !traffic_light, |this| {
                                    this.h_full().self_stretch()
                                })
                                .child(window_controls(false, traffic_light)),
                        )
                    }),
            )
    }
}

/// Where AppKit's traffic lights sit for the current settings and a title bar of `height`,
/// measured from the window's top-left corner. They are centred in the bar and kept as far from
/// the left edge as from the top, which centres them in the corner, with a little more room for
/// round corners. Hidden controls are moved off the window, since AppKit rebuilds them.
#[cfg(target_os = "macos")]
pub fn traffic_lights(settings: &AppSettings, height: Pixels) -> gpui::Point<Pixels> {
    let left = match settings.window_controls() {
        true => traffic_lights_left(settings, height),
        false => gpui::px(TRAFFIC_LIGHTS_HIDDEN),
    };
    gpui::point(left, traffic_lights_offset(height))
}

#[cfg(target_os = "macos")]
fn traffic_lights_left(settings: &AppSettings, height: Pixels) -> Pixels {
    let pad = match settings.window_rounding() {
        ui::Rounding::Round => gpui::px(TRAFFIC_LIGHTS_ROUND_PAD),
        _ => Pixels::ZERO,
    };
    traffic_lights_offset(height) + pad
}

#[cfg(target_os = "macos")]
fn traffic_lights_offset(height: Pixels) -> Pixels {
    let room = height - gpui::px(TITLE_BAR_BORDER + TRAFFIC_LIGHTS_SIZE);
    room / 2.
}

/// How far past the traffic lights the title bar's content starts while macOS shows them, or
/// `None` when it takes the same padding as the right edge.
fn traffic_lights_inset(settings: &AppSettings, height: Pixels) -> Option<Pixels> {
    #[cfg(target_os = "macos")]
    if settings.window_controls() {
        return Some(traffic_lights_left(settings, height) + gpui::px(TRAFFIC_LIGHTS_SPAN));
    }
    #[cfg(not(target_os = "macos"))]
    let _ = (settings, height);
    None
}

/// One section in the middle of the bar, with a foreground tint while its page is open.
fn section(entry: NavEntry, active: bool, cx: &App) -> Button {
    let theme = cx.theme();
    let size = theme.metrics.field;

    Button::new(("title-bar-section", entry as usize))
        .ghost()
        .icon(entry.icon())
        .icon_size(theme.text(Text::Large))
        .tooltip(entry.key())
        .tint(match active {
            true => theme.foreground,
            false => theme.muted_foreground,
        })
        .size(size)
        .px_0()
        .rounded_full()
        .occlude()
}

fn window_controls(leading: bool, traffic_light: bool) -> AnyElement {
    match traffic_light {
        true => TrafficLightControls::new(leading).into_any_element(),
        false => WindowControls::new(leading).into_any_element(),
    }
}

/// Runs the title bar's double-click action. macOS performs the action set in System Settings;
/// the window zooms everywhere else, since there is no such preference.
fn titlebar_double_click(window: &Window) {
    #[cfg(target_os = "macos")]
    window.titlebar_double_click();
    #[cfg(not(target_os = "macos"))]
    window.zoom_window();
}
