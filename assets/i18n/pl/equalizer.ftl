# Sound and equalizer
equalizer-add-filter = Dodaj filtr
equalizer-advanced = Zaawansowany
equalizer-bass = Bas
equalizer-copied = Skopiowano profil jako tekst Equalizer APO
equalizer-decibels = dB
equalizer-delete = Usuń profil
equalizer-export = Eksportuj
equalizer-export-file = Eksportuj plik…
equalizer-exported = Wyeksportowano profil
equalizer-file-unreadable = Nie udało się odczytać pliku
equalizer-file-unwritable = Nie udało się zapisać pliku
equalizer-filter-limit = Profil mieści do { $max } filtrów.
equalizer-hertz = Hz
equalizer-import = Importuj
equalizer-import-file = Importuj plik…
equalizer-import-nothing = Nie znaleziono ustawień korektora do zaimportowania
equalizer-import-too-large = Nie zaimportowano: tekst ma ponad 1 MB, za dużo jak na profil korektora
equalizer-import-too-many = Nie zaimportowano: { $count ->
        [one] włączony jest jeden filtr
        [few] włączone są { $count } filtry
       *[other] włączonych jest { $count } filtrów
    }, więcej niż mieści profil
equalizer-imported-skipped = Zaimportowano { $name } bez { $count ->
        [one] jednego wiersza
        [few] { $count } wierszy
       *[other] { $count } wierszy
    }
equalizer-invalid-frequency = Częstotliwość musi wynosić od { $low } do { $high } Hz
equalizer-invalid-gain = Wzmocnienie musi wynosić od { $low } do { $high } dB
equalizer-invalid-octaves = Szerokość pasma musi wynosić od { $low } do { $high } oktaw
equalizer-invalid-preamp = Przedwzmacniacz musi wynosić od { $low } do { $high } dB
equalizer-invalid-q = Q musi wynosić od { $low } do { $high }
equalizer-invalid-shelf-octaves = Filtr półkowy przyjmuje Q, a nie szerokość pasma
equalizer-kind-all-pass = Wszechprzepustowy
equalizer-kind-band-pass = Pasmowoprzepustowy
equalizer-kind-high-pass = Górnoprzepustowy
equalizer-kind-high-shelf = Półkowy wysoki
equalizer-kind-low-pass = Dolnoprzepustowy
equalizer-kind-low-shelf = Półkowy niski
equalizer-kind-notch = Wycinający
equalizer-kind-peak = Szczytowy
equalizer-level = Poziom
equalizer-lower-preamp = Obniż przedwzmacniacz
equalizer-no-filters = Brak filtrów. Dodaj filtr, wybierz preset powyżej lub zaimportuj profil.
equalizer-no-gain = —
equalizer-octaves = okt.
equalizer-output-port = { $device } · { $port }
equalizer-overload = Przesterowanie o { $db } dB
equalizer-overload-slight = Przesterowanie poniżej 0,1 dB
equalizer-paste-empty = Schowek nie zawiera tekstu
equalizer-preamp = Przedwzmacniacz
equalizer-preamp-auto = Automatyczny
equalizer-preamp-manual = Ręczny
equalizer-profile-default = Domyślny
equalizer-profile-name-hint = Nazwa profilu
equalizer-profile-new = Profil { $number }
equalizer-profile-pasted = Wklejony profil
equalizer-q = Q
equalizer-redo = Ponów
equalizer-remove-filter = Usuń filtr
equalizer-rename = Zmień nazwę profilu
equalizer-rule-add = Dodaj regułę
equalizer-rule-choose-device = Wybierz urządzenie
equalizer-rule-choose-sound = Wybierz brzmienie
equalizer-rule-disconnected = Niepodłączone
equalizer-rule-other = Każde inne urządzenie
equalizer-rule-playing = Teraz gra
equalizer-rule-remove = Usuń regułę
equalizer-rule-replaces = Zastępuje jego regułę
equalizer-rules = Reguły urządzeń
equalizer-save-as = Zapisz jako profil
equalizer-simple = Prosty
equalizer-skip-expression = wartość zapisana jako wyrażenie
equalizer-skip-no-frequency = brak częstotliwości
equalizer-skip-no-gain = brak wzmocnienia
equalizer-skip-no-preamp = przedwzmacniacz nie jest liczbą
equalizer-skip-no-width = brak Q ani szerokości pasma
equalizer-skip-unknown-type = nieznany typ filtra { $name }
equalizer-skip-unreadable = to nie jest filtr, który Sonora potrafi odczytać
equalizer-skip-unsupported = { $name } nie jest obsługiwany
equalizer-skipped-line = Wiersz { $line }: { $reason }
equalizer-skipped-more = i { $count ->
        [one] jeszcze jeden
        [few] jeszcze { $count }
       *[other] jeszcze { $count }
    }
equalizer-status-altered = Brak bit-perfect z powodu:
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Bit-perfect z Sonory
equalizer-status-bits = { $from } → { $to } bit
equalizer-status-channels = { $from ->
    [1] Mono
    [2] Stereo
   *[other] { $from } kan.
} → { $to ->
    [1] mono
    [2] stereo
   *[other] { $to } kan.
}
equalizer-status-equalized = EQ
equalizer-status-float = Float → { $to } bit
equalizer-status-normalized = Normalizacja
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = Serwer dźwięku konwertuje go potem do { $khz } kHz
equalizer-status-volume = Głośność
equalizer-tone-reset = Przywróć 0 dB
equalizer-transfer = Import i eksport
equalizer-treble = Soprany
equalizer-value-refused = Zachowano poprzednią wartość
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
