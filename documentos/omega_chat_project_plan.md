# Projeto Ômega Chat: Plataforma P2P Open Source

> **Tagline:** "Comunicação verdadeiramente descentralizada, privada e sem censura"

## 🎯 Visão do Projeto

**Ômega Chat** é uma plataforma de mensagens peer-to-peer open source que combina:
- **Privacidade máxima** (sem servidores centrais vendo conteúdo)
- **Confiabilidade** (mensagens offline via store-and-forward)
- **Multi-plataforma** (Android, iOS, Linux, macOS, Windows)
- **Extensível** (APIs para bots, agents IA, integrações)

### Diferencial competitivo

| Recurso | WhatsApp | Telegram | Signal | Matrix | Session | **Ômega** |
|---------|----------|----------|--------|--------|---------|-----------|
| E2E por padrão | ✅ | ❌ | ✅ | ⚠️ | ✅ | ✅ |
| Sem telefone | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| P2P direto | ❌ | ❌ | ❌ | ❌ | ⚠️ | ✅ |
| Mensagens offline | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Self-hostable | ❌ | ❌ | ❌ | ✅ | ⚠️ | ✅ |
| Multi-device | ✅ | ✅ | ✅ | ✅ | ⚠️ | ✅ |
| Open source | ❌ | ⚠️ | ✅ | ✅ | ✅ | ✅ |
| APIs para bots | ✅ | ✅ | ❌ | ✅ | ❌ | ✅ |

### Casos de uso

**Fase 1 - Comunidade:**
- Ativistas e jornalistas em regimes autoritários
- Profissionais que valorizam privacidade
- Comunidades tech/crypto/privacy-focused
- Usuários querendo escapar de Big Tech

**Fase 2 - VendaX.ai Integration:**
- Canal de comunicação vendedor-cliente com privacidade
- Agents IA como peers na rede (atendimento automático)
- Histórico descentralizado (compliance LGPD)
- Integrações com ERPs via bots

---

## 🏗️ Arquitetura Técnica

### Visão geral

```
┌─────────────────────────────────────────────────────────────┐
│                    ÔMEGA ECOSYSTEM                           │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Android    │  │     iOS      │  │   Desktop    │      │
│  │   (Kotlin)   │  │   (Swift)    │  │   (Tauri)    │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                 │                 │               │
│         └─────────────────┼─────────────────┘               │
│                           │                                 │
│                  ┌────────▼────────┐                        │
│                  │  Ômega Core     │                        │
│                  │  (Rust Library) │                        │
│                  │  ┌────────────┐ │                        │
│                  │  │ libp2p     │ │ Networking             │
│                  │  │ Signal     │ │ E2E Crypto             │
│                  │  │ SQLite     │ │ Local Storage          │
│                  │  │ CRDTs      │ │ Sync                   │
│                  │  └────────────┘ │                        │
│                  └─────────────────┘                        │
│                           │                                 │
│         ┌─────────────────┼─────────────────┐               │
│         │                 │                 │               │
│  ┌──────▼───────┐  ┌──────▼───────┐  ┌──────▼────────┐    │
│  │  Discovery   │  │Message Store │  │ Relay Server  │    │
│  │   (DHT)      │  │ (14d TTL)    │  │  (Fallback)   │    │
│  └──────────────┘  └──────────────┘  └───────────────┘    │
│                                                              │
│  Operado por: Comunidade / Self-hosted / Cloud opcional     │
└─────────────────────────────────────────────────────────────┘
```

### Componentes principais

#### 1. **omega-core** (Rust Library)

**Responsabilidades:**
- Gerenciamento de identidade (Ed25519 keypairs)
- Networking P2P via libp2p
- Criptografia E2E (Signal Protocol)
- Armazenamento local (SQLite + CRDTs)
- Sincronização multi-device
- API C FFI para bindings nativos

