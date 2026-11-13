# Notifications and status
get-sonora = Merr Sonora
notice-never = Mos e shfaq më
offline-go-online = Kalo në linjë
offline-title = Je jashtë linje.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio po funksionon, por ALSA nuk ka rrugë drejt tij, kështu që asgjë që luan nuk dëgjohet. Instalo pulseaudio-alsa dhe rinis Sonora.
   *[pipewire] PipeWire po funksionon, por ALSA nuk ka rrugë drejt tij, kështu që asgjë që luan nuk dëgjohet. Instalo pipewire-alsa dhe rinis Sonora.
}
requirement-audio-bridge-title = Sonora nuk arrin te serveri yt i zërit
requirement-audio-output = Sonora nuk mundi të hapte pajisjen tënde audio, kështu që asgjë që luan nuk dëgjohet. Kontrollo që një pajisje është e lidhur dhe nuk po përdoret nga një aplikacion tjetër.
requirement-audio-output-title = Nuk ka dalje zëri
requirement-help = Çfarë të instalosh
requirement-web-engine = Instalo webkit2gtk-4.1 për të hyrë në YouTube Music, Deezer ose Apple Music në një dritare shfletuesi. Deri atëherë, ngjit cookies.
requirement-web-engine-title = Hyrja me shfletues nuk është e disponueshme
toast-code-copied = Kodi u kopjua
toast-download-cancelled = Shkarkimi i { $name } u anulua
toast-download-failed = Këngët për shkarkim nuk mund të listoheshin.
toast-downloads-cancelled = { $count ->
    [one] Shkarkimi i { $count } kënge u anulua
   *[other] Shkarkimi i { $count } këngëve u anulua
}
toast-favorite-albums-removed = { $count ->
        [one] U hoq një album
       *[other] U hoqën { $count } albume
    } nga të preferuarat
toast-favorite-artists-removed = { $count ->
        [one] U hoq një artist
       *[other] U hoqën { $count } artistë
    } nga të preferuarat
toast-favorite-removed = { $name } u hoq nga të preferuarat
toast-favorite-songs-removed = { $count ->
        [one] U hoq një këngë
       *[other] U hoqën { $count } këngë
    } nga të preferuarat
toast-folder-missing = { $name } nuk është më në disk
toast-folder-removed = U hoq { $name }
toast-history-clear-failed = Historiku nuk mund të pastrohej
toast-history-remove-failed = Një këngë nuk mund të hiqej nga historiku
toast-history-removed = { $count ->
    [one] U hoq nga historiku
   *[other] U hoqën { $count } këngë nga historiku
}
toast-history-restore-failed = Historiku nuk mund të rikthehej
toast-jam-ended = Jam-i përfundoi
toast-jam-failed = Jam-i nuk mund të nisej dhe as nuk u bashkove dot.
toast-jam-leave-failed = Jam-i nuk mund të përfundohej dhe as nuk u largove dot.
toast-jam-left = U largove nga Jam-i
toast-jam-needs-connect = Aktivizo “Shfaq si pajisje Spotify” te cilësimet për t'u bashkuar në një Jam
toast-jam-unreadable = Kjo lidhje nuk të çon te një Jam ku mund të bashkohesh.
toast-keys-error = Gabim në çelësin e luajtjes
toast-keys-refused = Spotify mund të mos punojë
toast-last-album = Albumi luhet i fundit
toast-last-artist = Artisti luhet i fundit
toast-last-playlist = Lista e luajtjes luhet e fundit
toast-last-track = { $name } luhet e fundit
toast-last-tracks = { $count } këngë luhen të fundit
toast-library-add-failed = { $name } nuk mund të shtohej te biblioteka
toast-library-added = U shtua te biblioteka
toast-library-albums-removed = { $count ->
        [one] U hoq një album
       *[other] U hoqën { $count } albume
    } nga biblioteka jote
