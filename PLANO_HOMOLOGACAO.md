# Plano de Tarefas — Preparação para Homologação

**Data:** 2026-08-06
**Base:** AUDIT_REPORT_V4.md
**Critério de saída:** CI verde em `main` + stack dev funcional (`make up`) + deploy de produção cobrindo P2P/TURN + apps validados em device.

---

## Fase A — Restaurar build verde (P0, bloqueante) — 1-2 dias

- [x] **A1.** Corrigir `core/tests/message_integration.rs` (TST-02) para o contrato "nunca plaintext por padrão": estabelecer sessão E2E (prekey exchange via `ensure_remote_prekey`/bundle) antes do `send_text_message`, ou ajustar o teste à política. Evidência: falha `Crypto("No E2E session... plaintext fallback is disabled")` em `message_integration.rs:97`. **Feito** — `test_end_to_end_message_exchange` agora troca o prekey bundle de B via `get_prekey_bundle_json` + `set_contact_prekey_bundle` antes do envio (SEC-01).
- [x] **A2.** Corrigir clippy `-D warnings` no workspace. **Feito** — workspace e servers limpos.
- [x] **A3.** Rodar `cargo fmt` no workspace (3 arquivos divergentes: `core/src/api/client.rs`, `core/src/ffi/client.rs`). **Feito** — `cargo fmt --all --check` limpo.
- [x] **A4.** Adicionar gates de clippy/fmt ao `.github/workflows/ci.yml` (alinhar com AGENTS.md). **Feito** — `ci.yml` roda `cargo fmt --all --check` e `cargo clippy --workspace -- -D warnings`.
- [x] **A5.** Push branch → PR → merge em `main` **passando CI completo** (13 commits órfãos c58c0f4..764ef7b). **Feito** — commits pushados e CI verde desde então.
- [x] **A6.** Verificar se `test_end_to_end_message_exchange` cobre também o path de fallback plaintext com `ZAPLIVRE_ALLOW_PLAINTEXT=1` (teste do downgrade explícito). **Feito** — novo binário isolado `core/tests/plaintext_integration.rs` (`test_plaintext_downgrade_policy`) valida que sem env o envio falha (SEC-01) e com `ZAPLIVRE_ALLOW_PLAINTEXT=true` o downgrade explícito entrega a mensagem plaintext; adicionado ao `ci.yml`.

## Fase B — Stack dev funcional (P1) — 1-2 dias

- [x] **B1.** Corrigir healthcheck do Redis em `docker-compose.yml:36`: usar `redis-cli -a $REDIS_PASSWORD ping` (espelhar `stack.yml:39`) ou `--no-auth-warning`. **Feito** — healthcheck com `CMD-SHELL redis-cli -a "$$REDIS_PASSWORD" --no-auth-warning --raw incr ping` + env `REDIS_PASSWORD` exposta ao container (também corrigido no `stack.yml`).
- [x] **B2.** Corrigir `server/coturn/healthcheck.sh`: testar só porta 3478 (remover 5349/TLS enquanto `no-tls`); usar `nc` compatível com Alpine. **Feito** — a imagem coturn não tem `nc`; healthcheck reescrito com `/dev/tcp` do bash, só porta 3478. Healthcheck do coturn no compose trocado para `CMD-SHELL` (o formato `["/bin/sh", ...]` era inválido).
- [x] **B3.** Criar `server/monitoring/prometheus.yml` e dashboards Grafana, **ou** remover as referências. **Feito** — criado `server/monitoring/` com `prometheus.yml`, `blackbox.yml` (endpoints `/health` retornam JSON, não métricas → usa blackbox-exporter para probe HTTP), `grafana/datasources/` e `grafana/dashboards/`. Adicionado serviço `blackbox-exporter` (profile monitoring). Validado: todos os targets `up` e dashboard "ZapLivre Services" provisionado.
- [x] **B4.** Validar `make up` de ponta a ponta. **Feito** — 9 serviços `healthy` (postgres, redis, coturn, turn-credentials, bootstrap, store, push, identity, signaling). Observação: `TURN_RELAY_PORT_RANGE` parametrizado no compose (default 49152-65535) porque o Colima/macOS não suporta publicar 16k portas UDP.
- [x] **B5.** Definir/decidir estratégia de migração de schema Postgres. **Feito** — `server/postgres/MIGRATIONS.md` documenta adoção de `sqlx::migrate!` no message-store, seed `0001_init.sql` e processo de evolução.

