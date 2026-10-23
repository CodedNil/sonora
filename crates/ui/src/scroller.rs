use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, DispatchPhase, Div, ElementId, Entity, EntityId, Interactivity, MouseButton,
    MouseMoveEvent, Pixels, ScrollHandle, ScrollWheelEvent, StyleRefinement, Window, canvas, div,
    px,
};

use crate::button::Button;
use crate::glass::{blurring, glass};
use crate::scrollbar::{Scrollbar, activate_middle_scroll, cancel_middle_scroll};
use crate::theme::ActiveTheme as _;

/// How far a region has to be scrolled before the trip back is worth a button, in rows.
const REACH: f32 = 3.;
/// How far a perched control floats off the bottom of its region.
const PERCH: Pixels = px(12.);

/// Measured starts shared by a scroller and its sections or virtualized rows. Mark a container,
/// not its text, so settling preserves the section's own internal padding.
#[derive(Clone)]
pub struct ScrollAnchors {
    scroll: ScrollHandle,
    points: Rc<RefCell<HashMap<ElementId, Pixels>>>,
    inset: Rc<Cell<Pixels>>,
    marker: Rc<Cell<Option<Pixels>>>,
    marker_owner: Rc<Cell<Option<EntityId>>>,
    table: Rc<Cell<bool>>,
}

impl ScrollAnchors {
    pub fn new(scroll: ScrollHandle) -> Self {
        Self {
            scroll,
            points: Rc::default(),
            inset: Rc::default(),
            marker: Rc::default(),
            marker_owner: Rc::default(),
            table: Rc::default(),
        }
    }

    /// Keeps the scrollbar faintly visible while it belongs to an overflowing table.
    pub fn set_table(&self, table: bool) {
        self.table.set(table);
    }

    pub(crate) fn has_table(&self) -> bool {
        self.table.get()
    }

    /// Sets a highlighted content position for the scrollbar, independent of magnetic targets.
    pub fn set_marker(&self, top: Option<Pixels>, cx: &mut App) -> bool {
        if self.marker.replace(top) == top {
            return false;
        }
        if let Some(owner) = self.marker_owner.get() {
            cx.notify(owner);
        }
        true
    }

    pub(crate) fn watch_marker(&self, owner: EntityId) {
        self.marker_owner.set(Some(owner));
    }

    pub(crate) fn marker(&self) -> Option<Pixels> {
        self.marker.get()
    }

    /// Converts a viewport position to the unscrolled content coordinate used by scrollbar marks.
    pub(crate) fn content_position(&self, top: Pixels) -> Pixels {
        top - self.scroll.bounds().top() - self.scroll.offset().y
    }

    /// Leaves room for a persistent page header above each magnetic target.
    pub fn set_inset(&self, inset: Pixels) {
        self.inset.set(inset);
    }

    /// Measures a section without changing its content or layout.
    pub fn mark(&self, id: impl Into<ElementId>, content: impl IntoElement) -> Div {
        let id = id.into();
        let anchors = self.clone();
        div()
            .min_w_0()
            .flex_none()
            .child(content)
            .on_children_prepainted(move |bounds, _, _| {
                if let Some(bounds) = bounds.first() {
                    anchors.record(id.clone(), bounds.top(), Pixels::ZERO);
                }
            })
    }

    /// Records an unscrolled position relative to the viewport, with room for a sticky header.
    pub fn record(&self, id: impl Into<ElementId>, top: Pixels, inset: Pixels) {
        let at =
            top - self.scroll.bounds().top() - self.scroll.offset().y - inset - self.inset.get();
        self.points
            .borrow_mut()
            .insert(id.into(), at.max(Pixels::ZERO));
    }

    /// Chooses a nearby, reachable section ahead in the scroll direction, never one left behind.
    pub(crate) fn nearest(
        &self,
        at: Pixels,
        reach: Pixels,
        maximum: Pixels,
        direction: f32,
    ) -> Option<Pixels> {
        self.points
            .borrow()
            .values()
            .copied()
            .filter(|point| {
                *point <= maximum
                    && (*point - at).abs() <= reach
                    && (*point - at).as_f32() * direction >= 0.
            })
            .min_by(|a, b| {
                (a.as_f32() - at.as_f32())
                    .abs()
                    .total_cmp(&(b.as_f32() - at.as_f32()).abs())
                    .then_with(|| a.as_f32().total_cmp(&b.as_f32()))
            })
    }
}

