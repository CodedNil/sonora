# Tables, filters and counts
column-added-by = Ajouté par
column-album = Album
column-artist = Artiste
column-date-added = Date d'ajout
column-genre = Genre
column-index = #
column-length = Durée
column-modified = Modifié
column-name = Nom
column-owner = Propriétaire
column-played-at = Écouté
column-plays = Écoutes
column-popularity = Popularité
column-title = Titre
column-tracks = Titres
column-year = Année
count-songs =
    { $count ->
        [one] { $count } titre
       *[other] { $count } titres
    }
count-tracks =
    { $count ->
        [one] { $count } titre
       *[other] { $count } titres
    }
filter-album = Filtrer les titres de l'album
filter-downloaded = Téléchargés
filter-duration = Durée
filter-favorites = Favoris uniquement
filter-hide-explicit = Masquer les explicites
filter-history = Filtrer l’historique d’écoute
filter-library = Filtrer la bibliothèque
filter-owned = Par vous
filter-playable = Lisibles uniquement
filter-reset = Réinitialiser les filtres
filter-year = Année
tool-filters = Filtres
tool-sort = Trier

# Library and discovery
genre-empty = Rien à afficher ici pour l'instant
history-clear = Effacer l’historique
history-clear-confirm = Toutes les écoutes seront supprimées de cet appareil. Cette action est irréversible.
history-clear-title = Effacer l’historique d’écoute
history-empty = Les titres écoutés apparaîtront ici.
history-not-loaded = Impossible de charger l’historique d’écoute.
home-collection-albums = Albums de votre collection
home-favorite-albums = Albums favoris
home-quick-picks = Sélection rapide
home-quick-picks-empty = Likez quelques titres et ils apparaîtront ici
home-recently-added = Ajoutés récemment
home-recently-played = Écoutés récemment
library-liked-songs = Favoris
library-local-unconfigured = Configurez votre bibliothèque locale
library-no-albums = Aucun album enregistré pour l'instant
library-no-artists = Aucun artiste favori pour l'instant
library-no-catalog-albums = Aucun album trouvé
library-no-catalog-artists = Aucun artiste trouvé
library-no-catalog-songs = Aucun titre trouvé
library-no-local-albums = Aucun album importé trouvé
library-no-local-artists = Aucun artiste importé
library-no-local-playlists = Aucune playlist locale
library-no-local-songs = Aucun titre importé trouvé
library-no-playlists = Aucune playlist pour l'instant
library-no-songs = Aucun favori pour l’instant
library-not-loaded = Votre bibliothèque ne s’est pas chargée
library-part-not-loaded = Cette partie de votre bibliothèque ne s’est pas chargée
library-scanning = Analyse de vos dossiers…
library-scanning-progress = Lecture de votre musique… { $read } sur { $found }
search-albums-playlists = Albums et playlists
search-best-match = Meilleur résultat
search-browse = Tout parcourir
search-placeholder = Que voulez-vous écouter ?
search-recent = Recherches récentes
search-results = Résultats
search-saved =
    { $count ->
        [one] { $count } titre dans la bibliothèque
       *[other] { $count } titres dans la bibliothèque
    }
search-tag = { $kind } ·
user-eyebrow = Profil
user-followers = { $count ->
    [one] { $value } abonné
   *[other] { $value } abonnés
}
user-following = { $count ->
    [one] { $value } abonnement
   *[other] { $value } abonnements
}
user-playlists = Playlists publiques
user-playlists-empty = Aucune playlist publique pour le moment
