# Notifications and status
get-sonora = Baixar o Sonora
notice-never = Não mostrar novamente
offline-go-online = Ficar online
offline-title = Você está offline.
requirement-audio-bridge = { $server ->
    [pulseaudio] O PulseAudio está em execução, mas o ALSA não tem rota até ele, então nada do que você toca é ouvido. Instale o pulseaudio-alsa e reinicie o Sonora.
   *[pipewire] O PipeWire está em execução, mas o ALSA não tem rota até ele, então nada do que você toca é ouvido. Instale o pipewire-alsa e reinicie o Sonora.
}
requirement-audio-bridge-title = O Sonora não consegue acessar seu servidor de som
requirement-audio-output = O Sonora não conseguiu abrir seu dispositivo de áudio, então nada do que você toca é ouvido. Verifique se há um dispositivo conectado e se outro app não o está usando.
requirement-audio-output-title = Sem saída de som
requirement-help = O que instalar
requirement-web-engine = Instale o webkit2gtk-4.1 para entrar no YouTube Music, Deezer ou Apple Music em uma janela do navegador. Até lá, cole seus cookies.
requirement-web-engine-title = Login pelo navegador indisponível
toast-code-copied = Código copiado
toast-download-cancelled = Download de { $name } cancelado
toast-download-failed = Não foi possível listar as faixas para baixar.
toast-downloads-cancelled = { $count ->
    [one] Download de { $count } faixa cancelado
   *[other] Download de { $count } faixas cancelado
}
toast-favorite-albums-removed = { $count ->
        [one] Um álbum removido
       *[other] { $count } álbuns removidos
    } das Favoritas
toast-favorite-artists-removed = { $count ->
        [one] Um artista removido
       *[other] { $count } artistas removidos
    } das Favoritas
toast-favorite-removed = { $name } removido das Favoritas
toast-favorite-songs-removed = { $count ->
        [one] Uma música removida
       *[other] { $count } músicas removidas
    } das Favoritas
toast-folder-missing = { $name } não está mais no disco
toast-folder-removed = Pasta { $name } removida
toast-history-clear-failed = Não foi possível limpar o histórico
toast-history-remove-failed = Não foi possível remover uma música do histórico
toast-history-removed = { $count ->
    [one] Removida do histórico
   *[other] { $count } músicas removidas do histórico
}
toast-history-restore-failed = Não foi possível restaurar o histórico
toast-jam-ended = Jam encerrada
toast-jam-failed = Não foi possível iniciar ou entrar na Jam.
toast-jam-leave-failed = Não foi possível encerrar ou sair da Jam.
toast-jam-left = Você saiu da Jam
toast-jam-needs-connect = Ative “Mostrar como dispositivo Spotify” nas configurações para entrar em uma Jam
toast-jam-unreadable = Esse link não leva a uma Jam em que você possa entrar.
toast-keys-error = Erro na chave
toast-keys-refused = Spotify pode falhar
toast-last-album = Álbum tocará por último
toast-last-artist = Artista tocará por último
toast-last-playlist = Playlist tocará por último
toast-last-track = { $name } tocará por último
toast-last-tracks = { $count } faixas tocarão por último
toast-library-add-failed = { $name } não pôde ser adicionada à sua biblioteca
toast-library-added = Adicionado à sua biblioteca
toast-library-albums-removed = { $count ->
        [one] Um álbum removido
       *[other] { $count } álbuns removidos
    } da sua biblioteca
toast-library-pin-failed = Não foi possível atualizar o item fixado no Spotify.
toast-library-remove-failed = { $name } não pôde ser removida da sua biblioteca
toast-library-removed-one = { $name } removido da sua biblioteca
toast-library-scanned = { $count ->
    [one] Varredura concluída, uma música encontrada
   *[other] Varredura concluída, { $count } músicas encontradas
}
toast-library-songs-removed = { $count ->
        [one] Uma música removida
       *[other] { $count } músicas removidas
    } da sua biblioteca
toast-link-copied = Link copiado
toast-link-sign-in = Entre no { $name } para abrir este link
toast-link-unreadable = Esse link não leva a nada que o Sonora consiga abrir
toast-local-delete-failed = Alguns arquivos de faixa não puderam ser excluídos
toast-next-album = Álbum tocará a seguir
toast-next-artist = Artista tocará a seguir
toast-next-playlist = Playlist tocará a seguir
toast-next-track = { $name } tocará a seguir
toast-next-tracks = { $count } faixas tocarão a seguir
toast-offline = Sem conexão. Nada será transmitido até ela voltar.
toast-offline-mode = Você está offline. Fique online para ouvir esta música.
toast-offline-unsupported = { $name } só toca downloads quando está online.
toast-open-all-failed = { $count } arquivos não puderam ser abertos
toast-open-failed = Não foi possível abrir { $name }
toast-open-some-failed = Alguns arquivos não puderam ser abertos
toast-open-unsupported = { $name } não é um arquivo de áudio que o Sonora consegue tocar
toast-output-default-failed = Não foi possível selecionar o dispositivo padrão do sistema
toast-output-select-failed = Não foi possível selecionar { $name }
toast-playback-stopped = A reprodução parou porque várias faixas seguidas não puderam ser tocadas
toast-playlist-added = Playlist adicionada à sua biblioteca
toast-playlist-busy = Outra alteração ainda está em andamento
toast-playlist-cover-changed = Capa da playlist alterada
toast-playlist-cover-removed = Capa da playlist removida
toast-playlist-created = Playlist criada
toast-playlist-deleted = Playlist excluída
toast-playlist-exported = Playlist exportada
toast-playlist-failed = Não foi possível salvar a alteração
toast-playlist-import-empty = Nenhuma música de { $name } está na sua biblioteca
toast-playlist-imported-partly = Playlist { $name } importada sem { $count ->
        [one] uma música que não está
       *[other] { $count } músicas que não estão
    } na sua biblioteca
