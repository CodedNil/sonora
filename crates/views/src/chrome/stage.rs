//! The parts of the right sidebar that are about the song playing rather than its lyrics or
//! queue: the artwork or Canvas it sits on, the song's name, the frost its sections share, and
//! the sections about its artist and everyone credited on it.

use gpui::prelude::*;
use gpui::{
    AnimationExt as _, AnyElement, App, Div, ElementId, Entity, FontWeight, Interpolate as _,
    MouseButton, Pixels, SharedString, SpringAnimation, SpringConfig, Window, div, px,
};
use music::{SavedArtist, Track};
use router::{Destination, Link as _};
use state::{Origin, Sonora};
use ui::{
    ActiveTheme as _, Artwork, Button, Card, ClipPlayer, Separator, Text, blurring,
    ease_in_out_cubic, eyebrow, snapped,
};

use crate::shared::about::plain_biography;
use crate::shared::cells::{artist_links, count};
use crate::shared::hero::release_date_label;
use crate::shared::menus::{CardMenu, Item};

/// The overlay stays faint at the top and gains contrast as its text scrolls over the video.
const FROST_FILL_TOP: f32 = 0.09;
const FROST_FILL: f32 = 0.18;
/// Blur follows the same scroll progress, keeping the Canvas nearly clear at the top.
const FROST_BLUR_TOP: Pixels = px(3.);
const FROST_BLUR: Pixels = px(24.);
/// Slow contrast changes remain independent of wheel speed.
const FROST_SPRING: SpringConfig = SpringConfig::new(25., 10., 1.);
const FROST_SETTLED: f32 = 0.001;
/// How tall the artist's picture is, in cover heights.
const PORTRAIT: f32 = 1.4;
/// Tint the whole Canvas toward the theme background, darkening or lightening with the theme.
const BACKDROP_TINT: f32 = 0.22;
/// How much of the foreground colour secondary text and headings keep on the frost, where the
/// theme's muted grey sinks into the artwork.
const SOFT: f32 = 0.9;
/// How faint the rule between two sections is.
const RULE: f32 = 0.1;
/// How many of the artist's popular songs and similar artists show.
const POPULAR_SHOWN: usize = 6;
const RELATED_SHOWN: usize = 6;
/// How large a similar artist's picture is, in cover heights.
const RELATED_PICTURE: f32 = 0.5;
/// How many credits show before Show all, and how many lines of the biography.
const CREDITS_SHOWN: usize = 3;
const BIOGRAPHY_LINES: usize = 3;

/// What the sidebar sits on: the song's Canvas clip once it plays, the Canvas still or the
/// album artwork until then.
pub(crate) fn backdrop(
    clip: Option<Entity<ClipPlayer>>,
    still: Option<String>,
    window: &Window,
    cx: &App,
) -> Div {
    let fill = |this: Div| this.absolute().inset_0();

    // The artwork stays under the clip, so a clip still starting up never leaves a gap.
    fill(div())
        .child(
            Artwork::new(still)
                .overlay()
                .corner_radius(cx.theme().radius)
                .size(snapped(window.viewport_size().height, window))
                .absolute()
                .inset_0()
                .size_full(),
        )
        .children(clip.map(|clip| fill(div()).child(clip)))
        .child(
            fill(div())
                .rounded(cx.theme().radius)
                .bg(cx.theme().background.opacity(BACKDROP_TINT)),
        )
}

/// One shared frosted layer, whose contrast follows scroll progress through a GPUI spring.
#[derive(IntoElement)]
struct Frost {
    top: Pixels,
    progress: f32,
    height: Pixels,
    background: Div,
}

/// Frosts only the sidebar background, keeping neighbouring content outside the blur.
pub(crate) fn frost(
    top: Pixels,
    progress: f32,
    height: Pixels,
    background: Div,
) -> impl IntoElement {
    Frost {
        top,
        progress,
        height,
        background,
    }
    .with_spring(
        "aside-frost",
        SpringAnimation::new(FROST_SPRING)
            .to(progress.clamp(0., 1.))
            .from(0.)
            .with_epsilon(FROST_SETTLED),
        |frost, progress| Frost { progress, ..frost },
    )
}

