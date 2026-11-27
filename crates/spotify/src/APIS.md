# Spotify APIs

## HTTP calls that use JSON

| Area | Endpoint | Use |
| --- | --- | --- |
| Discovery and offline recovery | `GET apresolve.spotify.com/v2?type=accesspoint&type=dealer&type=spclient` | Finds regional access point, Dealer and spclient servers. Also the network probe that tells Sonora when Spotify is reachable again. |
| OAuth | `POST accounts.spotify.com/api/token` | Exchanges the PKCE authorization code, then refreshes the access token and saves each rotated refresh token. Sends form data and receives JSON. |
| Playlist cover upload | `POST image-upload.spotify.com/v4/playlist` | Sends the JPEG body and receives an upload token as JSON. |
| Connect player commands | `POST /connect-state/v1/player/command/from/{from}/to/{to}` | Sends playback and queue commands as gzipped JSON. Forward queue jumps name the target in `skip_next`. History jumps use `play` with the selected track and its context when available. |
| Connect transfer | `POST /connect-state/v1/connect/transfer/from/{from}/to/{to}` | Moves playback between devices, asking the target to keep the paused state. |

## Protobuf HTTP calls

| Area | Endpoint | Use |
| --- | --- | --- |
| Playback contexts | `POST /sequence-proxy/spotify.sequenceproxy.v1.SequenceProxyService/ResolveContext` | Reads occurrence IDs when publishing a Connect queue, reads playlists through the `?spotify-apply-lenses=enhance` lens for smart shuffle and marks the songs it recommends, resolves play commands whose track or queue cannot be reconstructed locally, and requests radio stations for tracks, albums, playlists, and artists. |
| Playback contexts | `POST /sequence-proxy/spotify.sequenceproxy.v1.SequenceProxyService/ResolvePage` | Fetches entries for page IDs returned by ResolveContext. Batches known page IDs and follows continuation pages in order when Spotify returns a chain. |
| Search | `GET /searchview/v3/search` · `entity_types=track` | Searches tracks. Lyrics lookup by title and artist uses it too. |
| Search | `GET /searchview/v3/search` · `entity_types=album` | Searches albums. |
| Search | `GET /searchview/v3/search` · `entity_types=artist` | Searches artists. |
| Search | `GET /searchview/v3/search` · `entity_types=playlist` | Searches playlists. The full Search page sends all four kinds at once. |
| Metadata | `POST /extended-metadata/v0/extended-metadata` | Loads catalogue metadata, compact profiles, and Home and Browse cards. |
| Browse | `GET /browsita/v1/browse` | Supplies music categories and directory links for the flat Browse grid. Category contents come from GetLists. |
| Home | `GET /casita/v1/home/default?timezone={tz}` | Loads Home shelves and shortcuts. The Made For You shelves added to Home come from GetLists. |
| Lyrics | `GET /color-lyrics/v3/track/{id}` | Loads lyrics and timing for a Spotify track, or for the closest track matches of a title and artist search. |
| Profiles | `GET /user-profile-view/v3/profile/{user}` | Loads a profile, follower and following counts, and public playlists. |
| Saved music | `POST /collection/v2/paging` · `collection`, `artist` | Reads pages of saved tracks and albums, and of followed artists. |
| Library pins | `POST /collection/v2/paging` · `ylpin` | Reads the account's Spotify pins. |
| Library pins | `POST /collection/v2/delta` · `ylpin` | Reads pin changes of the last 89 days, since a paging read can lag behind a pin made through another region. |
| Saved music and pins | `POST /collection/v2/write` | Updates saved music and pins. |
| Playlists | `POST /playlist/spotify.list.v1.ListReadService/GetLists` | Reads playlist details, dated entries and contributors, library pages, and Browse and Made For You sections with their cards. Also batches summaries of playlists the app already lists. |
| Playlists | `POST /playlist/v2/playlist` | Creates a playlist. |
| Playlists | `POST /playlist/v2/playlist/{id}/changes` | Applies edits. Appends batch tracks without reading a revision. Name and cover can change together. |
| Playlist cover registration | `POST /playlist/v2/playlist/{id}/register-image` | Registers the uploaded image and returns its picture ID. |
| Playlists | `POST /playlist/v2/user/{user}/rootlist/changes` | Updates the playlist library. Appends need no prior read. |
| Audio manifest | `GET /track-playback/v1/media/{uri}` | Lists the track's web AAC files and duration. |
| Audio storage | `GET /storage-resolve/v2/files/audio/interactive/{format}/{file-id}` | Resolves signed CDN URLs for the chosen MP4. |
| Audio licensing | `GET /melody/v1/license_url` and `POST /widevine-license/v1/audio/license` | Discovers the endpoint and exchanges the CDM challenge for a Widevine license. |
| Connect device state | `PUT /connect-state/v1/devices/{device}` | Publishes device state, including the smart shuffle mode and its lens while it is on, and receives the account cluster. The capabilities say the device supports smart shuffle. |
| Connect device state | `PUT /connect-state/v1/devices/{device}/inactive?notify=false` | Marks the device inactive. |
| Connect device state | `DELETE /connect-state/v1/devices/{device}` | Unregisters the device. |
| Connect volume | `PUT /connect-state/v1/connect/volume/from/{from}/to/{to}` | Sets another device’s volume. |
| Jam | `GET /social-connect/v2/sessions/current` | Reads the current Jam. |
| Jam | `GET /social-connect/v2/sessions/current_or_new` | Starts or resumes a Jam. |
| Jam | `GET /social-connect/v2/sessions/info/{token}` | Reads Jam invitation details. |
| Jam | `POST /social-connect/v2/sessions/join/{token}` | Joins a Jam by invitation. |
| Jam | `DELETE /social-connect/v3/sessions/{id}` | Ends a hosted Jam. |
| Jam | `POST /social-connect/v3/sessions/{id}/leave` | Leaves a Jam as a guest. |
| Jam | `POST /social-connect/v3/sessions/{id}/member/{member}/kick` | Removes a Jam member. |

