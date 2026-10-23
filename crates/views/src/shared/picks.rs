use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, ClickEvent, Entity, EntityId, Pixels, Point, ScrollHandle, SharedString,
    Window, div,
};
use music::{GenreItem, Track};
use state::Playback;
use ui::{ActiveTheme as _, Button, Card, Glide, heading, snapped, vacant};

use crate::shared::cards;
use crate::shared::track_card::{StartHandler, TrackCard};

const ROWS: usize = 5;
const MAX_COLUMNS: usize = 3;
const MIN_COLUMN_WIDTH: Pixels = gpui::px(280.);

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

pub(crate) fn column_count(width: Pixels) -> usize {
    ((width / MIN_COLUMN_WIDTH).floor().max(1.) as usize).min(MAX_COLUMNS)
}

#[derive(Clone, Copy)]
pub(crate) struct Shape {
    pub(crate) columns: usize,
    pub(crate) pages: usize,
}

impl Shape {
    pub(crate) fn new(width: Pixels, count: usize) -> Self {
        let columns = column_count(width);

        Self {
            columns,
            pages: count.div_ceil(columns * ROWS).max(1),
        }
    }

    fn slots(self) -> usize {
        self.columns * ROWS
    }
}

/// Persistent paging motion, using the same glide as horizontal card rails.
pub(crate) struct PicksMotion {
    scroll: ScrollHandle,
    glide: Glide,
}

impl PicksMotion {
    pub(crate) fn new(host: EntityId) -> Rc<Self> {
        let mut glide = Glide::default();
        glide.watch(host);
        Rc::new(Self {
            scroll: ScrollHandle::new(),
            glide,
        })
    }
}

/// What the deck lists: a run of tracks that queue together, or shelf items of any kind that
/// each stand alone.
pub(crate) enum Rows {
    Tracks(Rc<Vec<Track>>),
    Mixed(Rc<Vec<GenreItem>>),
}

impl From<Rc<Vec<Track>>> for Rows {
    fn from(tracks: Rc<Vec<Track>>) -> Self {
        Self::Tracks(tracks)
    }
}

impl From<Rc<Vec<GenreItem>>> for Rows {
    fn from(items: Rc<Vec<GenreItem>>) -> Self {
        Self::Mixed(items)
    }
}

impl Rows {
    fn len(&self) -> usize {
        match self {
            Self::Tracks(tracks) => tracks.len(),
            Self::Mixed(items) => items.len(),
        }
    }
}

/// A deck of list rows in up to three columns, paged with the arrows in its head.
#[derive(IntoElement)]
pub(crate) struct Picks {
    id: &'static str,
    title: &'static str,
    vacancy: &'static str,
    rows: Rows,
    playback: Entity<Playback>,
    width: Pixels,
    page: usize,
    detailed: bool,
    loading: bool,
    on_previous: Option<ClickHandler>,
    on_next: Option<ClickHandler>,
    on_start: Option<StartHandler>,
    controls: Option<AnyElement>,
    motion: Rc<PicksMotion>,
}

impl Picks {
    /// Track rows queue together; mixed shelf items each open or play independently.
    pub(crate) fn new(
        id: &'static str,
        rows: impl Into<Rows>,
        playback: Entity<Playback>,
        width: Pixels,
        page: usize,
        motion: &Rc<PicksMotion>,
    ) -> Self {
        Self {
            id,
            title: "",
            vacancy: "",
            rows: rows.into(),
            playback,
            width,
            page,
            detailed: false,
            loading: false,
            on_previous: None,
            on_next: None,
            on_start: None,
            controls: None,
            motion: motion.clone(),
        }
    }

    /// Adds section-specific controls alongside the paging buttons.
    pub(crate) fn controls(mut self, controls: impl IntoElement) -> Self {
        self.controls = Some(controls.into_any_element());
        self
    }

    pub(crate) fn title(mut self, key: &'static str) -> Self {
        self.title = key;
        self
    }

    pub(crate) fn vacancy(mut self, key: &'static str) -> Self {
        self.vacancy = key;
        self
    }

    pub(crate) fn detailed(mut self) -> Self {
        self.detailed = true;
        self
    }

    pub(crate) fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub(crate) fn on_previous(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_previous = Some(Rc::new(handler));
        self
    }

    pub(crate) fn on_next(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_next = Some(Rc::new(handler));
        self
    }

