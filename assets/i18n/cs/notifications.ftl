# Notifications and status
get-sonora = Získat Sonoru
notice-never = Už nezobrazovat
offline-go-online = Přejít online
offline-title = Jsi offline.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio běží, ale ALSA k němu nemá cestu, takže nic, co přehraješ, není slyšet. Nainstaluj pulseaudio-alsa a restartuj Sonoru.
   *[pipewire] PipeWire běží, ale ALSA k němu nemá cestu, takže nic, co přehraješ, není slyšet. Nainstaluj pipewire-alsa a restartuj Sonoru.
}
requirement-audio-bridge-title = Sonora se nemůže spojit s tvým zvukovým serverem
requirement-audio-output = Sonora nemohla otevřít tvoje zvukové zařízení, takže nic, co přehraješ, není slyšet. Zkontroluj, že je zařízení připojené a nedrží ho jiná aplikace.
requirement-audio-output-title = Žádný zvukový výstup
requirement-help = Co nainstalovat
requirement-web-engine = Nainstaluj webkit2gtk-4.1, aby ses mohl přihlásit k YouTube Music, Deezeru nebo Apple Music v okně prohlížeče. Do té doby místo toho vlož své cookies.
requirement-web-engine-title = Přihlášení přes prohlížeč není k dispozici
toast-code-copied = Kód zkopírován
toast-download-cancelled = Stahování { $name } zrušeno
toast-download-failed = Nepodařilo se zjistit, které skladby stáhnout.
toast-downloads-cancelled = { $count ->
    [one] Stahování { $count } skladby zrušeno
    [few] Stahování { $count } skladeb zrušeno
   *[other] Stahování { $count } skladeb zrušeno
}
toast-favorite-albums-removed = Z tvých oblíbených { $count ->
        [one] odebráno jedno album
        [few] odebrána { $count } alba
       *[other] odebráno { $count } alb
    }
toast-favorite-artists-removed = Z tvých oblíbených { $count ->
        [one] odebrán jeden interpret
        [few] odebráni { $count } interpreti
       *[other] odebráno { $count } interpretů
    }
toast-favorite-removed = Odebráno z tvých oblíbených: { $name }
toast-favorite-songs-removed = Z tvých oblíbených { $count ->
        [one] odebrána jedna skladba
        [few] odebrány { $count } skladby
       *[other] odebráno { $count } skladeb
    }
toast-folder-missing = { $name } už na disku není
toast-folder-removed = Odebráno: { $name }
toast-history-clear-failed = Historii se nepodařilo vymazat
toast-history-remove-failed = Skladbu se nepodařilo odebrat z historie
toast-history-removed = { $count ->
    [one] Odebráno z historie
    [few] Z historie odebrány { $count } skladby
   *[other] Z historie odebráno { $count } skladeb
}
toast-history-restore-failed = Historii se nepodařilo obnovit
toast-jam-ended = Jam skončil
toast-jam-failed = Jam se nepodařilo spustit ani se k němu připojit.
toast-jam-leave-failed = Jam se nepodařilo ukončit ani opustit.
toast-jam-left = Opustil jsi Jam
toast-jam-needs-connect = Pro připojení k Jamu zapni v nastavení „Zobrazit jako zařízení Spotify“
toast-jam-unreadable = Tento odkaz nevede na Jam, ke kterému by ses mohl připojit.
toast-keys-error = Chyba klíče pro přehrávání
toast-keys-refused = Spotify tu nemusí fungovat
toast-last-album = Album se přehrá jako poslední
toast-last-artist = Interpret se přehrá jako poslední
toast-last-playlist = Playlist se přehrá jako poslední
toast-last-track = { $name } se přehraje jako poslední
toast-last-tracks = { $count ->
    [one] { $count } skladba se přehraje jako poslední
    [few] { $count } skladby se přehrají jako poslední
   *[other] { $count } skladeb se přehraje jako poslední
}
toast-library-add-failed = { $name } se nepodařilo přidat do tvé knihovny
toast-library-added = Přidáno do tvé knihovny
toast-library-albums-removed = Z tvé knihovny { $count ->
        [one] odebráno jedno album
        [few] odebrána { $count } alba
       *[other] odebráno { $count } alb
    }
