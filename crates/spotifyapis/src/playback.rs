//! Web player audio discovery and Widevine license transport.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use bytes::Bytes;
use reqwest::{Method, Url, header};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{Session, http, id};

const MP4_MANIFEST: &str = "file_ids_mp4";
const WEB_CLIENT_ID: &str = "d8a5ed958d274c2e8ee717e6a4b0971d";
const APP_VERSION: &str = "1.3.13.408.g393ff66413c5";
const SDK_VERSION: &str = "4.82.0";
const CLIENT_TOKEN_URL: &str = "https://clienttoken.spotify.com/v1/clienttoken";
const LICENSE_PREFIX: &str = "/widevine-license/v1/audio/license";
const LICENSE_ERROR_BODY_LIMIT: usize = 4096;
const LICENSE_ERROR_CODES: &[&str] = &[
    "invalid_token",
    "insufficient_scope",
    "invalid_client",
    "unauthorized_client",
    "invalid_request",
    "access_denied",
    "deprecated-version",
];
const CERTIFICATE_URL: &str =
    "https://spclient.wg.spotify.com/widevine-license/v1/application-certificate";

/// Spotify's assignment for the web player's AAC stream in an encrypted MP4.
#[derive(Deserialize)]
pub struct AudioFile {
    pub file_id: String,
    pub bitrate: Option<u32>,
    format: Option<FileFormat>,
}

/// The playable AAC files and catalog length of one web track.
pub struct TrackAudio {
    pub files: Vec<AudioFile>,
    pub duration: Option<Duration>,
}

/// The web manifest can represent its format ID as a number or a string.
#[derive(Deserialize)]
#[serde(untagged)]
enum FileFormat {
    Number(u32),
    Text(String),
}

impl AudioFile {
    /// Returns the storage resolver format when the manifest includes one.
    fn format(&self) -> Option<u32> {
        match self.format.as_ref()? {
            FileFormat::Number(format) => Some(*format),
            FileFormat::Text(format) => format.parse().ok(),
        }
    }

    /// Loads the binary Widevine initialization data from the public v2 seek table.
    pub async fn widevine_init(&self) -> Result<Vec<u8>> {
        anyhow::ensure!(
            valid_file_id(&self.file_id),
            "invalid Spotify audio file ID"
        );
        let endpoint = format!(
            "https://seektables.spotifycdn.com/v1/seektable/{}",
            self.file_id
        );
        let response = http::request(|client| client.get(&endpoint)).await?;
        let table: SeekTable = serde_json::from_slice(&http::response_bytes(response).await?)
            .context("cannot decode Spotify audio seek table")?;
        anyhow::ensure!(
            table.seektable_version == 2,
            "unsupported Spotify audio seek table"
        );
        let init = STANDARD
            .decode(table.pssh.widevine)
            .context("invalid Spotify Widevine PSSH")?;
        anyhow::ensure!(!init.is_empty(), "Spotify Widevine PSSH is empty");
        Ok(init)
    }
}

/// The v2 seek table carries DRM initialization data alongside segment offsets.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SeekTable {
    seektable_version: u32,
    pssh: ProtectionHeaders,
}

/// DRM initialization data keyed by Content Decryption Module type.
#[derive(Deserialize)]
struct ProtectionHeaders {
    widevine: String,
}

/// The media endpoint indexes descriptors by their requested Spotify URI.
#[derive(Deserialize)]
struct MediaResponse {
    media: HashMap<String, MediaEntry>,
}

/// The descriptor envelope returned for one track.
#[derive(Deserialize)]
struct MediaEntry {
    item: MediaItem,
}

/// The available manifests grouped by Spotify's file family names.
#[derive(Deserialize)]
struct MediaItem {
    manifest: HashMap<String, Vec<AudioFile>>,
    metadata: Option<MediaMetadata>,
}

/// Catalog fields carried with a web playback manifest.
#[derive(Deserialize)]
struct MediaMetadata {
    duration: Option<u64>,
}

