use gpui::{Pixels, TextAlign, px};
use ui::rank::{HANDY, NICE, SPARE, USEFUL};
use ui::{ColumnSpec, Width};

use self::TrackField as F;
use crate::shared::cells::{ADDED_DATE, DATE, NUMBER};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TrackField {
    Index,
    Cover,
    Title,
    Artists,
    Album,
    Genre,
    AddedAt,
    AddedBy,
    PlayedAt,
    Plays,
    Duration,
    Year,
    Tags,
    Popularity,
}

const LENGTH: Pixels = px(84.);
const CREDITED: Pixels = px(192.);
const CONTRIBUTOR: Pixels = px(168.);
const FILED: Pixels = px(132.);
const PLAYED: Pixels = px(168.);
const ALBUM_LENGTH: Pixels = px(120.);
const ALBUM_PLAYS: Pixels = px(108.);

type Column = ColumnSpec<TrackField>;

const INDEX: Column = Column::numbering(F::Index, NUMBER);
const COVER: Column = Column::artwork(F::Cover);
const TITLE: Column = Column::new(F::Title, "title", "column-title", Width::Fill(0.515));
const ARTISTS: Column =
    Column::new(F::Artists, "artists", "column-artist", Width::Fill(0.212)).ranked(HANDY);
const ALBUM: Column =
    Column::new(F::Album, "album", "column-album", Width::Fill(0.273)).ranked(SPARE);
/// Genres sit beside the album and stay hidden until the listener enables them.
const GENRE: Column = Column::new(F::Genre, "genre", "column-genre", Width::Fixed(FILED))
    .ranked(SPARE)
    .optional();
const ADDED_AT: Column = Column::new(
    F::AddedAt,
    "added-at",
    "column-date-added",
    Width::Fixed(ADDED_DATE),
)
.ranked(NICE)
.descending();
const ADDED_BY: Column = Column::new(
    F::AddedBy,
    "added-by",
    "column-added-by",
    Width::Fixed(CONTRIBUTOR),
)
.ranked(NICE);
const PLAYS: Column = Column::new(F::Plays, "plays", "column-plays", Width::Fixed(DATE))
    .ranked(NICE)
    .aligned(TextAlign::Right);
const DURATION: Column = Column::new(
    F::Duration,
    "duration",
    "column-length",
    Width::Fixed(LENGTH),
)
.ranked(USEFUL)
.aligned(TextAlign::Right);
const PLAYED_AT: Column = Column::new(
    F::PlayedAt,
    "played-at",
    "column-played-at",
    Width::Fixed(PLAYED),
)
.ranked(NICE);
const YEAR: Column = Column::new(
    F::Year,
    "year",
    "column-year",
    Width::Fixed(crate::shared::cells::YEAR),
)
.ranked(SPARE)
.aligned(TextAlign::Right)
.optional();
const TAGS: Column = Column::new(F::Tags, "tags", "column-tags", Width::Fixed(CREDITED))
    .ranked(SPARE)
    .optional();
const POPULARITY: Column = Column::new(
    F::Popularity,
    "popularity",
    "column-popularity",
    Width::Fixed(LENGTH),
)
.ranked(SPARE)
.aligned(TextAlign::Right)
.optional();

pub(crate) const LIBRARY_COLUMNS: &[Column] = &listing(ADDED_AT);
pub(crate) const HISTORY_COLUMNS: &[Column] = &listing(PLAYED_AT);

const PLAYLIST_COLUMNS: &[Column] = &playlist(ADDED_AT, ADDED_BY.optional());
const PLAYLIST_COLUMNS_SHARED: &[Column] =
    &playlist(ADDED_AT.sized(Width::Fixed(CREDITED)), ADDED_BY.optional());
const PLAYLIST_COLUMNS_BLEND: &[Column] = &playlist(ADDED_BY, ADDED_AT.optional());
const ARTIST_COLUMNS: &[Column] = &artist(PLAYS);
const ARTIST_COLUMNS_LEAN: &[Column] = &artist(PLAYS.optional());
const ALBUM_COLUMNS: &[Column] = &album(true);
const ALBUM_COLUMNS_LEAN: &[Column] = &album(false);

pub(crate) fn playlist_columns(blend: bool, shared: bool) -> &'static [Column] {
    match (blend, shared) {
        (true, _) => PLAYLIST_COLUMNS_BLEND,
        (false, true) => PLAYLIST_COLUMNS_SHARED,
        (false, false) => PLAYLIST_COLUMNS,
    }
}

pub(crate) fn artist_columns(playcounts: bool) -> &'static [Column] {
    match playcounts {
        true => ARTIST_COLUMNS,
        false => ARTIST_COLUMNS_LEAN,
    }
}

pub(crate) fn album_columns(playcounts: bool) -> &'static [Column] {
    match playcounts {
        true => ALBUM_COLUMNS,
        false => ALBUM_COLUMNS_LEAN,
    }
}

/// The shared library/history schema, differing only in the date represented.
const fn listing(date: Column) -> [Column; 12] {
    [
        INDEX,
        COVER,
        TITLE,
        ARTISTS,
        ALBUM,
        GENRE,
        date,
        PLAYS.optional(),
        DURATION,
        YEAR,
        TAGS,
        POPULARITY,
    ]
}

/// Playlist provenance varies with collaborative and blended lists; the rest stays identical.
const fn playlist(first: Column, second: Column) -> [Column; 13] {
    [
        INDEX,
        COVER,
        TITLE,
        ARTISTS,
        ALBUM,
        GENRE,
        first,
        second,
        PLAYS.optional(),
        DURATION,
        YEAR,
        TAGS,
        POPULARITY,
    ]
}

/// Artist pages show play counts by default only when their provider reports them.
const fn artist(plays: Column) -> [Column; 10] {
    [
        INDEX,
        COVER,
        TITLE,
        ARTISTS.ranked(SPARE),
        ALBUM.ranked(HANDY),
        plays,
        DURATION,
        YEAR,
        TAGS,
        POPULARITY,
    ]
}

/// Album lists keep their wider title, artist and duration cells and provider-specific count order.
const fn album(playcounts: bool) -> [Column; 10] {
    let middle = match playcounts {
        true => [PLAYS.sized(Width::Fixed(ALBUM_PLAYS)), ALBUM.optional()],
        false => [ALBUM.optional(), PLAYS.optional()],
    };
    let [first, second] = middle;
    [
        INDEX,
        COVER.optional(),
        TITLE.sized(Width::Fill(0.665)),
        ARTISTS.sized(Width::Fill(0.335)),
        first,
        second,
        DURATION.sized(Width::Fixed(ALBUM_LENGTH)),
        YEAR,
        TAGS,
        POPULARITY,
    ]
}
