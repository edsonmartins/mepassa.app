# ZapLivre — Levantamento de UI/UX e Plano de Rebranding

**Data:** 2026-08-09
**Objetivo:** tornar o ZapLivre visualmente competitivo com o WhatsApp (não business), aplicando o logo nas telas e adotando o tema navy + âmbar definido nos protótipos (`documentos/zaplivre-{desktop,mobile}-prototype.html` + `zaplivre-paleta-cores.md`).

---

## 1. Identidade atual × Identidade desejada

| Aspecto | Hoje (código) | Desejado (protótipos/paleta) |
|---|---|---|
| Primária | **Azul `#2F6BFF`** (Android/iOS `Color.kt`/`ZapTheme.swift`) e **Teal `#00af93`** (Desktop Tailwind) — **3 identidades diferentes** | **Amber `#FFAA00`** + **Amarelo `#FFD400`** (marca) |
| Fundo dark | Preto `#0B141A` | **Navy `#03152E`** / `#061C3A` / `#0B2A50` |
| Fundo claro | Branco `#FFFFFF` | Cloud `#F7F9FC` |
| Gradiente de marca | Azul→Ciano "spark" (`#2F6BFF→#37E0FF`) | **Amarelo→Âmbar→Laranja** `#FFD400→#FFAA00→#FF7900` |
| Bolha enviada dark | Azul `#1B49B8` | **Âmbar escurecido `#3B3214`** |
| Bolha recebida dark | `#1F2C33` | Navy `#102B4D` |
| Bolha enviada claro | Azul `#2F6BFF` | **Amarelo-claro `#FFF3C4`** |
| Logo | Ícone "raio" (Bolt) no gradiente spark | **Logo real do app** (`assets/branding/zaplivre-icon-1024.png`) |
| Splash | **Não existe** (Android tema padrão; iOS `UILaunchScreen: {}` vazio) | Splash com logo + navy |

**Problema central encontrado:** as três plataformas têm **temas divergentes** (Android/iOS azul, Desktop teal) — não há identidade unificada. O logo real (navy + âmbar, extraído por análise de pixels: `#010d26` ≈80%, `#f4af03` ≈15%) não é usado em nenhuma tela.

---

## 2. Gap por plataforma (o que existe hoje)

### 2.1 Android (Compose)
- **Tema:** `ui/theme/Color.kt` — `ZapColors` com primária azul, `sparkBrush` azul→ciano, `avatarPalette` (8 cores derivadas por hash).
- **Logo:** `ZapComponents.kt:87` `ZapLogo` = quadrado arredondado + gradiente spark + ícone `Bolt` branco. Não é o logo real.
- **Splash:** `res/values/themes.xml` = `android:Theme.Material.Light.NoActionBar` (padrão). **Sem splash screen customizada** (sem `windowSplashScreenBackground` / logo / navy).
- **Telas com marca:** Onboarding (`ZapLogo` 104dp + wordmark "ZapLivre"), Settings.
- **Protótipo mobile** usa navy `#03152E`, brandmark "Z" com gradiente amarelo→laranja, tabs (Todas/Não lidas/Grupos/Favoritas), bottom nav (Conversas/Chamadas/Grupos/Ajustes), bolha enviada âmbar.

### 2.2 iOS (SwiftUI)
- **Tema:** `DesignSystem/ZapTheme.swift` — espelha o Android (primária `#2F6BFF`, `sparkGradient`, `avatarPalette`). Cores light/dark via `Color(light:dark:)`.
- **Logo:** `DesignPreviewView`/onboarding usam marca com raio.
- **Splash:** `project.yml` `UILaunchScreen: {}` — **vazio** (tela branca de launch). Precisa de Launch Screen com logo.

### 2.3 Desktop (Tauri + React + Tailwind)
- **Tema:** `tailwind.config.js` — `primary` **teal `#00af93`** (escala 50–900). Onboarding com `bg-gradient-to-br from-primary-50 to-primary-100` e círculo teal com ícone de balão.
- **Logo:** ícone de balão de conversa (SVG inline) em círculo teal — não é o logo real.
- **Splash:** sem splash (Tauri sem window config de splash).
- **Protótipo desktop** usa 3 colunas (sidebar 76px / lista 300px / chat), navy, botão Enviar com gradiente, chamada em andamento, mensagens navy/âmbar.

---

## 3. Comparativo com WhatsApp (não business)

O que o WhatsApp tem e que precisamos checar/espelhar (levantamento de UI/UX):

