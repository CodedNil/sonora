# Tables, filters and counts
column-added-by = Добавил
column-album = Альбом
column-artist = Исполнитель
column-date-added = Дата добавления
column-genre = Жанр
column-index = #
column-length = Длительность
column-modified = Изменён
column-name = Название
column-owner = Владелец
column-played-at = Прослушано
column-plays = Прослушивания
column-popularity = Популярность
column-title = Название
column-tracks = Треки
column-year = Год
count-songs =
    { $count ->
        [one] { $count } трек
        [few] { $count } трека
       *[other] { $count } треков
    }
count-tracks =
    { $count ->
        [one] { $count } трек
        [few] { $count } трека
       *[other] { $count } треков
    }
filter-album = Фильтр треков альбома
filter-downloaded = Скачанное
filter-duration = Длительность
filter-favorites = Только избранное
filter-hide-explicit = Скрыть с ненормативной лексикой
filter-history = Фильтр истории прослушиваний
filter-library = Фильтр медиатеки
filter-owned = Ваши
filter-playable = Только доступные
filter-reset = Сбросить фильтры
filter-year = Год
tool-filters = Фильтры
tool-sort = Сортировка

# Library and discovery
genre-empty = Здесь пока пусто
history-clear = Очистить историю
history-clear-confirm = Все прослушивания будут удалены с этого устройства. Отменить это нельзя.
history-clear-title = Очистить историю прослушиваний
history-empty = Прослушанные треки появятся здесь.
history-not-loaded = Не удалось загрузить историю прослушиваний.
home-collection-albums = Альбомы из вашей коллекции
home-favorite-albums = Избранные альбомы
home-quick-picks = Быстрый выбор
home-quick-picks-empty = Добавьте несколько треков, и они появятся здесь
home-recently-added = Недавно добавленные
home-recently-played = Недавно прослушанные
library-liked-songs = Избранное
library-local-unconfigured = Настройте локальную библиотеку
library-no-albums = Пока нет сохранённых альбомов
library-no-artists = Пока нет избранных исполнителей
library-no-catalog-albums = Альбомы не найдены
library-no-catalog-artists = Исполнители не найдены
library-no-catalog-songs = Треки не найдены
library-no-local-albums = Импортированные альбомы не найдены
library-no-local-artists = Импортированные исполнители не найдены
library-no-local-playlists = Локальных плейлистов пока нет
library-no-local-songs = Импортированные треки не найдены
library-no-playlists = Пока нет плейлистов
library-no-songs = Пока нет избранного
library-not-loaded = Библиотека не загрузилась
library-part-not-loaded = Этот раздел библиотеки не загрузился
library-scanning = Просматриваю ваши папки…
library-scanning-progress = Читаю вашу музыку… { $read } из { $found }
search-albums-playlists = Альбомы и плейлисты
search-best-match = Лучшее совпадение
search-browse = Всё подряд
search-placeholder = Что хотите послушать?
search-recent = Недавние запросы
search-results = Результаты
search-saved =
    { $count ->
        [one] { $count } трек в медиатеке
        [few] { $count } трека в медиатеке
       *[other] { $count } треков в медиатеке
    }
search-tag = { $kind } ·
user-eyebrow = Профиль
user-followers = { $count ->
    [one] { $value } подписчик
    [few] { $value } подписчика
   *[other] { $value } подписчиков
}
user-following = { $count ->
    [one] { $value } подписка
    [few] { $value } подписки
   *[other] { $value } подписок
}
user-playlists = Публичные плейлисты
user-playlists-empty = Пока нет публичных плейлистов
