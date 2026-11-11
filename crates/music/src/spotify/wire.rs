use std::fmt::Write as _;

use librespot_protocol::metadata::Image;
use librespot_protocol::metadata::image::Size as ImageSize;
use librespot_protocol::playlist4_external::{ListAttributes, SelectedListContent as RootList};

use crate::models;

pub const UNKNOWN: &str = "Unknown";
const IMAGE_CDN: &str = "https://i.scdn.co/image/";
const ALBUM_COVER: &str = "ab67616d0000";
/// The size codes that follow `ALBUM_COVER` in an album cover id.
const LARGE: &str = "b273";
const ORIGINAL: &str = "82c1";
const BLEND: &str = "blend";
const BY_SIZE: [&str; 4] = ["xlarge", "large", "default", "small"];

pub fn image_url(file_id: &[u8]) -> Option<String> {
    if file_id.is_empty() {
        return None;
    }

    let hex = file_id.iter().fold(String::new(), |mut hex, byte| {
        let _ = write!(hex, "{byte:02x}");
        hex
    });
    Some(format!("{IMAGE_CDN}{hex}"))
}

/// An album cover from `image_url` at another size. Album cover ids are `ab67616d0000`, a
/// two-byte size code and a hash shared by every size, and `82c1` asks the CDN for the original
/// upload, up to 2000 pixels where the listed sizes stop at 640.
pub fn cover_at(url: &str, size: models::CoverSize) -> Option<String> {
    let id = url.strip_prefix(IMAGE_CDN)?;
    let (album, rest) = id.split_at_checked(ALBUM_COVER.len())?;
    if album != ALBUM_COVER || rest.len() != 28 {
        return None;
    }
    let code = match size {
        models::CoverSize::Large => LARGE,
        models::CoverSize::Full => ORIGINAL,
    };
    Some(format!(
        "{IMAGE_CDN}{ALBUM_COVER}{code}{}",
        &rest[code.len()..]
    ))
}

/// An image's width in pixels, taken from its size class when the metadata leaves the width
/// out.
pub fn image_width(image: &Image) -> i32 {
    if image.width() > 0 {
        return image.width();
    }

    match image.size() {
        ImageSize::SMALL => 64,
        ImageSize::DEFAULT => 300,
        ImageSize::LARGE => 640,
        ImageSize::XLARGE => 1_000,
    }
}

pub fn blend(attributes: &ListAttributes) -> bool {
    if attributes.format().to_ascii_lowercase().contains(BLEND) {
        return true;
    }

    attributes
        .format_attributes
        .iter()
        .any(|attribute| attribute.key().to_ascii_lowercase().starts_with(BLEND))
}

pub fn fetchable(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

fn playlist_cover(attributes: &ListAttributes) -> Option<String> {
    for target in BY_SIZE {
        if let Some(size) = attributes
            .picture_size
            .iter()
            .find(|size| size.target_name() == target)
            .filter(|size| fetchable(size.url()))
        {
            return Some(size.url().to_owned());
        }
    }

    attributes
        .picture_size
        .first()
        .map(|size| size.url())
        .filter(|url| fetchable(url))
        .map(str::to_owned)
        .or_else(|| image_url(attributes.picture()))
}

pub fn playlist_from(id: &str, content: &RootList, username: &str) -> models::Playlist {
    let owner = match content.owner_username() {
        "" => UNKNOWN,
        owner => owner,
    };
    let name = match content.attributes.name() {
        "" => UNKNOWN,
        name => name,
    };

    models::Playlist {
        id: id.to_owned(),
        name: name.to_owned(),
        owner: owner.to_owned(),
        owner_id: (!content.owner_username().is_empty())
            .then(|| content.owner_username().to_owned()),
        owned: owner == username,
        can_edit_tracks: content
            .capabilities
            .as_ref()
            .and_then(|permissions| permissions.can_edit_items)
            .unwrap_or_else(|| owner == username || content.attributes.collaborative()),
        blend: blend(&content.attributes),
        public: None,
        cover: playlist_cover(&content.attributes),
        track_count: content.length.and_then(|count| u32::try_from(count).ok()),
        modified_at: timestamp(content.timestamp()),
    }
}

pub fn playlists_from(rootlist: &RootList, username: &str) -> Vec<models::Playlist> {
    let contents = &rootlist.contents;
    let meta = &contents.meta_items;

    contents
        .items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let id = item.uri().strip_prefix("spotify:playlist:")?;
            let meta = meta.get(index);

            let name = meta
                .map(|meta| meta.attributes.name())
                .filter(|name| !name.is_empty())
                .unwrap_or(UNKNOWN);
            let owner = meta
                .map(|meta| meta.owner_username())
                .filter(|owner| !owner.is_empty())
                .unwrap_or(UNKNOWN);

            Some(models::Playlist {
                id: id.to_owned(),
                name: name.to_owned(),
                owner: owner.to_owned(),
                owner_id: meta
                    .map(|meta| meta.owner_username())
                    .filter(|owner| !owner.is_empty())
                    .map(str::to_owned),
                owned: owner == username,
                can_edit_tracks: meta.is_some_and(|meta| {
                    meta.capabilities
                        .as_ref()
                        .and_then(|permissions| permissions.can_edit_items)
                        .unwrap_or_else(|| owner == username || meta.attributes.collaborative())
                }),
                blend: meta.is_some_and(|meta| blend(&meta.attributes)),
                public: item.attributes.public,
                cover: meta.and_then(|meta| playlist_cover(&meta.attributes)),
                track_count: meta
                    .and_then(|meta| meta.length)
                    .and_then(|count| u32::try_from(count).ok()),
                modified_at: None,
            })
        })
        .collect()
}

/// A copyright line as a release page prints it, led by ℗ for the sound recording or © for
/// the work. Spotify keeps the symbol out of the text and names the kind beside it, so a
/// line that already carries a symbol is left as it is.
pub fn copyright(recording: bool, text: &str) -> String {
    match text.starts_with(['©', '℗']) {
        true => text.to_owned(),
        false => match recording {
            true => format!("℗ {text}"),
            false => format!("© {text}"),
        },
    }
}

pub fn timestamp(millis: i64) -> Option<crate::Timestamp> {
    (millis > 0)
        .then(|| crate::Timestamp::from_millisecond(millis).ok())
        .flatten()
}

/// Retains the precision of Spotify's optional protobuf date components.
pub(super) fn release_date(
    date: &librespot_protocol::metadata::Date,
) -> Option<crate::ReleaseDate> {
    crate::ReleaseDate::from_parts(
        date.year(),
        date.month
            .filter(|month| *month > 0)
            .map(|month| month as u32),
        date.day.filter(|day| *day > 0).map(|day| day as u32),
    )
}
