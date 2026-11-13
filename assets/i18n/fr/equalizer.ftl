# Sound and equalizer
equalizer-add-filter = Ajouter un filtre
equalizer-advanced = Avancé
equalizer-bass = Graves
equalizer-copied = Profil copié au format Equalizer APO
equalizer-decibels = dB
equalizer-delete = Supprimer le profil
equalizer-export = Exporter
equalizer-export-file = Exporter un fichier…
equalizer-exported = Profil exporté
equalizer-file-unreadable = Impossible de lire le fichier
equalizer-file-unwritable = Impossible d'écrire le fichier
equalizer-filter-limit = Un profil contient au plus { $max } filtres.
equalizer-hertz = Hz
equalizer-import = Importer
equalizer-import-file = Importer un fichier…
equalizer-import-nothing = Aucun réglage d'égaliseur à importer
equalizer-import-too-large = Non importé : le texte dépasse 1 Mo, trop pour un profil d'égaliseur
equalizer-import-too-many = Non importé : { $count ->
        [one] un filtre est actif
       *[other] { $count } filtres sont actifs
    }, plus qu'un profil ne peut en contenir
equalizer-imported-skipped = { $name } importé sans { $count ->
        [one] une ligne
       *[other] { $count } lignes
    }
equalizer-invalid-frequency = La fréquence doit être comprise entre { $low } et { $high } Hz
equalizer-invalid-gain = Le gain doit être compris entre { $low } et { $high } dB
equalizer-invalid-octaves = La largeur de bande doit être comprise entre { $low } et { $high } octaves
equalizer-invalid-preamp = Le préampli doit être compris entre { $low } et { $high } dB
equalizer-invalid-q = Le Q doit être compris entre { $low } et { $high }
equalizer-invalid-shelf-octaves = Un plateau prend un Q, pas une largeur de bande
equalizer-kind-all-pass = Passe-tout
equalizer-kind-band-pass = Passe-bande
equalizer-kind-high-pass = Passe-haut
equalizer-kind-high-shelf = Plateau aigu
equalizer-kind-low-pass = Passe-bas
equalizer-kind-low-shelf = Plateau grave
equalizer-kind-notch = Réjecteur
equalizer-kind-peak = Cloche
equalizer-level = Niveau
equalizer-lower-preamp = Baisser le préampli
equalizer-no-filters = Aucun filtre pour l'instant. Ajoutez-en un, choisissez un préréglage ci-dessus ou importez un profil.
equalizer-no-gain = —
equalizer-octaves = oct.
equalizer-output-port = { $device } · { $port }
equalizer-overload = Écrêtage de { $db } dB
equalizer-overload-slight = Écrêtage inférieur à 0,1 dB
equalizer-paste-empty = Le presse-papiers ne contient pas de texte
equalizer-preamp = Préampli
equalizer-preamp-auto = Automatique
equalizer-preamp-manual = Manuel
equalizer-profile-default = Par défaut
equalizer-profile-name-hint = Nom du profil
equalizer-profile-new = Profil { $number }
equalizer-profile-pasted = Profil collé
equalizer-q = Q
equalizer-redo = Rétablir
equalizer-remove-filter = Retirer le filtre
equalizer-rename = Renommer le profil
equalizer-rule-add = Ajouter une règle
equalizer-rule-choose-device = Choisir un périphérique
equalizer-rule-choose-sound = Choisir un son
equalizer-rule-disconnected = Non connecté
equalizer-rule-other = Tout autre périphérique
equalizer-rule-playing = En lecture
equalizer-rule-remove = Retirer la règle
equalizer-rule-replaces = Remplace sa règle
equalizer-rules = Règles par périphérique
equalizer-save-as = Enregistrer comme profil
equalizer-simple = Simple
equalizer-skip-expression = une valeur écrite sous forme d'expression
equalizer-skip-no-frequency = pas de fréquence
equalizer-skip-no-gain = pas de gain
equalizer-skip-no-preamp = le préampli n'est pas un nombre
equalizer-skip-no-width = ni Q ni largeur de bande
equalizer-skip-unknown-type = type de filtre inconnu { $name }
equalizer-skip-unreadable = pas un filtre que Sonora sait lire
equalizer-skip-unsupported = { $name } n'est pas pris en charge
equalizer-skipped-line = Ligne { $line } : { $reason }
equalizer-skipped-more = et { $count ->
        [one] une de plus
       *[other] { $count } de plus
    }
equalizer-status-altered = Pas bit-perfect à cause de :
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Bit-perfect depuis Sonora
equalizer-status-bits = { $from } → { $to } bits
equalizer-status-channels = { $from ->
    [1] Mono
    [2] Stéréo
   *[other] { $from } canaux
} → { $to ->
    [1] mono
    [2] stéréo
   *[other] { $to } canaux
}
equalizer-status-equalized = EQ
equalizer-status-float = Flottant → { $to } bits
equalizer-status-normalized = Normalisé
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = Le serveur de son le convertit ensuite en { $khz } kHz
equalizer-status-volume = Volume
equalizer-tone-reset = Remettre à 0 dB
equalizer-transfer = Importer et exporter
equalizer-treble = Aigus
equalizer-value-refused = Valeur précédente conservée
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
