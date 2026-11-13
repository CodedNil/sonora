# Notifications and status
get-sonora = Scarica Sonora
notice-never = Non mostrare più
offline-go-online = Torna online
offline-title = Sei offline.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio è in esecuzione, ma ALSA non ha un percorso verso di esso, quindi non si sente nulla. Installa pulseaudio-alsa e riavvia Sonora.
   *[pipewire] PipeWire è in esecuzione, ma ALSA non ha un percorso verso di esso, quindi non si sente nulla. Installa pipewire-alsa e riavvia Sonora.
}
requirement-audio-bridge-title = Sonora non riesce a raggiungere il server audio
requirement-audio-output = Sonora non è riuscita ad aprire il dispositivo audio, quindi non si sente nulla. Controlla che un dispositivo sia collegato e non sia occupato da un'altra app.
requirement-audio-output-title = Nessuna uscita audio
requirement-help = Cosa installare
requirement-web-engine = Installa webkit2gtk-4.1 per accedere a YouTube Music, Deezer o Apple Music in una finestra del browser. Nel frattempo, incolla i cookie.
requirement-web-engine-title = Accesso tramite browser non disponibile
toast-code-copied = Codice copiato
toast-download-cancelled = Download di { $name } annullato
toast-download-failed = Impossibile elencare i brani da scaricare.
toast-downloads-cancelled = { $count ->
    [one] Download di { $count } brano annullato
   *[other] Download di { $count } brani annullato
}
toast-favorite-albums-removed = { $count ->
        [one] Un album rimosso
       *[other] { $count } album rimossi
    } dai preferiti
toast-favorite-artists-removed = { $count ->
        [one] Un artista rimosso
       *[other] { $count } artisti rimossi
    } dai preferiti
toast-favorite-removed = { $name } rimosso dai preferiti
toast-favorite-songs-removed = { $count ->
        [one] Un brano rimosso
       *[other] { $count } brani rimossi
    } dai preferiti
toast-folder-missing = { $name } non è più sul disco
toast-folder-removed = { $name } rimossa
toast-history-clear-failed = Impossibile svuotare la cronologia
toast-history-remove-failed = Impossibile rimuovere un brano dalla cronologia
toast-history-removed = { $count ->
    [one] Rimosso dalla cronologia
   *[other] { $count } brani rimossi dalla cronologia
}
toast-history-restore-failed = Impossibile ripristinare la cronologia
toast-jam-ended = Jam terminata
toast-jam-failed = Impossibile avviare la Jam o unirsi.
toast-jam-leave-failed = Impossibile terminare o lasciare la Jam.
toast-jam-left = Hai lasciato la Jam
toast-jam-needs-connect = Attiva “Mostra come dispositivo Spotify” nelle impostazioni per unirti a una Jam
toast-jam-unreadable = Questo link non porta a una Jam a cui puoi unirti.
toast-keys-error = Errore di chiave
toast-keys-refused = Spotify forse non va
toast-last-album = L'album verrà riprodotto per ultimo
toast-last-artist = L'artista verrà riprodotto per ultimo
toast-last-playlist = La playlist verrà riprodotta per ultima
toast-last-track = { $name } verrà riprodotto per ultimo
toast-last-tracks = { $count } brani verranno riprodotti per ultimi
toast-library-add-failed = Impossibile aggiungere { $name } alla tua libreria
toast-library-added = Aggiunto alla tua libreria
toast-library-albums-removed = { $count ->
        [one] Un album rimosso
       *[other] { $count } album rimossi
    } dalla tua libreria
toast-library-pin-failed = Impossibile aggiornare l'elemento fissato su Spotify.
toast-library-remove-failed = Impossibile rimuovere { $name } dalla tua libreria
toast-library-removed-one = { $name } rimosso dalla tua libreria
toast-library-scanned = { $count ->
    [one] Libreria scansionata, trovato un brano
   *[other] Libreria scansionata, trovati { $count } brani
}
toast-library-songs-removed = { $count ->
        [one] Un brano rimosso
       *[other] { $count } brani rimossi
    } dalla tua libreria
toast-link-copied = Link copiato
toast-link-sign-in = Accedi a { $name } per aprire questo link
toast-link-unreadable = Questo link non porta a nulla che Sonora possa aprire
toast-local-delete-failed = Impossibile eliminare alcuni file dei brani
toast-next-album = L'album verrà riprodotto per prossimo
toast-next-artist = L'artista verrà riprodotto per prossimo
toast-next-playlist = La playlist verrà riprodotta per prossima
toast-next-track = { $name } verrà riprodotto per prossimo
toast-next-tracks = { $count } brani verranno riprodotti per prossimi
toast-offline = Nessuna connessione. Lo streaming riprenderà quando tornerà.
toast-offline-mode = Sei offline. Torna online per ascoltare questo brano in streaming.
toast-offline-unsupported = { $name } riproduce i download solo online.
toast-open-all-failed = Impossibile aprire { $count } file
toast-open-failed = Impossibile aprire { $name }
toast-open-some-failed = Impossibile aprire alcuni file
toast-open-unsupported = { $name } non è un file audio che Sonora può riprodurre
toast-output-default-failed = Impossibile selezionare il dispositivo predefinito di sistema
toast-output-select-failed = Impossibile selezionare { $name }
toast-playback-stopped = Riproduzione interrotta dopo diversi brani di fila non riproducibili
toast-playlist-added = Playlist aggiunta alla tua libreria
toast-playlist-busy = Un'altra modifica è ancora in corso
toast-playlist-cover-changed = Copertina della playlist cambiata
toast-playlist-cover-removed = Copertina della playlist rimossa
toast-playlist-created = Playlist creata
toast-playlist-deleted = Playlist eliminata
toast-playlist-exported = Playlist esportata
toast-playlist-failed = Impossibile salvare la modifica
toast-playlist-import-empty = Nessun brano di { $name } è nella tua libreria
toast-playlist-imported-partly = Importazione di { $name } completata, senza { $count ->
        [one] un brano assente
       *[other] { $count } brani assenti
    } dalla tua libreria
