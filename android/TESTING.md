# Guia de Teste - MePassa Android App

**Data:** 2025-01-20
**Versão:** 0.1.0-alpha

Este guia documenta passo a passo como compilar e testar o app Android MePassa.

## 📋 Pré-requisitos

### 1. Verificar Instalações

```bash
# Java (deve ser JDK 17)
java -version
# Esperado: openjdk version "17.0.x"

# Android Studio
# Versão mínima: Hedgehog (2023.1.1)
# Verificar em: Android Studio > About Android Studio

# Android SDK
# Deve ter API 34 instalado
# Verificar em: Android Studio > Settings > Android SDK
```

### 2. Verificar Arquivos Necessários

```bash
cd /Users/edsonmartins/desenvolvimento/mepassa/android

# 1. Biblioteca nativa (CRÍTICO)
ls -lh app/src/main/jniLibs/arm64-v8a/libmepassa_core.so
# Esperado: 6.3MB

# 2. Bindings UniFFI (CRÍTICO)
ls -lh app/src/main/kotlin/uniffi/mepassa/mepassa.kt
# Esperado: 80KB

# 3. Gradle wrapper
ls -lh gradle/wrapper/gradle-wrapper.properties
# Esperado: arquivo existe

# 4. Build config
ls -lh build.gradle.kts app/build.gradle.kts
# Esperado: ambos existem
```

Se algum arquivo estiver faltando, execute:
```bash
cd ../core
cargo run --example generate_bindings
cp target/aarch64-linux-android/release/libmepassa_core.so \
   ../android/app/src/main/jniLibs/arm64-v8a/
```

## 🚀 Passo 1: Abrir Projeto no Android Studio

### Método 1: Via Interface

1. Abrir Android Studio
2. `File > Open...`
3. Navegar até: `/Users/edsonmartins/desenvolvimento/mepassa/android`
4. Selecionar a pasta `android/` e clicar em `Open`

### Método 2: Via Terminal

```bash
cd /Users/edsonmartins/desenvolvimento/mepassa/android
open -a "Android Studio" .
```

### O que Esperar

Quando o projeto abrir, você verá:

```
✅ Gradle sync iniciando automaticamente (barra de progresso no topo)
✅ Indexing files (barra de progresso no rodapé)
✅ Building Gradle project info...
```

**Tempo estimado:** 2-5 minutos (primeira vez)

### Possíveis Mensagens

#### Mensagem 1: "Gradle Sync Failed"
```
Causa: Gradle wrapper não configurado
Solução: Clique em "Download Gradle 8.5"
```

#### Mensagem 2: "SDK not found"
```
Causa: Android SDK 34 não instalado
Solução:
1. File > Settings > Android SDK
2. Marcar "Android 14.0 (API 34)"
3. Clicar em Apply > OK
```

#### Mensagem 3: "JDK version incompatible"
```
Causa: JDK não é versão 17
Solução:
1. File > Project Structure
2. SDK Location > Gradle Settings
3. Gradle JDK: selecionar JDK 17
```

## 🔧 Passo 2: Configurar Gradle Sync

### Executar Sync Manualmente

```
Menu: File > Sync Project with Gradle Files
Ou: Clique no ícone de elefante (Gradle) no topo
```

### Verificar Output

Abra a janela "Build" (Alt+1 ou Cmd+1):

```
✅ BUILD SUCCESSFUL in 45s
✅ 22 modules resolved

Ou se houver erros:
❌ BUILD FAILED
   > Task :app:compileDebugKotlin FAILED
```

### Se Sync Falhar

**Erro comum 1: "Could not resolve dependencies"**
```bash
# Limpar cache do Gradle
cd android
./gradlew clean
./gradlew --refresh-dependencies
```

**Erro comum 2: "Kotlin compiler version mismatch"**
```
Solução: Verificar build.gradle.kts raiz:
plugins {
    id("org.jetbrains.kotlin.android") version "1.9.21" apply false
}
```

