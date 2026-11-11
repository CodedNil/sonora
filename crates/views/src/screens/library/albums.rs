use super::source::Field;
use crate::shared::cells::{DATE, NUMBER, TRAILING, YEAR};
use crate::shared::text::holds;
use gpui::TextAlign;
use music::Album;
use std::io::Write as _;
use ui::rank::{HANDY, NICE, SPARE, USEFUL};
use ui::{ColumnSpec, Sort, Width};

const COLUMN: ColumnSpec<Field> = ColumnSpec::filling(Field::Index);

const INDEX: ColumnSpec<Field> = ColumnSpec::numbering(Field::Index, NUMBER);

const COVER: ColumnSpec<Field> = ColumnSpec::artwork(Field::Cover);

const NAME: ColumnSpec<Field> = ColumnSpec {
    field: Field::Name,
    key: "name",
    header: "column-album",
    width: Width::Fill(0.55),
    ..COLUMN
};

const ARTISTS: ColumnSpec<Field> = ColumnSpec {
    field: Field::Artists,
    key: "artists",
    header: "column-artist",
    width: Width::Fill(0.45),
    rank: USEFUL,
    ..COLUMN
};

const RELEASE_YEAR: ColumnSpec<Field> = ColumnSpec {
    field: Field::Year,
    key: "year",
    header: "column-year",
    align: TextAlign::Right,
    width: Width::Fixed(YEAR),
    rank: HANDY,
    ..COLUMN
};

const TRACK_COUNT: ColumnSpec<Field> = ColumnSpec {
    field: Field::TrackCount,
    key: "tracks",
    header: "column-tracks",
    align: TextAlign::Right,
    width: Width::Fixed(TRAILING),
    rank: SPARE,
    ..COLUMN
};

const ADDED_AT: ColumnSpec<Field> = ColumnSpec {
    field: Field::AddedAt,
    key: "added-at",
    header: "column-date-added",
    width: Width::Fixed(DATE),
    rank: NICE,
    first: Sort::Descending,
    ..COLUMN
};

pub(super) const COLUMNS: &[ColumnSpec<Field>] = &[
    INDEX,
    COVER,
    NAME,
    ARTISTS,
    RELEASE_YEAR,
    ADDED_AT,
    TRACK_COUNT,
];

pub(super) fn hits(album: &Album, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    holds(&album.name, query) || holds(&album.artists, query) || year_holds(album, query)
}

/// Whether the album's year, as `year` prints it, holds the query. The digits are written on the
/// stack, since a filter asks this of every album on every keystroke.
fn year_holds(album: &Album, query: &str) -> bool {
    let Some(year) = album.year() else {
        return query.is_empty();
    };
    let mut digits = [0u8; 12];
    let mut cursor = std::io::Cursor::new(&mut digits[..]);
    if write!(cursor, "{year}").is_err() {
        return false;
    }
    let written = cursor.position() as usize;
    std::str::from_utf8(&digits[..written]).is_ok_and(|text| holds(text, query))
}
