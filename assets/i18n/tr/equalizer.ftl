# Sound and equalizer
equalizer-add-filter = Filtre ekle
equalizer-advanced = Gelişmiş
equalizer-bass = Bas
equalizer-copied = Profil Equalizer APO metni olarak kopyalandı
equalizer-decibels = dB
equalizer-delete = Profili sil
equalizer-export = Dışa aktar
equalizer-export-file = Dosyaya aktar…
equalizer-exported = Profil dışa aktarıldı
equalizer-file-unreadable = Dosya okunamadı
equalizer-file-unwritable = Dosya yazılamadı
equalizer-filter-limit = Bir profil en fazla { $max } filtre alır.
equalizer-hertz = Hz
equalizer-import = İçe aktar
equalizer-import-file = Dosya içe aktar…
equalizer-import-nothing = İçe aktarılacak ekolayzer ayarı bulunamadı
equalizer-import-too-large = İçe aktarılmadı: metin 1 MB'tan büyük, bir ekolayzer profili için fazla büyük
equalizer-import-too-many = İçe aktarılmadı: { $count ->
        [one] bir filtre açık
       *[other] { $count } filtre açık
    }, bu bir profilin alabileceğinden fazla
equalizer-imported-skipped = { $name } içe aktarıldı, { $count ->
        [one] bir satır
       *[other] { $count } satır
    } atlandı
equalizer-invalid-frequency = Frekans { $low } ile { $high } Hz arasında olmalı
equalizer-invalid-gain = Kazanç { $low } ile { $high } dB arasında olmalı
equalizer-invalid-octaves = Bant genişliği { $low } ile { $high } oktav arasında olmalı
equalizer-invalid-preamp = Ön kazanç { $low } ile { $high } dB arasında olmalı
equalizer-invalid-q = Q { $low } ile { $high } arasında olmalı
equalizer-invalid-shelf-octaves = Raf filtresi bant genişliği değil, Q alır
equalizer-kind-all-pass = Tüm geçiren
equalizer-kind-band-pass = Bant geçiren
equalizer-kind-high-pass = Yüksek geçiren
equalizer-kind-high-shelf = Yüksek raf
equalizer-kind-low-pass = Alçak geçiren
equalizer-kind-low-shelf = Alçak raf
equalizer-kind-notch = Çentik
equalizer-kind-peak = Tepe
equalizer-level = Düzey
equalizer-lower-preamp = Ön kazancı düşür
equalizer-no-filters = Henüz filtre yok. Bir tane ekle, yukarıdan bir hazır ayar seç ya da bir profil içe aktar.
equalizer-no-gain = —
equalizer-octaves = okt
equalizer-output-port = { $device } · { $port }
equalizer-overload = { $db } dB kırpılma
equalizer-overload-slight = 0,1 dB'den az kırpılma
equalizer-paste-empty = Panoda metin yok
equalizer-preamp = Ön kazanç
equalizer-preamp-auto = Otomatik
equalizer-preamp-manual = Elle
equalizer-profile-default = Varsayılan
equalizer-profile-name-hint = Profil adı
equalizer-profile-new = Profil { $number }
equalizer-profile-pasted = Yapıştırılan profil
equalizer-q = Q
equalizer-redo = Yinele
equalizer-remove-filter = Filtreyi kaldır
equalizer-rename = Profili yeniden adlandır
equalizer-rule-add = Kural ekle
equalizer-rule-choose-device = Bir cihaz seç
equalizer-rule-choose-sound = Bir ses seç
equalizer-rule-disconnected = Bağlı değil
equalizer-rule-other = Diğer tüm cihazlar
equalizer-rule-playing = Şimdi çalıyor
equalizer-rule-remove = Kuralı kaldır
equalizer-rule-replaces = Kendi kuralının yerini alır
equalizer-rules = Cihaz kuralları
equalizer-save-as = Profil olarak kaydet
equalizer-simple = Basit
equalizer-skip-expression = ifade olarak yazılmış bir değer
equalizer-skip-no-frequency = frekans yok
equalizer-skip-no-gain = kazanç yok
equalizer-skip-no-preamp = ön kazanç bir sayı değil
equalizer-skip-no-width = Q ya da bant genişliği yok
equalizer-skip-unknown-type = bilinmeyen filtre türü { $name }
equalizer-skip-unreadable = Sonora'nın okuyabildiği bir filtre değil
equalizer-skip-unsupported = { $name } desteklenmiyor
equalizer-skipped-line = Satır { $line }: { $reason }
equalizer-skipped-more = ve { $count ->
        [one] bir tane daha
       *[other] { $count } tane daha
    }
equalizer-status-altered = Bit-perfect değil, nedeni:
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Sonora'dan bit-perfect
equalizer-status-bits = { $from } → { $to } bit
equalizer-status-channels = { $from ->
    [1] Mono
    [2] Stereo
   *[other] { $from } kanal
} → { $to ->
    [1] mono
    [2] stereo
   *[other] { $to } kanal
}
equalizer-status-equalized = EQ
equalizer-status-float = Float → { $to } bit
equalizer-status-normalized = Eşitlendi
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = Ses sunucusu ardından bunu { $khz } kHz'e dönüştürür
equalizer-status-volume = Ses düzeyi
equalizer-tone-reset = 0 dB'ye sıfırla
equalizer-transfer = İçe ve dışa aktar
equalizer-treble = Tiz
equalizer-value-refused = Önceki değer korundu
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