toast-playlist-removed = Playlist removida da sua biblioteca
toast-playlist-renamed = Playlist renomeada
toast-playlist-signed-out = Faça login para alterar playlists
toast-playlist-updated = Playlist atualizada
toast-playlist-visibility = Visibilidade da playlist alterada
toast-playlists-removed = { $count ->
        [one] Uma playlist removida
       *[other] { $count } playlists removidas
    } da sua biblioteca
toast-queue-cleared = Fila limpa
toast-queue-failed = Não foi possível adicionar à fila
toast-queue-local-elsewhere = Arquivos locais não podem ser enfileirados em outro dispositivo
toast-queue-removed-track = { $name } removida da fila
toast-queue-removed-tracks = { $count } faixas removidas da fila
toast-queued-album = Álbum adicionado à fila
toast-queued-artist = Artista adicionado à fila
toast-queued-playlist = Playlist adicionada à fila
toast-queued-track = { $name } adicionada à fila
toast-queued-tracks = { $count } faixas adicionadas à fila
toast-radio-failed = Não foi possível iniciar a rádio de { $name }
toast-restore-failed = { $name } não conseguiu reconectar, então o Sonora seguiu sem ele
toast-sample-burst = Aviso número { $name }
toast-sample-delete = { $name } excluído
toast-sample-done = Salvo na sua biblioteca
toast-sample-event = Evento criado
toast-sample-event-detail = Domingo, 3 de dezembro às 9:00
toast-sample-failed = Não foi possível acessar o servidor
toast-sample-info = Chegue 10 minutos antes do evento.
toast-sample-lasting = Este fica até você fechá-lo
toast-sample-loading = Enviando suas faixas
toast-sample-long = Este aviso diz bem mais que os outros, então o texto precisa quebrar para uma segunda e talvez uma terceira linha sem empurrar o botão de fechar para fora do cartão
toast-sample-plain = O evento foi criado
toast-sample-save-failed = Não foi possível salvar { $name }
toast-sample-save-failed-detail = O servidor respondeu 503 Service Unavailable
toast-sample-saved = { $name } salvo
toast-sample-saving = Salvando { $name }
toast-sample-undone = Desfeito
toast-sample-warning = Sua sessão termina em cinco minutos
toast-scan-failed = Não foi possível varrer suas pastas de música
toast-settings-broken = Corrija a linha { $name } do settings.json para salvar as alterações
toast-settings-overwritten = O settings.json mudou no disco, então sua última alteração não foi salva
toast-settings-save-failed = Não foi possível salvar as configurações
toast-sign-in-to-play = { $name } só reproduz para um ouvinte conectado
toast-similar-failed = Não foi possível encontrar faixas semelhantes a { $name }
toast-sleep-ended = Timer de desligamento encerrado
toast-still-throttled = { $name } ainda está limitando a reprodução. Aperte play para tentar de novo
toast-tags-failed = Não foi possível salvar as tags
toast-tags-saved = Tags salvas para { $name }
toast-throttled = { $name } está limitando a reprodução por enquanto. Tentando de novo em instantes
toast-track-added = Adicionada a { $name }
toast-track-removed = Removida de { $name }
toast-track-removed-unnamed = Removida da playlist
toast-track-unplayable = { $name } não pôde ser reproduzida
toast-tracks-removed = { $count ->
        [one] Uma música removida
       *[other] { $count } músicas removidas
    } de { $name }
toast-tracks-removed-unnamed = { $count ->
        [one] Uma música removida
       *[other] { $count } músicas removidas
    } da playlist
toast-tray-unavailable = Não há área de notificação para colocar o ícone
toast-unpinned = { $name } desafixado
toast-why = Por quê?
toast-widevine-borrowed = Tente atualizar o Widevine
toast-widevine-download-failed = Não foi possível baixar o Widevine
toast-widevine-failed = Falha na reprodução
toast-widevine-install-failed = Não foi possível instalar o Widevine
toast-widevine-remove-failed = Não foi possível remover o Widevine
trouble-not-loaded = Não foi possível carregar
trouble-offline = Sem conexão
trouble-offline-detail = Verifique sua conexão com a internet e tente novamente.
trouble-retry = Tentar novamente
update-available = Sonora { $version } está disponível
update-detail = Você está na versão { $running }. Veja o que mudou ou atualize agora.
update-detail-notes = Você está na versão { $running }. Veja o que mudou e atualize o Sonora da forma como você o instalou.
update-failed = Não foi possível instalar a atualização. Tente novamente pela página de lançamentos.
update-never = Não verificar atualizações
update-notes = O que há de novo
update-now = Atualizar
update-working = Baixando a atualização…
wake-reason = Música tocando
