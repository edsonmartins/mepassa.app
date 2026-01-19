# Plano de Execução – Projeto MePassa Platform (v1 - FINAL)

> **Instrução:** Sempre que uma tarefa avançar de status, atualize esta tabela com a nova situação e registre a data no campo "Última atualização". Os status sugeridos são `TODO`, `IN_PROGRESS`, `BLOCKED` e `DONE`.

## Legend
- `TODO`: ainda não iniciado.
- `IN_PROGRESS`: em execução.
- `BLOCKED`: impedida por dependência externa.
- `DONE`: concluída e validada.

**IMPORTANTE:**

- Seguir arquitetura **HÍBRIDA P2P + Servidor** (80% P2P direto, 15% TURN relay, 5% store-and-forward)
- Implementar usando Rust (core), Kotlin (Android), Swift (iOS), Tauri (Desktop)
- Core compartilhado via UniFFI (FFI bindings)
- **Prioridade máxima: Chamadas de voz (Mês 4)** - sem isso ninguém adota
- **NÃO implementar testes** extensivos neste momento (foco em MVP funcional)
- **NÃO implementar observabilidade** complexa neste momento

**CONTEXTO DO PROJETO:**
MePassa é uma plataforma de mensagens instantâneas híbrida P2P + Servidor, focando em:
1. **Privacidade:** 80% tráfego P2P direto (servidor não vê conteúdo)
2. **Confiabilidade:** Funciona sempre (offline, NAT simétrico, firewall)
3. **Economia:** 85% mais barato que centralizado puro
4. **Chamadas:** VoIP obrigatório para adoção em massa

Diferencial: Como WhatsApp (funciona sempre) + Melhor que WhatsApp (privado, sem ban, self-hosting).

---

## 📊 STATUS GERAL DO PROJETO (Atualizado: 2025-01-19)

### ✅ Fases Planejadas

| Fase | Componente | Progresso | Status | Arquivos | Linhas de Código | Última Atualização |
|------|------------|-----------|--------|----------|------------------|--------------------|
| **FASE 0: Setup & Fundação** | Infra | 70% | `IN_PROGRESS` | 7/10 | ~3.500/500 | 2025-01-19 |
| **FASE 1: Core - Identidade & Crypto** | Rust | 100% | `DONE` | 15/15 | ~3.024/2.000 | 2025-01-19 |
| **FASE 1.5: Identity Server & Username** | Rust | 0% | `TODO` | 0/12 | 0/1.500 | - |
| **FASE 2: Core - Networking P2P** | Rust | 0% | `TODO` | 0/8 | 0/1.500 | - |
| **FASE 3: Core - Storage Local** | Rust | 0% | `TODO` | 0/8 | 0/1.200 | - |
| **FASE 4: Core - Protocolo & API** | Rust | 0% | `TODO` | 0/10 | 0/1.500 | - |
| **FASE 5: Core - FFI (UniFFI)** | Rust | 0% | `TODO` | 0/5 | 0/800 | - |
| **FASE 6: Android - Setup & UI** | Kotlin | 0% | `TODO` | 0/25 | 0/3.000 | - |
| **FASE 7: Desktop - Setup & UI** | Tauri | 0% | `TODO` | 0/20 | 0/2.500 | - |
| **FASE 8: Push Notifications** | Multi | 0% | `TODO` | 0/8 | 0/1.000 | - |
| **FASE 9: Server - Bootstrap & DHT** | Rust | 0% | `TODO` | 0/6 | 0/800 | - |
| **FASE 10: Server - TURN Relay** | Rust | 0% | `TODO` | 0/5 | 0/600 | - |
| **FASE 11: Server - Message Store** | Rust | 0% | `TODO` | 0/10 | 0/1.500 | - |
| **FASE 12: VOIP - Chamadas** 🔥 | Multi | 0% | `TODO` | 0/15 | 0/2.500 | - |
| **FASE 13: iOS App** | Swift | 0% | `TODO` | 0/30 | 0/4.000 | - |
| **FASE 14: Videochamadas** | Multi | 0% | `TODO` | 0/12 | 0/1.800 | - |
| **FASE 15: Grupos** | Multi | 0% | `TODO` | 0/15 | 0/2.000 | - |
| **FASE 16: Mídia & Polimento** | Multi | 0% | `TODO` | 0/20 | 0/2.500 | - |
| **FASE 17: Multi-Device Sync** | Rust | 0% | `TODO` | 0/10 | 0/1.500 | - |

**TOTAIS:**
- **Fases:** 19 (incluindo FASE 1.5 - Identity Server)
- **Arquivos estimados:** ~244
- **Linhas de código:** ~32.700
- **Duração:** ~6-7 meses

---

## 🎯 FASE 0: SETUP & FUNDAÇÃO (Mês 1-2)

### Objetivo
Estrutura base do repositório, CI/CD, documentação inicial, comunidade.

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| 0.1 | Criar organização GitHub (integralltech/mepassa) | `BLOCKED` | Manual | - | - | 2025-01-19 | Acesso externo necessário |
| 0.2 | Setup monorepo (estrutura de pastas completa) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | - |
| 0.3 | Configurar GitHub Actions (CI/CD básico) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 0.2 |
| 0.4 | Configurar Rust workspace (Cargo.toml principal) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 0.2 |
| 0.5 | Criar README.md + CONTRIBUTING.md + CODE_OF_CONDUCT.md | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 0.2 |
| 0.6 | Setup Docker Compose (dev environment) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 0.2 |
| 0.7 | Registrar domínio (mepassa.app) | `BLOCKED` | Manual | - | - | 2025-01-19 | Acesso externo necessário |
| 0.8 | Setup Discord/Matrix para comunidade | `BLOCKED` | Manual | - | - | 2025-01-19 | Acesso externo necessário |
| 0.9 | Criar landing page (captação beta testers) | `TODO` | - | - | - | - | 0.7 |
| 0.10 | Documentar arquitetura híbrida (docs/) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 0.2 |

**Entregáveis:**
- ✅ Repositório GitHub público - **PARCIAL** (local, precisa push)
- ✅ CI/CD básico funcionando - **DONE** (4 workflows criados)
- ✅ Documentação inicial - **DONE** (README, CONTRIBUTING, CODE_OF_CONDUCT)
- ⏸️ Landing page captando emails - **BLOCKED** (depende domínio)
- ⏸️ 50-100 beta testers cadastrados - **BLOCKED** (depende landing page)

**Arquivos Criados:** ~65
**LoC:** ~3.500 (excedeu estimativa inicial por configuração detalhada)

### 📋 Resumo FASE 0 (70% Concluída)

**✅ CONCLUÍDO:**
1. Estrutura monorepo completa (core/, android/, ios/, desktop/, server/, docs/)
2. Cargo.toml workspace configurado com todas dependências
3. Core library skeleton (9 módulos: identity, crypto, network, storage, sync, voip, protocol, api, utils)
4. GitHub Actions: 4 workflows (core-ci, android-ci, ios-ci, desktop-ci)
5. Docker Compose: 8 services (PostgreSQL, Redis, coturn, bootstrap, store, push, prometheus, grafana)
6. Documentação:
   - README.md (projeto completo)
   - CONTRIBUTING.md (guia de contribuição)
   - CODE_OF_CONDUCT.md (código de conduta)
   - docs/architecture/ (2 documentos detalhados)
   - docs/guides/getting-started.md
7. Ferramentas:
   - Makefile (30+ comandos)
   - scripts/build.sh
   - .env.example
   - .gitignore completo
8. Database schema PostgreSQL completo (init.sql)
9. TURN server configuração (turnserver.conf)
10. Dockerfiles para todos servidores (bootstrap, store, push)

