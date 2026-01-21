# Build & Test Guide - MePassa VoIP

Este guia descreve como buildar e testar o MePassa com funcionalidades VoIP (FASE 12).

## 📋 Pré-requisitos

### Rust Core
```bash
rustc --version  # Requer Rust 1.75+
cargo --version
```

### Android
```bash
# Android Studio Hedgehog | 2023.1.1+
# Android SDK 34 (API Level 34)
# NDK 26.1.10909125+
```

### Desktop (Tauri)
```bash
node --version   # v18+
npm --version    # v9+
```

### Servidores Backend
```bash
docker --version
docker-compose --version
```

---

## 🔧 Build Android APK

### 1. Build Rust Core (libmepassa_core.so)

```bash
cd core

# Build para Android (ARM64)
cargo ndk -t arm64-v8a -o ../android/app/src/main/jniLibs build --release

# Build para Android (x86_64 - emulador)
cargo ndk -t x86_64 -o ../android/app/src/main/jniLibs build --release

# Verificar se as libs foram geradas
ls -lh ../android/app/src/main/jniLibs/arm64-v8a/libmepassa_core.so
ls -lh ../android/app/src/main/jniLibs/x86_64/libmepassa_core.so
```

**Tamanho esperado:** ~6-8 MB (release)

### 2. Generate UniFFI Bindings

```bash
cd core
cargo run --bin uniffi-bindgen generate --library target/release/libmepassa_core.so --language kotlin --out-dir ../android/app/src/main/kotlin/uniffi/mepassa
```

### 3. Build Android APK

```bash
cd android

# Debug build (para testes)
./gradlew assembleDebug

# Release build (otimizado)
./gradlew assembleRelease

# APK localizado em:
# app/build/outputs/apk/debug/app-debug.apk
# app/build/outputs/apk/release/app-release.apk
```

### 4. Instalar em Dispositivo

```bash
# Via USB (adb)
adb devices  # Verificar se dispositivo está conectado
adb install -r app/build/outputs/apk/debug/app-debug.apk

# Ou via Android Studio: Run > Run 'app'
```

---

## 🖥️ Build Desktop App

### 1. Build Rust Core (lib)

```bash
cd core
cargo build --release
```

### 2. Build Tauri Desktop

```bash
cd desktop

# Instalar dependências
npm install

# Development mode (hot reload)
npm run tauri dev

# Production build
npm run tauri build

# Binários em:
# src-tauri/target/release/mepassa-desktop (Linux)
# src-tauri/target/release/mepassa-desktop.app (macOS)
# src-tauri/target/release/mepassa-desktop.exe (Windows)
```

---

## 🧪 Testes VoIP (FASE 12)

### Checklist Pré-Teste

- [ ] 2 dispositivos físicos Android (ou 1 físico + 1 emulador)
- [ ] Ambos na mesma rede WiFi (para P2P direto)
- [ ] Permissões RECORD_AUDIO concedidas
- [ ] Servidores backend rodando (Bootstrap + TURN)

### Iniciar Servidores Backend

```bash
cd server
docker-compose up -d bootstrap-node-1 coturn

# Verificar logs
docker-compose logs -f bootstrap-node-1
docker-compose logs -f coturn

# Verificar health
curl http://localhost:8000/health  # Bootstrap
curl http://localhost:8001/health  # TURN credentials (se disponível)
```

### Teste 1: P2P Direto (Mesma Rede)

**Setup:**
1. Dispositivo A: Abrir MePassa, anotar Peer ID
2. Dispositivo B: Abrir MePassa, anotar Peer ID

**Passos:**
1. Device A: Ir em Conversations > Add Contact (colar Peer ID do Device B)
2. Device A: Abrir chat com Device B
3. Device A: Clicar botão Phone (🔊 ícone)
4. Sistema solicita permissões → Conceder RECORD_AUDIO
5. Device B: Deve receber notificação (manual nav para IncomingCallScreen)
6. Device B: Clicar "Atender"
7. Ambos em CallScreen ativo