## Fase C — Produção P2P/DevOps (P0/P1) — 3-5 dias

- [x] **C1.** Adicionar serviço `bootstrap` ao `docker-compose.yml` de `zaplivre-devops` (porta P2P + health) e `coturn`/TURN com credenciais do `turn-credentials` (TURN_STATIC_SECRET, TURN_EXTERNAL_IP). **Feito** — `bootstrap-node-1/2` (4001/8000 e 4002/8001, seeds configuráveis) + `coturn` (3478/5349/relay via `TURN_RELAY_PORT_RANGE`); rotas Caddy `dht1/dht2.zaplivre.app`.
- [x] **C2.** Adicionar `zaplivre-bootstrap` à matriz de imagens em `.github/workflows/build-server-images.yml`. **Feito** — `binary: zaplivre-bootstrap` / `image: zaplivre-bootstrap` (binário no workspace).
- [x] **C3.** Trocar bootstrap hardcoded `dht1/dht2.associahub.com.br` em `core/src/ffi/client.rs:1003-1006` por nós `*.zaplivre.app` configuráveis (env/build-config exposto nos apps). **Feito** — domínios `dht1/dht2.zaplivre.app`; Android expõe `BOOTSTRAP_PEERS` (buildConfigField → `ZAPLIVRE_BOOTSTRAP`); override por env continua no core.
- [x] **C4.** Unificar domínios: iOS `Info.plist:25-46` e `PushNotificationManager.swift:26` de `*.associahub.com.br` → `*.zaplivre.app`; alinhar `project.yml` × `Info.plist`. **Feito** — `store/identity/signal.zaplivre.app` no Info.plist, project.yml e PushNotificationManager; `push.zaplivre.app` no Swift.
- [ ] **C5.** Provisionar APNs no devops (`.p8`, APNS_KEY_ID, APNS_TEAM_ID, mount read-only) e FCM real; validar entrega iOS+Android. **Parcial (devops pronto)** — envs APNs + mount `secrets/apns-key.p8` e FCM no compose do devops; falta gerar/provisionar as chaves reais nos secrets e validar entrega em device.
- [x] **C6.** Corrigir `stack.yml` (nomes de imagem com registry GHCR; remover hosts legados). **Feito** — todas as imagens `zaplivre-*:latest` → `ghcr.io/integrall-tech/zaplivre-*:${ZAPLIVRE_TAG}` (incl. bootstrap); hosts `*.associahub.com.br` → `*.zaplivre.app` (signal, dht1/dht2, turn, store, push, identity).
- [x] **C7.** CD mínimo: health-check HTTP `/health` em `devops/scripts/health-check.sh`; bloquear tag `latest` no `deploy.sh`. **Feito** — health-check valida containers + HTTP 200 em 6 endpoints (com `BAIL_HTTP=1` para pré-DNS); `deploy.sh` rejeita `ZAPLIVRE_TAG=latest`.
- [x] **C8.** Documentar backup/restore do Postgres e rotação dos segredos (TURN, PUSH_SERVICE_SECRET, GHCR token). **Feito** — `zaplivre-devops/docs/BACKUP_AND_SECRETS.md`.

## Fase D — Segurança de grupos + escopo sync (P0/P1) — 2-4 dias