#[derive(IntoElement)]
pub struct Scroller {
    base: Div,
    id: ElementId,
    bar: Entity<Scrollbar>,
    children: Vec<AnyElement>,
    present_surface: bool,
    owns_scroll: bool,
    bottom_inset: Pixels,
    sectioned: bool,
}

impl Scroller {
    #[track_caller]
    pub fn new(id: impl Into<ElementId>, bar: &Entity<Scrollbar>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            bar: bar.clone(),
            children: Vec::new(),
            present_surface: true,
            owns_scroll: true,
            bottom_inset: Pixels::ZERO,
            sectioned: true,
        }
    }

    /// A region whose child scrolls itself, a `uniform_list` above all. The surface still
    /// carries the wheel, the middle button and the bar, and leaves the offset to the handle
    /// the child tracks.
    #[track_caller]
    pub fn listing(id: impl Into<ElementId>, bar: &Entity<Scrollbar>) -> Self {
        Self {
            owns_scroll: false,
            ..Self::new(id, bar)
        }
    }

    /// Keeps continuous text, editing controls and self-following content free-scrolling.
    pub fn free_scroll(mut self) -> Self {
        self.sectioned = false;
        self
    }

    /// Lets a caller merge the scroll presentation into child transforms, avoiding nested
    /// compositor sampling when those children already have their own spring motion.
    pub fn manual_presentation(mut self) -> Self {
        self.present_surface = false;
        self
    }

    /// Adds room after the scrollable content so its final item can rise above an overlaid
    /// control. This is added to caller padding, rather than replacing it. A self-scrolling
    /// listing keeps only its scrollbar clear; its list must provide its own trailing space.
    pub fn bottom_inset(mut self, inset: Pixels) -> Self {
        self.bottom_inset = inset.max(Pixels::ZERO);
        self
    }
}

impl Styled for Scroller {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Scroller {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl ParentElement for Scroller {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Scroller {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            mut base,
            id,
            bar,
            children,
            present_surface,
            owns_scroll,
            bottom_inset,
            sectioned,
        } = self;
        bar.read(cx).anchors().watch_marker(bar.entity_id());
        let anchors = sectioned.then(|| bar.read(cx).anchors());
        if let Some(anchors) = &anchors {
            anchors.points.borrow_mut().clear();
        }
        let measuring = anchors.clone();

        bar.update(cx, |bar, cx| {
            bar.set_track_bottom(bottom_inset, cx);
            bar.set_anchors(anchors);
        });
        let scroll = bar.read(cx).scroll().clone();
        let overrides = std::mem::take(base.style());
        bar.read(cx).sync();
        let presentation = bar.read(cx).presentation();
        let gliding = bar.clone();

        let base = base.when_some(measuring, |surface, anchors| {
            surface.on_children_prepainted(move |bounds, _, _| {
                for (index, bounds) in bounds.iter().enumerate() {
                    anchors.record(("section", index), bounds.top(), Pixels::ZERO);
                }
            })
        });
        let mut surface = middle_scroll(base, &bar)
            .id(id)
            .size_full()
            .when(owns_scroll, |surface| {
                surface
                    .overflow_y_scroll()
                    .restrict_scroll_to_axis()
                    .track_scroll(&scroll)
            })
            .on_scroll_wheel(move |event: &ScrollWheelEvent, window, cx| {
                gliding.update(cx, |bar, cx| bar.wheel(event, window, cx));
            })
            .children(children);

        surface.style().refine(&overrides);
        if owns_scroll && bottom_inset > Pixels::ZERO {
            let padding = surface
                .style()
                .padding
                .bottom
                .unwrap_or_default()
                .to_pixels(Pixels::ZERO.into(), window.rem_size());
            surface = surface.pb(padding + bottom_inset);
        }
        if present_surface {
            surface = surface.layer_translate(presentation);
        }

        div()
            .relative()
            .size_full()
            .min_h_0()
            .child(surface)
            .child(bar)
    }
}

/// Lays a layer over a region nested inside a larger scroll, which takes the wheel before
/// anything scrollable in the region can and scrolls `bar`'s region with it, the way the wheel
/// does over the rest of that scroll. The region below still scrolls when its own code asks it to,
/// so a lyrics card can follow the song while the wheel moves the page it sits on. The parent
/// has to be `relative`.
pub fn wheel_to(bar: &Entity<Scrollbar>) -> impl IntoElement {
    let bar = bar.clone();
    canvas(
        |_, _, _| {},
        move |bounds, _, window, _| {
            let bar = bar.clone();
            window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
                if phase != DispatchPhase::Capture || !bounds.contains(&event.position) {
                    return;
                }
                cx.stop_propagation();
                let scroll = bar.read(cx).scroll().clone();
                let delta = event.delta.pixel_delta(window.line_height());
                let mut offset = scroll.offset();
                offset.y = (offset.y + delta.y).clamp(-scroll.max_offset().y, Pixels::ZERO);
                scroll.set_offset(offset);
                bar.update(cx, |bar, cx| bar.wheel(event, window, cx));
                window.refresh();
            });
        },
    )
    .absolute()
    .inset_0()
}