impl RenderOnce for Frost {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let top = self.top;
        let progress = ease_in_out_cubic(self.progress.clamp(0., 1.));
        let fill = f32::interpolate(FROST_FILL_TOP, FROST_FILL, progress);
        let blur = Pixels::interpolate(FROST_BLUR_TOP, FROST_BLUR, progress);
        div()
            .absolute()
            .top(top.max(Pixels::ZERO))
            .bottom_0()
            .left_0()
            .right_0()
            .rounded_b(theme.radius)
            .when(top <= Pixels::ZERO, |this| this.rounded_t(theme.radius))
            .overflow_hidden()
            .when(blurring(cx), |this| {
                this.child(
                    div()
                        .absolute()
                        .top(-top.max(Pixels::ZERO))
                        .left_0()
                        .right_0()
                        .h(self.height)
                        .child(self.background.blur(blur)),
                )
            })
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded_b(theme.radius)
                    .when(top <= Pixels::ZERO, |this| this.rounded_t(theme.radius))
                    .bg(theme.sidebar.alpha(fill)),
            )
    }
}

/// A section of the right sidebar the user can show, hide and move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Section {
    Lyrics,
    Queue,
    About,
    Popular,
    Album,
    Related,
    Credits,
}

impl Section {
    /// Every section, in the order they show until the user moves them.
    pub(crate) const ALL: [Self; 7] = [
        Self::Lyrics,
        Self::Album,
        Self::Queue,
        Self::About,
        Self::Popular,
        Self::Related,
        Self::Credits,
    ];

    pub(crate) fn id(self) -> &'static str {
        match self {
            Self::Lyrics => "lyrics",
            Self::Queue => "queue",
            Self::About => "about",
            Self::Popular => "popular",
            Self::Album => "album",
            Self::Related => "related",
            Self::Credits => "credits",
        }
    }

    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Lyrics => "lyrics-title",
            Self::Queue => "queue-title",
            Self::About => "artist-about",
            Self::Popular => "artist-popular",
            Self::Album => "stage-album",
            Self::Related => "stage-related",
            Self::Credits => "song-credits",
        }
    }

    /// Every section in the order the user set, each with whether it shows. A section the
    /// saved order does not name, as one added since, follows the rest in its own place.
    pub(crate) fn arranged(cx: &App) -> Vec<(Self, bool)> {
        let settings = Sonora::global(cx).settings.read(cx);
        let saved = settings
            .stage_order()
            .iter()
            .filter_map(|id| Self::ALL.into_iter().find(|section| section.id() == id));
        let mut sections: Vec<Self> = Vec::new();
        for section in saved.chain(Self::ALL) {
            if !sections.contains(&section) {
                sections.push(section);
            }
        }
        sections
            .into_iter()
            .map(|section| (section, settings.stage_shown(section.id())))
            .collect()
    }

    /// Moves this section to a gap in the complete order, keeping its visibility.
    pub(crate) fn move_to(self, gap: usize, cx: &mut App) {
        let mut sections = Self::arranged(cx);
        let Some(from) = sections.iter().position(|(section, _)| *section == self) else {
            return;
        };
        let gap = gap.min(sections.len());
        let moved = sections.remove(from);
        sections.insert(if gap > from { gap - 1 } else { gap }, moved);
        Self::save(&sections, cx);
    }

    /// Saves `sections` as the sidebar's order and what it shows.
    pub(crate) fn save(sections: &[(Self, bool)], cx: &mut App) {
        let saved = sections
            .iter()
            .map(|(section, shown)| (section.id().to_owned(), *shown))
            .collect();
        Sonora::global(cx)
            .settings
            .clone()
            .update(cx, |settings, cx| settings.set_stage(saved, cx));
    }
}

/// The dark frost a card filling the sidebar sits on, over the artwork or Canvas still
/// playing behind it.
pub(crate) fn shroud(cx: &App) -> Div {
    let theme = cx.theme();
    let frosted = blurring(cx);

    div()
        .absolute()
        .inset_0()
        .rounded(theme.radius)
        .bg(theme.sidebar.alpha(FROST_FILL))
        .when(frosted, |this| this.backdrop_blur(FROST_BLUR))
}

/// The sections on the frost, each kept apart from the next by a faint rule.
pub(crate) fn sheet(sections: impl IntoIterator<Item = AnyElement>, cx: &App) -> Div {
    let rule = cx.theme().foreground.opacity(RULE);
    let pad = cx.theme().metrics.pad;
    let mut column = div().flex().flex_col().gap_4().px(pad).py(pad * 2.);
    for (index, section) in sections.into_iter().enumerate() {
        if index > 0 {
            column = column.child(Separator::horizontal().mx_2().bg(rule));
        }
        column = column.child(section);
    }
    column
}

