# ADR 001: Username-Based Identity System

**Status:** ✅ Aceito  
**Data:** 2025-01-19  
**Decisor:** Edson Martins + Claude Code  

---

## Contexto

MePassa usa **Peer IDs** derivados de chaves públicas Ed25519 para identificação criptográfica:
```
peer_id: "mepassa_BfvwEnRx79B9LQYdYyyABHirY3y6GzPHVNAkbbTy66ta"
```

**Problema:** Esse ID é impossível de compartilhar de forma prática para usuários finais.

WhatsApp resolve isso usando **números de telefone**, mas isso:
- ❌ Expõe informação pessoal (privacidade ruim)
- ❌ Requer SMS gateway (custo + complexidade)
- ❌ Permite metadata leaking (servidor vê quem fala com quem)

---

## Decisão

Implementar sistema de **@username** (como Telegram/Signal) com Identity Server leve.

### Como funciona:

```
1. Usuário escolhe username único: @joao, @maria_silva
2. App registra no Identity Server:
   - username → peer_id mapping
   - prekey_bundle para X3DH
3. Outro usuário busca @joao:
   - Identity Server retorna peer_id + prekey_bundle
4. Estabelece conexão P2P + X3DH
```

---

## Alternativas Consideradas

### ❌ Opção A: Número de Telefone (como WhatsApp)
**Por que rejeitada:**
- Privacidade ruim (expõe número real)
- Custo de SMS gateway
- Metadata leaking
- Conflita com proposta de privacidade do MePassa

### ❌ Opção B: QR Code Only (como Briar)
**Por que rejeitada:**
- UX horrível (precisa estar fisicamente próximo)
- Impossível adicionar remotamente
- Baixa taxa de adoção

### ✅ Opção C: @Username System (escolhida)
**Vantagens:**
- Privacidade boa (não expõe telefone)
- UX aceitável ("Me adiciona: @joao")
- Custo zero (sem SMS)
- Simples de implementar
- Global namespace único

---

## Implementação

### Identity Server (Novo componente)

```rust
// server/identity/src/main.rs

// POST /api/v1/register
{
    "username": "joao",
    "peer_id": "mepassa_BfvwE...",
    "prekey_bundle": {
        "identity_key": [...],
        "signed_prekey_id": 1,
        "signed_prekey": [...],
        "signed_prekey_signature": [...],
        "one_time_prekey": {...}
    }
}

// GET /api/v1/lookup?username=joao
Response:
{
    "username": "joao",
    "peer_id": "mepassa_BfvwE...",
    "prekey_bundle": {...},
    "last_updated": "2025-01-19T10:00:00Z"
}

// PUT /api/v1/prekeys (atualizar prekeys periodicamente)
```

### Schema PostgreSQL

```sql
CREATE TABLE usernames (
    username TEXT PRIMARY KEY,
    peer_id TEXT NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,
    prekey_bundle JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_updated TIMESTAMP DEFAULT NOW(),
    
    CONSTRAINT username_format CHECK (username ~ '^[a-z0-9_]{3,20}$')
);

CREATE INDEX idx_usernames_peer_id ON usernames(peer_id);
CREATE INDEX idx_usernames_created_at ON usernames(created_at);
```

### Schema SQLite (Client)

```sql
-- Atualização na tabela contacts
CREATE TABLE contacts (
    peer_id TEXT PRIMARY KEY,
    username TEXT UNIQUE,           -- @joao (NOVO)
    display_name TEXT,              -- "João Silva" (apelido local)
    public_key BLOB NOT NULL,
    prekey_bundle_json TEXT,        -- Cache do bundle (NOVO)
    last_seen INTEGER,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX idx_contacts_username ON contacts(username);
```

### Client Flow (Android/iOS/Desktop)

