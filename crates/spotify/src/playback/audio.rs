//! Feeds Spotify's web AAC transport into Sonora's CENC reader and decoder.

use std::time::Duration;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use engine::Fetch;
use engine::cenc::Media;
use music::drm::{self, Cdm};
use music::{Encoding, Offer, Quality};
use sound::aac::Aac;
use sound::decode::{self, Decoded};
use sound::ranged::{Address, Resuming};
use sound::trim;

use crate::session::Session;

const MAX_RANGE: u64 = 2 * 1024 * 1024;
const MAX_DOWNLOAD: u64 = 256 * 1024 * 1024;

/// Playback quality choices for the two AAC variants offered in web manifests.
pub(in crate::playback) const QUALITIES: &[Offer] = &[
    Offer {
        quality: Quality::High,
        encoding: Encoding::Aac { kbps: 256 },
        up_to: true,
    },
    Offer {
        quality: Quality::Normal,
        encoding: Encoding::Aac { kbps: 128 },
        up_to: false,
    },
];

/// Loads a web track using the Spotify protocol and the shared audio engine.
pub(super) struct Audio {
    pub(super) session: Session,
    pub(super) quality: Quality,
}

/// A licensed AAC download and its playback metadata.
#[derive(Clone)]
pub(super) struct Loaded {
    media: Media,
    duration: Option<Duration>,
    encoding: Option<Encoding>,
    edit: Option<trim::Trim>,
}

#[async_trait]
impl Fetch for Audio {
    type Loaded = Loaded;

    fn name(&self) -> &'static str {
        "spotify"
    }

    fn plays_offline(&self) -> bool {
        false
    }

    fn gated(&self, error: &anyhow::Error) -> bool {
        matches!(
            error.downcast_ref::<spotifyapis::auth::AuthError>(),
            Some(spotifyapis::auth::AuthError::Credentials)
        ) || spotifyapis::http_status(error) == Some(reqwest::StatusCode::UNAUTHORIZED)
    }

    fn refused(&self, error: &anyhow::Error) -> bool {
        error.is::<spotifyapis::playback::LicenseRefused>()
    }

    async fn load(&self, id: &str) -> Result<Loaded> {
        drm::require()?;
        let api = self.session.playback();
        let track = api.track(id).await?;
        let target: u32 = match self.quality {
            Quality::Low | Quality::Normal => 128_000,
            _ => 256_000,
        };
        let file = track
            .files
            .into_iter()
            .min_by_key(|file| file.bitrate.map_or(u32::MAX, |rate| rate.abs_diff(target)))
            .context("Spotify offered no web AAC file")?;
        let encoding = file.bitrate.map(|rate| Encoding::Aac { kbps: rate / 1000 });
        let urls = api.audio_urls(&file).await?;
        let response = open_file(urls).await?;
        let total = response.total();
        let mut media = Media::pulling(response, total);
        let license = async {
            let (init, certificate, endpoint) = tokio::try_join!(
                file.widevine_init(),
                api.application_certificate(),
                api.license_url(),
            )?;
            let (cdm, challenge) = tokio::task::spawn_blocking(move || -> Result<_> {
                let cdm = Cdm::open_certified(&certificate)?;
                let challenge = cdm.challenge(&init)?;
                Ok((cdm, challenge))
            })
            .await
            .context("cannot create Spotify Widevine challenge")??;
            let license = api.license(&endpoint, challenge.into()).await?;
            tokio::task::spawn_blocking(move || -> Result<Cdm> {
                cdm.accept(&license)?;
                Ok(cdm)
            })
            .await
            .context("cannot apply Spotify Widevine license")?
        };
        let (cdm, key_id) = tokio::try_join!(license, media.key_id())?;
        media.license(cdm, key_id.to_vec())?;
        media.release_when_downloaded();
        media.prime().await?;
        let edit = media.header().as_deref().and_then(trim::from_mp4);
        Ok(Loaded {
            media,
            duration: edit.and_then(|edit| edit.take).or(track.duration),
            encoding,
            edit,
        })
    }

    fn encoding(&self, loaded: &Loaded) -> Option<Encoding> {
        loaded.encoding
    }

    fn length(&self, loaded: &Loaded) -> Option<Duration> {
        loaded.duration
    }

    async fn downloaded(&self, loaded: &Loaded) {
        loaded.media.finished().await;
    }

    fn progress(&self, loaded: &Loaded) -> Option<f32> {
        loaded.media.progress()
    }

    fn reopen_to_seek(&self) -> bool {
        true
    }

    fn open(&self, id: &str, loaded: &Loaded, at: Duration) -> Option<Decoded> {
        let priming = loaded.edit.map_or(Duration::ZERO, |edit| edit.skip);
        let (reader, skip) = if at.is_zero() {
            (loaded.media.reader(), priming)
        } else {
            let target = at.checked_add(priming)?;
            let entry = loaded.media.entry(target)?;
            (loaded.media.reader_at(entry), entry.into)
        };
        let decoder = rodio::Decoder::builder()
            .with_symphonia_decoder::<Aac>()
            .with_data(decode::Media::new(reader))
            .with_seekable(false)
            .build()
            .inspect_err(|error| log::warn!("spotify: cannot decode web track {id}: {error}"))
            .ok()?;
        let take = loaded.duration.map(|duration| duration.saturating_sub(at));
        Some(Decoded::trimmed(decoder, skip, take))
    }
}

/// Opens one signed CDN address without attaching the web session's credentials.
async fn open_file(urls: Vec<reqwest::Url>) -> Result<Resuming> {
    let http = spotifyapis::media_client()?;
    for url in urls {
        let address = Address::new("spotify", http.clone(), url.into()).capped(MAX_RANGE);
        if let Ok(file) = Resuming::open(address).await
            && file.total().is_some_and(|total| total <= MAX_DOWNLOAD)
        {
            return Ok(file);
        }
    }
    anyhow::bail!("cannot open Spotify web audio on its CDN")
}
