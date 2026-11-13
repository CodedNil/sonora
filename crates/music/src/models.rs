use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{ReleaseDate, Timestamp};

/// A size of the same cover that a provider can name without fetching anything.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverSize {
    /// Big enough for a header or a desktop widget, 640 pixels on Spotify.
    Large,
    /// The original upload, for a cover that fills the screen.
    Full,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UserProfile {
    pub id: String,
    pub display_name: String,
    /// A picture for the account, when the provider hands one out. `None` falls back to the
    /// initials of `display_name`.
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UserDetail {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub followers: Option<u64>,
    pub following: Option<u64>,
    pub playlists: Vec<Playlist>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Contributor {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
}

impl Contributor {
    pub fn unnamed(id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            name: id.clone(),
            id,
            avatar: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtistRef {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// The short looping clip a song shows behind its artwork, or the still an artist set instead.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Canvas {
    pub url: String,
    /// Whether `url` is a clip to loop rather than a still image.
    pub video: bool,
}

impl ArtistRef {
    /// Joins individual credits without changing their provider-supplied order or wording.
    pub(crate) fn names(artists: &[Self]) -> String {
        artists
            .iter()
            .map(|artist| artist.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Builds navigable credits and their display text from the same ordered list.
    pub(crate) fn credits(artists: impl IntoIterator<Item = Self>) -> (String, Vec<Self>) {
        let artists: Vec<_> = artists.into_iter().collect();
        (Self::names(&artists), artists)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Credit {
    pub name: String,
    pub role: String,
    pub id: Option<String>,
}

/// Measured musical attributes on a 0–1 scale, separate from a provider's descriptive mood tags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AudioFeatures {
    /// How suitable the recording is for dancing.
    pub danceability: Option<f32>,
    /// Perceived intensity and activity.
    pub energy: Option<f32>,
    /// Musical positivity, from somber to cheerful.
    pub valence: Option<f32>,
    /// Confidence that the recording is acoustic.
    pub acousticness: Option<f32>,
    /// Confidence that the recording contains no vocals.
    pub instrumentalness: Option<f32>,
}

impl AudioFeatures {
    /// Classifies measured audio features using provider-neutral mood names.
    pub fn moods(&self) -> impl Iterator<Item = &'static str> {
        let at_least =
            |value: Option<f32>, threshold| value.is_some_and(|value| value >= threshold);
        let energy = self.energy;
        let low_valence = self.valence.is_some_and(|value| value <= 0.3);
        [
            ("energetic", at_least(energy, 0.7)),
            ("danceable", at_least(self.danceability, 0.7)),
            ("positive", at_least(self.valence, 0.6)),
            ("negative", low_valence && at_least(energy, 0.5)),
            (
                "sad",
                low_valence && energy.is_some_and(|value| value < 0.5),
            ),
            ("calming", energy.is_some_and(|value| value <= 0.35)),
            ("instrumental", at_least(self.instrumentalness, 0.5)),
            ("acoustic", at_least(self.acousticness, 0.6)),
        ]
        .into_iter()
        .filter(|(_, matches)| *matches)
        .map(|(mood, _)| mood)
    }
}

/// A song or episode as a provider lists it. The default is an unplayable track with no id, so a
/// caller that builds one from `..Default::default()` sets `playable` itself.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Track {
    /// Provider track ID, absent when a listing supplies no playable identity.
    pub id: Option<String>,
    /// Track title as supplied by the provider or file tags.
    pub name: String,
    /// Whether the provider considers this track available for playback.
    pub playable: bool,
    /// Formatted artist credit, preserving provider wording and separators.
    pub artists: String,
    /// Individual artist credits with optional IDs for navigation.
    pub artist_refs: Vec<ArtistRef>,
    /// Album display name, empty when unavailable.
    pub album: String,
    /// Provider album ID for navigation, when available.
    pub album_id: Option<String>,
    /// Artwork URL or local image path, when available.
    pub cover: Option<String>,
    /// Reported running time, absent until the provider or decoder can determine it.
    pub duration: Option<Duration>,
    /// Time of saving, adding, playing or modifying, depending on the list.
    pub added_at: Option<Timestamp>,
    /// Person who added this playlist entry, when supplied.
    pub added_by: Option<Arc<Contributor>>,
    /// Release date at the precision supplied, absent when unknown.
    pub release_date: Option<ReleaseDate>,
    /// Provider play count, which may be global or listener-specific.
    pub playcount: Option<u64>,
    /// Provider popularity score from 0 to 100, absent when unavailable.
    pub popularity: Option<u8>,
    /// Whether the provider or file tags mark the content as explicit.
    pub explicit: bool,
    /// Position within the disc or album, starting at one; zero when unknown.
    pub track_number: u32,
    /// One-based disc number; providers use zero for unknown or default to one.
    pub disc_number: u32,
    /// Musical genres supplied by the provider or file tags.
    pub genres: Vec<String>,
    /// Descriptive words such as activities or styles, distinct from genres.
    pub tags: Vec<String>,
    /// Descriptive mood tags supplied by the provider or local file.
    pub moods: Vec<String>,
    /// Measured audio attributes, when the source supplies them.
    pub audio_features: Option<AudioFeatures>,
    /// Tempo in whole beats per minute, absent when unknown.
    pub bpm: Option<u16>,
    /// Languages of the performance, normalized to codes where recognized.
    pub languages: Vec<String>,
    /// Named contributors and their credited roles, when supplied.
    pub credits: Vec<Credit>,
}

impl Track {
    /// The known release year, including dates supplied at finer precision.
    pub fn year(&self) -> Option<u16> {
        self.release_date.map(ReleaseDate::year)
    }
}

/// A playlist summary; its tracks are loaded separately through the provider.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Playlist {
    /// Provider playlist ID used for lookup and playlist actions.
    pub id: String,
    /// Playlist title as supplied by the provider.
    pub name: String,
    /// Display name of the owner or curator, empty when unavailable.
    pub owner: String,
    /// Provider owner ID for navigation, absent when unavailable.
    pub owner_id: Option<String>,
    /// Whether this is the listener's own playlist; track-editing access is separate.
    pub owned: bool,
    /// Whether the listener may add or remove tracks, independently of ownership.
    pub can_edit_tracks: bool,
    /// Whether this is a Spotify Blend with per-song contributor attribution.
    pub blend: bool,
    /// Public visibility reported by the provider, absent when unknown.
    pub public: Option<bool>,
    /// Artwork URL or local image path, when available.
    pub cover: Option<String>,
    /// Reported number of tracks, absent when unknown.
    pub track_count: Option<u32>,
    /// Provider change time, when available.
    pub modified_at: Option<Timestamp>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReleaseType {
    #[default]
    Album,
    Single,
    Compilation,
    Ep,
    Audiobook,
    Podcast,
}

impl ReleaseType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Album => "Album",
            Self::Single => "Single",
            Self::Compilation => "Compilation",
            Self::Ep => "EP",
            Self::Audiobook => "Audiobook",
            Self::Podcast => "Podcast",
        }
    }

    /// Reads the MusicBrainz release types a tag or an OpenSubsonic server lists, such as
    /// `["album", "compilation"]` or a single `"EP; Live"`, in any case. A primary EP or single
    /// wins over a compilation, and a release naming neither is an album.
    pub fn from_musicbrainz<'a>(
        types: impl IntoIterator<Item = &'a str>,
        compilation: bool,
    ) -> Self {
        let mut named = compilation.then_some(Self::Compilation);
        let parts = types
            .into_iter()
            .flat_map(|types| types.split([';', '/', ',', '\0']))
            .map(str::trim);
        for part in parts {
            match part.to_ascii_lowercase().as_str() {
                "ep" => return Self::Ep,
                "single" => return Self::Single,
                "compilation" => named = Some(Self::Compilation),
                _ => {}
            }
        }
        named.unwrap_or(Self::Album)
    }
}

/// A release summary; its tracks are loaded separately through the provider.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Album {
    /// Provider album ID used for lookup and navigation.
    pub id: String,
    /// Release title as supplied by the provider or file tags.
    pub name: String,
    /// Formatted artist credit, preserving provider wording and separators.
    pub artists: String,
    /// Individual artist credits with optional IDs for navigation.
    pub artist_refs: Vec<ArtistRef>,
    /// Artwork URL or local image path for cards and lists.
    pub cover: Option<String>,
    /// Larger artwork URL or local image path for the release header.
    pub cover_large: Option<String>,
    /// Kind of release, such as an album, single or EP.
    pub release_type: ReleaseType,
    /// Release date at the precision supplied, absent when unknown.
    pub release_date: Option<ReleaseDate>,
    /// Reported number of tracks, absent when unknown.
    pub track_count: Option<u32>,
    /// Record label display text, empty when unavailable.
    pub label: String,
    /// Provider copyright and phonographic copyright notices.
    pub copyrights: Vec<String>,
    /// Library addition or local file modification time.
    pub added_at: Option<Timestamp>,
}

