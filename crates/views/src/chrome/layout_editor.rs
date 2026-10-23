use gpui::prelude::*;
use gpui::{
    App, Context, DragMoveEvent, Entity, EntityId, Global, Pixels, Point, Render, ScrollHandle,
    SharedString, Window, div, svg,
};
use router::{NavEntry, Place};
use state::{AppSettings, Sonora};
use ui::{ActiveTheme as _, Edge, Ghost, Menu, MenuItem, Popup, Scrollbar, Text, drop_marker};

const DRAGGED_OPACITY: f32 = 0.4;

struct Installed(Entity<LayoutEditor>);
impl Global for Installed {}

/// The shared layout editor floats above both the toolbar and sidebar, including when closed.
pub(crate) struct LayoutEditor {
    settings: Entity<AppSettings>,
    nav_drag: Option<NavEntry>,
    nav_gap: Option<(Place, usize)>,
    nav_menu: Option<Point<Pixels>>,
    nav_motion: ui::ReorderMotion,
    scrollbar: Entity<Scrollbar>,
}

/// One navigation entry being reordered in the left-sidebar menu.
#[derive(Clone)]
struct NavDrag {
    owner: EntityId,
    entry: NavEntry,
    label: SharedString,
    position: Point<Pixels>,
}

impl Render for NavDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Ghost::new(self.position, self.label.clone())
    }
}

impl LayoutEditor {
    /// Returns the single editor shared by the workspace's chrome surfaces.
    pub(crate) fn entity(cx: &mut App) -> Entity<Self> {
        if cx.try_global::<Installed>().is_none() {
            let editor = cx.new(|cx| {
                let settings = Sonora::global(cx).settings.clone();
                cx.observe(&settings, |_, _, cx| cx.notify()).detach();
                let me = cx.entity_id();
                Self {
                    settings,
                    nav_drag: None,
                    nav_gap: None,
                    nav_menu: None,
                    nav_motion: ui::ReorderMotion::default(),
                    scrollbar: cx.new(|_| Scrollbar::new(ScrollHandle::new()).watching(me)),
                }
            });
            cx.set_global(Installed(editor));
        }
        cx.global::<Installed>().0.clone()
    }

    /// Opens the same editor from either chrome surface.
    pub(crate) fn open(position: Point<Pixels>, cx: &mut App) {
        Self::entity(cx).update(cx, |this, cx| {
            this.nav_menu = Some(position);
            this.nav_gap = None;
            this.nav_drag = None;
            this.nav_motion = ui::ReorderMotion::default();
            this.scrollbar
                .read(cx)
                .scroll()
                .set_offset(Point::default());
            cx.notify();
        });
    }

    /// The context menu that groups navigation entries by where they appear and lets them be
    /// dragged between those groups.
    fn arrangement_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let at = self.nav_menu?;
        let theme = *cx.theme();
        let row = theme.metrics.control;
        let gap = theme.metrics.pad / 2.;
        let step = row + gap;
        let entries = super::ordered_navigation(cx);
        let places = Place::ALL;
        let targets = places.iter().flat_map(|&place| {
            let grouped = entries
                .iter()
                .copied()
                .filter(|entry| super::place(*entry, cx) == place)
                .collect::<Vec<_>>();
            let from = self.nav_drag.and_then(|dragged| {
                (super::place(dragged, cx) == place)
                    .then(|| grouped.iter().position(|entry| *entry == dragged))
                    .flatten()
            });
            let gap = self
                .nav_gap
                .filter(|(target, _)| *target == place)
                .map(|(_, gap)| gap);
            grouped.into_iter().enumerate().map(move |(index, entry)| {
                let target = match (from, gap) {
                    (Some(from), Some(gap)) => ui::reorder_index(index, Some(from), Some(gap)),
                    (None, Some(gap)) if index >= gap => index + 1,
                    _ => index,
                };
                (entry.id().into(), step * target as f32, false)
            })
        });
        self.nav_motion.animate(targets, window, cx);