    pub(crate) fn on_start(mut self, handler: impl Fn(usize, &mut App) + 'static) -> Self {
        self.on_start = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Picks {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = *cx.theme();
        let shape = Shape::new(self.width, self.rows.len());
        let page = self.page.min(shape.pages.saturating_sub(1));
        let motion = &self.motion;
        motion.glide.sync(&motion.scroll);
        let goal = Point {
            x: -self.width * page as f32,
            y: Pixels::ZERO,
        };
        if (motion.glide.goal(&motion.scroll).x - goal.x).abs() > gpui::px(0.5) {
            motion.glide.aim(&motion.scroll, goal, window);
        }
        let row = snapped(theme.metrics.list_row, window);
        let rows = self.rows;
        let empty = rows.len() == 0;
        let barren = empty && !self.loading;
        let id = self.id;
        let on_previous = self.on_previous;
        let on_next = self.on_next;
        let on_start = self.on_start;

        div()
            .w_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .items_end()
                    .justify_between()
                    .gap_4()
                    .pb_3()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_0()
                            .child(heading(i18n::lookup(self.title, None), cx)),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .pb_1()
                            .children(self.controls)
                            .child(
                                Button::new(SharedString::from(format!("{id}-previous")))
                                    .small()
                                    .outline()
                                    .icon("icons/chevron-left.svg")
                                    .tooltip("common-previous")
                                    .disabled(empty || page == 0)
                                    .when_some(on_previous, |button, handler| {
                                        button.on_click(move |event, window, cx| {
                                            handler(event, window, cx)
                                        })
                                    }),
                            )
                            .child(
                                Button::new(SharedString::from(format!("{id}-next")))
                                    .small()
                                    .outline()
                                    .icon("icons/chevron-right.svg")
                                    .tooltip("common-next")
                                    .disabled(empty || page + 1 >= shape.pages)
                                    .when_some(on_next, |button, handler| {
                                        button.on_click(move |event, window, cx| {
                                            handler(event, window, cx)
                                        })
                                    }),
                            ),
                    ),
            )
            .when(barren, |this| {
                this.child(vacant(i18n::lookup(self.vacancy, None), cx))
            })
            .when(!barren, |this| {
                let viewport = div()
                    .id(SharedString::from(format!("{id}-pages")))
                    .w_full()
                    .overflow_x_hidden()
                    .track_scroll(&self.motion.scroll)
                    .child(div().flex().children((0..shape.pages).map(|page| {
                        let start = page * shape.slots();
                        div().flex().flex_none().w(self.width).gap_4().children(
                            (0..shape.columns).map(|column| {
                                column_shell().children((0..ROWS).map(|slot| {
                                    let place = start + column * ROWS + slot;
                                    match &rows {
                                        _ if empty => {
                                            skeleton(id, column * ROWS + slot).into_any_element()
                                        }
                                        Rows::Tracks(tracks) if place < tracks.len() => {
                                            TrackCard::new(
                                                id,
                                                place,
                                                tracks.clone(),
                                                self.playback.clone(),
                                            )
                                            .detailed(self.detailed)
                                            .start(on_start.clone())
                                            .render(cx)
                                            .into_any_element()
                                        }
                                        Rows::Mixed(items) if place < items.len() => cards::listed(
                                            (id, place),
                                            &items[place],
                                            &self.playback,
                                            cx,
                                        )
                                        .into_any_element(),
                                        _ => div().flex_none().h(row).into_any_element(),
                                    }
                                }))
                            }),
                        )
                    })));
                this.child(viewport)
            })
    }
}

fn column_shell() -> gpui::Div {
    div().flex().flex_1().min_w_0().flex_col().gap_1()
}

fn skeleton(id: &'static str, place: usize) -> impl IntoElement {
    Card::skeleton((id, place))
}

#[cfg(test)]
mod tests {
    use gpui::px;

    use super::{ROWS, Shape, column_count};

    #[test]
    fn columns_follow_width_and_stop_at_three() {
        assert_eq!(column_count(px(279.)), 1);
        assert_eq!(column_count(px(560.)), 2);
        assert_eq!(column_count(px(840.)), 3);
        assert_eq!(column_count(px(2_000.)), 3);
    }

    #[test]
    fn pages_include_every_track() {
        assert_eq!(Shape::new(px(279.), 30).pages, 6);
        assert_eq!(Shape::new(px(560.), 30).pages, 3);
        assert_eq!(Shape::new(px(840.), 30).pages, 2);
        assert_eq!(Shape::new(px(840.), 0).pages, 1);
    }

    #[test]
    fn the_deck_keeps_its_shape_whatever_it_holds() {
        for count in 0..80 {
            for width in [px(279.), px(560.), px(840.)] {
                let shape = Shape::new(width, count);

                assert_eq!(shape.slots(), shape.columns * ROWS);
                assert!(shape.pages * shape.slots() >= count);
                assert!((shape.pages - 1) * shape.slots() < count.max(1));
            }
        }
    }
}
