# Plano de Tarefas — Preparação para Homologação

**Data:** 2026-08-06
**Base:** AUDIT_REPORT_V4.md
**Critério de saída:** CI verde em `main` + stack dev funcional (`make up`) + deploy de produção cobrindo P2P/TURN + apps validados em device.

---

## Fase A — Restaurar build verde (P0, bloqueante) — 1-2 dias

- [ ] **A1.** Corrigir `core/tests/message_integration.rs` (TST-02) para o contrato "nunca plaintext por padrão": estabelecer sessão E2E (prekey exchange via `ensure_remote_prekey`/bundle) antes do `send_text_message`, ou ajustar o teste à política. Evidência: falha `Crypto("No E2E session... plaintext fallback is disabled")` em `message_integration.rs:97`.
- [ ] **A2.** Corrigir clippy `-D warnings` no workspace:
  - `zaplivre-store`: `error: associated function with_interval is never used`.
  - `zaplivre-bootstrap`: 2 erros clippy.
- [ ] **A3.** Rodar `cargo fmt` no workspace (3 arquivos divergentes: `core/src/api/client.rs`, `core/src/ffi/client.rs`).
- [ ] **A4.** Adicionar gates de clippy/fmt ao `.github/workflows/ci.yml` (alinhar com AGENTS.md).
- [ ] **A5.** Push branch → PR → merge em `main` **passando CI completo** (13 commits órfãos c58c0f4..764ef7b).
- [ ] **A6.** Verificar se `test_end_to_end_message_exchange` cobre também o path de fallback plaintext com `ZAPLIVRE_ALLOW_PLAINTEXT=1` (teste do downgrade explícito).

## Fase B — Stack dev funcional (P1) — 1-2 dias

- [ ] **B1.** Corrigir healthcheck do Redis em `docker-compose.yml:36`: usar `redis-cli -a $REDIS_PASSWORD ping` (espelhar `stack.yml:39`) ou `--no-auth-warning`.
- [ ] **B2.** Corrigir `server/coturn/healthcheck.sh`: testar só porta 3478 (remover 5349/TLS enquanto `no-tls`); usar `nc` compatível com Alpine (`open` em vez de `succeeded`).
- [ ] **B3.** Criar `server/monitoring/prometheus.yml` e dashboards Grafana, **ou** remover as referências de `docker-compose.yml:260,279-280`, `stack.yml:338,361-362` e do alvo `make up-monitoring` (Makefile:37).
- [ ] **B4.** Validar `make up` de ponta a ponta: postgres, redis, coturn, turn-credentials, bootstrap, store, push, identity, signaling todos `healthy`.
- [ ] **B5.** Definir/decidir estratégia de migração de schema Postgres (init.sql só roda em volume vazio) — documentar processo de evolução.

## Fase C — Produção P2P/DevOps (P0/P1) — 3-5 dias

- [ ] **C1.** Adicionar serviço `bootstrap` ao `docker-compose.yml` de `zaplivre-devops` (porta P2P + health) e `coturn`/TURN com credenciais do `turn-credentials` (TURN_STATIC_SECRET, TURN_EXTERNAL_IP).
- [ ] **C2.** Adicionar `zaplivre-bootstrap` (e `coturn` se buildado) à matriz de imagens em `.github/workflows/build-server-images.yml`.
- [ ] **C3.** Trocar bootstrap hardcoded `dht1/dht2.associahub.com.br` em `core/src/ffi/client.rs:1003-1006` por nós `*.zaplivre.app` configuráveis (env/build-config exposto nos apps).
- [ ] **C4.** Unificar domínios: iOS `Info.plist:25-46` e `PushNotificationManager.swift:26` de `*.associahub.com.br` → `*.zaplivre.app`; alinhar `project.yml` × `Info.plist` (IDENTITY_SERVER_URL/PUSH_SERVER_URL).
- [ ] **C5.** Provisionar APNs no devops (`.p8`, APNS_KEY_ID, APNS_TEAM_ID, mount read-only) e FCM real; validar entrega iOS+Android.
- [ ] **C6.** Corrigir `stack.yml` (nomes de imagem com registry GHCR; remover hosts legados) ou substituí-lo pelo compose do devops como fonte da verdade.
- [ ] **C7.** CD mínimo: health-check HTTP `/health` em `devops/scripts/health-check.sh`; bloquear tag `latest` no `deploy.sh`; `restart:`/`healthcheck:`/log rotation no compose de produção.
- [ ] **C8.** Documentar backup/restore do Postgres e rotação dos segredos (TURN, PUSH_SERVICE_SECRET, GHCR token).

## Fase D — Segurança de grupos + escopo sync (P0/P1) — 2-4 dias

- [ ] **D1.** Implementar rotação de sender key ao remover/sair de membro (`core/src/crypto/group.rs`, `core/src/group/manager.rs`).
- [ ] **D2.** Bloquear envio do `GroupControlEnvelope` (contém seed) em plaintext sem sessão E2E — remover fallback em `core/src/api/client.rs:2112-2124` ou exigir `ZAPLIVRE_ALLOW_PLAINTEXT`.
- [ ] **D3.** Decidir escopo do módulo `sync` (`core/src/sync/`): implementar CRDT multi-device ou remover do anúncio no `lib.rs`/README.
- [ ] **D4.** Registrar avaliação/auditoria da dependência `libsignal-protocol-syft 0.85.3-beta.5` (fork beta) e considerar pin + plano de upgrade.

## Fase E — Validação de homologação (P0/P1) — 3-5 dias

- [ ] **E1.** Corrigir `.so` x86_64 do Android (`build-native.sh` com `BUILD_ANDROID_ALL=1`) ou remover do `abiFilters`; validar em emulador Intel e device físico arm64.
- [ ] **E2.** Gerar `libzaplivre_core_ios.a` (device) e documentar build via `build-rust.sh`; validar build device assinado.
- [ ] **E3.** Rodar suíte Maestro Android em device/emulador (10 flows, hoje só `check-syntax`); corrigir os flows que falharem.
- [ ] **E4.** Validar chamada VoIP real (desbloquear/rodar 7 testes `voip_integration` `#[ignore]`); verificar TODO de áudio `ios/.../CallManager.swift:325`.
- [ ] **E5.** Decidir e corrigir bug de envio offline (persistência local + retry + feedback de erro) se ainda reproduzir (ISSUES_BACKLOG 4b).
- [ ] **E6.** Autenticar `GET /api/stats` do store (`server/store/src/api.rs:215`) e comparação constant-time do `PUSH_SERVICE_SECRET` (`server/push/src/auth.rs:105`).
- [ ] **E7.** Desktop: criar view de Settings; decidir escopo de bundle (macOS vs cross-platform).

## Fase F — P2/P3 (após homologação, se houver tempo)

- [ ] **F1.** Criar testes instrumentados Android (`androidTest`).
- [ ] **F2.** Persistir toggles de Settings no Android.
- [ ] **F3.** Remover doc drift (identity README, push `.env.example` FCM legacy, desktop README porta 5173→5174).
- [ ] **F4.** Scan de vulnerabilidade + SBOM nas imagens; remover tag `latest` do GHCR.
- [ ] **F5.** Limpar dead code (presença `user_presence`, `is_peer_online`), `utils/config.rs`, `traefik/dynamic/` vazio, `scripts/build.sh` iOS.
- [ ] **F6.** Revisar teste `testFormatTimestampKnownDefectAlwaysReturnsAgora` (iOS) e `NatType::PortRestricted` inalcançável.

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