- [x] **D1.** Implementar rotação de sender key ao remover/sair de membro (`core/src/crypto/group.rs`, `core/src/group/manager.rs`). **Feito** — `rotate_sender_key`/`replace_member_sender_key` no `GroupSessionManager`; `rotate_my_sender_key` no `GroupManager`; admin rotaciona e distribui nova seed (E2E) após `remove_group_member`; `rotate_and_distribute` no `builder.rs` dispara para MEMBER_REMOVED/LEAVE; testes de rotação.
- [x] **D2.** Bloquear envio do `GroupControlEnvelope` (contém seed) em plaintext sem sessão E2E — remover fallback em `core/src/api/client.rs:2112-2124` ou exigir `ZAPLIVRE_ALLOW_PLAINTEXT`. **Feito** — envelopes com `sender_key_seed` agora exigem E2E obrigatoriamente (erro se sem sessão), mesmo com `ZAPLIVRE_ALLOW_PLAINTEXT=true` (SEC-02).
- [x] **D3.** Decidir escopo do módulo `sync` (`core/src/sync/`): implementar CRDT multi-device ou remover do anúncio no `lib.rs`/README. **Feito** — adiado (fora da homologação); módulo vira placeholder documentado e o anúncio "CRDTs for multi-device sync" foi removido do `lib.rs` (doc drift).
- [x] **D4.** Registrar avaliação/auditoria da dependência `libsignal-protocol-syft 0.85.3-beta.5` (fork beta) e considerar pin + plano de upgrade. **Feito** — `docs/AUDIT_LIBSIGNAL_SYFT.md`: veredito aceitável para beta fechado com pin no Cargo.lock, revisão do consumo e plano de upgrade.

## Fase E — Validação de homologação (P0/P1) — 3-5 dias

- [x] **E1.** Corrigir `.so` x86_64 do Android (era ARM aarch64) e atualizar arm64 (NDK 28.2; symlinks `<triple>-ar → llvm-ar` no `build-native.sh`); APK debug rebuildado e instalado no device físico arm64.
- [x] **E2.** Gerar `libzaplivre_core_ios.a` (arm64 device) e `libzaplivre_core_sim.a` universal (x86_64+arm64 via `lipo -create`); `xcodebuild` iOS Simulator e device → BUILD SUCCEEDED; README iOS atualizado.
- [x] **E3.** Suíte Maestro Android **10/10 verdes** em device físico (Samsung SM-X115, ~5m53s). Fixes: `testTagsAsResourceId` aplicado **no conteúdo de cada AlertDialog** (janela própria não herda o flag da raiz do MainActivity); `seed_peer` registra peer E2E com peer ID libp2p real; `set_contact_prekey_bundle` normaliza bundle DTO (base64) → core; flows corrigidos (id do diálogo de username, scroll para "Sair", `hideKeyboard` antes do back na busca, ASCII no envio). Detalhes em `e2e/maestro/README.md`.
- [x] **E4.** VoIP real validado: `voip_integration` com `--features voip -- --include-ignored` → 9/9 ok; `CallManager.startAudio()` reescrito (inicia `audioManager.start()` + callback `onAudioCaptured` → `sendAudioFrame`; guarda `audioStarted` contra double-start); `didActivate` refatorado para usar `startAudio()`; iOS build compila.
- [x] **E5.** Bug de envio offline corrigido: `send_group_message` **persiste a mensagem (status Pending) antes** do publish (regressão 4b "tabela vazia"); teste `test_group_message_persists_locally` adicionado. Bônus: envio 1:1 para peer offline normalizado (bundle DTO) → Pending em vez de falhar; teste `test_dto_prekey_bundle_is_normalized`.
- [x] **E6.** `GET /api/stats` exige `Authorization: Bearer <PUSH_SERVICE_SECRET>` (`verify_service_token` em `server/store/src/auth.rs` com `subtle::ConstantTimeEq`); comparação do secret do push também constant-time (`server/push/src/auth.rs`); `subtle` no workspace deps; testes store/push verdes.
- [x] **E7.** Desktop: criar view de Settings; decidir escopo de bundle (macOS vs cross-platform). **Feito** — `desktop/src/views/SettingsView.tsx` (toggles de notificação/privacidade + identidade + backup, espelha o iOS) com rota `/settings` e botão na `ConversationsView`; tsc + 79 testes vitest verdes. **Bundle: macOS** (`app`+`dmg` no `tauri.conf.json`) para homologação; cross-platform (Windows/Linux) adiado.

