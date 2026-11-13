use super::source::{ADDED_AT, COVER, Field, INDEX, TRACK_COUNT};
use crate::shared::cells::YEAR;
use crate::shared::text::holds;
use gpui::TextAlign;
use music::Album;
use std::io::Write as _;
use ui::rank::{HANDY, USEFUL};
use ui::{ColumnSpec, Width};

pub(super) const COLUMNS: &[ColumnSpec<Field>] = &[
    INDEX,
    COVER,
    ColumnSpec::new(Field::Name, "name", "column-album", Width::Fill(0.55)),
    ColumnSpec::new(
        Field::Artists,
        "artists",
        "column-artist",
        Width::Fill(0.45),
    )
    .ranked(USEFUL),
    ColumnSpec::new(Field::Year, "year", "column-year", Width::Fixed(YEAR))
        .ranked(HANDY)
        .aligned(TextAlign::Right),
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
