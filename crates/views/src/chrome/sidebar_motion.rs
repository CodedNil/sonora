use std::time::Instant;

use gpui::{Context, Pixels, SpringConfig, SpringState, Window, px};

/// How far past its minimum the edge has to be pulled before the sidebar folds down to its rail,
/// and how far past the rail before it unfolds again.
pub(super) const POP: Pixels = px(72.);
/// The most the edge ever gives past a limit, however hard it is pulled.
pub(super) const GIVE: Pixels = px(18.);
/// How stiffly the edge resists a pull past a limit: lower gives less for the same travel.
pub(super) const RESISTANCE: f32 = 0.6;
/// The speed, in pixels a second, the edge is nudged with as it first meets a limit, a small
/// bump that says the limit gives rather than stops.
pub(super) const KNOCK: f32 = 110.;
/// The speed the sidebar is sent off with as it folds or unfolds, so it lands with a snap.
pub(super) const FLING: f32 = 150.;
/// The longest step the spring takes in one frame, so a stalled frame never jumps it.
const BEAT: f32 = 1. / 30.;
/// How close to its target, in pixels, the give has to be before it counts as settled.
const SETTLED: f32 = 0.25;

/// The spring the edge rides while it is held past a limit and as it is let go: firm, so it
/// follows the pointer closely and settles with barely an overshoot.
pub(super) const HELD: SpringConfig = SpringConfig::new(600., 30., 1.);
/// The one it rides through a fold, damped nearly to rest so the new width lands cleanly.
pub(super) const POP_SPRING: SpringConfig = SpringConfig::new(600., 38., 1.);

/// Advances a shared sidebar spring, preserving its injected drag velocity and notifying its owner.
pub(super) fn settle<T: 'static>(
    state: &mut SpringState,
    ticked: &mut Instant,
    target: Pixels,
    spring: SpringConfig,
    window: &mut Window,
    cx: &mut Context<T>,
) -> Pixels {
    let now = Instant::now();
    let beat = now.duration_since(*ticked).as_secs_f32().min(BEAT);
    *ticked = now;
    let target = target.as_f32();
    let moving = !cx.reduce_motion() && !spring.is_settled(*state, target, SETTLED);
    *state = match moving {
        true => spring.step(*state, target, beat),
        false => SpringState {
            position: target,
            velocity: 0.,
        },
    };
    if moving {
        cx.on_next_frame(window, |_, _, cx| cx.notify());
    }
    px(state.position)
}

/// How far the edge gives for a pull `over` past a limit: nearly all of it at first, less and
/// less after, and never more than `GIVE`.
pub(super) fn rubber(over: Pixels) -> Pixels {
    let travel = over / GIVE * RESISTANCE;
    GIVE * (1. - 1. / (travel + 1.))
}