| Recurso/Visual | WhatsApp | ZapLivre hoje | Ação sugerida |
|---|---|---|---|
| Splash com logo | Sim (logo + cor de marca) | **Não** | Criar splash nas 3 plataformas |
| Tema unificado | Verde (dark/light consistentes) | Azul×Teal×Navy desalinhados | Adotar paleta navy+âmbar |
| Header de conversa (avatar+status+ações) | Sim | Sim (todas as plataformas) | Recolorir p/ navy |
| Bolhas com metadata (✓✓, hora) | Sim | Sim (`MessageStatusIndicator`) | Recolorir p/ âmbar |
| Bottom nav (Conversas/Chamadas/Grupos/Ajustes) | Sim | Mobile tem; desktop usa sidebar | Manter estrutura, aplicar tema |
| Filtros de conversa (Todas/Não lidas/Grupos/Favoritas) | Sim (iOS) | Protótipo tem; app? | Verificar/adicionar |
| Estados vazios com ilustração | Sim | Sim (SkeletonLoader) | Aplicar logo/marca |
| Avatar com foto/fallback colorido | Sim | Sim (`accent(seed)`) | Manter, ajustar paleta |
| Chamadas com tela dedicada | Sim | Sim (Call/VideoCall) | Recolorir (encerrar = vermelho) |
| Dark mode consistente | Sim | Sim | Adotar navy |
| Tipografia arredondada | Sim | `ZapType` (rounded) | Manter |

---

## 4. Plano de implementação proposto

### Fase 1 — Foundation (tema e tokens)
1. **Android:** atualizar `Color.kt` → paleta navy/âmbar (tokens da `paleta-cores.md`):
   - `primary` = amber `#FFAA00`, `brand` = yellow `#FFD400`, `spark` → `brandBrush` amarelo→laranja.
   - Dark: `canvas` `#03152E`, `surface` `#0B2A50`, `bubbleOut` `#3B3214`, `bubbleIn` `#102B4D`, `hairline` `#193B61`.
   - Light: `canvas` `#F7F9FC`, `bubbleOut` `#FFF3C4`, texto sobre âmbar = navy `#061C3A`.
2. **iOS:** espelhar em `ZapTheme.swift` (mesmos tokens light/dark).
3. **Desktop:** substituir Tailwind `primary` (teal) pela escala navy/âmbar; atualizar `index.css` e componentes que usam `primary-*`.

### Fase 2 — Logo real nas telas
4. Gerar/reutilizar o logo real em cada plataforma (Android drawable, iOS Assets, Desktop PNG).
5. **Android:** trocar `ZapLogo` (raio) pelo logo real; aplicar em Onboarding, Settings, telas vazias.
6. **iOS:** aplicar em Onboarding/DesignPreview.
7. **Desktop:** trocar balão teal pelo logo real no Onboarding, sidebar, telas vazias.

### Fase 3 — Splash
8. **Android:** `themes.xml` + splash screen (navy + logo central) — API 31+ `windowSplashScreenBackground`.
9. **iOS:** `UILaunchScreen` com logo + fundo navy (via `project.yml`/Assets).
10. **Desktop:** splash window no Tauri (logo + navy) enquanto o core carrega.

### Fase 4 — Detalhes de paridade com WhatsApp
11. Filtros de conversa (Todas/Não lidas/Grupos/Favoritas) se ainda não houver.
12. Ajustar estados vazios, botão "Enviar" com gradiente de marca, header e chamada com tema navy.

---

## 5. Riscos / decisões pendentes

- **Contraste:** paleta define texto `#061C3A` sobre âmbar/amarelo (evitar branco sobre `#FFD400`/`#FFAA00`) — seguir rigorosamente.
- **Gradiente em áreas grandes:** a paleta recomenda usar o gradiente só em marca/ações (não em fundos). Respeitar em splash e headers.
- **Mudança de identidade:** azul (Android/iOS) e teal (desktop) hoje são diferentes — a unificação para navy+âmbar é uma mudança visível; validar com o usuário se quer trocar **tudo** ou só os pontos-chave.
- **Escopo:** rebranding completo das 3 plataformas é esforço considerável; sugerir começar por **splash + logo + tema core** (Fases 1–3) e deixar os detalhes de paridade (Fase 4) como segunda etapa.

---

## 6. Conclusão

O produto tem base funcional forte (telas, componentes, DesignSystem existente), mas a identidade visual está **fragmentada** (3 temas diferentes, logo genérico, sem splash). Com a paleta navy+âmbar já definida e os protótipos como referência, o caminho é: unificar os tokens nas 3 plataformas → aplicar o logo real → criar splash → refinar paridade com o WhatsApp.
