use std::{cell::RefCell, cmp::Ordering, collections::HashSet};

use gpui::{AnyElement, App, Entity, SharedString};
use i18n::t;
use music::{Album, Playlist, SavedArtist};
use state::{Collection, Library, Network, Origin, Playback, Shelf, Sonora};
use ui::{
    ActiveTheme as _, Cell, ColumnSpec, Filter, FilterChange, Menu, Pin, TableSource, Unit, Width,
};

use super::{LibraryFilters, Section, albums, artists, playlists};
use crate::chrome::tools;
use crate::shared::{
    cells,
    menus::{ItemMenu, album_menu, artist_menu, playlist_menu},
    pins::Pinned as _,
    text::{folded, holds},
    tracks::initial,
};

/// Fields shared by the album, artist and playlist shelves; each shelf offers its own columns.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Field {
    Index,
    Cover,
    Name,
    Artists,
    Year,
    TrackCount,
    AddedAt,
    Owner,
    Modified,
}

pub(super) const INDEX: ColumnSpec<Field> = ColumnSpec::numbering(Field::Index, cells::NUMBER);
pub(super) const COVER: ColumnSpec<Field> = ColumnSpec::artwork(Field::Cover);
pub(super) const NAME: ColumnSpec<Field> =
    ColumnSpec::new(Field::Name, "name", "column-name", Width::Fill(1.));
pub(super) const ADDED_AT: ColumnSpec<Field> = ColumnSpec::new(
    Field::AddedAt,
    "added-at",
    "column-date-added",
    Width::Fixed(cells::DATE),
)
.ranked(ui::rank::NICE)
.descending();
pub(super) const TRACK_COUNT: ColumnSpec<Field> = ColumnSpec::new(
    Field::TrackCount,
    "tracks",
    "column-tracks",
    Width::Fixed(cells::TRAILING),
)
.ranked(ui::rank::SPARE)
.aligned(gpui::TextAlign::Right);

/// A borrowed library row, keeping provider models intact for menus and navigation.
#[derive(Clone, Copy)]
enum Row<'a> {
    Album(&'a Album),
    Artist(&'a SavedArtist),
    Playlist(&'a Playlist),
}

/// Shared table behavior for library collections, with independent state for each shelf.
pub(super) struct Source {
    section: Section,
    library: Entity<Library>,
    playback: Entity<Playback>,
    shelf: Shelf,
    menu: Option<ItemMenu>,
    filters: LibraryFilters,
    years: RefCell<Option<Vec<f32>>>,
    heard: RefCell<Option<HashSet<String>>>,
}

impl<'a> Row<'a> {
    fn id(self) -> &'a str {
        match self {
            Self::Album(row) => &row.id,
            Self::Artist(row) => &row.id,
            Self::Playlist(row) => &row.id,
        }
    }

    fn text(self, field: Field) -> &'a str {
        match (self, field) {
            (Self::Album(row), Field::Name) => &row.name,
            (Self::Artist(row), Field::Name) => &row.name,
            (Self::Playlist(row), Field::Name) => &row.name,
            (Self::Album(row), Field::Artists) => &row.artists,
            (Self::Playlist(row), Field::Owner) => &row.owner,
            _ => "",
        }
    }

    fn number(self, field: Field) -> Option<i64> {
        match (self, field) {
            (Self::Album(row), Field::Year) => row.year().map(i64::from),
            (Self::Album(row), Field::TrackCount) => row.track_count.map(i64::from),
            (Self::Playlist(row), Field::TrackCount) => row.track_count.map(i64::from),
            _ => None,
        }
    }

    fn origin(self) -> Origin {
        match self {
            Self::Album(row) => Origin::album(row.id.clone()),
            Self::Artist(row) => Origin::artist(row.id.clone()),
            Self::Playlist(row) => Origin::playlist(row.id.clone()),
        }
    }

    fn pin(self) -> Option<Pin> {
        match self {
            Self::Album(row) => row.pin(),
            Self::Artist(row) => row.pin(),
            Self::Playlist(row) => row.pin(),
        }
    }
    fn timestamp(self, field: Field) -> Option<music::Timestamp> {
        match (self, field) {
            (Self::Album(row), Field::AddedAt) => row.added_at,
            (Self::Artist(row), Field::AddedAt) => row.added_at,
            (Self::Playlist(row), Field::Modified) => row.modified_at,
            _ => None,
        }
    }
}

impl Source {
    /// Builds a collection table. Songs use `TrackSource`; albums require their playlist menu.
    pub(super) fn new(
        section: Section,
        library: Entity<Library>,
        playback: Entity<Playback>,
        shelf: Shelf,
        menu: Option<ItemMenu>,
    ) -> Self {
        Self {
            section,
            library,
            playback,
            shelf,
            menu,
            filters: LibraryFilters::default(),
            years: RefCell::default(),
            heard: RefCell::default(),
        }
    }