**Módulos:**
```
omega-core/
├── crypto/          # Signal Protocol, key management
├── network/         # libp2p, NAT traversal, DHT
├── storage/         # SQLite, migrations, queries
├── sync/            # CRDTs, multi-device
├── protocol/        # Message formats, serialization
├── identity/        # Keypairs, verification
└── ffi/             # C bindings para Swift/Kotlin
```

**Tecnologias:**
- `rust-libp2p` - Networking P2P
- `libsignal-client` - E2E encryption
- `rusqlite` - Storage local
- `automerge` - CRDTs para sync
- `uniffi` - FFI bindings
- `tokio` - Async runtime

#### 2. **omega-android** (Kotlin/Jetpack Compose)

**Features:**
- UI nativa com Material Design 3
- Notificações push via UnifiedPush
- Background service para manter conexões
- Camera, galeria, compartilhamento de arquivos
- Integração com contatos (opcional)

**Stack:**
- Jetpack Compose (UI)
- Kotlin Coroutines (async)
- Room (cache local complementar)
- WorkManager (background tasks)
- CameraX (camera)

#### 3. **omega-ios** (Swift/SwiftUI)

**Features:**
- UI nativa com SwiftUI
- NotificationServiceExtension para push
- Background fetch para sync
- PhotoKit integration
- Keychain para chaves criptográficas

**Stack:**
- SwiftUI (UI)
- Combine (reactive)
- CoreData (cache opcional)
- Network.framework (conectividade)

**Desafios iOS:**
- Background connections limitadas
- Push notifications obrigatório para wake-up
- Relay server essencial para confiabilidade

#### 4. **omega-desktop** (Tauri 2.0)

**Features:**
- UI web responsiva (React/Vue/Svelte)
- Tray icon com notificações
- Auto-updates
- Deep linking
- Multi-account support

**Stack:**
- Tauri 2.0 (Rust backend + web frontend)
- React/TypeScript (UI)
- Vite (build)
- TailwindCSS (styling)

**Vantagens:**
- Binário pequeno (~10-15MB)
- Compartilha omega-core diretamente
- Cross-platform nativo (Linux/macOS/Windows)

#### 5. **omega-relay** (Servidor Relay/TURN)

**Responsabilidades:**
- TURN server para NAT traversal
- WebSocket relay para fallback
- Não descriptografa conteúdo
- Logs apenas metadados (IPs, bandwidth)

**Stack:**
- `coturn` (TURN/STUN server) ou
- Implementação custom em Rust com `libp2p-relay`
- Nginx para load balancing
- Prometheus para metrics

#### 6. **omega-store** (Message Store)

**Responsabilidades:**
- Store-and-forward para mensagens offline
- TTL de 14 dias configurável
- Sharding por recipient hash
- Replicação entre nós (opcional)

**Stack:**
- Redis (in-memory storage)
- PostgreSQL (persistence)
- S3-compatible storage (arquivos grandes)
- gRPC API

**Schema simplificado:**
```sql
CREATE TABLE messages (
    message_id UUID PRIMARY KEY,
    recipient_hash VARCHAR(64) NOT NULL,
    encrypted_payload BYTEA NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    ttl_seconds INTEGER DEFAULT 1209600, -- 14 dias
    delivered BOOLEAN DEFAULT FALSE,
    INDEX idx_recipient_ttl (recipient_hash, created_at)
);
```

#### 7. **omega-discovery** (DHT/Bootstrap)

**Responsabilidades:**
- Kademlia DHT para peer discovery
- Bootstrap nodes para novos peers
- Peer routing information
- Health checks

**Stack:**
- `rust-libp2p` DHT implementation
- Múltiplos bootstrap nodes geograficamente distribuídos
- Pode rodar em VPS de $5/mês

---

## 📋 Especificação de Protocolos

### Protocol Buffer Messages