/// Signed CDN locations returned by the JSON storage resolver.
#[derive(Deserialize)]
struct CdnResponse {
    cdnurl: Vec<String>,
}

/// The relative license endpoint selected by the Melody service.
#[derive(Deserialize)]
struct LicenseResponse {
    uri: String,
}

/// A Spotify audio license endpoint validated to stay on the API host.
pub struct AudioLicense {
    path: String,
}

/// The license endpoint rejected a Widevine exchange with HTTP 403.
#[derive(Debug)]
pub struct LicenseRefused {
    reason: &'static str,
    response: &'static str,
    http_version: reqwest::Version,
    oauth_matches_web_client: bool,
    challenge_bytes: usize,
}

impl std::fmt::Display for LicenseRefused {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Spotify refused the Widevine license request (reason: {}, response: {}, transport: {:?}, session OAuth client matches web client: {}, challenge: {} bytes)",
            self.reason,
            self.response,
            self.http_version,
            self.oauth_matches_web_client,
            self.challenge_bytes,
        )
    }
}

impl std::error::Error for LicenseRefused {}

/// Spotify web audio requests for one signed-in account and device. The OAuth
/// session renews its bearer and the client token is refreshed independently.
pub struct Playback {
    session: Arc<Session>,
    device_id: String,
    client_token: Mutex<Option<ClientToken>>,
}

/// A client token and the time at which the grant should be renewed.
struct ClientToken {
    value: header::HeaderValue,
    refresh_at: Instant,
}

/// The web client's unauthenticated JSON request for a client token.
#[derive(Serialize)]
struct ClientTokenRequest<'a> {
    client_data: ClientData<'a>,
}

/// Identifies the app and device making a client-token request.
#[derive(Serialize)]
struct ClientData<'a> {
    client_version: &'static str,
    client_id: &'static str,
    js_sdk_data: DeviceData<'a>,
}

/// Device details carried by the web player's client-token request.
#[derive(Serialize)]
struct DeviceData<'a> {
    device_brand: &'static str,
    device_model: &'static str,
    os: &'static str,
    os_version: &'static str,
    device_id: &'a str,
    device_type: &'static str,
}

/// Either a granted token or a response without a usable grant.
#[derive(Deserialize)]
struct ClientTokenResponse {
    granted_token: Option<ClientGrant>,
}

/// Token material and the service's lifetime and domain restrictions.
#[derive(Deserialize)]
struct ClientGrant {
    token: String,
    expires_after_seconds: u64,
    refresh_after_seconds: u64,
    domains: Vec<ClientDomain>,
}

/// One domain the client token is valid for.
#[derive(Deserialize)]
struct ClientDomain {
    domain: String,
}

impl Playback {
    pub fn new(session: Arc<Session>, device_id: String) -> Self {
        Self {
            session,
            device_id,
            client_token: Mutex::default(),
        }
    }

    /// Looks up the AAC files and duration offered by the web playback service.
    pub async fn track(&self, track_id: &str) -> Result<TrackAudio> {
        anyhow::ensure!(id::raw_id(track_id).is_some(), "invalid Spotify track ID");
        let uri = id::track_uri(track_id);
        let path = format!("/track-playback/v1/media/{uri}?manifestFileFormat={MP4_MANIFEST}");
        let mut headers = self.headers().await?;
        headers.insert(
            "app-platform",
            header::HeaderValue::from_static("WebPlayer"),
        );
        headers.insert(
            "spotify-app-version",
            header::HeaderValue::from_static(APP_VERSION),
        );
        let body = self
            .request(Method::GET, &path, headers, Bytes::new())
            .await?;
        let mut response: MediaResponse =
            serde_json::from_slice(&body).context("cannot decode Spotify web audio manifest")?;
        let mut item = response
            .media
            .remove(&uri)
            .map(|entry| entry.item)
            .context("Spotify omitted the web audio manifest")?;
        let files = item
            .manifest
            .remove(MP4_MANIFEST)
            .unwrap_or_default()
            .into_iter()
            .filter(|file| file.format().is_some() && valid_file_id(&file.file_id))
            .collect();
        let duration = item
            .metadata
            .and_then(|metadata| metadata.duration)
            .map(Duration::from_millis);
        Ok(TrackAudio { files, duration })
    }

