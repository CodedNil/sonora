# Notifications and status
get-sonora = Dapatkan Sonora
notice-never = Jangan tampilkan lagi
offline-go-online = Kembali online
offline-title = Anda sedang offline.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio berjalan, tetapi ALSA tidak memiliki jalur ke sana, jadi tidak ada suara yang terdengar. Pasang pulseaudio-alsa dan mulai ulang Sonora.
   *[pipewire] PipeWire berjalan, tetapi ALSA tidak memiliki jalur ke sana, jadi tidak ada suara yang terdengar. Pasang pipewire-alsa dan mulai ulang Sonora.
}
requirement-audio-bridge-title = Sonora tidak dapat menjangkau server suara Anda
requirement-audio-output = Sonora tidak dapat membuka perangkat audio Anda, jadi tidak ada suara yang terdengar. Pastikan perangkat terhubung dan tidak sedang dipakai aplikasi lain.
requirement-audio-output-title = Tidak ada keluaran suara
requirement-help = Apa yang perlu dipasang
requirement-web-engine = Pasang webkit2gtk-4.1 untuk masuk ke YouTube Music, Deezer, atau Apple Music di jendela peramban. Sementara itu, tempel cookie Anda secara manual.
requirement-web-engine-title = Masuk lewat peramban tidak tersedia
toast-code-copied = Kode disalin
toast-download-cancelled = Unduhan { $name } dibatalkan
toast-download-failed = Daftar lagu untuk diunduh tidak dapat dimuat.
toast-downloads-cancelled = { $count ->
    [one] Unduhan { $count } lagu dibatalkan
   *[other] Unduhan { $count } lagu dibatalkan
}
toast-favorite-albums-removed = { $count ->
        [one] Satu album
       *[other] { $count } album
    } dihapus dari Favorit
toast-favorite-artists-removed = { $count ->
        [one] Satu artis
       *[other] { $count } artis
    } dihapus dari Favorit
toast-favorite-removed = { $name } dihapus dari Favorit
toast-favorite-songs-removed = { $count ->
        [one] Satu lagu
       *[other] { $count } lagu
    } dihapus dari Favorit
toast-folder-missing = { $name } sudah tidak ada di disk
toast-folder-removed = { $name } dihapus
toast-history-clear-failed = Riwayat tidak dapat dihapus
toast-history-remove-failed = Lagu tidak dapat dihapus dari riwayat
toast-history-removed = { $count ->
    [one] Dihapus dari riwayat
   *[other] { $count } lagu dihapus dari riwayat
}
toast-history-restore-failed = Riwayat tidak dapat dipulihkan
toast-jam-ended = Jam berakhir
toast-jam-failed = Jam tidak dapat dimulai atau diikuti.
toast-jam-leave-failed = Jam tidak dapat diakhiri atau ditinggalkan.
toast-jam-left = Anda keluar dari Jam
toast-jam-needs-connect = Aktifkan “Tampilkan sebagai perangkat Spotify” di pengaturan untuk bergabung ke Jam
toast-jam-unreadable = Tautan itu tidak mengarah ke Jam yang bisa Anda ikuti.
toast-keys-error = Kunci putar gagal
toast-keys-refused = Spotify mungkin gagal
toast-last-album = Album akan diputar terakhir
toast-last-artist = Artis akan diputar terakhir
toast-last-playlist = Playlist akan diputar terakhir
toast-last-track = { $name } akan diputar terakhir
toast-last-tracks = { $count } lagu akan diputar terakhir
toast-library-add-failed = { $name } tidak dapat ditambahkan ke koleksi Anda
toast-library-added = Ditambahkan ke koleksi Anda
toast-library-albums-removed = { $count ->
        [one] Satu album
       *[other] { $count } album
    } dihapus dari koleksi Anda
