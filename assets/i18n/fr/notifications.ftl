# Notifications and status
get-sonora = Obtenir Sonora
notice-never = Ne plus afficher
offline-go-online = Passer en ligne
offline-title = Vous êtes hors ligne.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio fonctionne, mais ALSA n'a aucune route vers lui, donc rien de ce que vous écoutez n'est audible. Installez pulseaudio-alsa et redémarrez Sonora.
   *[pipewire] PipeWire fonctionne, mais ALSA n'a aucune route vers lui, donc rien de ce que vous écoutez n'est audible. Installez pipewire-alsa et redémarrez Sonora.
}
requirement-audio-bridge-title = Sonora ne peut pas joindre votre serveur de son
requirement-audio-output = Sonora n'a pas pu ouvrir votre périphérique audio, donc rien de ce que vous écoutez n'est audible. Vérifiez qu'un périphérique est connecté et qu'aucune autre appli ne l'occupe.
requirement-audio-output-title = Aucune sortie audio
requirement-help = Que faut-il installer
requirement-web-engine = Installez webkit2gtk-4.1 pour vous connecter à YouTube Music, Deezer ou Apple Music dans une fenêtre de navigateur. D'ici là, collez plutôt vos cookies.
requirement-web-engine-title = La connexion par navigateur est indisponible
toast-code-copied = Code copié
toast-download-cancelled = Téléchargement de { $name } annulé
toast-download-failed = Impossible de lister les titres à télécharger.
toast-downloads-cancelled = { $count ->
    [one] Téléchargement de { $count } titre annulé
   *[other] Téléchargement de { $count } titres annulé
}
toast-favorite-albums-removed = { $count ->
        [one] Un album retiré
       *[other] { $count } albums retirés
    } de vos favoris
toast-favorite-artists-removed = { $count ->
        [one] Un artiste retiré
       *[other] { $count } artistes retirés
    } de vos favoris
toast-favorite-removed = { $name } retiré de vos favoris
toast-favorite-songs-removed = { $count ->
        [one] Un titre retiré
       *[other] { $count } titres retirés
    } de vos favoris
toast-folder-missing = { $name } n'est plus sur le disque
toast-folder-removed = { $name } retiré
toast-history-clear-failed = L'historique n'a pas pu être effacé
toast-history-remove-failed = Un titre n'a pas pu être retiré de l'historique
toast-history-removed = { $count ->
    [one] Retiré de l'historique
   *[other] { $count } titres retirés de l'historique
}
toast-history-restore-failed = L'historique n'a pas pu être restauré
toast-jam-ended = Jam terminée
toast-jam-failed = La Jam n'a pas pu être lancée ou rejointe.
toast-jam-leave-failed = La Jam n'a pas pu être terminée ou quittée.
toast-jam-left = Vous avez quitté la Jam
toast-jam-needs-connect = Activez « Afficher comme appareil Spotify » dans les paramètres pour rejoindre une Jam
toast-jam-unreadable = Ce lien ne mène à aucune Jam que vous pouvez rejoindre.
toast-keys-error = Erreur de clé de lecture
toast-keys-refused = Spotify peut échouer
toast-last-album = L'album sera lu en dernier
toast-last-artist = L'artiste sera lu en dernier
toast-last-playlist = La playlist sera lue en dernier
toast-last-track = { $name } sera lu en dernier
toast-last-tracks = { $count } titres seront lus en dernier
toast-library-add-failed = { $name } n'a pas pu être ajouté à votre bibliothèque
toast-library-added = Ajouté à votre bibliothèque
toast-library-albums-removed = { $count ->
        [one] Un album retiré
       *[other] { $count } albums retirés
    } de votre bibliothèque
toast-library-pin-failed = Impossible de mettre à jour l'épingle dans Spotify.
toast-library-remove-failed = { $name } n'a pas pu être retiré de votre bibliothèque
toast-library-removed-one = { $name } retiré de votre bibliothèque
toast-library-scanned = { $count ->
    [one] Bibliothèque analysée, un titre trouvé
   *[other] Bibliothèque analysée, { $count } titres trouvés
}
toast-library-songs-removed = { $count ->
        [one] Un titre retiré
       *[other] { $count } titres retirés
    } de votre bibliothèque
toast-link-copied = Lien copié
toast-link-sign-in = Connectez-vous à { $name } pour ouvrir ce lien
toast-link-unreadable = Ce lien ne mène à rien que Sonora puisse ouvrir
toast-local-delete-failed = Certains fichiers de titres n'ont pas pu être supprimés
toast-next-album = L'album sera lu juste après
toast-next-artist = L'artiste sera lu juste après
toast-next-playlist = La playlist sera lue juste après
toast-next-track = { $name } sera lu juste après
toast-next-tracks = { $count } titres seront lus juste après
toast-offline = Pas de connexion. Aucune lecture en streaming tant qu'elle n'est pas rétablie.
toast-offline-mode = Vous êtes hors ligne. Passez en ligne pour écouter ce titre en streaming.
toast-offline-unsupported = { $name } ne lit les téléchargements qu'en ligne.
toast-open-all-failed = { $count } fichiers n'ont pas pu être ouverts
toast-open-failed = Impossible d'ouvrir { $name }
toast-open-some-failed = Certains fichiers n'ont pas pu être ouverts
toast-open-unsupported = { $name } n'est pas un fichier audio que Sonora peut lire
toast-output-default-failed = Impossible de sélectionner le périphérique par défaut du système
toast-output-select-failed = Impossible de sélectionner { $name }
toast-playback-stopped = Lecture arrêtée : plusieurs titres d'affilée n'ont pas pu être lus
toast-playlist-added = Playlist ajoutée à votre bibliothèque
toast-playlist-busy = Une autre modification est encore en cours
toast-playlist-cover-changed = Couverture de la playlist modifiée
toast-playlist-cover-removed = Couverture de la playlist retirée
toast-playlist-created = Playlist créée
toast-playlist-deleted = Playlist supprimée
toast-playlist-exported = Playlist exportée
toast-playlist-failed = Cette modification n'a pas pu être enregistrée
toast-playlist-import-empty = Aucun titre de { $name } n'est dans votre bibliothèque
toast-playlist-imported-partly = Import terminé : { $name }, sans { $count ->
        [one] un titre absent
       *[other] { $count } titres absents
    } de votre bibliothèque
