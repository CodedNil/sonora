use std::time::Duration;

use opensubsonic::data::{AlbumId3, ArtistId3, Child, ItemGenre, RecordLabel};

use crate::{Album, ArtistRef, Playlist, ReleaseType, SavedArtist, Track, UserProfile};

pub fn track(song: Child, cover: Option<String>) -> Track {
    let (artists, artist_refs) = artists_of(
        song.artist,
        song.artist_id,
        song.artists.as_ref(),
        song.display_artist,
    );
    Track {
        id: Some(song.id.clone()),
        name: song.title,
        playable: !song.is_video.unwrap_or(false),
        artists,
        artist_refs,
        album: song.album.unwrap_or_default(),
        album_id: song.album_id.filter(|id| !id.is_empty()),
        cover,
        duration: song
            .duration
            .and_then(|value| u64::try_from(value).ok())
            .map(Duration::from_secs),
        added_at: song.created.as_deref().and_then(crate::parse_timestamp),
        release_date: song.year.and_then(crate::ReleaseDate::from_year),
        playcount: song.play_count.map(|count| count as u64),
        explicit: song.explicit_status.as_deref() == Some("explicit"),
        track_number: song.track.unwrap_or(0).max(0) as u32,
        disc_number: song.disc_number.unwrap_or(1).max(1) as u32,
        genres: genres(song.genres.as_deref(), song.genre),
        moods: song.moods.unwrap_or_default(),
        bpm: song
            .bpm
            .and_then(|bpm| u16::try_from(bpm).ok())
            .filter(|bpm| *bpm > 0),
        ..Default::default()
    }
}

/// The genres a song is filed under. An OpenSubsonic server lists each of them, and any other
/// server names one genre at most.
fn genres(listed: Option<&[ItemGenre]>, single: Option<String>) -> Vec<String> {
    let listed: Vec<String> = listed
        .unwrap_or_default()
        .iter()
        .map(|genre| genre.name.trim())
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect();
    if !listed.is_empty() {
        return listed;
    }

    single
        .map(|genre| genre.trim().to_owned())
        .filter(|genre| !genre.is_empty())
        .into_iter()
        .collect()
}

pub fn album(source: AlbumId3, cover: Option<String>, cover_large: Option<String>) -> Album {
    let (artists, artist_refs) = artists_of(
        source.artist,
        source.artist_id,
        source.artists.as_ref(),
        source.display_artist,
    );
    let label = labels(source.record_labels.as_deref());
    Album {
        id: source.id,
        name: source.name,
        artists,
        artist_refs,
        cover,
        cover_large,
        release_type: release_type(source.release_types.as_deref(), source.is_compilation),
        track_count: source
            .song_count
            .and_then(|count| u32::try_from(count).ok()),
        release_date: source.year.and_then(crate::ReleaseDate::from_year),
        label,
        copyrights: Vec::new(),
        added_at: source.created.as_deref().and_then(crate::parse_timestamp),
    }
}

/// The record labels an OpenSubsonic server lists for an album, joined into one credit. A
/// server without the extension lists none, which leaves the label empty.
pub fn labels(labels: Option<&[RecordLabel]>) -> String {
    labels
        .unwrap_or_default()
        .iter()
        .map(|label| label.name.trim())
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

/// The kind of release an OpenSubsonic server lists for an album. A server without the extension
/// lists no types, which leaves every album an album unless it is flagged as a compilation.
pub fn release_type(types: Option<&[String]>, compilation: Option<bool>) -> ReleaseType {
    ReleaseType::from_musicbrainz(
        types.unwrap_or_default().iter().map(String::as_str),
        compilation.unwrap_or(false),
    )
}

pub fn playlist(
    id: &str,
    name: &str,
    owner: Option<&str>,
    public: Option<bool>,
    track_count: Option<u32>,
    cover: Option<String>,
    username: &str,
) -> Playlist {
    let owner = owner.unwrap_or_default().to_owned();
    Playlist {
        id: id.to_owned(),
        name: name.to_owned(),
        owner: owner.clone(),
        owner_id: (!owner.is_empty()).then(|| owner.clone()),
        owned: !owner.is_empty() && owner == username,
        can_edit_tracks: !owner.is_empty() && owner == username,
        public,
        cover,
        track_count,
        ..Default::default()
    }
}

pub fn saved_artist(source: &ArtistId3, cover: Option<String>) -> SavedArtist {
    SavedArtist {
        id: source.id.clone(),
        name: source.name.clone(),
        cover,
        added_at: None,
    }
}

pub fn profile(username: String) -> UserProfile {
    UserProfile {
        id: username.clone(),
        display_name: username,
        avatar: None,
    }
}

pub(crate) fn artists_of(
    artist: Option<String>,
    artist_id: Option<String>,
    many: Option<&Vec<ArtistId3>>,
    display: Option<String>,
) -> (String, Vec<ArtistRef>) {
    if let Some(list) = many.filter(|list| !list.is_empty()) {
        let (joined, refs) = ArtistRef::credits(list.iter().map(|item| ArtistRef {
            name: item.name.clone(),
            id: Some(item.id.clone()),
        }));
        return (display.unwrap_or(joined), refs);
    }
    let name = display.or(artist).unwrap_or_default();
    let id = artist_id.filter(|id| !id.is_empty());
    let refs = match (name.is_empty(), id) {
        (true, _) => Vec::new(),
        (false, id) => vec![ArtistRef {
            name: name.clone(),
            id,
        }],
    };
    (name, refs)
}
