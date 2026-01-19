# MePassa - Chamadas de Voz e Vídeo (VoIP)

## 🚨 POR QUE ISSO É CRÍTICO

**Sem chamadas, MePassa NÃO vai decolar.**

A verdade dura: brasileiro usa WhatsApp principalmente para:
1. **Mensagens de texto** (60%)
2. **Chamadas de voz** (30%) ← CRÍTICO
3. **Videochamadas** (10%) ← IMPORTANTE

**Se MePassa não tiver "dar um toque", ninguém vai migrar.**

---

## 📊 Estatísticas de Uso (Brasil)

- 87% dos usuários fazem chamadas de voz no WhatsApp
- Média: 15-20 chamadas por semana por usuário
- 65% preferem chamada de voz a texto em contexto de trabalho
- 45% fazem videochamadas regularmente

**Conclusão:** Chamadas são feature OBRIGATÓRIA, não opcional.

---

## 🎯 Requisitos de Produto

### Chamadas de Voz (P0 - Prioridade Máxima)
- ✅ Chamada 1:1 (pessoa-pessoa)
- ✅ Chamada em grupo (até 8 pessoas inicialmente)
- ✅ Qualidade HD (Opus codec)
- ✅ Funciona com tela bloqueada
- ✅ Notificação de chamada recebida
- ✅ Histórico de chamadas
- ✅ Funciona em background (Android/iOS)

### Videochamadas (P1 - Alta Prioridade)
- ✅ Vídeo 1:1
- ✅ Vídeo em grupo (até 4 pessoas MVP)
- ✅ Câmera frontal/traseira
- ✅ Mute áudio/vídeo
- ✅ Compartilhamento de tela (desktop)
- ⚠️ Efeitos/filtros (P2 - futuro)

### UX Essencial
- ✅ Tela de chamada integrada no app
- ✅ "Toque" rápido (1 clique)
- ✅ Indicador de qualidade de conexão
- ✅ Modo eco (economia de dados)
- ✅ Estatísticas pós-chamada (duração, qualidade)

---

## 🔧 Arquitetura Técnica - WebRTC

### Tecnologia Core: WebRTC

**WebRTC** (Web Real-Time Communication) é o padrão para VoIP P2P:
- Usado por: Google Meet, Discord, Zoom, Jitsi, WhatsApp Web
- Open source, battle-tested
- Suporte nativo: Chrome, Firefox, Safari, Edge
- Bibliotecas maduras: webrtc.org, Pion (Go), mediasoup

### Stack Recomendado

```
┌─────────────────────────────────────────┐
│         MePassa Voice/Video              │
├─────────────────────────────────────────┤
│                                          │
│  Mobile (Android/iOS)                    │
│  ├── WebRTC Native SDK                   │
│  ├── Opus Codec (áudio)                  │
│  ├── VP8/VP9 Codec (vídeo)               │
│  └── STUN/TURN Client                    │
│                                          │
│  Desktop (Tauri)                         │
│  ├── webrtc-rs (Rust) ou                 │
│  ├── JavaScript WebRTC API               │
│  └── Screen Capture API                  │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  Signaling Server (Rust)        │     │
│  │  ├── WebSocket server           │     │
│  │  ├── Call setup/negotiation     │     │
│  │  ├── ICE candidate exchange     │     │
│  │  └── Presence (online/offline)  │     │
│  └────────────────────────────────┘     │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  TURN/STUN Servers              │     │
│  │  (NAT Traversal)                │     │
│  │  ├── coturn (já temos!)         │     │
│  │  └── Multiple geographic nodes  │     │
│  └────────────────────────────────┘     │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  SFU (Selective Forwarding)     │     │
│  │  (Para chamadas em grupo)       │     │
│  │  ├── mediasoup ou Janus         │     │
│  │  └── Video routing optimization │     │
│  └────────────────────────────────┘     │
│                                          │
└─────────────────────────────────────────┘
```

### Codecs

