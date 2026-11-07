use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use super::{Account, Link, Play, Secret, Service};
use crate::{Album, ListeningStats, Ranked, SavedArtist, StatsPeriod, Track};

/// The public instance. An account that names its own `server` is used instead, which is how a
/// self-hosted ListenBrainz is reached without a second settings field.
const API: &str = "https://api.listenbrainz.org";
const SETTINGS: &str = "https://listenbrainz.org/settings/";
/// How many listens the endpoint takes in one call.
const BATCH: usize = 50;
const CLIENT: &str = "Sonora";
const STATS_PERIODS: &[StatsPeriod] = &[
    StatsPeriod::ThisWeek,
    StatsPeriod::Week,
    StatsPeriod::ThisMonth,
    StatsPeriod::Month,
    StatsPeriod::Quarter,
    StatsPeriod::HalfYear,
    StatsPeriod::ThisYear,
    StatsPeriod::Year,
    StatsPeriod::AllTime,
];

pub struct ListenBrainz;

#[async_trait]
impl Service for ListenBrainz {
    fn id(&self) -> &'static str {
        "listenbrainz"
    }

    fn link(&self) -> Link {
        Link::Token
    }

    fn signup(&self) -> Option<&'static str> {
        Some(SETTINGS)
    }

    async fn connect(&self, secret: Secret) -> Result<Account> {
        let Secret::Token(token) = secret else {
            bail!("listenbrainz needs a user token");
        };
        let token = token.trim().to_owned();
        if token.is_empty() {
            bail!("listenbrainz needs a user token");
        }

        let answer: Validation = super::http()
            .get(format!("{API}/1/validate-token"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .send()
            .await
            .context("cannot reach listenbrainz")?
            .json()
            .await
            .context("cannot read the listenbrainz answer")?;

        if !answer.valid {
            let message = answer.message.unwrap_or_default();
            bail!("listenbrainz refused the token: {message}");
        }

        Ok(Account {
            session: token,
            name: answer.user_name.unwrap_or_default(),
            enabled: true,
            ..Account::default()
        })
    }

    async fn now_playing(&self, account: &Account, play: &Play) -> Result<()> {
        submit(account, "playing_now", &[listen(play, false)]).await
    }

    async fn scrobble(&self, account: &Account, plays: &[Play]) -> Result<()> {
        for batch in plays.chunks(BATCH) {
            let listens: Vec<Listen<'_>> = batch.iter().map(|play| listen(play, true)).collect();
            submit(account, "single", &listens).await?;
        }
        Ok(())
    }

    fn stats_periods(&self) -> &'static [StatsPeriod] {
        STATS_PERIODS
    }

    async fn stats(&self, account: &Account, period: StatsPeriod) -> Result<ListeningStats> {
        let range = match period {
            StatsPeriod::ThisWeek => "this_week",
            StatsPeriod::Week | StatsPeriod::Weeks => "week",
            StatsPeriod::ThisMonth => "this_month",
            StatsPeriod::Month => "month",
            StatsPeriod::Quarter => "quarter",
            StatsPeriod::HalfYear | StatsPeriod::Months => "half_yearly",
            StatsPeriod::ThisYear => "this_year",
            StatsPeriod::Year => "year",
            StatsPeriod::AllTime => "all_time",
        };
        let (artists, recordings, releases) = tokio::try_join!(
            stats_request(account, "artists", range),
            stats_request(account, "recordings", range),
            stats_request(account, "release-groups", range),
        )?;
        let artists = payload_rows(&artists, "artists")
            .iter()
            .map(|row| Ranked {
                item: SavedArtist { name: value_string(row, "artist_name"), ..Default::default() },
                count: value_count(row),
            })
            .collect();
        let tracks = payload_rows(&recordings, "recordings")
            .iter()
            .map(|row| Ranked {
                item: Track {
                    id: None,
                    name: value_string(row, "recording_name"),
                    artists: value_string(row, "artist_name"),
                    album: value_string(row, "release_name"),
                    playable: false,
                    ..Default::default()
                },
                count: value_count(row),
            })
            .collect();
        let albums = payload_rows(&releases, "release_groups")
            .iter()
            .map(|row| Ranked {
                item: Album {
                    name: value_string(row, "release_group_name"),
                    artists: value_string(row, "artist_name"),
                    ..Default::default()
                },
                count: value_count(row),
            })
            .collect();
        let plays = recordings
            .pointer("/payload/total_listen_count")
            .and_then(serde_json::Value::as_u64)
            .and_then(|count| usize::try_from(count).ok());
        Ok(ListeningStats { artists, tracks, albums, plays, ..Default::default() })
    }
}