toast-playlist-removed = Playlist retirée de votre bibliothèque
toast-playlist-renamed = Playlist renommée
toast-playlist-signed-out = Connectez-vous pour modifier des playlists
toast-playlist-updated = Playlist mise à jour
toast-playlist-visibility = Visibilité de la playlist modifiée
toast-playlists-removed = { $count ->
        [one] Une playlist retirée
       *[other] { $count } playlists retirées
    } de votre bibliothèque
toast-queue-cleared = File d'attente vidée
toast-queue-failed = Impossible d'ajouter cela à la file d'attente
toast-queue-local-elsewhere = Les fichiers locaux ne peuvent pas être ajoutés à la file d'un autre appareil
toast-queue-removed-track = { $name } retiré de la file d'attente
toast-queue-removed-tracks = { $count } titres retirés de la file d'attente
toast-queued-album = Album ajouté à la file d'attente
toast-queued-artist = Artiste ajouté à la file d'attente
toast-queued-playlist = Playlist ajoutée à la file d'attente
toast-queued-track = { $name } ajouté à la file d'attente
toast-queued-tracks = { $count } titres ajoutés à la file d'attente
toast-radio-failed = Impossible de lancer la radio de { $name }
toast-restore-failed = { $name } n'a pas pu se reconnecter, Sonora a donc continué sans
toast-sample-burst = Toast numéro { $name }
toast-sample-delete = { $name } supprimé
toast-sample-done = Enregistré dans votre bibliothèque
toast-sample-event = Événement créé
toast-sample-event-detail = Dimanche 3 décembre à 9 h 00
toast-sample-failed = Le serveur est injoignable
toast-sample-info = Arrivez 10 minutes avant l'événement.
toast-sample-lasting = Celui-ci reste affiché jusqu'à ce que vous le fermiez
toast-sample-loading = Envoi de vos titres
toast-sample-long = Ce toast en dit bien plus que les autres, donc le texte doit passer sur une deuxième, voire une troisième ligne sans pousser le bouton de fermeture hors de la carte
toast-sample-plain = L'événement a été créé
toast-sample-save-failed = { $name } n'a pas pu être enregistré
toast-sample-save-failed-detail = Le serveur a répondu 503 Service Unavailable
toast-sample-saved = { $name } enregistré
toast-sample-saving = Enregistrement de { $name }
toast-sample-undone = Annulé
toast-sample-warning = Votre session se termine dans cinq minutes
toast-scan-failed = Vos dossiers de musique n'ont pas pu être analysés
toast-settings-broken = Corrigez la ligne { $name } de settings.json pour enregistrer les modifications
toast-settings-overwritten = settings.json a changé sur le disque, votre dernière modification n'a donc pas été enregistrée
toast-settings-save-failed = Les paramètres n'ont pas pu être enregistrés
toast-sign-in-to-play = { $name } ne diffuse qu'aux auditeurs connectés
toast-similar-failed = Aucun titre similaire à { $name } trouvé
toast-sleep-ended = Minuteur de veille terminé
toast-still-throttled = { $name } limite toujours la lecture. Appuyez sur Lecture pour réessayer
toast-tags-failed = Les tags n’ont pas pu être enregistrés
toast-tags-saved = Tags de { $name } enregistrés
toast-throttled = { $name } limite la lecture pour l'instant. Nouvel essai dans un moment
toast-track-added = Ajouté à { $name }
toast-track-moved = Déplacé vers { $name }
toast-track-removed = Retiré de { $name }
toast-track-removed-unnamed = Retiré de la playlist
toast-track-unplayable = { $name } n'a pas pu être lu
toast-tracks-removed = { $count ->
        [one] Un titre retiré
       *[other] { $count } titres retirés
    } de { $name }
toast-tracks-removed-unnamed = { $count ->
        [one] Un titre retiré
       *[other] { $count } titres retirés
    } de la playlist
toast-tray-unavailable = Il n'y a pas de zone de notification où placer l'icône
toast-unpinned = { $name } détaché
toast-why = Pourquoi ?
toast-widevine-borrowed = Essayez de mettre à jour Widevine
toast-widevine-download-failed = Widevine n'a pas pu être téléchargé
toast-widevine-failed = Échec de la lecture
toast-widevine-install-failed = Widevine n'a pas pu être installé
toast-widevine-remove-failed = Widevine n'a pas pu être supprimé
trouble-not-loaded = Chargement impossible
trouble-offline = Pas de connexion
trouble-offline-detail = Vérifiez votre connexion internet et réessayez.
trouble-retry = Réessayer
update-available = Sonora { $version } est disponible
update-detail = Vous êtes en { $running }. Voyez ce qui a changé, ou mettez à jour maintenant.
update-detail-notes = Vous êtes en { $running }. Voyez ce qui a changé, puis mettez Sonora à jour comme vous l'avez installée.
update-failed = La mise à jour n'a pas pu être installée. Réessayez depuis la page des versions.
update-never = Ne pas rechercher de mises à jour
update-notes = Nouveautés
update-now = Mettre à jour
update-working = Téléchargement de la mise à jour…
wake-reason = Musique en cours de lecture
