# Tables, filters and counts
column-added-by = Adicionado por
column-album = Álbum
column-artist = Artista
column-date-added = Data de adição
column-genre = Gênero
column-index = #
column-length = Duração
column-modified = Modificado
column-name = Nome
column-owner = Proprietário
column-played-at = Tocada em
column-plays = Reproduções
column-popularity = Popularidade
column-title = Título
column-tracks = Faixas
column-year = Ano
count-songs =
    { $count ->
        [one] { $count } música
       *[other] { $count } músicas
    }
count-tracks =
    { $count ->
        [one] { $count } faixa
       *[other] { $count } faixas
    }
filter-album = Filtrar faixas do álbum
filter-downloaded = Baixadas
filter-duration = Duração
filter-favorites = Apenas favoritas
filter-hide-explicit = Ocultar explícitas
filter-history = Filtrar histórico de reprodução
filter-library = Filtrar sua biblioteca
filter-owned = Suas
filter-playable = Reproduzíveis apenas
filter-reset = Redefinir filtros
filter-year = Ano
tool-filters = Filtros
tool-sort = Ordenar

# Library and discovery
genre-empty = Nada para mostrar aqui ainda
history-clear = Limpar histórico
history-clear-confirm = Todas as reproduções serão removidas deste dispositivo. Isto não pode ser desfeito.
history-clear-title = Limpar histórico de reprodução
history-empty = As faixas que você tocar aparecerão aqui.
history-not-loaded = O histórico de reprodução não pôde ser carregado.
home-collection-albums = Álbuns da sua coleção
home-favorite-albums = Álbuns favoritos
home-quick-picks = Escolhas rápidas
home-quick-picks-empty = Curta algumas músicas e elas aparecerão aqui
home-recently-added = Adicionados recentemente
home-recently-played = Tocados recentemente
library-liked-songs = Favoritas
library-local-unconfigured = Configure sua biblioteca local
library-no-albums = Nenhum álbum salvo ainda
library-no-artists = Nenhum artista favorito ainda
library-no-catalog-albums = Nenhum álbum encontrado
library-no-catalog-artists = Nenhum artista encontrado
library-no-catalog-songs = Nenhuma música encontrada
library-no-local-albums = Nenhum álbum importado encontrado
library-no-local-artists = Nenhum artista importado encontrado
library-no-local-playlists = Nenhuma playlist local ainda
library-no-local-songs = Nenhuma música importada encontrada
library-no-playlists = Nenhuma playlist ainda
library-no-songs = Nenhuma favorita ainda
library-not-loaded = Sua biblioteca não carregou
library-part-not-loaded = Esta parte da sua biblioteca não carregou
library-scanning = Procurando nas suas pastas…
library-scanning-progress = Lendo suas músicas… { $read } de { $found }
search-albums-playlists = Álbuns e playlists
search-best-match = Melhor resultado
search-browse = Explorar tudo
search-placeholder = O que você quer ouvir?
search-recent = Buscas recentes
search-results = Resultados
search-saved =
    { $count ->
        [one] { $count } música na Biblioteca
       *[other] { $count } músicas na Biblioteca
    }
search-tag = { $kind } ·
user-eyebrow = Perfil
user-followers = { $count ->
    [one] { $value } seguidor
   *[other] { $value } seguidores
}
user-following = { $count ->
   *[other] { $value } seguindo
}
user-playlists = Playlists públicas
user-playlists-empty = Nenhuma playlist pública ainda
