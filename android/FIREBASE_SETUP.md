# Firebase Setup - MePassa Android

Este documento explica como configurar Firebase Cloud Messaging (FCM) para notificações push no app Android.

## 📋 Pré-requisitos

- Conta Google
- Acesso ao [Firebase Console](https://console.firebase.google.com/)

---

## 🔧 Passo a Passo

### 1. Criar Projeto Firebase

1. Acesse [Firebase Console](https://console.firebase.google.com/)
2. Clique em **"Add project"** (Adicionar projeto)
3. Nome do projeto: **MePassa** (ou outro nome de sua preferência)
4. (Opcional) Desabilite Google Analytics se não for usar
5. Clique em **"Create project"**

### 2. Adicionar App Android ao Projeto

1. No painel do projeto Firebase, clique no ícone **Android** (robot icon)
2. Preencha os dados:
   - **Android package name:** `com.mepassa` (IMPORTANTE: deve ser exatamente este)
   - **App nickname:** MePassa Android (opcional)
   - **Debug signing certificate SHA-1:** (opcional, pode pular por enquanto)
3. Clique em **"Register app"**

### 3. Download google-services.json

1. Após registrar o app, clique em **"Download google-services.json"**
2. Salve o arquivo no diretório:
   ```
   /Users/edsonmartins/desenvolvimento/mepassa/android/app/google-services.json
   ```
3. **IMPORTANTE:** Este arquivo contém credenciais do Firebase e NÃO deve ser commitado no Git
   - Já está no `.gitignore`
   - Guarde uma cópia segura em local privado

### 4. Verificar Instalação

O projeto já possui as dependências Firebase instaladas:
- ✅ `build.gradle.kts` (raiz) - Plugin google-services configurado
- ✅ `app/build.gradle.kts` - Firebase BoM e firebase-messaging-ktx adicionados
- ✅ `AndroidManifest.xml` - FirebaseMessagingService registrado
- ✅ `MePassaFirebaseMessagingService.kt` - Service criado
- ✅ `NotificationHelper.kt` - Helper de notificações criado

Você só precisa adicionar o `google-services.json` conforme passo 3.

### 5. Obter Server Key (para Push Server)

1. No Firebase Console, vá em **Project Settings** (engrenagem no topo) → **Cloud Messaging**
2. Na seção **"Cloud Messaging API (Legacy)"**, copie o **Server key**
3. Salve esta chave - será usada na ETAPA 3 (Push Server)
   - Formato: `AAAAxxxxxxx:xxxxxxxxxxxxxxxxxxxxxxxxxxxxx`

**NOTA:** Se não aparecer "Cloud Messaging API (Legacy)":
1. Procure por **"Cloud Messaging API"** (sem Legacy)
2. Se necessário, habilite a API clicando em **"Manage API in Google Cloud Console"**
3. Após habilitar, volte ao Firebase Console e pegue a chave

---

## 🧪 Testar Configuração

### Teste Manual (via Firebase Console)

1. Build e instale o app no dispositivo/emulador:
   ```bash
   cd /Users/edsonmartins/desenvolvimento/mepassa/android
   ./gradlew installDebug
   ```

2. Abra o app pelo menos uma vez (para gerar o FCM token)

3. Veja o token nos logs do Logcat:
   ```bash
   adb logcat | grep FCM
   ```
   Procure por: `New FCM token received: ...`

4. No Firebase Console, vá em **Cloud Messaging** → **Send your first message**

5. Preencha:
   - **Notification title:** Teste
   - **Notification text:** Mensagem de teste
   - **Target:** App: com.mepassa (MePassa Android)

6. Clique em **"Send test message"**

7. Cole o FCM token que apareceu no Logcat

8. Clique em **"Test"**

9. Você deve receber a notificação no dispositivo

---

## 📊 Estrutura de Arquivos

```
android/
├── app/
│   ├── google-services.json          ← VOCÊ PRECISA ADICIONAR ESTE ARQUIVO
│   ├── build.gradle.kts               ✅ Firebase dependencies adicionadas
│   └── src/main/
│       ├── AndroidManifest.xml        ✅ FCM Service registrado
│       └── kotlin/com/mepassa/
│           ├── service/
│           │   ├── MePassaFirebaseMessagingService.kt  ✅ Criado
│           │   └── MePassaService.kt
│           └── util/
│               └── NotificationHelper.kt  ✅ Criado
└── build.gradle.kts                   ✅ google-services plugin adicionado
```

---

## 🚨 Troubleshooting

### Erro: "google-services.json is missing"

**Solução:** Baixe o arquivo conforme Passo 3 acima.

### Erro: "Default FirebaseApp is not initialized"

**Causa:** `google-services.json` não foi encontrado ou está corrompido.

**Solução:**
1. Verifique se o arquivo está em `android/app/google-services.json`
2. Re-baixe o arquivo do Firebase Console
3. Clean e rebuild: `./gradlew clean build`

### Token FCM não aparece nos logs

**Solução:**
1. Certifique-se que o app está rodando (não apenas instalado)
2. Verifique permissão de notificações (Android 13+)
3. Check logs com: `adb logcat | grep -i firebase`

### Notificações não chegam

**Checklist:**
- [ ] `google-services.json` está presente
- [ ] App está instalado e foi aberto pelo menos uma vez
- [ ] Token FCM foi gerado (veja logs)
- [ ] Server key do Firebase está correto
- [ ] Notificação foi enviada para o token correto

---

## 📝 Próximos Passos

1. ✅ ETAPA 2 completa - FCM configurado
2. ⏳ ETAPA 3 - Push Server (Rust + Axum)
3. ⏳ ETAPA 4 - Integration (Android → Push Server)

---

**Última atualização:** 2026-01-20
**Versão do Firebase BoM:** 32.7.0
