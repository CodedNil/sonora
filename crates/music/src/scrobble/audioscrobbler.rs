use std::time::Duration;

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use md5::{Digest, Md5};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use super::{Account, Link, Play, Secret, Service};
use crate::{Album, ListeningStats, Ranked, SavedArtist, StatsPeriod, Track};
use crate::escape;

/// The loopback address the browser hands the token back on.
const PORT: u16 = 8990;
const PATH: &str = "/scrobble";
/// How long the user has to approve in the browser before the link is given up on.
const WAIT: Duration = Duration::from_secs(300);
/// How many listens last.fm's `track.scrobble` takes in one call.
const LASTFM_BATCH: usize = 50;
const STATS_PERIODS: &[StatsPeriod] = &[
    StatsPeriod::Weeks,
    StatsPeriod::Months,
    StatsPeriod::Year,
    StatsPeriod::AllTime,
];

const PAGE: &str = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body>Sonora is connected. You can close this window.</body></html>";

/// Libre.fm issues no API accounts, so the key and secret its protocol still asks for are ours
/// and constant.
const ANONYMOUS: &str = "sonora";

/// The last.fm 2.0 API. Libre.fm runs GNU FM, which answers the same methods at its own urls, so
/// the two services are one type with different addresses.
pub struct AudioScrobbler {
    id: &'static str,
    endpoint: &'static str,
    authorize: &'static str,
    signup: Option<&'static str>,
    link: Link,
    /// How many listens one `track.scrobble` carries. One means the server only implements the
    /// single unindexed form, which is what GNU FM does.
    batch: usize,
}

pub const LASTFM: AudioScrobbler = AudioScrobbler {
    id: "lastfm",
    endpoint: "https://ws.audioscrobbler.com/2.0/",
    authorize: "https://www.last.fm/api/auth/",
    signup: Some("https://www.last.fm/api/account/create"),
    link: Link::Keys,
    batch: LASTFM_BATCH,
};

pub const LIBREFM: AudioScrobbler = AudioScrobbler {
    id: "librefm",
    endpoint: "https://libre.fm/2.0/",
    authorize: "https://libre.fm/api/auth/",
    signup: None,
    link: Link::Browser,
    batch: 1,
};