impl Album {
    /// The known release year, including dates supplied at finer precision.
    pub fn year(&self) -> Option<u16> {
        self.release_date.map(ReleaseDate::year)
    }

    /// Credits the album to `artist` when it names no one. A provider calls this on releases
    /// read from an artist's own listing, which often leaves the artist out of each entry.
    pub fn credit(mut self, artist: &ArtistRef) -> Self {
        if self.artists.is_empty() && self.artist_refs.is_empty() && !artist.name.is_empty() {
            self.artists = artist.name.clone();
            self.artist_refs = vec![artist.clone()];
        }
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlbumDetail {
    pub album: Album,
    pub tracks: Vec<Track>,
}

/// What an album page fills in once its tracks are already on screen: the releases the
/// provider lists as related, with more from the same artist first and similar artists'
/// releases topping the rail up, plus the similar artists themselves for the rail's
/// artists tab. Every list replaces what the page held, and an empty one leaves that
/// part alone.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AlbumCatalogue {
    pub also_like: Vec<Album>,
    pub similar: Vec<SavedArtist>,
}

impl AlbumCatalogue {
    pub fn is_empty(&self) -> bool {
        self.also_like.is_empty() && self.similar.is_empty()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Genre {
    pub id: String,
    pub name: String,
    pub cover: Option<String>,
}

/// One card of a browse shelf. A provider's home and genre pages mix whatever the shelf holds,
/// so a track or an artist sits beside albums and playlists in the same row.
#[derive(Clone, Debug, PartialEq)]
pub enum GenreItem {
    Playlist(Playlist),
    Album(Album),
    Genre(Genre),
    Track(Track),
    Artist(SavedArtist),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GenreSection {
    pub title: String,
    pub items: Vec<GenreItem>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HomeFeed {
    /// What the user has been playing lately, in every shape the provider lists it: songs and
    /// videos, but also the albums, playlists and artists they came from.
    pub listen_again: Vec<GenreItem>,
    pub quick_picks: Option<Vec<Track>>,
    pub sections: Vec<GenreSection>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GenreDetail {
    pub name: String,
    pub sections: Vec<GenreSection>,
}

impl GenreDetail {
    /// The picture a tile for this genre wears: the first album cover on the page, else the
    /// first playlist cover.
    pub fn cover(&self) -> Option<String> {
        let mut playlist = None;
        for item in self.sections.iter().flat_map(|section| &section.items) {
            match item {
                GenreItem::Album(album) if album.cover.is_some() => return album.cover.clone(),
                GenreItem::Playlist(list) if playlist.is_none() => playlist = list.cover.clone(),
                _ => {}
            }
        }
        playlist
    }
}

/// What reading a playlist file into the library came to.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ImportedPlaylist {
    /// The playlist made from the file, or `None` when none of its songs is in the library,
    /// in which case nothing was made.
    pub playlist: Option<Playlist>,
    /// The file's name without its extension, which the new playlist is called.
    pub name: String,
    /// How many entries named a file the library does not hold, and so were left out.
    pub skipped: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TrackTags {
    pub title: String,
    /// The track's artists, one name each, in the order the file credits them.
    pub artists: Vec<String>,
    pub album: String,
    pub album_artist: String,
    pub track_number: String,
    pub track_total: String,
    pub disc_number: String,
    pub disc_total: String,
    pub year: String,
    pub genre: String,
    pub composer: String,
    pub publisher: String,
    pub isrc: String,
    pub comment: String,
    pub lyrics: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlaylistDetail {
    pub playlist: Playlist,
    pub tracks: Vec<Track>,
    pub continuation: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArtistProfile {
    pub name: String,
    pub cover_large: Option<String>,
    pub biography: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SavedArtist {
    pub id: String,
    pub name: String,
    pub cover: Option<String>,
    pub added_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Artist {
    pub name: String,
    pub cover_large: Option<String>,
    /// A wide picture for the top of the artist page, when the provider keeps one apart from
    /// the portrait in `cover_large`.
    pub banner: Option<String>,
    pub biography: Option<String>,
    pub monthly_listeners: Option<u64>,
    pub top_tracks: Vec<Track>,
    pub albums: Vec<Album>,
}

/// What an artist page fills in once its overview is already on screen. Every list
/// replaces what `MusicApi::artist` answered with, and an empty one leaves that part alone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ArtistCatalogue {
    pub albums: Vec<Album>,
    pub top_tracks: Vec<Track>,
    /// What the artist guests on, from the provider's appears-on listing.
    pub appears_on: Vec<Album>,
}

impl ArtistCatalogue {
    pub fn is_empty(&self) -> bool {
        self.albums.is_empty() && self.top_tracks.is_empty() && self.appears_on.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lyrics {
    Plain {
        text: String,
        /// One line for each line of `text`, blank where that line needs no romanization.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        romanized: Option<RomanizedText>,
    },
    Synced {
        lines: Arc<[LyricsLine]>,
    },
}

/// No lyrics at all, as empty plain text.
impl Default for Lyrics {
    fn default() -> Self {
        Self::Plain {
            text: String::new(),
            romanized: None,
        }
    }
}

impl Lyrics {
    pub fn plain(text: impl Into<String>) -> Self {
        let text = text.into();
        let romanized = crate::lyrics::romanize::plain(&text);
        Self::Plain { text, romanized }
    }

    pub fn synced(&self) -> bool {
        matches!(self, Self::Synced { .. })
    }

    pub fn worded(&self) -> bool {
        match self {
            Self::Plain { .. } => false,
            Self::Synced { lines } => lines.iter().any(LyricsLine::worded),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Plain { text, .. } => text.trim().is_empty(),
            Self::Synced { lines } => lines.is_empty(),
        }
    }

    pub fn span(&self) -> Option<Duration> {
        let Self::Synced { lines } = self else {
            return None;
        };
        lines
            .iter()
            .map(|line| line.end.unwrap_or(line.start))
            .max()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricsLine {
    pub start: Duration,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<Duration>,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub romanized: Option<RomanizedText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<LyricsWord>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secondary: Vec<LyricsLane>,
    #[serde(default, skip_serializing_if = "Voice::lead")]
    pub voice: Voice,
}

impl LyricsLine {
    pub fn worded(&self) -> bool {
        self.words.as_ref().is_some_and(|words| !words.is_empty())
            || self.secondary.iter().any(LyricsLane::worded)
    }

    pub fn sung_end(&self) -> Option<Duration> {
        let primary = self
            .words
            .as_ref()
            .and_then(|words| words.iter().rev().find(|word| !word.text.trim().is_empty()))
            .map(|word| word.end.max(word.start).max(self.start))
            .or(self.end);
        self.secondary
            .iter()
            .filter_map(LyricsLane::sung_end)
            .chain(primary)
            .max()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricsLane {
    pub start: Duration,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<Duration>,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub romanized: Option<RomanizedText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<LyricsWord>>,
}

impl LyricsLane {
    pub fn worded(&self) -> bool {
        self.words.as_ref().is_some_and(|words| !words.is_empty())
    }

    pub fn sung_end(&self) -> Option<Duration> {
        self.words
            .as_ref()
            .and_then(|words| words.iter().rev().find(|word| !word.text.trim().is_empty()))
            .map(|word| word.end.max(word.start).max(self.start))
            .or(self.end)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct RomanizedText {
    pub text: String,
    pub writing_system: WritingSystem,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WritingSystem {
    Japanese,
    Chinese,
    Korean,
    Cyrillic,
    Greek,
    Arabic,
    #[default]
    Other,
}

impl WritingSystem {
    pub const ALL: [Self; 7] = [
        Self::Japanese,
        Self::Chinese,
        Self::Korean,
        Self::Cyrillic,
        Self::Greek,
        Self::Arabic,
        Self::Other,
    ];
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Voice {
    #[default]
    Lead,
    Counter,
}

impl Voice {
    pub fn lead(&self) -> bool {
        matches!(self, Self::Lead)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricsWord {
    pub start: Duration,
    pub end: Duration,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TrackKey {
    pub provider: &'static str,
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LyricsQuery {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration: Duration,
    pub track: Option<TrackKey>,
}

impl LyricsQuery {
    pub fn id_for(&self, provider: &str) -> Option<&str> {
        self.track
            .as_ref()
            .filter(|track| track.provider == provider)
            .map(|track| track.id.as_str())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LyricsHit {
    pub source: &'static str,
    pub trust: u32,
    pub lyrics: Lyrics,
    pub instrumental: bool,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration: Option<Duration>,
    pub writers: Vec<String>,
}

/// Something the provider keeps sidebar pins for, with whether it is pinned now. The uri is
/// the provider's own and is what `MusicApi::set_pinned` takes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PinTarget {
    pub uri: String,
    pub name: String,
    pub subtitle: String,
    pub cover: Option<String>,
    pub kind: PinTargetKind,
    pub pinned: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinTargetKind {
    Playlist,
    Album,
    Artist,
    LikedSongs,
    Audiobook,
    Show,
    Folder,
}

/// How a provider answered a pin change. `LimitReached` means it turned the pin away for
/// holding too many already, and `Outside` that it can only pin what is in the listener's
/// library. Either way the pin stays a local one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinOutcome {
    Updated,
    LimitReached,
    Outside,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn musicbrainz_types_name_the_release() {
        let cases = [
            (&["ep"][..], false, ReleaseType::Ep),
            (&["EP", "Live"], false, ReleaseType::Ep),
            (&["single"], false, ReleaseType::Single),
            (&["album", "compilation"], false, ReleaseType::Compilation),
            (&["Album; Compilation"], false, ReleaseType::Compilation),
            (&["ep/compilation"], false, ReleaseType::Ep),
            (&["album"], true, ReleaseType::Compilation),
            (&["album", "soundtrack"], false, ReleaseType::Album),
            (&[], false, ReleaseType::Album),
        ];
        for (types, compilation, expected) in cases {
            assert_eq!(
                ReleaseType::from_musicbrainz(types.iter().copied(), compilation),
                expected,
                "{types:?}"
            );
        }
    }
}