**🚫 BLOQUEADO (Acesso Externo):**
- Criar organização GitHub (requer conta)
- Registrar domínio mepassa.app (requer registrador)
- Setup Discord/Matrix (requer acesso às plataformas)

**⏭️ PRÓXIMO:**
- Landing page (após domínio registrado)

**🎯 Pronto para FASE 1:** SIM ✅

O ambiente de desenvolvimento está completo. Podemos iniciar a implementação do core library.

---

## 🦀 FASE 1: CORE LIBRARY - IDENTIDADE & CRYPTO (Mês 2-3)

### Objetivo
Fundação do mepassa-core: gerenciamento de identidade e criptografia E2E (Signal Protocol).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **1.1 - Setup Core** ||||||||
| 1.1.1 | Criar crate mepassa-core (Cargo.toml com deps) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 0.4 |
| 1.1.2 | Setup estrutura de módulos (lib.rs) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.1.1 |
| 1.1.3 | Configurar dependencies (libp2p, rusqlite, etc) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.1.1 |
| 1.1.4 | Setup logging (tracing + tracing-subscriber) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.1.2 |
| **1.2 - Identidade** ||||||||
| 1.2.1 | Implementar identity/keypair.rs (Ed25519 generation) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.1.2 |
| 1.2.2 | Implementar identity/prekeys.rs (X25519, pool de 100) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.2.1 |
| 1.2.3 | Implementar identity/storage.rs (Keychain seguro) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.2.1 |
| 1.2.4 | Testes unitários identity (28 testes, 100% passed) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.2.3 |
| **1.3 - Criptografia** ||||||||
| 1.3.1 | Implementar crypto/signal.rs (X3DH + AES-GCM, 5 testes) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.2.2 |
| 1.3.2 | Implementar crypto/session.rs (Session management, 9 testes) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.3.1 |
| 1.3.3 | Implementar crypto/ratchet.rs (Double Ratchet, 7 testes) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.3.1 |
| 1.3.4 | Implementar crypto/group.rs (Sender Keys, 9 testes) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.3.2 |
| 1.3.5 | Testes E2E crypto (Alice → Bob encrypted, 59 testes total) | `DONE` | Claude Code | 2025-01-19 | 2025-01-19 | 2025-01-19 | 1.3.4 |

**Entregáveis:**
- ✅ Keypairs gerados (Ed25519)
- ✅ Prekeys gerados (pool de 100)
- ✅ Mensagem E2E encrypted (Alice → Bob)
- ✅ Testes >80% coverage

**Arquivos implementados:**
- `identity/keypair.rs` (~400 linhas, 12 testes)
- `identity/prekeys.rs` (~450 linhas, 13 testes)
- `identity/storage.rs` (~300 linhas, 4 testes)
- `crypto/signal.rs` (~300 linhas, 5 testes)
- `crypto/session.rs` (~450 linhas, 9 testes)
- `crypto/ratchet.rs` (~350 linhas, 7 testes)
- `crypto/group.rs` (~657 linhas, 9 testes) ✨ **NOVO**
- `utils/error.rs`, `utils/logging.rs`, `utils/config.rs` (~100 linhas)

**Resultados dos Testes (2025-01-19 - FINAL):**
```
running 59 tests (identity: 29, crypto: 30)
✅ identity::keypair::tests (12 testes) - 100% passed
✅ identity::prekeys::tests (13 testes) - 100% passed
✅ identity::storage::tests (4 testes) - 100% passed
✅ crypto::signal::tests (5 testes) - 100% passed
  - test_x3dh_key_agreement
  - test_encrypt_decrypt
  - test_encrypt_decrypt_different_key_fails
  - test_nonce_randomness
  - test_e2e_alice_to_bob
✅ crypto::session::tests (9 testes) - 100% passed ✨ NOVO
  - test_session_creation
  - test_session_encrypt_decrypt
  - test_session_manager_create_and_get
  - test_session_manager_encrypt_decrypt
  - test_session_manager_remove
  - test_session_manager_list_sessions
  - test_session_not_found
  - test_e2e_alice_to_bob_with_sessions
  - test_multiple_messages_in_session
✅ crypto::ratchet::tests (7 testes) - 100% passed
  - test_ratchet_state_creation
  - test_ratchet_encrypt_decrypt
  - test_ratchet_multiple_messages
  - test_ratchet_forward_secrecy
  - test_ratchet_different_root_keys
  - test_e2e_with_x3dh_and_ratchet
  - test_counters
✅ crypto::group::tests (9 testes) - 100% passed ✨ NOVO
  - test_sender_key_generation
  - test_sender_key_encrypt_decrypt
  - test_group_session_creation
  - test_group_session_add_remove_members
  - test_group_message_flow
  - test_group_session_manager
  - test_group_with_three_members
  - test_list_groups
  - test_sender_key_forward_secrecy

test result: ok. 59 passed; 0 failed; 0 ignored
```

**Funcionalidades Crypto (COMPLETAS):**
- ✅ X3DH (Simplified): Key agreement usando X25519 prekeys
- ✅ AES-256-GCM: Encryption/decryption com authenticated encryption
- ✅ HKDF-SHA256: Key derivation para shared secrets
- ✅ Session Management: Gerenciamento de sessões E2E com múltiplos peers
- ✅ Double Ratchet: Forward secrecy com ratcheting de chaves
- ✅ Group Messaging: Sender Keys para grupos (até 256 membros) ✨ NOVO
- ✅ E2E flow completo: X3DH + Sessions + Ratchet + Groups funcionando!

**LoC:** ~3.024/2.000 (151% - ultrapassou meta)
**Progresso:** 15/15 tarefas (100%) ✅ FASE 1 COMPLETA!

---

## 🆔 FASE 1.5: IDENTITY SERVER & USERNAME SYSTEM (Mês 2-3)

### Objetivo
Sistema de @username para identificação user-friendly (como Telegram/Signal), substituindo o peer_id criptográfico impossível de compartilhar.

**CONTEXTO:** WhatsApp usa números de telefone, mas isso:
- ❌ Expõe informação pessoal (privacidade ruim)
- ❌ Requer SMS gateway (custo + complexidade)
- ❌ Permite metadata leaking

**DECISÃO:** @username system (ADR 001) - privacidade boa + UX aceitável + custo zero.

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **1.5.1 - Identity Server (Backend)** ||||||||
| 1.5.1.1 | Criar server/identity/ (Rust + Axum) | `TODO` | - | - | - | - | 0.2 |
| 1.5.1.2 | Setup PostgreSQL schema (usernames table) | `TODO` | - | - | - | - | 1.5.1.1 |
| 1.5.1.3 | Implementar POST /api/v1/register (username → peer_id) | `TODO` | - | - | - | - | 1.5.1.2 |
| 1.5.1.4 | Implementar GET /api/v1/lookup?username=X | `TODO` | - | - | - | - | 1.5.1.3 |
| 1.5.1.5 | Implementar PUT /api/v1/prekeys (atualizar prekeys) | `TODO` | - | - | - | - | 1.5.1.3 |
| 1.5.1.6 | Username validation (regex: ^[a-z0-9_]{3,20}$) | `TODO` | - | - | - | - | 1.5.1.3 |
| 1.5.1.7 | Rate limiting (Redis) - anti-spam | `TODO` | - | - | - | - | 1.5.1.3 |
| 1.5.1.8 | Health check endpoint (/health) | `TODO` | - | - | - | - | 1.5.1.1 |
| **1.5.2 - Client Integration** ||||||||
| 1.5.2.1 | Core: Implementar identity_client.rs (HTTP client) | `TODO` | - | - | - | - | 1.5.1.4 |
| 1.5.2.2 | Core: register_username(username, peer_id, prekey_bundle) | `TODO` | - | - | - | - | 1.5.2.1 |
| 1.5.2.3 | Core: lookup_username(username) → (peer_id, prekey_bundle) | `TODO` | - | - | - | - | 1.5.2.1 |
| 1.5.2.4 | Core: update_prekeys() | `TODO` | - | - | - | - | 1.5.2.1 |
| **1.5.3 - Database Schemas** ||||||||
| 1.5.3.1 | PostgreSQL: CREATE TABLE usernames | `TODO` | - | - | - | - | 1.5.1.2 |
| 1.5.3.2 | SQLite (client): ALTER TABLE contacts ADD COLUMN username | `TODO` | - | - | - | - | 3.1.3 |
| **1.5.4 - Testes** ||||||||
| 1.5.4.1 | Teste: registro username único funciona | `TODO` | - | - | - | - | 1.5.1.3 |
| 1.5.4.2 | Teste: lookup retorna peer_id correto | `TODO` | - | - | - | - | 1.5.1.4 |
| 1.5.4.3 | Teste: username duplicado retorna erro 409 | `TODO` | - | - | - | - | 1.5.1.3 |
| 1.5.4.4 | Teste: rate limiting funciona (anti-spam) | `TODO` | - | - | - | - | 1.5.1.7 |