## 📱 Passo 3: Configurar Emulador

### Opção A: Criar Novo AVD (Android Virtual Device)

1. Abrir AVD Manager:
   ```
   Menu: Tools > Device Manager
   Ou: Ícone de celular no topo
   ```

2. Clicar em `Create Device`

3. Selecionar Hardware:
   ```
   Categoria: Phone
   Device: Pixel 6 (recomendado)
   Clicar em: Next
   ```

4. Selecionar System Image:
   ```
   Release: Android 14.0 (API 34)
   ABI: arm64-v8a (IMPORTANTE!)
   Target: Google APIs

   Se não estiver baixado:
   - Clicar em "Download" ao lado
   - Aguardar download (~1GB)

   Clicar em: Next
   ```

5. Configurar AVD:
   ```
   AVD Name: MePassa_Test
   Startup orientation: Portrait

   Avançado (opcional):
   - RAM: 2048 MB (mínimo)
   - VM heap: 512 MB
   - Internal Storage: 2048 MB

   Clicar em: Finish
   ```

### Opção B: Usar Dispositivo Real

#### Android 8.0 - 13 (API 26-33)

1. No dispositivo:
   ```
   Settings > About Phone
   Toque 7 vezes em "Build Number"

   Settings > Developer Options
   ✅ Enable "USB Debugging"
   ✅ Enable "Stay Awake" (opcional)
   ```

2. Conectar via USB:
   ```bash
   # Verificar se dispositivo aparece
   adb devices

   # Esperado:
   # List of devices attached
   # SERIAL123456    device

   # Se "unauthorized":
   # - Autorizar no dispositivo (popup)
   # - Executar: adb devices novamente
   ```

#### Android 14+ (API 34)

Mesmos passos, mas também:
```
Settings > Developer Options
✅ Enable "Wireless Debugging" (opcional para USB-free)
```

## ▶️ Passo 4: Executar o App

### 4.1 Selecionar Target

No topo do Android Studio:
```
┌─────────────────────┬──────────────────┐
│ app                 │ Pixel 6 API 34   │ ▶️
│ (módulo)            │ (device)         │
└─────────────────────┴──────────────────┘
```

Clicar na dropdown do device:
- Selecionar seu AVD criado OU
- Selecionar dispositivo físico conectado

### 4.2 Iniciar Build & Run

**Método 1: Via Botão**
```
Clicar no botão verde ▶️ (Run)
Ou: Shift+F10 (Windows/Linux)
Ou: Ctrl+R (macOS)
```

**Método 2: Via Terminal**
```bash
cd android

# Build + Install
./gradlew installDebug

# Ou apenas build (sem instalar)
./gradlew assembleDebug
```

### 4.3 Acompanhar Build

Janela "Build" mostrará:

```
> Task :app:preBuild
> Task :app:compileDebugKotlin
> Task :app:mergeDebugJniLibFolders       ← IMPORTANTE (copia .so)
> Task :app:packageDebug
> Task :app:assembleDebug
> Task :app:installDebug

BUILD SUCCESSFUL in 1m 23s
89 actionable tasks: 89 executed
```

**Tempo estimado:**
- Primeira build: 2-4 minutos
- Builds subsequentes: 30-60 segundos

### 4.4 Verificar Instalação

```bash
# Verificar se APK foi instalado
adb shell pm list packages | grep mepassa
# Esperado: package:com.mepassa

# Verificar se biblioteca nativa foi copiada
adb shell run-as com.mepassa ls -l /data/data/com.mepassa/lib/
# Esperado: libmepassa_core.so
```

## 🧪 Passo 5: Testar Funcionalidades

### Teste 1: Onboarding (Primeira Execução)

#### O que Deve Acontecer:

1. **App Abre**
   - Splash screen (breve)
   - Tela de Onboarding aparece

