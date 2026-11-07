use std::rc::Rc;

use gpui::accesskit::Role;
use gpui::prelude::*;
use gpui::{AnyElement, App, Context, Entity, Render, SharedString, Window, div, relative};
use i18n::t;
use music::{Ranked, SavedArtist, StatsPeriod};
use state::{Playback, Sonora, Stats, StatsSource};
use ui::{
    ActiveTheme as _, Button, Card, Input, RangeScrubber, RangeState, Scrollbar, Scroller, TabBar,
    Text,
};

use crate::chrome::Chrome;
use crate::shared::album_grid::CardGrid;
use crate::shared::cards;
use crate::shared::track_card::{StartHandler, TrackCard};

const DAY: i64 = 86_400;
const PAGE: usize = 12;

/// Personal rankings with provider windows and an exact-date recorded-history view.
pub(crate) struct StatsView {
    stats: Entity<Stats>,
    playback: Entity<Playback>,
    scrollbar: Entity<Scrollbar>,
    range: RangeState,
    start: Entity<Input>,
    end: Entity<Input>,
    invalid: bool,
    shown: [usize; 5],
    preset: Option<StatsPeriod>,
}

impl StatsView {
    pub(crate) fn new(
        stats: Entity<Stats>,
        playback: Entity<Playback>,
        cx: &mut Context<Self>,
    ) -> Self {
        let chrome = Chrome::entity(cx);
        cx.observe(&chrome, |_, _, cx| cx.notify()).detach();
        cx.observe(&stats, |this, _, cx| {
            this.sync_dates(cx);
            cx.notify();
        })
        .detach();
        cx.observe(&playback, |_, _, cx| cx.notify()).detach();
        let session = Sonora::global(cx).session.clone();
        cx.observe(&session, |_, _, cx| cx.notify()).detach();
        let host = cx.entity_id();
        let scrollbar = cx.new(|_| Scrollbar::inset().watching(host));
        Self {
            stats,
            playback,
            scrollbar,
            range: RangeState::new("stats-date-range"),
            start: cx.new(|cx| Input::new("stats-date-hint", cx).name("stats-from")),
            end: cx.new(|cx| Input::new("stats-date-hint", cx).name("stats-to")),
            invalid: false,
            shown: [0; 5],
            preset: None,
        }
    }

    pub(crate) fn refresh(&mut self, cx: &mut Context<Self>) {
        self.stats.update(cx, |stats, cx| stats.refresh(cx));
    }

    /// Keeps text fields aligned with the current range after an asynchronous history read.
    fn sync_dates(&mut self, cx: &mut Context<Self>) {
        let stats = self.stats.read(cx);
        if stats.source() != StatsSource::Recorded {
            return;
        }
        let Some(dates) = stats.dates().or(stats.bounds()) else {
            return;
        };
        self.start
            .update(cx, |input, cx| input.set_text(date_label(dates.0), cx));
        self.end
            .update(cx, |input, cx| input.set_text(date_label(dates.1), cx));
    }

    fn select(&mut self, source: StatsSource, period: StatsPeriod, cx: &mut Context<Self>) {
        self.shown = [0; 5];
        self.stats
            .update(cx, |stats, cx| stats.select(source, period, cx));
        if source == StatsSource::Recorded {
            let days = match period {
                StatsPeriod::Weeks => 28,
                StatsPeriod::Months => 183,
                StatsPeriod::Year => 365,
                _ => 365,
            };
            let now = jiff::Timestamp::now().as_second();
            if let Some(dates) = parse_date(&date_label(now - days * DAY), false)
                .zip(parse_date(&date_label(now), true))
            {
                self.set_dates(dates, cx);
            }
        }
        self.invalid = false;
        self.preset = Some(period);
        cx.notify();
    }

    fn set_dates(&mut self, dates: (i64, i64), cx: &mut Context<Self>) {
        self.stats
            .update(cx, |stats, cx| stats.set_dates(dates, cx));
        self.start
            .update(cx, |input, cx| input.set_text(date_label(dates.0), cx));
        self.end
            .update(cx, |input, cx| input.set_text(date_label(dates.1), cx));
        self.invalid = false;
        self.preset = None;
        self.shown = [0; 5];
        cx.notify();
    }

