# Sound and equalizer
equalizer-add-filter = Přidat filtr
equalizer-advanced = Pokročilé
equalizer-bass = Basy
equalizer-copied = Profil zkopírován jako text Equalizer APO
equalizer-decibels = dB
equalizer-delete = Smazat profil
equalizer-export = Exportovat
equalizer-export-file = Exportovat soubor…
equalizer-exported = Profil exportován
equalizer-file-unreadable = Soubor se nepodařilo přečíst
equalizer-file-unwritable = Soubor se nepodařilo zapsat
equalizer-filter-limit = Profil pojme nejvýše { $max } filtrů.
equalizer-hertz = Hz
equalizer-import = Importovat
equalizer-import-file = Importovat soubor…
equalizer-import-nothing = Nenalezeno žádné nastavení ekvalizéru k importu
equalizer-import-too-large = Neimportováno: text má přes 1 MB, na profil ekvalizéru je příliš velký
equalizer-import-too-many = Neimportováno: { $count ->
        [one] je zapnutý jeden filtr
        [few] jsou zapnuté { $count } filtry
       *[other] je zapnuto { $count } filtrů
    }, víc, než profil pojme
equalizer-imported-skipped = Importováno: { $name } bez { $count ->
        [one] jednoho řádku
       *[other] { $count } řádků
    }
equalizer-invalid-frequency = Frekvence musí být mezi { $low } a { $high } Hz
equalizer-invalid-gain = Zisk musí být mezi { $low } a { $high } dB
equalizer-invalid-octaves = Šířka pásma musí být mezi { $low } a { $high } oktávami
equalizer-invalid-preamp = Předzesílení musí být mezi { $low } a { $high } dB
equalizer-invalid-q = Q musí být mezi { $low } a { $high }
equalizer-invalid-shelf-octaves = Shelf používá Q, ne šířku pásma
equalizer-kind-all-pass = Všepropust
equalizer-kind-band-pass = Pásmová propust
equalizer-kind-high-pass = Horní propust
equalizer-kind-high-shelf = Horní shelf
equalizer-kind-low-pass = Dolní propust
equalizer-kind-low-shelf = Dolní shelf
equalizer-kind-notch = Pásmová zádrž
equalizer-kind-peak = Špička
equalizer-level = Úroveň
equalizer-lower-preamp = Snížit předzesílení
equalizer-no-filters = Zatím žádné filtry. Přidej nějaký, vyber předvolbu výše nebo importuj profil.
equalizer-no-gain = —
equalizer-octaves = okt.
equalizer-output-port = { $device } · { $port }
equalizer-overload = Přebuzení o { $db } dB
equalizer-overload-slight = Přebuzení pod 0,1 dB
equalizer-paste-empty = Ve schránce není žádný text
equalizer-preamp = Předzesílení
equalizer-preamp-auto = Automaticky
equalizer-preamp-manual = Ručně
equalizer-profile-default = Výchozí
equalizer-profile-name-hint = Název profilu
equalizer-profile-new = Profil { $number }
equalizer-profile-pasted = Vložený profil
equalizer-q = Q
equalizer-redo = Znovu
equalizer-remove-filter = Odebrat filtr
equalizer-rename = Přejmenovat profil
equalizer-rule-add = Přidat pravidlo
equalizer-rule-choose-device = Vyber zařízení
equalizer-rule-choose-sound = Vyber zvuk
equalizer-rule-disconnected = Nepřipojeno
equalizer-rule-other = Jakékoli jiné zařízení
equalizer-rule-playing = Právě hraje
equalizer-rule-remove = Odebrat pravidlo
equalizer-rule-replaces = Nahrazuje jeho pravidlo
equalizer-rules = Pravidla zařízení
equalizer-save-as = Uložit jako profil
equalizer-simple = Jednoduché
equalizer-skip-expression = hodnota zapsaná jako výraz
equalizer-skip-no-frequency = chybí frekvence
equalizer-skip-no-gain = chybí zisk
equalizer-skip-no-preamp = předzesílení není číslo
equalizer-skip-no-width = chybí Q nebo šířka pásma
equalizer-skip-unknown-type = neznámý typ filtru { $name }
equalizer-skip-unreadable = není filtr, který Sonora umí přečíst
equalizer-skip-unsupported = { $name } není podporováno
equalizer-skipped-line = Řádek { $line }: { $reason }
equalizer-skipped-more = a { $count ->
        [one] jeden další
        [few] { $count } další
       *[other] { $count } dalších
    }
equalizer-status-altered = Není bit-perfect kvůli:
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Bit-perfect ze Sonory
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
equalizer-status-normalized = Normalizace
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = Zvukový server ho pak převede na { $khz } kHz
equalizer-status-volume = Hlasitost
equalizer-tone-reset = Vrátit na 0 dB
equalizer-transfer = Import a export
equalizer-treble = Výšky
equalizer-value-refused = Ponechána předchozí hodnota
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