## Fase F — P2/P3 (após homologação, se houver tempo)

- [x] **F1.** Criar testes instrumentados Android (`androidTest`). **Feito** — `AppSettingsTest` (persistência do F2) e `SettingsScreenTest` (UI Compose dos toggles, testTags `settings_toggle_*`); 7 testes, 0 falhas em device (SM-X115 e 25028PC03G). Infra já existia no `build.gradle.kts` (androidTestImplementation + compose BOM + testInstrumentationRunner).
- [x] **F2.** Persistir toggles de Settings no Android. **Feito** — `core/AppSettings.kt` (SharedPreferences `zaplivre_settings`); `SettingsScreen` carrega/persiste os 5 toggles; `NotificationHelper` respeita `notifications_enabled` (gate) e `sound/vibration` (recria o canal); suíte Maestro **10/10 verdes** com o APK final.
- [x] **F3.** Remover doc drift. **Feito** — identity README/`.env.example` (porta 8080→8083, db `zaplivre`); push `.env.example` (FCM legacy `FCM_SERVER_KEY` → v1 `FCM_SERVICE_ACCOUNT_PATH`); desktop README (5173→5174). Bônus: URLs de serviço do iOS movidas pro `project.yml` (o xcodegen dropava `IDENTITY_SERVER_URL`/`MESSAGE_STORE_URL`/`SIGNALING_SERVER_URL` do Info.plist na regeração).
- [x] **F4.** Scan de vulnerabilidade + SBOM nas imagens; remover tag `latest` do GHCR. **Feito** — workflow `build-server-images.yml`: tag `latest` removida (deploys usam sha/branch/semver) + passos Trivy (scan não-bloqueante HIGH/CRITICAL) e SBOM CycloneDX, ambos uploadados como artifacts.
- [x] **F5.** Limpar dead code. **Feito** — presença: funções `is_peer_online`/`set_peer_online`/`set_peer_offline` removidas do `redis_client.rs` do store e schema `user_presence`/`update_presence` removido do `init.sql` (com atualização do `MIGRATIONS.md`); `utils/config.rs` (placeholder) removido; `scripts/build.sh` iOS agora delega pro `ios/build-all.sh` (sem cargo-lipo/xcworkspace). `traefik/dynamic/` vazio não existe neste repo.
- [x] **F6.** Revisar teste `testFormatTimestampKnownDefectAlwaysReturnsAgora` (iOS) e `NatType::PortRestricted` inalcançável. **Feito** — `formatTimestamp` corrigido para comparar o intervalo total (`timeIntervalSince`) em vez dos restos por unidade; teste substituído por `testFormatTimestampMinutesHoursAndDate` (valida "5min"/"3h"/dd/MM/yyyy); suíte iOS passa. `NatType::Restricted`/`PortRestricted` e `ConnectionStrategy::HolePunchFirst` removidos (nunca produzidos pelo heurístico de endereços — detecção real exige STUN/AutoNAT, NAT-01); `relay_integration` ajustado.

---

## Ordem sugerida e donos

| Fase | Prioridade | Estimativa | Dono sugerido |
|---|---|---|---|
| A — Verde de novo | P0 | 1-2 d | Core/CI |
| B — Stack dev | P1 | 1-2 d | Backend/DevOps |
| C — Produção P2P | P0/P1 | 3-5 d | DevOps |
| D — Segurança grupos/sync | P0/P1 | 2-4 d | Core |
| E — Validação homologação | P0/P1 | 3-5 d | Mobile+QA |
| F — P2/P3 | baixa | contínuo | Todos |

## Definição de pronto (para abrir homologação)

1. `main` com CI verde (testes, clippy `-D warnings`, fmt).
2. `make up` sobe todos os serviços `healthy` localmente.
3. Deploy de produção (devops) com bootstrap + TURN + FCM/APNs + health-check HTTP.
4. Todos os apps apontando para o mesmo domínio `*.zaplivre.app` com bootstrap/identity/push configuráveis.
5. E2E Maestro Android verde em device e iOS verde; VoIP validado em chamada real.
6. Auditoria de segurança: sender key rotacionada, sem plaintext de seed, secrets em secret manager.