**Entregáveis:**
- ✅ Identity Server rodando (identity.mepassa.app)
- ✅ Usuário pode registrar @username
- ✅ Outro usuário pode buscar @username e obter peer_id
- ✅ Prekey bundle retornado junto para X3DH
- ✅ Rate limiting funciona (anti-spam)

**Schema PostgreSQL:**
```sql
CREATE TABLE usernames (
    username TEXT PRIMARY KEY,
    peer_id TEXT NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,
    prekey_bundle JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_updated TIMESTAMP DEFAULT NOW(),

    CONSTRAINT username_format CHECK (username ~ '^[a-z0-9_]{3,20}$')
);
```

**Schema SQLite (Client):**
```sql
-- Atualização na tabela contacts
ALTER TABLE contacts ADD COLUMN username TEXT UNIQUE;
ALTER TABLE contacts ADD COLUMN prekey_bundle_json TEXT;
CREATE INDEX idx_contacts_username ON contacts(username);
```

**API Endpoints:**
- `POST /api/v1/register` - Registrar username
- `GET /api/v1/lookup?username=joao` - Buscar peer_id
- `PUT /api/v1/prekeys` - Atualizar prekeys
- `GET /health` - Health check

**Flow de Uso:**
1. Alice registra @alice no primeiro uso
2. Bob quer adicionar Alice
3. Bob digita "@alice" no app
4. App busca no Identity Server
5. App obtém peer_id + prekey_bundle
6. App estabelece X3DH + P2P connection

**Arquivos criados:**
- `server/identity/src/main.rs` (~400 linhas)
- `server/identity/src/db.rs` (~200 linhas)
- `server/identity/src/api.rs` (~300 linhas)
- `core/src/identity/identity_client.rs` (~200 linhas)

**LoC:** ~1.500
**Progresso:** 0/18 tarefas (0%)

**Referência:** ADR 001 (docs/decisions/001-username-identity-system.md)

---

## 🌐 FASE 2: CORE LIBRARY - NETWORKING P2P (Mês 3)

### Objetivo
Conectividade P2P básica usando libp2p (conexão direta, sem relay ainda).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **2.1 - Transport Layer** ||||||||
| 2.1.1 | Implementar network/transport.rs (TCP + QUIC) | `TODO` | - | - | - | - | 1.1.3 |
| 2.1.2 | Implementar network/behaviour.rs (libp2p behaviour) | `TODO` | - | - | - | - | 2.1.1 |
| 2.1.3 | Setup Noise protocol (encryption de transporte) | `TODO` | - | - | - | - | 2.1.1 |
| 2.1.4 | Setup Yamux (multiplexing) | `TODO` | - | - | - | - | 2.1.1 |
| **2.2 - Discovery (DHT)** ||||||||
| 2.2.1 | Implementar network/dht.rs (Kademlia DHT) | `TODO` | - | - | - | - | 2.1.2 |
| 2.2.2 | Implementar peer discovery (DHT lookup) | `TODO` | - | - | - | - | 2.2.1 |
| 2.2.3 | Implementar peer routing | `TODO` | - | - | - | - | 2.2.2 |
| **2.3 - P2P Direto** ||||||||
| 2.3.1 | Implementar conexão P2P direta | `TODO` | - | - | - | - | 2.2.3 |
| 2.3.2 | Implementar envio de mensagem P2P | `TODO` | - | - | - | - | 2.3.1 |
| 2.3.3 | Implementar ACK de mensagem | `TODO` | - | - | - | - | 2.3.2 |
| 2.3.4 | Teste E2E: 2 peers conectam e trocam mensagem | `TODO` | - | - | - | - | 2.3.3 |

**Entregáveis:**
- ✅ 2 peers conectam P2P direto (localhost)
- ✅ Mensagem vai peer-to-peer encrypted
- ✅ ACK confirmando entrega

**Arquivos:** `network/transport.rs`, `network/behaviour.rs`, `network/dht.rs`
**LoC:** ~1.500

---

## 💾 FASE 3: CORE LIBRARY - STORAGE LOCAL (Mês 3)

### Objetivo
Persistência local de mensagens, contatos e configurações em SQLite.

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **3.1 - Database Setup** ||||||||
| 3.1.1 | Implementar storage/database.rs (SQLite wrapper) | `TODO` | - | - | - | - | 1.1.3 |
| 3.1.2 | Definir storage/schema.rs (tabelas: messages, contacts, groups) | `TODO` | - | - | - | - | 3.1.1 |
| 3.1.3 | Implementar storage/migrations.rs (schema evolution) | `TODO` | - | - | - | - | 3.1.2 |
| **3.2 - CRUD Operations** ||||||||
| 3.2.1 | Implementar storage/messages.rs (messages CRUD) | `TODO` | - | - | - | - | 3.1.3 |
| 3.2.2 | Implementar storage/contacts.rs (contacts CRUD) | `TODO` | - | - | - | - | 3.1.3 |
| 3.2.3 | Implementar storage/groups.rs (groups CRUD) | `TODO` | - | - | - | - | 3.1.3 |
| 3.2.4 | Setup WAL mode (Write-Ahead Logging) | `TODO` | - | - | - | - | 3.1.1 |
| 3.2.5 | Setup FTS5 (full-text search) | `TODO` | - | - | - | - | 3.2.1 |
| **3.3 - Testes** ||||||||
| 3.3.1 | Testes de persistência (insert/select) | `TODO` | - | - | - | - | 3.2.3 |
| 3.3.2 | Testes de busca (FTS5) | `TODO` | - | - | - | - | 3.2.5 |

**Entregáveis:**
- ✅ Mensagens salvas localmente
- ✅ Query de conversas funciona
- ✅ Busca em mensagens funciona

**Arquivos:** `storage/database.rs`, `storage/schema.rs`, `storage/messages.rs`, `storage/contacts.rs`
**LoC:** ~1.200

---

## 🔀 FASE 4: CORE LIBRARY - PROTOCOLO & API (Mês 3)

