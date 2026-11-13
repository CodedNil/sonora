# Notifications and status
get-sonora = Sonora'yı edin
notice-never = Bir daha gösterme
offline-go-online = Çevrimiçi ol
offline-title = Çevrimdışısın.
requirement-audio-bridge = { $server ->
    [pulseaudio] PulseAudio çalışıyor ama ALSA'nın ona giden bir yolu yok, bu yüzden çaldığın hiçbir şey duyulmuyor. pulseaudio-alsa paketini kur ve Sonora'yı yeniden başlat.
   *[pipewire] PipeWire çalışıyor ama ALSA'nın ona giden bir yolu yok, bu yüzden çaldığın hiçbir şey duyulmuyor. pipewire-alsa paketini kur ve Sonora'yı yeniden başlat.
}
requirement-audio-bridge-title = Sonora ses sunucuna ulaşamıyor
requirement-audio-output = Sonora ses cihazını açamadı, bu yüzden çaldığın hiçbir şey duyulmuyor. Bir cihazın bağlı olduğunu ve başka bir uygulamanın onu kullanmadığını denetle.
requirement-audio-output-title = Ses çıkışı yok
requirement-help = Neyi kurmalı
requirement-web-engine = YouTube Music, Deezer ya da Apple Music için tarayıcı penceresinde oturum açmak üzere webkit2gtk-4.1 paketini kur. O zamana kadar çerezlerini yapıştır.
requirement-web-engine-title = Tarayıcıyla oturum açma kullanılamıyor
toast-code-copied = Kod kopyalandı
toast-download-cancelled = { $name } indirmesi iptal edildi
toast-download-failed = İndirilecek parçalar listelenemedi.
toast-downloads-cancelled = { $count ->
   *[other] { $count } parçanın indirmesi iptal edildi
}
toast-favorite-albums-removed = { $count ->
        [one] Bir albüm
       *[other] { $count } albüm
    } favorilerinden çıkarıldı
toast-favorite-artists-removed = { $count ->
        [one] Bir sanatçı
       *[other] { $count } sanatçı
    } favorilerinden çıkarıldı
toast-favorite-removed = { $name } favorilerinden çıkarıldı
toast-favorite-songs-removed = { $count ->
        [one] Bir şarkı
       *[other] { $count } şarkı
    } favorilerinden çıkarıldı
toast-folder-missing = { $name } artık diskte yok
toast-folder-removed = { $name } kaldırıldı
toast-history-clear-failed = Geçmiş temizlenemedi
toast-history-remove-failed = Bir şarkı geçmişten kaldırılamadı
toast-history-removed = { $count ->
    [one] Geçmişten kaldırıldı
   *[other] { $count } şarkı geçmişten kaldırıldı
}
toast-history-restore-failed = Geçmiş geri yüklenemedi
toast-jam-ended = Jam bitti
toast-jam-failed = Jam başlatılamadı ya da Jam'e katılınamadı.
toast-jam-leave-failed = Jam bitirilemedi ya da Jam'den ayrılınamadı.
toast-jam-left = Jam'den ayrıldın
toast-jam-needs-connect = Jam'e katılmak için ayarlarda “Spotify cihazı olarak göster” seçeneğini aç
toast-jam-unreadable = Bu bağlantı katılabileceğin bir Jam'e götürmüyor.
toast-keys-error = Çalma anahtarı hatası
toast-keys-refused = Spotify çalışmayabilir
toast-last-album = Albüm en son çalacak
toast-last-artist = Sanatçı en son çalacak
toast-last-playlist = Çalma listesi en son çalacak
toast-last-track = { $name } en son çalacak
toast-last-tracks = { $count } parça en son çalacak
toast-library-add-failed = { $name } kitaplığına eklenemedi
toast-library-added = Kitaplığına eklendi
toast-library-albums-removed = { $count ->
        [one] Bir albüm
       *[other] { $count } albüm
    } kitaplığından çıkarıldı
