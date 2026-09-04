# ma-runtime – Português
lang-name = Português

own-did-published = Documento DID próprio publicado no IPNS
own-did-publish-failed = Falha ao publicar o documento DID próprio
own-did-publish-timeout = Publicação do documento DID próprio expirou após 2 minutos
started = ma runtime iniciado
shutdown-requested = Encerramento solicitado
closing-endpoint = Fechando ponto de conexão iroh...
shutdown-complete = Encerramento concluído
status-listening = Servidor de status a escutar
ipfs-message-rejected = Mensagem IPFS rejeitada
ctrlc-handler-failed = Falha no manipulador Ctrl-C
node-connected = Nó conectado ao protocolo
received-encrypted-ma-msg = Mensagem ma cifrada recebida em /ma/ipfs/0.0.1
ping-received = :ping recebido, enviando :pong
did-publish-request-received = Pedido de publicação de documento DID recebido
document-published = Documento publicado
did-publish-cid-reply-sent = Resposta CID enviada para publicação DID
did-publish-resolve-failed = Não foi possível resolver o remetente para entregar a resposta ipfs-publish
ipfs-store-request-received = Pedido de armazenamento IPFS recebido
ipfs-stored = Conteúdo armazenado no IPFS
ipfs-store-cid-reply-sent = Resposta CID enviada
ipfs-store-resolve-failed = Não foi possível resolver o remetente para entregar a resposta ipfs-store

# Despacho de entidades
bootstrap-complete = Bootstrap concluído
entity-loaded = Plugin de entidade carregado
entity-load-failed = Falha ao carregar o plugin de entidade
root-list-entities = #root: listar entidades
entity-created = Entidade criada
entity-reloaded = Plugin de entidade recarregado
entity-deleted = Entidade eliminada
entity-states-saving = Guardando estados de entidades no IPFS
entity-state-saving = Guardando estado de entidade
entity-state-saved = Estado de entidade guardado
entity-state-empty = Plugin retornou estado vazio, ignorando persistência
entity-states-saved = Estados de entidades guardados

# Primeiro arranque / auto-init

# Propriedade
runtime-claimed = Runtime registado.

# Elementos raiz protegidos
refuse-delete-root = Recuso-me firmemente a eliminar um elemento raiz necessário
runtime-claim-persisted = Proprietário escrito na configuração.


# Namespace creation (:create)
crud-message-received = Mensagem CRUD recebida
crud-acl-updated = ACL de transporte raiz atualizada

# CRUD validation errors
cidv1-required = o valor deve ser um CIDv1 puro (começa com 'b'; CIDv0 'Qm…' não aceito)
config-key-protected = a chave de config '%key%' está protegida
config-key-no-delete = a chave de config '%key%' do daemon não pode ser eliminada
config-key-not-manifest = a chave de config '%key%' não é uma chave de manifest config conhecida
wrong-crud-protocol = protocolo CRUD incorreto: %type%
entity-name-invalid = o nome da entity deve ser UTF-8 imprimível
reserved-entity-name = o nome da entity '%name%' está reservado
genesis-kind-owner-only = Apenas um proprietário do runtime pode criar um entity do tipo genesis

# IPv6 config
ipv6-enabled = IPv6 ativado — vinculando IPv4 e IPv6 simultaneamente
ipv6-disabled = IPv6 desativado — vinculando apenas IPv4 (reinício necessário para reativar)
ipv6-enable-restart-required = Guardado. É necessário reiniciar para que esta alteração entre em vigor.
ipv6-enable-unchanged = ipv6_enable já está definido com esse valor — sem alterações.