    fn apply_dates(&mut self, cx: &mut Context<Self>) {
        let start = parse_date(self.start.read(cx).text(), false);
        let end = parse_date(self.end.read(cx).text(), true);
        let dates = start.zip(end).filter(|(a, b)| a <= b);
        match dates {
            Some(dates) => self.set_dates(dates, cx),
            None => {
                self.invalid = true;
                cx.notify();
            }
        }
    }

    fn date_controls(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = *cx.theme();
        let stats = self.stats.read(cx);
        let Some(bounds) = stats.bounds() else {
            return div().into_any_element();
        };
        let dates = stats.dates().unwrap_or(bounds);
        let retained = date_label(bounds.0);
        let low = civil_day(bounds.0);
        let high = civil_day(jiff::Timestamp::now().as_second()).max(low + 1);
        let span = (high - low) as f32;
        let value = (
            ((civil_day(dates.0) - low) as f32 / span).clamp(0., 1.),
            ((civil_day(dates.1) - low) as f32 / span).clamp(0., 1.),
        );
        div()
            .flex()
            .flex_col()
            .gap(theme.metrics.pad)
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_end()
                    .gap(theme.metrics.pad)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w(theme.metrics.cover)
                            .child(ui::faint(cx).child(t!("stats-from")))
                            .child(self.start.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w(theme.metrics.cover)
                            .child(ui::faint(cx).child(t!("stats-to")))
                            .child(self.end.clone()),
                    )
                    .child(
                        Button::new("stats-apply-dates")
                            .label(t!("stats-apply"))
                            .on_click(cx.listener(|this, _, _, cx| this.apply_dates(cx))),
                    ),
            )
            .child(
                RangeScrubber::new(&self.range, value)
                    .name(t!("stats-custom-dates"))
                    .values(date_label(dates.0), date_label(dates.1))
                    .colors(theme.primary, theme.secondary, theme.foreground)
                    .on_change(cx.listener(move |this, &(a, b): &(f32, f32), _, cx| {
                        let from = low + (a * span).round() as i64;
                        let to = low + (b * span).round() as i64;
                        if let Some(dates) = day_date(from)
                            .and_then(|date| parse_date(&date, false))
                            .zip(day_date(to).and_then(|date| parse_date(&date, true)))
                        {
                            this.set_dates(dates, cx);
                        }
                    })),
            )
            .child(ui::faint(cx).child(t!("stats-retained-since", date = retained.as_str())))
            .when(self.invalid, |el| {
                el.child(
                    ui::spoken("stats-invalid-dates", t!("stats-invalid-dates"))
                        .text_color(theme.danger),
                )
            })
            .into_any_element()
    }

    fn artist(
        &self,
        ranked: &Ranked<SavedArtist>,
        rank: usize,
        size: gpui::Pixels,
        cx: &App,
    ) -> AnyElement {
        let artist = &ranked.item;
        let card = match artist.id.is_empty() {
            true => Card::new(
                SharedString::from(format!("stats-artist-card-{}", artist.name)),
                artist.name.clone(),
            )
            .fallback("icons/user-round.svg")
            .circle(),
            false => cards::artist_card(
                SharedString::from(format!("stats-artist-card-{}", artist.id)),
                artist,
                &self.playback,
                cx,
            ),
        };
        card.tile(size)
            .flat()
            .eyebrow(ranking_label(rank, ranked.count))
            .into_any_element()
    }

    /// Shows listening volume and habits derived from recorded play timestamps.
    fn overview(&self, cx: &mut Context<Self>) -> AnyElement {
        let stats = self.stats.read(cx);
        let data = stats.data();
        let theme = *cx.theme();
        let mut values = Vec::new();
        if let Some(count) = data.unique_tracks {
            values.push(t!("stats-overview-tracks", count = count));
        }
        if let Some(count) = data.unique_artists {
            values.push(t!("stats-overview-artists", count = count));
        }
        if let Some(count) = data.new_artists {
            values.push(t!("stats-overview-new-artists", count = count));
        }
        if let Some(count) = data.active_days {
            values.push(t!("stats-overview-days", count = count));
        }
        if let Some(count) = data.longest_streak {
            values.push(t!("stats-overview-streak", count = count));
        }
        if let Some(hour) = data.peak_hour {
            values.push(t!("stats-overview-hour", hour = format!("{hour:02}:00")));
        }
        let max_hour = data.hourly.iter().copied().max().unwrap_or_default().max(1);
        let bars = div()
            .id("stats-hourly-chart")
            .role(Role::List)
            .aria_label(t!("stats-hourly-chart-label"))
            .flex()
            .items_end()
            .gap(theme.metrics.pad / 4.)
            .h(theme.metrics.cover / 2.)
            .children(data.hourly.iter().enumerate().map(|(hour, count)| {
                div()
                    .id(("stats-hour", hour))
                    .role(Role::ListItem)
                    .aria_label(t!(
                        "stats-hourly-entry",
                        hour = format!("{hour:02}:00"),
                        count = *count
                    ))
                    .aria_position_in_set(hour + 1)
                    .aria_size_of_set(24)
                    .flex_1()
                    .h(relative(*count as f32 / max_hour as f32))
                    .min_h(theme.metrics.pad / 4.)
                    .rounded(theme.radius)
                    .bg(theme.primary)
                    .into_any_element()
            }))
            .into_any_element();
        div()
            .flex()
            .flex_col()
            .gap(theme.metrics.pad)
            .child(div().flex().flex_wrap().gap(theme.metrics.pad).children(
                values.into_iter().map(|value| {
                    div()
                        .px(theme.metrics.pad)
                        .py(theme.metrics.pad / 2.)
                        .rounded(theme.radius)
                        .bg(theme.secondary)
                        .child(ui::spoken("stats-overview-item", value))
                        .into_any_element()
                }),
            ))
            .when(!data.hourly.is_empty(), |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.metrics.pad / 4.)
                        .child(bars)
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .children([0, 6, 12, 18, 23].map(|hour| {
                                    ui::faint(cx).child(t!(
                                        "stats-hour-label",
                                        hour = format!("{hour:02}:00")
                                    ))
                                })),
                        ),
                )
            })
            .into_any_element()
    }

    fn section(
        &self,
        key: &'static str,
        index: usize,
        columns: usize,
        width: gpui::Pixels,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let theme = *cx.theme();
        let stats = self.stats.read(cx);
        let data = stats.data();
        let total = match index {
            0 => data.artists.len(),
            1 => data.tracks.len(),
            2 => data.albums.len(),
            3 => data.genres.len(),
            _ => data.descriptors.len(),
        };
        let visible = self.shown[index].max(columns).min(total);
        let layout = CardGrid::layout(width);
        let size = match index {
            3 | 4 => {
                (width - theme.metrics.inset * columns.saturating_sub(1) as f32) / columns as f32
            }
            _ => layout.card,
        };
        let tracks = Rc::new(
            data.tracks
                .iter()
                .map(|ranked| ranked.item.clone())
                .collect::<Vec<_>>(),
        );
        let played = tracks.clone();
        let playback = self.playback.clone();
        let start: StartHandler = Rc::new(move |place, cx| {
            playback.update(cx, |playback, cx| {
                playback.start(played.as_ref().clone(), place, None, cx)
            });
        });
        let mut rows = Vec::new();
        for chunk in (0..visible).collect::<Vec<_>>().chunks(columns) {
            let mut cells = Vec::new();
            for &at in chunk {
                let (id, card) = match index {
                    0 => {
                        let artist = &data.artists[at].item;
                        (
                            format!("stats-artist-{}-{}", artist.id, artist.name),
                            self.artist(&data.artists[at], at + 1, size, cx),
                        )
                    }
                    1 => {
                        let ranked = &data.tracks[at];
                        let track = &ranked.item;
                        let id =
                            format!("stats-track-{}", track.id.as_deref().unwrap_or(&track.name));
                        let card = TrackCard::new(
                            "stats-track-card",
                            at,
                            tracks.clone(),
                            self.playback.clone(),
                        )
                        .start(Some(start.clone()))
                        .render(cx)
                        .tile(size)
                        .flat()
                        .eyebrow(ranking_label(at + 1, ranked.count))
                        .into_any_element();
                        (id, card)
                    }
                    2 => {
                        let ranked = &data.albums[at];
                        let id = format!("stats-album-{}", ranked.item.id);
                        let card = cards::album_card(
                            SharedString::from(id.clone()),
                            &ranked.item,
                            &self.playback,
                            cx,
                        )
                        .tile(size)
                        .flat()
                        .eyebrow(ranking_label(at + 1, ranked.count))
                        .into_any_element();
                        (id, card)
                    }
                    _ => {
                        let collection = if index == 3 {
                            &data.genres
                        } else {
                            &data.descriptors
                        };
                        let genre = &collection[at];
                        let count = genre.count.unwrap_or(0);
                        let total_hits: usize = collection.iter().filter_map(|g| g.count).sum();
                        let share = count as f32 / total_hits.max(1) as f32;
                        let label = t!(
                            "stats-genre-share",
                            genre = genre.item.as_str(),
                            percent = (share * 100.).round() as usize
                        );
                        let card = div()
                            .w(size)
                            .flex()
                            .flex_col()
                            .gap(theme.metrics.pad)
                            .p(theme.metrics.pad)
                            .rounded(theme.radius)
                            .bg(theme.secondary)
                            .child(ui::spoken(
                                SharedString::from(format!("stats-genre-label-{}", genre.item)),
                                label,
                            ))
                            .child(
                                div()
                                    .w_full()
                                    .h(theme.metrics.pad / 2.)
                                    .rounded(theme.radius)
                                    .bg(theme.border)
                                    .child(
                                        div()
                                            .w(relative(share))
                                            .h_full()
                                            .rounded(theme.radius)
                                            .bg(theme.primary),
                                    ),
                            )
                            .into_any_element();
                        (format!("stats-term-{index}-{}", genre.item), card)
                    }
                };
                cells.push(
                    div()
                        .id(SharedString::from(id))
                        .role(Role::ListItem)
                        .aria_label(t!("stats-rank", rank = at + 1))
                        .aria_position_in_set(at + 1)
                        .aria_size_of_set(total)
                        .child(card)
                        .into_any_element(),
                );
            }
            rows.push(match index {
                3 | 4 => div()
                    .flex()
                    .gap(theme.metrics.inset)
                    .w_full()
                    .children(cells)
                    .into_any_element(),
                _ => CardGrid::new(width).children(cells).into_any_element(),
            });
        }
        let note = match index {
            3 if stats.source() == StatsSource::Provider => t!("stats-provider-genres-unavailable"),
            3 => t!("stats-genres-empty"),
            4 => t!("stats-descriptors-empty"),
            _ => t!("stats-empty"),
        };
        div()
            .flex()
            .flex_col()
            .gap(theme.metrics.inset)
            .child(ui::spoken_heading(key, i18n::lookup(key, None), 2, cx))
            .when(
                index == 2 && stats.source() == StatsSource::Provider && total > 0,
                |el| {
                    el.child(
                        ui::spoken(
                            "stats-provider-albums-note",
                            t!("stats-provider-albums-note"),
                        )
                        .text_size(theme.text(Text::Small))
                        .text_color(theme.muted_foreground),
                    )
                },
            )
            .when(
                index == 3 && stats.source() == StatsSource::Provider && total > 0,
                |el| {
                    el.child(
                        ui::spoken("stats-provider-genre-note", t!("stats-provider-genre-note"))
                            .text_size(theme.text(Text::Small))
                            .text_color(theme.muted_foreground),
                    )
                },
            )
            .when(
                index == 4 && stats.source() == StatsSource::Provider && total > 0,
                |el| {
                    el.child(
                        ui::spoken("stats-genre-basis", t!("stats-provider-descriptors-note"))
                            .text_size(theme.text(Text::Small))
                            .text_color(theme.muted_foreground),
                    )
                },
            )
            .child(
                div()
                    .id(key)
                    .role(Role::List)
                    .aria_label(i18n::lookup(key, None))
                    .flex()
                    .flex_col()
                    .gap(theme.metrics.inset)
                    .children(rows),
            )
            .when(total == 0, |el| el.child(ui::vacant(note, cx)))
            .when(visible < total, |el| {
                el.child(
                    Button::new(SharedString::from(format!("{key}-more")))
                        .outline()
                        .label(t!("stats-show-more"))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.shown[index] = visible + PAGE;
                            cx.notify();
                        })),
                )
            })
            .into_any_element()
    }
}

