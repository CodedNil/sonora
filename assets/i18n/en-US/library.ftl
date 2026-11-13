# Tables, filters and counts
column-added-by = Added by
column-album = Album
column-artist = Artist
column-date-added = Date added
column-genre = Genre
column-index = #
column-length = Length
column-modified = Modified
column-name = Name
column-owner = Owner
column-played-at = Played
column-plays = Plays
column-popularity = Popularity
column-tags = Tags
column-title = Title
column-tracks = Tracks
column-year = Year
count-songs =
    { $count ->
        [one] { $count } song
       *[other] { $count } songs
    }
count-tracks =
    { $count ->
        [one] { $count } track
       *[other] { $count } tracks
    }
filter-album = Filter album tracks
filter-bpm = BPM
filter-collaborations = Collaborations
filter-downloaded = Downloaded
filter-duration = Duration
filter-favorites = Favorites only
filter-genre = { column-genre }
filter-hide-explicit = Hide explicit
filter-history = Filter listening history
filter-library = Filter your library
filter-mood = Mood
filter-owned = By you
filter-play-count = Play count
filter-playable = Playable only
filter-popularity = { column-popularity }
filter-reset = Reset filters
filter-tag = Tag
filter-tracks = Filter tracks
filter-year = Year
rail-back = Scroll back
rail-forward = Scroll forward
range-from = From
range-to = To
table-sorted-ascending = Sorted ascending
table-sorted-descending = Sorted descending
tool-filters = Filters
tool-sort = Sort

# Library and discovery
genre-empty = Nothing to show here yet
history-clear = Clear history
history-clear-confirm = Every play is removed from this device. This cannot be undone.
history-clear-title = Clear listening history
history-empty = Tracks you play will appear here.
history-not-loaded = Listening history could not be loaded.
home-collection-albums = Albums from your collection
home-favorite-albums = Favorite albums
home-quick-picks = Quick picks
home-quick-picks-empty = Like a few songs and they will show up here
home-recently-added = Recently added
home-recently-played = Recently played
library-liked-songs = Favorites
library-local-unconfigured = Configure your local library
library-no-albums = No saved albums yet
library-no-artists = No favorite artists yet
library-no-catalog-albums = No albums found
library-no-catalog-artists = No artists found
library-no-catalog-songs = No songs found
library-no-local-albums = No imported albums found
library-no-local-artists = No imported artists found
library-no-local-playlists = No local playlists yet
library-no-local-songs = No imported songs found
library-no-playlists = No playlists yet
library-no-songs = No favorites yet
library-not-loaded = Your library did not load
library-part-not-loaded = This part of your library did not load
library-play-liked-songs = Play favorites
library-scanning = Looking through your folders…
library-scanning-progress = Reading your music… { $read } of { $found }
search-albums-playlists = Albums & playlists
search-best-match = Best match
search-browse = Browse all
search-open-link = Open link in Sonora
search-placeholder = What do you want to listen to?
search-recent = Recent searches
search-results = Results
search-saved =
    { $count ->
        [one] { $count } song in Library
       *[other] { $count } songs in Library
    }
search-tag = { $kind } ·
user-eyebrow = Profile
user-followers = { $count ->
    [one] { $value } follower
   *[other] { $value } followers
}
user-following = { $count ->
   *[other] { $value } following
}
user-playlists = Public playlists
user-playlists-empty = No public playlists yet
