# Tables, filters and counts
column-added-by = Přidáno uživatelem
column-album = Album
column-artist = Interpret
column-date-added = Datum přidání
column-genre = Žánr
column-index = #
column-length = Délka
column-modified = Změněno
column-name = Název
column-owner = Vlastník
column-played-at = Přehráno
column-plays = Přehrání
column-popularity = Popularita
column-title = Název
column-tracks = Skladby
column-year = Rok
count-songs =
    { $count ->
        [one] { $count } skladba
        [few] { $count } skladby
       *[other] { $count } skladeb
    }
count-tracks =
    { $count ->
        [one] { $count } skladba
        [few] { $count } skladby
       *[other] { $count } skladeb
    }
filter-album = Filtrovat skladby alba
filter-downloaded = Stažené
filter-duration = Délka
filter-favorites = Jen oblíbené
filter-hide-explicit = Skrýt explicitní
filter-history = Filtrovat historii poslouchání
filter-library = Filtrovat tvoji knihovnu
filter-owned = Vytvořené tebou
filter-playable = Jen přehratelné
filter-reset = Resetovat filtry
filter-year = Rok
tool-filters = Filtry
tool-sort = Řazení

# Library and discovery
genre-empty = Tady zatím nic není
history-clear = Vymazat historii
history-clear-confirm = Z tohoto zařízení se odstraní všechna přehrání. Tuto akci nelze vzít zpět.
history-clear-title = Vymazat historii poslouchání
history-empty = Skladby, které přehraješ, se objeví tady.
history-not-loaded = Historii poslouchání se nepodařilo načíst.
home-collection-albums = Alba z tvé knihovny
home-favorite-albums = Oblíbená alba
home-quick-picks = Rychlý výběr
home-quick-picks-empty = Označ pár skladeb jako oblíbené a objeví se tady
home-recently-added = Nedávno přidané
home-recently-played = Nedávno přehrávané
library-liked-songs = Oblíbené
library-local-unconfigured = Nastav svou místní knihovnu
library-no-albums = Zatím nemáš žádné uložené albumy
library-no-artists = Zatím nemáš žádné oblíbené interprety
library-no-catalog-albums = Nenalezeny žádné albumy
library-no-catalog-artists = Nenalezeni žádní interpreti
library-no-catalog-songs = Nenalezeny žádné skladby
library-no-local-albums = Nenalezeny žádné importované albumy
library-no-local-artists = Nenalezeni žádní importovaní interpreti
library-no-local-playlists = Zatím žádné místní playlisty
library-no-local-songs = Nenalezeny žádné importované skladby
library-no-playlists = Zatím žádné playlisty
library-no-songs = Zatím nemáš žádné oblíbené
library-not-loaded = Tvoje knihovna se nenačetla
library-part-not-loaded = Tato část tvé knihovny se nenačetla
library-scanning = Procházím tvoje složky…
library-scanning-progress = Načítám tvoji hudbu… { $read } z { $found }
search-albums-playlists = Alba a playlisty
search-best-match = Nejlepší shoda
search-browse = Procházet vše
search-placeholder = Co chceš poslouchat?
search-recent = Nedávná hledání
search-results = Výsledky
search-saved =
    { $count ->
        [one] { $count } skladba v knihovně
        [few] { $count } skladby v knihovně
       *[other] { $count } skladeb v knihovně
    }
search-tag = { $kind } ·
user-eyebrow = Profil
user-followers = { $count ->
    [one] { $value } sledující
    [few] { $value } sledující
   *[other] { $value } sledujících
}
user-following = { $count ->
    [one] { $value } sledovaný
    [few] { $value } sledovaní
   *[other] { $value } sledovaných
}
user-playlists = Veřejné playlisty
user-playlists-empty = Zatím žádné veřejné playlisty