/// Adds browser-style middle-button auto-scrolling to a scrollable surface. A middle click
/// turns the mode on until the next press of any button. A middle press that is held and
/// dragged scrolls only while held, and `Root` ends it on the release. A middle press that a
/// child handles itself, such as a text field pasting on Linux, does not start it. A middle
/// press never moves focus or shows a pressed style on the children it passes through.
pub fn middle_scroll(surface: Div, bar: &Entity<Scrollbar>) -> Div {
    surface
        .capture_any_mouse_down(|event, window, cx| {
            if event.button == MouseButton::Middle {
                window.prevent_default();
            } else if cancel_middle_scroll(cx) {
                window.refresh();
                cx.stop_propagation();
            }
        })
        .on_mouse_down(MouseButton::Middle, {
            let gliding = bar.clone();
            move |event, window, cx| {
                let started = gliding.update(cx, |bar, cx| {
                    bar.middle_scroll_start(event.position, window, cx)
                });
                if started {
                    activate_middle_scroll(&gliding, cx);
                } else {
                    cancel_middle_scroll(cx);
                }
                window.refresh();
                cx.stop_propagation();
            }
        })
        .on_mouse_move({
            let gliding = bar.clone();
            move |event: &MouseMoveEvent, window, cx| {
                gliding.update(cx, |bar, cx| {
                    bar.middle_scroll_move(event.position, window, cx)
                });
            }
        })
}

/// The shape every control that floats over a scrolling region takes: a round bordered pill,
/// centred along the bottom. It swallows clicks meant for it rather than the rows behind, and
/// still lets the wheel through. The caller places it with `bottom_*`.
pub fn perched(button: Button, cx: &App) -> Div {
    let theme = *cx.theme();
    let button = match blurring(cx) {
        true => glass(button, cx),
        false => button.bg(theme.popover),
    };

    div()
        .absolute()
        .bottom(PERCH)
        .w_full()
        .flex()
        .justify_center()
        .child(
            div().flex().flex_none().block_mouse_except_scroll().child(
                button
                    .ghost()
                    .small()
                    .rounded_full()
                    .border_1()
                    .border_color(theme.border),
            ),
        )
}

/// The room a scrolling region has to keep under its last row for a perched control to float in
/// without covering it.
pub fn perch_room(cx: &App) -> Pixels {
    PERCH * 2. + cx.theme().metrics.control_small
}

/// A perched button that glides a scrolling region back to its top. It stays away until the
/// region has been scrolled far enough for the trip to be worth one. The parent has to be
/// `relative`.
pub fn return_top(id: impl Into<ElementId>, bar: &Entity<Scrollbar>, cx: &App) -> Option<Div> {
    return_to(id, bar, Pixels::ZERO, "nav-return-top", cx)
}

/// A perched button that glides a scrolling region back to a resting offset, in either
/// direction. It stays away until the region has drifted far enough from that spot for the trip
/// to be worth one. `goal` is how far down the region should sit, in the positive pixels
/// `Scrollbar::offset` reports, and is turned into gpui's negative offset before it is aimed at.
/// `tooltip` is an i18n key. The parent has to be `relative`.
pub fn return_to(
    id: impl Into<ElementId>,
    bar: &Entity<Scrollbar>,
    goal: Pixels,
    tooltip: &'static str,
    cx: &App,
) -> Option<Div> {
    let viewport = bar.read(cx).viewport();
    let reach = (cx.theme().metrics.list_row * REACH).min(viewport / 2.);
    if viewport <= Pixels::ZERO || (bar.read(cx).offset() - goal).abs() < reach {
        return None;
    }
    let bar = bar.clone();

    Some(perched(
        Button::new(id)
            .secondary()
            .icon("icons/undo-2.svg")
            .tooltip(tooltip)
            .on_click(move |_, window, cx| {
                bar.update(cx, |bar, _| bar.aim(-goal, window));
            }),
        cx,
    ))
}