### Objetivo
Definir protocolos de mensagem (Protobuf) e API pública do core.

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **4.1 - Protocol Buffers** ||||||||
| 4.1.1 | Definir proto/messages.proto (Message, MessageType, etc) | `TODO` | - | - | - | - | 1.1.3 |
| 4.1.2 | Implementar protocol/codec.rs (encode/decode) | `TODO` | - | - | - | - | 4.1.1 |
| 4.1.3 | Implementar protocol/validation.rs (message validation) | `TODO` | - | - | - | - | 4.1.2 |
| **4.2 - Client API** ||||||||
| 4.2.1 | Implementar api/client.rs (Client struct + métodos) | `TODO` | - | - | - | - | 3.2.3 |
| 4.2.2 | Implementar api/events.rs (Event system: MessageReceived, etc) | `TODO` | - | - | - | - | 4.2.1 |
| 4.2.3 | Implementar api/callbacks.rs (Callback handlers) | `TODO` | - | - | - | - | 4.2.2 |
| **4.3 - Builder Pattern** ||||||||
| 4.3.1 | Implementar ClientBuilder | `TODO` | - | - | - | - | 4.2.1 |
| 4.3.2 | Implementar configuração (bootstrap peers, data dir, etc) | `TODO` | - | - | - | - | 4.3.1 |
| **4.4 - Testes E2E** ||||||||
| 4.4.1 | Teste: send_text() funciona | `TODO` | - | - | - | - | 4.2.1 |
| 4.4.2 | Teste: receive message event funciona | `TODO` | - | - | - | - | 4.2.3 |

**Entregáveis:**
- ✅ API pública Client definida
- ✅ Protobuf messages funcionando
- ✅ Event system emitindo eventos

**Arquivos:** `protocol/`, `api/client.rs`, `api/events.rs`
**LoC:** ~1.500

---

## 🔗 FASE 5: CORE LIBRARY - FFI (UniFFI) (Mês 3)

### Objetivo
Bindings Rust → Kotlin/Swift para uso nos apps mobile/desktop.

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **5.1 - UniFFI Setup** ||||||||
| 5.1.1 | Criar ffi/mepassa.udl (interface definition) | `TODO` | - | - | - | - | 4.2.3 |
| 5.1.2 | Implementar ffi/types.rs (FFI-safe types) | `TODO` | - | - | - | - | 5.1.1 |
| 5.1.3 | Setup build.rs (uniffi-bindgen) | `TODO` | - | - | - | - | 5.1.1 |
| **5.2 - Bindings Kotlin** ||||||||
| 5.2.1 | Gerar bindings Kotlin (uniffi-bindgen) | `TODO` | - | - | - | - | 5.1.3 |
| 5.2.2 | Testar chamada de Kotlin → Rust (sample) | `TODO` | - | - | - | - | 5.2.1 |
| **5.3 - Bindings Swift** ||||||||
| 5.3.1 | Gerar bindings Swift (uniffi-bindgen) | `TODO` | - | - | - | - | 5.1.3 |
| 5.3.2 | Testar chamada de Swift → Rust (sample) | `TODO` | - | - | - | - | 5.3.1 |
| **5.4 - Build Artifacts** ||||||||
| 5.4.1 | Build libmepassa_core.so (Android ARM64) | `TODO` | - | - | - | - | 5.2.2 |
| 5.4.2 | Build libmepassa_core.dylib (iOS ARM64) | `TODO` | - | - | - | - | 5.3.2 |
| 5.4.3 | Build mepassa_core.dll (Windows x64) | `TODO` | - | - | - | - | 5.1.3 |

**Entregáveis:**
- ✅ Bindings Kotlin gerados
- ✅ Bindings Swift gerados
- ✅ Libs nativas compiladas (.so, .dylib, .dll)

**Arquivos:** `ffi/mepassa.udl`, `ffi/types.rs`, `build.rs`
**LoC:** ~800

---

## 📱 FASE 6: ANDROID APP - SETUP & UI BÁSICO (Mês 3-4)

### Objetivo
App Android funcional com UI mínima (login, lista de conversas, chat).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **6.1 - Setup Projeto** ||||||||
| 6.1.1 | Criar android/ (Gradle project) | `TODO` | - | - | - | - | 0.2 |
| 6.1.2 | Setup Jetpack Compose (Material Design 3) | `TODO` | - | - | - | - | 6.1.1 |
| 6.1.3 | Setup Navigation Compose | `TODO` | - | - | - | - | 6.1.2 |
| 6.1.4 | Integrar libmepassa_core.so (FFI) | `TODO` | - | - | - | - | 5.4.1 |
| **6.2 - Telas Básicas** ||||||||
| 6.2.1 | Implementar OnboardingScreen (gerar keypair) | `TODO` | - | - | - | - | 6.1.3 |
| 6.2.2 | Implementar ConversationsScreen (lista) | `TODO` | - | - | - | - | 6.2.1 |
| 6.2.3 | Implementar ChatScreen (mensagens) | `TODO` | - | - | - | - | 6.2.2 |
| 6.2.4 | Implementar MessageInput (enviar texto) | `TODO` | - | - | - | - | 6.2.3 |
| **6.3 - Integração Core** ||||||||
| 6.3.1 | Criar MePassaService (background service) | `TODO` | - | - | - | - | 6.1.4 |
| 6.3.2 | Inicializar MePassaClient via FFI | `TODO` | - | - | - | - | 6.3.1 |
| 6.3.3 | Implementar send_message() | `TODO` | - | - | - | - | 6.3.2 |
| 6.3.4 | Implementar event listener (receive messages) | `TODO` | - | - | - | - | 6.3.2 |
| **6.4 - Storage & Crypto** ||||||||
| 6.4.1 | Salvar keypair no EncryptedSharedPreferences | `TODO` | - | - | - | - | 6.2.1 |
| 6.4.2 | Implementar Keystore integration | `TODO` | - | - | - | - | 6.4.1 |

**Entregáveis:**
- ✅ App Android abre
- ✅ Gera keypair no primeiro uso
- ✅ Envia mensagem de texto
- ✅ Recebe mensagem de texto
- ✅ UI funcional (não polida)

**Arquivos:** `MainActivity.kt`, `OnboardingScreen.kt`, `ConversationsScreen.kt`, `ChatScreen.kt`, `MePassaService.kt`
**LoC:** ~3.000

---

## 🖥️ FASE 7: DESKTOP APP - SETUP & UI BÁSICO (Mês 3-4)

### Objetivo
App Desktop (Tauri) com UI mínima (mesmo escopo que Android).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **7.1 - Setup Projeto** ||||||||
| 7.1.1 | Criar desktop/ (Tauri 2.0 project) | `TODO` | - | - | - | - | 0.2 |
| 7.1.2 | Setup React frontend (Vite + TypeScript) | `TODO` | - | - | - | - | 7.1.1 |
| 7.1.3 | Setup TailwindCSS | `TODO` | - | - | - | - | 7.1.2 |
| 7.1.4 | Integrar mepassa-core (Rust backend Tauri) | `TODO` | - | - | - | - | 4.3.2 |
| **7.2 - Telas Básicas** ||||||||
| 7.2.1 | Implementar OnboardingView (React) | `TODO` | - | - | - | - | 7.1.3 |
| 7.2.2 | Implementar ConversationsView | `TODO` | - | - | - | - | 7.2.1 |
| 7.2.3 | Implementar ChatView | `TODO` | - | - | - | - | 7.2.2 |
| 7.2.4 | Implementar MessageInput | `TODO` | - | - | - | - | 7.2.3 |
| **7.3 - Tauri Commands** ||||||||
| 7.3.1 | Implementar tauri command: init_client() | `TODO` | - | - | - | - | 7.1.4 |
| 7.3.2 | Implementar tauri command: send_message() | `TODO` | - | - | - | - | 7.3.1 |
| 7.3.3 | Implementar tauri event: message_received | `TODO` | - | - | - | - | 7.3.1 |
| **7.4 - Features Desktop** ||||||||
| 7.4.1 | Implementar tray icon | `TODO` | - | - | - | - | 7.1.1 |
| 7.4.2 | Implementar desktop notifications | `TODO` | - | - | - | - | 7.4.1 |

**Entregáveis:**
- ✅ App Desktop abre
- ✅ Envia/recebe mensagens
- ✅ Tray icon funciona
- ✅ Notificações desktop

**Arquivos:** `src-tauri/main.rs`, `src/OnboardingView.tsx`, `src/ConversationsView.tsx`, `src/ChatView.tsx`
**LoC:** ~2.500