```protobuf
// omega-protocol/messages.proto

syntax = "proto3";
package omega.protocol;

// Tipos de mensagem
enum MessageType {
    TEXT = 0;
    IMAGE = 1;
    VIDEO = 2;
    AUDIO = 3;
    FILE = 4;
    LOCATION = 5;
    CONTACT = 6;
    STICKER = 7;
    REACTION = 8;
    EDIT = 9;
    DELETE = 10;
}

// Mensagem base
message Message {
    string message_id = 1;        // UUID
    string sender_id = 2;         // Public key hash
    string recipient_id = 3;      // Public key hash ou group_id
    MessageType type = 4;
    int64 timestamp = 5;          // Unix timestamp
    bytes encrypted_content = 6;  // E2E encrypted payload
    bytes signature = 7;          // Ed25519 signature
    
    // Metadados opcionais
    optional string reply_to = 8;
    optional int32 ttl_days = 9;
    repeated string mentions = 10;
}

// Conteúdo descriptografado (nunca trafega na rede)
message MessageContent {
    oneof content {
        TextContent text = 1;
        MediaContent media = 2;
        LocationContent location = 3;
        ContactContent contact = 4;
    }
}

message TextContent {
    string text = 1;
    repeated TextEntity entities = 2; // Menções, links, formatação
}

message MediaContent {
    string mime_type = 1;
    int64 size = 2;
    bytes thumbnail = 3;           // Thumbnail pequeno
    string file_hash = 4;          // Para deduplicação
    optional int32 width = 5;
    optional int32 height = 6;
    optional int32 duration = 7;   // Para audio/video
}

// Grupo
message Group {
    string group_id = 1;
    string name = 2;
    bytes avatar_hash = 3;
    repeated Member members = 4;
    GroupPermissions permissions = 5;
    int64 created_at = 6;
}

message Member {
    string user_id = 1;
    MemberRole role = 2;
    int64 joined_at = 3;
}

enum MemberRole {
    MEMBER = 0;
    ADMIN = 1;
    OWNER = 2;
}
```

### Fluxo de mensagens

#### 1:1 Chat (P2P direto)

```
[Alice]                                              [Bob]
   │                                                    │
   ├─ 1. Gera Message ID (UUID)                        │
   ├─ 2. Encrypt content (Signal Protocol)             │
   ├─ 3. Sign message (Ed25519)                        │
   │                                                    │
   ├─ 4. Lookup Bob no DHT ────────────────────────────▶│
   │    (Discovery Server responde endereço)            │
   │                                                    │
   ├─ 5. Establish libp2p connection ─────────────────▶│
   │    (NAT traversal via STUN/TURN)                   │
   │                                                    │
   ├─ 6. Send Message protobuf ───────────────────────▶│
   │                                                    │
   │                                    ┌── Verify signature
   │                                    ├── Decrypt content
   │                                    ├── Save to SQLite
   │                                    └── Display UI
   │                                                    │
   │◀─ 7. ACK (message_id + timestamp) ─────────────────┤
   │                                                    │
   └── Mark as delivered                                │
```

#### Offline message (via Store)

```
[Alice]           [Message Store]                  [Bob]
   │                    │                             │
   ├─ 1. Try P2P ───────┼─────────────────────────────▶X (offline)
   │    (fails)          │                             │
   │                    │                             │
   ├─ 2. Send to Store─▶│                             │
   │    POST /store     │                             │
   │    {               │                             │
   │      recipient: hash(Bob.pubkey),                │
   │      payload: encrypted_msg,                     │
   │      ttl: 14d      │                             │
   │    }               │                             │
   │                    ├─ Save to Redis/PostgreSQL   │
   │                    ├─ Set TTL timer              │
   │                    │                             │
   │   [Later... Bob comes online]                    │
   │                    │                             │
   │                    │   ◀─ 3. Poll for messages ──┤
   │                    │      GET /store?recipient=hash
   │                    │                             │
   │                    ├─ 4. Return messages ───────▶│
   │                    │    [encrypted_msg1, msg2]   │
   │                    │                             │
   │                    │   ◀─ 5. ACK received ───────┤
   │                    │      DELETE /store/{ids}    │
   │                    │                             │
   │                    ├─ Delete from storage        │
```

#### Group chat (GossipSub)

