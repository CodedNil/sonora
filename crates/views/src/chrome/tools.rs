use std::cell::RefCell;
use std::rc::Rc;

use gpui::prelude::*;
use gpui::{AnyElement, App, SharedString, div, svg};
use ui::{
    ActiveTheme as _, Button, Filter, FilterChange, MenuItem, Mode, Picker, Popovers,
    RangeScrubber, RangeState, Sort, SortAxis, Text, Tipped as _, Unit, eyebrow,
};

const FILTERS: &str = "filters";
const SORTS: &str = "sorts";

#[derive(Default)]
pub(crate) struct Sliders(RefCell<Vec<(&'static str, RangeState)>>);

impl Sliders {
    fn state(&self, key: &'static str) -> RangeState {
        let mut cache = self.0.borrow_mut();
        if let Some((_, state)) = cache.iter().find(|(known, _)| *known == key) {
            return state.clone();
        }

        let state = RangeState::new(key);
        cache.push((key, state.clone()));
        state
    }
}

pub(crate) fn sorts(
    group: &Popovers,
    axes: Vec<SortAxis>,
    rank: impl Fn(&'static str, &mut App) + 'static,
    cx: &App,
) -> AnyElement {
    let theme = *cx.theme();
    let sorted = axes.iter().any(|axis| axis.order.is_some());
    let rank = Rc::new(rank);

    Picker::icon(SORTS, group, "icons/arrow-up-down.svg")
        .tooltip("tool-sort")
        .sticky()
        .tint(match sorted {
            true => theme.primary,
            false => theme.muted_foreground,
        })
        .items(axes.into_iter().map(move |axis| {
            let key = axis.key;
            let rank = rank.clone();
            let arrow = axis.order.map(|order| match order {
                Sort::Ascending => "icons/chevron-up.svg",
                Sort::Descending => "icons/chevron-down.svg",
            });

            MenuItem::new(key, axis.label)
                .selected(axis.order.is_some())
                .when_some(arrow, MenuItem::icon)
                .on_click(move |_, _, cx| rank(key, cx))
        }))
        .into_any_element()
}

pub(crate) fn filters(
    group: &Popovers,
    sliders: &Sliders,
    axes: Vec<Filter>,
    filter_fn: impl Fn(FilterChange, &mut App) + 'static,
    cx: &App,
) -> AnyElement {
    if axes.is_empty() {
        return div().into_any_element();
    }
    let theme = *cx.theme();
    let narrowed = axes.iter().any(|axis| axis.narrowed());
    let active = div().flex().items_center().gap_1().children(
        axes.iter()
            .filter(|axis| axis.narrowed())
            .map(|axis| match axis {
                Filter::Range(axis) => div()
                    .id(axis.key)
                    .text_size(theme.text(Text::Small))
                    .tip_label(axis.label.clone(), ui::Perch::Pointer)
                    .child(compact_range(axis))
                    .into_any_element(),
                Filter::Flag(axis) => div()
                    .id(axis.key)
                    .tip_label(axis.label.clone(), ui::Perch::Pointer)
                    .map(|this| match filter_icon(axis.key) {
                        Some(icon) => this.child(
                            svg()
                                .path(icons::path(icon))
                                .size(theme.text(Text::Label))
                                .flex_none()
                                .text_color(theme.foreground),
                        ),
                        None => this.child(axis.label.clone()),
                    })
                    .into_any_element(),
            })
            .collect::<Vec<_>>(),
    );
    let filter_fn = Rc::new(filter_fn);

    let items = axes.iter().map(|axis| match axis {
        Filter::Range(axis) => {
            let key = axis.key;
            let unit = axis.unit;
            let copy = axis.clone();
            let filter_fn = filter_fn.clone();
            let state = sliders.state(key);

            MenuItem::new(key, axis.label.clone()).content(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .py_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .child(eyebrow(axis.label.clone(), cx))
                            .child(
                                div()
                                    .text_size(theme.text(Text::Small))
                                    .text_color(theme.muted_foreground)
                                    .child(format!(
                                        "{} - {}",
                                        unit.say(axis.value.0),
                                        unit.say(axis.value.1)
                                    )),
                            ),
                    )
                    .child(
                        RangeScrubber::new(&state, axis.share())
                            .stops(axis.stops())
                            .colors(theme.progress_bar, theme.muted, theme.foreground)
                            .on_change(move |share: &(f32, f32), _, cx| {
                                filter_fn(FilterChange::Range(key, copy.at(*share)), cx);
                            }),
                    ),
            )
        }
        Filter::Flag(axis) => {
            let key = axis.key;
            let on = axis.on;
            let filter_fn = filter_fn.clone();

            MenuItem::new(key, axis.label.clone())
                .when_some(filter_icon(key), |item, icon| item.icon(icon))
                .selected(on)
                .on_click(move |_, _, cx| filter_fn(FilterChange::Flag(key, !on), cx))
        }
    });

    let reset = filter_fn.clone();

    Picker::icon(FILTERS, group, "icons/funnel.svg")
        .when(narrowed, |picker| {
            picker.button(
                Button::new("active-filters")
                    .outline()
                    .small()
                    .rounded_full()
                    .icon("icons/funnel.svg")
                    .leading(active),
            )
        })
        .tooltip("tool-filters")
        .sticky()
        .width(Picker::WIDE)
        .tint(match narrowed {
            true => theme.primary,
            false => theme.muted_foreground,
        })
        .items(items)
        .item(MenuItem::separator("filters-end"))
        .item(
            MenuItem::new("filters-reset", i18n::t!("filter-reset"))
                .on_click(move |_, _, cx| reset(FilterChange::Reset, cx)),
        )
        .into_any_element()
}

pub(crate) fn views(
    group: &Popovers,
    mode: Mode,
    shift: impl Fn(Mode, &mut App) + 'static,
) -> AnyElement {
    let next = match mode {
        Mode::List => Mode::Grid,
        Mode::Grid => Mode::List,
    };
    let group = group.clone();

    Button::new("view-toggle")
        .icon(next.icon())
        .tooltip(next.key())
        .small()
        .ghost()
        .on_click(move |_, _, cx| {
            group.close();
            shift(next, cx);
        })
        .into_any_element()
}

/// Uses the same bounds as the range control, omitting the unchanged end from its summary.
fn compact_range(axis: &ui::RangeAxis) -> SharedString {
    let low = axis.unit.say(axis.value.0);
    let high = axis.unit.say(axis.value.1);
    let (from, to) = axis.share();
    let value = match (from > f32::EPSILON, to < 1. - f32::EPSILON) {
        (false, true) => i18n::t!("filter-range-max", value = high.as_ref()),
        (true, false) => i18n::t!("filter-range-min", value = low.as_ref()),
        _ => i18n::t!("filter-range-both", min = low.as_ref(), max = high.as_ref()),
    };
    match axis.unit {
        Unit::Clock => value,
        Unit::Plain => i18n::t!(
            "filter-active-value",
            filter = axis.label.as_ref(),
            value = value.as_ref()
        ),
    }
}

/// Shared compact symbols for filter flags, with the localized name retained as a tooltip.
fn filter_icon(key: &str) -> Option<&'static str> {
    Some(match key {
        "filter-hide-explicit" => "icons/explicit-off.svg",
        "filter-playable" => "icons/circle-check.svg",
        "filter-favorites" => "icons/heart-filled.svg",
        "filter-collaborations" => "icons/users-round-collaboration.svg",
        "filter-downloaded" => "icons/download.svg",
        "filter-owned" => "icons/user-round.svg",
        _ => return None,
    })
}
