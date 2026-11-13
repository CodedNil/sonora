# Tables, filters and counts
column-added-by = Dodane przez
column-album = Album
column-artist = Wykonawca
column-date-added = Data dodania
column-genre = Gatunek
column-index = #
column-length = Czas
column-modified = Zmodyfikowano
column-name = Nazwa
column-owner = Właściciel
column-played-at = Odtworzono
column-plays = Odtworzenia
column-popularity = Popularność
column-title = Tytuł
column-tracks = Utwory
column-year = Rok
count-songs =
    { $count ->
        [one] { $count } utwór
        [few] { $count } utwory
       *[other] { $count } utworów
    }
count-tracks =
    { $count ->
        [one] { $count } utwór
        [few] { $count } utwory
       *[other] { $count } utworów
    }
filter-album = Filtruj utwory albumu
filter-downloaded = Pobrane
filter-duration = Czas trwania
filter-favorites = Tylko ulubione
filter-hide-explicit = Ukryj z wulgaryzmami
filter-history = Filtruj historię słuchania
filter-library = Filtruj bibliotekę
filter-owned = Moje
filter-playable = Tylko dostępne
filter-reset = Wyczyść filtry
filter-year = Rok
tool-filters = Filtry
tool-sort = Sortowanie

# Library and discovery
genre-empty = Nie ma tu jeszcze nic
history-clear = Wyczyść historię
history-clear-confirm = Wszystkie odtworzenia zostaną usunięte z tego urządzenia. Nie można tego cofnąć.
history-clear-title = Wyczyść historię słuchania
history-empty = Odtworzone utwory pojawią się tutaj.
history-not-loaded = Nie udało się wczytać historii słuchania.
home-collection-albums = Albumy z Twojej kolekcji
home-favorite-albums = Ulubione albumy
home-quick-picks = Szybki wybór
home-quick-picks-empty = Polub kilka utworów, a pojawią się tutaj
home-recently-added = Ostatnio dodane
home-recently-played = Ostatnio odtwarzane
library-liked-songs = Ulubione
library-local-unconfigured = Skonfiguruj lokalną bibliotekę
library-no-albums = Brak zapisanych albumów
library-no-artists = Brak ulubionych wykonawców
library-no-catalog-albums = Nie znaleziono albumów
library-no-catalog-artists = Nie znaleziono wykonawców
library-no-catalog-songs = Nie znaleziono utworów
library-no-local-albums = Nie znaleziono zaimportowanych albumów
library-no-local-artists = Nie znaleziono zaimportowanych wykonawców
library-no-local-playlists = Nie ma jeszcze lokalnych playlist
library-no-local-songs = Nie znaleziono zaimportowanych utworów
library-no-playlists = Brak playlist
library-no-songs = Brak ulubionych
library-not-loaded = Biblioteka się nie wczytała
library-part-not-loaded = Ta część biblioteki się nie wczytała
library-scanning = Przeglądam twoje foldery…
library-scanning-progress = Czytam twoją muzykę… { $read } z { $found }
search-albums-playlists = Albumy i playlisty
search-best-match = Najlepszy wynik
search-browse = Przeglądaj
search-placeholder = Czego chcesz posłuchać?
search-recent = Ostatnie wyszukiwania
search-results = Wyniki
search-saved =
    { $count ->
        [one] { $count } utwór w bibliotece
        [few] { $count } utwory w bibliotece
       *[other] { $count } utworów w bibliotece
    }
search-tag = { $kind } ·
user-eyebrow = Profil
user-followers = { $count ->
    [one] { $value } obserwujący
    [few] { $value } obserwujących
   *[other] { $value } obserwujących
}
user-following = { $count ->
    [one] { $value } obserwowany
    [few] { $value } obserwowanych
   *[other] { $value } obserwowanych
}
user-playlists = Publiczne playlisty
user-playlists-empty = Brak publicznych playlist