```
                    [Group: "Dev Team"]
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
      [Alice]           [Bob]            [Carol]
         │                 │                 │
         ├─ 1. Post message to topic         │
         │    Topic: /omega/group/{group_id} │
         │                 │                 │
         ├────────────────▶│                 │
         │                 ├─ Forward ──────▶│
         │                 │                 │
         ├─────────────────┼────────────────▶│
         │                 │                 │
         │    [All peers receive & decrypt]  │
         │    (Sender Keys para eficiência)  │
```

---

## 🔐 Modelo de Segurança

### Identidade e chaves

**Keypairs por usuário:**
- **Identity Key** (Ed25519): Identidade de longo prazo, ~32 bytes
- **Signed Prekey** (Curve25519): Rotacionado mensalmente
- **One-time Prekeys**: Pool de ~100 chaves, consumidas por sessão

**Storage seguro:**
- Android: EncryptedSharedPreferences + Keystore
- iOS: Keychain com kSecAttrAccessibleWhenUnlockedThisDeviceOnly
- Desktop: OS keyring (libsecret/Keychain/Credential Manager)

### Signal Protocol flow

```
[Alice first message to Bob]

1. Alice fetches Bob's key bundle:
   - Identity Key
   - Signed Prekey
   - One-time Prekey (consumido)

2. Alice executa X3DH:
   DH1 = DH(IK_alice, SPK_bob)
   DH2 = DH(EK_alice, IK_bob)
   DH3 = DH(EK_alice, SPK_bob)
   DH4 = DH(EK_alice, OPK_bob)
   
   SK = KDF(DH1 || DH2 || DH3 || DH4)

3. Alice inicia Double Ratchet com SK

4. Para cada mensagem:
   - Chain key ratcheting
   - Message key derivation
   - Encrypt AES-256-GCM
   - Authenticate HMAC-SHA256

5. Bob recebe, executa ratchet simétrico, decrypta
```

### Proteção de metadados

**O que é protegido:**
- ✅ Conteúdo das mensagens (E2E encrypted sempre)
- ✅ Arquivos enviados (E2E encrypted)
- ✅ Identidade real (não precisa telefone/email)
- ✅ Lista de contatos (armazenada localmente)

**O que NÃO é protegido:**
- ⚠️ Timing de mensagens (quando enviou)
- ⚠️ Tamanho aproximado de mensagem
- ⚠️ IPs dos peers (visível ao relay)
- ⚠️ Padrão de comunicação (quem fala com quem via Discovery)

**Mitigações futuras (roadmap):**
- Onion routing (tipo Session/Tor)
- Padding de mensagens
- Mix networks
- Bloom filters para queries no DHT

---

## 🚀 Roadmap de Desenvolvimento

### Fase 0: Setup & Foundation (Mês 1-2)

**Objetivos:**
- Estrutura de repositórios
- CI/CD básico
- Documentação inicial
- Core team definido

**Entregáveis:**
```
omega/
├── omega-core/        # Rust library (v0.1.0)
├── omega-android/     # Kotlin app (skeleton)
├── omega-ios/         # Swift app (skeleton)
├── omega-desktop/     # Tauri app (skeleton)
├── omega-relay/       # Relay server (basic)
├── omega-store/       # Message store (basic)
├── omega-discovery/   # DHT bootstrap (basic)
├── docs/             # Architecture, protocols, API
└── scripts/          # Build, test, deploy
```

**Tecnologia:**
- GitHub Actions (CI)
- Docker Compose (dev environment)
- GitBook ou Docusaurus (docs)
- Discord (comunidade)

### Fase 1: Core Library MVP (Mês 2-4)

**omega-core v0.1.0:**
- [x] Identity management (keypair generation)
- [x] libp2p integration (transporte TCP/QUIC)
- [x] Basic DHT peer discovery
- [x] Signal Protocol E2E (1:1 apenas)
- [x] SQLite storage (mensagens, contatos)
- [x] FFI bindings (C headers)
- [ ] Unit tests (>80% coverage)

