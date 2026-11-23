use super::source::{ADDED_AT, COVER, Field, INDEX, NAME};
use ui::ColumnSpec;

pub(super) const COLUMNS: &[ColumnSpec<Field>] =
    &[INDEX, COVER, NAME, ADDED_AT.ranked(ui::rank::HANDY)];
