use serde::{Deserialize, Deserializer};

/// Artwork shared by Pathfinder album, artist, search and browse responses.
#[derive(Default, Deserialize)]
pub(super) struct Artwork {
    #[serde(default, deserialize_with = "nullable")]
    pub sources: Vec<Image>,
}

/// One artwork rendition; browse cards may omit its dimensions.
#[derive(Deserialize)]
pub(super) struct Image {
    pub height: Option<u32>,
    pub url: String,
}

/// The smallest rendition for a row, or the first at least header-sized, falling back to the largest.
pub(super) fn image(sources: &[Image], large: bool) -> Option<&Image> {
    let height = |image: &&Image| image.height.unwrap_or_default();
    match large {
        true => sources
            .iter()
            .filter(|image| height(image) >= 300)
            .min_by_key(height)
            .or_else(|| sources.iter().max_by_key(height)),
        false => sources.iter().min_by_key(height),
    }
}

/// Pathfinder dates arrive either as ISO text with precision, or separate components.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Date {
    iso_string: Option<String>,
    precision: Option<String>,
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
}

impl Date {
    pub fn release(&self) -> Option<crate::ReleaseDate> {
        match self.iso_string.as_deref() {
            Some(iso) => {
                let length = match self.precision.as_deref() {
                    Some("YEAR") => 4,
                    Some("MONTH") => 7,
                    _ => 10,
                };
                crate::ReleaseDate::parse(iso.get(..length).unwrap_or(iso))
            }
            None => crate::ReleaseDate::from_parts(
                self.year?,
                self.month.filter(|month| *month > 0),
                self.day.filter(|day| *day > 0),
            ),
        }
    }
}

/// Copyright notices use the same shape in release and artist-discography responses.
#[derive(Default, Deserialize)]
pub(super) struct Copyrights {
    #[serde(default)]
    pub items: Vec<Copyright>,
}

/// One notice, with an optional recording/work distinction.
#[derive(Deserialize)]
pub(super) struct Copyright {
    pub text: String,
    #[serde(rename = "type", default)]
    pub kind: String,
}

/// Treats nullable response collections as empty, matching omitted collections.
pub(super) fn nullable<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

/// Artist credits share the same shape across Pathfinder release, search and browse payloads.
#[derive(Default, Deserialize)]
pub(super) struct Artists {
    #[serde(default, deserialize_with = "nullable")]
    items: Vec<Artist>,
}

/// One credited artist, whose identity and profile may be incomplete.
#[derive(Deserialize)]
struct Artist {
    #[serde(default, deserialize_with = "nullable")]
    uri: String,
    profile: Option<Profile>,
}

/// The display portion shared by artist references and search hits.
#[derive(Deserialize)]
pub(super) struct Profile {
    #[serde(default, deserialize_with = "nullable")]
    pub name: String,
}

impl Artists {
    /// Builds display text and navigable credits together, preserving provider order.
    pub fn credits(&self) -> (String, Vec<crate::ArtistRef>) {
        let refs: Vec<_> = self
            .items
            .iter()
            .filter_map(|artist| {
                let name = artist.profile.as_ref()?.name.trim();
                (!name.is_empty()).then(|| crate::ArtistRef {
                    name: name.to_owned(),
                    id: artist
                        .uri
                        .strip_prefix("spotify:artist:")
                        .filter(|id| !id.is_empty())
                        .map(str::to_owned),
                })
            })
            .collect();
        (crate::ArtistRef::names(&refs), refs)
    }
}
