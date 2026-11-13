# Tables, filters and counts
column-added-by = Ekleyen
column-album = Albüm
column-artist = Sanatçı
column-date-added = Eklenme tarihi
column-genre = Tür
column-index = #
column-length = Süre
column-modified = Değiştirilme
column-name = Ad
column-owner = Sahibi
column-played-at = Çalınma
column-plays = Dinlenme
column-popularity = Popülerlik
column-title = Başlık
column-tracks = Parçalar
column-year = Yıl
count-songs =
    { $count ->
       *[other] { $count } şarkı
    }
count-tracks =
    { $count ->
       *[other] { $count } parça
    }
filter-album = Albüm parçalarını süz
filter-downloaded = İndirilenler
filter-duration = Süre
filter-favorites = Yalnızca favoriler
filter-hide-explicit = Müstehcenleri gizle
filter-history = Dinleme geçmişini süz
filter-library = Kitaplığını süz
filter-owned = Senin
filter-playable = Yalnızca çalınabilir
filter-reset = Süzgeçleri sıfırla
filter-year = Yıl
tool-filters = Süzgeçler
tool-sort = Sırala

# Library and discovery
genre-empty = Burada henüz gösterilecek bir şey yok
history-clear = Geçmişi temizle
history-clear-confirm = Bu cihazdaki her dinleme silinir. Bu işlem geri alınamaz.
history-clear-title = Dinleme geçmişini temizle
history-empty = Çaldığın parçalar burada görünecek.
history-not-loaded = Dinleme geçmişi yüklenemedi.
home-collection-albums = Koleksiyonundaki albümler
home-favorite-albums = Favori albümler
home-quick-picks = Hızlı seçimler
home-quick-picks-empty = Birkaç şarkı beğen, burada görünsünler
home-recently-added = Son eklenenler
home-recently-played = Son çalınanlar
library-liked-songs = Favoriler
library-local-unconfigured = Yerel kitaplığını yapılandır
library-no-albums = Henüz kaydedilmiş albüm yok
library-no-artists = Henüz favori sanatçı yok
library-no-catalog-albums = Albüm bulunamadı
library-no-catalog-artists = Sanatçı bulunamadı
library-no-catalog-songs = Şarkı bulunamadı
library-no-local-albums = İçe aktarılmış albüm bulunamadı
library-no-local-artists = İçe aktarılmış sanatçı bulunamadı
library-no-local-playlists = Henüz yerel çalma listesi yok
library-no-local-songs = İçe aktarılmış şarkı bulunamadı
library-no-playlists = Henüz çalma listesi yok
library-no-songs = Henüz favori yok
library-not-loaded = Kitaplığın yüklenemedi
library-part-not-loaded = Kitaplığının bu bölümü yüklenemedi
library-scanning = Klasörlerine bakılıyor…
library-scanning-progress = Müziğin okunuyor… { $read } / { $found }
search-albums-playlists = Albümler ve çalma listeleri
search-best-match = En iyi eşleşme
search-browse = Tümüne göz at
search-placeholder = Ne dinlemek istersin?
search-recent = Son aramalar
search-results = Sonuçlar
search-saved =
    { $count ->
       *[other] Kitaplıkta { $count } şarkı
    }
search-tag = { $kind } ·
user-eyebrow = Profil
user-followers =
    { $count ->
       *[other] { $value } takipçi
    }
user-following =
    { $count ->
       *[other] { $value } takip ediliyor
    }
user-playlists = Herkese açık çalma listeleri
user-playlists-empty = Henüz herkese açık çalma listesi yok