async fn stats_request(account: &Account, entity: &str, range: &str) -> Result<serde_json::Value> {
    let answer = super::http()
        .get(format!("{}/1/stats/user/{}/{entity}", api(account), crate::escape::component(&account.name)))
        .query(&[("range", range), ("count", "100")])
        .header(AUTHORIZATION, format!("Token {}", account.session))
        .send()
        .await
        .context("cannot reach listenbrainz stats")?;
    let status = answer.status();
    if status.as_u16() == 204 {
        return Ok(serde_json::Value::Null);
    }
    let value: serde_json::Value = answer.json().await.context("cannot read listenbrainz stats")?;
    if !status.is_success() || value.get("error").is_some() {
        bail!("listenbrainz refused the stats request: {value}");
    }
    Ok(value)
}

fn payload_rows<'a>(value: &'a serde_json::Value, key: &str) -> &'a [serde_json::Value] {
    value.pointer(&format!("/payload/{key}")).and_then(serde_json::Value::as_array).map_or(&[], Vec::as_slice)
}

fn value_string(value: &serde_json::Value, key: &str) -> String {
    value.get(key).and_then(serde_json::Value::as_str).unwrap_or_default().to_owned()
}

fn value_count(value: &serde_json::Value) -> Option<usize> {
    value.get("listen_count").and_then(serde_json::Value::as_u64).and_then(|count| usize::try_from(count).ok())
}

/// Where this account's instance lives. An empty `server` means the public one.
fn api(account: &Account) -> &str {
    match account.server.is_empty() {
        true => API,
        false => account.server.trim_end_matches('/'),
    }
}

async fn submit(account: &Account, kind: &str, payload: &[Listen<'_>]) -> Result<()> {
    let answer = super::http()
        .post(format!("{}/1/submit-listens", api(account)))
        .header(AUTHORIZATION, format!("Token {}", account.session))
        .json(&Submission {
            listen_type: kind,
            payload,
        })
        .send()
        .await
        .context("cannot reach listenbrainz")?;

    if answer.status().is_success() {
        return Ok(());
    }
    let status = answer.status();
    let body = answer.text().await.unwrap_or_default();
    bail!("listenbrainz refused the request ({status}): {body}");
}

/// Builds one listen. `timed` is false for the now-playing report, which carries no start time.
fn listen(play: &Play, timed: bool) -> Listen<'_> {
    Listen {
        listened_at: timed.then(|| play.timestamp()),
        track_metadata: Metadata {
            artist_name: &play.artist,
            track_name: &play.title,
            release_name: play.release(),
            additional_info: Extra {
                duration_ms: play.duration.as_millis().min(u64::MAX as u128) as u64,
                media_player: CLIENT,
                submission_client: CLIENT,
            },
        },
    }
}

#[derive(Deserialize)]
struct Validation {
    valid: bool,
    user_name: Option<String>,
    message: Option<String>,
}

#[derive(Serialize)]
struct Submission<'a> {
    listen_type: &'a str,
    payload: &'a [Listen<'a>],
}

#[derive(Serialize)]
struct Listen<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    listened_at: Option<i64>,
    track_metadata: Metadata<'a>,
}

#[derive(Serialize)]
struct Metadata<'a> {
    artist_name: &'a str,
    track_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_name: Option<&'a str>,
    additional_info: Extra,
}

#[derive(Serialize)]
struct Extra {
    duration_ms: u64,
    media_player: &'static str,
    submission_client: &'static str,
}