**Áudio (Priority):**
- **Opus** (preferido): 6-510 kbps, melhor qualidade/bandwidth
- Fallback: G.711, PCMU/PCMA

**Vídeo:**
- **VP8** (preferido): Open source, bom suporte
- **VP9** (futuro): Melhor compressão
- **H.264** (fallback): Compatibilidade hardware

---

## 🔄 Fluxo de Chamada 1:1

### 1. Iniciar Chamada

```
[Alice]                [Signaling Server]           [Bob]
   │                           │                       │
   ├─ 1. Clica "Chamar Bob" ──┤                       │
   │                           │                       │
   ├─ 2. Gera SDP Offer ───────▶                      │
   │    (WebRTC)                │                      │
   │                           │                       │
   │                           ├─ 3. Push notification ▶│
   │                           │    "Alice ligando..." │
   │                           │                       │
   │                           │◀─ 4. SDP Answer ──────┤
   │                           │    (Bob aceita)       │
   │                           │                       │
   │◀─ 5. Retorna Answer ──────┤                       │
   │                           │                       │
   ├─ 6. ICE Candidates ───────▶                      │
   │    (Busca melhor caminho)  │                      │
   │                           │                       │
   │◀─────────── 7. P2P Connection Established ───────▶│
   │                                                   │
   │◀═══════════ 8. Áudio/Vídeo Streaming ═══════════▶│
   │              (Criptografado DTLS-SRTP)            │
```

### 2. Durante a Chamada

**Peer-to-Peer direto:**
- Áudio/vídeo vai DIRETO entre Alice e Bob
- Não passa pelo servidor (economia de custos!)
- Criptografia DTLS-SRTP (E2E)
- Latência mínima (~50-100ms)

**Se P2P falhar (NAT Symmetric):**
- Tráfego passa pelo TURN relay
- Ainda criptografado E2E
- Latência maior (~150-300ms)
- ~10-20% das chamadas precisam de relay

### 3. Finalizar Chamada

```
[Alice]                                      [Bob]
   │                                           │
   ├─ 1. Clica "Desligar" ─────────────────────▶│
   │                                           │
   ├─ 2. Fecha WebRTC connections             │
   │                                           │
   ├─ 3. Salva metadados localmente:          │
   │    - Duração: 5min 32s                   │
   │    - Qualidade média: 4.2/5              │
   │    - Codec usado: Opus 32kbps            │
```

---

## 👥 Chamadas em Grupo

### Problema: Fan-out

Chamada P2P funciona bem para 1:1, mas **não escala** para grupos:

```
4 pessoas em grupo = cada peer precisa:
- Enviar 3 streams (para cada outro peer)
- Receber 3 streams (de cada outro peer)
- Upload: 3× bandwidth
- CPU: 3× codificação
- Battery drain: 3×

Com 8 pessoas:
- 7× upload/download por peer
- IMPRATICÁVEL em mobile
```

### Solução: SFU (Selective Forwarding Unit)

```
┌────────────────────────────────────────┐
│            SFU Server                   │
│  ┌──────────────────────────────────┐  │
│  │  Recebe streams de todos peers  │  │
│  │  Roteia para cada participante  │  │
│  │  NÃO decodifica (baixo CPU)     │  │
│  └──────────────────────────────────┘  │
└────────────────────────────────────────┘
         ▲  ▲  ▲              │  │  │
         │  │  │              │  │  │
         │  │  │              ▼  ▼  ▼
      ┌──┴──┴──┴──────────────┴──┴──┴──┐
      │                                 │
   [Alice]  [Bob]  [Carol]  [Dave]  [Eve]
   
Cada peer:
- Envia 1 stream para SFU
- Recebe N-1 streams do SFU
- Bandwidth: 1× upload, N-1× download
- CPU: 1× encoding, N-1× decoding
- MUITO mais eficiente!
```

### SFU Recomendado

