//! The full credits of a song, writers and producers included, from the credits view the
//! desktop client opens. The track's own metadata names only its performing artists.

use anyhow::{Context as _, Result};
use http::Method;
use librespot_core::Session;
use serde::Deserialize;

use crate::{Credit, escape};

const ARTIST_PREFIX: &str = "spotify:artist:";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct View {
    #[serde(default)]
    role_credits: Vec<Role>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Role {
    #[serde(default)]
    role_title: String,
    #[serde(default)]
    artists: Vec<Person>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    #[serde(default)]
    name: String,
    #[serde(default)]
    uri: String,
    #[serde(default)]
    subroles: Vec<String>,
}

/// Everyone credited on the song, once each, with every role they had joined in the order the
/// view lists them.
pub(crate) async fn credits(session: &Session, track_id: &str) -> Result<Vec<Credit>> {
    let endpoint = format!(
        "/track-credits-view/v0/experimental/{}/credits",
        escape::component(track_id)
    );
    let raw = session
        .spclient()
        .request(&Method::GET, &endpoint, None, None)
        .await
        .context("cannot read the song's credits")?;
    let view: View = serde_json::from_slice(&raw).context("cannot decode the song's credits")?;

    let mut credits: Vec<(Credit, Vec<String>)> = Vec::new();
    for role in view.role_credits {
        for person in role
            .artists
            .into_iter()
            .filter(|person| !person.name.is_empty())
        {
            let roles = match person.subroles.is_empty() {
                true => vec![role.role_title.clone()],
                false => person.subroles,
            };
            let id = person.uri.strip_prefix(ARTIST_PREFIX).map(str::to_owned);
            let at = match credits
                .iter()
                .position(|(held, _)| held.name == person.name)
            {
                Some(at) => at,
                None => {
                    let credit = Credit {
                        name: person.name,
                        role: String::new(),
                        id: None,
                    };
                    credits.push((credit, Vec::new()));
                    credits.len() - 1
                }
            };
            let (credit, held) = &mut credits[at];
            credit.id = credit.id.take().or(id);
            for role in roles
                .iter()
                .filter(|role| !role.is_empty())
                .map(|role| capitalised(role))
            {
                if !held.contains(&role) {
                    held.push(role);
                }
            }
        }
    }

    Ok(credits
        .into_iter()
        .map(|(credit, roles)| Credit {
            role: roles.join(" • "),
            ..credit
        })
        .collect())
}

/// The view writes subroles in lower case, so "main artist" becomes "Main artist".
fn capitalised(role: &str) -> String {
    let mut letters = role.chars();
    match letters.next() {
        Some(first) => first.to_uppercase().chain(letters).collect(),
        None => String::new(),
    }
}