**Validações:**
- [ ] Audio funciona nos dois lados
- [ ] Latência percebida < 200ms (conversa fluida)
- [ ] Timer contando corretamente
- [ ] Botão Mute funciona (silencia microfone)
- [ ] Botão Speaker funciona (alterna alto-falante)
- [ ] Botão Hangup encerra chamada
- [ ] Voltam para ConversationsScreen após hangup

**Logs Esperados:**
```
# Device A (caller)
MePassaClient: Starting call to <peer_id_B>
WebRTC: Creating PeerConnection
WebRTC: Local SDP offer created
Signaling: Sending offer to <peer_id_B>
WebRTC: ICE gathering complete
CallAudioManager: Starting call audio management
CallAudioManager: Mode: MODE_IN_COMMUNICATION

# Device B (callee)
Signaling: Received offer from <peer_id_A>
WebRTC: Creating PeerConnection
WebRTC: Setting remote SDP
WebRTC: Creating answer
CallAudioManager: Starting call audio management
```

### Teste 2: TURN Relay (Redes Diferentes)

**Setup:**
1. Dispositivo A: WiFi casa
2. Dispositivo B: 4G/5G (ou WiFi outra rede)

**Passos:**
1. Repetir Teste 1
2. Conexão P2P direta deve falhar
3. Sistema automaticamente usa TURN relay

**Validações:**
- [ ] Chamada estabelecida (mesmo sem P2P direto)
- [ ] Latência percebida < 300ms
- [ ] Audio funciona nos dois lados

**Logs Esperados:**
```
WebRTC: Direct connection failed
TURN: Requesting allocation from coturn
TURN: Relay established via <turn_server>
ICE: Using relay candidate
```

### Teste 3: Bluetooth Headset

**Setup:**
1. Conectar fone Bluetooth ao dispositivo
2. Iniciar chamada

**Validações:**
- [ ] Audio automaticamente roteado para Bluetooth
- [ ] Microfone do Bluetooth funciona
- [ ] Botão Speaker desativa Bluetooth e usa alto-falante
- [ ] Após desconectar Bluetooth, volta para earpiece

**Logs Esperados:**
```
CallAudioManager: Bluetooth device detected
CallAudioManager: Routing to Bluetooth
AudioManager: isBluetoothScoOn = true
```

### Teste 4: Background Call

**Setup:**
1. Iniciar chamada
2. Pressionar Home (minimizar app)

**Validações:**
- [ ] Chamada continua ativa em background
- [ ] Notificação foreground visível
- [ ] Audio continua funcionando
- [ ] Retornar ao app volta para CallScreen

### Teste 5: Permissions Denied

**Setup:**
1. Desinstalar app
2. Reinstalar

**Passos:**
1. Abrir chat
2. Clicar Phone button
3. Sistema solicita RECORD_AUDIO
4. Negar permissão

**Validações:**
- [ ] Snackbar aparece com mensagem explicativa
- [ ] Chamada NÃO inicia
- [ ] Clicar Phone novamente re-solicita permissão

---

## 📊 Métricas de Sucesso

### Performance
- **Latência P2P:** < 100ms (ideal: ~50ms)
- **Latência TURN:** < 300ms (ideal: ~200ms)
- **Dropout rate:** < 5%
- **MOS Score (audio quality):** > 4.0/5.0

### Funcionalidade
- **Connection success rate:** > 95%
- **Mute toggle response:** < 100ms
- **Speaker toggle response:** < 200ms
- **Bluetooth routing:** < 1s após conexão

### Como Medir

**Latência (ping simples):**
```bash
# No backend, adicionar timestamp nas mensagens
# Frontend calcula: received_at - sent_at

# Ou usar ferramentas:
adb shell ping <ip_do_outro_dispositivo>
```