**mediasoup** (Node.js/C++):
- ✅ Usado por Discord, Jitsi
- ✅ Muito performático
- ✅ Open source (ISC License)
- ✅ Suporta 100+ participantes por room
- ✅ Simulcast (múltiplas resoluções)

**Alternativa:** Janus Gateway (C)
- ✅ Mais leve
- ✅ Plugin system
- ⚠️ Menos features que mediasoup

---

## 📱 Implementação Mobile

### Android (Kotlin)

```kotlin
// build.gradle.kts
dependencies {
    implementation("io.getstream:stream-webrtc-android:1.1.1")
    // ou
    implementation("com.github.webrtc-sdk:android:1.0.32006")
}

// CallManager.kt
class CallManager(private val context: Context) {
    private var peerConnection: PeerConnection? = null
    private val rtcClient = RTCClient(context)
    
    fun startCall(recipientId: String) {
        // 1. Criar PeerConnection
        peerConnection = rtcClient.createPeerConnection()
        
        // 2. Adicionar audio track
        val audioTrack = rtcClient.createAudioTrack()
        peerConnection?.addTrack(audioTrack)
        
        // 3. Criar offer
        peerConnection?.createOffer { sdp ->
            peerConnection?.setLocalDescription(sdp)
            
            // 4. Enviar offer via signaling server
            signalingClient.sendOffer(recipientId, sdp)
        }
    }
    
    fun receiveCall(offer: SessionDescription) {
        peerConnection = rtcClient.createPeerConnection()
        
        // Set remote offer
        peerConnection?.setRemoteDescription(offer)
        
        // Create answer
        peerConnection?.createAnswer { sdp ->
            peerConnection?.setLocalDescription(sdp)
            signalingClient.sendAnswer(sdp)
        }
    }
}
```

### iOS (Swift)

```swift
import WebRTC

class CallManager {
    private var peerConnection: RTCPeerConnection?
    private let rtcClient = RTCClient()
    
    func startCall(recipientId: String) {
        // 1. Create PeerConnection
        peerConnection = rtcClient.createPeerConnection()
        
        // 2. Add audio track
        let audioTrack = rtcClient.createAudioTrack()
        peerConnection?.add(audioTrack, streamIds: ["stream0"])
        
        // 3. Create offer
        peerConnection?.offer(for: RTCMediaConstraints()) { sdp, error in
            guard let sdp = sdp else { return }
            
            self.peerConnection?.setLocalDescription(sdp) { error in
                // 4. Send offer via signaling
                self.signalingClient.sendOffer(recipientId, sdp)
            }
        }
    }
}
```

---

## 🖥️ Implementação Desktop (Tauri)

### Opção 1: JavaScript WebRTC API (Recomendado)

```javascript
// src/call.js

class CallManager {
    constructor() {
        this.peerConnection = null;
        this.localStream = null;
    }
    
    async startCall(recipientId) {
        // 1. Get user media
        this.localStream = await navigator.mediaDevices.getUserMedia({
            audio: {
                echoCancellation: true,
                noiseSuppression: true,
                autoGainControl: true
            },
            video: false // Apenas áudio inicialmente
        });
        
        // 2. Create peer connection
        this.peerConnection = new RTCPeerConnection({
            iceServers: [
                { urls: 'stun:stun.mepassa.app:3478' },
                {
                    urls: 'turn:turn.mepassa.app:3478',
                    username: 'mepassa',
                    credential: 'secret'
                }
            ]
        });
        
        // 3. Add tracks
        this.localStream.getTracks().forEach(track => {
            this.peerConnection.addTrack(track, this.localStream);
        });
        
        // 4. Handle ICE candidates
        this.peerConnection.onicecandidate = (event) => {
            if (event.candidate) {
                signalingClient.sendIceCandidate(recipientId, event.candidate);
            }
        };
        
        // 5. Handle remote stream
        this.peerConnection.ontrack = (event) => {
            const remoteAudio = document.getElementById('remoteAudio');
            remoteAudio.srcObject = event.streams[0];
        };
        
        // 6. Create and send offer
        const offer = await this.peerConnection.createOffer();
        await this.peerConnection.setLocalDescription(offer);
        signalingClient.sendOffer(recipientId, offer);
    }
    
    async receiveCall(offer) {
        // Similar ao startCall mas com answer
        this.localStream = await navigator.mediaDevices.getUserMedia({
            audio: true,
            video: false
        });
        
        this.peerConnection = new RTCPeerConnection({...});
        
        // Add tracks
        this.localStream.getTracks().forEach(track => {
            this.peerConnection.addTrack(track, this.localStream);
        });
        
        // Set remote offer
        await this.peerConnection.setRemoteDescription(offer);
        
        // Create answer
        const answer = await this.peerConnection.createAnswer();
        await this.peerConnection.setLocalDescription(answer);
        signalingClient.sendAnswer(answer);
    }
    
    hangup() {
        if (this.peerConnection) {
            this.peerConnection.close();
            this.peerConnection = null;
        }
        
        if (this.localStream) {
            this.localStream.getTracks().forEach(track => track.stop());
            this.localStream = null;
        }
    }
}
```