/// One section on the frost under its heading, with `action` at the heading's end.
fn section(header: AnyElement, action: Option<AnyElement>) -> Div {
    div()
        .flex()
        .flex_col()
        .flex_none()
        .gap_3()
        .w_full()
        .px_2()
        .child(row(div().child(header), action))
}

/// A section's name, small and in capitals, bright enough to read on the frost.
pub(crate) fn heading(title: SharedString, cx: &App) -> Div {
    eyebrow(title, cx).text_color(cx.theme().foreground.opacity(SOFT))
}

/// The song's name and its artists, large, under the clear stretch of the backdrop.
pub(crate) fn title(track: &Track, cx: &App) -> Div {
    let theme = cx.theme();
    let facts = [
        release(cx),
        Sonora::global(cx).stage.read(cx).plays().map(|plays| {
            let value = count(plays);
            i18n::t!("stage-plays", count = plays, value = &value)
        }),
    ]
    .into_iter()
    .flatten()
    .map(String::from)
    .collect::<Vec<_>>()
    .join(" · ");

    div()
        .relative()
        .flex()
        .flex_col()
        .gap_1()
        .min_w_0()
        .pt(theme.metrics.pad)
        .px_4()
        .child(
            div()
                .on_mouse_down(
                    MouseButton::Right,
                    CardMenu::opener(
                        Item::Track(track.clone()),
                        Sonora::global(cx).playback.clone(),
                    ),
                )
                .text_size(theme.text(Text::Title))
                .font_weight(FontWeight::BOLD)
                .line_clamp(2)
                .child(track.name.clone()),
        )
        .child(
            artist_links(
                "stage-artists",
                track.artist_refs.clone(),
                track.artists.clone(),
                theme.foreground,
            )
            .font_weight(FontWeight::SEMIBOLD)
            .truncate(),
        )
        .when(
            !facts.is_empty()
                || Sonora::global(cx).stage.read(cx).loading("album")
                || Sonora::global(cx).stage.read(cx).loading("play count"),
            |this| {
                this.child(
                    soft(facts.into(), Text::Small, cx)
                        .min_h(theme.text(Text::Small))
                        .truncate(),
                )
            },
        )
}

/// About the artist: their picture, name, monthly listeners and the start of their biography.
/// `None` until the artist has loaded.
pub(crate) fn about(header: AnyElement, window: &Window, cx: &App) -> Option<AnyElement> {
    let theme = cx.theme();
    let (id, artist) = Sonora::global(cx).stage.read(cx).artist()?;
    let listeners = artist.monthly_listeners.map(|listeners| {
        let value = count(listeners);
        i18n::t!(
            "artist-monthly-listeners",
            count = listeners,
            value = &value
        )
    });
    let saved = SavedArtist {
        id: id.to_owned(),
        name: artist.name.clone(),
        cover: artist.cover_large.clone(),
        added_at: None,
    };
    let name = div()
        .id("stage-artist-name")
        .text_size(theme.text(Text::Large))
        .font_weight(FontWeight::BOLD)
        .truncate()
        .cursor_pointer()
        .hover(|style| style.underline())
        .link(Destination::Artist(id.to_owned().into()))
        .on_mouse_down(
            MouseButton::Right,
            CardMenu::opener(
                Item::Artist(saved.clone()),
                Sonora::global(cx).playback.clone(),
            ),
        )
        .child(artist.name.clone());

    Some(
        section(header, None)
            .children(artist.cover_large.clone().map(|cover| {
                // Decoded wide enough for the widest sidebar, and cropped to the picture's box.
                div()
                    .relative()
                    .flex_none()
                    .w_full()
                    .h(snapped(theme.metrics.cover * PORTRAIT, window))
                    .rounded(theme.radius)
                    .overflow_hidden()
                    .on_mouse_down(
                        MouseButton::Right,
                        CardMenu::opener(
                            Item::Artist(saved.clone()),
                            Sonora::global(cx).playback.clone(),
                        ),
                    )
                    .child(
                        Artwork::new(Some(cover))
                            .size(snapped(theme.metrics.cover * 4., window))
                            .corner_radius(theme.radius)
                            .absolute()
                            .inset_0()
                            .size_full(),
                    )
            }))
            .child(row(
                div()
                    .child(name)
                    .children(listeners.map(|listeners| soft(listeners, Text::Body, cx))),
                follow("stage-about-follow", saved, cx),
            ))
            .children(
                (artist.biography.as_deref())
                    .filter(|bio| !bio.is_empty())
                    .map(|bio| {
                        soft(plain_biography(bio), Text::Small, cx).line_clamp(BIOGRAPHY_LINES)
                    }),
            )
            .into_any_element(),
    )
}