---

## 🔔 FASE 8: PUSH NOTIFICATIONS (Mês 4)

### Objetivo
Notificações push para acordar app quando mensagem chega (Android FCM + iOS APNs).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **8.1 - Android FCM** ||||||||
| 8.1.1 | Setup FCM (Firebase Cloud Messaging) | `TODO` | - | - | - | - | 6.3.4 |
| 8.1.2 | Implementar FirebaseMessagingService | `TODO` | - | - | - | - | 8.1.1 |
| 8.1.3 | Enviar FCM token para servidor | `TODO` | - | - | - | - | 8.1.2 |
| 8.1.4 | Teste: notificação acorda app | `TODO` | - | - | - | - | 8.1.3 |
| **8.2 - iOS APNs** ||||||||
| 8.2.1 | Setup APNs (Apple Push Notification) | `TODO` | - | - | - | - | - |
| 8.2.2 | Implementar NotificationServiceExtension | `TODO` | - | - | - | - | 8.2.1 |
| 8.2.3 | Enviar APNs token para servidor | `TODO` | - | - | - | - | 8.2.2 |
| **8.3 - Push Server** ||||||||
| 8.3.1 | Implementar push notification server (Rust) | `TODO` | - | - | - | - | - |
| 8.3.2 | Integrar FCM SDK (reqwest HTTP) | `TODO` | - | - | - | - | 8.3.1 |
| 8.3.3 | Integrar APNs SDK (a2 crate) | `TODO` | - | - | - | - | 8.3.1 |

**Entregáveis:**
- ✅ Android: notificações funcionam
- ✅ iOS: notificações funcionam
- ✅ Server envia push quando mensagem offline

**Arquivos:** `FirebaseMessagingService.kt`, `server/push/main.rs`
**LoC:** ~1.000

---

## 🏗️ FASE 9: SERVER - BOOTSTRAP & DHT (Mês 4)

### Objetivo
Servidores bootstrap para peer discovery (DHT).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **9.1 - Bootstrap Node** ||||||||
| 9.1.1 | Criar server/bootstrap/ (Rust project) | `TODO` | - | - | - | - | 0.2 |
| 9.1.2 | Setup libp2p (DHT mode, Kademlia) | `TODO` | - | - | - | - | 9.1.1 |
| 9.1.3 | Implementar peer discovery handler | `TODO` | - | - | - | - | 9.1.2 |
| 9.1.4 | Implementar health check endpoint | `TODO` | - | - | - | - | 9.1.3 |
| **9.2 - Deploy** ||||||||
| 9.2.1 | Deploy bootstrap node 1 (Brasil - São Paulo) | `TODO` | - | - | - | - | 9.1.4 |
| 9.2.2 | Deploy bootstrap node 2 (US - Virginia) | `TODO` | - | - | - | - | 9.1.4 |
| 9.2.3 | Deploy bootstrap node 3 (EU - Frankfurt) | `TODO` | - | - | - | - | 9.1.4 |
| **9.3 - Monitoramento** ||||||||
| 9.3.1 | Setup Prometheus metrics (básico) | `TODO` | - | - | - | - | 9.2.3 |
| 9.3.2 | Dashboard básico (Grafana) | `TODO` | - | - | - | - | 9.3.1 |

**Entregáveis:**
- ✅ 3 bootstrap nodes online
- ✅ Clients descobrem peers via DHT
- ✅ Monitoramento básico

**Arquivos:** `server/bootstrap/main.rs`
**LoC:** ~800

---

## 🔄 FASE 10: SERVER - TURN RELAY (Mês 4)

### Objetivo
TURN relay para fallback quando P2P direto falha (NAT simétrico).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **10.1 - TURN Server** ||||||||
| 10.1.1 | Setup coturn (Docker container) | `TODO` | - | - | - | - | - |
| 10.1.2 | Configurar credentials (TURN authentication) | `TODO` | - | - | - | - | 10.1.1 |
| 10.1.3 | Deploy TURN server (Brasil) | `TODO` | - | - | - | - | 10.1.2 |
| **10.2 - Client Integration** ||||||||
| 10.2.1 | Core: Adicionar TURN config (endpoint + credentials) | `TODO` | - | - | - | - | 10.1.3 |
| 10.2.2 | Core: Fallback automático para TURN | `TODO` | - | - | - | - | 10.2.1 |
| 10.2.3 | Core: Detectar NAT simétrico (STUN test) | `TODO` | - | - | - | - | 10.2.1 |
| **10.3 - Testes** ||||||||
| 10.3.1 | Teste: NAT simétrico usa TURN | `TODO` | - | - | - | - | 10.2.2 |
| 10.3.2 | Teste: mensagem via TURN funciona | `TODO` | - | - | - | - | 10.3.1 |

**Entregáveis:**
- ✅ TURN relay online
- ✅ Client detecta quando precisa relay
- ✅ Fallback automático funciona
- ✅ 100% usuários conseguem conectar

**Arquivos:** `server/turn/docker-compose.yml`, `network/relay.rs`
**LoC:** ~600

---

## 💾 FASE 11: SERVER - MESSAGE STORE (Store & Forward) (Mês 4)

### Objetivo
Armazenamento offline de mensagens (PostgreSQL + Redis).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **11.1 - Database Setup** ||||||||
| 11.1.1 | Setup PostgreSQL (Docker container) | `TODO` | - | - | - | - | - |
| 11.1.2 | Criar schema (offline_messages table + indexes) | `TODO` | - | - | - | - | 11.1.1 |
| 11.1.3 | Setup Redis (presence + message queue) | `TODO` | - | - | - | - | - |
| **11.2 - Server Implementation** ||||||||
| 11.2.1 | Criar server/store/ (Rust project - Actix Web) | `TODO` | - | - | - | - | 0.2 |
| 11.2.2 | Implementar POST /store (salvar mensagem encrypted) | `TODO` | - | - | - | - | 11.1.2 |
| 11.2.3 | Implementar GET /store (buscar mensagens pendentes) | `TODO` | - | - | - | - | 11.2.2 |
| 11.2.4 | Implementar DELETE /store (confirmar entrega) | `TODO` | - | - | - | - | 11.2.3 |
| 11.2.5 | Implementar TTL job (deletar após 14 dias) | `TODO` | - | - | - | - | 11.2.2 |
| **11.3 - Client Integration** ||||||||
| 11.3.1 | Core: Detectar destinatário offline (DHT lookup fail) | `TODO` | - | - | - | - | 2.3.1 |
| 11.3.2 | Core: Enviar para Message Store via HTTP | `TODO` | - | - | - | - | 11.3.1 |
| 11.3.3 | Core: Poll store ao ficar online (GET /store) | `TODO` | - | - | - | - | 11.3.2 |
| 11.3.4 | Core: ACK após receber mensagens (DELETE /store) | `TODO` | - | - | - | - | 11.3.3 |
| **11.4 - Testes** ||||||||
| 11.4.1 | Teste: mensagem offline salva no DB encrypted | `TODO` | - | - | - | - | 11.3.2 |
| 11.4.2 | Teste: mensagem entregue ao ficar online | `TODO` | - | - | - | - | 11.3.4 |
| 11.4.3 | Teste: TTL deleta após 14 dias | `TODO` | - | - | - | - | 11.2.5 |

**Entregáveis:**
- ✅ Message Store funcionando
- ✅ Mensagem offline salva encrypted
- ✅ Entrega ao ficar online
- ✅ Auto-delete após entrega ou 14 dias

**Arquivos:** `server/store/main.rs`, `server/store/db.rs`, `server/store/api.rs`
**LoC:** ~1.500

---

## 📞 FASE 12: VOIP - CHAMADAS DE VOZ (Mês 4) 🔥 **PRIORIDADE MÁXIMA**