toast-playlist-removed = Playlist rimossa dalla tua libreria
toast-playlist-renamed = Playlist rinominata
toast-playlist-signed-out = Accedi per modificare le playlist
toast-playlist-updated = Playlist aggiornata
toast-playlist-visibility = Visibilità playlist modificata
toast-playlists-removed = { $count ->
        [one] Una playlist rimossa
       *[other] { $count } playlist rimosse
    } dalla tua libreria
toast-queue-cleared = Coda svuotata
toast-queue-failed = Impossibile aggiungere alla coda
toast-queue-local-elsewhere = I file locali non si possono mettere in coda su un altro dispositivo
toast-queue-removed-track = { $name } rimosso dalla coda
toast-queue-removed-tracks = { $count } brani rimossi dalla coda
toast-queued-album = Album aggiunto alla coda
toast-queued-artist = Artista aggiunto alla coda
toast-queued-playlist = Playlist aggiunta alla coda
toast-queued-track = { $name } aggiunto alla coda
toast-queued-tracks = { $count } brani aggiunti alla coda
toast-radio-failed = Impossibile avviare la radio di { $name }
toast-restore-failed = Impossibile riaccedere a { $name }, quindi Sonora ha continuato senza
toast-sample-burst = Notifica numero { $name }
toast-sample-delete = { $name } eliminato
toast-sample-done = Salvato nella tua libreria
toast-sample-event = Evento creato
toast-sample-event-detail = Domenica 3 dicembre alle 9:00
toast-sample-failed = Impossibile raggiungere il server
toast-sample-info = Arriva 10 minuti prima dell'evento.
toast-sample-lasting = Questa resta finché non la chiudi
toast-sample-loading = Caricamento dei tuoi brani
toast-sample-long = Questa notifica dice molto più delle altre, quindi il testo deve andare a capo su una seconda e forse una terza riga senza spingere il pulsante di chiusura fuori dalla scheda
toast-sample-plain = L'evento è stato creato
toast-sample-save-failed = Impossibile salvare { $name }
toast-sample-save-failed-detail = Il server ha risposto 503 Service Unavailable
toast-sample-saved = { $name } salvato
toast-sample-saving = Salvataggio di { $name }
toast-sample-undone = Annullato
toast-sample-warning = La tua sessione scade tra cinque minuti
toast-scan-failed = Impossibile scansionare le tue cartelle musicali
toast-settings-broken = Correggi la riga { $name } di settings.json per salvare le modifiche
toast-settings-overwritten = settings.json è cambiato sul disco, quindi l'ultima modifica non è stata salvata
toast-settings-save-failed = Impossibile salvare le impostazioni
toast-sign-in-to-play = { $name } trasmette solo agli ascoltatori connessi
toast-similar-failed = Impossibile trovare brani simili a { $name }
toast-sleep-ended = Timer di spegnimento terminato
toast-still-throttled = { $name } sta ancora limitando la riproduzione. Premi Riproduci per riprovare
toast-tags-failed = Impossibile salvare i tag
toast-tags-saved = Tag salvati per { $name }
toast-throttled = { $name } sta limitando la riproduzione. Nuovo tentativo tra un momento
toast-track-added = Aggiunto a { $name }
toast-track-removed = Rimosso da { $name }
toast-track-removed-unnamed = Rimosso dalla playlist
toast-track-unplayable = Impossibile riprodurre { $name }
toast-tracks-removed = { $count ->
        [one] Un brano rimosso
       *[other] { $count } brani rimossi
    } da { $name }
toast-tracks-removed-unnamed = { $count ->
        [one] Un brano rimosso
       *[other] { $count } brani rimossi
    } dalla playlist
toast-tray-unavailable = Non c'è un'area di notifica in cui mettere l'icona
toast-unpinned = { $name } rimosso dai fissati
toast-why = Perché?
toast-widevine-borrowed = Prova ad aggiornare Widevine
toast-widevine-download-failed = Impossibile scaricare Widevine
toast-widevine-failed = Riproduzione non riuscita
toast-widevine-install-failed = Impossibile installare Widevine
toast-widevine-remove-failed = Impossibile rimuovere Widevine
trouble-not-loaded = Impossibile caricare
trouble-offline = Nessuna connessione
trouble-offline-detail = Controlla la connessione internet e riprova.
trouble-retry = Riprova
update-available = Sonora { $version } disponibile
update-detail = Stai usando { $running }. Leggi cosa è cambiato, o aggiorna ora.
update-detail-notes = Stai usando { $running }. Leggi cosa è cambiato, poi aggiorna Sonora come l'hai installata.
update-failed = Impossibile installare l'aggiornamento. Riprova dalla pagina delle release.
update-never = Non cercare aggiornamenti
update-notes = Novità
update-now = Aggiorna
update-working = Scaricamento aggiornamento in corso…
wake-reason = Musica in riproduzione