2. **Tela Visível:**
   ```
   ┌─────────────────────────────┐
   │         [ MP ]              │  ← Logo placeholder
   │                             │
   │   Bem-vindo ao MePassa      │
   │ Mensagens privadas e        │
   │   seguras via P2P           │
   │                             │
   │  [ Começar ]                │  ← Botão
   └─────────────────────────────┘
   ```

3. **Clicar em "Começar":**
   - Loading spinner aparece
   - Texto muda para "Gerando identidade…"

4. **Após 1-2 segundos:**
   - Card aparece mostrando:
     ```
     Seu Peer ID:
     12D3KooWABC123... (truncado)
     ```

5. **Navegação Automática (após 1.5s):**
   - Tela de Conversas aparece

#### Verificar nos Logs:

```bash
adb logcat | grep MePassa
```

**Logs esperados:**
```
MePassaApplication: Native library loaded successfully
MePassaApplication: MePassa Application created
MainActivity: MainActivity created
MainActivity: MePassaClient initialized successfully
MainActivity: Starting MePassaService
MePassaService: Service created
MePassaService: Initializing MePassaClient from service
MePassaClientWrapper: Initializing MePassaClient with dataDir: /data/user/0/com.mepassa/files/mepassa_data
MePassaClientWrapper: Client initialized successfully. PeerId: 12D3KooW...
MePassaService: Starting P2P listener
MePassaService: Starting bootstrap
```

#### Se Falhar:

**Erro 1: "UnsatisfiedLinkError"**
```
Causa: libmepassa_core.so não foi incluída
Verificar:
adb shell run-as com.mepassa ls -l /data/data/com.mepassa/lib/

Solução:
1. Clean build: Build > Clean Project
2. Rebuild: Build > Rebuild Project
3. Verificar jniLibs: ls app/src/main/jniLibs/arm64-v8a/
```

**Erro 2: "Failed to initialize client"**
```
Logs:
MePassaClientWrapper: Failed to initialize client
java.lang.RuntimeException: ...

Possíveis causas:
1. Permissões de storage negadas
2. Diretório de dados inacessível

Solução:
adb shell pm clear com.mepassa  # Limpa dados do app
Executar app novamente
```

### Teste 2: Notificação de Foreground Service

#### O que Verificar:

1. **Barra de Notificação:**
   - Puxar barra de notificações
   - Deve ter notificação:
     ```
     MePassa
     Conectado - 0 peers
     ```

2. **Atualização Dinâmica:**
   - Aguardar 10 segundos
   - Contagem de peers pode mudar (se houver bootstrap)

#### Verificar nos Logs:

```bash
adb logcat | grep MePassaService
```

**Logs esperados (a cada 10s):**
```
MePassaService: Connected peers: 0
MePassaService: Connected peers: 0
...
```

### Teste 3: Tela de Conversas

#### O que Deve Aparecer:

```
┌─────────────────────────────┐
│  Conversas              [+] │  ← TopBar + FAB
├─────────────────────────────┤
│                             │
│   Nenhuma conversa ainda    │  ← Estado vazio
│                             │
└─────────────────────────────┘
```

#### Teste: Adicionar Nova Conversa

1. **Clicar no FAB (+):**
   - Dialog aparece:
     ```
     ┌─────────────────────────┐
     │ Nova conversa           │
     │                         │
     │ ┌─────────────────────┐ │
     │ │ Peer ID             │ │  ← TextField
     │ │ 12D3KooW...         │ │
     │ └─────────────────────┘ │
     │                         │
     │  [Cancelar]  [OK]       │
     └─────────────────────────┘
     ```

2. **Inserir Peer ID:**
   - Copiar seu próprio Peer ID (da tela de onboarding)
   - OU usar um Peer ID fictício: `12D3KooWTest123`

3. **Clicar em OK:**
   - Dialog fecha
   - Navega para ChatScreen

### Teste 4: Tela de Chat