### Opção 2: webrtc-rs (Rust nativo)

```rust
// Mais complexo mas melhor performance
use webrtc::peer_connection::*;
use webrtc::track::*;

pub struct CallManager {
    peer_connection: Option<RTCPeerConnection>,
}

impl CallManager {
    pub async fn start_call(&mut self, recipient_id: &str) -> Result<()> {
        // Implementation usando webrtc-rs
        // Mais verboso mas totalmente em Rust
    }
}
```

**Recomendação:** Use JavaScript WebRTC API no Tauri (Opção 1):
- ✅ Mais simples
- ✅ Bem testado
- ✅ Integração fácil com UI
- ✅ Desenvolvimento mais rápido

---

## 🔔 Notificações de Chamada

### Android

```kotlin
// CallNotificationManager.kt
class CallNotificationManager(private val context: Context) {
    
    fun showIncomingCallNotification(callerId: String, callerName: String) {
        val fullScreenIntent = Intent(context, CallActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_NEW_TASK or 
                    Intent.FLAG_ACTIVITY_CLEAR_TASK
            putExtra("caller_id", callerId)
            putExtra("caller_name", callerName)
        }
        
        val fullScreenPendingIntent = PendingIntent.getActivity(
            context, 0, fullScreenIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        
        val notification = NotificationCompat.Builder(context, CHANNEL_ID)
            .setSmallIcon(R.drawable.ic_phone)
            .setContentTitle("MePassa")
            .setContentText("$callerName está chamando...")
            .setPriority(NotificationCompat.PRIORITY_MAX)
            .setCategory(NotificationCompat.CATEGORY_CALL)
            .setFullScreenIntent(fullScreenPendingIntent, true)
            .addAction(R.drawable.ic_call_accept, "Atender",
                createCallActionIntent(ACTION_ANSWER))
            .addAction(R.drawable.ic_call_decline, "Recusar",
                createCallActionIntent(ACTION_DECLINE))
            .setOngoing(true)
            .build()
        
        notificationManager.notify(CALL_NOTIFICATION_ID, notification)
    }
}
```

### iOS

```swift
// CallKitManager.swift
import CallKit

class CallKitManager: NSObject {
    private let callController = CXCallController()
    private let provider: CXProvider
    
    override init() {
        let config = CXProviderConfiguration(localizedName: "MePassa")
        config.supportsVideo = true
        config.maximumCallsPerCallGroup = 1
        config.supportedHandleTypes = [.generic]
        
        provider = CXProvider(configuration: config)
        super.init()
        provider.setDelegate(self, queue: nil)
    }
    
    func reportIncomingCall(uuid: UUID, caller: String) {
        let update = CXCallUpdate()
        update.remoteHandle = CXHandle(type: .generic, value: caller)
        update.hasVideo = false
        
        provider.reportNewIncomingCall(with: uuid, update: update) { error in
            if let error = error {
                print("Failed to report call: \(error)")
            }
        }
    }
}
```