#[async_trait]
impl Service for AudioScrobbler {
    fn id(&self) -> &'static str {
        self.id
    }

    fn link(&self) -> Link {
        self.link
    }

    fn signup(&self) -> Option<&'static str> {
        self.signup
    }

    async fn connect(&self, secret: Secret) -> Result<Account> {
        let (key, shared) = match secret {
            Secret::Keys { key, secret } => (key.trim().to_owned(), secret.trim().to_owned()),
            Secret::None => (ANONYMOUS.to_owned(), ANONYMOUS.to_owned()),
            _ => bail!("{} needs an api key and a secret", self.id),
        };
        if key.is_empty() || shared.is_empty() {
            bail!("{} needs an api key and a secret", self.id);
        }

        let listener = TcpListener::bind(("127.0.0.1", PORT))
            .await
            .context("cannot listen for the scrobbling callback")?;
        open::that_in_background(authorize_url(self.authorize, &key));

        let token = token(listener).await?;
        let form = vec![
            ("method".to_owned(), "auth.getSession".to_owned()),
            ("api_key".to_owned(), key.clone()),
            ("token".to_owned(), token),
        ];
        let answer = post(self.endpoint, &shared, form).await?;
        let Some(granted) = answer.session else {
            bail!("{} returned no session", self.id);
        };

        Ok(Account {
            key,
            secret: shared,
            session: granted.key,
            name: granted.name,
            enabled: true,
            ..Account::default()
        })
    }

    async fn now_playing(&self, account: &Account, play: &Play) -> Result<()> {
        let mut form = vec![
            ("method".to_owned(), "track.updateNowPlaying".to_owned()),
            ("artist".to_owned(), play.artist.clone()),
            ("track".to_owned(), play.title.clone()),
            ("duration".to_owned(), play.duration.as_secs().to_string()),
        ];
        if let Some(album) = play.release() {
            form.push(("album".to_owned(), album.to_owned()));
        }
        self.call(account, form).await
    }

    async fn scrobble(&self, account: &Account, plays: &[Play]) -> Result<()> {
        for batch in plays.chunks(self.batch) {
            let mut form = vec![("method".to_owned(), "track.scrobble".to_owned())];
            for (index, play) in batch.iter().enumerate() {
                form.push((self.slot("artist", index), play.artist.clone()));
                form.push((self.slot("track", index), play.title.clone()));
                form.push((self.slot("timestamp", index), play.timestamp().to_string()));
                form.push((
                    self.slot("duration", index),
                    play.duration.as_secs().to_string(),
                ));
                if let Some(album) = play.release() {
                    form.push((self.slot("album", index), album.to_owned()));
                }
            }
            self.call(account, form).await?;
        }
        Ok(())
    }

    fn stats_periods(&self) -> &'static [StatsPeriod] {
        STATS_PERIODS
    }

    async fn stats(&self, account: &Account, period: StatsPeriod) -> Result<ListeningStats> {
        let range = match period {
            StatsPeriod::Weeks => "7day",
            StatsPeriod::Months => "6month",
            StatsPeriod::Year => "12month",
            StatsPeriod::AllTime => "overall",
        };
        let key = match account.key.is_empty() {
            true => ANONYMOUS,
            false => &account.key,
        };
        let (artists, tracks, albums) = tokio::try_join!(
            self.stats_call("user.getTopArtists", account, key, range),
            self.stats_call("user.getTopTracks", account, key, range),
            self.stats_call("user.getTopAlbums", account, key, range),
        )?;
        let artists = rows(&artists, "topartists", "artist")
            .into_iter()
            .map(|row| Ranked {
                item: SavedArtist {
                    name: string(row, "name"),
                    ..Default::default()
                },
                count: number(row, "playcount"),
            })
            .collect();
        let tracks = rows(&tracks, "toptracks", "track")
            .into_iter()
            .map(|row| {
                let artist = row
                    .get("artist")
                    .and_then(|artist| artist.get("name").or(Some(artist)))
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_owned();
                Ranked {
                    item: Track {
                        name: string(row, "name"),
                        artists: artist,
                        album: row
                            .get("album")
                            .and_then(|album| album.get("#text"))
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or_default()
                            .to_owned(),
                        playable: false,
                        ..Default::default()
                    },
                    count: number(row, "playcount"),
                }
            })
            .collect();
        let albums = rows(&albums, "topalbums", "album")
            .into_iter()
            .map(|row| Ranked {
                item: Album {
                    name: string(row, "name"),
                    artists: row
                        .get("artist")
                        .and_then(|artist| artist.get("name").or(Some(artist)))
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or_default()
                        .to_owned(),
                    ..Default::default()
                },
                count: number(row, "playcount"),
            })
            .collect();
        Ok(ListeningStats { artists, tracks, albums, ..Default::default() })
    }
}

impl AudioScrobbler {
    async fn stats_call(
        &self,
        method: &str,
        account: &Account,
        key: &str,
        period: &str,
    ) -> Result<serde_json::Value> {
        let answer = super::http()
            .get(self.endpoint)
            .query(&[
                ("method", method),
                ("user", account.name.as_str()),
                ("api_key", key),
                ("format", "json"),
                ("period", period),
                ("limit", "50"),
            ])
            .send()
            .await
            .context("cannot reach scrobbling stats")?;
        let status = answer.status();
        let value: serde_json::Value = answer
            .json()
            .await
            .context("cannot read scrobbling stats")?;
        if !status.is_success() || value.get("error").is_some() {
            bail!("{} refused the stats request: {}", self.id, value);
        }
        Ok(value)
    }
}

