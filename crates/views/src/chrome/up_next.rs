//! The card that floats above the player bar naming the song after the one playing, with a bar
//! along its foot that fills over the last stretch of the song before it.

use std::time::Duration;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, Context, Entity, Pixels, Render, SharedString, Task, Window, div, px, relative,
};
use music::Track;
use state::{Playback, Queue, QueuePosition, Sonora};
use ui::{ActiveTheme as _, Card, Rising as _, Text, blurring};

/// How long before a song ends the card starts counting down to the next.
const LEAD: Duration = Duration::from_secs(20);
/// How often the card looks at the clock while a song plays outside the countdown.
const TICK: Duration = Duration::from_millis(500);
/// How hard the card blurs the page behind it and how much of the popover colour it keeps, so it
/// stands clear of whatever it floats over.
const BLUR: Pixels = px(32.);
const FILL: f32 = 0.6;
/// How thick the countdown bar along the card's foot is.
const BAR: Pixels = px(3.);

pub(crate) struct UpNext {
    playback: Entity<Playback>,
    _watch: Task<()>,
}

impl UpNext {
    pub(crate) fn new(
        playback: Entity<Playback>,
        queue: Entity<Queue>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&queue, |_, _, cx| cx.notify()).detach();
        cx.observe(&playback, |_, _, cx| cx.notify()).detach();
        let devices = Sonora::global(cx).devices.clone();
        cx.observe(&devices, |_, _, cx| cx.notify()).detach();
        // The clock moves without telling anyone, so the card looks at it now and then, and
        // every frame while it counts down.
        let watch = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(TICK).await;
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    return;
                }
            }
        });

        Self {
            playback,
            _watch: watch,
        }
    }

    /// Reserves the preview's row and one shared inset above the player bar. The sidebar's own
    /// bottom inset leaves the same gap between its island and the floating preview.
    pub(crate) fn clearance(&self, cx: &App) -> Pixels {
        match self.next(cx).is_some() {
            true => {
                let metrics = cx.theme().metrics;
                metrics.list_row + metrics.island_gap()
            }
            false => Pixels::ZERO,
        }
    }

    /// Renders the same next-track preview inside the expanded left sidebar.
    pub(crate) fn inline(&self, window: &mut Window, cx: &App) -> Option<AnyElement> {
        let (track, countdown) = self.next(cx)?;
        if countdown.is_some() {
            window.request_animation_frame();
        }
        let theme = *cx.theme();
        let card = self
            .card(track, true, cx)
            .flat()
            .list_art(theme.metrics.list_row)
            .hover(move |style| style.bg(theme.sidebar_accent));
        let bar = countdown.map(|run| {
            div()
                .absolute()
                .bottom_0()
                .left(theme.radius)
                .right(theme.radius)
                .h(BAR)
                .child(
                    div()
                        .h_full()
                        .w(relative(run))
                        .rounded_full()
                        .bg(theme.primary),
                )
        });

        Some(
            div()
                .id("up-next-sidebar")
                .relative()
                .flex_none()
                .flex()
                .flex_col()
                .gap_1()
                .bg(theme.sidebar)
                .pt(theme.metrics.pad * 1.5)
                .px(theme.metrics.pad)
                .pb(theme.metrics.pad)
                .child(ui::eyebrow(i18n::lookup("queue-up-next", None), cx).px_2())
                .child(card)
                .children(bar)
                .into_any_element(),
        )
    }

    /// The track after the one playing, and how much of the countdown to it has run, from 0 as
    /// it starts to 1 as the song ends. `None` for the countdown outside it or while paused.
    fn next(&self, cx: &App) -> Option<(Track, Option<f32>)> {
        let playback = self.playback.read(cx);
        let queue = playback.shown_queue(cx);
        let next = queue.track(QueuePosition::Upcoming(0), cx)?;
        let (position, duration) = playback.shown_time(cx);
        let left = duration.saturating_sub(position);
        let counting = queue.playing && !left.is_zero() && left <= LEAD;
        Some((
            next,
            counting.then(|| 1. - left.as_secs_f32() / LEAD.as_secs_f32()),
        ))
    }
}

impl Render for UpNext {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some((track, countdown)) = self.next(cx) else {
            return div();
        };
        if countdown.is_some() {
            window.request_animation_frame();
        }
        let theme = *cx.theme();
        let gap = theme.metrics.island_gap();
        let key = SharedString::from(format!(
            "up-next-{}",
            track.id.as_deref().unwrap_or_default()
        ));
        let card = self.card(track, false, cx);
        let strip = super::PlayerBar::strip(cx);
        // Clear both the player bar and its Jam strip, aligned with the sidebar.
        div()
            .absolute()
            .bottom(relative(1.))
            .left_0()
            .pl(gap)
            .pb(gap + strip)
            .child(
                div()
                    .id("up-next")
                    .relative()
                    .occlude()
                    .max_w((window.viewport_size().width - gap * 2.).max(Pixels::ZERO))
                    .rounded(theme.radius)
                    .shadow_lg()
                    .border_1()
                    .border_color(theme.border.opacity(1.))
                    .map(|this| match blurring(cx) {
                        true => this.bg(theme.popover.alpha(FILL)).backdrop_blur(BLUR),
                        false => this.bg(theme.popover),
                    })
                    .child(card)
                    // The renderer cannot crop to the rounded corners, so the bar runs along
                    // the straight stretch of the foot between them, as a rounded line.
                    .children(countdown.map(|run| {
                        div()
                            .absolute()
                            .bottom_0()
                            .left(theme.radius)
                            .right(theme.radius)
                            .h(BAR)
                            .child(
                                div()
                                    .h_full()
                                    .w(relative(run))
                                    .rounded_full()
                                    .bg(theme.primary),
                            )
                    }))
                    .rising(key),
            )
    }
}

impl UpNext {
    /// Builds the next-track card at the width of its floating or sidebar container.
    fn card(&self, track: Track, inline: bool, cx: &App) -> Card {
        let theme = cx.theme();
        let playback = self.playback.clone();
        Card::new("up-next-card", track.name.clone())
            .cover(track.cover.clone())
            .art_radius(match inline {
                true => theme.radius,
                false => (theme.radius - theme.metrics.pad).max(Pixels::ZERO),
            })
            .hint()
            .meta(track.artists.clone())
            .size(Text::Body)
            .when(inline, |card| card.w_full())
            .when(!inline, |card| card.w_auto())
            .max_w_full()
            .press(move |_, _, cx| playback.update(cx, |playback, cx| playback.next(cx)))
    }
}
