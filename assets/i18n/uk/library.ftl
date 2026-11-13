# Tables, filters and counts
column-added-by = Додав
column-album = Альбом
column-artist = Виконавець
column-date-added = Дата додавання
column-genre = Жанр
column-index = #
column-length = Тривалість
column-modified = Змінено
column-name = Назва
column-owner = Власник
column-played-at = Прослухано
column-plays = Прослуховування
column-popularity = Популярність
column-title = Назва
column-tracks = Треки
column-year = Рік
count-songs =
    { $count ->
        [one] { $count } трек
        [few] { $count } треки
       *[other] { $count } треків
    }
count-tracks =
    { $count ->
        [one] { $count } трек
        [few] { $count } треки
       *[other] { $count } треків
    }
filter-album = Фільтр треків альбому
filter-downloaded = Завантажене
filter-duration = Тривалість
filter-favorites = Лише улюблене
filter-hide-explicit = Приховати з ненормативною лексикою
filter-history = Фільтрувати історію прослуховувань
filter-library = Фільтр медіатеки
filter-owned = Мої
filter-playable = Лише доступні
filter-reset = Скинути фільтри
filter-year = Рік
tool-filters = Фільтри
tool-sort = Сортування

# Library and discovery
genre-empty = Тут поки що порожньо
history-clear = Очистити історію
history-clear-confirm = Усі прослуховування буде вилучено з цього пристрою. Скасувати це неможливо.
history-clear-title = Очистити історію прослуховувань
history-empty = Прослухані треки з’являться тут.
history-not-loaded = Не вдалося завантажити історію прослуховувань.
home-collection-albums = Альбоми з вашої колекції
home-favorite-albums = Улюблені альбоми
home-quick-picks = Швидкий вибір
home-quick-picks-empty = Додайте кілька треків, і вони з'являться тут
home-recently-added = Нещодавно додане
home-recently-played = Нещодавно прослухане
library-liked-songs = Улюблене
library-local-unconfigured = Налаштуйте локальну бібліотеку
library-no-albums = Поки немає збережених альбомів
library-no-artists = Поки немає улюблених виконавців
library-no-catalog-albums = Альбомів не знайдено
library-no-catalog-artists = Виконавців не знайдено
library-no-catalog-songs = Треків не знайдено
library-no-local-albums = Імпортовані альбоми не знайдено
library-no-local-artists = Імпортованих виконавців не знайдено
library-no-local-playlists = Локальних плейлистів ще немає
library-no-local-songs = Імпортовані треки не знайдено
library-no-playlists = Поки немає плейлистів
library-no-songs = Улюбленого ще немає
library-not-loaded = Бібліотека не завантажилася
library-part-not-loaded = Цей розділ бібліотеки не завантажився
library-scanning = Переглядаю ваші теки…
library-scanning-progress = Читаю вашу музику… { $read } з { $found }
search-albums-playlists = Альбоми та плейлисти
search-best-match = Найкращий збіг
search-browse = Огляд
search-placeholder = Що хочете послухати?
search-recent = Нещодавні запити
search-results = Результати
search-saved =
    { $count ->
        [one] { $count } трек у медіатеці
        [few] { $count } треки у медіатеці
       *[other] { $count } треків у медіатеці
    }
search-tag = { $kind } ·
user-eyebrow = Профіль
user-followers = { $count ->
    [one] { $value } підписник
    [few] { $value } підписники
   *[other] { $value } підписників
}
user-following = { $count ->
    [one] { $value } підписка
    [few] { $value } підписки
   *[other] { $value } підписок
}
user-playlists = Публічні плейлісти
user-playlists-empty = Поки немає публічних плейлістів
