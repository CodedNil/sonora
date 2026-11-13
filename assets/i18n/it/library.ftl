# Tables, filters and counts
column-added-by = Aggiunto da
column-album = Album
column-artist = Artista
column-date-added = Data aggiunta
column-genre = Genere
column-index = #
column-length = Durata
column-modified = Modificato
column-name = Nome
column-owner = Proprietario
column-played-at = Riprodotto
column-plays = Riproduzioni
column-popularity = Popolarità
column-title = Titolo
column-tracks = Brani
column-year = Anno
count-songs = { $count ->
    [one] { $count } brano
   *[other] { $count } brani
}
count-tracks = { $count ->
    [one] { $count } traccia
   *[other] { $count } tracce
}
filter-album = Filtra brani dell'album
filter-downloaded = Scaricati
filter-duration = Durata
filter-favorites = Solo preferiti
filter-hide-explicit = Nascondi espliciti
filter-history = Filtra cronologia di ascolto
filter-library = Filtra la tua libreria
filter-owned = Tuoi
filter-playable = Solo riproducibili
filter-reset = Reimposta filtri
filter-year = Anno
tool-filters = Filtri
tool-sort = Ordina

# Library and discovery
genre-empty = Nulla da mostrare qui
history-clear = Svuota cronologia
history-clear-confirm = Tutte le riproduzioni verranno rimosse da questo dispositivo. Questa operazione non può essere annullata.
history-clear-title = Svuota cronologia di ascolto
history-empty = I brani che ascolti appariranno qui.
history-not-loaded = Impossibile caricare la cronologia di ascolto.
home-collection-albums = Album dalla tua collezione
home-favorite-albums = Album preferiti
home-quick-picks = Scelte rapide
home-quick-picks-empty = Metti Mi piace a qualche brano e apparirà qui
home-recently-added = Aggiunti di recente
home-recently-played = Ascoltati di recente
library-liked-songs = Preferiti
library-local-unconfigured = Configura la tua libreria locale
library-no-albums = Ancora nessun album salvato
library-no-artists = Ancora nessun artista preferito
library-no-catalog-albums = Nessun album trovato
library-no-catalog-artists = Nessun artista trovato
library-no-catalog-songs = Nessun brano trovato
library-no-local-albums = Nessun album importato trovato
library-no-local-artists = Nessun artista importato trovato
library-no-local-playlists = Ancora nessuna playlist locale
library-no-local-songs = Nessun brano importato trovato
library-no-playlists = Ancora nessuna playlist
library-no-songs = Ancora nessun preferito
library-not-loaded = La tua libreria non è stata caricata
library-part-not-loaded = Questa parte della tua libreria non è stata caricata
library-scanning = Ricerca nelle tue cartelle…
library-scanning-progress = Lettura della tua musica… { $read } di { $found }
search-albums-playlists = Album e playlist
search-best-match = Miglior risultato
search-browse = Sfoglia tutto
search-placeholder = Cosa vuoi ascoltare?
search-recent = Ricerche recenti
search-results = Risultati
search-saved = { $count ->
    [one] { $count } brano in libreria
   *[other] { $count } brani in libreria
}
search-tag = { $kind } ·
user-eyebrow = Profilo
user-followers = { $count ->
    [one] { $value } follower
   *[other] { $value } follower
}
user-following = { $count ->
    [one] { $value } seguito
   *[other] { $value } seguiti
}
user-playlists = Playlist pubbliche
user-playlists-empty = Ancora nessuna playlist pubblica