    pub(super) fn album(&self, row: usize, cx: &App) -> Option<Album> {
        match self.row(row, cx)? {
            Row::Album(row) => Some(row.clone()),
            _ => None,
        }
    }

    pub(super) fn artist(&self, row: usize, cx: &App) -> Option<SavedArtist> {
        match self.row(row, cx)? {
            Row::Artist(row) => Some(row.clone()),
            _ => None,
        }
    }

    pub(super) fn playlist(&self, row: usize, cx: &App) -> Option<Playlist> {
        match self.row(row, cx)? {
            Row::Playlist(row) => Some(row.clone()),
            _ => None,
        }
    }

    fn row<'a>(&self, row: usize, cx: &'a App) -> Option<Row<'a>> {
        let library = self.library.read(cx).state(self.shelf);
        match self.section {
            Section::Albums => library.albums().get(row).map(Row::Album),
            Section::Artists => library.artists().get(row).map(Row::Artist),
            Section::Playlists => library.playlists().get(row).map(Row::Playlist),
            Section::Songs => None,
        }
    }

    /// Whether an entire collection is downloaded or queued; artists also include credited music.
    fn downloaded(&self, row: Row<'_>, cx: &App) -> bool {
        let downloads = Sonora::global(cx).downloads.read(cx);
        let collection = match row {
            Row::Album(_) => Collection::Album,
            Row::Playlist(_) => Collection::Playlist,
            Row::Artist(_) => {
                return self
                    .heard
                    .borrow_mut()
                    .get_or_insert_with(|| {
                        let library = self.library.read(cx).state(self.shelf);
                        let kept = library
                            .tracks()
                            .iter()
                            .filter(|track| track.id.as_deref().is_some_and(|id| downloads.has(id)))
                            .flat_map(|track| &track.artist_refs)
                            .filter_map(|artist| artist.id.as_deref());
                        downloads
                            .credited()
                            .chain(kept)
                            .map(str::to_owned)
                            .collect()
                    })
                    .contains(row.id());
            }
        };
        downloads.set(collection, row.id()).is_some()
    }
}

impl TableSource for Source {
    type Field = Field;

    fn invalidate(&mut self) {
        *self.years.get_mut() = None;
        *self.heard.get_mut() = None;
    }