toast-library-pin-failed = Připnutí se nepodařilo změnit.
toast-library-remove-failed = { $name } se nepodařilo odebrat z tvé knihovny
toast-library-removed-one = Odebráno z tvé knihovny: { $name }
toast-library-scanned = { $count ->
    [one] Knihovna prohledána, nalezena jedna skladba
    [few] Knihovna prohledána, nalezeny { $count } skladby
   *[other] Knihovna prohledána, nalezeno { $count } skladeb
}
toast-library-songs-removed = Z tvé knihovny { $count ->
        [one] odebrána jedna skladba
        [few] odebrány { $count } skladby
       *[other] odebráno { $count } skladeb
    }
toast-link-copied = Odkaz zkopírován
toast-link-sign-in = Pro otevření tohoto odkazu se přihlas k { $name }
toast-link-unreadable = Tento odkaz Sonora neumí otevřít
toast-local-delete-failed = Některé soubory skladeb se nepodařilo smazat
toast-next-album = Album se přehrá jako další
toast-next-artist = Interpret se přehrá jako další
toast-next-playlist = Playlist se přehrá jako další
toast-next-track = { $name } se přehraje jako další
toast-next-tracks = { $count ->
    [one] { $count } skladba se přehraje jako další
    [few] { $count } skladby se přehrají jako další
   *[other] { $count } skladeb se přehraje jako další
}
toast-offline = Žádné připojení. Dokud se neobnoví, nic se nebude streamovat.
toast-offline-mode = Jsi offline. Pro streamování této skladby přejdi online.
toast-offline-unsupported = { $name } přehrává stažené skladby jen online.
toast-open-all-failed = { $count ->
    [one] { $count } soubor se nepodařilo otevřít
    [few] { $count } soubory se nepodařilo otevřít
   *[other] { $count } souborů se nepodařilo otevřít
}
toast-open-failed = { $name } se nepodařilo otevřít
toast-open-some-failed = Některé soubory se nepodařilo otevřít
toast-open-unsupported = { $name } není zvukový soubor, který Sonora umí přehrát
toast-output-default-failed = Výchozí systémové zařízení se nepodařilo vybrat
toast-output-select-failed = { $name } se nepodařilo vybrat
toast-playback-stopped = Přehrávání se zastavilo, protože několik skladeb za sebou nešlo přehrát
toast-playlist-added = Playlist přidán do tvé knihovny
toast-playlist-busy = Jiná změna ještě probíhá
toast-playlist-cover-changed = Obal playlistu změněn
toast-playlist-cover-removed = Obal playlistu odebrán
toast-playlist-created = Playlist vytvořen
toast-playlist-deleted = Playlist smazán
toast-playlist-exported = Playlist exportován
toast-playlist-failed = Tuto změnu se nepodařilo uložit
toast-playlist-import-empty = Žádná skladba z { $name } není v tvé knihovně
toast-playlist-imported-partly = Importováno: { $name }, bez { $count ->
        [one] jedné skladby, která není
       *[other] { $count } skladeb, které nejsou
    } v tvé knihovně
toast-playlist-removed = Playlist odebrán z tvé knihovny
toast-playlist-renamed = Playlist přejmenován
toast-playlist-signed-out = Pro změnu playlistů se přihlas
toast-playlist-updated = Playlist upraven
toast-playlist-visibility = Viditelnost playlistu změněna
toast-playlists-removed = Z tvé knihovny { $count ->
        [one] odebrán jeden playlist
        [few] odebrány { $count } playlisty
       *[other] odebráno { $count } playlistů
    }
