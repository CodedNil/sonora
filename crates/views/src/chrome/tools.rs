use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use gpui::prelude::*;
use gpui::{AnyElement, App, Entity, EntityId, Hsla, SharedString, div, svg};
use state::FilterValue;
use ui::{
    ActiveTheme as _, Button, Choice, Filter, FilterChange, FilterKind, Menu, MenuItem, MenuSearch,
    Mode, Picker, Popovers, RangeAxis, RangeScrubber, RangeState, Scrollbar, Sort, SortAxis,
    SubmenuState, Text, Tipped as _, Unit, eyebrow,
};

const FILTERS: &str = "filters";
const SORTS: &str = "sorts";

/// Stores only narrowed axes, shared by track tables and library shelves.
#[derive(Clone, Default, PartialEq)]
pub(crate) struct FilterState(BTreeMap<&'static str, FilterValue>);

impl FilterState {
    pub(crate) fn active(&self) -> bool {
        !self.0.is_empty()
    }

    pub(crate) fn on(&self, key: &str) -> bool {
        matches!(self.0.get(key), Some(FilterValue::Flag(true)))
    }

    pub(crate) fn range(&self, key: &str) -> Option<(f32, f32)> {
        match self.0.get(key) {
            Some(FilterValue::Range(low, high)) => Some((*low, *high)),
            _ => None,
        }
    }

    pub(crate) fn picks(&self, key: &str) -> Option<&[String]> {
        match self.0.get(key) {
            Some(FilterValue::Choice(picked)) => Some(picked),
            _ => None,
        }
    }

    /// Offers enabled flag axes in caller order, resolving their labels at render time.
    pub(crate) fn flags(
        &self,
        axes: impl IntoIterator<Item = (&'static str, bool)>,
    ) -> Vec<Filter> {
        axes.into_iter()
            .filter(|(_, offered)| *offered)
            .map(|(key, _)| Filter {
                key,
                kind: FilterKind::Flag(self.on(key)),
            })
            .collect()
    }

    /// Applies a table event and drops values that no longer narrow it.
    pub(crate) fn apply(&mut self, change: FilterChange) -> bool {
        let (key, value) = match change {
            FilterChange::Range(key, value) => {
                (key, value.map(|(low, high)| FilterValue::Range(low, high)))
            }
            FilterChange::Flag(key, on) => (key, on.then_some(FilterValue::Flag(true))),
            FilterChange::Choice(key, id, on) => {
                let mut picked = self.picks(key).unwrap_or_default().to_vec();
                picked.retain(|known| known != id.as_ref());
                if on {
                    picked.push(id.to_string());
                }
                (
                    key,
                    (!picked.is_empty()).then_some(FilterValue::Choice(picked)),
                )
            }
            FilterChange::Reset => {
                self.0.clear();
                return true;
            }
        };
        match value {
            Some(value) => self.0.insert(key, value),
            None => self.0.remove(key),
        };
        true
    }
}

/// The shared background for active search and filter controls.
pub(crate) fn active_filter_background(cx: &App) -> Hsla {
    cx.theme().primary.opacity(0.2)
}

/// Builds a numeric axis when the available values span more than one value. Clock and tempo
/// sliders use continuous bounds; other numeric fields stop at the observed values.
pub(crate) fn range(
    key: &'static str,
    values: &[f32],
    current: Option<(f32, f32)>,
    unit: Unit,
) -> Option<Filter> {
    let low = values.iter().copied().reduce(f32::min)?;
    let high = values.iter().copied().reduce(f32::max)?;
    (high > low).then(|| Filter {
        key,
        kind: FilterKind::Range(
            RangeAxis {
                bounds: (low, high),
                value: current.unwrap_or((low, high)),
                unit,
                values: (unit == Unit::Plain && key != "filter-bpm").then(|| values.to_vec()),
            }
            .clamped(),
        ),
    })
}

/// What one choice filter's submenu keeps between frames: whether it is open, what is typed in
/// its search, and how far its list is scrolled.
#[derive(Clone)]
struct ChoiceMenu {
    state: SubmenuState,
    search: MenuSearch,
    scrollbar: Entity<Scrollbar>,
}

#[derive(Default)]
pub(crate) struct Sliders {
    ranges: RefCell<BTreeMap<&'static str, RangeState>>,
    choices: BTreeMap<&'static str, ChoiceMenu>,
}

impl Sliders {
    /// Gives the filters the submenus for the choice axes `keys` name. A screen whose table
    /// offers a choice axis must build its sliders this way, or the axis stays out of the menu.
    pub(crate) fn with_choices(
        keys: impl IntoIterator<Item = &'static str>,
        watching: EntityId,
        cx: &mut App,
    ) -> Self {
        Self {
            ranges: RefCell::default(),
            choices: keys
                .into_iter()
                .map(|key| {
                    (
                        key,
                        ChoiceMenu {
                            state: SubmenuState::instant(),
                            search: MenuSearch::new("common-search", cx),
                            scrollbar: cx.new(|_| Scrollbar::inset().watching(watching)),
                        },
                    )
                })
                .collect(),
        }
    }

    fn choice_menu(&self, key: &'static str) -> Option<ChoiceMenu> {
        self.choices.get(key).cloned()
    }

    fn state(&self, key: &'static str) -> RangeState {
        self.ranges
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| RangeState::new(key))
            .clone()
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
        .large()
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
    let theme = *cx.theme();
    let narrowed = axes.iter().any(Filter::narrowed);
    let text = |words: SharedString| {
        div()
            .min_w_0()
            .truncate()
            .text_size(theme.text(Text::Small))
            .child(words)
            .into_any_element()
    };
    let active = axes.iter().filter(|axis| axis.narrowed()).map(|axis| {
        let key = axis.key;
        let label = axis.label();
        let icon = match &axis.kind {
            FilterKind::Flag(_) => filter_icon(key),
            _ => None,
        };
        let icon_only = icon.is_some();
        let content = match &axis.kind {
            FilterKind::Range(range) => text(compact_range(key, range)),
            FilterKind::Choice(options) => {
                let picked: Vec<_> = options
                    .iter()
                    .filter(|option| option.on)
                    .map(|option| option.label.as_ref())
                    .collect();
                text(format!("{label}: {}", picked.join(", ")).into())
            }
            FilterKind::Flag(_) => match icon {
                Some(icon) => svg()
                    .path(icons::path(icon))
                    .size(theme.text(Text::Label))
                    .flex_none()
                    .text_color(theme.primary)
                    .into_any_element(),
                None => text(label.clone()),
            },
        };

        div()
            .id(key)
            .flex()
            .items_center()
            .h(theme.metrics.control_small)
            .when(icon_only, |this| {
                this.w(theme.metrics.control_small).justify_center()
            })
            .when(!icon_only, |this| {
                this.px_2().max_w(theme.metrics.control * 4.)
            })
            .rounded_full()
            .bg(active_filter_background(cx))
            .tip_label(label, ui::Perch::Below)
            .child(content)
            .into_any_element()
    });
    let active = div().flex().items_center().gap_1().children(active);
    let filter_fn = Rc::new(filter_fn);
    let items = axes.iter().map(|axis| {
        let key = axis.key;
        let label = axis.label();
        let item = MenuItem::new(key, label.clone());
        let filter_fn = filter_fn.clone();
        match &axis.kind {
            FilterKind::Range(range) => {
                let value = (range.unit.say(range.value.0), range.unit.say(range.value.1));
                let state = sliders.state(key);
                let range = range.clone();
                let slider = RangeScrubber::new(&state, range.share())
                    .name(label.clone())
                    .values(value.0.clone(), value.1.clone())
                    .stops(range.stops())
                    .colors(theme.progress_bar, theme.muted, theme.foreground)
                    .on_change(move |share: &(f32, f32), _, cx| {
                        filter_fn(FilterChange::Range(key, range.narrowing(*share)), cx);
                    });
                item.content(
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
                                .child(eyebrow(label, cx))
                                .child(
                                    ui::spoken(
                                        SharedString::from(format!("{key}-range")),
                                        format!("{} - {}", value.0, value.1),
                                    )
                                    .text_size(theme.text(Text::Small))
                                    .text_color(theme.muted_foreground),
                                ),
                        )
                        .child(slider),
                )
            }
            FilterKind::Flag(on) => {
                let on = *on;
                item.when_some(filter_icon(key), MenuItem::icon)
                    .selected(on)
                    .on_click(move |_, _, cx| filter_fn(FilterChange::Flag(key, !on), cx))
            }
            FilterKind::Choice(options) => match sliders.choice_menu(key) {
                Some(menu) => choice_item(key, options, menu, &filter_fn, cx),
                None => item.disabled(),
            },
        }
    });

    let reset = filter_fn.clone();
    let open = group.shows(FILTERS);
    let trigger = Button::new(match narrowed {
        true => "active-filters",
        false => "filters-picker",
    })
    .icon("icons/funnel.svg")
    .when_else(
        narrowed,
        |button| button.outline().rounded_full().pr_0().leading(active),
        |button| button.ghost().border_1(),
    )
    .border_color(match (open, narrowed) {
        (true, _) => theme.primary,
        (false, true) => theme.border,
        (false, false) => theme.border.opacity(0.),
    });
    Picker::icon(FILTERS, group, "icons/funnel.svg")
        .button(trigger)
        .tooltip("tool-filters")
        .sticky()
        .width(Picker::WIDE)
        .tint(theme.foreground)
        .items(items)
        .item(MenuItem::separator("filters-end"))
        .item(
            MenuItem::new("filters-reset", i18n::t!("filter-reset"))
                .on_click(move |_, _, cx| reset(FilterChange::Reset, cx)),
        )
        .into_any_element()
}

/// A menu row for a choice axis, opening a searchable list of its options that stays open as
/// each is picked.
fn choice_item(
    key: &'static str,
    options: &[Choice],
    menu: ChoiceMenu,
    filter_fn: &Rc<impl Fn(FilterChange, &mut App) + 'static>,
    cx: &App,
) -> MenuItem {
    let query = menu.search.query(cx);
    let found: Vec<&Choice> = options
        .iter()
        .filter(|option| option.label.to_lowercase().contains(&query))
        .collect();
    let unmatched = found.is_empty();
    let rows = found.into_iter().map(|option| {
        let id = option.id.clone();
        let on = option.on;
        let filter_fn = filter_fn.clone();
        MenuItem::new(
            SharedString::from(format!("{key}-{id}")),
            option.label.clone(),
        )
        .checked(on)
        .stays_open()
        .on_click(move |_, _, cx| filter_fn(FilterChange::Choice(key, id.clone(), !on), cx))
    });
    let submenu = Menu::new(SharedString::from(format!("{key}-menu")))
        .w(cx.theme().metrics.control * 7.)
        .max_h(cx.theme().metrics.row * 8.)
        .scrollbar(menu.scrollbar)
        .search(menu.search)
        .when(unmatched, |menu| {
            menu.item(
                MenuItem::new(
                    SharedString::from(format!("{key}-none")),
                    i18n::t!("common-no-matches"),
                )
                .disabled(),
            )
        })
        .items(rows);
    MenuItem::new(key, i18n::lookup(key, None))
        .selected(options.iter().any(|option| option.on))
        .submenu(submenu, menu.state)
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
        .ghost()
        .on_click(move |_, _, cx| {
            group.close();
            shift(next, cx);
        })
        .into_any_element()
}

/// Uses the same bounds as the range control, omitting the unchanged end from its summary.
fn compact_range(key: &str, axis: &ui::RangeAxis) -> SharedString {
    let low = axis.unit.say(axis.value.0);
    let high = axis.unit.say(axis.value.1);
    let (from, to) = axis.share();
    let value = match (from > f32::EPSILON, to < 1. - f32::EPSILON) {
        (false, true) => format!("<{high}"),
        (true, false) => format!(">{low}"),
        _ => format!("{low}–{high}"),
    };
    match axis.unit {
        Unit::Clock => value.into(),
        Unit::Plain => format!("{}: {value}", i18n::lookup(key, None)).into(),
    }
}

/// Shared compact symbols for filter flags, with the localized name retained as a tooltip.
fn filter_icon(key: &str) -> Option<&'static str> {
    match key {
        "filter-hide-explicit" => Some("icons/explicit-off.svg"),
        "filter-playable" => Some("icons/circle-check.svg"),
        "filter-favorites" => Some("icons/heart-filled.svg"),
        "filter-collaborations" => Some("icons/users-round-collaboration.svg"),
        "filter-downloaded" => Some("icons/download.svg"),
        "filter-owned" => Some("icons/user-round.svg"),
        _ => None,
    }
}