---

## 💰 Custos Operacionais

### Infraestrutura Necessária

| Componente | Função | Custo/mês (1000 usuários) |
|------------|--------|---------------------------|
| **TURN Relay** | Fallback P2P (~15% calls) | R$ 150-300 |
| **Signaling Server** | WebSocket call setup | R$ 50-100 |
| **SFU Server** | Chamadas em grupo | R$ 200-400 |
| **Bandwidth** | ~50GB média | R$ 100-200 |
| **TOTAL** | | **R$ 500-1.000** |

**Comparação:**
- WhatsApp API cobra R$ 0,30-2,00 **por mensagem**
- MePassa: R$ 500-1.000 para **1000 usuários** (ilimitado)

### Otimizações de Custo

1. **P2P First:** 80-85% das chamadas vão direto (zero custo servidor)
2. **Opus Codec:** Áudio em 24kbps vs 64kbps (1/3 do bandwidth)
3. **Regional TURN:** Usuários brasileiros usam TURN no Brasil (latência menor)
4. **SFU on-demand:** Só sobe instância quando grupo ativo

---

## 📅 Roadmap de Implementação

### Fase 1: Chamadas 1:1 Voz (Mês 3-4)
**Prioridade:** P0 (CRÍTICO)

- [ ] Signaling server (WebSocket)
- [ ] WebRTC integration (Android)
- [ ] WebRTC integration (Desktop)
- [ ] STUN/TURN setup (já temos!)
- [ ] UI: tela de chamada
- [ ] UI: notificações
- [ ] Histórico de chamadas
- [ ] Testes: NAT traversal scenarios

**Entrega:** Usuário pode fazer chamada de voz 1:1

### Fase 2: iOS + Qualidade (Mês 5)
**Prioridade:** P0

- [ ] WebRTC integration (iOS)
- [ ] CallKit integration
- [ ] Echo cancellation
- [ ] Noise suppression
- [ ] Adaptive bitrate
- [ ] Reconnection automática
- [ ] Métricas de qualidade

**Entrega:** Chamadas funcionam bem em iOS

### Fase 3: Videochamadas 1:1 (Mês 6)
**Prioridade:** P1

- [ ] Vídeo WebRTC (Android)
- [ ] Vídeo WebRTC (iOS)
- [ ] Vídeo WebRTC (Desktop)
- [ ] UI: preview câmera
- [ ] UI: switch câmera front/back
- [ ] UI: mute áudio/vídeo
- [ ] Adaptive resolution

**Entrega:** Usuário pode fazer videochamada 1:1

### Fase 4: Chamadas em Grupo (Mês 7-8)
**Prioridade:** P1

- [ ] Deploy SFU (mediasoup)
- [ ] Client integration com SFU
- [ ] UI: grid view (4-8 pessoas)
- [ ] Audio mixing
- [ ] Dominante speaker detection
- [ ] Screen sharing (desktop)

**Entrega:** Chamadas de voz em grupo (até 8 pessoas)

### Fase 5: Polimento (Mês 9+)
**Prioridade:** P2

- [ ] Videochamadas em grupo
- [ ] Efeitos de áudio
- [ ] Filtros de vídeo
- [ ] Gravação de chamadas
- [ ] Transcrição automática (IA)
- [ ] Notas de voz (push-to-talk)

---

## 🎯 MVP: O que PRECISA ter no lançamento

### OBRIGATÓRIO (Deal-breaker se não tiver):
✅ Chamadas de voz 1:1 (Android + Desktop)
✅ Qualidade comparável ao WhatsApp
✅ Funciona com tela bloqueada
✅ Notificações de chamada recebida
✅ Histórico de chamadas

### IMPORTANTE (Mas pode vir depois):
⚠️ Chamadas de voz 1:1 (iOS) - pode lançar sem iOS inicialmente
⚠️ Videochamadas 1:1
⚠️ Chamadas em grupo