    /// Resolves signed CDN URLs for the web player's encrypted MP4.
    pub async fn audio_urls(&self, file: &AudioFile) -> Result<Vec<Url>> {
        anyhow::ensure!(
            valid_file_id(&file.file_id),
            "invalid Spotify audio file ID"
        );
        let file_id = &file.file_id;
        let format = file
            .format()
            .context("Spotify omitted the web audio file format")?;
        let path = format!(
            "/storage-resolve/v2/files/audio/interactive/{format}/{file_id}?version=10000000&product=9&platform=39&alt=json"
        );
        let response: CdnResponse = self.json(&path).await?;
        let urls: Vec<_> = response
            .cdnurl
            .iter()
            .filter_map(|url| Url::parse(url).ok())
            .filter(|url| {
                url.scheme() == "https" && url.username().is_empty() && url.password().is_none()
            })
            .collect();
        anyhow::ensure!(!urls.is_empty(), "Spotify returned no web audio CDN URLs");
        Ok(urls)
    }

    /// Discovers the regional Widevine audio license endpoint.
    pub async fn license_url(&self) -> Result<AudioLicense> {
        let path = format!(
            "/melody/v1/license_url?keysystem=com.widevine.alpha&mediatype=audio&sdk_name=harmony&sdk_version={SDK_VERSION}"
        );
        let response: LicenseResponse = self.json(&path).await?;
        let relative = response.uri.strip_prefix('/').unwrap_or(&response.uri);
        let (location, _) = relative.split_once('?').unwrap_or((relative, ""));
        anyhow::ensure!(
            location == LICENSE_PREFIX.trim_start_matches('/') && !relative.contains('#'),
            "Spotify returned an unexpected audio license endpoint"
        );
        Ok(AudioLicense {
            path: format!("/{relative}"),
        })
    }

    /// Fetches the Widevine application certificate for a CDM session.
    pub async fn application_certificate(&self) -> Result<Bytes> {
        let headers = self.headers().await?;
        let response =
            http::request(|client| client.get(CERTIFICATE_URL).headers(headers.clone())).await?;
        http::response_bytes(response).await
    }

    /// Exchanges the CDM's binary challenge for the corresponding binary license.
    pub async fn license(&self, endpoint: &AudioLicense, challenge: Bytes) -> Result<Bytes> {
        anyhow::ensure!(!challenge.is_empty(), "Widevine challenge is empty");
        let challenge_bytes = challenge.len();
        let response = http::send_response(
            &self.session,
            Method::POST,
            &endpoint.path,
            self.headers().await?,
            challenge,
        )
        .await?;
        if response.status() == reqwest::StatusCode::FORBIDDEN
            && let Err(error) = response.error_for_status_ref()
        {
            let http_version = response.version();
            let (reason, response) = license_error_response(response).await;
            return Err(
                anyhow::Error::new(error.without_url()).context(LicenseRefused {
                    reason,
                    response,
                    http_version,
                    oauth_matches_web_client: self.session.client_id() == WEB_CLIENT_ID,
                    challenge_bytes,
                }),
            );
        }
        http::response_bytes(response).await
    }