### Objetivo
Chamadas de voz 1:1 funcionando (P2P + TURN fallback).

**CRÍTICO:** Sem isso, ninguém adota. É deal-breaker. 87% dos brasileiros usam WhatsApp para chamadas.

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **12.1 - Core WebRTC** ||||||||
| 12.1.1 | Implementar voip/ module (Rust) | `TODO` | - | - | - | - | 2.3.3 |
| 12.1.2 | Setup WebRTC (webrtc-rs crate) | `TODO` | - | - | - | - | 12.1.1 |
| 12.1.3 | Implementar signaling via libp2p | `TODO` | - | - | - | - | 12.1.2 |
| 12.1.4 | Implementar ICE candidate exchange | `TODO` | - | - | - | - | 12.1.3 |
| 12.1.5 | Implementar SDP offer/answer | `TODO` | - | - | - | - | 12.1.4 |
| **12.2 - Audio Codec & Quality** ||||||||
| 12.2.1 | Integrar Opus codec (libopus) | `TODO` | - | - | - | - | 12.1.5 |
| 12.2.2 | Implementar echo cancellation | `TODO` | - | - | - | - | 12.2.1 |
| 12.2.3 | Implementar noise suppression | `TODO` | - | - | - | - | 12.2.1 |
| 12.2.4 | Implementar adaptive bitrate (6-128kbps) | `TODO` | - | - | - | - | 12.2.1 |
| **12.3 - Android UI** ||||||||
| 12.3.1 | Implementar CallScreen (Compose) | `TODO` | - | - | - | - | 6.2.4 |
| 12.3.2 | Implementar IncomingCallScreen (fullscreen) | `TODO` | - | - | - | - | 12.3.1 |
| 12.3.3 | Botões: atender/recusar/desligar/mute | `TODO` | - | - | - | - | 12.3.1 |
| 12.3.4 | Implementar timer de duração | `TODO` | - | - | - | - | 12.3.1 |
| 12.3.5 | Implementar fullscreen notification (incoming) | `TODO` | - | - | - | - | 12.3.2 |
| **12.4 - Desktop UI** ||||||||
| 12.4.1 | Implementar CallView (React) | `TODO` | - | - | - | - | 7.2.4 |
| 12.4.2 | Implementar IncomingCallModal | `TODO` | - | - | - | - | 12.4.1 |
| **12.5 - Background & Bluetooth** ||||||||
| 12.5.1 | Android: funciona em background (foreground service) | `TODO` | - | - | - | - | 12.3.5 |
| 12.5.2 | Android: funciona com Bluetooth (AudioManager) | `TODO` | - | - | - | - | 12.3.5 |
| 12.5.3 | Implementar histórico de chamadas (DB) | `TODO` | - | - | - | - | 12.3.4 |
| **12.6 - Testes Críticos** ||||||||
| 12.6.1 | Teste: chamada P2P direto funciona (latência ~50ms) | `TODO` | - | - | - | - | 12.2.4 |
| 12.6.2 | Teste: chamada via TURN funciona (latência ~200ms) | `TODO` | - | - | - | - | 10.3.2 |
| 12.6.3 | Teste: qualidade áudio >4.0/5.0 (MOS score) | `TODO` | - | - | - | - | 12.2.3 |
| 12.6.4 | Teste: <5% dropped calls | `TODO` | - | - | - | - | 12.2.4 |
| 12.6.5 | Teste comparativo lado-a-lado com WhatsApp | `TODO` | - | - | - | - | 12.6.4 |

**Entregáveis:**
- ✅ Chamadas de voz 1:1 funcionam
- ✅ P2P direto (latência ~50ms)
- ✅ TURN fallback (latência ~200ms)
- ✅ Qualidade comparável ao WhatsApp
- ✅ 100% beta testers conseguem fazer chamadas
- ✅ Funciona em background
- ✅ Funciona com Bluetooth

**TESTE DECISIVO (Milestone Crítico):**
Perguntar aos beta testers: **"Você usaria MePassa como seu chat principal?"**
- **Se < 50% SIM:** ⛔ PARA TUDO e conserta chamadas
- **Se 50-70% SIM:** ⚠️ Continua com cautela, iterar feedback
- **Se > 70% SIM:** 🚀 Continua full speed

**Arquivos:** `voip/webrtc.rs`, `CallScreen.kt`, `CallView.tsx`
**LoC:** ~2.500

---

## 🍎 FASE 13: iOS APP (Mês 5)

### Objetivo
App iOS com paridade de features (mensagens + chamadas).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **13.1 - Setup** ||||||||
| 13.1.1 | Criar ios/ (Xcode project) | `TODO` | - | - | - | - | 0.2 |
| 13.1.2 | Setup SwiftUI | `TODO` | - | - | - | - | 13.1.1 |
| 13.1.3 | Integrar libmepassa_core.dylib (FFI) | `TODO` | - | - | - | - | 5.4.2 |
| **13.2 - UI** ||||||||
| 13.2.1 | Implementar OnboardingView | `TODO` | - | - | - | - | 13.1.2 |
| 13.2.2 | Implementar ConversationsView | `TODO` | - | - | - | - | 13.2.1 |
| 13.2.3 | Implementar ChatView | `TODO` | - | - | - | - | 13.2.2 |
| 13.2.4 | Implementar MessageInput | `TODO` | - | - | - | - | 13.2.3 |
| **13.3 - Chamadas** ||||||||
| 13.3.1 | Setup CallKit (native iOS calls UI) | `TODO` | - | - | - | - | 12.6.1 |
| 13.3.2 | Implementar CallView | `TODO` | - | - | - | - | 13.3.1 |
| 13.3.3 | Implementar IncomingCallView | `TODO` | - | - | - | - | 13.3.1 |
| **13.4 - Background** ||||||||
| 13.4.1 | Setup Background Fetch | `TODO` | - | - | - | - | 13.2.4 |
| 13.4.2 | Integrar APNs (FASE 8) | `TODO` | - | - | - | - | 8.2.3 |
| **13.5 - Storage** ||||||||
| 13.5.1 | Salvar keypair no Keychain (secure) | `TODO` | - | - | - | - | 13.2.1 |
| 13.5.2 | Setup CoreData (opcional, cache) | `TODO` | - | - | - | - | 13.4.1 |
| **13.6 - TestFlight** ||||||||
| 13.6.1 | Setup provisioning profiles (Apple Developer) | `TODO` | - | - | - | - | 13.5.2 |
| 13.6.2 | Deploy TestFlight beta | `TODO` | - | - | - | - | 13.6.1 |

**Entregáveis:**
- ✅ App iOS funcional
- ✅ Mensagens + Chamadas
- ✅ CallKit integration
- ✅ TestFlight beta disponível

**Arquivos:** `OnboardingView.swift`, `ConversationsView.swift`, `ChatView.swift`, `CallView.swift`
**LoC:** ~4.000

---

## 📹 FASE 14: VIDEOCHAMADAS (Mês 5)

### Objetivo
Videochamadas 1:1 (extensão do VoIP).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **14.1 - Core** ||||||||
| 14.1.1 | Adicionar video track (WebRTC) | `TODO` | - | - | - | - | 12.1.5 |
| 14.1.2 | Implementar codec H264/VP8 | `TODO` | - | - | - | - | 14.1.1 |
| 14.1.3 | Implementar camera switching (front/back) | `TODO` | - | - | - | - | 14.1.1 |
| **14.2 - Android** ||||||||
| 14.2.1 | Implementar VideoCallScreen | `TODO` | - | - | - | - | 12.3.4 |
| 14.2.2 | Integrar CameraX | `TODO` | - | - | - | - | 14.2.1 |
| 14.2.3 | Botões: mute áudio/vídeo, flip camera | `TODO` | - | - | - | - | 14.2.1 |
| **14.3 - iOS** ||||||||
| 14.3.1 | Implementar VideoCallView | `TODO` | - | - | - | - | 13.3.2 |
| 14.3.2 | Integrar AVFoundation (camera) | `TODO` | - | - | - | - | 14.3.1 |
| **14.4 - Desktop** ||||||||
| 14.4.1 | Implementar VideoCallView (React) | `TODO` | - | - | - | - | 12.4.2 |
| 14.4.2 | Usar JavaScript WebRTC API (browser API) | `TODO` | - | - | - | - | 14.4.1 |