#### O que Deve Aparecer:

```
┌─────────────────────────────┐
│ ← 12D3KooWTest1...          │  ← TopBar com peer ID
│   Online                    │
├─────────────────────────────┤
│                             │
│ Nenhuma mensagem ainda.     │  ← Estado vazio
│ Envie a primeira!           │
│                             │
├─────────────────────────────┤
│ ┌─────────────────────────┐ │
│ │ Mensagem…               │ │  ← Input field
│ └─────────────────────────┘ │
│                        [▶]  │  ← Send button
└─────────────────────────────┘
```

#### Teste: Enviar Mensagem

1. **Digitar Mensagem:**
   - Clicar no campo de texto
   - Digitar: "Teste de mensagem"

2. **Clicar em Send (▶):**
   - Loading spinner aparece brevemente
   - Mensagem aparece na lista:
     ```
     ┌────────────────────┐
     │ Teste de mensagem  │  ← Bubble azul (direita)
     │ 10:45              │  ← Timestamp
     └────────────────────┘
     ```

3. **Verificar Estado:**
   - Campo de input limpa automaticamente
   - Mensagem fica alinhada à direita (mensagem própria)
   - Timestamp mostra hora atual

#### Verificar nos Logs:

```bash
adb logcat | grep -E "(MePassaClientWrapper|ChatScreen)"
```

**Logs esperados:**
```
MePassaClientWrapper: Sending text message to: 12D3KooWTest123
MePassaClientWrapper: Message sent successfully: <message_id>
```

### Teste 5: Voltar para Conversas

1. **Clicar no botão Voltar (←):**
   - Retorna para ConversationsScreen

2. **Verificar Lista:**
   - Deve aparecer a conversa criada:
     ```
     ┌─────────────────────────┐
     │ 12D3KooWTest1...   10:45│  ← Conversa
     │ 12D3KooWTest1...        │  ← Peer ID
     └─────────────────────────┘
     ```

3. **Clicar na Conversa:**
   - Retorna para ChatScreen
   - Mensagens enviadas anteriormente aparecem

### Teste 6: App em Background

#### Teste: Minimizar App

1. **Pressionar Home:**
   - App vai para background

2. **Verificar Notificação:**
   - Notificação "MePassa - Conectado" permanece
   - Service continua rodando

3. **Verificar Logs:**
   ```bash
   adb logcat | grep MePassaService

   # Esperado: Logs a cada 10s continuam
   MePassaService: Connected peers: 0
   ```

#### Teste: Retornar ao App

1. **Abrir Recent Apps:**
   - Selecionar MePassa

2. **Verificar Estado:**
   - App retorna na última tela (Chat ou Conversas)
   - Estado preservado (mensagens ainda visíveis)

### Teste 7: Fechar e Reabrir App

#### Teste: Force Stop

```bash
# Forçar parada do app
adb shell am force-stop com.mepassa
```

#### Reabrir App:

1. **Clicar no ícone do app**

2. **O que Deve Acontecer:**
   - Pula onboarding (já inicializado)
   - Vai direto para ConversationsScreen
   - Conversas criadas anteriormente aparecem
   - Service reinicia automaticamente

3. **Verificar Logs:**
   ```bash
   adb logcat -c  # Limpar logs
   adb logcat | grep MePassa

   # Esperado:
   MainActivity: MePassaClient initialized successfully  ← Client já existe
   MePassaService: Service created
   ```

## 📊 Checklist de Validação

### ✅ Build & Deploy
- [ ] Gradle sync sem erros
- [ ] Build successful
- [ ] APK instalado no device/emulador
- [ ] App abre sem crash

### ✅ Onboarding
- [ ] Tela de onboarding aparece (primeira vez)
- [ ] Botão "Começar" funciona
- [ ] Loading spinner aparece
- [ ] Peer ID é gerado e exibido
- [ ] Navegação automática para Conversas funciona