```kotlin
// 1. Usuário digita "@joao" na busca
val username = "@joao"

// 2. Client busca no Identity Server
val response = identityServerClient.lookup(username)

// 3. Salva contato localmente
contactsDb.insert(
    peerId = response.peerId,
    username = username,
    displayName = username, // default, user can edit
    publicKey = response.prekeyBundle.identityKey,
    prekeyBundleJson = response.prekeyBundle.toJson()
)

// 4. Estabelece conexão P2P + X3DH
val (sharedSecret, ephemeralPub) = X3DH.initiate(response.prekeyBundle)
// ... continua com P2P connection
```

---

## Consequências

### ✅ Positivas
1. **UX aceitável** - Fácil compartilhar ("Me adiciona: @joao")
2. **Privacidade boa** - Não expõe número de telefone real
3. **Custo zero** - Sem SMS gateway
4. **Simples** - Identity Server stateless e leve
5. **Global** - Username único no mundo todo
6. **Compatível** - Funciona com arquitetura híbrida P2P

### ⚠️ Negativas (Mitigadas)
1. **Centralização do namespace**
   - *Mitigação:* Server stateless, pode ter múltiplas instâncias federadas
   - *Futuro:* DHT-based username registry (blockchain/IPNS)

2. **Precisa escolher username único**
   - *Mitigação:* Sugestões automáticas se ocupado (@joao1, @joao_silva)
   - *UX:* Similar a Telegram (usuários já entendem)

3. **Server pode saber quem busca quem**
   - *Mitigação:* Logs mínimos, retenção curta (7 dias)
   - *Futuro:* Private Information Retrieval (PIR)

4. **Username pode ser impersonated**
   - *Mitigação:* First-come-first-served
   - *Verificação:* Badges verificados para figuras públicas (futuro)

---

## Roadmap

### FASE 1.5 (NOVA): Identity Server & Username System
**Prioridade:** 🔥 ALTA (bloqueante para UX)
**Duração:** ~1 semana

**Tarefas:**
1. Implementar Identity Server (Rust + Axum)
   - POST /register
   - GET /lookup
   - PUT /prekeys
2. Atualizar schemas (PostgreSQL + SQLite)
3. Implementar username validation
4. Rate limiting (anti-spam)
5. Health checks + monitoring

**Deploy:**
- identity.mepassa.app (DNS)
- PostgreSQL para storage
- Redis para rate limiting
- HTTPS obrigatório

### FASE 6 (Android): Username UI
1. Onboarding: Escolher @username
2. Adicionar contato: Input "@username"
3. Buscar no Identity Server
4. Salvar em contacts
5. Iniciar chat

---

## Exemplos de Uso

### Cenário 1: Alice adiciona Bob

```
1. Bob registra @bob_silva no primeiro uso
2. Alice quer adicionar Bob
3. Alice digita "@bob_silva" no app
4. App busca no Identity Server
5. App obtém peer_id + prekey_bundle
6. App estabelece X3DH + P2P connection
7. Alice pode enviar mensagem encrypted para Bob
```

### Cenário 2: Compartilhar username

```
- Link: mepassa.app/add/@joao
- QR Code com: mepassa://add/@joao
- Texto: "Me adiciona no MePassa: @joao"
```

---

## Segurança

### Ameaças Mitigadas
1. ✅ **Username squatting**: First-come-first-served + verificação futura
2. ✅ **Phishing**: E2E encryption garante autenticidade
3. ✅ **Spam**: Rate limiting no registration
4. ✅ **Metadata leak**: Mínimo necessário (só username → peer_id)

### Ameaças Residuais
1. ⚠️ **Server downtime**: Identity Server offline = não adiciona novos contatos
   - *Mitigação:* QR code como fallback
2. ⚠️ **Server compromised**: Pode mapear username errado
   - *Mitigação:* Safety number verification (futuro)
   - *Mitigação:* Transparency logs (futuro)

---

## Referências

- **Telegram:** @username system (inspiração)
- **Signal:** Username discovery (privacy-preserving)
- **Matrix:** Identity Server (federado)
- **Keybase:** Social proofs + username

---

## Revisões

- **2025-01-19:** Decisão inicial (@username system)

