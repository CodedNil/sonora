use std::collections::HashMap;

use gpui::prelude::*;
use gpui::{AnyElement, App, Div, ElementId, Entity, Pixels, ScrollHandle, Window, div, px};
use ui::ActiveTheme as _;

use state::{AppSettings, FilterValue, Playback};
use ui::{
    Button, Filter, FilterChange, FilterKind, Listing, TableState, Viewport, quantize, scrolled,
};

use crate::shared::cells;
use crate::shared::tracks::{self, TrackSource};

/// Places a page heading and its controls in one consistent row.
pub(crate) fn row(leading: impl IntoElement, trailing: impl IntoElement) -> AnyElement {
    div()
        .flex()
        .w_full()
        .min_w_0()
        .items_center()
        .gap_x_4()
        .justify_between()
        .child(leading)
        .child(trailing)
        .into_any_element()
}

/// Creates a small directional button used by paged decks and horizontal shelves.
pub(crate) fn arrow_button(id: impl Into<ElementId>, previous: bool) -> Button {
    Button::new(id)
        .small()
        .outline()
        .icon(match previous {
            true => "icons/chevron-left.svg",
            false => "icons/chevron-right.svg",
        })
        .tooltip(match previous {
            true => "common-previous",
            false => "common-next",
        })
}

pub(crate) fn store(
    settings: &Entity<AppSettings>,
    table: &dyn Listing,
    layout_key: &str,
    sort_key: &str,
    cx: &mut App,
) {
    let layout = table.layout(cx);
    let sorting = table.sorting(cx);
    let axes = table.filters(cx);

    settings.update(cx, |settings, cx| {
        settings.set_table(layout_key, layout, cx);
        settings.set_sorting(sort_key, sorting, cx);
        let filters = snapshot(axes, settings.filters(sort_key).unwrap_or_default());
        settings.set_filters(sort_key, filters, cx);
    });
}

/// Merges the current axes into the stored ones: a narrowed axis upserts its key, a whole one
/// drops it, and an axis the table does not name right now leaves storage alone. That last
/// rule is what keeps a saved year span alive while its rows are still loading, and resetting
/// a table clears its entry on the next store.
pub(crate) fn snapshot(
    axes: Vec<Filter>,
    into: HashMap<String, FilterValue>,
) -> HashMap<String, FilterValue> {
    let mut stored = into;
    for axis in axes {
        let value = match axis.kind {
            FilterKind::Range(range) => {
                (!range.whole()).then_some(FilterValue::Range(range.value.0, range.value.1))
            }
            FilterKind::Flag(on) => on.then_some(FilterValue::Flag(true)),
            FilterKind::Choice(options) => {
                let picked: Vec<_> = options
                    .into_iter()
                    .filter(|option| option.on)
                    .map(|option| option.id.to_string())
                    .collect();
                (!picked.is_empty()).then_some(FilterValue::Choice(picked))
            }
        };
        match value {
            Some(value) => stored.insert(axis.key.to_owned(), value),
            None => stored.remove(axis.key),
        };
    }
    stored
}

/// Clears saved filter axes, including ones not currently offered, while keeping text search.
pub(crate) fn clear_filters(settings: &Entity<AppSettings>, key: &str, cx: &mut App) {
    settings.update(cx, |settings, cx| {
        let mut filters = settings.filters(key).unwrap_or_default();
        filters.retain(|_, value| matches!(value, FilterValue::Text(_)));
        settings.set_filters(key, filters, cx);
    });
}