### ✅ Biblioteca Nativa
- [ ] Log "Native library loaded successfully" aparece
- [ ] libmepassa_core.so está em /data/data/.../lib/
- [ ] MePassaClient inicializa sem erros

### ✅ Foreground Service
- [ ] Service inicia automaticamente
- [ ] Notificação aparece
- [ ] Contagem de peers exibida (0 no início)
- [ ] Service continua após minimizar app

### ✅ Conversas
- [ ] Tela de conversas aparece
- [ ] FAB (+) funciona
- [ ] Dialog de nova conversa abre
- [ ] Input de Peer ID aceita texto
- [ ] Navegação para chat funciona

### ✅ Chat
- [ ] Tela de chat abre
- [ ] Campo de input funciona
- [ ] Send button funciona
- [ ] Mensagem aparece na lista
- [ ] Message bubble formatada corretamente
- [ ] Timestamp exibido

### ✅ Navegação
- [ ] Botão voltar funciona
- [ ] Estado preservado ao navegar
- [ ] Deep linking funciona (chat/{peerId})

### ✅ Persistência
- [ ] Dados salvos após fechar app
- [ ] Conversas aparecem ao reabrir
- [ ] Mensagens preservadas
- [ ] Peer ID não muda

### ✅ Permissões
- [ ] Permissão de notificação solicitada (Android 13+)
- [ ] App funciona sem permissão de notificação
- [ ] Permissões de rede concedidas automaticamente

## 🐛 Troubleshooting

### Problema 1: Build Falha

**Sintoma:**
```
BUILD FAILED in 10s
Execution failed for task ':app:compileDebugKotlin'
```

**Soluções:**
```bash
# 1. Limpar build
cd android
./gradlew clean
./gradlew build

# 2. Invalidar cache do Android Studio
# File > Invalidate Caches > Invalidate and Restart

# 3. Verificar JDK
# File > Project Structure > SDK Location
# Garantir JDK 17

# 4. Recriar projeto
rm -rf .gradle build app/build
./gradlew build
```

### Problema 2: App Crasha ao Abrir

**Sintoma:**
```
App abre por 1 segundo e fecha
```

**Verificar Logs:**
```bash
adb logcat | grep AndroidRuntime
```

**Erros comuns:**

**A) UnsatisfiedLinkError**
```
java.lang.UnsatisfiedLinkError: couldn't find libmepassa_core.so
```
Solução:
```bash
# Verificar se .so existe
ls app/src/main/jniLibs/arm64-v8a/libmepassa_core.so

# Se não existir, copiar novamente
cp ../core/target/aarch64-linux-android/release/libmepassa_core.so \
   app/src/main/jniLibs/arm64-v8a/

# Rebuild
./gradlew clean build
```

**B) ClassNotFoundException (UniFFI)**
```
java.lang.ClassNotFoundException: uniffi.mepassa.MePassaClient
```
Solução:
```bash
# Verificar se bindings existem
ls app/src/main/kotlin/uniffi/mepassa/mepassa.kt

# Se não existir
cp ../core/target/bindings/uniffi/mepassa/mepassa.kt \
   app/src/main/kotlin/uniffi/mepassa/

# Sync Gradle
```

### Problema 3: Service Não Inicia

**Sintoma:**
```
Notificação não aparece
```

**Verificar:**
```bash
adb logcat | grep MePassaService

# Se nenhum log aparece:
# - Service não foi registrado no AndroidManifest
# - Permissão FOREGROUND_SERVICE faltando
```

**Solução:**
```xml
<!-- Verificar AndroidManifest.xml -->
<service
    android:name=".service.MePassaService"
    android:enabled="true"
    android:exported="false"
    android:foregroundServiceType="dataSync" />
```

### Problema 4: Mensagens Não Enviam

**Sintoma:**
```
Clicar em Send não faz nada
```

**Verificar Logs:**
```bash
adb logcat | grep MePassaClientWrapper
```

