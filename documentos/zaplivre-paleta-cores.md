# ZapLivre — Sugestão de Paleta de Cores

## Direção visual

A identidade do ZapLivre pode se diferenciar de mensageiros tradicionais evitando o verde como cor principal. O logo já estabelece uma combinação forte de **azul-marinho profundo + amarelo/âmbar + laranja**, transmitindo energia, liberdade, velocidade e tecnologia.

A recomendação é usar o **navy como base estrutural** e reservar o **amarelo/âmbar para marca, ações principais e estados ativos**.

## Paleta principal

| Token | Nome | Hex | Uso sugerido |
|---|---|---:|---|
| `navy-950` | ZapLivre Navy 950 | `#03152E` | Fundo dark, splash, áreas profundas |
| `navy-900` | ZapLivre Navy 900 | `#061C3A` | Header, sidebar, navegação |
| `navy-800` | ZapLivre Navy 800 | `#0B2A50` | Cards dark, hover, seleção |
| `yellow-500` | ZapLivre Yellow | `#FFD400` | Marca, destaque, indicadores ativos |
| `amber-500` | ZapLivre Amber | `#FFAA00` | Botões principais, gradientes |
| `orange-500` | ZapLivre Orange | `#FF7900` | Acentos, energia visual, gradientes |
| `cloud-50` | Cloud 50 | `#F7F9FC` | Fundo claro |
| `cloud-100` | Cloud 100 | `#EEF2F7` | Áreas secundárias e superfícies |
| `slate-500` | Slate 500 | `#64748B` | Texto secundário no tema claro |
| `ink-900` | Ink 900 | `#0F172A` | Texto principal no tema claro |

## Gradiente de marca

```css
linear-gradient(135deg, #FFD400 0%, #FFAA00 55%, #FF7900 100%)
```

Esse gradiente deve ser usado principalmente em:

- logo e elementos de marca;
- botões de ação principal;
- indicadores ativos;
- detalhes especiais de chamadas e recursos premium.

Evite usar o gradiente em grandes áreas de fundo.

## Tema dark

| Elemento | Cor |
|---|---:|
| Background principal | `#03152E` |
| Header / Sidebar | `#061C3A` |
| Cards / Hover | `#0B2A50` |
| Bordas | `#193B61` |
| Texto principal | `#F8FAFC` |
| Texto secundário | `#94A3B8` |
| Mensagem recebida | `#102B4D` |
| Mensagem enviada | `#3B3214` |
| Ação principal | `#FFAA00` |
| Destaque de marca | `#FFD400` |
| Sucesso / Online | `#22C55E` |
| Erro / Encerrar chamada | `#EF4444` |

### Mensagens no tema dark

**Recebida**

- fundo: `#102B4D`
- texto: `#F8FAFC`
- borda: `#193B61`

**Enviada**

- fundo: `#3B3214`
- texto: `#F8FAFC`
- metadata: `#C8AA49`

A bolha enviada usa um tom âmbar escurecido, mantendo a identidade sem comprometer a leitura.

## Tema light

| Elemento | Cor |
|---|---:|
| Background principal | `#F7F9FC` |
| Cards | `#FFFFFF` |
| Áreas secundárias | `#EEF2F7` |
| Header especial | `#061C3A` |
| Texto principal | `#0F172A` |
| Texto secundário | `#64748B` |
| Mensagem recebida | `#FFFFFF` |
| Mensagem enviada | `#FFF3C4` |
| Ação principal | `#FFB000` |
| Texto sobre ação principal | `#061C3A` |

## Cores semânticas

O verde não deve ser usado como identidade principal. Ele pode continuar presente apenas como estado semântico.

| Estado | Cor |
|---|---:|
| Online / sucesso | `#22C55E` |
| Informação | `#3B82F6` |
| Atenção | `#F59E0B` |
| Erro | `#EF4444` |
| Chamada encerrada | `#DC2626` |

## Recomendação de contraste

Evitar texto branco sobre `#FFD400` e `#FFAA00`. Para botões amarelos ou âmbar, usar preferencialmente:

```css
color: #061C3A;
```

Isso melhora muito o contraste e reforça a conexão com o navy da marca.

## Tokens CSS sugeridos

```css
:root {
  --zl-navy-950: #03152E;
  --zl-navy-900: #061C3A;
  --zl-navy-800: #0B2A50;
  --zl-border-dark: #193B61;

  --zl-yellow: #FFD400;
  --zl-amber: #FFAA00;
  --zl-orange: #FF7900;

  --zl-cloud-50: #F7F9FC;
  --zl-cloud-100: #EEF2F7;
  --zl-slate-500: #64748B;
  --zl-ink-900: #0F172A;

  --zl-success: #22C55E;
  --zl-info: #3B82F6;
  --zl-warning: #F59E0B;
  --zl-danger: #EF4444;
}
```

## Direção final recomendada

A assinatura visual do ZapLivre deve ser percebida como:

**Navy profundo + amarelo/âmbar energético + laranja pontual.**

A proposta diferencia o produto de mensageiros verdes, funciona bem em desktop e mobile e dá suporte visual a conversas individuais, grupos e chamadas de áudio e vídeo.