toast-library-pin-failed = Spotify'daki sabitleme güncellenemedi.
toast-library-remove-failed = { $name } kitaplığından çıkarılamadı
toast-library-removed-one = { $name } kitaplığından çıkarıldı
toast-library-scanned = { $count ->
    [one] Kitaplık tarandı, bir şarkı bulundu
   *[other] Kitaplık tarandı, { $count } şarkı bulundu
}
toast-library-songs-removed = { $count ->
        [one] Bir şarkı
       *[other] { $count } şarkı
    } kitaplığından çıkarıldı
toast-link-copied = Bağlantı kopyalandı
toast-link-sign-in = Bu bağlantıyı açmak için { $name } oturumu aç
toast-link-unreadable = Bu bağlantı Sonora'nın açabileceği bir yere götürmüyor
toast-local-delete-failed = Bazı parça dosyaları silinemedi
toast-next-album = Sırada albüm çalacak
toast-next-artist = Sırada sanatçı çalacak
toast-next-playlist = Sırada çalma listesi çalacak
toast-next-track = Sırada { $name } çalacak
toast-next-tracks = Sırada { $count } parça çalacak
toast-offline = Bağlantı yok. Bağlantı gelene kadar internetten hiçbir şey çalınmaz.
toast-offline-mode = Çevrimdışısın. Bu şarkıyı internetten çalmak için çevrimiçi ol.
toast-offline-unsupported = { $name }, indirmeleri yalnızca çevrimiçiyken çalar.
toast-open-all-failed = { $count } dosya açılamadı
toast-open-failed = { $name } açılamadı
toast-open-some-failed = Bazı dosyalar açılamadı
toast-open-unsupported = { $name }, Sonora'nın çalabildiği bir ses dosyası değil
toast-output-default-failed = Sistem varsayılanı cihaz seçilemedi
toast-output-select-failed = { $name } seçilemedi
toast-playback-stopped = Arka arkaya birkaç parça çalınamadığı için çalma durdu
toast-playlist-added = Çalma listesi kitaplığına eklendi
toast-playlist-busy = Başka bir değişiklik hâlâ sürüyor
toast-playlist-cover-changed = Çalma listesinin kapağı değişti
toast-playlist-cover-removed = Çalma listesinin kapağı kaldırıldı
toast-playlist-created = Çalma listesi oluşturuldu
toast-playlist-deleted = Çalma listesi silindi
toast-playlist-exported = Çalma listesi dışa aktarıldı
toast-playlist-failed = Bu değişiklik kaydedilemedi
toast-playlist-import-empty = { $name } içindeki şarkıların hiçbiri kitaplığında yok
toast-playlist-imported-partly = { $name } içe aktarıldı, kitaplığında olmayan { $count ->
        [one] bir şarkı
       *[other] { $count } şarkı
    } dışarıda bırakıldı
toast-playlist-removed = Çalma listesi kitaplığından çıkarıldı
toast-playlist-renamed = Çalma listesi yeniden adlandırıldı
toast-playlist-signed-out = Çalma listelerini değiştirmek için oturum aç
toast-playlist-updated = Çalma listesi güncellendi
toast-playlist-visibility = Çalma listesinin görünürlüğü değişti
toast-playlists-removed = { $count ->
        [one] Bir çalma listesi
       *[other] { $count } çalma listesi
    } kitaplığından çıkarıldı