    async fn json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let bytes = self
            .request(Method::GET, path, self.headers().await?, Bytes::new())
            .await?;
        serde_json::from_slice(&bytes)
            .with_context(|| format!("cannot decode Spotify web response for {path}"))
    }

    async fn headers(&self) -> Result<header::HeaderMap> {
        let mut headers = header::HeaderMap::new();
        headers.insert("client-token", self.client_token().await?);
        headers.insert(
            header::ORIGIN,
            header::HeaderValue::from_static("https://open.spotify.com"),
        );
        headers.insert(
            header::REFERER,
            header::HeaderValue::from_static("https://open.spotify.com/"),
        );
        Ok(headers)
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        headers: header::HeaderMap,
        body: Bytes,
    ) -> Result<Bytes> {
        http::send(&self.session, method, path, headers, body).await
    }

    async fn client_token(&self) -> Result<header::HeaderValue> {
        let mut cached = self.client_token.lock().await;
        if let Some(token) = cached
            .as_ref()
            .filter(|token| Instant::now() < token.refresh_at)
        {
            return Ok(token.value.clone());
        }
        let request = ClientTokenRequest {
            client_data: ClientData {
                client_version: APP_VERSION,
                client_id: WEB_CLIENT_ID,
                js_sdk_data: DeviceData {
                    device_brand: "unknown",
                    device_model: "unknown",
                    os: std::env::consts::OS,
                    os_version: "unknown",
                    device_id: &self.device_id,
                    device_type: "computer",
                },
            },
        };
        let response = http::request(|client| {
            client
                .post(CLIENT_TOKEN_URL)
                .header(header::ACCEPT, "application/json")
                .json(&request)
        })
        .await
        .context("cannot request Spotify playback client token")?;
        let response: ClientTokenResponse =
            serde_json::from_slice(&http::response_bytes(response).await?)
                .context("cannot decode Spotify playback client token response")?;
        let grant = response
            .granted_token
            .context("Spotify did not grant a playback client token")?;
        anyhow::ensure!(
            !grant.token.is_empty()
                && grant.expires_after_seconds > 0
                && grant
                    .domains
                    .iter()
                    .any(|domain| domain.domain.trim_start_matches('.') == "spotify.com"),
            "Spotify granted an unusable playback client token"
        );
        let lifetime = grant
            .refresh_after_seconds
            .max(1)
            .min(grant.expires_after_seconds);
        let refresh_at = Instant::now()
            .checked_add(Duration::from_secs(lifetime))
            .context("Spotify returned an invalid playback client token lifetime")?;
        let mut value = header::HeaderValue::from_str(&grant.token)?;
        value.set_sensitive(true);
        *cached = Some(ClientToken {
            value: value.clone(),
            refresh_at,
        });
        Ok(value)
    }
}

fn valid_file_id(file_id: &str) -> bool {
    file_id.len() == 40 && file_id.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// Extracts only fixed diagnostic labels from a forbidden response.
async fn license_error_response(response: reqwest::Response) -> (&'static str, &'static str) {
    let auth_reason = response
        .headers()
        .get_all(header::WWW_AUTHENTICATE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find_map(|value| {
            value.split([',', ' ']).find_map(|parameter| {
                known_license_error(parameter.strip_prefix("error=")?.trim_matches('"'))
            })
        });
    let Ok(body) =
        http::response_body(response, LICENSE_ERROR_BODY_LIMIT, LICENSE_ERROR_BODY_LIMIT).await
    else {
        return (auth_reason.unwrap_or("unreported"), "unavailable");
    };
    if body.is_empty() {
        return (auth_reason.unwrap_or("unreported"), "empty");
    }
    if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&body) {
        let reason = auth_reason.or_else(|| {
            ["/code", "/error/code", "/error", "/reason"]
                .into_iter()
                .find_map(|field| known_license_error(value.pointer(field)?.as_str()?))
        });
        return (reason.unwrap_or("unreported"), "JSON");
    }
    let kind = match std::str::from_utf8(&body).is_ok() {
        true => "text",
        false => "binary",
    };
    (auth_reason.unwrap_or("unreported"), kind)
}

fn known_license_error(value: &str) -> Option<&'static str> {
    LICENSE_ERROR_CODES
        .iter()
        .copied()
        .find(|code| *code == value)
}