        let items = places
            .into_iter()
            .enumerate()
            .map(|(section_index, place)| {
                let grouped = entries
                    .iter()
                    .copied()
                    .filter(|entry| super::place(*entry, cx) == place)
                    .collect::<Vec<_>>();
                let count = grouped.len();
                let active_gap = self
                    .nav_gap
                    .filter(|(target, _)| *target == place)
                    .map(|(_, gap)| gap);
                let rows = grouped
                    .iter()
                    .enumerate()
                    .map(|(index, &entry)| {
                        let offset = self.nav_motion.position(entry.id(), step * index as f32)
                            - step * index as f32;
                        let edge = match active_gap {
                            Some(gap) if gap == index => Some(Edge::Above),
                            Some(gap) if gap == count && index + 1 == count => Some(Edge::Below),
                            _ => None,
                        };
                        let label = i18n::lookup(entry.key(), None);
                        let dragged = NavDrag {
                            owner: cx.entity_id(),
                            entry,
                            label,
                            position: Point::default(),
                        };
                        let me = cx.weak_entity();
                        div()
                            .id(("nav-edit-row", entry as usize))
                            .relative()
                            .h(row)
                            .w_full()
                            .cursor_grab()
                            .on_drag(dragged, move |dragged, position, _, cx| {
                                me.update(cx, |this, cx| {
                                    this.nav_drag = Some(dragged.entry);
                                    this.nav_gap = None;
                                    this.nav_motion = ui::ReorderMotion::default();
                                    cx.notify();
                                })
                                .ok();
                                cx.new(|_| NavDrag {
                                    position,
                                    ..dragged.clone()
                                })
                            })
                            .child(
                                ui::menu_row(cx)
                                    .h(row)
                                    .w_full()
                                    .relative()
                                    .top(offset)
                                    .when(self.nav_drag == Some(entry), |row| {
                                        row.opacity(DRAGGED_OPACITY)
                                    })
                                    .child(ui::grip(cx))
                                    .child(
                                        svg()
                                            .path(icons::path(entry.icon()))
                                            .flex_none()
                                            .size(theme.text(Text::Body))
                                            .text_color(theme.muted_foreground),
                                    )
                                    .child(div().flex_1().min_w_0().truncate().child(
                                        ui::menu_label(i18n::lookup(entry.key(), None), cx),
                                    )),
                            )
                            .on_drop(cx.listener(|this, dragged: &NavDrag, _, cx| {
                                this.drop_navigation(dragged, cx)
                            }))
                            .when_some(edge, |line, edge| line.child(drop_marker(edge, cx)))
                            .into_any_element()
                    })
                    .collect::<Vec<_>>();
                let mut body = div()
                    .id(("nav-edit-place", section_index))
                    .relative()
                    .flex()
                    .flex_col()
                    .gap(gap)
                    .min_h(row)
                    // Only an incoming entry needs another row; moves within this group
                    // already fit, and the spare space follows the current destination.
                    .when(
                        active_gap.is_some()
                            && self
                                .nav_drag
                                .is_some_and(|entry| super::place(entry, cx) != place),
                        |body| body.pb(step),
                    )
                    .children(rows);
                if count == 0 && active_gap == Some(0) {
                    body = body.child(drop_marker(Edge::Above, cx));
                }
                let list = body
                    .on_drag_move(cx.listener(
                        move |this, event: &DragMoveEvent<NavDrag>, _, cx| {
                            if event.bounds.contains(&event.event.position) {
                                let dragged = event.drag(cx).clone();
                                if dragged.owner == cx.entity_id() {
                                    let relative = (event.event.position.y - event.bounds.top())
                                        .max(Pixels::ZERO);
                                    let gap = ((relative + row / 2.) / step).floor() as usize;
                                    this.preview_navigation(&dragged, place, gap.min(count), cx);
                                }
                            }
                        },
                    ))
                    .on_drop(cx.listener(|this, dragged: &NavDrag, _, cx| {
                        this.drop_navigation(dragged, cx)
                    }));
                let heading = div()
                    .id(("nav-edit-heading", section_index))
                    .flex()
                    .items_center()
                    .h(theme.metrics.control_small)
                    .child(ui::menu_heading(i18n::lookup(place.key(), None), cx))
                    .on_drag_move(cx.listener(
                        move |this, event: &DragMoveEvent<NavDrag>, _, cx| {
                            if event.bounds.contains(&event.event.position) {
                                let dragged = event.drag(cx).clone();
                                if dragged.owner == cx.entity_id() {
                                    this.preview_navigation(&dragged, place, count, cx);
                                }
                            }
                        },
                    ))
                    .on_drop(cx.listener(|this, dragged: &NavDrag, _, cx| {
                        this.drop_navigation(dragged, cx)
                    }));
                let content = div()
                    .flex()
                    .flex_col()
                    .gap(gap)
                    .child(heading)
                    .child(list)
                    .when(section_index > 0, |this| {
                        this.border_t_1().border_color(theme.border).pt(gap)
                    });
                MenuItem::new(("nav-edit-place", section_index), place.id()).content(content)
            });