**Infrastructure:**
- [ ] 3 bootstrap nodes (US/EU/ASIA)
- [ ] 1 relay server público
- [ ] 1 message store público

**Testes:**
- CLI tool para enviar mensagens P2P
- Testes automatizados de NAT traversal
- Load testing (1k peers simultâneos)

### Fase 2: Android App Alpha (Mês 4-6)

**omega-android v0.2.0:**
- [ ] Login/registro (keypair creation)
- [ ] Lista de conversas
- [ ] Chat 1:1 (text apenas)
- [ ] Envio de imagens
- [ ] Notificações push (UnifiedPush)
- [ ] Background service
- [ ] Settings básicos

**Features:**
- Material Design 3
- Dark mode
- Backup/restore de chaves
- QR code para adicionar contatos

**Release:**
- F-Droid (priority)
- Google Play Beta (opcional)

### Fase 3: Desktop App Alpha (Mês 5-7)

**omega-desktop v0.2.0:**
- [ ] Interface similar ao Telegram/Signal
- [ ] Chat 1:1 e grupos
- [ ] Envio de arquivos (drag & drop)
- [ ] Notificações desktop
- [ ] Multi-account (opcional)

**Plataformas:**
- Linux (AppImage + .deb)
- macOS (DMG, não assinado)
- Windows (MSI)

### Fase 4: iOS App Alpha (Mês 7-9)

**omega-ios v0.2.0:**
- [ ] Mesmas features do Android
- [ ] TestFlight beta
- [ ] NotificationServiceExtension
- [ ] Background fetch configurado
- [ ] Keychain integration

**Desafios esperados:**
- Apple Developer account ($99/ano)
- Code signing
- Background limitations
- App Store review (se for publicar)

### Fase 5: Group Chat & Media (Mês 8-10)

**Features:**
- [ ] Grupos de até 256 pessoas
- [ ] Admin controls
- [ ] Sender Keys (Signal Protocol groups)
- [ ] Voice messages
- [ ] Video messages
- [ ] File sharing (até 100MB)

**Optimizações:**
- Compression de mídia
- Thumbnails
- Progressive upload/download

### Fase 6: Multi-Device Sync (Mês 10-12)

**Features:**
- [ ] Link device via QR code
- [ ] CRDT sync entre devices
- [ ] Histórico completo sincronizado
- [ ] Device management (revoke)

**Tecnologia:**
- Automerge CRDTs
- Sync protocol via libp2p

### Fase 7: Advanced Features (Mês 12-18)

**Features avançadas:**
- [ ] Voice calls (WebRTC)
- [ ] Video calls (WebRTC)
- [ ] Screen sharing (desktop)
- [ ] Reactions e edição de mensagens
- [ ] Message search
- [ ] Archived chats
- [ ] Disappearing messages
- [ ] Backups criptografados

### Fase 8: Bot API & VendaX.ai Integration (Mês 18-24)

**Bot API:**
```rust
// omega-bot-sdk exemplo
use omega_bot::{Bot, Message, Context};

#[tokio::main]
async fn main() {
    let bot = Bot::new("bot_identity_key")
        .on_message(|ctx: Context, msg: Message| async move {
            if msg.text.starts_with("/help") {
                ctx.reply("Commands: /help, /status, /ping").await?;
            }
            Ok(())
        })
        .build();
    
    bot.run().await;
}
```

**VendaX.ai integration:**
- Agents IA como bots na rede
- Webhook para integrações ERP
- Analytics dashboard (metadados apenas)
- Self-hosted relay para clientes enterprise

---

## 💰 Modelo de Negócio (Opcional)

### Open Source Core + Serviços Pagos

**Sempre gratuito:**
- Código completo (AGPL v3)
- Apps (Android/iOS/Desktop)
- Documentação
- Relay comunitário (best-effort)

