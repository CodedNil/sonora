# Tables, filters and counts
column-added-by = Añadido por
column-album = Álbum
column-artist = Artista
column-date-added = Fecha de adición
column-genre = Género
column-index = N.º
column-length = Duración
column-modified = Modificado
column-name = Nombre
column-owner = Propietario
column-played-at = Reproducido
column-plays = Reproducciones
column-popularity = Popularidad
column-title = Título
column-tracks = Pistas
column-year = Año
count-songs =
    { $count ->
        [one] { $count } canción
       *[other] { $count } canciones
    }
count-tracks =
    { $count ->
        [one] { $count } pista
       *[other] { $count } pistas
    }
filter-album = Filtrar las pistas del álbum
filter-downloaded = Descargadas
filter-duration = Duración
filter-favorites = Solo favoritos
filter-hide-explicit = Ocultar explícitas
filter-history = Filtrar el historial de reproducción
filter-library = Filtrar tu biblioteca
filter-owned = Tuyas
filter-playable = Solo reproducibles
filter-reset = Restablecer los filtros
filter-year = Año
tool-filters = Filtros
tool-sort = Ordenar

# Library and discovery
genre-empty = Todavía no hay nada que mostrar aquí
history-clear = Vaciar el historial
history-clear-confirm = Se quitarán todas las reproducciones de este dispositivo. Esta acción no se puede deshacer.
history-clear-title = Vaciar el historial de reproducción
history-empty = Las pistas que reproduzcas aparecerán aquí.
history-not-loaded = No se pudo cargar el historial de reproducción.
home-collection-albums = Álbumes de tu colección
home-favorite-albums = Álbumes favoritos
home-quick-picks = Selección rápida
home-quick-picks-empty = Marca algunas canciones como favoritas y aparecerán aquí
home-recently-added = Añadidos recientemente
home-recently-played = Reproducidos recientemente
library-liked-songs = Favoritos
library-local-unconfigured = Configura tu biblioteca local
library-no-albums = Todavía no tienes álbumes guardados
library-no-artists = Todavía no tienes artistas favoritos
library-no-catalog-albums = No se encontraron álbumes
library-no-catalog-artists = No se encontraron artistas
library-no-catalog-songs = No se encontraron canciones
library-no-local-albums = No se encontraron álbumes importados
library-no-local-artists = No se encontraron artistas importados
library-no-local-playlists = Todavía no tienes listas locales
library-no-local-songs = No se encontraron canciones importadas
library-no-playlists = Todavía no tienes listas de reproducción
library-no-songs = Todavía no tienes favoritos
library-not-loaded = Tu biblioteca no se cargó
library-part-not-loaded = Esta parte de tu biblioteca no se cargó
library-scanning = Revisando tus carpetas…
library-scanning-progress = Leyendo tu música… { $read } de { $found }
search-albums-playlists = Álbumes y listas
search-best-match = Mejor coincidencia
search-browse = Explorar todo
search-placeholder = ¿Qué quieres escuchar?
search-recent = Búsquedas recientes
search-results = Resultados
search-saved =
    { $count ->
        [one] { $count } canción en la biblioteca
       *[other] { $count } canciones en la biblioteca
    }
search-tag = { $kind } ·
user-eyebrow = Perfil
user-followers = { $count ->
    [one] { $value } seguidor
   *[other] { $value } seguidores
}
user-following = { $count ->
   *[other] { $value } siguiendo
}
user-playlists = Listas públicas
user-playlists-empty = Todavía no hay listas públicas