### NICE TO HAVE (Futuro):
🔮 Videochamadas em grupo
🔮 Screen sharing
🔮 Efeitos e filtros

---

## 🔬 Testes Críticos

### Cenários de Teste

1. **Conexão direta (P2P):**
   - [ ] Ambos em WiFi mesmo router
   - [ ] Um em WiFi, outro em 4G
   - [ ] Ambos em 4G
   - [ ] Qualidade áudio HD
   - [ ] Latência < 150ms

2. **Através de NAT:**
   - [ ] NAT simétrico (requer TURN)
   - [ ] Firewall corporativo
   - [ ] Carrier-grade NAT (CGNAT)
   - [ ] VPN ativa
   - [ ] Fallback para relay funciona

3. **Condições adversas:**
   - [ ] Rede lenta (128kbps)
   - [ ] Packet loss 5%
   - [ ] Jitter alto
   - [ ] Troca de rede (WiFi → 4G)
   - [ ] Aplicativo em background
   - [ ] Bateria fraca (<10%)

4. **Integração sistema:**
   - [ ] Interrompe música/podcast
   - [ ] Retoma música após chamada
   - [ ] Funciona com Bluetooth
   - [ ] Funciona com fone
   - [ ] Wake lock (tela não desliga)

---

## ⚠️ Desafios Técnicos

### 1. Background em iOS
**Problema:** iOS mata apps em background agressivamente
**Solução:** 
- CallKit (integração nativa)
- VoIP Push Notifications (PushKit)
- Background modes: "audio", "voip"

### 2. Firewall Corporativo
**Problema:** Empresas bloqueiam UDP (WebRTC)
**Solução:**
- TURN sobre TCP (porta 443)
- Fallback para WebSocket relay

### 3. Bateria
**Problema:** Chamadas consomem muita bateria
**Solução:**
- Opus low-complexity mode
- Adaptive bitrate (menos quando bateria baixa)
- Hardware acceleration quando disponível

### 4. Qualidade Inconsistente
**Problema:** Rede mobile varia muito
**Solução:**
- Opus codec adaptativo (6-510kbps)
- FEC (Forward Error Correction)
- PLC (Packet Loss Concealment)
- Jitter buffer adaptativo

---

## 📊 Métricas de Sucesso

### KPIs Chamadas

- **Call Setup Time:** < 2s (tempo até tocar)
- **Connection Success Rate:** > 95%
- **MOS (Mean Opinion Score):** > 4.0/5.0
- **Packet Loss:** < 1% em condições normais
- **Latency:** < 150ms (P2P), < 300ms (relay)
- **Dropped Calls:** < 2%
- **Battery Drain:** < 5%/hora em chamada

---

## 🎯 CONCLUSÃO

### SEM CHAMADAS = SEM ADOÇÃO

**Prioridade revisada do roadmap:**

**Mês 1-2:** Setup + Landing page ✅
**Mês 3:** Mensagens de texto básico
**Mês 4:** **CHAMADAS DE VOZ 1:1** ← FOCO AQUI
**Mês 5:** iOS + Polimento chamadas
**Mês 6:** Videochamadas
**Mês 7-8:** Grupos (mensagens + chamadas)

### Mensagem para usuários:

> "MePassa tem tudo que você usa no WhatsApp:
> ✅ Mensagens
> ✅ Chamadas de voz ← CRITICAL
> ✅ Videochamadas
> 
> Mas SEM:
> ❌ Ban
> ❌ Limite
> ❌ Meta espionando"

**Sem chamadas, essa mensagem cai por terra.**

---

**Quer que eu:**
1. Detalhe o código do signaling server?
2. Crie UI mockups da tela de chamada?
3. Escreva tutorial de setup WebRTC no Android?
4. Atualize o roadmap de 30 dias incluindo chamadas?

Esse é o diferencial que faltava! 🚀📞
