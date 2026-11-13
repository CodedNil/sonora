# Tables, filters and counts
column-added-by = Hinzugefügt von
column-album = Album
column-artist = Künstler
column-date-added = Hinzugefügt am
column-genre = Genre
column-index = #
column-length = Länge
column-modified = Geändert
column-name = Name
column-owner = Ersteller
column-played-at = Abgespielt
column-plays = Wiedergaben
column-popularity = Beliebtheit
column-title = Titel
column-tracks = Titel
column-year = Jahr
count-songs =
    { $count ->
        [one] { $count } Song
       *[other] { $count } Songs
    }
count-tracks =
    { $count ->
        [one] { $count } Titel
       *[other] { $count } Titel
    }
filter-album = Albumtitel filtern
filter-downloaded = Heruntergeladen
filter-duration = Dauer
filter-favorites = Nur Favoriten
filter-hide-explicit = Explizite ausblenden
filter-history = Wiedergabeverlauf filtern
filter-library = Bibliothek filtern
filter-owned = Von dir
filter-playable = Nur abspielbare
filter-reset = Filter zurücksetzen
filter-year = Jahr
tool-filters = Filter
tool-sort = Sortieren

# Library and discovery
genre-empty = Hier gibt es noch nichts zu sehen
history-clear = Verlauf löschen
history-clear-confirm = Alle Wiedergaben werden von diesem Gerät entfernt. Das lässt sich nicht rückgängig machen.
history-clear-title = Wiedergabeverlauf löschen
history-empty = Abgespielte Titel erscheinen hier.
history-not-loaded = Der Wiedergabeverlauf konnte nicht geladen werden.
home-collection-albums = Alben aus deiner Sammlung
home-favorite-albums = Lieblingsalben
home-quick-picks = Schnellauswahl
home-quick-picks-empty = Markiere ein paar Songs als Favoriten, dann erscheinen sie hier
home-recently-added = Kürzlich hinzugefügt
home-recently-played = Kürzlich gespielt
library-liked-songs = Favoriten
library-local-unconfigured = Richte deine lokale Bibliothek ein
library-no-albums = Noch keine gespeicherten Alben
library-no-artists = Noch keine Lieblingskünstler
library-no-catalog-albums = Keine Alben gefunden
library-no-catalog-artists = Keine Künstler gefunden
library-no-catalog-songs = Keine Songs gefunden
library-no-local-albums = Keine importierten Alben gefunden
library-no-local-artists = Keine importierten Künstler gefunden
library-no-local-playlists = Noch keine lokalen Playlists
library-no-local-songs = Keine importierten Songs gefunden
library-no-playlists = Noch keine Playlists
library-no-songs = Noch keine Favoriten
library-not-loaded = Deine Bibliothek wurde nicht geladen
library-part-not-loaded = Dieser Teil deiner Bibliothek wurde nicht geladen
library-scanning = Deine Ordner werden durchsucht…
library-scanning-progress = Deine Musik wird gelesen… { $read } von { $found }
search-albums-playlists = Alben & Playlists
search-best-match = Beste Übereinstimmung
search-browse = Alles durchsuchen
search-placeholder = Was möchtest du hören?
search-recent = Letzte Suchen
search-results = Ergebnisse
search-saved =
    { $count ->
        [one] { $count } Song in der Bibliothek
       *[other] { $count } Songs in der Bibliothek
    }
search-tag = { $kind } ·
user-eyebrow = Profil
user-followers = { $count ->
    [one] { $value } Follower
   *[other] { $value } Follower
}
user-following = { $count ->
   *[other] Folgt { $value }
}
user-playlists = Öffentliche Playlists
user-playlists-empty = Noch keine öffentlichen Playlists