/// Applies the stored axes a table has not narrowed itself yet. Ranges clamp to the current
/// bounds, so a span saved before its rows arrived still lands, while one the library outgrew
/// stays whole instead of filtering everything out. Stored keys the table no longer names are
/// ignored.
pub(crate) fn restore(
    settings: &Entity<AppSettings>,
    table: &dyn Listing,
    key: &str,
    cx: &mut App,
) {
    let Some(stored) = settings.read(cx).filters(key) else {
        return;
    };
    for axis in table.filters(cx) {
        let key = axis.key;
        match (axis.kind, stored.get(key)) {
            (FilterKind::Range(range), Some(FilterValue::Range(low, high))) => {
                let value = (
                    low.clamp(range.bounds.0, range.bounds.1),
                    high.clamp(range.bounds.0, range.bounds.1),
                );
                if value != range.bounds && value != range.value {
                    table.filter(FilterChange::Range(key, Some(value)), cx);
                }
            }
            (FilterKind::Flag(on), Some(FilterValue::Flag(wanted))) if on != *wanted => {
                table.filter(FilterChange::Flag(key, *wanted), cx);
            }
            (FilterKind::Choice(options), Some(FilterValue::Choice(picked))) => {
                for option in options {
                    let wanted = picked.iter().any(|id| option.id == id.as_str());
                    if wanted != option.on {
                        table.filter(FilterChange::Choice(key, option.id, wanted), cx);
                    }
                }
            }
            _ => {}
        }
    }
}

pub(crate) fn reserved(inset: Pixels) -> Pixels {
    inset * 2. + px(2.)
}

pub(crate) fn play(
    table: &Entity<TableState<TrackSource>>,
    playback: &Entity<Playback>,
    display: usize,
    cx: &mut App,
) {
    let queued = tracks::ordered(table, cx);
    // A row that cannot play while the network is gone stays inert, with its tooltip saying why.
    let unreachable = queued
        .get(display)
        .and_then(|track| playback.read(cx).unreachable_reason(track, cx));
    if unreachable.is_some() {
        return;
    }
    let from = tracks::whence(table, cx);
    playback.update(cx, |playback, cx| playback.start(queued, display, from, cx));
}

pub(crate) fn play_or_toggle(
    table: &Entity<TableState<TrackSource>>,
    playback: &Entity<Playback>,
    display: usize,
    cx: &mut App,
) {
    let queued = tracks::ordered(table, cx);
    let Some(track) = queued.get(display) else {
        return;
    };
    let current = playback.read(cx).track();
    let same = current.and_then(|track| track.id.as_deref()) == track.id.as_deref();
    match same {
        true => playback.update(cx, |playback, cx| playback.toggle_play(cx)),
        false => play(table, playback, display, cx),
    }
}

pub(crate) fn resize(
    table: &dyn Listing,
    width: &mut Pixels,
    inset: Pixels,
    window: &Window,
    cx: &mut App,
) {
    let next = cells::content_width(window, reserved(inset), cx);
    if (next - *width).abs() < px(0.5) {
        return;
    }
    *width = next;
    table.set_width(next, cx);
}

pub(crate) fn viewport(scroll: &ScrollHandle, header: Pixels, window: &Window) -> Viewport {
    quantize(scroll, window);
    let hero = scroll
        .bounds_for_item(0)
        .map(|bounds| bounds.size.height)
        .unwrap_or_default();
    let visible = scroll.bounds().size.height;

    let mut viewport =
        Viewport::measured(scrolled(scroll) - hero + header, visible - header, window);
    viewport.underlap = header;
    viewport
}

/// Reveals the compact header as the full hero scrolls away, keeping table and snap targets below it.
pub(crate) fn sticky(bar: &Entity<ui::Scrollbar>, cx: &App) -> Pixels {
    let bar = bar.read(cx);
    let scroll = bar.scroll();
    let height = cx.theme().metrics.control + cx.theme().metrics.pad * 2.;
    let hero = scroll.bounds_for_item(0).map(|bounds| bounds.size.height);
    let shown = hero.is_some_and(|hero| scrolled(scroll) + height >= hero);
    let height = match shown {
        true => height,
        false => Pixels::ZERO,
    };
    bar.anchors().set_inset(height);
    height
}

/// The shared surface for a compact page header, above the sticky table headings.
pub(crate) fn header(content: impl IntoElement, inset: Pixels, cx: &App) -> Div {
    ui::popover_surface(
        div()
            .absolute()
            .top_0()
            .left(inset)
            .right(inset)
            .px(cx.theme().metrics.pad)
            .rounded_tl(cx.theme().radius)
            .rounded_tr(cx.theme().radius)
            .min_w_0()
            .overflow_hidden()
            .occlude(),
        cx,
    )
    .child(content)
}