toast-queue-cleared = Sıra temizlendi
toast-queue-failed = Bu, sıraya eklenemedi
toast-queue-local-elsewhere = Yerel dosyalar başka bir cihazda sıraya eklenemez
toast-queue-removed-track = { $name } sıradan çıkarıldı
toast-queue-removed-tracks = { $count } parça sıradan çıkarıldı
toast-queued-album = Albüm sıraya eklendi
toast-queued-artist = Sanatçı sıraya eklendi
toast-queued-playlist = Çalma listesi sıraya eklendi
toast-queued-track = { $name } sıraya eklendi
toast-queued-tracks = { $count } parça sıraya eklendi
toast-radio-failed = { $name } için radyo başlatılamadı
toast-restore-failed = { $name } oturumu yeniden açılamadı, Sonora onsuz devam etti
toast-sample-burst = Bildirim { $name }
toast-sample-delete = { $name } silindi
toast-sample-done = Kitaplığına kaydedildi
toast-sample-event = Etkinlik oluşturuldu
toast-sample-event-detail = 3 Aralık Pazar, 09:00
toast-sample-failed = Sunucuya ulaşılamadı
toast-sample-info = Etkinlikten 10 dakika önce gel.
toast-sample-lasting = Bu, sen kapatana kadar kalır
toast-sample-loading = Parçaların yükleniyor
toast-sample-long = Bu bildirim diğerlerinden çok daha fazlasını söylüyor, bu yüzden metin kapatma düğmesini kartın dışına itmeden ikinci, belki de üçüncü satıra geçmek zorunda
toast-sample-plain = Etkinlik oluşturuldu
toast-sample-save-failed = { $name } kaydedilemedi
toast-sample-save-failed-detail = Sunucu 503 Service Unavailable yanıtını verdi
toast-sample-saved = { $name } kaydedildi
toast-sample-saving = { $name } kaydediliyor
toast-sample-undone = Geri alındı
toast-sample-warning = Oturumun beş dakika içinde sona erecek
toast-scan-failed = Müzik klasörlerin taranamadı
toast-settings-broken = Değişiklikleri kaydetmek için settings.json dosyasındaki { $name }. satırı düzelt
toast-settings-overwritten = settings.json diskte değişti, bu yüzden son değişikliğin kaydedilmedi
toast-settings-save-failed = Ayarlar kaydedilemedi
toast-sign-in-to-play = { $name } yalnızca oturum açmış dinleyicilere yayınlanır
toast-similar-failed = { $name } için benzer parça bulunamadı
toast-sleep-ended = Uyku zamanlayıcısı bitti
toast-still-throttled = { $name } hâlâ çalmayı sınırlıyor. Yeniden denemek için Çal'a bas
toast-tags-failed = Etiketler kaydedilemedi
toast-tags-saved = { $name } için etiketler kaydedildi
toast-throttled = { $name } şimdilik çalmayı sınırlıyor. Birazdan yeniden denenecek
toast-track-added = { $name } listesine eklendi
toast-track-removed = { $name } listesinden çıkarıldı
toast-track-removed-unnamed = Çalma listesinden çıkarıldı
toast-track-unplayable = { $name } çalınamadı
toast-tracks-removed = { $count ->
        [one] Bir şarkı
       *[other] { $count } şarkı
    } { $name } listesinden çıkarıldı
toast-tracks-removed-unnamed = { $count ->
        [one] Bir şarkı
       *[other] { $count } şarkı
    } çalma listesinden çıkarıldı
toast-tray-unavailable = Simgeyi koyacak bir sistem tepsisi yok
toast-unpinned = Sabitleme kaldırıldı: { $name }
toast-why = Neden?
toast-widevine-borrowed = Widevine'ı güncellemeyi dene
toast-widevine-download-failed = Widevine indirilemedi
toast-widevine-failed = Çalma başarısız oldu
toast-widevine-install-failed = Widevine kurulamadı
toast-widevine-remove-failed = Widevine kaldırılamadı
trouble-not-loaded = Yüklenemedi
trouble-offline = Bağlantı yok
trouble-offline-detail = İnternet bağlantını denetleyip yeniden dene.
trouble-retry = Yeniden dene
update-available = Sonora { $version } çıktı
update-detail = { $running } sürümünü kullanıyorsun. Nelerin değiştiğini oku ya da şimdi güncelle.
update-detail-notes = { $running } sürümünü kullanıyorsun. Nelerin değiştiğini oku, sonra Sonora'yı kurduğun yoldan güncelle.
update-failed = Güncelleme kurulamadı. Yayınlar sayfasından yeniden dene.
update-never = Güncellemeleri denetleme
update-notes = Yenilikler
update-now = Güncelle
update-working = Güncelleme indiriliyor…
wake-reason = Müzik çalıyor
