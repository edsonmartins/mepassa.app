# Auditoria Técnica V4 — ZapLivre

**Data:** 2026-08-06
**Escopo:** core (Rust), server (bootstrap/identity/push/store/signaling/turn-credentials/coturn), apps (android/ios/desktop), e2e (Maestro), CI/CD (.github), devops (zaplivre-devops), docker-compose/stack.
**Método:** análise estática (4 agentes) + verificação prática: `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --check`, `docker compose config`, `git fetch`, `gh run list`, worktree de `origin/main` para comparação.

## Veredito

**NÃO está pronto para homologação.** Existem bloqueadores objetivos (P0) que impedem abrir o ciclo de homologação com base, incluindo a branch de trabalho atual **quebrada** (teste de integração falhando) e o deploy de produção **sem os componentes P2P/TURN** do produto. A infraestrutura de fundo (E2E 1:1 real, mídia, grupos assinados, VoIP, store-and-forward) está substancialmente implementada e um beta fechado é plausível após os P0/P1 abaixo.

---

## 1. Evidências práticas (verificadas na máquina)

| # | Achado | Evidência |
|---|---|---|
| V1 | `cargo test --workspace` FALHA na branch `feat/rebrand-zaplivre` (764ef7b) | `test_end_to_end_message_exchange` → `Crypto("No E2E session with ... and plaintext fallback is disabled")` em `core/tests/message_integration.rs:97` |
| V2 | Os mesmos testes PASSAM em `origin/main` (927103e) | Worktree em `/tmp/zl-main-check`: `message_integration` 3/3 ok |
| V3 | 13 commits da branch nunca passaram pelo CI de testes | `git log origin/main..HEAD` = c58c0f4..764ef7b; último run `ci.yml` = 06/07 (PR #4) via `gh run list` |
| V4 | `cargo clippy --workspace -- -D warnings` FALHA | `error: associated function with_interval is never used` (store), erros em bootstrap |
| V5 | `cargo fmt --check` FALHA em 3 arquivos | `core/src/api/client.rs:142`, `core/src/ffi/client.rs:116,1201` |
| V6 | `c58c0f4` inverteu o default de E2E sem atualizar testes | `e2e_required()` passou de `ZAPLIVRE_REQUIRE_E2E` para `!ZAPLIVRE_ALLOW_PLAINTEXT` (`core/src/api/client.rs:2567-2571`) — a intenção é correta (nunca plaintext por padrão), mas TST-02 não estabelece sessão/prekey antes de enviar |
| V7 | `docker compose config` valida com secrets obrigatórios `:?` | Enforce funciona; falha esperada sem `PUSH_SERVICE_SECRET` |
| V8 | Redis healthcheck sem autenticação | `docker-compose.yml:36` `redis-cli --raw incr ping` contra Redis com `requirepass` → NOAUTH; `stack.yml:39` já corrige com `-a`. `store`/`identity` (depends_on `service_healthy`) nunca sobem |
| V9 | Healthcheck do coturn quebrado | `server/coturn/healthcheck.sh:5-13`: testa porta 5349 (TLS desligado via `no-tls`) e usa grep `succeeded` (saída Debian nc; imagem é Alpine/busybox → `open`). `turn-credentials` nunca sobe |
| V10 | `server/monitoring/` inexistente, referenciado no compose | `docker-compose.yml:260,279-280`; `make up-monitoring` (Makefile:37) quebra |

---

## 2. Gaps P0 — Bloqueadores

### 2.1 Processo / Build
1. **Branch de trabalho vermelha (V1/V3/V4/V5):** regressão de teste E2E + clippy `-D warnings` falhando + fmt sujo; 13 commits sem CI. Risco: homologar sobre base não verificada.
2. **Testes de integração não refletem o novo default de segurança:** TST-02 precisa estabelecer sessão E2E (prekey exchange) ou o teste deve ser ajustado ao contrato "nunca plaintext por padrão" — sem isso, o CI (que roda `--test message_integration`) ficaria vermelho após o merge.

### 2.2 DevOps (zaplivre-devops)
3. **Deploy de produção não cobre o núcleo P2P:** sem bootstrap node e sem coturn/TURN no `docker-compose.yml` do devops (só `turn-credentials`, que emite credenciais sem servidor TURN). VoIP e descoberta P2P não funcionam em produção.
4. **Imagem `zaplivre-bootstrap` fora do pipeline de imagens:** `.github/workflows/build-server-images.yml:24-34` publica identity/store/push/turn-credentials/signaling — bootstrap ausente; `stack.yml`/`server/bootstrap/stack.yml` dependem de `zaplivre-bootstrap:latest`.
5. **Cliente hardcoda bootstrap legado:** `core/src/ffi/client.rs:1003-1006` → `dht1/dht2.associahub.com.br` + fallback IPFS público (1008-1014). Override só via env `ZAPLIVRE_BOOTSTRAP`, não exposto nos apps.
6. **Divergência de domínio de produção:** Android/Desktop `*.zaplivre.app`; iOS/core `*.associahub.com.br` (`ios/.../Info.plist:25-46`, `PushNotificationManager.swift:26`). Bloqueante cross-platform.
7. **APNs não provisionado em produção:** devops monta apenas FCM; push iOS silencioso em produção.
8. **Sem CD, monitoramento, alertas e backup em produção:** deploy 100% manual via SSH (`scripts/deploy.sh`); health-check não faz HTTP GET `/health`; sem Prometheus/Grafana/backups.

### 2.3 Core
9. **Sync multi-device é stub:** `core/src/sync/mod.rs` vazio; `automerge` comentado (`core/Cargo.toml:45-46`) — anunciado no lib.rs como "CRDTs for multi-device sync".
10. **Segurança de grupos:** sem rotação de sender key ao remover/sair de membro (`crypto/group.rs:20-23` admite); fallback plaintext do envelope de controle (contém seed de sender key) quando não há sessão Signal (`api/client.rs:2112-2124`).
11. **Dependência de criptografia central em fork beta:** `libsignal-protocol-syft 0.85.3-beta.5` (`core/Cargo.toml:42`), sem auditoria/benchmarks registrados.

### 2.4 Apps
12. **Android `.so` x86_64 errado:** `jniLibs/x86_64/libzaplivre_core.so` é binário aarch64 (e_machine=183) → crash em emulador Intel; `build-native.sh:121-123` só compila arm64 por default.
13. **iOS sem lib de device:** `libzaplivre_core_ios.a` não está no repo (`.gitignore:61`); `xcodegen generate` regrediria `IDENTITY_SERVER_URL` (project.yml × Info.plist); TODO de áudio WebRTC em `CallManager.swift:325`.
14. **Desktop sem view de Settings e bundle só macOS** (`tauri.conf.json:40`), apesar do README "cross-platform".

---

## 3. Gaps P1 — Alta prioridade (produção/beta fechado)

| # | Gap | Evidência |
|---|---|---|
| P1-1 | Healthchecks de dev quebrados (Redis/coturn) — `make up` não sobe stack completo | `docker-compose.yml:36`, `server/coturn/healthcheck.sh` |
| P1-2 | `GET /api/stats` do store sem autenticação | `server/store/src/api.rs:115,215-228` |
| P1-3 | Comparação não-constante do `PUSH_SERVICE_SECRET` | `server/push/src/auth.rs:105` |
| P1-4 | Anti-replay/rate-limit process-local (multi-réplica enfraquece) | `server/store/src/auth.rs:20-21`, `push/src/auth.rs:11-12`, `turn-credentials/src/request_auth.rs:11-12`, `signaling/src/main.rs:34` |
| P1-5 | Testes de integração do identity driftados (Kyber obrigatório no modelo, ausente nos testes) | `server/identity/src/models.rs:20-22` vs `tests/integration_tests.rs:92-104` |
| P1-6 | `turns:5349` anunciado sem TLS no coturn; `TURN_HOST` nunca configurado | `turn-credentials/src/config.rs:33,28` |
| P1-7 | Nomes de imagem inconsistentes no `stack.yml` (locais vs GHCR) | `stack.yml:86,119,196,233,270,302` |
| P1-8 | Sem estratégia de migração de schema (init.sql só em volume vazio) | `docker-compose.yml:17` |
| P1-9 | E2E Android Maestro nunca executado em device (só `check-syntax`) | `e2e/maestro/README.md` |
| P1-10 | Bug de envio offline/erro engolido documentado (validar se ainda ocorre) | `e2e/maestro/README.md`, `ISSUES_BACKLOG.md:4b` |
| P1-11 | 7 testes VoIP `#[ignore]` (fluxo completo só manual com hardware) | `core/tests/voip_integration.rs` |
| P1-12 | Google-services.json com API key commitada | `android/app/google-services.json` |

---

## 4. Gaps P2 — Média prioridade

| # | Gap | Evidência |
|---|---|---|
| P2-1 | `GET /api/stats` não autenticado; dead code de presença (`user_presence`, `is_peer_online`) | `server/store/src/redis_client.rs:40-80`, `server/postgres/init.sql:90-106` |
| P2-2 | Doc drift: identity README (formato de assinatura/schema), push `.env.example` (FCM legacy) | `server/identity/README.md:155-203`, `server/push/.env.example:4-6` |
| P2-3 | Sem testes instrumentados Android (`androidTest`) | `android/app/build.gradle.kts` |
| P2-4 | Toggles de Settings não persistem (Android) | `e2e/maestro/README.md` |
| P2-5 | `scripts/build.sh` iOS obsoleto (xcworkspace/cargo-lipo) e lint não-bloqueante | `scripts/build.sh:54-61,116-127` |
| P2-6 | CI sem clippy/fmt como gate (AGENTS.md promete, CI não aplica) | `.github/workflows/ci.yml` |
| P2-7 | Imagens sem scan de vulnerabilidade/SBOM/assinatura; tag `latest` publicada | `build-server-images.yml:51-65` |
| P2-8 | `utils/config.rs` placeholder vazio | `core/src/utils/config.rs:6` |
| P2-9 | DNS transport desabilitado no Android (`/dns4/` bootstrap) | `core/src/network/transport.rs:64` |
| P2-10 | `deploy.sh` aceita `latest`; `traefik/dynamic/` vazio; `stack.yml` legado com domínios antigos | devops `scripts/deploy.sh:6`, `traefik/dynamic/`, `server/bootstrap/stack.yml:5,21` |

---

## 5. Gaps P3 — Baixa prioridade (polimento)

- Warnings e imports não utilizados em core (19 warnings clippy).
- Docs desatualizadas: desktop README porta 5173→5174; `FASE_*`/`EXECUCAO.md` históricos.
- `NatType::PortRestricted` inatingível na heurística de NAT (`nat_detection.rs:104-117`).
- Peer ID bootstrap hardcoded bloqueia migração para chave aleatória (`server/bootstrap/src/main.rs:302-304`).

---

## 6. Recomendações de ordem de execução

**Fase A — Verde de novo (1-2 dias):** corrigir TST-02 para o contrato "nunca plaintext por padrão" (ou garantir prekey exchange no teste), clippy `-D warnings`, fmt; rodar CI completo; mesclar branch → main.

**Fase B — Infra dev funcional (1-2 dias):** corrigir Redis/coturn healthchecks; criar `server/monitoring/` ou remover referências; validar `make up` de ponta a ponta.

**Fase C — Produção P2P (3-5 dias):** adicionar bootstrap + coturn ao compose devops e ao CI de imagens; corrigir bootstrap hardcoded p/ `*.zaplivre.app`; unificar domínios nos apps; provisionar APNs + secrets reais; CD com health-check HTTP.

**Fase D — Segurança grupos/sync (2-4 dias):** rotação de sender key; bloquear seed sem sessão E2E; decidir escopo do sync (implementar ou remover do anunciado).

**Fase E — Validação homologação (3-5 dias):** E2E Maestro Android em device; chamada VoIP real (desbloquear testes); iOS build device assinado; corrigir `.so` x86_64; testes instrumentados Android.

---

## 7. Conclusão

A base técnica é real e madura para um beta fechado (E2E 1:1 PQXDH, mídia, grupos, VoIP, store-and-forward, segredos limpos). Porém, os bloqueadores de processo (branch quebrada sem CI), de infraestrutura (bootstrap/TURN ausentes em produção, healthchecks quebrados) e de segurança/escopo (sync inexistente, sender key sem rotação, domínios divergentes) impedem a abertura da homologação hoje.