fn rows<'a>(value: &'a serde_json::Value, parent: &str, child: &str) -> Vec<&'a serde_json::Value> {
    value
        .get(parent)
        .and_then(|value| value.get(child))
        .map(|items| match items {
            serde_json::Value::Array(items) => items.iter().collect(),
            serde_json::Value::Object(_) => vec![items],
            _ => Vec::new(),
        })
        .unwrap_or_default()
}

fn string(value: &serde_json::Value, key: &str) -> String {
    value.get(key).and_then(serde_json::Value::as_str).unwrap_or_default().to_owned()
}

fn number(value: &serde_json::Value, key: &str) -> Option<usize> {
    value.get(key).and_then(|number| number.as_str().and_then(|s| s.parse().ok()).or_else(|| number.as_u64().map(|n| n as usize)))
}

impl AudioScrobbler {
    /// Names one listen's parameter. A batching server numbers them, a single-listen one takes
    /// the plain name and would read a numbered parameter as an array.
    fn slot(&self, name: &str, index: usize) -> String {
        match self.batch > 1 {
            true => format!("{name}[{index}]"),
            false => name.to_owned(),
        }
    }

    async fn call(&self, account: &Account, mut form: Vec<(String, String)>) -> Result<()> {
        form.push(("api_key".to_owned(), account.key.clone()));
        form.push(("sk".to_owned(), account.session.clone()));
        post(self.endpoint, &account.secret, form).await.map(|_| ())
    }
}

#[derive(Deserialize)]
struct Answer {
    error: Option<u32>,
    message: Option<String>,
    session: Option<Granted>,
}

#[derive(Deserialize)]
struct Granted {
    key: String,
    name: String,
}

fn authorize_url(authorize: &str, key: &str) -> String {
    format!(
        "{authorize}?api_key={}&cb={}",
        escape::component(key),
        escape::component(&format!("http://127.0.0.1:{PORT}{PATH}"))
    )
}

/// Waits for the browser to come back with the request token. Anything else that reaches the
/// listener is answered and ignored, so a stray favicon fetch does not end the wait.
async fn token(listener: TcpListener) -> Result<String> {
    let deadline = tokio::time::Instant::now() + WAIT;

    loop {
        let accepted = tokio::time::timeout_at(deadline, listener.accept())
            .await
            .context("the browser did not come back in time")?;
        let (mut stream, _) = accepted.context("cannot accept the scrobbling callback")?;

        let mut line = String::new();
        let read = BufReader::new(&mut stream).read_line(&mut line).await;
        if let Err(error) = read {
            log::warn!("scrobble: cannot read the callback: {error}");
            continue;
        }
        let found = parameter(&line, "token");
        stream.write_all(PAGE.as_bytes()).await.ok();
        if let Some(token) = found {
            return Ok(token);
        }
    }
}

/// Signs the form the way the protocol asks, posts it and turns a refusal into an error.
async fn post(endpoint: &str, secret: &str, mut form: Vec<(String, String)>) -> Result<Answer> {
    form.sort_by(|left, right| left.0.cmp(&right.0));
    form.push(("api_sig".to_owned(), signature(&form, secret)));
    form.push(("format".to_owned(), "json".to_owned()));

    let answer: Answer = super::http()
        .post(endpoint)
        .form(&form)
        .send()
        .await
        .context("cannot reach the scrobbling service")?
        .json()
        .await
        .context("cannot read the scrobbling answer")?;

    if let Some(code) = answer.error {
        let message = answer.message.unwrap_or_default();
        bail!("the scrobbling service refused the request ({code}): {message}");
    }
    Ok(answer)
}

fn signature(form: &[(String, String)], secret: &str) -> String {
    let mut base = String::new();
    for (name, value) in form {
        base.push_str(name);
        base.push_str(value);
    }
    base.push_str(secret);
    format!("{:x}", Md5::digest(base.as_bytes()))
}

fn parameter(request: &str, name: &str) -> Option<String> {
    let (_, query) = request.split_whitespace().nth(1)?.split_once('?')?;
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key == name).then(|| percent_decode_str(value).decode_utf8_lossy().into_owned())
    })
}
