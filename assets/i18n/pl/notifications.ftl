# Notifications and status
get-sonora = Pobierz Sonorę
notice-never = Nie pokazuj ponownie
offline-go-online = Przejdź do trybu online
offline-title = Jesteś offline.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio działa, ale ALSA nie ma do niego dostępu, więc nic nie słychać. Zainstaluj pulseaudio-alsa i uruchom Sonorę ponownie.
   *[pipewire] PipeWire działa, ale ALSA nie ma do niego dostępu, więc nic nie słychać. Zainstaluj pipewire-alsa i uruchom Sonorę ponownie.
}
requirement-audio-bridge-title = Sonora nie może połączyć się z serwerem dźwięku
requirement-audio-output = Sonora nie mogła otworzyć urządzenia audio, więc nic nie słychać. Sprawdź, czy urządzenie jest podłączone i nie zajmuje go inna aplikacja.
requirement-audio-output-title = Brak wyjścia dźwięku
requirement-help = Co zainstalować
requirement-web-engine = Zainstaluj webkit2gtk-4.1, aby logować się do YouTube Music, Deezer lub Apple Music w oknie przeglądarki. Do tego czasu wklej pliki cookie ręcznie.
requirement-web-engine-title = Logowanie w przeglądarce jest niedostępne
toast-code-copied = Skopiowano kod
toast-download-cancelled = Anulowano pobieranie { $name }
toast-download-failed = Nie udało się ustalić listy utworów do pobrania.
toast-downloads-cancelled = { $count ->
    [one] Anulowano pobieranie { $count } utworu
    [few] Anulowano pobieranie { $count } utworów
   *[other] Anulowano pobieranie { $count } utworów
}
toast-favorite-albums-removed = Usunięto { $count ->
        [one] jeden album
        [few] { $count } albumy
       *[other] { $count } albumów
    } z ulubionych
toast-favorite-artists-removed = Usunięto { $count ->
        [one] jednego wykonawcę
        [few] { $count } wykonawców
       *[other] { $count } wykonawców
    } z ulubionych
toast-favorite-removed = Usunięto { $name } z ulubionych
toast-favorite-songs-removed = Usunięto { $count ->
        [one] jeden utwór
        [few] { $count } utwory
       *[other] { $count } utworów
    } z ulubionych
toast-folder-missing = Folderu { $name } nie ma już na dysku
toast-folder-removed = Usunięto { $name }
toast-history-clear-failed = Nie udało się wyczyścić historii
toast-history-remove-failed = Nie udało się usunąć utworu z historii
toast-history-removed = { $count ->
    [one] Usunięto z historii
    [few] Usunięto { $count } utwory z historii
   *[other] Usunięto { $count } utworów z historii
}
toast-history-restore-failed = Nie udało się przywrócić historii
toast-jam-ended = Jam zakończony
toast-jam-failed = Nie udało się rozpocząć Jamu ani do niego dołączyć.
toast-jam-leave-failed = Nie udało się zakończyć ani opuścić Jamu.
toast-jam-left = Opuszczono Jam
toast-jam-needs-connect = Włącz „Pokazuj jako urządzenie Spotify” w ustawieniach, aby dołączyć do Jamu
toast-jam-unreadable = Ten link nie prowadzi do Jamu, do którego można dołączyć.
toast-keys-error = Błąd klucza
toast-keys-refused = Spotify może nie grać
toast-last-album = Album zabrzmi na końcu
toast-last-artist = Wykonawca zabrzmi na końcu
toast-last-playlist = Playlista zabrzmi na końcu
toast-last-track = { $name } zabrzmi na końcu
toast-last-tracks = Utwory zabrzmią na końcu: { $count }
toast-library-add-failed = Nie udało się dodać { $name } do biblioteki
toast-library-added = Dodano do biblioteki
toast-library-albums-removed = Usunięto { $count ->
        [one] jeden album
        [few] { $count } albumy
       *[other] { $count } albumów
    } z biblioteki