toast-library-pin-failed = Nuk mund të përditësohej fiksimi.
toast-library-remove-failed = { $name } nuk mund të hiqej nga biblioteka
toast-library-removed-one = { $name } u hoq nga biblioteka jote
toast-library-scanned = { $count ->
    [one] Biblioteka u skanua, u gjet një këngë
   *[other] Biblioteka u skanua, u gjetën { $count } këngë
}
toast-library-songs-removed = { $count ->
        [one] U hoq një këngë
       *[other] U hoqën { $count } këngë
    } nga biblioteka jote
toast-link-copied = Lidhja u kopjua
toast-link-sign-in = Hyr te { $name } për ta hapur këtë lidhje
toast-link-unreadable = Kjo lidhje nuk të çon diku që Sonora mund ta hapë
toast-local-delete-failed = Disa skedarë këngësh nuk mund të fshiheshin
toast-next-album = Albumi luhet i radhës
toast-next-artist = Artisti luhet i radhës
toast-next-playlist = Lista e luajtjes luhet e radhës
toast-next-track = { $name } luhet e radhës
toast-next-tracks = { $count } këngë luhen të radhës
toast-offline = Pa lidhje. Asgjë nuk do të transmetohet derisa të kthehet.
toast-offline-mode = Je jashtë linje. Kalo në linjë për ta transmetuar këtë këngë.
toast-offline-unsupported = { $name } i luan shkarkimet vetëm kur je në linjë.
toast-open-all-failed = { $count } skedarë nuk mund të hapeshin
toast-open-failed = { $name } nuk mund të hapej
toast-open-some-failed = Disa skedarë nuk mund të hapeshin
toast-open-unsupported = { $name } nuk është skedar audio që Sonora mund ta luajë
toast-output-default-failed = Pajisja parazgjedhur e sistemit nuk mund të zgjidhej
toast-output-select-failed = { $name } nuk mund të zgjidhej
toast-playback-stopped = Luajtja u ndal pasi disa këngë radhazi nuk mund të luheshin
toast-playlist-added = Lista e luajtjes u shtua te biblioteka
toast-playlist-busy = Një ndryshim tjetër është ende në vazhdim
toast-playlist-cover-changed = Kopertina e listës së luajtjes u ndryshua
toast-playlist-cover-removed = Kopertina e listës së luajtjes u hoq
toast-playlist-created = Lista e luajtjes u krijua
toast-playlist-deleted = Lista e luajtjes u fshi
toast-playlist-exported = Lista e luajtjes u eksportua
toast-playlist-failed = Ai ndryshim nuk mund të ruhej
toast-playlist-import-empty = Asnjë këngë nga { $name } nuk është në bibliotekën tënde
toast-playlist-imported-partly = U importua { $name }, pa { $count ->
        [one] një këngë që nuk është
       *[other] { $count } këngë që nuk janë
    } në bibliotekën tënde
toast-playlist-removed = Lista e luajtjes u hoq nga biblioteka
toast-playlist-renamed = Lista e luajtjes u riemërtua
toast-playlist-signed-out = Hyr për të ndryshuar listat e luajtjes
toast-playlist-updated = Lista e luajtjes u përditësua
toast-playlist-visibility = Dukshmëria e listës së luajtjes u ndryshua
toast-playlists-removed = { $count ->
        [one] U hoq një listë luajtjeje
       *[other] U hoqën { $count } lista luajtjeje
    } nga biblioteka jote