boot-default-root-config-populate-failed = Falha ao preencher a raiz de configuração padrão
boot-default-root-config-populated = Raiz de configuração padrão preenchida
boot-entity-load-processed = Plugins de entidade carregados
boot-group-load-failed = Falha ao carregar grupo na inicialização
boot-group-loaded-into-cache = Grupo carregado no cache
boot-kinds-overlay-applied = Sobreposição de kinds aplicada
boot-kinds-overlay-no-change = A sobreposição de kinds não fez alterações no manifesto
boot-load-manifest-for-acl-cache-failed = Falha ao carregar manifesto para preenchimento do cache ACL
boot-minimal-manifest-bootstrapped = Manifesto mínimo inicializado
boot-minimal-manifest-not-found = Nenhum CID raiz de runtime encontrado em IPNS; inicializando manifesto mínimo
boot-no-root-entity = Nenhuma entidade raiz registrada para a raiz de configuração padrão
boot-reconciled-owners-manifest-failed = Falha ao reconciliar proprietários no manifesto na inicialização
boot-reconciled-owners-persist-failed = Falha ao persistir proprietários reconciliados no config.yaml
boot-reconciled-owners-published = Proprietários reconciliados do config.yaml/--owner no manifesto na inicialização
boot-root-acl-load-cache-failed = Falha ao carregar ACL raiz na inicialização
boot-root-acl-load-failed = Falha ao carregar ACL raiz do manifesto
boot-root-acl-loaded-from-manifest = ACL de transporte raiz carregada do manifesto
boot-root-acl-loaded-into-cache = ACL raiz carregada no cache
bootstrap-acl-published = Nó ACL publicado
bootstrap-endpoint-close-stuck = Endpoint ainda retido por tarefas em execução após 10 s; fechamento forçado
bootstrap-endpoint-close-timeout = Fechamento do endpoint expirou após 5 s; saída forçada
bootstrap-entity-lifecycle-update-failed = Falha ao escrever ciclo de vida de entidade atualizado no IPFS
bootstrap-entity-lifecycle-updated = Ciclo de vida de entidade atualizado no IPFS
bootstrap-entity-node-shutdown-updated = Nó de entidade atualizado no encerramento
bootstrap-entity-published = Nó de entidade publicado
bootstrap-entity-registering-prepublished = Registrando entidade pré-publicada
bootstrap-entity-registry-fetch-failed = Falha ao obter nó de entidade
bootstrap-entity-registry-kind-extends-failed = Falha ao resolver cadeia de extensão de kind
bootstrap-entity-registry-kind-fetch-failed = Falha ao obter nó de kind
bootstrap-entity-registry-kind-missing = Kind não encontrado no manifesto; entidade ignorada
bootstrap-entity-registry-not-in-manifest = Entidade no registro mas não no manifesto, ignorada
bootstrap-entity-state-save-failed = Falha ao salvar estados de entidade
bootstrap-entity-state-shutdown-aborted = Encerramento abortado; runtime permanece ativo para que o estado possa ser salvo na próxima tentativa
bootstrap-entity-state-update-fetch-failed = Falha ao obter nó de entidade para atualização de estado
bootstrap-envelope-delivery-failed = Falha na entrega do envelope de plugin; envelope descartado
bootstrap-envelope-open-failed = Envelope de plugin: falha ao abrir caixa de saída; envelope descartado
bootstrap-group-published = Nó de grupo publicado
bootstrap-kind-published = Nó de kind publicado
bootstrap-kind-registry-extends-failed = Falha ao resolver cadeia de extensão de kind para o registro
bootstrap-kind-registry-fetch-log-failed = Falha ao obter nó de kind para o registro
bootstrap-kind-registry-hydrated = Registro de kinds hidratado do manifesto
bootstrap-kinds-overlay-pin-update-failed = Falha em pin/atualização após sobreposição de kinds
bootstrap-kinds-overlay-published = Manifesto de runtime publicado após sobreposição de kinds
bootstrap-kinds-tree-published = Árvore de kinds de runtime publicada
bootstrap-lifecycle-manifest-pin-update-failed = Falha em pin/atualização após persistência do ciclo de vida
bootstrap-lifecycle-manifest-publish-failed = Falha ao publicar manifesto após transições do ciclo de vida
bootstrap-lifecycle-manifest-published = Manifesto atualizado publicado após transições do ciclo de vida
bootstrap-manifest-fetch-failed = Falha ao obter manifesto de runtime
bootstrap-minimal-manifest-failed = Falha ao inicializar manifesto mínimo
bootstrap-remote-root-pin-confirmed = Pin raiz remoto confirmado
bootstrap-remote-root-pin-misconfigured = Pinning raiz remoto está mal configurado
bootstrap-root-acl-published = ACL de transporte raiz publicada
bootstrap-root-cid-shutdown-persist-failed = Falha ao persistir root_cid durante o encerramento
bootstrap-root-cid-shutdown-publish-failed = Publicação runtime_ipns durante o encerramento falhou
bootstrap-root-cid-shutdown-publish-succeeded = Publicação runtime_ipns durante o encerramento bem-sucedida
bootstrap-root-cid-shutdown-publish-timeout = Publicação runtime_ipns durante o encerramento expirou
bootstrap-root-pin-replacement-failed = Continuando após falha na substituição do pin raiz remoto
bootstrap-root-pin-update-failed = Falha em pin/atualização após bootstrap
bootstrap-runtime-manifest-published = Manifesto raiz de runtime publicado
crud-message-rejected = Mensagem CRUD rejeitada
entity-reload-current-node-load-failed = Falha ao carregar nó de entidade atual antes do recarregamento; plugin atual mantido
entity-reload-failed = Falha no recarregamento da entidade; desativada até o próximo recarregamento
entity-reload-kind-extends-failed = Falha ao resolver cadeia de extensão de kind durante recarregamento de entidade
entity-reload-kind-fetch-failed = Falha ao obter nó de kind durante recarregamento de entidade
entity-reload-kind-lookup-failed = Falha ao carregar manifesto para busca de kind durante recarregamento de entidade
entity-reload-kind-missing = Kind não encontrado no manifesto; não é possível recarregar entidade
entity-reload-manifest-state-update-failed = Falha ao atualizar manifesto com estado atual antes do recarregamento; plugin atual mantido
entity-reload-skipped = Recarregamento de entidade ignorado porque o portão de recarregamento está fechado
entity-reload-started = Recarregamento de entidade iniciado
entity-reload-state-persist-failed = Falha ao persistir estado atual antes do recarregamento; plugin atual mantido
entity-reload-state-produced-failed = Falha ao persistir estado produzido durante o recarregamento
entity-reloaded-manifest-update-failed = Falha ao atualizar entidade recarregada no manifesto
entity-reloaded-manifest-updated = Entidade recarregada atualizada no manifesto
inbox-message-rejected = Mensagem da caixa de entrada rejeitada
ma-create-entity-already-exists = ma_create_entity: entidade já existe; entidade atual mantida
ma-create-entity-invalid-behaviour = ma_create_entity: referência de behaviour inválida; ignorado
ma-create-entity-kind-missing = ma_create_entity: kind ausente do registro; ignorado
manifest-pin-update-failed = Falha em pin_update do manifesto
plugin-envelope-build-failed = Envelope de plugin: falha ao construir mensagem; ignorado
plugin-envelope-create-requests-ignored = Envelope de plugin: solicitações de criação ignoradas sem contexto de efeito colateral
plugin-envelope-local-dispatch-failed = Envelope de plugin: despacho local falhou
plugin-envelope-local-dispatch-finish = Envelope de plugin: despacho local concluído
plugin-envelope-local-dispatch-start = Envelope de plugin: despacho local iniciado
plugin-envelope-local-gate-closed = Envelope de plugin: portão de despacho local fechado
plugin-envelope-local-recipient-unknown = Envelope de plugin: destinatário local desconhecido; ignorado
plugin-envelope-local-timeout = Envelope de plugin: despacho local expirou
plugin-envelope-recipient-invalid = Envelope de plugin: DID de destinatário inválido; ignorado
plugin-envelope-remote-limit = Envelope de plugin: limite de entrega remota atingido; envelope descartado
plugin-outbox-congested = Caixa de saída de plugin congestionada; envelopes podem ser descartados se o canal encher
plugin-outbox-drain-limit = Orçamento de drenagem da caixa de saída de plugin esgotado; envelopes restantes adiados
schedule-dispatch-firing = Despacho agendado em execução
schedule-entity-not-found = Despacho agendado: entidade não encontrada
schedule-random-chain-stopped = Cadeia de agendamento aleatório parada: substituída por definição mais recente
schedule-random-create-failed = Falha ao criar próximo trabalho aleatório
schedule-random-reschedule-failed = Falha ao reagendar trabalho aleatório
schedule-stale-dispatch-skipped = Despacho agendado ignorado: agendamento obsoleto
scheduled-dispatch-error = Erro no despacho agendado
scheduled-dispatch-manifest-writer-unavailable = Despacho agendado: escritor de manifesto não está pronto; estado de entidade pendente
