use super::source::Field;
use crate::shared::cells::{DATE, NUMBER, TRAILING};
use gpui::TextAlign;
use ui::rank::{ESSENTIAL, HANDY, NICE, SPARE};
use ui::{ColumnSpec, Width};

const COLUMN: ColumnSpec<Field> = ColumnSpec::filling(Field::Index);

const INDEX: ColumnSpec<Field> = ColumnSpec::numbering(Field::Index, NUMBER);

const COVER: ColumnSpec<Field> = ColumnSpec::artwork(Field::Cover);

const NAME: ColumnSpec<Field> = ColumnSpec {
    field: Field::Name,
    key: "name",
    header: "column-name",
    width: Width::Fill(0.55),
    rank: ESSENTIAL,
    ..COLUMN
};

const OWNER: ColumnSpec<Field> = ColumnSpec {
    field: Field::Owner,
    key: "owner",
    header: "column-owner",
    width: Width::Fill(0.45),
    rank: NICE,
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

const MODIFIED: ColumnSpec<Field> = ColumnSpec {
    field: Field::Modified,
    key: "modified",
    header: "column-modified",
    width: Width::Fixed(DATE),
    rank: HANDY,
    ..COLUMN
};

pub(super) const COLUMNS: &[ColumnSpec<Field>] =
    &[INDEX, COVER, NAME, OWNER, MODIFIED, TRACK_COUNT];
