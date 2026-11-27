# Spotify playback research

## Current state

- **Active direction:** Web/Widevine only. The desktop transport trial was removed from `spotifyapis`. The Linux capture findings and offline tools are retained below.
- **Sonora playback:** client-token acquisition succeeds, but Spotify returned HTTP 403 on the license POST. Sonora uses a desktop OAuth bearer with a web client token. Firefox's successful requests used a web-issued bearer. Their CDM challenges matched in structure and client identification, so the bearer is the strongest unverified difference.
- **Current blocker:** the license POST returned an empty HTTP 403 even with the matched bearer and client token used together in a successful Firefox request. Both credential substitutions failed. Sonora's diagnostic overrides were removed after these comparisons.
- **Reporting:** the desktop capture records Connect state, resume-point writes, and stream events. These are research leads. We have not implemented new reporting or established which writes count toward history, recommendations, or Wrapped.

## Web findings from the public bundle

Source: [Spotify web vendor bundle](https://open.spotifycdn.com/cdn/build/web-player/vendor~web-player.4ac0a26d.js), inspected without account credentials. The paths and JSON fields below are static observations. No playback endpoints were invoked.

1. `GET /track-playback/v1/media/{spotify:track:…}?manifestFileFormat=file_ids_mp4`. The AAC file comes from `media[uri].item.manifest.file_ids_mp4[]`.
2. `GET /storage-resolve/v2/files/audio/interactive/{format}/{file_id}?version=10000000&product=9&platform=39&alt=json`. The reply provides `cdnurl`. The bundle uses the legacy-shaped path without `v2` and the format segment when it has no format ID.
3. `GET https://seektables.spotifycdn.com/v1/seektable/{file_id}`. The bundle expects `seektableVersion: 2` and decodes `pssh.widevine` for the normal CENC route.
4. The web player fetches `/widevine-license/v1/application-certificate` and calls EME's `setServerCertificate`, continuing if that step fails. Sonora can now install a service certificate on a separate CDM host before generating its challenge. Spotify acceptance is unverified.
5. `GET /melody/v1/license_url?keysystem=com.widevine.alpha&mediatype=audio&sdk_name=…&sdk_version=…` returns a relative `uri`. The web player sends the binary CDM message to that route. Its production fallback is `/widevine-license/v1/audio/license`. Whether a standalone system CDM is accepted by Spotify remains unverified.
6. The web bundle asks for MP4 FLAC only when its lossless capability and DRM robustness checks pass. The first Sonora web transport targets AAC.
7. The vendor bundle posts JSON at `https://clienttoken.spotify.com/v1/clienttoken`. Its `client_data` includes `client_version`, `client_id`, and `js_sdk_data` with device and platform metadata. It reads `granted_token.token`, permitted domains, and the refresh interval. The main web bundle uses same-origin `/api/token` and `/api/server-time`. Which existing Sonora grants can replace that flow remains open.

## Desktop Widevine

- Static inspection of the installed Linux 1.2.95 build's `Apps/xpui/xpui-modules.js` found the browser EME path in Harmony. The desktop integration constructs Harmony with a video container and subscribes to native commands over `sp://videoplayer/v1/commands`. This identifies a desktop video integration. The same bundle includes generic web AAC and MP4 FLAC resolution code, but the capture and the video coordinator do not establish that the desktop uses it for standalone music.
- The shared Harmony license resolver builds the same `/melody/v1/license_url` request shape as the Firefox bundle. Its fallback is `/widevine-license/v1/{media-type}/license`. On the video path, Harmony can instead take `encryption_data` and `license_server_endpoint` directly from the manifest's matching `encryption_infos` entry.
- Both the JSONL and the raw proxy capture contain only an HTTP 200 **OPTIONS** preflight for the Widevine application certificate. The preflight asks to allow a GET, but the GET itself is absent. There is no captured Melody license discovery or Widevine audio/video license POST in the Linux session.
- The raw capture contains seven successful `GET /manifests/v9/json/sources/{id}/options/supports_drm` replies. All seven list H.264 or VP9 video profiles, zero audio profiles, and empty `encryption_infos`. These are clear video manifests. The music streams in the same session used PlayPlay.
- The seven video manifest requests all used the bearer issued by the desktop OAuth refresh and the same client token as the native PlayPlay requests. The video requests came from the embedded UI origin. This confirms that Harmony can reuse the desktop session for manifest access. The captured video was clear, so license acceptance is still an open question.
- Run `node scripts/desktop-widevine.mjs events.jsonl` to compare another capture locally. It reports response counts, media protection, and credential matches without printing credentials, request bodies, or file identifiers.
- A protected music video played through the official Linux app would be the next capture to confirm the live desktop video license route and credential source. It would not by itself establish a Widevine route for ordinary songs. No application launch or signed-in request was needed for the findings above.

## Failed attempts and limits

- A default curl request loaded the mobile site. A desktop browser user agent exposed the desktop bundles.
- The installed Nix launcher is a shell wrapper. The underlying ELF has dynamic symbols but no exported PlayPlay unwrap symbol, so name-based lookup did not identify the implementation.
- The Linux capture's PlayPlay token bytes occur once in the installed ELF. For the inspected 1.2.95 build, the getter is at `0x1548fd0`, next to getters for version and token length. Direct call sites are around `0x149851e`, `0x1504027`, and `0x1536e1d`. The endpoint is absent as one literal string. These observations locate request identity but leave the unwrap unresolved.
- The two request-building call sites at `0x149851e` and `0x1504027` both create the same object before reading the version and token. The third caller formats the token. The trace has not reached a verified consumer of the response fields.
- `node scripts/desktop-fixtures.mjs raw_flows.mitm` matches responses by audio file ID and extracts two complete offline fixtures into a private temporary directory. Each has a 24-byte PlayPlay response, a 3 MiB encrypted CDN range from offset zero, and a plaintext Ogg head of 199,381 or 186,588 bytes. Using the wrapped field verbatim as an AES-CTR key failed both full-prefix and nonzero-offset comparisons. A candidate unwrap still needs to pass both fixtures.
- The first desktop round-trip check incorrectly assumed the first captured PlayPlay request was interactive. It was prefetch. After selecting both modes explicitly, the fixture verified the request bytes, wrapped response, signed CDN response, and stereo metadata containing Vorbis and FLAC. No PlayPlay request was sent to Spotify.
- The original Linux capture continued to roughly ten minutes after the stop command failed. Normal quit did not stop the client, which was force-closed. The proxy preference was restored. Five early TLS handshakes failed before retries succeeded. The AP connection remained encrypted.
- The desktop capture streamed Vorbis at 160 kbps. No pause, remote-device command, FLAC stream, web license exchange, or client-token grant was captured.
- The first offline decoder open failed because the captured MP4 lists two AAC sample descriptions and Symphonia accepts one. Both descriptions in the sample have the same AAC configuration. The shared reader now presents one description while preserving byte offsets. A second offline check decoded one second of the captured clear fragment at 44.1 kHz.
- The local host initially missed Firefox’s Widevine module because this profile lives under the XDG config directory. After adding that profile root to discovery, Sonora found the module automatically, applied the captured application certificate, and generated a nonempty challenge from the captured PSSH offline. The challenge was never sent to Spotify.
- The first Sonora playback attempt stopped at client-token acquisition with HTTP 400. An anonymous comparison with fresh device IDs reproduced the 400 when the web JSON request used Sonora's desktop OAuth client ID. The public web player client ID from the Firefox HAR yielded HTTP 200 and a grant. Sonora now uses that web client ID for the client token and keeps its existing OAuth bearer for authenticated requests. The service has not been tested with that pair of credentials.

## Sonora setup

Sonora renews its OAuth bearer using its existing Spotify sign-in and obtains the playback client token itself. A system Widevine module is required. The browser HAR used a web-issued bearer, so acceptance of the existing Sonora OAuth grant is still an open integration check. The manual credential-file experiment was removed.

## Firefox playback capture

- The web player successfully obtained a web access token, a JSON client-token grant, and a Widevine application certificate. The certificate preflight went to the global spclient host with `client-token` and no bearer. Three audio license POSTs with a bearer and a client token returned HTTP 200. The browser sent binary license payloads without a `Content-Type` header. Melody advertised its audio license through the SDK identity `harmony`, version `4.82.0`.
- Two media responses carry JSON under `media[uri].item.manifest`. The web build requested several families, including `file_ids_mp4` and `file_ids_mp4_dual`. AAC formats are encoded as strings in that response. It advertised 256 kbps AAC, and the requests in this capture streamed format 10. No lossless stream appears.
- The CDN responses contain usable binary MP4 ranges in the HAR. The first file has a 1,068-byte MP4 init with one encrypted audio entry, a default key ID, 8-byte per-sample IVs, and a 44,100 Hz timescale. The first complete fragment uses clear sample description 2. The next uses CENC description 1 and the shared parser indexed all 431 encrypted samples offline. The initial read assumed the first fragment would be encrypted. The actual sample description switch explained the empty encrypted-sample list.
- Playback state includes `POST /track-playback/v1/devices` and `PUT /track-playback/v1/devices/{device}/state`. Registration sends the connection ID, client version, device descriptor and volume. The 13 state writes send `state_ref`, `sub_state`, `seq_num` and `debug_source`. Melody received 23 HTTP 202 batch responses. This confirms browser reporting traffic, but the capture cannot establish which updates count toward listening history.
- The original HAR remains local and ignored by Git. Inspection used only existing capture data and ran no signed-in requests or playback probes.

## Follow-up on Sonora's license refusal

- The next secondary-account run reached the regional Widevine license POST and got HTTP 403. The earlier anonymous client-token failure was past. All three HTTP 200 Firefox license responses used the bearer issued by the web player's own app ID. Sonora uses the web app's client token alongside its desktop OAuth bearer. That difference is a leading hypothesis, not an established cause.
- The local CDM challenge and the first Firefox challenge are both 1,739 bytes. Using the same captured certificate and initialization data, the signed envelopes and request field layouts matched, and the client identification bytes matched exactly. Per-request fields are different, as expected for separate challenges. This comparison does not establish that Spotify accepts Sonora's CDM challenge or why it returned 403. No captured license or bearer was replayed.
- Previously, Sonora discarded the response body before reporting HTTP 403. The licensing path now inspects up to 4 KiB of encoded and decoded response data. It emits only fixed labels for known OAuth error codes and the response type. A reason of `unreported` means the response did not match the allowlist. No new authenticated request was made to obtain this diagnostic.
- The next secondary-account run returned `reason: unreported, response: empty` with a 1,739-byte challenge and a session OAuth client ID different from the web client ID.
- The Firefox capture shows 15 media ranges beginning exactly at a seek-table segment boundary, plus three initialization ranges. Two tracks jumped forward more than 400 KB between requests. Offline, the shared CENC parser indexed 431 encrypted samples at 70.05 seconds from only the captured init segment and the target range. Firefox can start fetching at the chosen fragment. Sonora currently waits for sequential delivery to reach it.
- Spotify accepted 13 web playback-state writes and 23 Melody event batches. The HAR's `fetchEntitiesForRecentlyPlayed` queries carry URIs to hydrate. They are not a server-side attribution check. Neither the browser nor desktop capture establishes what Spotify will credit for plays from Sonora.

### Credential comparisons and remaining differences

- The first single-request debug run substituted a fresh Firefox bearer and kept Sonora's web client token. The second substituted both credentials as a pair. The pair was observed on a successful Firefox request, but the fresh HAR contained no license POST. Both Sonora runs generated fresh 1,739-byte challenges and received empty HTTP 403 responses. No captured challenge or license was replayed.
- All three successful license POSTs in the earlier Firefox HAR used HTTP/3. Sonora's HTTP client supports HTTP/2 and HTTP/1.1. The browser also sent fetch metadata headers. The new Sonora rejection log includes the negotiated HTTP version, but the effect of these differences remains unverified.
- Mozilla's official build calls `VerifyCdmHost_0` before CDM initialization and passes its own host files. The locally installed CDM exports this function and the Firefox binary references it. Sonora's standalone host does not perform that verification. The local inspection did not establish that Firefox's host verification succeeds or that Spotify requires it. [Firefox CDM adapter](https://github.com/mozilla-firefox/firefox/blob/main/dom/media/gmp/ChromiumCDMAdapter.cpp)
- An official Firefox play on the same account and track with HTTP/3 disabled temporarily would check whether Spotify still issues a license on the older transport. The ordinary Firefox session is a useful control. It does not test Sonora or reuse any of the captured credentials.

## Linux capture reference

## 1. Capture method (reproducible)

| Item | Value |
|---|---|
| Proxy | mitmproxy 12.3.1, `mitmdump -p 18080 --set ssl_insecure=true -s scripts/logger.py -w raw_flows.mitm` |
| Spotify proxy config | `~/.config/spotify/prefs`: `network.proxy.mode=2` and `network.proxy.addr="127.0.0.1:18080@http"` (removed afterwards) |
| Launch | `SSL_CERT_FILE=<system CAs + mitm CA> spotify --proxy-server=http://127.0.0.1:18080 --ignore-certificate-errors` |
| TLS | Both the C++ core (spclient, CDN, accounts) and the CEF UI (xpui) **accepted the MITM CA**; no certificate pinning was hit. 5 early `gew1-spclient` handshakes failed with "connection closed early", then retried fine. |
| Not decrypted | The Access Point (`ap-gew1.spotify.com:4070`) uses Spotify's own Shannon-encrypted protocol tunnelled through HTTP CONNECT. It was captured as raw TCP bytes only. |

The core routes **everything** (including the AP socket) through the configured HTTP proxy using `CONNECT`.

---

## 2. Hosts involved

| Host | Used by | Purpose |
|---|---|---|
| `apresolve.spotify.com` | core | Service discovery (AP, spclient, dealer) |
| `ap-gew1.spotify.com:4070` (also `:443`, `:80`) | core | Access Point: login, Mercury, legacy channels. Shannon-encrypted. |
| `accounts.spotify.com` | core | OAuth token refresh (DPoP-bound) |
| `gew1-spclient.spotify.com` | core + UI | Main API gateway: metadata, storage-resolve, playplay, connect-state, playlists… |
| `spclient.wg.spotify.com` | UI + core | Same gateway, anycast name (metadata/4, gabo, audio-analysis) |
| `gew1-dealer.g2.spotify.com` | UI | WebSocket push channel (`wss://…/?access_token=…`) |
| `audio-fa.scdn.co` | core | **Audio CDN (Fastly)**, primary |
| `audio-cf.spotifycdn.com` | core | Audio CDN (Cloudflare), fallback |
| `audio4-ak.spotifycdn.com` | core | Audio CDN (Akamai), fallback |
| `heads-fa-tls13.spotifycdn.com` | core | "Head files": plaintext start of each audio file |
| `video-cf.spotifycdn.com` | UI | Canvas (looping video) segments, not audio |
| `i.scdn.co`, `image-cdn-fa.spotifycdn.com`, `pickasso.spotifycdn.com` | UI | Cover art |
| `api-partner.spotify.com/pathfinder/v2/query` | UI | GraphQL (browse/home), not playback |

`gew1` = Europe-West-1 region (the capture was made from the UK). Other regions returned by apresolve: `guc3`, `gue1`, `gew4`, `gae2`.

---

## 3. Bootstrap / session establishment

### 3.1 apresolve

```
GET https://apresolve.spotify.com/?time=1795718586&type=accesspoint&type=spclient&type=dealer-g2
user-agent: Spotify/129500453 (8; 0; 5)
client-token: <REDACTED>
accept-language: en
accept-encoding: gzip, deflate, br, zstd
```
Response `200 application/json`:
```json
{
 "dealer-g2":  ["gew1-dealer.g2.spotify.com:443","guc3-dealer.g2.spotify.com:443","gue1-dealer.g2.spotify.com:443","gew4-dealer.g2.spotify.com:443"],
 "accesspoint":["ap-gew1.spotify.com:4070","ap-gew1.spotify.com:443","ap-gew1.spotify.com:80","ap-guc3.spotify.com:4070","ap-gue1.spotify.com:443","ap-gew4.spotify.com:80"],
 "spclient":   ["gew1-spclient.spotify.com:443","guc3-spclient.spotify.com:443","gue1-spclient.spotify.com:443","gew4-spclient.spotify.com:443"]
}
```
A second variant, `?time=…&type=accesspoint` with no client-token, is also sent. The UI later calls `?type=dealer-g2&type=spclient` (with a CORS preflight).

### 3.2 Access Point (raw TCP, port 4070)

- First client frame starts `00 04 00 00 01 64 …`: version byte `0x00 0x04`, then 4-byte big-endian length `0x164` (356), then the `ClientHello` protobuf. **[inferred]** This is the standard keyexchange (`ClientHello` → `APResponseMessage` → `ClientResponsePlaintext`), after which every frame is Shannon-encrypted.
- In this session the AP carried no visible audio data. Audio keys came over HTTPS (`/playplay/v1/key`, §5.3) and audio bytes came from the HTTPS CDN. The AP is still used for login/session and Mercury.
- 28 TCP messages total (3 client→server, 25 server→client) are in the dump.

### 3.3 OAuth token refresh

```
POST https://accounts.spotify.com/api/token
content-type: application/x-www-form-urlencoded
user-agent: Spotify/129500453 Linux/0 (PC desktop)
client-token: <REDACTED>
dpop: <ES256 JWT, header {"alg":"ES256","typ":"dpop+jwt","jwk":{"crv":"P-256","kty":"EC","x":…,"y":…}}, claims include "htu":"https://accounts.spotify.com/api/token", "htm":"POST", …>
origin: https://accounts.spotify.com

client_id=65b708073fc0480ea92a077233ca87bd&grant_type=refresh_token&refresh_token=<REDACTED>
```
Response:
```
set-cookie: __Host-device_id=…; Secure; HttpOnly; SameSite=Lax
dpop-nonce: <nonce>
{"access_token":"<REDACTED>","token_type":"Bearer","expires_in":3600,
 "scope":"playlist-modify ugc-image-upload user-follow-read user-read-email user-read-private app-remote-control streaming user-follow-modify user-modify-playback-state user-library-modify playlist-modify-public playlist-read user-read-birthdate …"}
```
- `65b708073fc0480ea92a077233ca87bd` is the desktop client ID. It also appears in `connect-state` DeviceInfo field 13 and in `device-capabilities`.
- The access token (`BQ…`) is the `Authorization: Bearer` on every spclient call and the `access_token` query parameter on the dealer websocket.

### 3.4 Device capabilities / settings

```
GET https://gew1-spclient.spotify.com/device-capabilities/v1/capabilities?device_type=computer&client_id=65b708073fc0480ea92a077233ca87bd&device_model=PC%20desktop&device_brand=spotify&client_version=1.2.95.453.g0eeebbed
```
Response (JSON, excerpt):
```json
{"license":"tft","supports_v2_playlist_uris":false,
 "supported_media_types":["audio/track","audio/episode","audio/dj","audio/media","audio/agnostic","audio/ad","audio/interruption"],
 "supports_external_episodes":true,"effective_license":"premium",
 "supported_audio_quality":"HIFI","audio_quality":"HIFI_24","debug_client_type":"client-zelda",
 "supports_hifi":{"fully_supported":true, …}}
```
`PUT /clientsettings/api/v1/` sends protobuf `{1:"preferred-locale", 2:{4:"en"}}`.

Playback-settings gRPC-web (`content-type: application/x-protobuf` in, `application/grpc` out, `grpc-status: 0`):
- `POST /playback-settings/spotify.playbacksettings.PlaybackSettingsService/GetDeviceSettings`: req `{1:{1:{1:device_id, 2:"dan-pc", 3:client_id, 4:"unknown"}}}`.
- `…/GetSettingsDeviceSelection`, `…/GetAllStoredContentValues` (req `{1:1000}`, returns a list of context URIs with stored per-context settings).
- `…/WriteContentValue`: sent when a per-context playback setting changes (e.g. shuffle mode on a playlist). It's echoed back over the dealer as `playback-settings/content-settings-update`.

---

## 4. Common request headers

### 4.1 Core (C++) → spclient
```
accept-language: en
app-platform: Linux
authorization: Bearer <access_token>
spotify-app-version: 129500453            # 1.2.95.453 encoded as 1 29 500 453
user-agent: Spotify/129500453 Linux/Unknown
client-token: <client token, base64-ish, ~300 chars>
origin: https://gew1-spclient.spotify.com      # on POST/PUT
sec-fetch-site: same-origin
sec-fetch-mode: no-cors
sec-fetch-dest: empty
accept-encoding: gzip, deflate, br, zstd
priority: u=4, i
```
Protobuf bodies use `content-type: application/x-protobuf` or `application/protobuf`. Large PUT bodies are gzip-compressed and marked with **`x-transfer-encoding: gzip`** (not `content-encoding`).

### 4.2 Core → audio CDN (HTTP/1.1 to Fastly)
```
Host: audio-fa.scdn.co
Connection: keep-alive
Pragma: no-cache
Cache-Control: no-cache, no-store
Accept-Encoding:                       # empty: no compression
Range: bytes=0-3145727
User-Agent: Spotify/129500453 Linux/0 (PC desktop)
x-playback-id: bfc69ce695cc3943e4fb6fb5824c5714   # 32 hex; all zeros for prefetch
Accept-Language: en
Sec-Fetch-Site: same-origin
Sec-Fetch-Mode: no-cors
Sec-Fetch-Dest: empty
```
No `Authorization` header. The CDN request is authorised only by the signed query string.

### 4.3 UI (xpui, CEF) → spclient
```
origin: https://xpui.app.spotify.com
referer: https://xpui.app.spotify.com/
user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.7680.179 Spotify/1.2.95.453 Safari/537.36
app-platform: Linux
spotify-app-version: 1.2.95.453          # dotted form in the UI
authorization: Bearer <access_token>
client-token: <same client token>
```
These are cross-origin, so each one is preceded by an `OPTIONS` CORS preflight.

---

## 5. The audio pipeline in detail

### 5.1 Track → audio files: `extended-metadata`

```
POST https://gew1-spclient.spotify.com/extended-metadata/v0/extended-metadata
content-type: application/protobuf
(body usually gzip, x-transfer-encoding: gzip)
```
Request (`BatchedEntityRequest`):
```
1 {                       # header
  1: "GB"                 # country
  2 { … }                 # catalogue/product
  3: <16 bytes>           # cache/etag token
}
2 {                       # one per entity (repeated)
  1: "spotify:track:7pMZ5uiSbU8SmdyN9f1bwU"
  2 { 1: 10 }             # extension kind (repeated)
}
```
Response (`BatchedExtensionResponse`): `2 { 2: <kind>, 3 { 2: <entity_uri>, 3 { 1: <type_url>, 2: <bytes> } } }`, with field 3 repeated per entity.

Extension kinds seen in this session:

| kind | entity | type_url | count |
|---|---|---|---|
| **10** | track | `spotify.metadata.Track` | 1134 |
| **5** | audio (`spotify:audio:<b62>`) | `spotify.extendedmetadata.audiofiles.AudioFilesExtensionResponse` | 147 |
| 9 | album | `spotify.metadata.Album` | 77 |
| 178 | track | `spotify.contentagnostic.v2.IdentityTrait` | 679 |
| 212 | track | `spotify.contentagnostic.v2.PlaybackTrait` | 167 (full file list incl. FLAC + loudness, see §15.2) |
| 239 / 249 / 179 / 182 | track | ContentCapability / ContentExperience / VisualIdentity / ConsumptionExperience traits | |
| 16 | track | `spotify.canvaz.cache.EntityCanvazResponse.Canvaz` | 209 |
| 99 | track | `spotify.bumblebee.video_associations.v1.VideoAssociations` | 26 |

**Track (kind 10).** Field 12 (`file`, repeated) = `{1: file_id (20 bytes), 2: format}`. Example for "66 Days" (402 882 ms):
```
12 { 1: 79c9c0cde958b09a160cd99206e5bfd6382b49ee  2: 2 }   # OGG_VORBIS_320
12 { 1: 5aa5476f7f614a6aa2e20e228ada738f5bcc488a  2: 1 }   # OGG_VORBIS_160
12 { 1: b26ece28366de73ce72b50fb7e47153dad95185c  2: 0 }   # OGG_VORBIS_96
12 { 1: 0bf7ddcbc4ccff8828013c3ccfb6de4b6a2559a4  2: 8 }   # AAC_24
```
Other Track fields: 1 gid, 2 name, 3 album, 4 artist, 7 duration_ms.

**AudioFiles (kind 5)** on `spotify:audio:…`. This newer path includes lossless:
```
1 { 1 { 1: <file_id>  2: <format> }  4: <avg bitrate bps> }    # repeated per file
2 { 1: <f32> 2: <f32> }   # loudness normalisation (track gain / peak)  [inferred]
3 { 1: <f32> 2: <f32> }   # album gain / peak                            [inferred]
4 { … }                   # misc audio attributes
```
Real example (`spotify:audio:3e2GOUzX1INmA58qj54qN5`):

| format | name | avg bitrate field 4 |
|---|---|---|
| 0 | OGG_VORBIS_96 | 90 223 |
| 1 | OGG_VORBIS_160 | 148 519 |
| 2 | OGG_VORBIS_320 | 313 007 |
| 8 | AAC_24 | 24 233 |
| 16 | FLAC_FLAC (16-bit) | 806 352 |
| 22 | FLAC_FLAC_24BIT | 1 507 888 |

(Format enum names match librespot's `AudioFile.Format`. 16 and 22 only appear in kind-5 responses.)

**What this client actually streamed:** **OGG_VORBIS_160**, even though the account reports `audio_quality: HIFI_24`. Evidence: the `AudioFileSelection` telemetry says `"cached file"`, bitrate `160000`, quality label `"audiophile"`; `RawCoreStream` says `"Vorbis 160 kbps"`; and the file `2b4379a1…` is listed in extended-metadata as `{1:<id>, 2:1, 4:160461}`. So the format is picked locally (streaming quality setting), not by the server.

UI-side JSON metadata also exists (used for display, no file list):
```
GET https://spclient.wg.spotify.com/metadata/4/track/{gid_hex32}?market=from_token
accept: application/json
→ {"gid":…,"name":…,"album":{…,"cover_group":{"image":[{"file_id":"ab67616d0000b273…","size":"LARGE",…}]}},
   "duration":178186,"has_lyrics":true,"original_audio":{"uuid":"…","format":"AUDIO_FORMAT_STEREO"},"media_type":"AUDIO", "file":null, …}
```
(`file` is `null` in the JSON form. The JSON response is `content-encoding: zstd`.)

### 5.2 File → CDN URLs: `storage-resolve`

```
GET https://gew1-spclient.spotify.com/storage-resolve/v2/files/audio/interactive/1/{file_id_hex40}?product=0
GET https://gew1-spclient.spotify.com/storage-resolve/v2/files/audio/interactive_prefetch/1/{file_id_hex40}?product=0
pragma: no-cache
cache-control: no-cache
+ core headers (§4.1)
```
- `interactive` is used for the track the user just started. `interactive_prefetch` is used for upcoming tracks, preloaded in the background.
- The `/1/` path segment is constant here. **[inferred]** It's a version/flag.

Response `200 application/x-protobuf` (gzip):
```
1: 0                                  # result = CDN
2: "https://audio-fa.scdn.co/audio/<file-id>?<signature>"
2: "https://audio-cf.spotifycdn.com/audio/517af76a…?verify=1795805000-UvMCT8Gz…%3D"
2: "https://audio4-ak.spotifycdn.com/audio/517af76a…?__token__=exp=1795805000~hmac=7f55a88d…"
4: <file_id, 20 raw bytes>
5: 86400                              # TTL seconds
```
The three URL signing schemes:
- **Fastly**: `?{expiry_unix}_{base64url HMAC}`
- **Cloudflare**: `?verify={expiry}-{base64 HMAC}`
- **Akamai**: `?__token__=exp={expiry}~hmac={hex sha256}`

Expiry is about **24 h** after the request (`1795805000 − 1795718600 = 86400`), which matches field 5. The client always used the first (Fastly) URL.

### 5.3 Decryption key: `playplay`

```
POST https://gew1-spclient.spotify.com/playplay/v1/key/{file_id_hex40}
content-type: application/x-www-form-urlencoded     # sic: the body is actually protobuf
content-length: 30
origin: https://gew1-spclient.spotify.com
+ core headers (§4.1)
```
Request body (protobuf), all 7 observed requests:

| field | value | meaning |
|---|---|---|
| 1 | `5` | PlayPlay protocol version |
| 2 | 16 bytes, **identical in every request** (`<redacted>`) | client/build PlayPlay token |
| 4 | `1` for `interactive` files, `3` for `interactive_prefetch` files | interactivity (matches the storage-resolve path) |
| 5 | `1` | content type (audio track) **[inferred]** |
| 6 | unix seconds | request timestamp |

Response `200`, `content-length: 24`, no content-type:
```
1: <16 bytes>    # obfuscated key (PlayPlay-wrapped AES-128 key)
2: <4 bytes>     # (unknown; possibly checksum/flags)
```
The playplay key request goes out in the same ~100 ms as storage-resolve and the CDN fetch, in parallel. It's also requested for files already in the local cache (first track `1c950b00…` had no storage-resolve/CDN fetch, only playplay).

> Unwrapping the PlayPlay key happens inside the closed-source client binary. This capture only shows the obfuscated key on the wire, and that step isn't documented here. **[inferred]** Once unwrapped, the audio body is AES-128-CTR, matching librespot's long-standing description of the format.

### 5.4 Head file (fast start)

```
GET https://heads-fa-tls13.spotifycdn.com/head/{file_id_hex40}
accept-encoding:
user-agent: Spotify/129500453 Linux/0 (PC desktop)
x-playback-id: <playback id>
accept-language: en
priority: u=4, i
```
- HTTP/2, `200 application/octet-stream`, `cache-control: max-age=315360000, no-transform`, GCS-backed (`x-goog-*`).
- Body is **plaintext Ogg** (starts with `OggS`). Sizes seen: 199 381 B and 186 588 B, which is enough to start decoding instantly while the key and main body arrive.
- Only fetched for `interactive` (user-initiated) plays. Prefetched tracks skip it.
- Also reported in telemetry as `HeadFileDownload` (`host`, size, latency ms, `"interactive"`).

### 5.5 Audio body from the CDN

```
GET https://audio-fa.scdn.co/audio/{file_id_hex40}?{fastly_sig}
Range: bytes=0-3145727              # interactive: first 3 MiB …
Range: bytes=3145728-4198452        # … then the remainder, ~70 ms later
Range: bytes=0-9437183              # prefetch: one request up to 9 MiB (server clamps to file size)
```
Response `206 application/octet-stream`:
```
Content-Range: bytes 0-3145727/4198453
x-goog-stored-content-length: 4198453
etag: "1be64477b665f3f12fb466b547cfa502"
accept-ranges: bytes
x-cache: HIT, HIT
access-control-allow-origin: *
cache-control: max-age=315360000, no-transform
```
- The body is high-entropy (encrypted). No `OggS` magic at offset 0.
- **The whole file is pulled immediately.** For a 4.2 MB, 220 s track, both ranges were requested within 70 ms of each other.
- HTTP/1.1 keep-alive is used to Fastly (not h2).

All audio fetches in the session:

| t (s) | file_id | kind | ranges | total size |
|---|---|---|---|---|
| 13.2 | 517af76a… | prefetch | 0-9437183 | 3 179 112 |
| 31.0 | cbb73fc0… | interactive (+head) | 0-3145727, 3145728-4198452 | 4 198 453 |
| 95.5 | 2b4379a1… | interactive (+head) | 0-3145727, 3145728-4866677 | 4 866 678 |
| 100.5 | 231d036c… | prefetch | 0-9437183 | 3 265 344 |
| 100.9 | ec7a8d4a… | prefetch | 0-9437183 | 4 025 028 |
| 107.7 | 055150b7… | prefetch | 0-9437183 | 5 104 153 |

### 5.6 Prefetch behaviour
- Prefetch requests use `x-playback-id: 000…0` (32 zeros) and playplay `field4=3`.
- After a track change, the client prefetched the next **2–3** queue items in one go (t=100.5 → 107.7 s).
- Already-cached files skip storage-resolve and CDN entirely (`AudioFileSelection: "cached file"`), but still get a playplay key.

---

## 6. Playback state: `connect-state`

```
PUT https://gew1-spclient.spotify.com/connect-state/v1/devices/{device_id_hex40}
content-type: application/protobuf
x-transfer-encoding: gzip           # body is gzip
x-spotify-connection-id: <dealer Spotify-Connection-Id>
+ core headers
```
- Response: `200`, `content-encoding: br`, a `Cluster` protobuf (all devices + active player state).
- `GET /connect-state/v1/cluster` fetches the same thing on demand.
- `device_id` = `24dfe2b8bb4861620c0cda29663df4c6d435176f` (40 hex, stable per install).

### 6.1 `PutStateRequest` (observed fields)
```
2 {                                # Device
  1 { … DeviceInfo … }
  2 { … PlayerState … }
}
3: 2                               # member_type = CONNECT_STATE
4: 1                               # is_active
5: <put_state_reason>
7: "5ba58d28-9cba-42f6-87f7-9043e517aea9"   # last_command_sent_by_device_id (only after a remote/UI command)
8: 422353573                       # last_command_message_id
9: 1795718595148                   # started_playing_at (ms)
12: 1795718682850                  # client_side_timestamp (ms)
```
`put_state_reason` values seen: **9** `NEW_CONNECTION` (first PUT), **5** `VOLUME_CHANGED` (burst of 7 at t≈20–23 s while dragging volume), **4** `PLAYER_STATE_CHANGED` (track change, seek, play/pause).

### 6.2 DeviceInfo
```
1: 1                     # can_play
2: 65535                 # volume (0–65535)
3: "dan-pc"              # name
4 {                      # capabilities
  2: 1  5: 1  6: 1  7: 1
  8: 64                  # volume_steps
  9: "audio/track" "audio/episode" "audio/ad" "audio/local" "audio/podcast-chapter" "video/track" …  # supported_types
  10,15–20,23,25,29,33–38: 1 (feature flags)   30: 5
}
6: "1.2.95.453.g0eeebbed"   # client version
7: 1                         # device_type = COMPUTER
9: "3.2.6"                   # spirc version
10: "24dfe2b8…"              # device_id
13: "65b708073fc0480ea92a077233ca87bd"   # client_id
14: "spotify"  15: "PC desktop"           # brand / model
16 { "device_address_mask": "192.168.0.84/24" } { "debug_level": "1" } { "tier1_port": "0" }
22: "<public ip>"
```

### 6.3 PlayerState (the playback engine's state)
```
1: 1795718621973                      # timestamp (ms): position is valid as of this time
2: "spotify:playlist:3iLrPTTOZmfpHYTuEmTjwU"     # context_uri
3: "context://spotify:playlist:…"                # context_url
5 { 1:"home" 2:"xpui-snapshot_2026-08-05_…" 5:"home" }   # play_origin (feature id / version / view)
6 { 2: 177 }                          # index in context
7 {                                   # current track (ProvidedTrack)
  1: "spotify:track:62V2ZHslgQV98gH4AuVXnr"
  2: "836c7e229600f9c8"               # uid
  3 { "title": … } { "album_title": … } { "artist_uri": … } { "image_*_url": … } { "track_player": "audio" } …
  6: "context"                        # provider
}
8: "bfc69ce695cc3943e4fb6fb5824c5714" # playback_id  (= CDN x-playback-id)
9: 1.0 (double)                       # playback_speed
10: 3851                              # position_as_of_timestamp (ms)
11: 220564                            # duration (ms)
12: 1                                 # is_playing
13: (absent = false)                  # is_paused
14: (absent = false)                  # is_buffering
16 { 1: 1  5: {context_enhancement: RECOMMENDATION} {jam: off} … }   # options (shuffle/repeat/modes)
17 { 2:"not_paused" 6:"no_prev_track" 25:"not_supported_by_content_type" … }   # restrictions
19 [prev_tracks]   20 [next_tracks]   # queue, each a ProvidedTrack
21 [context_metadata]
23: "3g5OdSdF7L4WJdpOr82oWW"          # session_id
24: "6866021328398609791"             # queue_revision
33: "automix-preview" "reshuffle" "interact" "speed-preview" …   # signals
35: "c3d40aa7789e1d531bd3b3e60028522d"
```
**Current position = `pos_as_of_ts + (now − timestamp) × speed`**, while `is_playing && !is_paused`.

### 6.4 Session timeline decoded from connect-state PUTs

| t (s) | reason | track | pos_as_of_ts | event |
|---|---|---|---|---|
| 0.0 | NEW_CONNECTION | – | – | device registered |
| 8.1 | PLAYER_STATE_CHANGED | 5uPaqMMt…(178 s) | 45 273 | resume playback |
| 19.6 | PLAYER_STATE_CHANGED | same | 33 601 | **seek back** |
| 20.8–22.8 | VOLUME_CHANGED ×7 | – | – | **volume drag** (DeviceInfo.2: 65535→44953→31105→26885→26632→12241→21993→65535) |
| 24.3 | PLAYER_STATE_CHANGED | same | 103 038 | **seek forward** |
| 26.2 | PLAYER_STATE_CHANGED | 18gnbzwr… | 0 | **skip next** |
| 28.6 | PLAYER_STATE_CHANGED | 34ZfAjSG… (other playlist) | 0 | new context |
| 30.8 | PLAYER_STATE_CHANGED | 62V2ZHsl… | 0 | track change → interactive fetch |
| 34.6 / 35.4 / 36.2 / 37.6 | PLAYER_STATE_CHANGED | same | 3 851 → 4 592 → 5 369 → 101 838 | settings change, then **seek** |
| 93.5 | PLAYER_STATE_CHANGED (+ last_command) | 0mCGK2B8… | 0 | skip |
| 95.5 | PLAYER_STATE_CHANGED | 0EKbLCcc… | 0 | skip → interactive fetch |
| 102.4 | PLAYER_STATE_CHANGED | same | 2 694 | settings change |

No PUT had `is_paused=1`, so no pause/resume was captured.

---

## 7. Dealer WebSocket (push channel)

```
GET wss://gew1-dealer.g2.spotify.com/?access_token=<access_token>     → 101 Switching Protocols
```
Text frames, JSON:

- **Keep-alive:** client `{"type":"ping"}` every 30 s, server `{"type":"pong"}`.
- **First server message** gives the connection id that must be echoed in `x-spotify-connection-id` on connect-state PUTs:
  ```json
  {"headers":{"Spotify-Connection-Id":"<base64>"},"method":"PUT","type":"message",
   "uri":"hm://pusher/v1/connections/<urlencoded base64>"}
  ```
- **Messages** look like `{"headers":{…},"payloads":["<base64 protobuf or JSON string>"],"type":"message","uri":"<topic>"}`. Topics seen:

| uri | payload |
|---|---|
| `hm://playlist/v2/playlist/{id}` | playlist revision change (protobuf) |
| `hm://herodotus/uri/spotify:list:play-history:v1/resume-point-revision/{uuid}` | resume point recorded for the track just played |
| `hm://herodotus/batch` | batch of resume points |
| `playback-settings/content-settings-update` | `{2:"spotify:playlist:…", 3:{1:4,2:{2:1},3:ts}, 3:{1:5,2:{3:1},3:ts}}`, per-context setting change |
| `hm://collection/collection/{username}` and `/json` | library change: `{"items":[{"type":"track","removed":true,"identifier":"0EKbLCcc…"}]}` |

**[inferred]** Remote-control commands from other Connect devices (`hm://connect-state/v1/player/command`, `…/cluster`) arrive on this channel. None were sent in this session, because all control was local. The local UI drives the core player in-process, so local play/seek/skip doesn't produce dealer traffic.

---

## 8. Telemetry relevant to playback (`gabo-receiver-service`)

```
POST https://spclient.wg.spotify.com/gabo-receiver-service/v3/events/          (authenticated)
POST https://spclient.wg.spotify.com/gabo-receiver-service/public/v3/events/   (NonAuth events)
```
Body is protobuf: `1 { 2: "<EventName>", 3 { 1: "message", 2: <event payload protobuf> } }`, repeated.

Counts in this session: RawCoreStreamSegment 23, AudioSessionEvent 19, Download 11, CacheRealmReport 10, BoomboxPlaybackSession 8, AudioFileSelection 7, RawCoreStream 7, Prefetch 4, ShuffleContextEvent 4, ShuffleSequenceEvent 4, AudioDriverInfo 3, HeadFileDownload 2, CorePlaybackCommandCorrelation 2, TrackNotPlayed 1, AudioStreamingSettingsReport 1, AudioSettingsReport 1, …

Key payloads (field numbers from decode_raw):

**AudioFileSelection**
```
1: <playback_id 16B>  2: "cached file"  3: 160000 (bitrate)  4: "<uuid>"  5: "audiophile"  6: 160000
```

**HeadFileDownload**
```
1: <file_id 20B>  2: <playback_id 16B>  3: "https"  4: "heads-fa-tls13.spotifycdn.com"
5/6/13: 199381 (bytes)  8/9/10: 182/182/183 (ms timings)  11: 200 (HTTP status)  16: "interactive"  17: "missing" (cache)
```

**Download** (one per CDN file fetch)
```
1: <file_id>  2: <playback_id>  10: 2607956 (bytes)  38: "interactive"  40: 160000 (bitrate)  44: "complete"  … many timing fields (-1 = n/a)
```

**AudioSessionEvent**: `{1:"open"|"close", 4:<playback_id>, 5:"remote", 6:"unknown", 7:<ms>, 8:0, 9:1}`

**RawCoreStream** (end-of-stream summary per playback)
```
1: <playback_id>   4: <file_id>  5: "audio"  9: "playlist"
10: reason_start (observed: "remote", "appload")   11: source   12: reason_end (observed: "fwdbtn", "unknown")
13: start ts ms   14/15: ms_played   22: "Vorbis 160 kbps"
23: context_uri   24: track_uri   28: "context"   29: "home"
34/35: session uuids  38: "full"  44: "local"  45: "boombox" (player engine)
56: end ts ms  57: ms seeked/skipped  69: "boombox"  71: "context-player"  78-80: "spotify"/"PC desktop"/"computer"
```

**RawCoreStreamSegment** (one per continuous play segment, split at seeks)
```
1: <playback_id>  2/3: start/end position ms  5: reason_start ("remote","seek-bck","seek-fwd",…)  6: reason_end
7: 1.0 speed  8/9: wall-clock ms  13: "audio"  15: track_uri  16/17: bytes …  20: "boombox"  25: context_uri
```
"Boombox" is the internal name of the desktop core's playback engine.

Other playback-adjacent calls:
- `herodotus/spotify.resumption.v1.ResumePointRevisionService/CreateResumePointRevision`: req `{2:"spotify:list:play-history:v1", 4:{2:{6:track_uri}, 3:{1:sec,2:nanos}}}`, sent on every track start.
- `herodotus/…/BatchCreateResumePointRevisions`, `…/ListResumePointRevisions`, `CurrentStateService/ListCurrentStates`: resume/history sync at startup.
- `capping-api/spotify.cappingapi.v1.CappingApi/PutConsumption`: feature-usage caps (`"Spotify.Feature.PickAndShuffle"`, `"Spotify.Feature.Bluejay"`).
- `audio-attributes/v1/audio-analysis/{track_b62}?format=json` (UI, ~92 KB JSON: beats/segments; 404 for some tracks). It's used by visual features, not by the audio engine.

---

## 12. Gaps and caveats

- **AP channel not decrypted.** Anything carried over Mercury on the Shannon socket (e.g. some login and product-info details) isn't visible.
- **No pause/resume** and no remote (other-device) commands were captured.
- **Quality** was 160 kbps Vorbis. Lossless (FLAC 16/24-bit) file IDs appear in metadata but weren't streamed, so the FLAC CDN path (likely identical apart from format) is unverified.
- The PlayPlay key format and the meaning of response field 2 are only described at the wire level.
- Spicetify is injected into the UI. Core (C++) traffic shouldn't be affected.

---

## 13. Files in this folder

| File | Contents |
|---|---|
| `raw_flows.mitm` (57 MB) | **Complete raw dump**: every HTTP/1.1 and HTTP/2 request and response with full bodies (including the encrypted audio files), WebSocket frames and raw AP TCP bytes. Open with `mitmweb -r raw_flows.mitm` or `mitmproxy -r …`, or process with `mitmdump -nr raw_flows.mitm -s yourscript.py`. |
| `events.jsonl` (26 MB) | Same capture flattened to one JSON object per event (`type` = http / ws / tcp / http_error / tls_failed_*). Bodies are `{"text":…}` or `{"b64":…}`. Responses over 200 KB are marked `{"omitted":N}`; use the .mitm file for those. |
| `mitmdump.log` | Proxy console log |
| `analysis/*.txt` | Per-endpoint request/response dumps with protobuf decoded (storage-resolve, playplay, audio-cdn, connect-state, extmeta, token, events, meta4) |
| `scripts/` | Helper scripts: `pb.py` (minimal protobuf parser + gunzip), `extract.py` (per-endpoint dumper), `cs.py` (connect-state timeline), `ws.py` (dealer decoder), `em.py`/`af.py` (extended-metadata / audio files), `gabo2.py` (telemetry decoder), `logger.py` (capture addon) |
| `conf/` | mitmproxy CA used for this capture (private key included) |

Example: re-run the connect-state timeline:
```sh
nix shell nixpkgs#python3 -c python3 -I scripts/cs.py events.jsonl scripts
```

## 14. ⚠ Sensitive data in the raw dump

`raw_flows.mitm`, `events.jsonl` and `analysis/` contain **live credentials**: the OAuth **refresh token**, access tokens, client-token, DPoP key material, CDN-signed URLs, your Spotify username, LAN and public IP addresses, and the Spotify auto-login blob in request flows. The refresh token is long-lived. Don't share these files. To invalidate the captured refresh token, log out of Spotify on this device (or "Sign out everywhere" in account settings). `conf/mitmproxy-ca.pem` is a CA private key, so keep it private or delete it.

---

## 15. Follow-up findings (verified against the capture and the clients' own schemas)

Sources for this section:

- **The capture itself** (`events.jsonl`), re-decoded.
- **Protobuf descriptors embedded in the desktop core** (`.spotify-wrapped` in the Spotify 1.2.95 Nix package). It links full protobuf, so 918 `FileDescriptorProto`s with real message and field names can be read out of the binary.
- **Protobuf-lite classes in the Android APK** (`com.spotify.music.apk`). Field names survive in `newMessageInfo` tables.

Names below come from those schemas unless marked **[inferred]**.

### 15.1 Head files are the plaintext start of the audio file

Both head files in the capture (`cbb73fc0…`, 199 381 B; `2b4379a1…`, 186 588 B) start with `OggS` and parse as clean Ogg pages:

| offset | size | granule | page flags | what |
|---|---|---|---|---|
| 0 | 167 (0xa7) | 0 | 6 (BOS+EOS) | Spotify's own header page |
| 167 | 58 | 0 | 2 | Vorbis identification header |
| 225 | 4286 | 0 | 0 | Vorbis comment + setup headers |
| 4511 | … | 3648, 7744, … | 0 | audio pages |

The ReplayGain pair sits at byte 144 of the header page (`f32 gain_db`, `f32 peak`, little-endian): `-8.583 / 1.028` for `cbb73fc0…` and `-10.212 / 0.975` for `2b4379a1…`. So a head file is byte-for-byte the decrypted prefix of the CDN file, the 0xa7-byte Spotify header page included. A player can decode it (and read the loudness) before the decryption key arrives, then carry on with CDN bytes from `head.len()`. The head host needs no signature and no auth.

### 15.2 Extension 212: `spotify.contentagnostic.v2.PlaybackTrait`

Requested on the **track URI**. This is the modern file list: every format the track has, lossless included, plus loudness, in the same batch as TrackV4. From `playback_trait.proto` (desktop core):

```proto
message PlaybackTrait {
  message AudioMapEntry { optional AudioMediaFormat key = 1; optional AudioPlayback value = 2; }
  message VideoMapEntry { optional VideoMediaFormat key = 1; optional VideoPlayback value = 2; }
  repeated AudioMapEntry audio = 1;
  repeated VideoMapEntry video = 2;
  optional spotify.playback_platform.transition.v1.TransitionMaps transition_maps = 3;
}
enum AudioMediaFormat { UNSPECIFIED = 0; STEREO = 1; SPATIAL = 2; ADAPTIVE = 3; }
message AudioPlayback {
  oneof asset { AudioPassThrough pass_through = 1; AudioHosted hosted = 2; }
  optional string reporting_uri = 3;
  optional PlaybackOffsets offsets = 4;
  optional string uri_for_migration_purposes = 6;
}
message AudioHosted {
  repeated AudioFile files = 1;
  optional bytes audio_id = 2;                     // 16 bytes = spotify:audio:<base62>
  optional NormalizationParams file_norm = 3;      // per track
  optional NormalizationParams group_norm = 4;     // per album
  optional PlaybackOffsets offsets = 5;
}
message AudioFile {
  optional bytes file_id = 1;                      // 20 bytes
  optional spotify.audiophile.Format format = 2;   // absent = 0 = OGG_VORBIS_96
  optional int32 average_bitrate = 3;              // bps
}
message NormalizationParams { optional float loudness_db = 1; optional float true_peak_db = 2; }
message AudioPassThrough { optional FlatFile flat_file = 1; optional AudioHosted fallback = 3; optional AdDetection ad_detection = 4; optional PlaybackOffsets offsets = 5; }
message PlaybackOffsets { optional google.protobuf.Duration start = 1; optional google.protobuf.Duration end = 2; }
```

The Android APK has the same classes (`com.spotify.contentagnostic.v2.{PlaybackTrait, AudioPlayback, AudioHosted, AudioFile, NormalizationParams}`), but its build predates `AudioFile.format`.

Real example (`spotify:track:06rX4GEEjsVTSbRBWycs2U`): `audio[0].key = STEREO`, `hosted.files` lists formats 0, 1, 2, 8, 10, 11, 12, 13, 14, 15, 16, 17, 22, 23 and 29. `file_norm = {-10.80, 0.116}` and `group_norm = {-9.34, 1.16}`. `reporting_uri` and field 6 are both the track URI.

**Empty entries are cache hits, not gaps.** Every extension query carries an etag, and the response header's status field says what came back:

| kind | 200 (payload) | 304 (no payload, client cache valid) | 404 |
|---|---|---|---|
| 10 TrackV4 | 345 | 789 | – |
| 212 PlaybackTrait | 95 | 71 | 1 |
| 5 AudioFiles | 41 | 106 | – |

A client that sends no etag always gets a payload.

### 15.3 Extension 5: `spotify.extendedmetadata.audiofiles.AudioFilesExtensionResponse`

The older route to the same data. It goes through TrackV4 field 24 (`original_audio`, `spotify.metadata.Audio { bytes uuid = 1; Format format = 2 /* UNKNOWN, DEFAULT, PARROT */ }`): base62-encode the 16-byte `uuid`, then query `spotify:audio:<b62>` with kind 5. 5 of the 41 kind-5 entities in the capture were matched to a track this way.

```proto
message AudioFilesExtensionResponse {
  repeated ExtendedAudioFile files = 1;
  optional NormalizationParams default_file_normalization_params = 2;
  optional NormalizationParams default_album_normalization_params = 3;
  optional bytes audio_id = 4;
}
message ExtendedAudioFile { optional spotify.metadata.AudioFile file = 1; optional int32 average_bitrate = 4; }
```

Kind 212 makes this two-step lookup unnecessary.

### 15.4 Extended-metadata request details

- Header `1 { 1: "GB" (country), 2: "premium" (catalogue), 3: <16 bytes> }`.
- Each query is `{1: extension_kind, 2: etag}`.
- Background batches carry `client-feature-id: prefetch` and are sent as `content-encoding: gzip` with `accept: application/protobuf`.

### 15.5 Audio format enum (`spotify.audiophile.Format`, desktop core)

| # | name | # | name |
|---|---|---|---|
| 0 | OGG_VORBIS_96 | 16 | FLAC_FLAC (16-bit) |
| 1 | OGG_VORBIS_160 | 17 | MP4_FLAC |
| 2 | OGG_VORBIS_320 | 18–20 | XHE_AAC_24 / _16 / _12 |
| 3–7 | MP3_256, MP3_320, MP3_160, MP3_96, MP3_160_ENC | 21 | HE_AAC_64 |
| 8, 9 | AAC_24, AAC_48 | 22 | FLAC_FLAC_24BIT |
| 10, 11 | MP4_128, MP4_256 | 23 | MP4_FLAC_24BIT |
| 12, 13 | MP4_128_DUAL, MP4_256_DUAL | 24, 25 | MP4_128_CLEAR, MP4_256_CLEAR |
| 14, 15 | MP4_128_CBCS, MP4_256_CBCS | 26 | WAV_PCM_UNENCRYPTED |
| | | 27 | OGG_VORBIS_160000_UNENCRYPTED |
| | | 28, 29 | MP4_HE_AAC_V1_64, MP4_HE_AAC_V1_96 |
| | | 30–53 | MP3 mono and stereo ladders |

Also `spotify.audiophile.Bucket { UNKNOWN, AUDIO, LOSSLESS, MP3_PREVIEW }`.

The Android build numbers differ in places. It has no 10–15, and adds `PHONO_LOW/HIGH/CLEAR` (54–56) and `FLAC_FLAC_PETREL` (57). Its file-selection code only accepts 0–9, 16, 18–20, 22 and 54–57.

The MP4 formats (10–15, 17, 23) are what the web player streams through Widevine. The desktop core streamed Ogg Vorbis (format 1).

### 15.6 Loudness: metadata vs the file header

For `2b4379a1…` (track `0EKbLCcc…`):

| source | values |
|---|---|
| Ogg header page, byte 144 | gain −10.21 dB, peak 0.975 (ReplayGain style, linear peak) |
| kind 5 `default_file_normalization_params` | loudness −6.79, true peak −0.22 |
| kind 5 `default_album_normalization_params` | loudness −6.66, true peak 1.49 |

The two are on different scales. The metadata reads as integrated loudness in LUFS plus true peak in dBTP **[inferred from the names `loudness_db` / `true_peak_db`]**. FLAC files carry no Spotify header page (unverified, nothing lossless was streamed), so for them the metadata is the only loudness source.

### 15.7 `storage-resolve` response schema

From `storage-resolve.proto` (`spotify.download.proto`):

```proto
message StorageResolveResponse {
  enum Result { CDN = 0; STORAGE = 1; RESTRICTED = 3; }
  optional Result result = 1;
  repeated string cdnurl = 2;
  optional bytes fileid = 4;
}
```

Field 5 (`86400`, URL lifetime in seconds) is on the wire but not in this descriptor, so the server is newer than the client's schema.

The former Sonora route was `/storage-resolve/files/audio/interactive/{file_id}`. The captured desktop client uses `/storage-resolve/v2/files/audio/{interactive|interactive_prefetch}/1/{file_id}?product=0`.

### 15.8 Resume points (`spotify.resumption.v1`, desktop core)

```proto
message CreateResumePointRevisionRequest { optional string user_id = 1; optional string uri = 2; optional ResumePointRevision resume_point_revision = 4; }
message ResumePointRevision { optional string resume_point_revision_id = 1; optional ResumePoint snapshot = 2; optional google.protobuf.Timestamp create_time = 3; optional google.protobuf.Timestamp update_time = 4; }
message ResumePoint {
  optional string resume_point_id = 1;
  oneof resume_point {
    google.protobuf.Duration position = 2; google.protobuf.Empty started = 3; google.protobuf.Empty finished = 4;
    int32 counter = 5; string uri = 6; google.protobuf.Any custom = 7;
    google.protobuf.Empty marked_as_finished = 8; google.protobuf.Empty unmarked_as_finished = 9;
    RelativePosition relative_position = 10; google.protobuf.Empty tombstone = 11; ListResumePoint list_resume_point = 12;
  }
}
```

The captured call (§8) is `uri = "spotify:list:play-history:v1"`, `snapshot.uri = <track uri>`, `create_time = now`. The server answers with the revision id and `update_time`. Also present: `ListResumePointRevisions`, `BatchCreateResumePointRevisions` and `CurrentStateService/ListCurrentStates`.

### 15.9 Client token

The client-token request (`clienttoken.spotify.com`) is **not in this capture**; the token was cached from an earlier run. Its schema is in the core (`spotify/client_token/v0/client_token_http.proto`): `ClientTokenRequest { request_type; client_data | challenge_answers }`, `ClientTokenResponse { granted_token { token, expires_after_seconds, refresh_after_seconds, domains } | challenges }`, with challenge types `CLIENT_SECRET_HMAC`, `EVALUATE_JS` and `HASH_CASH`. This gap applies to the Linux capture. The later Firefox HAR includes the web client-token request and grant.

### 15.10 PlayPlay

No descriptor for the PlayPlay request or response exists in the desktop core or the APK. §5.3 (from the wire) is all that's known, and the key unwrap remains undocumented.

