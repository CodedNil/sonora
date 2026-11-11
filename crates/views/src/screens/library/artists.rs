use super::source::Field;
use crate::shared::cells::{DATE, NUMBER};
use ui::rank::{ESSENTIAL, HANDY};
use ui::{ColumnSpec, Sort, Width};

const COLUMN: ColumnSpec<Field> = ColumnSpec::filling(Field::Index);

const INDEX: ColumnSpec<Field> = ColumnSpec::numbering(Field::Index, NUMBER);

const COVER: ColumnSpec<Field> = ColumnSpec::artwork(Field::Cover);

const NAME: ColumnSpec<Field> = ColumnSpec {
    field: Field::Name,
    key: "name",
    header: "column-name",
    rank: ESSENTIAL,
    ..COLUMN
};

const ADDED_AT: ColumnSpec<Field> = ColumnSpec {
    field: Field::AddedAt,
    key: "added-at",
    header: "column-date-added",
    width: Width::Fixed(DATE),
    rank: HANDY,
    first: Sort::Descending,
    ..COLUMN
};

pub(super) const COLUMNS: &[ColumnSpec<Field>] = &[INDEX, COVER, NAME, ADDED_AT];