toast-queue-cleared = Radha u pastrua
toast-queue-failed = Nuk mund të shtohej në radhë
toast-queue-local-elsewhere = Skedarët lokalë nuk mund të shtohen në radhë në një pajisje tjetër
toast-queue-removed-track = { $name } u hoq nga radha
toast-queue-removed-tracks = { $count } këngë u hoqën nga radha
toast-queued-album = Albumi u shtua në radhë
toast-queued-artist = Artisti u shtua në radhë
toast-queued-playlist = Lista e luajtjes u shtua në radhë
toast-queued-track = { $name } u shtua në radhë
toast-queued-tracks = { $count } këngë u shtuan në radhë
toast-radio-failed = Radio për { $name } nuk mund të nisej
toast-restore-failed = { $name } nuk mundi të rihynte, kështu që Sonora vazhdoi pa të
toast-sample-burst = Njoftimi numër { $name }
toast-sample-delete = U fshi { $name }
toast-sample-done = U ruajt në bibliotekën tënde
toast-sample-event = Ngjarja u krijua
toast-sample-event-detail = E diel, 3 dhjetor, ora 9:00
toast-sample-failed = Serveri nuk mund të arrihej
toast-sample-info = Mbërri 10 minuta para ngjarjes.
toast-sample-lasting = Ky qëndron derisa ta mbyllësh
toast-sample-loading = Duke ngarkuar këngët e tua
toast-sample-long = Ky njoftim thotë shumë më tepër se të tjerët, ndaj teksti duhet të kalojë në një rresht të dytë e ndoshta të tretë pa e shtyrë butonin e mbylljes jashtë kartës
toast-sample-plain = Ngjarja u krijua
toast-sample-save-failed = { $name } nuk mund të ruhej
toast-sample-save-failed-detail = Serveri u përgjigj 503 Service Unavailable
toast-sample-saved = U ruajt { $name }
toast-sample-saving = Duke ruajtur { $name }
toast-sample-undone = U zhbë
toast-sample-warning = Seanca jote mbaron për pesë minuta
toast-scan-failed = Dosjet e tua të muzikës nuk mund të skanoheshin
toast-settings-broken = Rregullo rreshtin { $name } të settings.json për të ruajtur ndryshimet
toast-settings-overwritten = settings.json ndryshoi në disk, kështu që ndryshimi yt i fundit nuk u ruajt
toast-settings-save-failed = Cilësimet nuk mund të ruheshin
toast-sign-in-to-play = { $name } transmeton vetëm për një dëgjues të lidhur
toast-similar-failed = Nuk u gjetën këngë të ngjashme me { $name }
toast-sleep-ended = Kohëmatësi i gjumit përfundoi
toast-still-throttled = { $name } ende po e kufizon luajtjen. Shtyp luaj për të provuar përsëri
toast-tags-failed = Etiketat nuk mund të ruheshin
toast-tags-saved = U ruajtën etiketat për { $name }
toast-throttled = { $name } po e kufizon luajtjen për momentin. Do të provohet përsëri pas pak
toast-track-added = U shtua te { $name }
toast-track-removed = U hoq nga { $name }
toast-track-removed-unnamed = U hoq nga lista e luajtjes
toast-track-unplayable = { $name } nuk mund të luhej
toast-tracks-removed = { $count ->
        [one] U hoq një këngë
       *[other] U hoqën { $count } këngë
    } nga { $name }
toast-tracks-removed-unnamed = { $count ->
        [one] U hoq një këngë
       *[other] U hoqën { $count } këngë
    } nga lista e luajtjes
toast-tray-unavailable = Nuk ka shirit sistemi ku të vendoset ikona
toast-unpinned = U hoq fiksimi i { $name }
toast-why = Pse?
toast-widevine-borrowed = Provo të përditësosh Widevine
toast-widevine-download-failed = Widevine nuk mund të shkarkohej
toast-widevine-failed = Luajtja dështoi
toast-widevine-install-failed = Widevine nuk mund të instalohej
toast-widevine-remove-failed = Widevine nuk mund të hiqej
trouble-not-loaded = Nuk mund të ngarkohej
trouble-offline = Pa lidhje
trouble-offline-detail = Kontrollo lidhjen e internetit dhe provo përsëri.
trouble-retry = Provo përsëri
update-available = Sonora { $version } është jashtë
update-detail = Je te { $running }. Lexo çfarë ndryshoi, ose përditëso tani.
update-detail-notes = Je te { $running }. Lexo çfarë ndryshoi, pastaj përditëso Sonora ashtu si e instalove.
update-failed = Përditësimi nuk mund të instalohej. Provo përsëri nga faqja e publikimeve.
update-never = Mos kontrollo për përditësime
update-notes = Çfarë ka të re
update-now = Përditëso
update-working = Duke shkarkuar përditësimin…
wake-reason = Po luhet muzikë