**Opções pagas:**
- **Ômega Cloud Relay** ($5-20/mês): SLA 99.9%, suporte prioritário
- **Enterprise Self-Hosted**: Suporte técnico, instalação, treinamento
- **VendaX.ai Integration**: Pacote específico com agents IA
- **Custom features**: Desenvolvimento sob demanda

**Modelo de custeio:**
- Infraestrutura básica: ~$50-100/mês (bootstrap + relay comunitário)
- Doações (OpenCollective)
- Sponsors empresariais (logo no site)

---

## 📦 Estrutura de Repositórios

### Monorepo vs Multi-repo

**Recomendação: Monorepo**

```
omega/ (GitHub: integralltech/omega)
├── .github/
│   ├── workflows/
│   │   ├── core-ci.yml
│   │   ├── android-ci.yml
│   │   ├── ios-ci.yml
│   │   └── desktop-ci.yml
│   └── ISSUE_TEMPLATE/
├── core/                    # Rust library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── crypto/
│   │   ├── network/
│   │   ├── storage/
│   │   ├── sync/
│   │   └── ffi/
│   └── tests/
├── android/                 # Kotlin app
│   ├── app/
│   └── gradle/
├── ios/                     # Swift app
│   ├── Omega.xcodeproj
│   └── Omega/
├── desktop/                 # Tauri app
│   ├── src-tauri/
│   └── src/                # React frontend
├── server/
│   ├── relay/              # Rust relay server
│   ├── store/              # Message store
│   └── discovery/          # Bootstrap DHT
├── protocol/               # Protobuf definitions
├── docs/                   # Documentation
│   ├── architecture/
│   ├── api/
│   └── guides/
└── scripts/
    ├── build.sh
    ├── test.sh
    └── deploy.sh
```

### Licença

**AGPL v3** para todo código + CLA (Contributor License Agreement)

**Justificativa:**
- AGPL impede empresas de fazer fork fechado
- Permite mudança para Apache 2.0 se necessário (com CLA)
- Compatible com objetivos open source
- Permite dual-licensing para enterprise (receita)

---

## 🧪 Testes e QA

### Estratégia de testes

**Unit tests (omega-core):**
- Coverage mínimo: 80%
- Property-based testing (proptest)
- Fuzzing de protocolos (cargo-fuzz)

**Integration tests:**
- Testes E2E com múltiplos peers
- NAT traversal scenarios
- Network partition resilience
- Message delivery guarantees

**Performance tests:**
- Latência P2P direto vs relay
- Throughput de mensagens
- Memory usage (leak detection)
- Battery impact (mobile)

**Security audits:**
- Trail of Bits (se budget permitir)
- Cure53 (alternativa)
- Bug bounty program (após v1.0)

### CI/CD Pipeline

```yaml
# .github/workflows/core-ci.yml
name: Core CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cd core && cargo test --all-features
      - run: cd core && cargo clippy -- -D warnings
      - run: cd core && cargo fmt -- --check
  
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/tarpaulin@v0.1
        with:
          args: '--ignore-tests --out Lcov'
      - uses: codecov/codecov-action@v3
```

---

## 🌍 Estratégia Open Source

### Comunidade

**Canais:**
- GitHub Discussions (primário)
- Discord (chat em tempo real)
- Matrix room (dogfooding!)
- Reddit r/OmegaChat (outreach)

**Governança:**
- BDFL (você) inicialmente
- Steering committee após 1 ano
- RFC process para features grandes

### Contribuições

**Tipos de contributors:**
- Core developers (Rust/Kotlin/Swift)
- Designers (UI/UX)
- Documentação (writers)
- Tradutores (i18n)
- Infrastructure (DevOps)

**Onboarding:**
- `good-first-issue` labels
- Detailed CONTRIBUTING.md
- Code review guidelines
- Developer setup scripts

### Marketing

**Lançamento:**
- Show HN (Hacker News)
- r/programming, r/privacy
- Product Hunt
- Telegram/Signal groups de privacy
- Write-up no blog IntegrallTech

**Diferenciação:**
- "First Brazilian P2P messenger"
- "LGPD-compliant by design"
- "Built for VendaX.ai B2B platform"