toast-library-pin-failed = Gagal memperbarui sematan di Spotify.
toast-library-remove-failed = { $name } tidak dapat dihapus dari koleksi Anda
toast-library-removed-one = { $name } dihapus dari koleksi Anda
toast-library-scanned = { $count ->
    [one] Koleksi dipindai, satu lagu ditemukan
   *[other] Koleksi dipindai, { $count } lagu ditemukan
}
toast-library-songs-removed = { $count ->
        [one] Satu lagu
       *[other] { $count } lagu
    } dihapus dari koleksi Anda
toast-link-copied = Tautan disalin
toast-link-sign-in = Masuk ke { $name } untuk membuka tautan ini
toast-link-unreadable = Tautan itu tidak mengarah ke apa pun yang bisa dibuka Sonora
toast-local-delete-failed = Beberapa file lagu tidak dapat dihapus
toast-next-album = Album akan diputar berikutnya
toast-next-artist = Artis akan diputar berikutnya
toast-next-playlist = Playlist akan diputar berikutnya
toast-next-track = { $name } akan diputar berikutnya
toast-next-tracks = { $count } lagu akan diputar berikutnya
toast-offline = Tidak ada koneksi. Tidak ada yang bisa di-stream sampai koneksi kembali.
toast-offline-mode = Anda sedang offline. Kembali online untuk memutar lagu ini.
toast-offline-unsupported = { $name } hanya memutar unduhan saat online.
toast-open-all-failed = { $count } file tidak dapat dibuka
toast-open-failed = { $name } tidak dapat dibuka
toast-open-some-failed = Beberapa file tidak dapat dibuka
toast-open-unsupported = { $name } bukan file audio yang dapat diputar Sonora
toast-output-default-failed = Tidak dapat memilih perangkat bawaan sistem
toast-output-select-failed = Tidak dapat memilih { $name }
toast-playback-stopped = Pemutaran dihentikan karena beberapa lagu berturut-turut tidak dapat diputar
toast-playlist-added = Playlist ditambahkan ke koleksi Anda
toast-playlist-busy = Perubahan lain masih diproses
toast-playlist-cover-changed = Sampul playlist diganti
toast-playlist-cover-removed = Sampul playlist dihapus
toast-playlist-created = Playlist berhasil dibuat
toast-playlist-deleted = Playlist berhasil dihapus
toast-playlist-exported = Playlist berhasil diekspor
toast-playlist-failed = Perubahan tidak dapat disimpan
toast-playlist-import-empty = Tidak ada lagu dari { $name } yang ada di koleksi Anda
toast-playlist-imported-partly = { $name } berhasil diimpor tanpa { $count ->
        [one] satu lagu
       *[other] { $count } lagu
    } yang tidak ada di koleksi Anda
toast-playlist-removed = Playlist dihapus dari koleksi Anda
toast-playlist-renamed = Nama playlist berhasil diubah
toast-playlist-signed-out = Masuk akun untuk mengubah playlist
toast-playlist-updated = Playlist diperbarui
toast-playlist-visibility = Visibilitas playlist diubah
toast-playlists-removed = { $count ->
        [one] Satu playlist
       *[other] { $count } playlist
    } dihapus dari koleksi Anda
