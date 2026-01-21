# MePassa

> **Comunicação verdadeiramente híbrida: P2P quando possível, servidor quando necessário**

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-FASE%2012%20VoIP-green)](https://github.com/integralltech/mepassa)

## 🎯 Visão

**MePassa** é uma plataforma de mensagens instantâneas com arquitetura **HÍBRIDA P2P + Servidor**:

- **80% P2P direto:** Mensagens vão peer-to-peer (privacidade máxima, zero custo)
- **15% TURN relay:** Fallback quando NAT simétrico/firewall
- **5% Store & Forward:** Destinatário offline (PostgreSQL, TTL 14 dias)

### Diferencial

| | WhatsApp | Telegram | Signal | **MePassa** |
|---|---|---|---|---|
| **E2E por padrão** | ✅ | ❌ | ✅ | ✅ |
| **Sem telefone** | ❌ | ❌ | ❌ | ✅ |
| **P2P direto** | ❌ | ❌ | ❌ | ✅ (80%) |
| **VoIP integrado** | ✅ | ✅ | ✅ | ✅ |
| **Funciona offline** | ✅ | ✅ | ✅ | ✅ |
| **Self-hosting** | ❌ | ❌ | ❌ | ✅ |
| **Open source** | ❌ | ⚠️ | ✅ | ✅ |
| **Sem ban** | ❌ | ❌ | ❌ | ✅ |

**TL;DR:** Como WhatsApp (funciona sempre) + Melhor que WhatsApp (privado, sem ban, self-hosting).

---

## 🏗️ Arquitetura

```
┌──────────────────────────────────────────────────┐
│              MEPASSA HÍBRIDO                      │
├──────────────────────────────────────────────────┤
│                                                   │
│  CENÁRIO 1: P2P Direto (80%)                     │
│  ────────────────────────────                    │
│  [Alice] ←────── P2P ──────→ [Bob]               │
│  • Zero custo servidor                           │
│  • Latência ~50ms                                │
│  • Privacidade máxima                            │
│                                                   │
│  CENÁRIO 2: TURN Relay (15%)                     │
│  ────────────────────────────                    │
│  [Alice] ──→ [TURN] ──→ [Bob]                    │
│  • NAT simétrico/Firewall                        │
│  • Ainda E2E encrypted                           │
│  • Latência ~200ms                               │
│                                                   │
│  CENÁRIO 3: Store & Forward (5%)                 │
│  ────────────────────────────────                │
│  [Alice] ──→ [Store] ··· [Bob offline]           │
│                │                                  │
│                └──→ [Bob] (quando online)        │
│  • TTL 14 dias                                   │
│  • Encrypted no servidor                         │
│  • Auto-delete após entrega                      │
│                                                   │
└──────────────────────────────────────────────────┘
```

---

## 📦 Stack Técnico

### Core (Rust)
- **libp2p:** Networking P2P (Kademlia DHT, GossipSub, Circuit Relay v2)
- **Signal Protocol:** E2E encryption (Double Ratchet, X3DH)
- **WebRTC:** VoIP chamadas de voz (webrtc-rs + Opus codec)
- **SQLite:** Storage local thread-safe
- **UniFFI 0.31:** FFI bindings (Rust → Kotlin/Swift)

### Apps
- **Android:** Kotlin + Jetpack Compose + Material3
- **iOS:** Swift + SwiftUI *(em desenvolvimento)*
- **Desktop:** Tauri 2.0 (Rust + React + TypeScript)

### Servidor (Self-hosted)
- **Bootstrap Nodes:** libp2p DHT + Kademlia (peer discovery)
- **TURN Relay:** coturn (NAT traversal para WebRTC)
- **Message Store:** PostgreSQL + Redis (offline delivery)
- **Push Notifications:** FCM (Android) + APNs (iOS)
- **Identity Server:** Username resolution (@alice → peer_id)

---

## 🚧 Progresso Atual

**Status:** ✅ **FASE 12 (VoIP) - 95% COMPLETA** - Pronto para testes físicos!

### ✅ Completado (11 de 19 fases - 58%)

**FASE 1-5: Core Library (100%)** ✅
- ✅ Identity (Ed25519) + Crypto (Signal Protocol Double Ratchet)
- ✅ Network (libp2p: Kademlia DHT, GossipSub, mDNS, Identify)
- ✅ Storage (SQLite thread-safe, migrations, FTS5 search)
- ✅ Protocol (Protobuf) + Client API completa
- ✅ FFI Bindings (UniFFI 0.31: Kotlin + Swift)
- 📊 **~9.000 LoC**, 110+ testes passando

**FASE 6: Android App MVP (100%)** ✅
- ✅ Jetpack Compose + Material3
- ✅ 3 telas: Onboarding → Conversations → Chat
- ✅ MePassaClientWrapper (singleton, coroutines)
- ✅ Foreground Service P2P + notificação persistente
- ✅ Mensagens texto 1:1 funcionais
- 📊 **~1.500 LoC**, 22 arquivos

**FASE 7: Desktop App MVP (100%)** ✅
- ✅ Tauri 2.0 + React 18 + TypeScript
- ✅ 3 views: Onboarding → Conversations → Chat
- ✅ FFI integration via Tauri commands
- ✅ System tray + menu contextual
- ✅ Cross-platform (DMG, MSI, AppImage)
- 📊 **~2.200 LoC**, 20 arquivos

**FASE 9: Bootstrap + DHT Server (100%)** ✅
- ✅ Kademlia DHT para peer discovery
- ✅ SQLite persistence (peers, records)
- ✅ Health check endpoint
- ✅ Docker + docker-compose
- 📊 **~700 LoC**, 6 arquivos

**FASE 10: P2P Relay + TURN Server (100%)** ✅
- ✅ libp2p Circuit Relay v2 (server + client)
- ✅ DCUtR hole punching automático
- ✅ coturn TURN server configurado
- ✅ TURN credentials service (HMAC-SHA1)
- ✅ Fallback automático (direto → hole punch → relay)
- 📊 **~1.650 LoC**, 18 arquivos

**FASE 11: Message Store (100%)** ✅
- ✅ PostgreSQL + Redis para store & forward
- ✅ API REST (store, retrieve, delete)
- ✅ TTL automático (14 dias)
- ✅ Encryption em repouso
- 📊 **~900 LoC**, 7 arquivos

**FASE 12: VoIP - Chamadas de Voz (95%)** 🔥 **← VOCÊ ESTÁ AQUI**
- ✅ WebRTC integration (webrtc-rs + SDP + ICE)
- ✅ Opus codec (24kbps, 20ms frames)
- ✅ P2P signaling via libp2p
- ✅ TURN client integration
- ✅ Android UI (CallScreen + IncomingCallScreen)
- ✅ Desktop UI (CallView + IncomingCallModal)
- ✅ Runtime permissions (RECORD_AUDIO, BLUETOOTH_CONNECT)
- ✅ CallAudioManager (Bluetooth auto-routing)
- ✅ Background calls (foreground service PHONE_CALL)
- ✅ Call history database (SQLite schema v2)
- ⏳ **Pendente:** Testes em dispositivos físicos
- 📊 **~4.600 LoC**, 21/24 tarefas completas

### 📊 Estatísticas Gerais

| Componente | Status | Arquivos | LoC | Testes |
|------------|--------|----------|-----|--------|
| Core (Rust) | ✅ 100% | 70 | ~11.200 | 110+ |
| FFI Bindings | ✅ 100% | 5 | ~300 | - |
| Android (Kotlin) | ✅ 95% | 30 | ~3.000 | - |
| Desktop (TypeScript) | ✅ 95% | 25 | ~2.900 | - |
| Servers (Rust) | ✅ 100% | 45 | ~4.200 | - |
| Docs | ✅ | 14 | ~4.450 | - |
| **TOTAL** | **70%** | **198** | **~22.764** | **110+** |

### 🎯 Próximo: Testes Reais VoIP

**Validar FASE 12:**
- [ ] Build Android APK (debug)
- [ ] Testar em 2 dispositivos físicos
- [ ] Medir latência P2P (<100ms esperado)
- [ ] Validar MOS Score >4.0
- [ ] Comparar com WhatsApp

**Após testes bem-sucedidos:** FASE 13 (iOS App) ou FASE 14 (Videochamadas)

---

## 🚀 Roadmap

### Mês 1-2: Setup & Fundação ✅
- [x] Estrutura do monorepo
- [x] Workspace Rust configurado
- [x] Core library completa
- [ ] CI/CD básico
- [ ] Landing page
- [ ] 50-100 beta testers

### Mês 3: Mensagens Básicas ✅
- [x] Core library (Identity + Crypto + Network + Storage)
- [x] Android MVP (mensagens texto)
- [x] Desktop MVP (Tauri)
- [x] Bootstrap + TURN + Store servers
- [ ] 10 beta testers trocando mensagens

### Mês 4: CHAMADAS DE VOZ 🔥 **95% COMPLETO**
- [x] WebRTC integration
- [x] TURN relay
- [x] UI de chamadas (Android + Desktop)
- [x] Runtime permissions + Bluetooth
- [ ] Qualidade >4.0/5.0 MOS (testar)
- [ ] **Teste decisivo:** "Você usaria MePassa como chat principal?"

### Mês 5: iOS + Videochamadas ⏳
- [ ] App iOS (Swift + SwiftUI)
- [ ] Videochamadas 1:1
- [ ] CallKit integration (iOS)

### Mês 6: Grupos + Polimento ⏳
- [ ] Grupos (até 256 pessoas)
- [ ] Chamadas em grupo (até 8)
- [ ] Mídia (imagens, vídeos, arquivos)
- [ ] Multi-device sync

---

## 🛠️ Desenvolvimento

### Pré-requisitos

- **Rust:** 1.75+ (`rustup default stable`)
- **Node.js:** 18+ (para desktop app)
- **Android Studio:** Hedgehog+ (para Android app)
- **NDK:** 26.1.10909125+
- **Docker:** (para servidores)

### Build Rápido

```bash
# Core library
cd core
cargo build --release

# Android app (requer NDK)
cd android
./gradlew assembleDebug

# Desktop app
cd desktop
npm install
npm run tauri dev

# Servidores (Bootstrap + TURN + Store)
cd server
docker-compose up -d
```

### Testes

```bash
# Core tests (110+ testes)
cd core
cargo test --workspace

# Benchmarks
cargo bench

# Android (manual)
# Seguir BUILD_AND_TEST.md
```

**Documentação completa:** [BUILD_AND_TEST.md](BUILD_AND_TEST.md)

---

## 📖 Documentação

### Guias Principais
- [**Plano de Execução**](EXECUCAO.md) - Fases detalhadas, progresso atual
- [**Build & Test Guide**](BUILD_AND_TEST.md) - Como buildar e testar VoIP
- [**Arquitetura Híbrida**](docs/architecture/hibrida.md) - Por que P2P + Servidor
- [**Tech Stack**](docs/architecture/tech-stack.md) - Bibliotecas e justificativas

### Por Componente
- **Android:** [BUILD_GUIDE.md](android/BUILD_GUIDE.md) | [TESTING.md](android/TESTING.md) | [README.md](android/README.md)
- **Desktop:** [README.md](desktop/README.md) | [ARCHITECTURE.md](desktop/ARCHITECTURE.md)
- **Core:** [FFI_IMPLEMENTATION.md](core/FFI_IMPLEMENTATION.md) | [FASE5_ARTIFACTS.md](core/FASE5_ARTIFACTS.md)
- **Servers:** READMEs em `server/bootstrap`, `server/store`, `server/push`

---

## 🤝 Contribuindo

Aceitamos contribuições! Veja [CONTRIBUTING.md](CONTRIBUTING.md) para detalhes.

**Áreas que precisamos:**
- 🦀 **Core Developers** (Rust: libp2p, WebRTC, crypto)
- 📱 **Mobile Developers** (Kotlin/Compose, Swift/SwiftUI)
- 🖥️ **Desktop Developers** (Tauri, React, TypeScript)
- 🎨 **Designers** (UI/UX para Android/iOS/Desktop)
- 📝 **Documentação** (technical writers)
- 🌍 **Tradutores** (i18n: pt-BR, en, es)
- 🧪 **QA Testers** (testes VoIP em dispositivos reais)

---

## 📊 Status Detalhado

**Versão:** 0.1.0-alpha (em desenvolvimento)

| Fase | Componente | Status | Progresso |
|------|------------|--------|-----------|
| 1-5 | Core (Rust) | ✅ Completo | 100% |
| 6 | Android App | ✅ Completo | 100% |
| 7 | Desktop App | ✅ Completo | 100% |
| 8 | Push Notifications | 🚧 Em progresso | 75% |
| 9 | Bootstrap + DHT | ✅ Completo | 100% |
| 10 | P2P Relay + TURN | ✅ Completo | 100% |
| 11 | Message Store | ✅ Completo | 100% |
| 12 | **VoIP Calls** | 🔥 **Quase pronto** | **95%** |
| 13 | iOS App | ⏳ Aguardando | 0% |
| 14 | Videochamadas | ⏳ Aguardando | 0% |
| 15 | Grupos | ⏳ Aguardando | 0% |
| 16 | Mídia & Polimento | ⏳ Aguardando | 0% |
| 17 | Multi-Device | ⏳ Aguardando | 0% |

**Progresso geral:** 11/19 fases (58%) | ~22.764 LoC (70% do estimado)

---

## 💰 Modelo de Negócio

**Open Source Core + Serviços Opcionais**

### Sempre gratuito:
- ✅ Código completo (AGPL v3)
- ✅ Apps (Android/iOS/Desktop)
- ✅ Documentação
- ✅ Relay comunitário (best-effort)

### Opções pagas (futuro):
- **MePassa Cloud Relay** ($5-20/mês): SLA 99.9%, suporte
- **Enterprise Self-Hosted:** Suporte técnico, instalação
- **Custom Development:** Features sob demanda

---

## 🎯 Milestone Crítico (Próximo)

**TESTE DECISIVO após FASE 12:**

Perguntar a 20+ beta testers:
> **"Você usaria MePassa como seu chat principal?"**

- **< 50% SIM:** ⛔ PARA TUDO e conserta VoIP
- **50-70% SIM:** ⚠️ Continua com cautela, itera feedback
- **> 70% SIM:** 🚀 Full speed para iOS (FASE 13)

*Chamadas de voz são deal-breaker. 87% dos brasileiros usam WhatsApp para chamadas.*

---

## 📜 Licença

[AGPL-3.0](LICENSE) - Este projeto é open source.

**IMPORTANTE:** AGPL impede forks fechados. Se você usar MePassa em um serviço, deve disponibilizar o código-fonte.

---

## 🙏 Agradecimentos

Construído com tecnologias open source incríveis:
- [**libp2p**](https://libp2p.io/) - Protocol Labs
- [**Signal Protocol**](https://signal.org/docs/) - Signal Foundation
- [**WebRTC**](https://webrtc.org/) - webrtc-rs
- [**Tauri**](https://tauri.app/)
- [**UniFFI**](https://mozilla.github.io/uniffi-rs/) - Mozilla
- E muitas outras...

---

## 📞 Contato

- **Website:** [mepassa.app](https://mepassa.app) *(em breve)*
- **GitHub:** [github.com/integralltech/mepassa](https://github.com/integralltech/mepassa)
- **Discord:** *(em breve)*
- **Email:** contato@integralltech.com.br

---

<div align="center">

**Feito com ❤️ por [IntegrallTech](https://integralltech.com.br)**

*"Não adianta ter privacidade perfeita se ninguém usar.*
*MePassa escolhe privacidade boa o suficiente + UX boa o suficiente = Adoção real."*

[![Star on GitHub](https://img.shields.io/github/stars/integralltech/mepassa?style=social)](https://github.com/integralltech/mepassa)

</div>
