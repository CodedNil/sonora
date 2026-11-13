use super::source::{COVER, Field, INDEX, NAME, TRACK_COUNT};
use crate::shared::cells::DATE;
use ui::rank::{HANDY, NICE};
use ui::{ColumnSpec, Width};

pub(super) const COLUMNS: &[ColumnSpec<Field>] = &[
    INDEX,
    COVER,
    NAME.sized(Width::Fill(0.55)),
    ColumnSpec::new(Field::Owner, "owner", "column-owner", Width::Fill(0.45)).ranked(NICE),
    ColumnSpec::new(
        Field::Modified,
        "modified",
        "column-modified",
        Width::Fixed(DATE),
    )
    .ranked(HANDY),
    TRACK_COUNT,
];