toast-queue-cleared = Antrean dikosongkan
toast-queue-failed = Item tidak dapat ditambahkan ke antrean
toast-queue-local-elsewhere = File lokal tidak dapat dimasukkan ke antrean di perangkat lain
toast-queue-removed-track = { $name } dihapus dari antrean
toast-queue-removed-tracks = { $count } lagu dihapus dari antrean
toast-queued-album = Album ditambahkan ke antrean
toast-queued-artist = Artis ditambahkan ke antrean
toast-queued-playlist = Playlist ditambahkan ke antrean
toast-queued-track = { $name } ditambahkan ke antrean
toast-queued-tracks = { $count } lagu ditambahkan ke antrean
toast-radio-failed = Radio untuk { $name } tidak dapat dimulai
toast-restore-failed = { $name } gagal masuk kembali, jadi Sonora berjalan tanpanya
toast-sample-burst = Notifikasi nomor { $name }
toast-sample-delete = { $name } dihapus
toast-sample-done = Disimpan ke koleksi Anda
toast-sample-event = Acara dibuat
toast-sample-event-detail = Minggu, 3 Desember pukul 09.00
toast-sample-failed = Server tidak dapat dijangkau
toast-sample-info = Datang 10 menit sebelum acara dimulai.
toast-sample-lasting = Notifikasi ini tetap tampil sampai Anda menutupnya
toast-sample-loading = Mengunggah lagu Anda
toast-sample-long = Notifikasi ini berisi jauh lebih banyak teks daripada yang lain, jadi teksnya harus turun ke baris kedua dan mungkin ketiga tanpa mendorong tombol tutup keluar dari kartu
toast-sample-plain = Acara telah dibuat
toast-sample-save-failed = { $name } tidak dapat disimpan
toast-sample-save-failed-detail = Server menjawab 503 Service Unavailable
toast-sample-saved = { $name } disimpan
toast-sample-saving = Menyimpan { $name }
toast-sample-undone = Diurungkan
toast-sample-warning = Sesi Anda berakhir dalam lima menit
toast-scan-failed = Folder musik Anda tidak dapat dipindai
toast-settings-broken = Perbaiki baris { $name } di settings.json untuk menyimpan perubahan
toast-settings-overwritten = settings.json berubah di disk, jadi perubahan terakhir Anda tidak disimpan
toast-settings-save-failed = Pengaturan tidak dapat disimpan
toast-sign-in-to-play = { $name } hanya dapat diputar untuk pendengar yang sudah masuk akun
toast-similar-failed = Tidak ditemukan lagu yang mirip dengan { $name }
toast-sleep-ended = Pengatur waktu tidur selesai
toast-still-throttled = { $name } masih membatasi pemutaran. Tekan putar untuk mencoba lagi
toast-tags-failed = Tag tidak dapat disimpan
toast-tags-saved = Tag untuk { $name } berhasil disimpan
toast-throttled = { $name } sedang membatasi pemutaran. Mencoba lagi sebentar lagi
toast-track-added = Ditambahkan ke { $name }
toast-track-removed = Dihapus dari { $name }
toast-track-removed-unnamed = Dihapus dari playlist
toast-track-unplayable = { $name } tidak dapat diputar
toast-tracks-removed = { $count ->
        [one] Satu lagu
       *[other] { $count } lagu
    } dihapus dari { $name }
toast-tracks-removed-unnamed = { $count ->
        [one] Satu lagu
       *[other] { $count } lagu
    } dihapus dari playlist
toast-tray-unavailable = Tidak ada baki sistem untuk menampilkan ikon
toast-unpinned = Sematan { $name } dilepas
toast-why = Kenapa?
toast-widevine-borrowed = Coba perbarui Widevine
toast-widevine-download-failed = Widevine tidak dapat diunduh
toast-widevine-failed = Pemutaran gagal
toast-widevine-install-failed = Widevine tidak dapat dipasang
toast-widevine-remove-failed = Widevine tidak dapat dihapus
trouble-not-loaded = Gagal memuat
trouble-offline = Tidak ada koneksi
trouble-offline-detail = Periksa koneksi internet Anda dan coba lagi.
trouble-retry = Coba lagi
update-available = Sonora { $version } telah dirilis
update-detail = Anda menggunakan versi { $running }. Baca perubahannya, atau perbarui sekarang.
update-detail-notes = Anda menggunakan versi { $running }. Baca perubahannya, lalu perbarui Sonora sesuai cara Anda memasangnya.
update-failed = Pembaruan tidak dapat dipasang. Coba lagi dari halaman rilis GitHub.
update-never = Jangan periksa pembaruan
update-notes = Yang baru
update-now = Perbarui
update-working = Mengunduh pembaruan…
wake-reason = Musik sedang diputar