        Some(
            Popup::new(
                at,
                Menu::new("sidebar-navigation-menu")
                    .items(items)
                    .scrollbar(self.scrollbar.clone())
                    .max_h(window.viewport_size().height - theme.metrics.title_bar * 2.),
            )
            .on_close(cx.listener(|this, _, _, cx| {
                this.nav_menu = None;
                this.nav_gap = None;
                cx.notify();
            })),
        )
    }

    /// Marks a valid destination gap and previews how the grouped rows will shift.
    fn preview_navigation(
        &mut self,
        dragged: &NavDrag,
        place: Place,
        gap: usize,
        cx: &mut Context<Self>,
    ) {
        if !dragged.entry.places().contains(&place) {
            if self.nav_gap.take().is_some() {
                cx.notify();
            }
            return;
        }
        let from = (super::place(dragged.entry, cx) == place)
            .then(|| {
                super::ordered_navigation(cx)
                    .into_iter()
                    .filter(|entry| super::place(*entry, cx) == place)
                    .position(|entry| entry == dragged.entry)
            })
            .flatten();
        let gap = match from {
            Some(from) if gap == from || gap == from + 1 => None,
            _ => Some((place, gap)),
        };
        if self.nav_gap != gap {
            self.nav_gap = gap;
            cx.notify();
        }
    }

    /// Saves a drop in its destination group and keeps each group's local order.
    fn drop_navigation(&mut self, dragged: &NavDrag, cx: &mut Context<Self>) {
        if dragged.owner == cx.entity_id()
            && let Some((place, gap)) = self.nav_gap.take()
            && dragged.entry.places().contains(&place)
        {
            let from = super::ordered_navigation(cx)
                .into_iter()
                .filter(|entry| super::place(*entry, cx) == place)
                .position(|entry| entry == dragged.entry);
            let gap = gap.saturating_sub(usize::from(from.is_some_and(|from| from < gap)));
            let mut groups: [Vec<NavEntry>; 5] = std::array::from_fn(|_| Vec::new());
            for entry in super::ordered_navigation(cx) {
                if entry != dragged.entry {
                    groups[nav_place_index(super::place(entry, cx))].push(entry);
                }
            }
            let target = &mut groups[nav_place_index(place)];
            target.insert(gap.min(target.len()), dragged.entry);
            let order = groups
                .into_iter()
                .flatten()
                .map(|entry| entry.id().to_owned())
                .collect();
            self.settings.update(cx, |settings, cx| {
                settings.set_nav_place(dragged.entry.id(), place.id(), cx);
                settings.set_nav_order(order, cx);
            });
        }
        self.nav_drag = None;
        self.nav_gap = None;
        self.nav_motion = ui::ReorderMotion::default();
        cx.notify();
    }
}

impl Render for LayoutEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !cx.has_active_drag() {
            self.nav_drag = None;
            self.nav_gap = None;
        }
        div().children(self.arrangement_menu(window, cx))
    }
}

/// The persistence group order matches the editor display order.
fn nav_place_index(place: Place) -> usize {
    Place::ALL
        .iter()
        .position(|held| *held == place)
        .unwrap_or_default()
}