/// Everyone credited on the song, a few at a time until Show all. `None` when the song credits
/// no one. `all` says whether every credit shows, and `toggle` flips it.
pub(crate) fn credits(
    header: AnyElement,
    all: bool,
    toggle: impl Fn(&mut Window, &mut App) + 'static,
    cx: &App,
) -> Option<AnyElement> {
    let theme = cx.theme();
    let credits = Sonora::global(cx).stage.read(cx).credits();
    if credits.is_empty() {
        return None;
    }
    let shown = match all {
        true => credits.len(),
        false => CREDITS_SHOWN,
    };
    let rows = credits
        .iter()
        .take(shown)
        .enumerate()
        .map(|(index, credit)| {
            let name = div()
                .id(("stage-credit-name", index))
                .truncate()
                .child(credit.name.clone())
                .when_some(credit.id.clone(), |this, id| {
                    this.cursor_pointer()
                        .hover(|style| style.underline())
                        .link(Destination::Artist(id.clone().into()))
                        .on_mouse_down(
                            MouseButton::Right,
                            CardMenu::opener(
                                Item::Artist(SavedArtist {
                                    id,
                                    name: credit.name.clone(),
                                    cover: None,
                                    added_at: None,
                                }),
                                Sonora::global(cx).playback.clone(),
                            ),
                        )
                });
            let saved = credit.id.clone().map(|id| SavedArtist {
                id,
                name: credit.name.clone(),
                cover: None,
                added_at: None,
            });
            row(
                div().child(name).when(!credit.role.is_empty(), |this| {
                    this.child(soft(credit.role.clone().into(), Text::Small, cx).truncate())
                }),
                saved.and_then(|saved| follow(("stage-credit-follow", index), saved, cx)),
            )
        });

    let all_shown = (credits.len() > CREDITS_SHOWN).then(|| {
        Button::new("stage-credits-all")
            .ghost()
            .small()
            .label(match all {
                true => i18n::t!("artist-popular-less"),
                false => i18n::t!("app-show-all"),
            })
            .tint(theme.foreground.opacity(SOFT))
            .on_click(move |_, window, cx| toggle(window, cx))
            .into_any_element()
    });

    Some(section(header, all_shown).children(rows).into_any_element())
}

/// The artist's most played songs, a few of them, each playing the list from there.
pub(crate) fn popular(
    header: AnyElement,
    width: Pixels,
    window: &Window,
    cx: &App,
) -> Option<AnyElement> {
    let theme = cx.theme();
    let (id, artist) = Sonora::global(cx).stage.read(cx).artist()?;
    if artist.top_tracks.is_empty() {
        return None;
    }
    let tracks: Vec<Track> = artist
        .top_tracks
        .iter()
        .take(POPULAR_SHOWN)
        .cloned()
        .collect();
    let origin = Origin::artist(id).named(artist.name.clone());
    let side = snapped(theme.metrics.cover * RELATED_PICTURE, window);
    let columns = columns(width, side, cx).min(tracks.len());
    let playback = Sonora::global(cx).playback.clone();
    let cards = tracks
        .iter()
        .take(columns)
        .enumerate()
        .map(|(index, track)| {
            let (tracks, origin) = (tracks.clone(), origin.clone());
            crate::shared::cards::track_card(("stage-popular", index), track, &playback, cx)
                .tile(side)
                .size(Text::Small)
                .min_w_0()
                .centered()
                .tint(theme.foreground)
                .when(track.explicit, Card::explicit)
                .press(move |_, _, cx| {
                    let playback = Sonora::global(cx).playback.clone();
                    let (tracks, origin) = (tracks.clone(), origin.clone());
                    playback.update(cx, |playback, cx| {
                        playback.start(tracks, index, Some(origin), cx)
                    });
                })
        });

    Some(
        section(header, None)
            .child(
                div()
                    .grid()
                    .grid_cols(columns as u16)
                    .gap_3()
                    .children(cards),
            )
            .into_any_element(),
    )
}

