# Notifications and status
get-sonora = Get Sonora
notice-never = Don't show again
offline-go-online = Go online
offline-title = You are offline.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio is running, but ALSA has no route to it, so nothing you play is heard. Install pulseaudio-alsa and restart Sonora.
   *[pipewire] PipeWire is running, but ALSA has no route to it, so nothing you play is heard. Install pipewire-alsa and restart Sonora.
}
requirement-audio-bridge-title = Sonora cannot reach your sound server
requirement-audio-output = Sonora could not open your audio device, so nothing you play is heard. Check that a device is connected and not held by another app.
requirement-audio-output-title = No sound output
requirement-help = What to install
requirement-web-engine = Install webkit2gtk-4.1 to sign in to YouTube Music, Deezer or Apple Music in a browser window. Until then, paste your cookies instead.
requirement-web-engine-title = Browser sign-in is unavailable
toast-code-copied = Code copied
toast-download-cancelled = Cancelled downloading { $name }
toast-download-failed = Could not list the tracks to download.
toast-download-started = Started downloading { $name }
toast-downloads-cancelled = { $count ->
    [one] Cancelled downloading { $count } track
   *[other] Cancelled downloading { $count } tracks
}
toast-downloads-started = { $count ->
    [one] Started downloading { $count } song
   *[other] Started downloading { $count } songs
}
toast-favorite-albums-removed = Removed { $count ->
        [one] one album
       *[other] { $count } albums
    } from your favorites
toast-favorite-artists-removed = Removed { $count ->
        [one] one artist
       *[other] { $count } artists
    } from your favorites
toast-favorite-removed = Removed { $name } from your favorites
toast-favorite-songs-removed = Removed { $count ->
        [one] one song
       *[other] { $count } songs
    } from your favorites
toast-folder-missing = { $name } is no longer on disk
toast-folder-removed = Removed { $name }
toast-history-clear-failed = History could not be cleared
toast-history-remove-failed = A song could not be removed from history
toast-history-removed = { $count ->
    [one] Removed from history
   *[other] Removed { $count } songs from history
}
toast-history-restore-failed = History could not be restored
toast-jam-ended = Jam ended
toast-jam-failed = The Jam could not be started or joined.
toast-jam-leave-failed = The Jam could not be ended or left.
toast-jam-left = You left the Jam
toast-jam-needs-connect = Turn on “Show as a Spotify device” in settings to join a Jam
toast-jam-unreadable = That link does not lead to a Jam you can join.
toast-keys-error = Playback key error
toast-keys-refused = Spotify may not work here
toast-last-album = Album plays last
toast-last-artist = Artist plays last
toast-last-playlist = Playlist plays last
toast-last-track = { $name } plays last
toast-last-tracks = { $count } tracks play last
toast-library-add-failed = { $name } could not be added to your library
toast-library-added = Added to your library
toast-library-albums-removed = Removed { $count ->
        [one] one album
       *[other] { $count } albums
    } from your library
toast-library-pin-failed = Could not update the pin.
toast-library-remove-failed = { $name } could not be removed from your library
toast-library-removed-one = Removed { $name } from your library
toast-library-scanned = { $count ->
    [one] Library scanned, one song found
   *[other] Library scanned, { $count } songs found
}
toast-library-songs-removed = Removed { $count ->
        [one] one song
       *[other] { $count } songs
    } from your library
toast-link-copied = Link copied
toast-link-sign-in = Sign in to { $name } to open this link
toast-link-unreadable = That link does not lead anywhere Sonora can open
toast-local-delete-failed = Some track files could not be deleted
toast-next-album = Album plays next
toast-next-artist = Artist plays next
toast-next-playlist = Playlist plays next
toast-next-track = { $name } plays next
toast-next-tracks = { $count } tracks play next
toast-offline = No connection. Nothing will stream until it is back.
toast-offline-mode = You are offline. Go online to stream this song.
toast-offline-unsupported = { $name } plays downloads only when online.
toast-open-all-failed = { $count } files could not be opened
toast-open-failed = Could not open { $name }
toast-open-some-failed = Some files could not be opened
toast-open-unsupported = { $name } is not an audio file Sonora can play
toast-output-default-failed = Unable to select the system default device
toast-output-direct-failed = Unable to play directly on the device
toast-output-rate-failed = Unable to switch the device's sample rate
toast-output-select-failed = Unable to select { $name }
toast-playback-stopped = Playback stopped after several tracks in a row could not be played
toast-playlist-added = Playlist added to your library
toast-playlist-busy = Another change is still running
toast-playlist-cover-changed = Playlist cover changed
toast-playlist-cover-removed = Playlist cover removed
toast-playlist-created = Playlist created
toast-playlist-deleted = Playlist deleted
toast-playlist-exported = Playlist exported
toast-playlist-failed = That change could not be saved
toast-playlist-import-empty = None of the songs in { $name } are in your library
toast-playlist-imported-partly = Imported { $name }, leaving out { $count ->
        [one] one song
       *[other] { $count } songs
    } not in your library