**Entregáveis:**
- ✅ Videochamadas 1:1 funcionam
- ✅ Android + iOS + Desktop
- ✅ Câmera front/back
- ✅ Mute áudio/vídeo

**Arquivos:** `VideoCallScreen.kt`, `VideoCallView.swift`, `VideoCallView.tsx`
**LoC:** ~1.800

---

## 👥 FASE 15: GRUPOS (Mês 6)

### Objetivo
Chat em grupo (até 256 pessoas) + chamadas em grupo (até 8 pessoas).

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **15.1 - Core: Group Chat** ||||||||
| 15.1.1 | Implementar GossipSub (libp2p pub/sub) | `TODO` | - | - | - | - | 2.2.3 |
| 15.1.2 | Implementar group management (create, invite, remove) | `TODO` | - | - | - | - | 15.1.1 |
| 15.1.3 | Implementar admin controls | `TODO` | - | - | - | - | 15.1.2 |
| 15.1.4 | Implementar Sender Keys (Signal Protocol groups) | `TODO` | - | - | - | - | 1.3.4 |
| **15.2 - UI: Groups** ||||||||
| 15.2.1 | Android: GroupChatScreen | `TODO` | - | - | - | - | 15.1.4 |
| 15.2.2 | iOS: GroupChatView | `TODO` | - | - | - | - | 15.1.4 |
| 15.2.3 | Desktop: GroupChatView | `TODO` | - | - | - | - | 15.1.4 |
| **15.3 - Group Calls (SFU)** ||||||||
| 15.3.1 | Deploy SFU server (mediasoup) | `TODO` | - | - | - | - | - |
| 15.3.2 | Core: Integrar com SFU (WebRTC multi-party) | `TODO` | - | - | - | - | 15.3.1 |
| 15.3.3 | UI: Group call (até 8 pessoas) | `TODO` | - | - | - | - | 15.3.2 |

**Entregáveis:**
- ✅ Grupos de até 256 pessoas
- ✅ Admin controls
- ✅ Chamadas em grupo (até 8)

**Arquivos:** `network/gossip.rs`, `GroupChatScreen.kt`, `GroupChatView.swift`
**LoC:** ~2.000

---

## 🖼️ FASE 16: MÍDIA & POLIMENTO (Mês 6)

### Objetivo
Envio de imagens, vídeos, arquivos e polimento geral da UI.

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **16.1 - Imagens** ||||||||
| 16.1.1 | Core: Upload/download de arquivos | `TODO` | - | - | - | - | 11.2.4 |
| 16.1.2 | Core: Compressão de imagens (JPEG/WebP) | `TODO` | - | - | - | - | 16.1.1 |
| 16.1.3 | Core: Thumbnails generation | `TODO` | - | - | - | - | 16.1.2 |
| 16.1.4 | Android: Image picker + preview | `TODO` | - | - | - | - | 16.1.3 |
| **16.2 - Vídeos** ||||||||
| 16.2.1 | Core: Upload/download de vídeos | `TODO` | - | - | - | - | 16.1.1 |
| 16.2.2 | Core: Compressão de vídeos (H264) | `TODO` | - | - | - | - | 16.2.1 |
| 16.2.3 | Android: Video player (ExoPlayer) | `TODO` | - | - | - | - | 16.2.2 |
| **16.3 - Arquivos** ||||||||
| 16.3.1 | Core: Upload/download arquivos (até 100MB) | `TODO` | - | - | - | - | 16.1.1 |
| 16.3.2 | Android: File picker | `TODO` | - | - | - | - | 16.3.1 |
| **16.4 - Mensagens de Voz** ||||||||
| 16.4.1 | Android: Record audio (MediaRecorder) | `TODO` | - | - | - | - | - |
| 16.4.2 | Core: Audio compression (Opus) | `TODO` | - | - | - | - | 16.4.1 |
| 16.4.3 | Android: Audio player (waveform UI) | `TODO` | - | - | - | - | 16.4.2 |
| **16.5 - Reactions & Edição** ||||||||
| 16.5.1 | Core: Reactions protocol (emoji) | `TODO` | - | - | - | - | 4.1.3 |
| 16.5.2 | Core: Edit message protocol | `TODO` | - | - | - | - | 16.5.1 |
| 16.5.3 | UI: Reactions UI (long press) | `TODO` | - | - | - | - | 16.5.1 |

**Entregáveis:**
- ✅ Envio de imagens
- ✅ Envio de vídeos
- ✅ Compartilhamento de arquivos
- ✅ Mensagens de voz
- ✅ Reactions
- ✅ Edição de mensagens

**Arquivos:** `media/upload.rs`, `ImagePicker.kt`, `AudioRecorder.kt`
**LoC:** ~2.500

---

## 🔄 FASE 17: MULTI-DEVICE SYNC (Mês 6)

### Objetivo
Sincronizar mensagens entre múltiplos devices do mesmo usuário.

### Tarefas

| # | Tarefa | Status | Responsável | Data Início | Data Fim | Última Atualização | Dependências |
|---|--------|--------|-------------|-------------|----------|--------------------|--------------|
| **17.1 - CRDTs** ||||||||
| 17.1.1 | Implementar sync/ module (Core) | `TODO` | - | - | - | - | 3.2.3 |
| 17.1.2 | Integrar Automerge (CRDTs library) | `TODO` | - | - | - | - | 17.1.1 |
| 17.1.3 | Implementar sync protocol (P2P) | `TODO` | - | - | - | - | 17.1.2 |
| **17.2 - Device Linking** ||||||||
| 17.2.1 | Implementar QR code linking (scan device) | `TODO` | - | - | - | - | 17.1.3 |
| 17.2.2 | Implementar device management (list devices) | `TODO` | - | - | - | - | 17.2.1 |
| 17.2.3 | Implementar device revoke (remove device) | `TODO` | - | - | - | - | 17.2.2 |
| **17.3 - Sync Server (Opcional)** ||||||||
| 17.3.1 | Implementar backup de CRDT state (encrypted) | `TODO` | - | - | - | - | 17.1.3 |
| 17.3.2 | Deploy sync server | `TODO` | - | - | - | - | 17.3.1 |

**Entregáveis:**
- ✅ Devices sincronizam via P2P
- ✅ QR code linking
- ✅ Device management
- ✅ Backup opcional de state

**Arquivos:** `sync/crdt.rs`, `sync/device.rs`
**LoC:** ~1.500

---

## ✅ VERIFICAÇÃO & VALIDAÇÃO FINAL

### Testes de Aceitação (MVP Mínimo - Mês 6)

**OBRIGATÓRIO para lançamento público:**
- [ ] Mensagens de texto 1:1 funcionam (100% entrega)
- [ ] Chamadas de voz 1:1 funcionam (qualidade >4.0/5.0 MOS)
- [ ] Notificações push funcionam
- [ ] Funciona offline (store-and-forward)
- [ ] Android + Desktop funcionam
- [ ] Grupos de texto (até 256)
- [ ] Envio de imagens funciona
- [ ] Histórico de conversas persiste
- [ ] NAT simétrico funciona (TURN fallback)
- [ ] < 5% taxa de bug crítico

