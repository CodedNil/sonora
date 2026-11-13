# Sound and equalizer
equalizer-add-filter = Aggiungi filtro
equalizer-advanced = Avanzato
equalizer-bass = Bassi
equalizer-copied = Profilo copiato come testo Equalizer APO
equalizer-decibels = dB
equalizer-delete = Elimina profilo
equalizer-export = Esporta
equalizer-export-file = Esporta file…
equalizer-exported = Profilo esportato
equalizer-file-unreadable = Impossibile leggere il file
equalizer-file-unwritable = Impossibile scrivere il file
equalizer-filter-limit = Un profilo contiene fino a { $max } filtri.
equalizer-hertz = Hz
equalizer-import = Importa
equalizer-import-file = Importa file…
equalizer-import-nothing = Nessuna impostazione dell'equalizzatore da importare
equalizer-import-too-large = Non importato: il testo supera 1 MB, troppo per un profilo dell'equalizzatore
equalizer-import-too-many = Non importato: { $count ->
        [one] è attivo un filtro
       *[other] sono attivi { $count } filtri
    }, più di quanti ne contenga un profilo
equalizer-imported-skipped = { $name } importato senza { $count ->
        [one] una riga
       *[other] { $count } righe
    }
equalizer-invalid-frequency = La frequenza deve essere tra { $low } e { $high } Hz
equalizer-invalid-gain = Il guadagno deve essere tra { $low } e { $high } dB
equalizer-invalid-octaves = La larghezza di banda deve essere tra { $low } e { $high } ottave
equalizer-invalid-preamp = Il preamp deve essere tra { $low } e { $high } dB
equalizer-invalid-q = Il Q deve essere tra { $low } e { $high }
equalizer-invalid-shelf-octaves = Uno shelf usa un Q, non una larghezza di banda
equalizer-kind-all-pass = Passa-tutto
equalizer-kind-band-pass = Passa-banda
equalizer-kind-high-pass = Passa-alto
equalizer-kind-high-shelf = Shelf alto
equalizer-kind-low-pass = Passa-basso
equalizer-kind-low-shelf = Shelf basso
equalizer-kind-notch = Notch
equalizer-kind-peak = Picco
equalizer-level = Livello
equalizer-lower-preamp = Abbassa il preamp
equalizer-no-filters = Ancora nessun filtro. Aggiungine uno, scegli un preset qui sopra o importa un profilo.
equalizer-no-gain = —
equalizer-octaves = ott
equalizer-output-port = { $device } · { $port }
equalizer-overload = Clipping di { $db } dB
equalizer-overload-slight = Clipping inferiore a 0,1 dB
equalizer-paste-empty = Gli appunti non contengono testo
equalizer-preamp = Preamp
equalizer-preamp-auto = Automatico
equalizer-preamp-manual = Manuale
equalizer-profile-default = Predefinito
equalizer-profile-name-hint = Nome profilo
equalizer-profile-new = Profilo { $number }
equalizer-profile-pasted = Profilo incollato
equalizer-q = Q
equalizer-redo = Ripeti
equalizer-remove-filter = Rimuovi filtro
equalizer-rename = Rinomina profilo
equalizer-rule-add = Aggiungi regola
equalizer-rule-choose-device = Scegli un dispositivo
equalizer-rule-choose-sound = Scegli un suono
equalizer-rule-disconnected = Non connesso
equalizer-rule-other = Qualsiasi altro dispositivo
equalizer-rule-playing = In riproduzione
equalizer-rule-remove = Rimuovi regola
equalizer-rule-replaces = Sostituisce la sua regola
equalizer-rules = Regole per dispositivo
equalizer-save-as = Salva come profilo
equalizer-simple = Semplice
equalizer-skip-expression = un valore scritto come espressione
equalizer-skip-no-frequency = nessuna frequenza
equalizer-skip-no-gain = nessun guadagno
equalizer-skip-no-preamp = il preamp non è un numero
equalizer-skip-no-width = nessun Q o larghezza di banda
equalizer-skip-unknown-type = tipo di filtro sconosciuto { $name }
equalizer-skip-unreadable = non è un filtro che Sonora può leggere
equalizer-skip-unsupported = { $name } non è supportato
equalizer-skipped-line = Riga { $line }: { $reason }
equalizer-skipped-more = e { $count ->
        [one] un'altra
       *[other] altre { $count }
    }
equalizer-status-altered = Non bit-perfect a causa di:
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Bit-perfect da Sonora
equalizer-status-bits = { $from } → { $to } bit
equalizer-status-channels = { $from ->
    [1] Mono
    [2] Stereo
   *[other] { $from } can.
} → { $to ->
    [1] mono
    [2] stereo
   *[other] { $to } can.
}
equalizer-status-equalized = EQ
equalizer-status-float = Float → { $to } bit
equalizer-status-normalized = Normalizzato
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = Il server audio poi lo converte a { $khz } kHz
equalizer-status-volume = Volume
equalizer-tone-reset = Reimposta a 0 dB
equalizer-transfer = Importa ed esporta
equalizer-treble = Alti
equalizer-value-refused = Mantenuto il valore precedente
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
