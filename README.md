# MePassa

> **Comunicação verdadeiramente híbrida: P2P quando possível, servidor quando necessário**

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-em%20desenvolvimento-yellow)](https://github.com/integralltech/mepassa)

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
- **libp2p:** Networking P2P (Kademlia DHT, GossipSub, Relay)
- **Signal Protocol:** E2E encryption (Double Ratchet, X3DH)
- **WebRTC:** VoIP (chamadas de voz/vídeo)
- **SQLite:** Storage local
- **Automerge:** CRDTs (multi-device sync)
- **UniFFI:** FFI bindings (Rust → Kotlin/Swift)

### Apps
- **Android:** Kotlin + Jetpack Compose
- **iOS:** Swift + SwiftUI
- **Desktop:** Tauri 2.0 (Rust + React)

### Servidor (Self-hosted)
- **Bootstrap Nodes:** libp2p DHT (peer discovery)
- **TURN Relay:** coturn (NAT traversal)
- **Message Store:** PostgreSQL + Redis (offline delivery)
- **Push Notifications:** FCM (Android) + APNs (iOS)

---

## 🚀 Roadmap

### Mês 1-2: Setup & Fundação ✅
- [x] Estrutura do monorepo
- [x] Workspace Rust configurado
- [ ] CI/CD básico
- [ ] Landing page
- [ ] 50-100 beta testers

### Mês 3: Mensagens Básicas
- [ ] Core library (Identity + Crypto + Network + Storage)
- [ ] Android MVP (mensagens texto)
- [ ] Desktop MVP (Tauri)
- [ ] 10 beta testers trocando mensagens

### Mês 4: CHAMADAS DE VOZ 🔥 **PRIORIDADE MÁXIMA**
- [ ] WebRTC integration
- [ ] TURN relay
- [ ] UI de chamadas (Android + Desktop)
- [ ] Qualidade >4.0/5.0 MOS
- [ ] **Teste decisivo:** "Você usaria MePassa como chat principal?"

### Mês 5: iOS + Videochamadas
- [ ] App iOS (Swift + SwiftUI)
- [ ] Videochamadas 1:1
- [ ] CallKit integration

### Mês 6: Grupos + Polimento
- [ ] Grupos (até 256 pessoas)
- [ ] Chamadas em grupo (até 8)
- [ ] Mídia (imagens, vídeos, arquivos)
- [ ] Multi-device sync

---

## 🛠️ Desenvolvimento

### Pré-requisitos

- **Rust:** 1.70+ (`rustup default stable`)
- **Node.js:** 18+ (para desktop app)
- **Android Studio:** (para Android app)
- **Xcode:** (para iOS app, macOS only)
- **Docker:** (para servidores)

### Build

```bash
# Core library
cd core
cargo build

# Android app
cd android
./gradlew assembleDebug

# iOS app
cd ios
xcodebuild -workspace MePassa.xcworkspace -scheme MePassa -configuration Debug

# Desktop app
cd desktop
npm install
npm run tauri build
```

### Testes

```bash
# Core tests
cargo test --workspace

# Benchmarks
cargo bench
```

---

## 📖 Documentação

- [**Arquitetura Híbrida**](docs/architecture/hibrida.md) - Por que P2P + Servidor
- [**Tech Stack Completo**](docs/architecture/tech-stack.md) - Bibliotecas e justificativas
- [**Plano de Execução**](EXECUCAO.md) - Fases detalhadas do projeto
- [**Guia de Contribuição**](CONTRIBUTING.md) - Como contribuir
- [**Código de Conduta**](CODE_OF_CONDUCT.md)

---

## 🤝 Contribuindo

Aceitamos contribuições! Veja [CONTRIBUTING.md](CONTRIBUTING.md) para detalhes.

**Áreas que precisamos:**
- 🦀 **Core Developers** (Rust: libp2p, crypto, WebRTC)
- 📱 **Mobile Developers** (Kotlin/Compose, Swift/SwiftUI)
- 🖥️ **Desktop Developers** (Tauri, React)
- 🎨 **Designers** (UI/UX)
- 📝 **Documentação** (writers)
- 🌍 **Tradutores** (i18n)

---

## 📊 Status do Projeto

**Versão:** 0.1.0-alpha (em desenvolvimento)

| Componente | Status | Progresso |
|------------|--------|-----------|
| Core (Rust) | 🚧 Em progresso | 5% |
| Android | ⏳ Aguardando | 0% |
| iOS | ⏳ Aguardando | 0% |
| Desktop | ⏳ Aguardando | 0% |
| Server | ⏳ Aguardando | 0% |

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

## 📜 Licença

[AGPL-3.0](LICENSE) - Este projeto é open source.

**IMPORTANTE:** AGPL impede forks fechados. Se você usar MePassa em um serviço, deve disponibilizar o código-fonte.

---

## 🙏 Agradecimentos

Construído com tecnologias open source incríveis:
- [**libp2p**](https://libp2p.io/) - Protocol Labs
- [**Signal Protocol**](https://signal.org/docs/) - Signal Foundation
- [**WebRTC**](https://webrtc.org/)
- [**Tauri**](https://tauri.app/)
- E muitas outras...

---

## 📞 Contato

- **Website:** [mepassa.app](https://mepassa.app) *(em breve)*
- **GitHub:** [github.com/integralltech/mepassa](https://github.com/integralltech/mepassa)
- **Discord:** *(em breve)*
- **Matrix:** *(em breve)*
- **Email:** contato@integralltech.com.br

---

<div align="center">

**Feito com ❤️ por [IntegrallTech](https://integralltech.com.br)**

*"Não adianta ter privacidade perfeita se ninguém usar.*
*MePassa escolhe privacidade boa o suficiente + UX boa o suficiente = Adoção real."*

</div>