**MOS Score (subjetivo):**
- 5.0 = Excelente (telefone fixo)
- 4.0 = Bom (WhatsApp em boa conexão)
- 3.0 = Aceitável (com eco/delay perceptível)
- 2.0 = Ruim (difícil entender)
- 1.0 = Muito ruim (ininteligível)

---

## 🐛 Troubleshooting

### Problema: "Failed to initialize MePassaClient"

**Causa:** Biblioteca Rust não carregada

**Solução:**
```bash
# Verificar se libmepassa_core.so existe
ls android/app/src/main/jniLibs/arm64-v8a/libmepassa_core.so

# Re-build se necessário
cd core && cargo ndk -t arm64-v8a build --release
```

### Problema: Permissões não solicitadas

**Causa:** AndroidManifest sem permissões ou runtime request falhando

**Solução:**
```xml
<!-- Verificar em AndroidManifest.xml -->
<uses-permission android:name="android.permission.RECORD_AUDIO" />
<uses-permission android:name="android.permission.MODIFY_AUDIO_SETTINGS" />
```

### Problema: Audio não funciona

**Causa:** CallAudioManager não iniciado ou AudioFocus não concedido

**Logs:**
```bash
adb logcat | grep CallAudioManager
adb logcat | grep AudioFocus
```

**Solução:**
- Verificar se CallAudioManager.startCall() foi chamado
- Verificar se MODE_IN_COMMUNICATION está setado
- Reiniciar dispositivo (bug AudioManager ocasional)

### Problema: Bluetooth não detectado

**Causa:** Permissão BLUETOOTH_CONNECT não concedida (Android 12+)

**Solução:**
```bash
# Verificar permissões concedidas
adb shell dumpsys package com.mepassa | grep permission

# Deve conter:
# android.permission.BLUETOOTH_CONNECT: granted=true
```

### Problema: WebRTC connection failed

**Causa:** Firewall bloqueando UDP ou TURN server offline

**Logs:**
```bash
adb logcat | grep WebRTC
adb logcat | grep ICE
```

**Solução:**
1. Verificar se coturn está rodando: `docker ps | grep coturn`
2. Verificar firewall: `sudo ufw status` (desabilitar temporariamente)
3. Testar na mesma rede WiFi primeiro

---

## 📝 Reportar Bugs

**Template:**
```
**Dispositivos:**
- Device A: [Modelo, Android version]
- Device B: [Modelo, Android version]

**Conexão:**
- [ ] Mesma rede WiFi
- [ ] Redes diferentes (WiFi + 4G)
- [ ] Ambos em 4G

**Descrição:**
[Descrever o problema]

**Passos para reproduzir:**
1. ...
2. ...

**Logs:**
```
[Colar logs relevantes de adb logcat]
```

**Comportamento esperado:**
[O que deveria acontecer]

**Screenshots/Videos:**
[Se aplicável]
```

---

## ✅ Checklist Final

Antes de considerar FASE 12 completa:

**Funcionalidades:**
- [ ] Chamadas P2P direto funcionam
- [ ] Chamadas via TURN relay funcionam
- [ ] Permissions solicitadas corretamente
- [ ] Mute toggle funciona
- [ ] Speaker toggle funciona
- [ ] Bluetooth routing funciona
- [ ] Background calls funcionam
- [ ] Hangup encerra chamada corretamente

**Performance:**
- [ ] Latência P2P < 100ms
- [ ] Latência TURN < 300ms
- [ ] MOS Score > 4.0
- [ ] Connection success > 95%

**UX:**
- [ ] Feedback visual imediato (toggles)
- [ ] Mensagens de erro claras
- [ ] Timer preciso
- [ ] Não trava/crash durante chamada

**Código:**
- [ ] Commits no Git
- [ ] EXECUCAO.md atualizado
- [ ] Documentação completa
- [ ] Build passa sem warnings

---

**Última atualização:** 2026-01-20
**Status:** FASE 12 - 85% → 95%
