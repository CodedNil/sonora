# Tables, filters and counts
column-added-by = Ditambahkan Oleh
column-album = Album
column-artist = Artis
column-date-added = Tanggal Ditambahkan
column-genre = Genre
column-index = #
column-length = Durasi
column-modified = Diubah
column-name = Nama
column-owner = Pemilik
column-played-at = Diputar
column-plays = Pemutaran
column-popularity = Popularitas
column-title = Judul
column-tracks = Trek
column-year = Tahun
count-songs =
    { $count ->
        [one] { $count } lagu
       *[other] { $count } lagu
    }
count-tracks =
    { $count ->
        [one] { $count } trek
       *[other] { $count } trek
    }
filter-album = Filter trek album
filter-downloaded = Diunduh
filter-duration = Durasi
filter-favorites = Hanya favorit
filter-hide-explicit = Sembunyikan eksplisit
filter-history = Filter riwayat pemutaran
filter-library = Filter koleksi Anda
filter-owned = Dibuat oleh Anda
filter-playable = Hanya yang bisa diputar
filter-reset = Reset filter
filter-year = Tahun
tool-filters = Filter
tool-sort = Urutkan

# Library and discovery
genre-empty = Belum ada yang bisa ditampilkan di sini
history-clear = Hapus riwayat
history-clear-confirm = Seluruh riwayat pemutaran akan dihapus dari perangkat ini. Tindakan ini tidak dapat dibatalkan.
history-clear-title = Hapus riwayat pemutaran
history-empty = Lagu yang Anda putar akan muncul di sini.
history-not-loaded = Riwayat pemutaran tidak dapat dimuat.
home-collection-albums = Album dari koleksi Anda
home-favorite-albums = Album favorit
home-quick-picks = Pilihan cepat
home-quick-picks-empty = Sukai beberapa lagu dan rekomendasinya akan muncul di sini
home-recently-added = Baru ditambahkan
home-recently-played = Baru diputar
library-liked-songs = Favorit
library-local-unconfigured = Atur folder musik lokal Anda
library-no-albums = Belum ada album tersimpan
library-no-artists = Belum ada artis favorit
library-no-catalog-albums = Tidak ada album yang ditemukan
library-no-catalog-artists = Tidak ada artis yang ditemukan
library-no-catalog-songs = Tidak ada lagu yang ditemukan
library-no-local-albums = Tidak ada album lokal yang ditemukan
library-no-local-artists = Tidak ada artis lokal yang ditemukan
library-no-local-playlists = Belum ada playlist lokal
library-no-local-songs = Tidak ada lagu lokal yang ditemukan
library-no-playlists = Belum ada playlist
library-no-songs = Belum ada lagu favorit
library-not-loaded = Koleksi Anda gagal dimuat
library-part-not-loaded = Sebagian dari koleksi Anda gagal dimuat
library-scanning = Memeriksa folder Anda…
library-scanning-progress = Membaca musik Anda… { $read } dari { $found }
search-albums-playlists = Album & playlist
search-best-match = Hasil teratas
search-browse = Jelajahi semua
search-placeholder = Apa yang ingin Anda dengarkan?
search-recent = Pencarian terbaru
search-results = Hasil pencarian
search-saved =
    { $count ->
        [one] { $count } lagu di Koleksi
       *[other] { $count } lagu di Koleksi
    }
search-tag = { $kind } ·
user-eyebrow = Profil
user-followers = { $count ->
    [one] { $value } pengikut
   *[other] { $value } pengikut
}
user-following = { $count ->
   *[other] { $value } mengikuti
}
user-playlists = Playlist publik
user-playlists-empty = Belum ada playlist publik