---

## 🔌 VendaX.ai Integration (Fase 8+)

### Architecture integration

```
┌─────────────────────────────────────────┐
│         VendaX.ai Platform               │
├─────────────────────────────────────────┤
│  ┌─────────────────────────────────┐    │
│  │   AI Agents (Python/Rust)        │    │
│  │   ├─ Agent Vendas                │    │
│  │   ├─ Agent Cobrança              │    │
│  │   ├─ Agent Atendimento           │    │
│  │   └─ Agent Analytics             │    │
│  └──────────┬──────────────────────┘    │
│             │                            │
│  ┌──────────▼──────────────────────┐    │
│  │   Ômega Bot SDK (Rust)          │    │
│  │   ├─ Message handling            │    │
│  │   ├─ E2E crypto                  │    │
│  │   └─ Webhook integration         │    │
│  └──────────┬──────────────────────┘    │
│             │                            │
│  ┌──────────▼──────────────────────┐    │
│  │   Ômega Self-Hosted Relay       │    │
│  │   (Private for VendaX clients)   │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
             │
             │ P2P/Relay
             ▼
┌─────────────────────────────────────────┐
│      Cliente Final (Distribuidor)        │
│  ┌─────────────────────────────────┐    │
│  │   Ômega Mobile/Desktop           │    │
│  │   ├─ Chat com vendedor           │    │
│  │   ├─ Chat com AI Agent           │    │
│  │   └─ Notificações pedidos        │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

### Use cases específicos

**1. Atendimento híbrido (humano + IA):**
```
Cliente: "Preciso de 50 caixas de tomate"
  ↓
Agent IA (Bot): Busca no ERP, verifica estoque
  ↓ (se disponível)
Agent IA: "Temos 120 caixas em estoque. Valor: R$ 2.450,00. Confirma pedido?"
  ↓ (se indisponível)
