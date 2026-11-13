# Tables, filters and counts
column-added-by = Shtuar nga
column-album = Albumi
column-artist = Artisti
column-date-added = Data e shtimit
column-genre = Zhanri
column-index = #
column-length = Kohëzgjatja
column-modified = Ndryshuar
column-name = Emri
column-owner = Pronari
column-played-at = Luajtur
column-plays = Luajtje
column-popularity = Popullariteti
column-title = Titulli
column-tracks = Këngë
column-year = Viti
count-songs =
    { $count ->
        [one] { $count } këngë
       *[other] { $count } këngë
    }
count-tracks =
    { $count ->
        [one] { $count } këngë
       *[other] { $count } këngë
    }
filter-album = Filtro këngët e albumit
filter-downloaded = Të shkarkuara
filter-duration = Kohëzgjatja
filter-favorites = Vetëm të preferuarat
filter-hide-explicit = Fshih eksplicitet
filter-history = Filtro historikun e dëgjimit
filter-library = Filtro bibliotekën
filter-owned = Nga ti
filter-playable = Vetëm të luajtshme
filter-reset = Rivendos filtrat
filter-year = Viti
tool-filters = Filtra
tool-sort = Rendit

# Library and discovery
genre-empty = Ende asgjë për të shfaqur këtu
history-clear = Pastro historikun
history-clear-confirm = Çdo luajtje hiqet nga ky pajisje. Kjo nuk mund të zhbëhet.
history-clear-title = Pastro historikun e dëgjimit
history-empty = Këngët që luan do të shfaqen këtu.
history-not-loaded = Historiku i dëgjimit nuk mund të ngarkohej.
home-collection-albums = Albume nga koleksioni yt
home-favorite-albums = Albumet e preferuara
home-quick-picks = Zgjedhje të shpejta
home-quick-picks-empty = Pëlqe disa këngë dhe ato do të shfaqen këtu
home-recently-added = Shtuar së fundi
home-recently-played = Luajtur së fundi
library-liked-songs = Të preferuarat
library-local-unconfigured = Konfiguro bibliotekën lokale
library-no-albums = Ende asnjë album i ruajtur
library-no-artists = Ende asnjë artist i preferuar
library-no-catalog-albums = Nuk u gjet asnjë album
library-no-catalog-artists = Nuk u gjet asnjë artist
library-no-catalog-songs = Nuk u gjet asnjë këngë
library-no-local-albums = Nuk u gjet asnjë album i importuar
library-no-local-artists = Nuk u gjet asnjë artist i importuar
library-no-local-playlists = Ende asnjë listë luajtjeje lokale
library-no-local-songs = Nuk u gjet asnjë këngë e importuar
library-no-playlists = Ende asnjë listë luajtjeje
library-no-songs = Ende asnjë e preferuar
library-not-loaded = Biblioteka jote nuk u ngarkua
library-part-not-loaded = Kjo pjesë e bibliotekës nuk u ngarkua
library-scanning = Duke shfletuar dosjet e tua…
library-scanning-progress = Duke lexuar muzikën… { $read } nga { $found }
search-albums-playlists = Albume dhe lista luajtjeje
search-best-match = Përputhja më e mirë
search-browse = Shfleto të gjitha
search-placeholder = Çfarë do të dëgjosh?
search-recent = Kërkimet e fundit
search-results = Rezultatet
search-saved =
    { $count ->
        [one] { $count } këngë në Bibliotekë
       *[other] { $count } këngë në Bibliotekë
    }
search-tag = { $kind } ·
user-eyebrow = Profili
user-followers = { $count ->
    [one] { $value } ndjekës
   *[other] { $value } ndjekës
}
user-following = { $count ->
   *[other] { $value } duke ndjekur
}
user-playlists = Lista publike luajtjeje
user-playlists-empty = Ende asnjë listë publike luajtjeje