impl Render for StatsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.theme();
        let source = self.stats.read(cx).source();
        let period = self.stats.read(cx).period();
        let session = Sonora::global(cx).session.read(cx);
        let provider = session.provider_name().unwrap_or_default();
        let periods = self.stats.read(cx).periods(cx);
        let width =
            (Chrome::content(window, cx) - theme.metrics.inset * 2.).max(theme.metrics.cover);
        let columns = CardGrid::layout(width).columns;
        let mut sources = vec![
            Button::new("stats-recorded-tab")
                .ghost()
                .label(t!("stats-recorded"))
                .selected(source == StatsSource::Recorded)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.select(StatsSource::Recorded, period, cx)
                })),
        ];
        if !periods.is_empty() {
            sources.insert(
                0,
                Button::new("stats-provider-tab")
                    .ghost()
                    .label(provider)
                    .selected(source == StatsSource::Provider)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.select(StatsSource::Provider, period, cx)
                    })),
            );
        }
        let scrobblers = self.stats.read(cx).scrobblers(cx);
        for id in scrobblers {
            let title = match id {
                "lastfm" => t!("settings-lastfm"),
                "librefm" => t!("settings-librefm"),
                "listenbrainz" => t!("settings-listenbrainz"),
                _ => id.into(),
            };
            sources.push(
                Button::new(SharedString::from(format!("stats-scrobbler-{id}")))
                    .ghost()
                    .label(title)
                    .selected(source == StatsSource::Scrobbler(id))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.select(StatsSource::Scrobbler(id), period, cx)
                    })),
            );
        }
        let windows = match source {
            StatsSource::Recorded => vec![StatsPeriod::Weeks, StatsPeriod::Months, StatsPeriod::Year],
            _ => periods.clone(),
        };
        let controls = windows
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                let label = match source == StatsSource::Provider
                    && periods.as_slice() == [StatsPeriod::Year]
                    && p == StatsPeriod::Year
                {
                    true => t!("stats-replay-year"),
                    false => i18n::lookup(period_key(p), None),
                };
                Button::new(("stats-period", i))
                    .outline()
                    .selected(match source {
                        StatsSource::Provider => p == period,
                        StatsSource::Recorded => self.preset == Some(p),
                        StatsSource::Scrobbler(_) => p == period,
                    })
                    .label(label)
                    .on_click(cx.listener(move |this, _, _, cx| this.select(source, p, cx)))
            })
            .collect::<Vec<_>>();
        let note = match source {
            StatsSource::Provider => t!("stats-provider-note"),
            StatsSource::Recorded => t!("stats-recorded-note"),
            StatsSource::Scrobbler(_) => t!("stats-scrobbler-note"),
        };
        let loading = self.stats.read(cx).is_loading();
        let failed = self.stats.read(cx).failed();
        let content = div()
            .flex()
            .flex_col()
            .gap(theme.metrics.inset)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(theme.metrics.pad)
                    .child(
                        ui::spoken_heading("stats-title", t!("nav-stats"), 1, cx)
                            .text_size(theme.text(Text::Display)),
                    )
                    .child(
                        TabBar::new("stats-source-tabs")
                            .name(t!("stats-source"))
                            .max_w_full()
                            .items(sources),
                    )
                    .child(ui::faint(cx).child(note)),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(theme.metrics.pad)
                    .children(controls)
                    .when(source == StatsSource::Recorded, |el| {
                        el.child(
                            Button::new("stats-all-time")
                                .outline()
                                .label(t!("stats-all-time"))
                                .selected(self.stats.read(cx).dates().is_none())
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.preset = None;
                                    this.stats.update(cx, |stats, cx| stats.all_dates(cx));
                                    this.sync_dates(cx);
                                })),
                        )
                    }),
            )
            .when(source == StatsSource::Recorded, |el| {
                el.child(self.date_controls(cx))
            })
            .when(loading, |el| {
                el.child(ui::spoken("stats-loading", t!("stats-loading")))
            })
            .when(failed, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.metrics.pad)
                        .child(ui::spoken("stats-failed", t!("stats-failed")))
                        .child(
                            Button::new("stats-retry")
                                .label(t!("stats-retry"))
                                .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
                        ),
                )
            })
            .when(!loading && !failed, |el| {
                el.when_some(self.stats.read(cx).data().plays, |el, count| {
                    el.child(ui::faint(cx).child(t!("stats-plays", count = count)))
                })
                .child(self.section("stats-top-artists", 0, columns, width, cx))
                .child(self.section("stats-top-songs", 1, columns, width, cx))
                .when(source == StatsSource::Recorded, |el| {
                    el.child(self.overview(cx))
                })
                .child(self.section("stats-top-albums", 2, columns, width, cx))
                .child(self.section("stats-top-genres", 3, columns.min(3), width, cx))
                .child(self.section(
                    "stats-top-descriptors",
                    4,
                    columns.min(3),
                    width,
                    cx,
                ))
            });
        Scroller::new("stats-page", &self.scrollbar)
            .size_full()
            .bottom_inset(Chrome::bottom(cx))
            .p(theme.metrics.inset)
            .child(content)
    }
}

