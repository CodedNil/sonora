use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;

use gpui::prelude::*;
use gpui::{
    App, Bounds, Div, Pixels, Point, SharedString, SpringState, StyleRefinement, Svg, Window, div,
    px, svg,
};

use crate::artwork::{Artwork, Avatar};
use crate::metrics::Text;
use crate::motion::Springs;
use crate::theme::ActiveTheme as _;

const CHIP: Pixels = px(240.);
const NUDGE: Pixels = px(8.);
const ART: Pixels = px(20.);
const MARKER: Pixels = px(2.);
const SLACK: Pixels = px(10.);

/// The maximum spring step and settling distance shared by reorder previews.
const REORDER_BEAT: f32 = 1. / 30.;
const REORDER_SETTLED: f32 = 0.25;

/// Shared spring positions for a reorderable list or table. Keys identify items across drops,
/// so committing an order does not restart the animation. Clones share the same positions.
#[derive(Clone)]
pub struct ReorderMotion(Rc<RefCell<ReorderState>>);

/// Positions and clock retained between frames of a reorder preview.
struct ReorderState {
    positions: HashMap<SharedString, SpringState>,
    ticked: Instant,
}

impl Default for ReorderMotion {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(ReorderState {
            positions: HashMap::new(),
            ticked: Instant::now(),
        })))
    }
}

impl ReorderMotion {
    /// Steps keyed targets once per frame. A held item follows its target immediately;
    /// neighbours use the same spring and reduced-motion behaviour everywhere.
    pub fn animate(
        &self,
        targets: impl IntoIterator<Item = (SharedString, Pixels, bool)>,
        window: &Window,
        cx: &App,
    ) {
        let mut state = self.0.borrow_mut();
        let now = Instant::now();
        let beat = now
            .duration_since(state.ticked)
            .as_secs_f32()
            .min(REORDER_BEAT);
        state.ticked = now;
        let mut live = HashSet::new();
        let mut animating = false;
        for (key, target, held) in targets {
            live.insert(key.clone());
            let target = target.as_f32();
            let position = state.positions.entry(key).or_insert(SpringState {
                position: target,
                velocity: 0.,
            });
            let animate = !held
                && !cx.reduce_motion()
                && !Springs::RESPONSIVE.is_settled(*position, target, REORDER_SETTLED);
            *position = match animate {
                true => Springs::RESPONSIVE.step(*position, target, beat),
                false => SpringState {
                    position: target,
                    velocity: 0.,
                },
            };
            animating |= animate;
        }
        state.positions.retain(|key, _| live.contains(key));
        if animating {
            window.request_animation_frame();
        }
    }

    /// The animated absolute position of an item, or its normal position before its first frame.
    pub fn position(&self, key: &str, normal: Pixels) -> Pixels {
        self.0
            .borrow()
            .positions
            .get(key)
            .map_or(normal, |position| px(position.position))
    }
}

/// Where an item previews when an existing item is moved into a gap. Listeners keep their
/// original slots while only the row contents move, so hit testing cannot oscillate.
pub fn reorder_index(index: usize, from: Option<usize>, gap: Option<usize>) -> usize {
    let (Some(from), Some(gap)) = (from, gap) else {
        return index;
    };
    let to = gap.saturating_sub(usize::from(gap > from));
    match index {
        index if index == from => to,
        index if from < index && index <= to => index - 1,
        index if to <= index && index < from => index + 1,
        _ => index,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Edge {
    Above,
    Below,
}

pub fn drop_gap(bounds: Bounds<Pixels>, position: Point<Pixels>, index: usize) -> Option<usize> {
    let slack = SLACK.min(bounds.size.height / 2.);
    if !bounds.dilate(slack).contains(&position) {
        return None;
    }

    Some(match position.y < bounds.center().y {
        true => index,
        false => index + 1,
    })
}

pub fn drop_marker(edge: Edge, cx: &App) -> Div {
    let line = div()
        .absolute()
        .left_2()
        .right_2()
        .h(MARKER)
        .rounded_full()
        .bg(cx.theme().primary);

    match edge {
        Edge::Above => line.top_0(),
        Edge::Below => line.bottom_0(),
    }
}

/// The dotted handle at the start of a row that can be dragged to a new place. It only marks the
/// row, so the caller wires the drag itself.
pub fn grip(cx: &App) -> Svg {
    let theme = cx.theme();

    svg()
        .path(icons::path("icons/grip-vertical.svg"))
        .flex_none()
        .size(theme.text(Text::Body))
        .text_color(theme.muted_foreground)
}

#[derive(IntoElement)]
pub struct Ghost {
    base: Div,
    position: Point<Pixels>,
    label: SharedString,
    cover: Option<String>,
    fallback: Option<SharedString>,
    art: bool,
    round: bool,
}

impl Ghost {
    pub fn new(position: Point<Pixels>, label: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            position,
            label: label.into(),
            cover: None,
            fallback: None,
            art: false,
            round: false,
        }
    }

    pub fn art(mut self, cover: Option<String>) -> Self {
        self.cover = cover;
        self.art = true;
        self
    }

    pub fn fallback(mut self, icon: impl Into<SharedString>) -> Self {
        self.fallback = Some(icon.into());
        self
    }

    pub fn circle(mut self) -> Self {
        self.round = true;
        self
    }
}

impl Styled for Ghost {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Ghost {
    fn render(mut self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = *cx.theme();
        let overrides = std::mem::take(self.base.style());
        let leading = self.art.then(|| match self.round {
            true => Avatar::new(self.cover.clone()).size(ART).into_any_element(),
            false => Artwork::new(self.cover.clone())
                .size(ART)
                .corner_radius(theme.radius)
                .when_some(self.fallback.clone(), Artwork::fallback)
                .into_any_element(),
        });

        let mut chip = self
            .base
            .flex()
            .items_center()
            .gap_2()
            .max_w(CHIP)
            .px_2()
            .py_1()
            .rounded(theme.radius)
            .bg(theme.secondary)
            .text_color(theme.foreground)
            .children(leading)
            .child(div().min_w_0().truncate().child(self.label));
        chip.style().refine(&overrides);

        div()
            .pl(self.position.x + NUDGE)
            .pt(self.position.y + NUDGE)
            .child(chip)
    }
}
