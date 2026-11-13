# Sound and equalizer
equalizer-add-filter = Filter hinzufügen
equalizer-advanced = Erweitert
equalizer-bass = Bass
equalizer-copied = Profil als Equalizer-APO-Text kopiert
equalizer-decibels = dB
equalizer-delete = Profil löschen
equalizer-export = Exportieren
equalizer-export-file = Datei exportieren…
equalizer-exported = Profil exportiert
equalizer-file-unreadable = Die Datei ließ sich nicht lesen
equalizer-file-unwritable = Die Datei ließ sich nicht schreiben
equalizer-filter-limit = Ein Profil fasst bis zu { $max } Filter.
equalizer-hertz = Hz
equalizer-import = Importieren
equalizer-import-file = Datei importieren…
equalizer-import-nothing = Keine Equalizer-Einstellungen zum Importieren gefunden
equalizer-import-too-large = Nicht importiert: Der Text ist größer als 1 MB, zu groß für ein Equalizer-Profil
equalizer-import-too-many = Nicht importiert: { $count ->
        [one] ein Filter ist aktiv
       *[other] { $count } Filter sind aktiv
    }, mehr als ein Profil fasst
equalizer-imported-skipped = { $name } importiert, ohne { $count ->
        [one] eine Zeile
       *[other] { $count } Zeilen
    }
equalizer-invalid-frequency = Die Frequenz muss zwischen { $low } und { $high } Hz liegen
equalizer-invalid-gain = Die Verstärkung muss zwischen { $low } und { $high } dB liegen
equalizer-invalid-octaves = Die Bandbreite muss zwischen { $low } und { $high } Oktaven liegen
equalizer-invalid-preamp = Die Vorverstärkung muss zwischen { $low } und { $high } dB liegen
equalizer-invalid-q = Q muss zwischen { $low } und { $high } liegen
equalizer-invalid-shelf-octaves = Ein Shelf-Filter nimmt einen Q-Wert, keine Bandbreite
equalizer-kind-all-pass = Allpass
equalizer-kind-band-pass = Bandpass
equalizer-kind-high-pass = Hochpass
equalizer-kind-high-shelf = High-Shelf
equalizer-kind-low-pass = Tiefpass
equalizer-kind-low-shelf = Low-Shelf
equalizer-kind-notch = Kerbfilter
equalizer-kind-peak = Peak
equalizer-level = Pegel
equalizer-lower-preamp = Vorverstärkung senken
equalizer-no-filters = Noch keine Filter. Füge einen hinzu, wähle oben ein Preset oder importiere ein Profil.
equalizer-no-gain = —
equalizer-octaves = Okt.
equalizer-output-port = { $device } · { $port }
equalizer-overload = Übersteuert um { $db } dB
equalizer-overload-slight = Übersteuert um weniger als 0,1 dB
equalizer-paste-empty = Die Zwischenablage enthält keinen Text
equalizer-preamp = Vorverstärkung
equalizer-preamp-auto = Automatisch
equalizer-preamp-manual = Manuell
equalizer-profile-default = Standard
equalizer-profile-name-hint = Profilname
equalizer-profile-new = Profil { $number }
equalizer-profile-pasted = Eingefügtes Profil
equalizer-q = Q
equalizer-redo = Wiederholen
equalizer-remove-filter = Filter entfernen
equalizer-rename = Profil umbenennen
equalizer-rule-add = Regel hinzufügen
equalizer-rule-choose-device = Gerät wählen
equalizer-rule-choose-sound = Klang wählen
equalizer-rule-disconnected = Nicht verbunden
equalizer-rule-other = Jedes andere Gerät
equalizer-rule-playing = Spielt gerade
equalizer-rule-remove = Regel entfernen
equalizer-rule-replaces = Ersetzt seine Regel
equalizer-rules = Geräteregeln
equalizer-save-as = Als Profil speichern
equalizer-simple = Einfach
equalizer-skip-expression = ein als Ausdruck geschriebener Wert
equalizer-skip-no-frequency = keine Frequenz
equalizer-skip-no-gain = keine Verstärkung
equalizer-skip-no-preamp = die Vorverstärkung ist keine Zahl
equalizer-skip-no-width = kein Q und keine Bandbreite
equalizer-skip-unknown-type = unbekannter Filtertyp { $name }
equalizer-skip-unreadable = kein Filter, den Sonora lesen kann
equalizer-skip-unsupported = { $name } wird nicht unterstützt
equalizer-skipped-line = Zeile { $line }: { $reason }
equalizer-skipped-more = und { $count ->
        [one] eine weitere
       *[other] { $count } weitere
    }
equalizer-status-altered = Nicht bitgenau wegen:
equalizer-status-bit-perfect = Bitgenau
equalizer-status-bit-perfect-detail = Bitgenau aus Sonora
equalizer-status-bits = { $from } → { $to } Bit
equalizer-status-channels = { $from ->
    [1] Mono
    [2] Stereo
   *[other] { $from } Kanäle
} → { $to ->
    [1] Mono
    [2] Stereo
   *[other] { $to } Kanäle
}
equalizer-status-equalized = EQ
equalizer-status-float = Float → { $to } Bit
equalizer-status-normalized = Angeglichen
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = Der Soundserver wandelt es dann in { $khz } kHz um
equalizer-status-volume = Lautstärke
equalizer-tone-reset = Auf 0 dB zurücksetzen
equalizer-transfer = Import und Export
equalizer-treble = Höhen
equalizer-value-refused = Vorheriger Wert beibehalten
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