fn period_key(period: StatsPeriod) -> &'static str {
    match period {
        StatsPeriod::Weeks => "stats-four-weeks",
        StatsPeriod::Months => "stats-six-months",
        StatsPeriod::Year => "stats-year",
        StatsPeriod::AllTime => "stats-all-time",
        StatsPeriod::ThisWeek => "stats-this-week",
        StatsPeriod::ThisMonth => "stats-this-month",
        StatsPeriod::ThisYear => "stats-this-year",
        StatsPeriod::Week => "stats-week",
        StatsPeriod::Month => "stats-month",
        StatsPeriod::Quarter => "stats-quarter",
        StatsPeriod::HalfYear => "stats-half-year",
    }
}

/// Adds recorded play counts to the rank without fabricating counts for provider rankings.
fn ranking_label(rank: usize, count: Option<usize>) -> SharedString {
    match count {
        Some(count) => t!("stats-ranked-plays", rank = rank, count = count),
        None => t!("stats-rank", rank = rank),
    }
}

/// Dates are local calendar days; the upper bound includes the whole day, across DST changes.
fn parse_date(value: &str, end: bool) -> Option<i64> {
    let date: jiff::civil::Date = value.trim().parse().ok()?;
    let date = match end {
        true => date.tomorrow().ok()?,
        false => date,
    };
    let stamp = date
        .at(0, 0, 0, 0)
        .to_zoned(jiff::tz::TimeZone::system())
        .ok()?
        .timestamp()
        .as_second();
    Some(stamp - i64::from(end))
}

fn date_label(seconds: i64) -> String {
    jiff::Timestamp::new(seconds, 0)
        .ok()
        .map(|stamp| {
            stamp
                .to_zoned(jiff::tz::TimeZone::system())
                .date()
                .to_string()
        })
        .unwrap_or_default()
}

/// Maps local dates to consecutive UTC day numbers so slider handles snap to calendar days.
fn civil_day(seconds: i64) -> i64 {
    date_label(seconds)
        .parse::<jiff::civil::Date>()
        .ok()
        .and_then(|date| date.at(0, 0, 0, 0).to_zoned(jiff::tz::TimeZone::UTC).ok())
        .map(|date| date.timestamp().as_second() / DAY)
        .unwrap_or_default()
}

fn day_date(day: i64) -> Option<String> {
    Some(
        jiff::Timestamp::new(day * DAY, 0)
            .ok()?
            .strftime("%Y-%m-%d")
            .to_string(),
    )
}
