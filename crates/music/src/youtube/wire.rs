use crate::models::{
    Album, AlbumDetail, Artist, ArtistProfile, ArtistRef, Playlist, PlaylistDetail, ReleaseType,
    Track, UserProfile,
};

/// The longer edge of a large cover, wide enough for the fullscreen player on a 2x display.
const COVER_LARGE: u32 = 1200;

pub fn track(source: ytmusic::Track, index: u32) -> Track {
    let (artists, artist_refs) =
        ArtistRef::credits(source.artists.into_iter().map(|artist| ArtistRef {
            name: artist.name,
            id: artist.id,
        }));
    Track {
        id: source.video_id,
        name: source.title,
        playable: source.available,
        artists,
        artist_refs,
        album: source
            .album
            .as_ref()
            .map(|album| album.name.clone())
            .unwrap_or_default(),
        album_id: source.album.and_then(|album| album.id),
        cover: cover(&source.thumbnails),
        duration: source.duration,
        explicit: source.explicit,
        track_number: index + 1,
        disc_number: 1,
        ..Default::default()
    }
}

pub fn album(source: ytmusic::Album) -> Album {
    let (artists, artist_refs) =
        ArtistRef::credits(source.artists.into_iter().map(|artist| ArtistRef {
            name: artist.name,
            id: artist.id,
        }));
    Album {
        id: source.browse_id,
        name: source.title,
        artists,
        artist_refs,
        cover: cover(&source.thumbnails),
        cover_large: cover_large(&source.thumbnails),
        release_type: release_type(source.kind),
        track_count: source.track_count,
        release_date: source.year.and_then(crate::ReleaseDate::from_year),
        ..Default::default()
    }
}

pub fn album_detail(source: ytmusic::AlbumDetail) -> AlbumDetail {
    let tracks = source
        .tracks
        .into_iter()
        .enumerate()
        .map(|(index, item)| track(item, index as u32))
        .collect();
    AlbumDetail {
        album: album(source.album),
        tracks,
    }
}

pub fn playlist(source: ytmusic::Playlist, owned: bool) -> Playlist {
    Playlist {
        id: source.id,
        name: source.title,
        owner: source.author.unwrap_or_default(),
        owned: owned || source.owned,
        can_edit_tracks: owned || source.owned,
        public: source.public,
        cover: cover(&source.thumbnails),
        track_count: source.track_count,
        ..Default::default()
    }
}

pub fn playlist_detail(source: ytmusic::PlaylistDetail) -> PlaylistDetail {
    let owned = source.playlist.owned;
    let tracks = source
        .tracks
        .into_iter()
        .enumerate()
        .map(|(index, item)| track(item, index as u32))
        .collect();
    PlaylistDetail {
        playlist: playlist(source.playlist, owned),
        tracks,
        continuation: source.continuation,
    }
}

pub fn playlist_page(source: ytmusic::PlaylistPage) -> (Vec<Track>, Option<String>) {
    let tracks = source
        .tracks
        .into_iter()
        .enumerate()
        .map(|(index, item)| track(item, index as u32))
        .collect();
    (tracks, source.continuation)
}

pub fn artist(source: ytmusic::Artist) -> Artist {
    let top_tracks = source
        .top_tracks
        .into_iter()
        .enumerate()
        .map(|(index, item)| track(item, index as u32))
        .collect();
    let albums = source
        .albums
        .into_iter()
        .chain(source.singles)
        .map(album)
        .collect();
    Artist {
        name: source.name,
        cover_large: cover_large(&source.thumbnails),
        // YouTube's header picture is also the artist's portrait, so the page washes the portrait
        // out behind the hero instead of showing the same picture twice.
        banner: None,
        biography: source.description,
        monthly_listeners: None,
        top_tracks,
        albums,
    }
}

pub fn artist_profile(source: &ytmusic::Artist) -> ArtistProfile {
    ArtistProfile {
        name: source.name.clone(),
        cover_large: cover_large(&source.thumbnails),
        biography: source.description.clone(),
    }
}

pub fn profile(source: ytmusic::Profile) -> UserProfile {
    UserProfile {
        avatar: source
            .thumbnails
            .last()
            .map(|thumbnail| thumbnail.url.clone()),
        id: source.email.unwrap_or_else(|| source.name.clone()),
        display_name: source.name,
    }
}

fn release_type(kind: ytmusic::AlbumKind) -> ReleaseType {
    match kind {
        ytmusic::AlbumKind::Album => ReleaseType::Album,
        ytmusic::AlbumKind::Single => ReleaseType::Single,
        ytmusic::AlbumKind::Ep => ReleaseType::Ep,
        ytmusic::AlbumKind::Compilation => ReleaseType::Compilation,
    }
}

pub fn cover(thumbnails: &[ytmusic::Thumbnail]) -> Option<String> {
    thumbnails.last().map(|thumb| thumb.url.clone())
}

/// The largest thumbnail, asked of Google's image server at `COVER_LARGE` on its longer edge
/// when the api offers less. YouTube Music hands out 544 pixels at most, though the server
/// scales the same picture to any size.
pub fn cover_large(thumbnails: &[ytmusic::Thumbnail]) -> Option<String> {
    thumbnails
        .last()
        .map(|thumb| resized(&thumb.url, COVER_LARGE))
}

/// Rewrites the `=w<width>-h<height>` options of a googleusercontent url so its longer edge is
/// at least `edge`, keeping the aspect and every other option. Any other url, or one already
/// that large, comes back unchanged.
fn resized(url: &str, edge: u32) -> String {
    let Some((base, options)) = url.rsplit_once('=') else {
        return url.to_owned();
    };
    if !base.contains(".googleusercontent.com/") {
        return url.to_owned();
    }
    let size = |prefix: char| {
        options
            .split('-')
            .find_map(|option| option.strip_prefix(prefix)?.parse::<u32>().ok())
    };
    let (Some(width), Some(height)) = (size('w'), size('h')) else {
        return url.to_owned();
    };
    let longer = width.max(height);
    if longer == 0 || longer >= edge {
        return url.to_owned();
    }
    let scale = |side: u32| u64::from(side) * u64::from(edge) / u64::from(longer);
    let numeric = |option: &str, prefix: char| {
        option
            .strip_prefix(prefix)
            .is_some_and(|value| value.parse::<u32>().is_ok())
    };
    let options = options
        .split('-')
        .map(|option| {
            if numeric(option, 'w') {
                format!("w{}", scale(width))
            } else if numeric(option, 'h') {
                format!("h{}", scale(height))
            } else {
                option.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("-");
    format!("{base}={options}")
}