/// When the song playing came out, in the reader's words, once its album has loaded.
pub(crate) fn release(cx: &App) -> Option<SharedString> {
    let sonora = Sonora::global(cx);
    let album = &sonora.stage.read(cx).album()?.album;
    let track = sonora.playback.read(cx).shown_track(cx)?;
    let playing = track.album_id.as_deref()?;
    (album.id == playing && !album.release_date.is_empty())
        .then(|| release_date_label(&album.release_date))
}

/// The album the song is on: its cover, name and release date, linking to it.
pub(crate) fn album(header: AnyElement, cx: &App) -> Option<AnyElement> {
    let album = &Sonora::global(cx).stage.read(cx).album()?.album;
    let facts = release(cx).map(String::from).unwrap_or_default();
    let to = Destination::Album(album.id.clone().into());

    Some(
        section(header, None)
            .child(
                crate::shared::cards::album_card(
                    "stage-album-card",
                    album,
                    &Sonora::global(cx).playback,
                    cx,
                )
                .meta(facts)
                .px_0()
                .tint(cx.theme().foreground)
                .press(move |_, _, cx| router::navigate(to.clone(), cx)),
            )
            .into_any_element(),
    )
}

/// Artists the song’s listeners also like, in one row with only as many cards as fit.
pub(crate) fn related(
    header: AnyElement,
    width: Pixels,
    window: &Window,
    cx: &App,
) -> Option<AnyElement> {
    let theme = cx.theme();
    let similar = Sonora::global(cx).stage.read(cx).similar();
    if similar.is_empty() {
        return None;
    }
    let side = snapped(theme.metrics.cover * RELATED_PICTURE, window);
    let columns = columns(width, side, cx)
        .min(RELATED_SHOWN)
        .min(similar.len());
    let playback = Sonora::global(cx).playback.clone();
    let artists = similar
        .iter()
        .take(columns)
        .enumerate()
        .map(|(index, artist)| {
            Card::new(("stage-related", index), artist.name.clone())
                .cover(artist.cover.clone())
                .circle()
                .tile(side)
                .centered()
                .flat()
                .size(Text::Small)
                .w_full()
                .min_w_0()
                .hint()
                .press({
                    let id = artist.id.clone();
                    move |_, _, cx| router::navigate(Destination::Artist(id.clone().into()), cx)
                })
                .menu(crate::shared::menus::CardMenu::opener(
                    crate::shared::menus::Item::Artist(artist.clone()),
                    playback.clone(),
                ))
        });
    Some(
        section(header, None)
            .child(
                div()
                    .grid()
                    .grid_cols(columns as u16)
                    .gap_3()
                    .children(artists),
            )
            .into_any_element(),
    )
}

/// Fits compact cards to the measured sidebar content, leaving room for captions and gaps.
fn columns(width: Pixels, art: Pixels, cx: &App) -> usize {
    let pad = cx.theme().metrics.pad;
    ((width - pad * 4.) / (art + pad * 2.)).floor().max(1.) as usize
}

/// A line of text and the button at its end.
fn row(text: Div, button: Option<AnyElement>) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_2()
        .child(text.flex().flex_col().min_w_0())
        .children(button)
}

/// Secondary text at `size`, bright enough to read on the frost.
fn soft(text: SharedString, size: Text, cx: &App) -> Div {
    let theme = cx.theme();
    div()
        .text_size(theme.text(size))
        .text_color(theme.foreground.opacity(SOFT))
        .child(text)
}

/// Follow or Following for an artist, where the provider lets the listener follow artists.
fn follow(id: impl Into<ElementId>, artist: SavedArtist, cx: &App) -> Option<AnyElement> {
    let sonora = Sonora::global(cx);
    if !sonora.session.read(cx).capabilities().follow_artists {
        return None;
    }
    let library = sonora.library.clone();
    let following = library.read(cx).saved_artist(&artist.id);
    let pending = library.read(cx).pending_artist(&artist.id);

    Some(
        Button::new(id)
            .outline()
            .small()
            .rounded_full()
            .label(match following {
                true => i18n::t!("artist-following"),
                false => i18n::t!("artist-follow"),
            })
            .disabled(pending)
            .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation())
            .on_click(move |_, _, cx| {
                library.update(cx, |library, cx| library.toggle_artist(artist.clone(), cx));
            })
            .into_any_element(),
    )
}
