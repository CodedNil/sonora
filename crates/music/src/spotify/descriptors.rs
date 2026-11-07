use std::collections::{HashMap, HashSet};

use anyhow::Result;
use librespot_core::Session;
use librespot_protocol::extension_kind::ExtensionKind;

use super::{collection, pb};

/// Reads Spotify's track-level genre/style descriptors through librespot's metadata transport.
/// Missing extensions leave a track untagged; malformed descriptors fail the optional lookup.
pub(crate) async fn genres(
    session: &Session,
    uris: &[String],
) -> Result<HashMap<String, Vec<String>>> {
    let entities = collection::extended(session, uris, ExtensionKind::TRACK_DESCRIPTOR).await?;
    entities
        .into_iter()
        .map(|entity| {
            let tags = concepts(&entity.extension_data.value)?;
            Ok((entity.entity_uri, tags))
        })
        .collect()
}

/// A descriptor contains repeated field-one concepts, each named by its first string field.
/// Confidence fields are deliberately not interpreted as listening weights or play counts.
fn concepts(bytes: &[u8]) -> Result<Vec<String>> {
    let mut reader = pb::Reader::new(bytes);
    let mut tags = Vec::new();
    let mut seen = HashSet::new();
    while let Some((field, value)) = reader.field()? {
        let (1, pb::Value::Bytes(concept)) = (field, value) else {
            continue;
        };
        let mut concept = pb::Reader::new(concept);
        while let Some((field, value)) = concept.field()? {
            let (1, pb::Value::Bytes(name)) = (field, value) else {
                continue;
            };
            let name = pb::text(name)?.trim().to_lowercase();
            let name = name
                .strip_prefix("spotify:concept:")
                .unwrap_or(&name)
                .replace('_', " ");
            let name = name.trim();
            if !name.is_empty() && name != "track" && seen.insert(name.to_owned()) {
                tags.push(name.to_owned());
            }
        }
    }
    Ok(tags)
}
