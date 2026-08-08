# Testes E2E de UI — Maestro (Android + iOS)

Flows black-box com [Maestro](https://maestro.mobile.dev): rodam contra o app
**real** em emulador/simulador, sem nenhuma alteração no código do app —
escolhido porque o `ZapLivreClientWrapper`/`ZapLivreCore` são singletons FFI
sem injeção de dependência (mockar exigiria refatoração).

## Estado de validação

| Plataforma | Flows | Estado |
|---|---|---|
| **iOS** | 8 (`01,02,03,04,05,07,08,09`) | **Executados e verdes** (8/8, ~2m25s) no simulador iPhone 17 / iOS 26.3 |
| **Android** | 10 (`01`–`10`) | **Executados e verdes** (10/10, ~5m53s) no device físico Samsung SM-X115 |

- iOS: suíte inteira roda verde no simulador (`maestro test e2e/maestro/ios/`).
  Os números 06 e 10 **não existem** no iOS (busca e galeria só vivem dentro da
  conversa 1:1 — ver "Limitações conhecidas com 1 device").
- Android: 10/10 verdes em device físico (Samsung SM-X115, arm64-v8a). Requer a
  stack local (ver "Android: stack local") com `adb reverse` das portas
  8083/8080/8081/8086 e o peer E2E seedado no identity server.

## Setup

```bash
# Instalar o Maestro
curl -Ls https://get.maestro.mobile.dev | bash

# Android: emulador rodando + APK instalado
cd android && ./gradlew installDebug

# iOS: simulador rodando + app instalado (ver pré-requisito de assinatura abaixo)
cd ios && ./build-all.sh
```

### Pré-requisito iOS: build assinado ad-hoc para o simulador

O `./build-all.sh` produz o app do dia a dia. **Para rodar os flows no
simulador**, porém, o app precisa de assinatura ad-hoc **com o entitlement de
Keychain**; sem isso o Keychain falha com o erro **-34018** no simulador, o app
não persiste a identidade e o backup falha (o app real assinado não tem esse
problema porque usa `application-identifier`).

Foi adicionado `keychain-access-groups` ao
`ios/ZapLivre/ZapLivre.dev.entitlements`. Buildar assim (**não** use
`CODE_SIGNING_ALLOWED=NO`):

```bash
xcodebuild -project ios/ZapLivre.xcodeproj -scheme ZapLivre \
  -sdk iphonesimulator -configuration Debug \
  -destination 'generic/platform=iOS Simulator' \
  -derivedDataPath build ARCHS=arm64 \
  CODE_SIGN_IDENTITY="-" CODE_SIGNING_REQUIRED=NO \
  CODE_SIGNING_ALLOWED=YES build
```

## Estrutura

### iOS — `e2e/maestro/ios/`

```
e2e/maestro/ios/
├── config.yaml              # suíte: "*.yml" da raiz; continueOnFailure
├── common/
│   └── _setup_identity.yml  # helper (não roda sozinho); clearState + onboarding
├── 01_onboarding_criar.yml
├── 02_navegacao_telas.yml
├── 03_criar_grupo.yml
├── 04_nova_conversa.yml
├── 05_backup_identidade.yml
├── 07_grupo_chat.yml
├── 08_group_info.yml
└── 09_settings_toggles.yml
```

### Android — `e2e/maestro/android/`

```
e2e/maestro/android/
├── config.yaml              # suíte: "*.yml" da raiz; continueOnFailure
├── common/
│   └── _setup_identity.yml  # helper (não roda sozinho); clearState + onboarding
├── 01_onboarding_criar.yml
├── 02_navegacao_telas.yml
├── 03_enviar_mensagem.yml
├── 04_backup_identidade.yml
├── 05_criar_grupo.yml
├── 06_busca.yml
├── 07_grupo_chat.yml
├── 08_group_info.yml
├── 09_settings_toggles.yml
└── 10_media_gallery.yml
```

- Em ambas as plataformas o `config.yaml` limita a execução aos flows numerados
  da raiz (`flows: ["*.yml"]` não desce em `common/`) e usa
  `executionOrder.continueOnFailure: true` — um flow quebrado não derruba a suíte.
- Os seletores usam `testTag` (Compose, via `testTagsAsResourceId`) / identifier
  (SwiftUI) expostos como resource-id, ex.: `id: "conversations_new_chat"` —
  imunes a mudança de texto/ícone.

> **Numeração**: iOS e Android **não** compartilham a mesma lista. O iOS não tem
> `06_busca` nem `10_media_gallery` (busca/galeria só existem dentro da conversa
> 1:1, inacessível com 1 device) e seu `04` é "nova conversa" em vez de "enviar
> mensagem". No Android a busca é **global** (a partir da lista de conversas),
> então `06_busca` é testável com 1 device — daí a diferença de cobertura.

### Setup por flow: `common/_setup_identity.yml` (novo comportamento)

Cada flow é **autossuficiente e cria identidade fresca** — não reusa mais
identidade entre flows. O helper agora:

1. dá `clearState` (parte do zero a cada flow);
2. `launchApp` com `permissions: notifications: allow` (pré-concede a
   permissão — o diálogo de notificações cobre o onboarding no primeiro launch);
   no iOS ainda há um `tapOn "Permitir"` **opcional**, porque o simulador dispara
   o diálogo mesmo assim;
3. espera o botão `onboarding_create` renderizar de forma **estável**
   (`extendedWaitUntil`, o init do FFI leva alguns segundos) antes de tocar;
4. toca em `onboarding_create` e espera "Conversas".

Isso substitui o modelo antigo (helper "idempotente que reusava identidade" e
"só o 01 usava clearState"). Agora **todos** os flows limpam o estado via o
helper; o `01_onboarding_criar` valida o onboarding em si.

## Rodando

```bash
# Suíte inteira de uma plataforma
maestro test e2e/maestro/ios/          # 8/8 verde no simulador
maestro test e2e/maestro/android/      # 10/10 verde no device físico

# Um flow isolado — funciona para QUALQUER flow, em qualquer ordem:
# todos são independentes (cada um faz clearState + cria identidade fresca)
maestro test e2e/maestro/android/07_grupo_chat.yml

# Checar sintaxe sem device
maestro check-syntax e2e/maestro/android/03_enviar_mensagem.yml
```

### Android: stack local

Os flows Android apontam o app para a stack **local** (`localhost` no device),
não para produção `*.zaplivre.app`. Para rodar:

```bash
# 1. APK com URLs locais (JAVA_HOME=Temurin 17; o Gradle 8.5 quebra com GraalVM)
cd android && ./gradlew assembleDebug \
  -Dorg.gradle.java.home=/Library/Java/JavaVirtualMachines/temurin-17.jdk/Contents/Home

# 2. Instalar + espelhar as portas do host para o device
adb -s R9XY7023XLL install -r app/build/outputs/apk/debug/app-debug.apk
adb -s R9XY7023XLL reverse tcp:8083 tcp:8083   # identity
adb -s R9XY7023XLL reverse tcp:8080 tcp:8080   # message-store
adb -s R9XY7023XLL reverse tcp:8081 tcp:8081   # push
adb -s R9XY7023XLL reverse tcp:8086 tcp:8086   # signaling

# 3. Stack local de pé (docker compose up -d) com rate limit desligado:
#    RATE_LIMIT_DISABLED=1 docker compose up -d identity-server
#    (o limite de 5/hora do /api/v1/register quebrava a suíte com 429)

# 4. Seedar o peer E2E dos flows 03/10 (envio para peer OFFLINE → Pending)
IDENTITY_SERVER_URL=http://localhost:8083 cargo run --example seed_peer
```

O `seed_peer` registra o username fixo `maestro_e2e_peer` no identity server com
o **peer ID libp2p real** (`12D3KooW…`) — registros com o ID de identidade
`zaplivre_…` não são parseáveis como `PeerId` e fazem o envio falhar. É
idempotente: se o username já existir, mantém o registro atual.

> **Independência de ordem**: nenhum flow depende de outro ter rodado antes —
> cada um recria a identidade via `common/_setup_identity.yml` (com `clearState`).

## Cobertura dos flows

### iOS (executados, 8/8 verde)

| Flow | O que valida | Regressão que pegaria |
|---|---|---|
| 01_onboarding_criar | Primeira execução → criar identidade → chegar em Conversas | corrida do auto-init; Keychain -34018 (identidade não persiste) |
| 02_navegacao_telas | Toda tela alcançável pela navegação (anti-órfã) | telas órfãs |
| 03_criar_grupo | Criar grupo → grupo aparece na lista | lista mockada vazia |
| 04_nova_conversa | NewChatView + tratamento de **peer offline** (erro de rede sem travar) | crash/UI sem feedback ao falhar dial (NÃO valida entrega) |
| 05_backup_identidade | Settings → exportar backup → conteúdo Base64 visível | backup inacessível; Keychain -34018 |
| 07_grupo_chat | Compositor de grupo (navegação, campo, botão enviar) sem crash | crash no GroupChatView (NÃO valida entrega — mensagem não persiste offline) |
| 08_group_info | GroupInfo acessível: nome + "Sair do grupo" visíveis | tela de info órfã/quebrada |
| 09_settings_toggles | Alternar switches de Settings sem crash | crash nos toggles |

### Android (executados, 10/10 verde)

| Flow | O que valida | Regressão que pegaria |
|---|---|---|
| 01_onboarding_criar | Primeira execução → criar identidade → chegar em Conversas | corrida do auto-init; diálogo de username inacessível |
| 02_navegacao_telas | Toda tela alcançável pela navegação (anti-órfã) | Settings/Search órfãs |
| 03_enviar_mensagem | Digitar + enviar → mensagem no chat **e input limpo** | fiação UI→FFI de envio; bundle de prekeys DTO não normalizado; envio para peer offline |
| 04_backup_identidade | Settings → exportar backup → conteúdo Base64 visível | backup inacessível |
| 05_criar_grupo | Criar grupo → grupo aparece na lista | lista mockada vazia |
| 06_busca | Busca **global** (a partir da lista) aceita query sem crashar | crash na tela de busca; texto de empty state errado |
| 07_grupo_chat | Criar grupo (nome único) → enviar mensagem em grupo | fiação de envio em grupo |
| 08_group_info | GroupInfo acessível: nome + "Sair do grupo" visíveis | tela de info órfã/quebrada |
| 09_settings_toggles | Alternar switches de Settings sem crash | crash nos toggles |
| 10_media_gallery | Galeria de mídia abre com empty state ("Nenhuma mídia") | crash/galeria órfã |

> **Diálogos no Android (Compose)**: o `AlertDialog` cria uma janela própria de
> acessibilidade e **não herda** o `testTagsAsResourceId` da raiz do
> `MainActivity`. Os testTags dentro de diálogos só resolvem porque o conteúdo
> de cada diálogo usado pelos flows aplica
> `Modifier.semantics { testTagsAsResourceId = true }`
> (`onboarding_username_input`/`onboarding_register`,
> `new_chat_peer_input`/`new_chat_confirm`,
> `grouplist_create_name_input`/`grouplist_create_confirm`).

## Limitações conhecidas com 1 device

Estes cenários **não são cobríveis com um único device** e vivem só em
`docs/guides/testing-manual.md` (roteiros de 2 dispositivos):

- **Envio de mensagem 1:1 com ENTREGA** — o peer seedado (`maestro_e2e_peer`)
  fica **offline**: o flow `03` valida que a mensagem é **persistida como
  Pending** (fica visível como bolha, input limpo), não a entrega em tempo real.
  A entrega de fato exige um segundo device com o app aberto.
- **Envio de mensagem em grupo com ENTREGA** — mesmo caso: o flow `07` valida
  que a mensagem própria **persiste localmente** na tabela do grupo (regressão
  4b), não a distribuição aos membros.
- **Recebimento de mensagem** e **chamadas (VoIP)** — exigem um segundo device
  enviando/ligando.
- **iOS: busca e galeria de mídia** — vivem **dentro da conversa 1:1**
  (inacessíveis com 1 device), por isso o iOS não tem `06_busca` nem
  `10_media_gallery`. No **Android** a busca é global (a partir da lista de
  conversas), então `06_busca` é testável solo — diferença entre plataformas.

## Lacunas conhecidas (sem flow de propósito)

- **ProfileScreen é órfã** (nenhuma rota de navegação chega nela) → sem flow
  de perfil/QR até ela ser ligada à navegação.
- **Toggles de Settings não persistem** (estado local do Composable) → o
  flow 09 só valida ausência de crash; não asserte persistência após reabrir.

## Notas

- Os textos usados nos flows são os literais atuais da UI (pt-BR/en misto);
  se a UI mudar textos, atualizar os flows — mas prefira sempre `testTag`/id.
- `hideKeyboard` **trava no simulador iOS 26**; onde tirar o foco do campo era
  necessário, foi substituído por tocar num label estático da tela.
- Flows 03 (Android) usam `env.PEER_ID` placeholder; para teste completo passe
  um peer real: `maestro test -e PEER_ID=12D3KooW... 03_enviar_mensagem.yml`.
- Flows 07/08 geram nome de grupo único (`Date.now()`), então podem rodar
  repetidamente sem colidir com grupos de execuções anteriores.
- iOS: os flows assumem o bundle id `app.zaplivre.ios`; Android `com.zaplivre`.

## Bug de produto observado (envio offline)

Descoberto durante a validação iOS, vale registrar (relacionado ao item 10 do
`ISSUES_BACKLOG.md`, "Outbound message retry/queue"):

- **O envio não persiste a mensagem localmente antes de distribuir.** Com o peer
  offline, a mensagem enviada (1:1 e grupo) **não aparece** no SQLite do app
  (tabela `messages` vazia). O esperado seria persistir localmente primeiro e só
  então tentar distribuir/enfileirar para retry.
- **`GroupChatView.sendMessage` engole o erro no `catch`** — sem nenhum feedback
  ao usuário quando o envio falha. Faltam estado de erro/indicação de "Pendente".

Enquanto isso, o envio ponta-a-ponta só é validável com 2 devices
(`docs/guides/testing-manual.md`).