toast-queue-cleared = Fronta vymazána
toast-queue-failed = Toto se nepodařilo přidat do fronty
toast-queue-local-elsewhere = Místní soubory nelze přidat do fronty na jiném zařízení
toast-queue-removed-track = Z fronty odebráno: { $name }
toast-queue-removed-tracks = { $count ->
    [one] Z fronty odebrána { $count } skladba
    [few] Z fronty odebrány { $count } skladby
   *[other] Z fronty odebráno { $count } skladeb
}
toast-queued-album = Album přidáno do fronty
toast-queued-artist = Interpret přidán do fronty
toast-queued-playlist = Playlist přidán do fronty
toast-queued-track = Do fronty přidáno: { $name }
toast-queued-tracks = { $count ->
    [one] { $count } skladba přidána do fronty
    [few] { $count } skladby přidány do fronty
   *[other] { $count } skladeb přidáno do fronty
}
toast-radio-failed = Rádio pro { $name } se nepodařilo spustit
toast-restore-failed = Službu { $name } se nepodařilo znovu přihlásit, a tak Sonora pokračovala bez ní
toast-sample-burst = Oznámení číslo { $name }
toast-sample-delete = Smazáno: { $name }
toast-sample-done = Uloženo do tvé knihovny
toast-sample-event = Událost vytvořena
toast-sample-event-detail = Neděle 3. prosince v 9:00
toast-sample-failed = Server se nepodařilo kontaktovat
toast-sample-info = Přijď 10 minut před začátkem události.
toast-sample-lasting = Toto oznámení zůstane, dokud ho nezavřeš
toast-sample-loading = Nahrávám tvoje skladby
toast-sample-long = Toto oznámení říká mnohem víc než ostatní, takže se text musí zalomit na druhý a možná i třetí řádek, aniž by vytlačil tlačítko zavřít z karty
toast-sample-plain = Událost byla vytvořena
toast-sample-save-failed = { $name } se nepodařilo uložit
toast-sample-save-failed-detail = Server odpověděl 503 Service Unavailable
toast-sample-saved = Uloženo: { $name }
toast-sample-saving = Ukládá se { $name }
toast-sample-undone = Vráceno zpět
toast-sample-warning = Tvoje relace skončí za pět minut
toast-scan-failed = Tvoje hudební složky se nepodařilo prohledat
toast-settings-broken = Oprav řádek { $name } v settings.json, aby se změny uložily
toast-settings-overwritten = Soubor settings.json se na disku změnil, a tak se tvoje poslední změna neuložila
toast-settings-save-failed = Nastavení se nepodařilo uložit
toast-sign-in-to-play = { $name } lze streamovat jen po přihlášení
toast-similar-failed = Ke skladbě { $name } se nepodařilo najít podobné skladby
toast-sleep-ended = Časovač usnutí skončil
toast-still-throttled = { $name } stále omezuje přehrávání. Stiskni přehrát a zkus to znovu
toast-tags-failed = Tagy se nepodařilo uložit
toast-tags-saved = Tagy pro { $name } uloženy
toast-throttled = { $name } teď omezuje přehrávání. Za chvíli to zkusím znovu
toast-track-added = Přidáno do playlistu: { $name }
toast-track-removed = Odebráno z playlistu: { $name }
toast-track-removed-unnamed = Odebráno z playlistu
toast-track-unplayable = { $name } se nepodařilo přehrát
toast-tracks-removed = Z playlistu { $name } { $count ->
        [one] odebrána jedna skladba
        [few] odebrány { $count } skladby
       *[other] odebráno { $count } skladeb
    }
toast-tracks-removed-unnamed = Z playlistu { $count ->
        [one] odebrána jedna skladba
        [few] odebrány { $count } skladby
       *[other] odebráno { $count } skladeb
    }
toast-tray-unavailable = Systémová lišta není k dispozici, ikonu nelze zobrazit
toast-unpinned = Odepnuto: { $name }
toast-why = Proč?
toast-widevine-borrowed = Zkus aktualizovat Widevine
toast-widevine-download-failed = Widevine se nepodařilo stáhnout
toast-widevine-failed = Přehrávání se nezdařilo
toast-widevine-install-failed = Widevine se nepodařilo nainstalovat
toast-widevine-remove-failed = Widevine se nepodařilo odebrat
trouble-not-loaded = Načtení se nezdařilo
trouble-offline = Žádné připojení
trouble-offline-detail = Zkontroluj připojení k internetu a zkus to znovu.
trouble-retry = Zkusit znovu
update-available = Vyšla Sonora { $version }
update-detail = Máš { $running }. Přečti si, co se změnilo, nebo aktualizuj hned teď.
update-detail-notes = Máš { $running }. Přečti, co se změnilo, a pak Sonoru aktualizuj stejným způsobem, jakým byla nainstalována.
update-failed = Aktualizaci se nepodařilo nainstalovat. Zkus to znovu ze stránky s vydáními.
update-never = Nekontrolovat aktualizace
update-notes = Co je nového
update-now = Aktualizovat
update-working = Aktualizace se stahuje…
wake-reason = Hraje hudba