Agent IA: [transfere para vendedor humano]
Vendedor: "Tomate acabou hoje mas chega amanhã às 14h. Reservo pra você?"
```

**2. Notificações proativas:**
```
Agent IA → Cliente:
"🚚 Seu pedido #1234 saiu para entrega. Previsão: 14h-16h"
"📊 Seu extrato mensal está pronto. Valor total: R$ 45.320,00"
"⚠️ Produto X que você sempre compra está em promoção (20% off)"
```

**3. Compliance e auditoria:**
```
Toda conversa é:
- E2E encrypted (privacidade)
- Armazenada localmente no cliente (LGPD)
- Metadados auditáveis (tempo, participantes)
- Exportável (relatórios, compliance)
```

### Monetização VendaX.ai

**Modelo sugerido:**
- Ômega open source é gratuito
- **VendaX.ai Platform** cobra:
  - R$ 50-150/mês por vendedor (SaaS)
  - Inclui relay dedicado + AI agents
  - Suporte prioritário
  - Dashboard analytics

---

## 📊 Métricas de Sucesso

### Fase Alpha (6 meses)
- [ ] 1000+ downloads (F-Droid + direct)
- [ ] 100+ daily active users
- [ ] 50+ GitHub stars
- [ ] 10+ contributors
- [ ] 5+ traduções (i18n)

### Fase Beta (12 meses)
- [ ] 10k+ downloads
- [ ] 1k+ daily active users
- [ ] 500+ GitHub stars
- [ ] Cobertura de mídia tech (1+ artigo)
- [ ] Partnership com 1+ privacy org

### Fase 1.0 (18 meses)
- [ ] 100k+ downloads
- [ ] 10k+ daily active users
- [ ] 2k+ GitHub stars
- [ ] Security audit completo
- [ ] VendaX.ai integration em produção

---

## 🎬 Próximos Passos Imediatos

### Semana 1-2: Setup
1. [ ] Criar organização GitHub `integralltech` (ou `omega-chat`)
2. [ ] Setup monorepo com estrutura básica
3. [ ] Configurar CI/CD (GitHub Actions)
4. [ ] Criar Discord/Matrix community
5. [ ] Registrar domínio (omega.chat?)

### Semana 3-4: Proof of Concept
1. [ ] omega-core: Gerar keypairs (Ed25519)
2. [ ] omega-core: Conectar 2 peers via libp2p (localhost)
3. [ ] omega-core: Enviar mensagem plaintext P2P
4. [ ] Documentar arquitetura em docs/
5. [ ] CLI tool básico para testes

### Mês 2: MVP Core
1. [ ] Signal Protocol integration (libsignal-client)
2. [ ] SQLite storage (mensagens)
3. [ ] DHT peer discovery (Kademlia)
4. [ ] NAT traversal básico (STUN)
5. [ ] FFI bindings (uniffi)

### Mês 3: Infrastructure
1. [ ] Deploy 3 bootstrap nodes (DigitalOcean/Hetzner)
2. [ ] Deploy relay server (coturn ou custom)
3. [ ] Deploy message store (Redis + PostgreSQL)
4. [ ] Monitoring (Prometheus + Grafana)
5. [ ] Docs deployment (GitBook)

---

## 🤝 Como IntegrallTech se Beneficia

### Curto Prazo (0-12 meses)
- **Brand awareness:** "Brazilian tech company building privacy tools"
- **Talent acquisition:** Atrai desenvolvedores Rust/Crypto
- **Portfolio:** Case de open source para apresentar clientes
- **Learning:** Experiência com P2P, crypto, mobile (aplicável a outros projetos)

### Médio Prazo (12-24 meses)
- **VendaX.ai differentiator:** "Único B2B SaaS com comunicação P2P privada"
- **Enterprise offerings:** Self-hosted relay para clientes grandes
- **Consulting:** Expertise em descentralização para outros projetos

### Longo Prazo (24+ meses)
- **Dual revenue:** Open source + VendaX.ai premium features
- **Ecosystem:** Plugins, extensions, bots comerciais
- **Exit potential:** Tecnologia atraente para aquisição (privacy-focused companies)

---

## ❓ FAQs

**P: Por que AGPL em vez de MIT/Apache?**
R: AGPL impede empresas de fazer fork fechado. Se queremos ser verdadeiramente open source, precisamos garantir que melhorias retornem à comunidade. Podemos dual-license para empresas se quiserem usar sem AGPL.

**P: Rust não é muito complexo? Por que não Go/Java?**
R: Rust tem melhor ecossistema P2P (libp2p), crypto (RustCrypto), e FFI. Go seria segunda escolha. Java não tem libp2p production-ready e mobile é mais complicado.

**P: Como competir com Signal/Telegram?**
R: Não competimos diretamente. Nosso nicho é:
1. Usuários que querem verdadeira descentralização (Signal é centralizado)
2. B2B que precisa self-hosting (Telegram não permite)
3. Desenvolvedores que querem extender (bot API melhor que Signal)
4. Brasileiro (compliance LGPD nativo)

**P: Quanto custa manter infraestrutura?**
R: Estimativa conservadora:
- Bootstrap nodes: $15/mês (3x $5 VPS)
- Relay server: $20-40/mês (bandwidth variável)
- Message store: $40-60/mês (Redis + PostgreSQL)
- Monitoring: $0 (self-hosted Grafana)
- **Total: ~$75-115/mês**

Com 1000+ usuários, doações podem cobrir. Com VendaX.ai integration, é despesa operacional justificável.

**P: Timeline realista para v1.0?**
R: 18-24 meses com:
- 1 dev full-time (você) + 
- 2-3 devs part-time (core, mobile) +
- Comunidade (contributors ocasionais)

Se focar apenas em alpha MVP (Android + Desktop, sem iOS): 8-12 meses.

---

**Quer que eu detalhe alguma parte específica?**
- Setup inicial do repositório?
- Arquitetura do omega-core em Rust?
- Estratégia de go-to-market?
- Integração VendaX.ai específica?