toast-library-pin-failed = Nie udało się zmienić przypięcia w Spotify.
toast-library-remove-failed = Nie udało się usunąć { $name } z biblioteki
toast-library-removed-one = Usunięto { $name } z biblioteki
toast-library-scanned = { $count ->
    [one] Przeskanowano bibliotekę, znaleziono jeden utwór
    [few] Przeskanowano bibliotekę, znaleziono { $count } utwory
   *[other] Przeskanowano bibliotekę, znaleziono { $count } utworów
}
toast-library-songs-removed = Usunięto { $count ->
        [one] jeden utwór
        [few] { $count } utwory
       *[other] { $count } utworów
    } z biblioteki
toast-link-copied = Skopiowano link
toast-link-sign-in = Zaloguj się do { $name }, aby otworzyć ten link
toast-link-unreadable = Ten link nie prowadzi do niczego, co Sonora potrafi otworzyć
toast-local-delete-failed = Nie udało się usunąć części plików utworów
toast-next-album = Album zabrzmi następny
toast-next-artist = Wykonawca zabrzmi następny
toast-next-playlist = Playlista zabrzmi następna
toast-next-track = { $name } zabrzmi następny
toast-next-tracks = Utwory zabrzmią jako następne: { $count }
toast-offline = Brak połączenia. Nic nie zostanie odtworzone, dopóki nie wróci.
toast-offline-mode = Jesteś offline. Przejdź do trybu online, aby streamować ten utwór.
toast-offline-unsupported = { $name } odtwarza pobrane utwory tylko w trybie online.
toast-open-all-failed = Nie udało się otworzyć plików: { $count }
toast-open-failed = Nie udało się otworzyć { $name }
toast-open-some-failed = Nie udało się otworzyć części plików
toast-open-unsupported = { $name } to nie plik audio, który Sonora potrafi odtworzyć
toast-output-default-failed = Nie udało się wybrać domyślnego urządzenia systemowego
toast-output-select-failed = Nie udało się wybrać { $name }
toast-playback-stopped = Odtwarzanie zatrzymane, bo kilku kolejnych utworów nie udało się odtworzyć
toast-playlist-added = Dodano playlistę do biblioteki
toast-playlist-busy = Inna zmiana wciąż trwa
toast-playlist-cover-changed = Zmieniono okładkę playlisty
toast-playlist-cover-removed = Usunięto okładkę playlisty
toast-playlist-created = Utworzono playlistę
toast-playlist-deleted = Usunięto playlistę
toast-playlist-exported = Wyeksportowano playlistę
toast-playlist-failed = Nie udało się zapisać zmiany
toast-playlist-import-empty = Żadnego utworu z { $name } nie ma w bibliotece
toast-playlist-imported-partly = Zaimportowano { $name } bez { $count ->
        [one] jednego utworu, którego
       *[other] { $count } utworów, których
    } nie ma w bibliotece
toast-playlist-removed = Usunięto playlistę z biblioteki
toast-playlist-renamed = Zmieniono nazwę playlisty
toast-playlist-signed-out = Zaloguj się, aby zmieniać playlisty
toast-playlist-updated = Zaktualizowano playlistę
toast-playlist-visibility = Zmieniono widoczność playlisty
toast-playlists-removed = Usunięto { $count ->
        [one] jedną playlistę
        [few] { $count } playlisty
       *[other] { $count } playlist
    } z biblioteki