---

## Re-auditoria V4 (2026-08-09) — resultado

Re-auditoria rigorosa de todos os gaps do `AUDIT_REPORT_V4.md`, com verificação
de código + CI real. Veredito: **pronto para homologação**, restando apenas
itens que dependem de ação do usuário ou são limitações assumidas.

### Corrigido nesta re-auditoria (commit `470d8fd`)

- **Bloqueador (regressão F4):** `build-server-images.yml` usava
  `trivy-action@0.28.0` (tag inexistente) → **nenhuma imagem GHCR era
  publicada desde a Fase F**. Corrigido para `v0.36.0`; run verde
  (6m32s) publicou as 6 imagens (identity, store, push, turn-credentials,
  signaling, bootstrap).
- **P1-5:** teste de integração do identity sem campos kyber (obrigatórios no
  modelo `models.rs:20-22`); o `RegisterRequest` do teste quebraria o
  register. Corrigido (`integration_tests.rs` inclui kyber).
- **P1-6:** `turns:5349` anunciado sem listener TLS (coturn `no-tls`) →
  `turn-credentials` agora só anuncia `turns:` com `TURN_TLS_ENABLED=true`;
  `TURN_HOST`/`TURN_TLS_ENABLED` documentados no `.env.example`.
- **C6 (resíduo):** `server/bootstrap/stack.yml` legado ainda usava
  `zaplivre-bootstrap:latest` + `dht1/dht2.associahub.com.br` → alinhado ao
  stack raiz (GHCR + `${ZAPLIVRE_TAG}` + `*.zaplivre.app`).
- **Doc drift:** `e2e/maestro/README` (toggles persistem desde F2),
  `desktop/README` + README raiz (bundle só macOS), `push/README`
  (FCM HTTP v1/PSH-01), `identity/README` (assinatura SEC-14),
  `self-hosting`/`github-secrets`/`BUILD_AND_TEST`/`android/README`
  (domínios `*.zaplivre.app`). Zero ocorrências de `associahub` em docs ativas.

### Verificado e já corrigido (sem ação)

- A1–A6 (testes E2E/plaintext, clippy, fmt, gates CI), B1–B5 (healthchecks
  Redis/coturn, monitoring, make up, MIGRATIONS), C1–C8 (bootstrap+coturn no
  compose devops, imagem no pipeline, domínios, APNs/FCM guia, GHCR tags,
  health-check HTTP, BACKUP_AND_SECRETS), D1–D4 (rotação sender key, seed
  E2E SEC-02, sync placeholder, libsignal audit), E1–E7 (.so, libs iOS, Maestro
  10/10, VoIP 9/9, offline send, stats auth, desktop settings),
  F1–F6 (instrumented 7/7, AppSettings, doc drift, SBOM/Trivy, dead code,
  formatTimestamp/NAT), mais as revisões de código da Fase E (bundle DTO na
  leitura, `to_core` TryInto, backup modal, best-effort group status,
  `audioStarted`/tap duplicado).

### Itens abertos (ação do usuário)

| Item | Status |
|---|---|
| **C5** — chaves reais APNs (`.p8`, `APNS_KEY_ID/TEAM_ID`) + FCM service account | Devops pronto + guia `docs/guides/push-keys-homologacao.md`; falta o usuário gerar/provisionar e validar entrega em device |
| **P1-4** — rate-limit/anti-replay process-local | Limitação assumida (comentada no código); produção com réplica única ou enforcement em gateway |

### Limitações documentadas (aceitas)

- `armeabi-v7a` só com `BUILD_ANDROID_ALL=1` (homologação usa arm64/x86_64).
- iOS libs `.a` gitignoradas (geradas por `ios/build-rust.sh`).
- Sync multi-device adiado (placeholder `core/src/sync/mod.rs`).