    fn columns(&self) -> &'static [ColumnSpec<Field>] {
        match self.section {
            Section::Albums => albums::COLUMNS,
            Section::Artists => artists::COLUMNS,
            Section::Playlists => playlists::COLUMNS,
            Section::Songs => &[],
        }
    }

    fn rows(&self, cx: &App) -> usize {
        let library = self.library.read(cx).state(self.shelf);
        match self.section {
            Section::Albums => library.albums().len(),
            Section::Artists => library.artists().len(),
            Section::Playlists => library.playlists().len(),
            Section::Songs => 0,
        }
    }

    fn matches(&self, row: usize, query: &str, cx: &App) -> bool {
        let Some(row) = self.row(row, cx) else {
            return false;
        };
        if self.filters.on("filter-favorites") {
            let library = self.library.read(cx);
            let saved = match row {
                Row::Album(row) => library.saved_album(&row.id),
                Row::Artist(row) => library.saved_artist(&row.id),
                Row::Playlist(_) => true,
            };
            if !saved {
                return false;
            }
        }
        if (self.filters.on("filter-downloaded") || Network::downloads_only(self.shelf, cx))
            && !self.downloaded(row, cx)
        {
            return false;
        }
        match row {
            Row::Album(album) => {
                self.filters.range("filter-year").is_none_or(|(low, high)| {
                    album.year().is_some_and(|year| {
                        f32::from(year) >= low - 0.5 && f32::from(year) <= high + 0.5
                    })
                }) && albums::hits(album, query)
            }
            Row::Artist(artist) => holds(&artist.name, query),
            Row::Playlist(playlist) => {
                (!self.filters.on("filter-owned") || playlist.owned)
                    && (holds(&playlist.name, query) || holds(&playlist.owner, query))
            }
        }
    }

    fn filter_axes(&self, cx: &App) -> Vec<Filter> {
        let mut axes = Vec::new();
        if self.section == Section::Albums {
            let mut years = self.years.borrow_mut();
            let years = years.get_or_insert_with(|| {
                let mut years: Vec<_> = self
                    .library
                    .read(cx)
                    .state(self.shelf)
                    .albums()
                    .iter()
                    .filter_map(|album| album.year().map(f32::from))
                    .collect();
                years.sort_by(f32::total_cmp);
                years.dedup();
                years
            });
            axes.extend(tools::range(
                "filter-year",
                years,
                self.filters.range("filter-year"),
                Unit::Plain,
            ));
        }
        let offered = self.filters.on("filter-downloaded")
            || !Sonora::global(cx).downloads.read(cx).is_empty();
        axes.extend(self.filters.flags([
            ("filter-owned", self.section == Section::Playlists),
            (
                "filter-favorites",
                self.section != Section::Playlists
                    && self.library.read(cx).shape(self.shelf).is_catalog(),
            ),
            (
                "filter-downloaded",
                self.section != Section::Artists
                    && offered
                    && !Network::downloads_only(self.shelf, cx),
            ),
        ]));
        axes
    }

    fn filter(&mut self, change: FilterChange, _cx: &App) -> bool {
        self.filters.apply(change)
    }
    fn filtered(&self, _cx: &App) -> bool {
        self.filters.active()
    }
    fn picking(&self) -> bool {
        true
    }

    fn playing(&self, row: usize, cx: &App) -> bool {
        self.row(row, cx).is_some_and(|row| {
            self.playback
                .read(cx)
                .playing_from(&row.origin(), cx)
                .is_some()
        })
    }

    fn is_loading(&self, cx: &App) -> bool {
        self.library
            .read(cx)
            .waiting(self.shelf, self.section.part(), cx)
    }
    fn pin(&self, row: usize, cx: &App) -> Option<Pin> {
        self.row(row, cx)?.pin()
    }

    fn context_menu(&self, rows: &[usize], _visible: &[Field], cx: &App) -> Option<Menu> {
        Some(match self.row(*rows.first()?, cx)? {
            Row::Album(row) => {
                album_menu(row.clone(), self.playback.clone(), self.menu.as_ref()?, cx)
            }
            Row::Artist(row) => artist_menu(row.clone(), self.playback.clone(), cx),
            Row::Playlist(row) => playlist_menu(row.clone(), self.playback.clone(), cx),
        })
    }

    fn context_menu_will_open(&self, _rows: &[usize], cx: &App) {
        if let Some(menu) = &self.menu {
            menu.reset(cx);
        }
    }

    fn cell(&self, cell: Cell<Field>, cx: &mut App) -> AnyElement {
        let Some(row) = self.row(cell.row, cx) else {
            return cells::blank(&cell);
        };
        let theme = cx.theme();
        let muted = theme.muted_foreground;
        match (row, cell.field) {
            (_, Field::Index) => {
                let origin = row.origin().named(row.text(Field::Name).to_owned());
                cells::collection_index(&cell, origin, &self.playback, cx)
            }
            (Row::Album(row), Field::Cover) => cells::artwork(&cell, row.cover.clone()),
            (Row::Artist(row), Field::Cover) => cells::avatar(&cell, row.cover.clone()),
            (Row::Playlist(row), Field::Cover) => cells::artwork(&cell, row.cover.clone()),
            (_, Field::Name) => {
                cells::dim(&cell, row.text(Field::Name).to_owned(), theme.foreground)
            }
            (Row::Album(row), Field::Artists) => {
                cells::artists(&cell, row.artist_refs.clone(), row.artists.clone(), muted)
            }
            (Row::Album(row), Field::Year) => cells::dim(
                &cell,
                row.year().map(|year| year.to_string()).unwrap_or_default(),
                muted,
            ),
            (_, Field::TrackCount) => cells::dim(
                &cell,
                row.number(cell.field)
                    .map(|n| n.to_string())
                    .unwrap_or_default(),
                muted,
            ),
            (_, Field::AddedAt) => {
                cells::dim(&cell, cells::added_stamp(row.timestamp(cell.field)), muted)
            }
            (_, Field::Modified) => {
                cells::dim(&cell, cells::stamp(row.timestamp(cell.field)), muted)
            }
            (Row::Playlist(row), Field::Owner) if row.owner_id.is_some() => cells::link(
                &cell,
                "playlist-owner",
                row.owner.clone(),
                muted,
                router::Destination::User(row.owner_id.clone().unwrap_or_default().into()),
            ),
            (_, Field::Owner) => cells::dim(&cell, row.text(Field::Owner).to_owned(), muted),
            _ => cells::blank(&cell),
        }
    }

    fn compare(&self, field: Field, a: usize, b: usize, cx: &App) -> Ordering {
        let text = |index| {
            self.row(index, cx)
                .map(|row| row.text(field))
                .unwrap_or_default()
        };
        let number = |index| self.row(index, cx).and_then(|row| row.number(field));
        match field {
            Field::Name | Field::Artists | Field::Owner => folded(text(a), text(b)),
            Field::AddedAt | Field::Modified => self
                .row(a, cx)
                .and_then(|row| row.timestamp(field))
                .cmp(&self.row(b, cx).and_then(|row| row.timestamp(field))),
            Field::Year | Field::TrackCount => number(a).cmp(&number(b)),
            Field::Index | Field::Cover => a.cmp(&b),
        }
    }

    fn group(&self, field: Field, row: usize, cx: &App) -> Option<SharedString> {
        match (self.row(row, cx)?, field) {
            (Row::Album(row), Field::Year) => Some(match row.year() {
                None => t!("common-unknown"),
                Some(year) => year.to_string().into(),
            }),
            (Row::Album(_), Field::Name | Field::Artists) | (Row::Artist(_), Field::Name) => {
                Some(initial(self.row(row, cx)?.text(field)))
            }
            _ => None,
        }
    }
}