**Erros comuns:**

**A) Client não inicializado**
```
IllegalStateException: Client not initialized
```
Solução: Aguardar onboarding completar

**B) Erro de rede**
```
MePassaFfiError.Network: Failed to send message
```
Causa: Peer ID inválido ou peer offline

### Problema 5: Emulador Lento

**Sintoma:**
```
App demora muito para responder
```

**Otimizações:**

```
AVD Manager > Edit AVD:
- Graphics: Hardware (não Software)
- RAM: Aumentar para 4096 MB
- VM heap: 1024 MB
- Use Host GPU: ✅
```

Ou use dispositivo físico (sempre mais rápido).

## 📸 Screenshots Esperados

Vou documentar como devem ficar as telas:

### Onboarding
```
┌─────────────────────────────┐
│          🎨 MP              │
│                             │
│  Bem-vindo ao MePassa       │
│                             │
│ Mensagens privadas e        │
│ seguras via P2P             │
│                             │
│ ┌─────────────────────────┐ │
│ │ Seu Peer ID:            │ │
│ │ 12D3KooWABC...          │ │
│ └─────────────────────────┘ │
│                             │
│     [ Começar ]             │
└─────────────────────────────┘
```

### Conversas (vazia)
```
┌─────────────────────────────┐
│  Conversas               [+]│
├─────────────────────────────┤
│                             │
│                             │
│  Nenhuma conversa ainda     │
│                             │
│                             │
└─────────────────────────────┘
```

### Conversas (com dados)
```
┌─────────────────────────────┐
│  Conversas               [+]│
├─────────────────────────────┤
│ 12D3KooWTest1...       10:45│
│ 12D3KooWTest1...            │
├─────────────────────────────┤
│ 12D3KooWABC...         Agora│
│ 12D3KooWABC...              │
└─────────────────────────────┘
```

### Chat
```
┌─────────────────────────────┐
│ ← 12D3KooWTest1...          │
│   Online                    │
├─────────────────────────────┤
│                             │
│          ┌────────────────┐ │
│          │ Olá!           │ │ ← Recebida
│          │ 10:30          │ │
│          └────────────────┘ │
│                             │
│ ┌────────────────┐          │
│ │ Teste!         │          │ ← Enviada
│ │ 10:45          │          │
│ └────────────────┘          │
│                             │
├─────────────────────────────┤
│ ┌─────────────────────────┐ │
│ │ Mensagem…               │ │
│ └─────────────────────────┘▶│
└─────────────────────────────┘
```

## 📝 Relatório de Teste

Após executar todos os testes, preencher:

```
# Relatório de Teste - MePassa Android

Data: ___/___/______
Testador: ________________
Device/Emulador: ______________
Android Version: ______________

## Resultados

Build:
[ ] ✅ Passou  [ ] ❌ Falhou
Observações: ________________________________

Onboarding:
[ ] ✅ Passou  [ ] ❌ Falhou
Observações: ________________________________

Conversas:
[ ] ✅ Passou  [ ] ❌ Falhou
Observações: ________________________________

Chat:
[ ] ✅ Passou  [ ] ❌ Falhou
Observações: ________________________________

Service:
[ ] ✅ Passou  [ ] ❌ Falhou
Observações: ________________________________

Persistência:
[ ] ✅ Passou  [ ] ❌ Falhou
Observações: ________________________________

## Bugs Encontrados

1. ________________________________________
2. ________________________________________
3. ________________________________________

## Performance

Tempo de build: _______ segundos
Tempo de startup: _______ segundos
Uso de RAM: _______ MB
Tamanho APK: _______ MB

## Conclusão

[ ] ✅ Aprovado para produção
[ ] ⚠️  Aprovado com ressalvas
[ ] ❌ Reprovado - necessita correções

Assinatura: _____________________
```

---

**Próximo passo:** Executar os testes e documentar os resultados!
