# C5 — Chaves de Push: APNs (iOS) + FCM (Android)

Guia de geração e provisionamento das chaves **reais** para validar entrega de
push em device físico (fecha o item **C5** do `PLANO_HOMOLOGACAO.md`).

Tudo se resume a **dois arquivos** no servidor local e **três env vars**:

| Destino | O que | Onde fica |
|---|---|---|
| APNs (iOS) | `apns_key.p8` + `APNS_KEY_ID` + `APNS_TEAM_ID` | `server/push/certs/apns_key.p8` + `.env` |
| FCM (Android) | `fcm_service_account.json` | `server/push/certs/fcm_service_account.json` |
| Android app | `google-services.json` | `android/app/google-services.json` |

> O compose monta `./server/push/certs:/etc/push:ro`, então os dois arquivos
> precisam estar nessa pasta **antes** do `make up`. O push server lê o JSON
> do service account (FCM HTTP v1 — PSH-01) e o `.p8` (APNs token-based).

---

## 1) APNs (iOS) — ~10 min

### 1.1 Criar a Auth Key (.p8)

1. [developer.apple.com/account](https://developer.apple.com/account) → **Certificates, Identifiers & Profiles**.
2. **Keys** → **+** (novo).
3. Nome: `ZapLivre APNs`.
4. Marcar **Apple Push Notifications service (APNs)**.
5. **Continue → Register → Download**.
   - ⚠️ Só dá para baixar **uma vez**.
   - Arquivo: `AuthKey_XXXXXXXXXX.p8` (chave EC P-256).

### 1.2 Anotar os IDs

- **Key ID** (10 chars) — na página da key.
- **Team ID** (10 chars) — **Membership Details** (topo da página).
- **Bundle ID** do app: `app.zaplivre.ios` (já fixo no `ios/project.yml:78`).

### 1.3 Colocar a chave no repo

```bash
mkdir -p server/push/certs
cp ~/Downloads/AuthKey_XXXXXXXXXX.p8 server/push/certs/apns_key.p8
chmod 600 server/push/certs/apns_key.p8
```

### 1.4 Preencher `.env`

```bash
# .env
APNS_KEY_ID=XXXXXXXXXX
APNS_TEAM_ID=XXXXXXXXXX
APNS_BUNDLE_ID=app.zaplivre.ios
APNS_PRODUCTION=false   # dev/sandbox; true só para App Store
```

---

## 2) FCM (Android) — ~15 min

### 2.1 Criar o projeto Firebase

1. [console.firebase.google.com](https://console.firebase.google.com) → **Add project** (pode reusar o projeto do ZapLivre).
2. Android app → **Add app**:
   - **Package name:** `com.zaplivre` (exato — `android/app/build.gradle.kts:16`).
   - **Download `google-services.json`** → salvar em `android/app/google-services.json`.
3. Console → **Project settings** → **Cloud Messaging** → conferir se o app aparece.

### 2.2 Service Account (chave do servidor)

1. Console → ⚙️ **Project settings** → **Service accounts**.
2. **Generate new private key** → baixa um JSON.
3. Salvar como `server/push/certs/fcm_service_account.json`.
   - O push server usa os campos `project_id`, `client_email`, `private_key`
     (`server/push/src/fcm.rs`) para gerar o JWT e chamar a HTTP v1.

```bash
cp ~/Downloads/zaplivre-firebase-adminsdk-XXXX.json \
   server/push/certs/fcm_service_account.json
chmod 600 server/push/certs/fcm_service_account.json
```

---

## 3) Subir a stack e validar

```bash
DOCKER_HOST="unix://$HOME/.colima/default/docker.sock" \
PUSH_SERVICE_SECRET="$(openssl rand -base64 24)" \
RATE_LIMIT_DISABLED=1 \
docker compose up -d --build --force-recreate push-server

docker logs zaplivre-push 2>&1 | grep -iE "apns|fcm|push"
```

**Esperado nos logs:**
```
✅ FCM client enabled (project: zaplivre-xxxx)
✅ APNs client enabled (bundle: app.zaplivre.ios, production: false)
```

Se aparecer "FCM disabled" / "APNs disabled", o arquivo não foi encontrado em
`/etc/push` (rever o mount `./server/push/certs:/etc/push:ro`).

### 3.1 Endpoint de teste direto

```bash
curl -s -X POST http://localhost:8081/api/v1/send \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $PUSH_SERVICE_SECRET" \
  -d '{"peer_id":"12D3KooW...","title":"C5 test","body":"hello","data":{}}'
```

---

## 4) Validação em device físico

### Android (`com.zaplivre`)

1. Rebuild com `google-services.json` presente (sem ele, o FCM é desabilitado no build — AND-12).
2. `adb reverse` na porta `8081` + instalar o APK.
3. Logs: `📱 FCM token obtained` → `✅ FCM token registered with Push Server`.

### iOS (`app.zaplivre.ios`)

1. `make up` local + APK/device no mesmo Wi-Fi (ou `adb reverse`/túnel).
2. Aceitar permissão de push; logs: `🍎 APNs device token` → `✅ registered`.
3. Enviar pelo endpoint acima e conferir o banner.

> **Sandbox × produção (BadDeviceToken):** build via Xcode/TestFlight usa o
> endpoint sandbox → `APNS_PRODUCTION=false`. Build via App Store usa produção.

---

## 5) Checklist C5

- [ ] `server/push/certs/apns_key.p8` presente
- [ ] `.env` com `APNS_KEY_ID`, `APNS_TEAM_ID`, `APNS_BUNDLE_ID=app.zaplivre.ios`
- [ ] `server/push/certs/fcm_service_account.json` presente
- [ ] `android/app/google-services.json` presente
- [ ] Logs do push: `APNs client enabled` + `FCM client enabled`
- [ ] Push recebido em **device iOS físico**
- [ ] Push recebido em **device Android físico**
- [ ] `.p8` e JSON **fora** do git (no `.gitignore` de `server/push/certs/`)

---

## 6) Segurança

```bash
# NUNCA commitar as chaves (`.gitignore` já cobre .p8 e fcm_service_account.json)
chmod 600 server/push/certs/*.p8 server/push/certs/*.json
```

Rotações: APNs a cada 12 meses (revogar a antiga no portal Apple); FCM pode
gerar novo service account a qualquer momento sem derrubar o app.

---

**Referências:** `docs/APNS_SETUP_GUIDE.md` (detalhes APNs), `docs/guides/push-checklist.md`, `server/push/src/fcm.rs` (PSH-01, HTTP v1), `server/push/src/apns.rs`.
