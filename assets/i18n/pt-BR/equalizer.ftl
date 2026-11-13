# Sound and equalizer
equalizer-add-filter = Adicionar filtro
equalizer-advanced = Avançado
equalizer-bass = Graves
equalizer-copied = Perfil copiado como texto do Equalizer APO
equalizer-decibels = dB
equalizer-delete = Excluir perfil
equalizer-export = Exportar
equalizer-export-file = Exportar arquivo…
equalizer-exported = Perfil exportado
equalizer-file-unreadable = Não foi possível ler o arquivo
equalizer-file-unwritable = Não foi possível gravar o arquivo
equalizer-filter-limit = Um perfil comporta até { $max } filtros.
equalizer-hertz = Hz
equalizer-import = Importar
equalizer-import-file = Importar arquivo…
equalizer-import-nothing = Nenhuma configuração de equalizador encontrada para importar
equalizer-import-too-large = Não importado: o texto passa de 1 MB, grande demais para um perfil de equalizador
equalizer-import-too-many = Não importado: { $count ->
        [one] um filtro está ativo
       *[other] { $count } filtros estão ativos
    }, mais do que um perfil comporta
equalizer-imported-skipped = { $name } importado sem { $count ->
        [one] uma linha
       *[other] { $count } linhas
    }
equalizer-invalid-frequency = A frequência deve estar entre { $low } e { $high } Hz
equalizer-invalid-gain = O ganho deve estar entre { $low } e { $high } dB
equalizer-invalid-octaves = A largura de banda deve estar entre { $low } e { $high } oitavas
equalizer-invalid-preamp = O pré-amp deve estar entre { $low } e { $high } dB
equalizer-invalid-q = O Q deve estar entre { $low } e { $high }
equalizer-invalid-shelf-octaves = Um shelf usa Q, não largura de banda
equalizer-kind-all-pass = Passa-tudo
equalizer-kind-band-pass = Passa-faixa
equalizer-kind-high-pass = Passa-altas
equalizer-kind-high-shelf = Shelf de agudos
equalizer-kind-low-pass = Passa-baixas
equalizer-kind-low-shelf = Shelf de graves
equalizer-kind-notch = Notch
equalizer-kind-peak = Pico
equalizer-level = Nível
equalizer-lower-preamp = Reduzir pré-amp
equalizer-no-filters = Nenhum filtro ainda. Adicione um, escolha uma predefinição acima ou importe um perfil.
equalizer-no-gain = —
equalizer-octaves = oit
equalizer-output-port = { $device } · { $port }
equalizer-overload = Clipping de { $db } dB
equalizer-overload-slight = Clipping abaixo de 0,1 dB
equalizer-paste-empty = A área de transferência não tem texto
equalizer-preamp = Pré-amp
equalizer-preamp-auto = Automático
equalizer-preamp-manual = Manual
equalizer-profile-default = Padrão
equalizer-profile-name-hint = Nome do perfil
equalizer-profile-new = Perfil { $number }
equalizer-profile-pasted = Perfil colado
equalizer-q = Q
equalizer-redo = Refazer
equalizer-remove-filter = Remover filtro
equalizer-rename = Renomear perfil
equalizer-rule-add = Adicionar regra
equalizer-rule-choose-device = Escolha um dispositivo
equalizer-rule-choose-sound = Escolha um som
equalizer-rule-disconnected = Não conectado
equalizer-rule-other = Qualquer outro dispositivo
equalizer-rule-playing = Tocando agora
equalizer-rule-remove = Remover regra
equalizer-rule-replaces = Substitui a regra existente
equalizer-rules = Regras por dispositivo
equalizer-save-as = Salvar como perfil
equalizer-simple = Simples
equalizer-skip-expression = um valor escrito como expressão
equalizer-skip-no-frequency = sem frequência
equalizer-skip-no-gain = sem ganho
equalizer-skip-no-preamp = o pré-amp não é um número
equalizer-skip-no-width = sem Q ou largura de banda
equalizer-skip-unknown-type = tipo de filtro desconhecido { $name }
equalizer-skip-unreadable = não é um filtro que o Sonora consegue ler
equalizer-skip-unsupported = { $name } não é suportado
equalizer-skipped-line = Linha { $line }: { $reason }
equalizer-skipped-more = e { $count ->
        [one] mais uma
       *[other] mais { $count }
    }
equalizer-status-altered = Não é bit-perfect por causa de:
equalizer-status-bit-perfect = Bit-perfect
equalizer-status-bit-perfect-detail = Bit-perfect a partir do Sonora
equalizer-status-bits = { $from } → { $to } bits
equalizer-status-channels = { $from ->
    [1] Mono
    [2] Estéreo
   *[other] { $from } canais
} → { $to ->
    [1] mono
    [2] estéreo
   *[other] { $to } canais
}
equalizer-status-equalized = EQ
equalizer-status-float = Float → { $to } bits
equalizer-status-normalized = Normalizado
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-separator = {", "}
equalizer-status-server = Depois, o servidor de som converte para { $khz } kHz
equalizer-status-volume = Volume
equalizer-tone-reset = Redefinir para 0 dB
equalizer-transfer = Importar e exportar
equalizer-treble = Agudos
equalizer-value-refused = O valor anterior foi mantido
response-graph-db = { $value } dB
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-readout = { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