## WebSocket

| Topic or connection | Payload | Use |
| --- | --- | --- |
| Dealer connection | TLS WebSocket with JSON envelopes | Opens a connection to a discovered Dealer host using OAuth, then exchanges heartbeats and acknowledgements. |
| `hm://pusher/v1/connections` | Connection ID in envelope headers | Identifies the Dealer connection for device registration and commands. |
| `hm://connect-state/v1/cluster` | Protobuf | Reports active playback, queue, and available devices. |
| `hm://connect-state/v1/connect/volume` | Protobuf | Reports volume changes. |
| `hm://connect-state/v1/player/command` | Compressed JSON, with embedded protobuf for transfer state | Delivers playback, queue, and transfer commands to Sonora. `set_options`, `update_context`, `play` options and transfers also carry the smart shuffle mode. |
| `hm://collection/collection` | Protobuf | Reports saved music changes. |
| `hm://playlist/v2/playlist` | Protobuf | Refreshes changed playlists. |
| `hm://playlist/user` and `hm://playlist/v2/user/{user}/rootlist` | Protobuf | Refreshes the library playlist list when its revision changes. |
| `social-connect/v2/session_update` | JSON | Reports Jam session changes. Live start and end notifications used JSON even with protobuf HTTP replies. |

## Other protocols and formats

| Area | Endpoint or source | Format and use |
| --- | --- | --- |
| Browser sign-in | `accounts.spotify.com/authorize` and a local callback | Browser pages and an HTTP loopback callback. |
| Artwork display | `i.scdn.co`, `image-cdn-{fa,ak}.spotifycdn.com`, `mosaic.scdn.co` and `misc.scdn.co` | Image bytes for covers, artist images, playlist mosaics and the Liked Songs cover. |
| Share links | `open.spotify.com/{kind}/{id}` and `open.spotify.com/socialsession/{token}` | Built for copied links and Jam invitations, and parsed from pasted ones. Never fetched. |
| Audio playback | `GET` signed audio CDN URL | Fetches the AAC MP4 in byte ranges. |
| Audio seek table | `GET seektables.spotifycdn.com/v1/seektable/{file-id}` | Provides the Widevine initialization data. |
| Playback client grant | `POST clienttoken.spotify.com/v1/clienttoken` | Grants a short-lived client token for the web player client ID. |
| Widevine certificate | `GET spclient.wg.spotify.com/widevine-license/v1/application-certificate` | Supplies the application certificate for Spotify's CDM session. |
| Account identity and market | Spotify access point over TCP | Reads the canonical account name and market for sign-in and catalogue lookups. |
| Short links | `GET spotify.link/{code}` | Follows redirects through the shared link helper in `music` to resolve shared links. |