**IMPORTANTE (mas pode vir depois do lançamento):**
- [ ] iOS app funciona
- [ ] Videochamadas funcionam
- [ ] Chamadas em grupo funcionam
- [ ] Mensagens de voz funcionam
- [ ] Compartilhamento de arquivos funciona

### Métricas de Sucesso (Mês 6)

| Métrica | Target | Status | Atual | Última Medição |
|---------|--------|--------|-------|----------------|
| Usuários ativos | 500+ | `TODO` | 0 | - |
| Empresas usando | 50+ | `TODO` | 0 | - |
| Retenção D7 | > 40% | `TODO` | - | - |
| NPS | > 70 | `TODO` | - | - |
| P2P direto | 70-85% | `TODO` | - | - |
| TURN relay | 10-20% | `TODO` | - | - |
| Store & forward | 3-10% | `TODO` | - | - |
| Comparação WhatsApp | "Tão bom quanto" | `TODO` | - | - |

### Teste Decisivo (Milestone Crítico - Mês 4)

**Após Fase 12 (Chamadas), perguntar aos beta testers:**
> "Você usaria MePassa como seu chat principal?"

**Critérios de Decisão:**
- **< 50% SIM:** ⛔ **PARA TUDO** e conserta chamadas (não avançar para iOS/grupos)
- **50-70% SIM:** ⚠️ Continua com cautela, iterar feedback
- **> 70% SIM:** 🚀 Continua full speed para iOS e features avançadas

**Análise do "Por quê NÃO":**
- Se "Chamadas ruins" → Prioridade máxima consertar (Fase 12)
- Se "Falta feature X" → Avaliar se é P0 antes de lançar
- Se "UI confusa" → Polimento UI (Fase 16)

---

## 📊 RESUMO DE ESTIMATIVAS

| Fase | Componente | Arquivos | LoC | Duração | Status |
|------|-----------|----------|-----|---------|--------|
| 0 | Setup & Fundação | 10 | 500 | 2 semanas | `TODO` |
| 1 | Core - Identidade & Crypto | 15 | 2.000 | 2 semanas | `TODO` |
| 1.5 | Identity Server & Username | 12 | 1.500 | 1 semana | `TODO` |
| 2 | Core - Networking P2P | 8 | 1.500 | 1 semana | `TODO` |
| 3 | Core - Storage Local | 8 | 1.200 | 1 semana | `TODO` |
| 4 | Core - Protocolo & API | 10 | 1.500 | 1 semana | `TODO` |
| 5 | Core - FFI (UniFFI) | 5 | 800 | 1 semana | `TODO` |
| 6 | Android MVP | 25 | 3.000 | 2 semanas | `TODO` |
| 7 | Desktop MVP | 20 | 2.500 | 2 semanas | `TODO` |
| 8 | Push Notifications | 8 | 1.000 | 1 semana | `TODO` |
| 9 | Server - Bootstrap & DHT | 6 | 800 | 1 semana | `TODO` |
| 10 | Server - TURN Relay | 5 | 600 | 1 semana | `TODO` |
| 11 | Server - Message Store | 10 | 1.500 | 1 semana | `TODO` |
| **12** | **VOIP - Chamadas** 🔥 | **15** | **2.500** | **3 semanas** | `TODO` |
| 13 | iOS App | 30 | 4.000 | 3 semanas | `TODO` |
| 14 | Videochamadas | 12 | 1.800 | 1 semana | `TODO` |
| 15 | Grupos | 15 | 2.000 | 2 semanas | `TODO` |
| 16 | Mídia & Polimento | 20 | 2.500 | 2 semanas | `TODO` |
| 17 | Multi-Device Sync | 10 | 1.500 | 1 semana | `TODO` |
| **TOTAL** | **Todos** | **~244** | **~32.700** | **~27 semanas** | **0%** |

**Estimativa:** ~6 meses (considerando 1 dev full-time + 2-3 devs part-time + comunidade)

---

## 🚨 DECISÕES CRÍTICAS & GATES

### Gate 1: Mês 2 (Após Fase 1-5 Core)
**Pergunta:** Core library funciona? (Alice → Bob encrypted message)
- **SIM:** Avança para apps (Fase 6-7)
- **NÃO:** Conserta core primeiro

### Gate 2: Mês 3 (Após Fase 6-7 Apps MVP)
**Pergunta:** 10 beta testers conseguem trocar mensagens?
- **SIM:** Avança para infraestrutura (Fase 8-11)
- **NÃO:** Conserta apps primeiro

### Gate 3: Mês 4 (Após Fase 12 VOIP) 🔥 **CRÍTICO**
**Pergunta:** "Você usaria MePassa como chat principal?"
- **> 70% SIM:** 🚀 Avança para iOS (Fase 13)
- **50-70% SIM:** ⚠️ Iterar feedback, considerar delay iOS
- **< 50% SIM:** ⛔ **PARA TUDO**, conserta chamadas

### Gate 4: Mês 6 (Lançamento Público)
**Pergunta:** MVP completo atende critérios mínimos?
- **SIM:** Lança público (F-Droid, Play Store)
- **NÃO:** Mais 2 meses de beta privado

---

## 📁 ESTRUTURA FINAL DO REPOSITÓRIO

```
mepassa/
├── .github/workflows/          # CI/CD
├── core/                       # Rust library (mepassa-core)
│   ├── src/
│   │   ├── identity/           # Keypairs
│   │   ├── crypto/             # Signal Protocol
│   │   ├── network/            # libp2p P2P
│   │   ├── storage/            # SQLite
│   │   ├── sync/               # CRDTs
│   │   ├── voip/               # WebRTC
│   │   ├── protocol/           # Protobuf
│   │   ├── api/                # Client API
│   │   └── ffi/                # UniFFI
│   └── Cargo.toml
├── android/                    # Kotlin + Compose
│   └── app/src/main/kotlin/
├── ios/                        # Swift + SwiftUI
│   └── MePassa/
├── desktop/                    # Tauri 2.0
│   ├── src-tauri/              # Rust backend
│   └── src/                    # React frontend
├── server/
│   ├── bootstrap/              # DHT nodes
│   ├── store/                  # Message store
│   └── push/                   # Push notifications
├── proto/                      # Protobuf
├── docs/                       # Documentation
├── scripts/                    # Build/deploy
└── README.md
```

---

## 🎯 PRÓXIMOS PASSOS IMEDIATOS (SEMANA 1-2)

| # | Ação | Responsável | Prazo | Status |
|---|------|-------------|-------|--------|
| 1 | Criar organização GitHub (integralltech/mepassa) | - | - | `TODO` |
| 2 | Setup monorepo (estrutura completa) | - | - | `TODO` |
| 3 | Configurar CI/CD (GitHub Actions básico) | - | - | `TODO` |
| 4 | Registrar domínio mepassa.app | - | - | `TODO` |
| 5 | Criar landing page (captação beta testers) | - | - | `TODO` |
| 6 | Documentar arquitetura híbrida (docs/) | - | - | `TODO` |
| 7 | Setup Discord/Matrix comunidade | - | - | `TODO` |
| 8 | Recrutar 50-100 beta testers | - | - | `TODO` |

---

**FILOSOFIA DO PROJETO:**

> "Não adianta ter privacidade perfeita se ninguém usar.
> MePassa escolhe privacidade boa o suficiente + UX boa o suficiente = Adoção real."

**PRIORIDADES:**
1. **Funciona sempre** (como WhatsApp) ← Tabela stakes
2. **Chamadas de voz** (deal-breaker) ← Prioridade máxima
3. **80% P2P direto** (privacidade + economia) ← Diferencial
4. **Self-hosting** (compliance LGPD) ← B2B enabler

---

**FIM DO DOCUMENTO DE EXECUÇÃO v1**

*Criado: 2025-01-19*
*Última atualização: 2025-01-19*