toast-playlist-removed = Playlist removed from your library
toast-playlist-renamed = Playlist renamed
toast-playlist-signed-out = Sign in to change playlists
toast-playlist-updated = Playlist updated
toast-playlist-visibility = Playlist visibility changed
toast-playlists-removed = Removed { $count ->
        [one] one playlist
       *[other] { $count } playlists
    } from your library
toast-queue-cleared = Queue cleared
toast-queue-failed = That could not be added to the queue
toast-queue-local-elsewhere = Local files cannot be queued on another device
toast-queue-removed-track = Removed { $name } from the queue
toast-queue-removed-tracks = Removed { $count } tracks from the queue
toast-queued-album = Album added to the queue
toast-queued-artist = Artist added to the queue
toast-queued-playlist = Playlist added to the queue
toast-queued-track = { $name } added to the queue
toast-queued-tracks = { $count } tracks added to the queue
toast-radio-failed = Could not start radio for { $name }
toast-restore-failed = { $name } could not sign back in, so Sonora carried on without it
toast-sample-burst = Toast number { $name }
toast-sample-delete = Deleted { $name }
toast-sample-done = Saved to your library
toast-sample-event = Event created
toast-sample-event-detail = Sunday, December 3 at 9:00 AM
toast-sample-failed = The server could not be reached
toast-sample-info = Arrive 10 minutes before the event.
toast-sample-lasting = This one stays until you close it
toast-sample-loading = Uploading your tracks
toast-sample-long = This toast says a good deal more than the others, so the text has to wrap onto a second and maybe a third line without pushing the close button out of the card
toast-sample-plain = Event has been created
toast-sample-save-failed = { $name } could not be saved
toast-sample-save-failed-detail = The server answered 503 Service Unavailable
toast-sample-saved = Saved { $name }
toast-sample-saving = Saving { $name }
toast-sample-undone = Undone
toast-sample-warning = Your session ends in five minutes
toast-scan-failed = Your music folders could not be scanned
toast-session-ended = Session ended
toast-session-failed = The Session could not be started or joined.
toast-session-full = That Session is full.
toast-session-invalidated = Your { $name } session was invalidated
toast-session-kicked = The host removed you from the Session
toast-session-left = You left the Session
toast-session-lost = Lost the connection to the Session
toast-session-needs-provider = Sign in to a streaming service to start a Session.
toast-session-not-found = No Session goes by that code.
toast-session-provider = This Session plays from { $name }. Sign in to { $name } to join it.
toast-session-rate-limited = Too many wrong codes. Try again in a few minutes.
toast-session-too-many = Your network already hosts as many Sessions as it may. End one to start another.
toast-session-unreachable = Could not reach Sonora Session. Try again later.
toast-sessions-server-empty = Type the address of your server first
toast-sessions-server-malformed = That does not read as a server address
toast-sessions-server-saved = Sessions server saved
toast-sessions-server-silent = No Sonora Sessions server answers at this address, so it was not saved
toast-settings-broken = Fix line { $name } of settings.json to save changes
toast-settings-overwritten = settings.json changed on disk, so your last change was not saved
toast-settings-save-failed = Settings could not be saved
toast-sign-in-to-play = { $name } only streams to a signed-in listener
toast-similar-failed = Could not find tracks similar to { $name }
toast-sleep-ended = Sleep timer ended
toast-soloist-ready = Spotify now plays through Soloist
toast-soloist-refused = Soloist can no longer sign in
toast-soloist-remove-failed = Soloist could not be removed
toast-still-throttled = { $name } is still limiting playback. Press play to try again
toast-tags-failed = The tags could not be saved
toast-tags-saved = Saved the tags for { $name }
toast-theme-delete-failed = Cannot delete { $name }
toast-theme-install-failed = Cannot install { $name }
toast-theme-link-unreadable = That theme link is damaged or holds something other than a theme
toast-throttled = { $name } is limiting playback for now. Trying again in a moment
toast-track-added = Added to { $name }
toast-track-moved = Moved to { $name }
toast-track-removed = Removed from { $name }
toast-track-removed-unnamed = Removed from the playlist
toast-track-unplayable = { $name } could not be played
toast-tracks-removed = Removed { $count ->
        [one] one song
       *[other] { $count } songs
    } from { $name }
toast-tracks-removed-unnamed = Removed { $count ->
        [one] one song
       *[other] { $count } songs
    } from the playlist
toast-tray-unavailable = There is no system tray to put the icon in
toast-unpinned = Unpinned { $name }
toast-why = Why?
toast-widevine-borrowed = Try updating Widevine
toast-widevine-download-failed = Widevine could not be downloaded
toast-widevine-failed = Playback failed
toast-widevine-install-failed = Widevine could not be installed
toast-widevine-remove-failed = Widevine could not be removed
trouble-not-loaded = Could not load
trouble-offline = No connection
trouble-offline-detail = Check your internet connection and try again.
trouble-retry = Try again
update-available = Sonora { $version } is out
update-channel-nightly = Nightly
update-channel-stable = Stable
update-detail = You are on { $running }. Read what changed, or update now.
update-detail-notes = You are on { $running }. Read what changed, then update Sonora the way you installed it.
update-failed = The update could not be installed. Try again from the releases page.
update-never = Don't check for updates
update-notes = What's new
update-now = Update
update-working = Downloading the update…
wake-reason = Music is playing