toast-queue-cleared = Wyczyszczono kolejkę
toast-queue-failed = Nie udało się dodać do kolejki
toast-queue-local-elsewhere = Lokalnych plików nie można dodać do kolejki na innym urządzeniu
toast-queue-removed-track = Usunięto { $name } z kolejki
toast-queue-removed-tracks = Usunięto z kolejki utwory: { $count }
toast-queued-album = Album dodano do kolejki
toast-queued-artist = Wykonawcę dodano do kolejki
toast-queued-playlist = Playlistę dodano do kolejki
toast-queued-track = { $name } dodano do kolejki
toast-queued-tracks = Dodano do kolejki utwory: { $count }
toast-radio-failed = Nie udało się uruchomić radia dla { $name }
toast-restore-failed = Nie udało się ponownie zalogować do { $name }, więc Sonora działa dalej bez tego serwisu
toast-sample-burst = Powiadomienie nr { $name }
toast-sample-delete = Usunięto { $name }
toast-sample-done = Zapisano w bibliotece
toast-sample-event = Utworzono wydarzenie
toast-sample-event-detail = Niedziela, 3 grudnia, 9:00
toast-sample-failed = Nie udało się połączyć z serwerem
toast-sample-info = Przyjdź 10 minut przed wydarzeniem.
toast-sample-lasting = To powiadomienie zostaje, dopóki go nie zamkniesz
toast-sample-loading = Przesyłanie utworów
toast-sample-long = To powiadomienie mówi o wiele więcej niż pozostałe, więc tekst musi zawinąć się do drugiego, a może i trzeciego wiersza, nie wypychając przycisku zamknięcia poza kartę
toast-sample-plain = Utworzono wydarzenie
toast-sample-save-failed = Nie udało się zapisać { $name }
toast-sample-save-failed-detail = Serwer odpowiedział 503 Service Unavailable
toast-sample-saved = Zapisano { $name }
toast-sample-saving = Zapisywanie { $name }
toast-sample-undone = Cofnięto
toast-sample-warning = Twoja sesja kończy się za pięć minut
toast-scan-failed = Nie udało się przeskanować folderów z muzyką
toast-settings-broken = Popraw wiersz { $name } w settings.json, aby zapisać zmiany
toast-settings-overwritten = Plik settings.json zmienił się na dysku, więc ostatnia zmiana nie została zapisana
toast-settings-save-failed = Nie udało się zapisać ustawień
toast-sign-in-to-play = { $name } udostępnia muzykę tylko po zalogowaniu
toast-similar-failed = Nie znaleziono utworów podobnych do { $name }
toast-sleep-ended = Wyłącznik czasowy zadziałał
toast-still-throttled = { $name } nadal ogranicza odtwarzanie. Naciśnij Odtwórz, aby spróbować ponownie
toast-tags-failed = Nie udało się zapisać tagów
toast-tags-saved = Zapisano tagi „{ $name }”
toast-throttled = { $name } chwilowo ogranicza odtwarzanie. Ponowna próba za moment
toast-track-added = Dodano do { $name }
toast-track-removed = Usunięto z { $name }
toast-track-removed-unnamed = Usunięto z playlisty
toast-track-unplayable = Nie udało się odtworzyć { $name }
toast-tracks-removed = Usunięto { $count ->
        [one] jeden utwór
        [few] { $count } utwory
       *[other] { $count } utworów
    } z { $name }
toast-tracks-removed-unnamed = Usunięto { $count ->
        [one] jeden utwór
        [few] { $count } utwory
       *[other] { $count } utworów
    } z playlisty
toast-tray-unavailable = Brak zasobnika systemowego, w którym można umieścić ikonę
toast-unpinned = Odpięto { $name }
toast-why = Dlaczego?
toast-widevine-borrowed = Spróbuj zaktualizować Widevine
toast-widevine-download-failed = Nie udało się pobrać Widevine
toast-widevine-failed = Odtwarzanie nie powiodło się
toast-widevine-install-failed = Nie udało się zainstalować Widevine
toast-widevine-remove-failed = Nie udało się usunąć Widevine
trouble-not-loaded = Nie udało się wczytać
trouble-offline = Brak połączenia
trouble-offline-detail = Sprawdź połączenie z internetem i spróbuj ponownie.
trouble-retry = Spróbuj ponownie
update-available = Sonora { $version } jest dostępna
update-detail = Masz wersję { $running }. Zobacz, co się zmieniło, albo zaktualizuj teraz.
update-detail-notes = Masz { $running }. Zobacz, co się zmieniło, a potem zaktualizuj Sonorę tak, jak ją zainstalowano.
update-failed = Nie udało się zainstalować aktualizacji. Spróbuj ze strony wydań.
update-never = Nie sprawdzaj aktualizacji
update-notes = Co nowego
update-now = Aktualizuj
update-working = Pobieranie aktualizacji…
wake-reason = Odtwarzana jest muzyka
