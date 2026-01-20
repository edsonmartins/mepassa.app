# Bootstrap Storage: SQLite Persistence ✅

**Data:** 2026-01-20
**Melhoria:** Adicionado persistência SQLite para DHT routing table

---

## 📋 Resumo

Implementação completa de persistência SQLite para o Bootstrap Node, permitindo que a tabela DHT sobreviva a restarts e mantenha histórico de peers conhecidos.

---

## ✅ O que foi implementado

### 1. storage.rs (274 linhas)

**Funcionalidades:**
- ✅ SQLite com WAL mode (melhor concurrency)
- ✅ Schema automático (`dht_peers` table)
- ✅ Operações async via `tokio::task::spawn_blocking`
- ✅ UPSERT automático (INSERT ... ON CONFLICT UPDATE)
- ✅ Garbage collection de peers stale
- ✅ Timestamps (first_seen, last_seen)
- ✅ Índice em last_seen para queries eficientes

**Estrutura da tabela:**
```sql
CREATE TABLE dht_peers (
    peer_id TEXT NOT NULL,
    multiaddr TEXT NOT NULL,
    first_seen INTEGER NOT NULL,
    last_seen INTEGER NOT NULL,
    PRIMARY KEY (peer_id, multiaddr)
);

CREATE INDEX idx_last_seen ON dht_peers(last_seen);
```

**API pública:**
```rust
pub struct DhtStorage {
    conn: Arc<Mutex<Connection>>,
}

impl DhtStorage {
    pub async fn new(db_path: PathBuf) -> Result<Self>
    pub async fn add_peer(&self, peer_id: &PeerId, addr: &Multiaddr) -> Result<()>
    pub async fn remove_peer(&self, peer_id: &PeerId, addr: &Multiaddr) -> Result<()>
    pub async fn load_peers(&self) -> Result<Vec<(PeerId, Vec<Multiaddr>)>>
    pub async fn cleanup_stale(&self, max_age_secs: i64) -> Result<usize>
    pub async fn get_stats(&self) -> Result<StorageStats>
}
```

---

### 2. Integração no main.rs

**Fluxo de inicialização:**
```rust
// 1. Inicializar storage
let db_path = config.data_dir.join("dht.db");
let storage = DhtStorage::new(db_path).await?;

// 2. Carregar peers existentes
let stored_peers = storage.load_peers().await?;
for (peer_id, addrs) in stored_peers {
    for addr in addrs {
        swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
    }
}

// 3. Cleanup de peers stale (> 7 dias)
storage.cleanup_stale(7 * 24 * 60 * 60).await?;
```

**Salvamento automático:**
```rust
// ConnectionEstablished → salva no storage
SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
    let addr = endpoint.get_remote_address();

    // Add to DHT
    swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());

    // Save to storage
    storage.add_peer(&peer_id, addr).await?;
}

// Identify → salva novos endereços descobertos
identify::Event::Received { peer_id, info } => {
    for addr in info.listen_addrs {
        swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
        storage.add_peer(&peer_id, &addr).await?;
    }
}
```

---

## 🔧 Decisões Técnicas

### Por que rusqlite ao invés de sqlx?

**Problema:** O core já usa `rusqlite 0.31`, e adicionar `sqlx` causava conflito:
```
error: failed to select a version for `libsqlite3-sys`.
package `libsqlite3-sys v0.27.0` (sqlx)
conflicts with
package `libsqlite3-sys v0.28.0` (rusqlite)
```

**Solução:** Usar `rusqlite` que já está no workspace, mantendo consistência.

### Por que tokio::task::spawn_blocking?

Rusqlite é **síncrono**, então usamos `spawn_blocking` para não bloquear o event loop async do libp2p.

```rust
pub async fn add_peer(&self, peer_id: &PeerId, addr: &Multiaddr) -> Result<()> {
    let conn = self.conn.clone();
    let peer_id_str = peer_id.to_string();
    let addr_str = addr.to_string();

    // Execute SQL em thread separada (blocking)
    tokio::task::spawn_blocking(move || {
        let conn = conn.lock().unwrap();
        conn.execute(/* SQL */, params![])?;
        Ok(())
    })
    .await?
}
```

### Por que Arc<Mutex<Connection>>?

- `Arc`: Permite clonar e passar para múltiplas tasks
- `Mutex`: SQLite Connection não é thread-safe, precisa de lock
- Alternativa seria usar `r2d2` pool, mas é overkill para nosso caso

---

## 📊 Benefícios

### 1. Zero Downtime em Restarts

**Antes (MemoryStore):**
```
1. Bootstrap inicia → Routing table VAZIA
2. Peers reconectam (1-2 segundos cada)
3. Tabela DHT se reconstrói gradualmente
```

**Depois (SQLite):**
```
1. Bootstrap inicia → Carrega 1000 peers do SQLite
2. Routing table JÁ populada
3. Novo peer encontra outros peers INSTANTANEAMENTE
```

### 2. Histórico de Peers

```sql
-- Ver peers mais antigos (confiáveis)
SELECT peer_id, datetime(first_seen, 'unixepoch') as joined
FROM dht_peers
ORDER BY first_seen ASC
LIMIT 10;

-- Ver peers mais ativos (recently seen)
SELECT peer_id, datetime(last_seen, 'unixepoch') as last_active
FROM dht_peers
ORDER BY last_seen DESC
LIMIT 10;
```

### 3. Garbage Collection Automático

```rust
// Limpa peers que não se conectam há 7 dias
storage.cleanup_stale(7 * 24 * 60 * 60).await?;
```

### 4. Fácil Debugging

```bash
# Inspecionar database
sqlite3 /app/data/dht.db

# Ver todos os peers
SELECT peer_id, multiaddr, datetime(last_seen, 'unixepoch')
FROM dht_peers
ORDER BY last_seen DESC;

# Contar peers por idade
SELECT
    CASE
        WHEN julianday('now') - julianday(last_seen, 'unixepoch') < 1 THEN 'last 24h'
        WHEN julianday('now') - julianday(last_seen, 'unixepoch') < 7 THEN 'last week'
        ELSE 'older'
    END as age_group,
    COUNT(*) as count
FROM dht_peers
GROUP BY age_group;
```

---

## 🧪 Como Testar

### Teste 1: Persistência entre Restarts

```bash
# 1. Iniciar bootstrap
cargo run

# Logs:
# 📂 Opening DHT storage at: "/app/data/dht.db"
# ✅ DHT storage ready
# 📥 Loaded 0 peers with 0 addresses from storage  (primeira vez)

# 2. Conectar alguns clients (Android, Desktop)
# Aguardar até peers estarem conectados

# 3. Reiniciar bootstrap (Ctrl+C e cargo run)
# Logs:
# 📥 Loaded 5 peers with 8 addresses from storage  ← Carregou do SQLite!

# 4. Verificar database
sqlite3 /tmp/mepassa-bootstrap/dht.db "SELECT COUNT(*) FROM dht_peers"
# Deve mostrar número de peers salvos
```

### Teste 2: Garbage Collection

```bash
# Adicionar peer "fake" com timestamp antigo
sqlite3 /tmp/mepassa-bootstrap/dht.db <<EOF
INSERT INTO dht_peers (peer_id, multiaddr, first_seen, last_seen)
VALUES ('fake_peer', '/ip4/1.2.3.4/tcp/4001',
        strftime('%s', 'now', '-30 days'),
        strftime('%s', 'now', '-30 days'));
EOF

# Reiniciar bootstrap
cargo run

# Log deve mostrar:
# 🧹 Cleaned up 1 stale peer records (older than 604800s)
```

### Teste 3: Inspecionar Database

```bash
# Ver schema
sqlite3 /tmp/mepassa-bootstrap/dht.db ".schema"

# Ver todos os peers
sqlite3 /tmp/mepassa-bootstrap/dht.db \
  "SELECT peer_id, multiaddr, datetime(last_seen, 'unixepoch') FROM dht_peers"

# Count por peer
sqlite3 /tmp/mepassa-bootstrap/dht.db \
  "SELECT peer_id, COUNT(*) as addr_count FROM dht_peers GROUP BY peer_id"
```

---

## 📁 Arquivos Criados/Modificados

### Criados (1 arquivo)
1. `STORAGE_SQLITE.md` - Esta documentação

### Modificados (3 arquivos)
1. `Cargo.toml` - Adicionado `rusqlite = { version = "0.31", features = ["bundled"] }`
2. `src/storage.rs` - Reescrito completamente (274 linhas)
3. `src/main.rs` - Integração com storage (load/save peers)

**Total:** ~300 linhas de código

---

## 🔄 Comparação: MemoryStore vs SQLite

| Aspecto | MemoryStore | SQLite |
|---------|------------|---------|
| **Persistência** | ❌ Perde tudo em restart | ✅ Mantém entre restarts |
| **Startup** | Tabela vazia | Carrega peers salvos |
| **Complexidade** | Simples (52 linhas) | Moderada (274 linhas) |
| **Debugging** | Difícil (RAM only) | Fácil (SQL queries) |
| **Performance** | Rápido (RAM) | Rápido (WAL mode) |
| **Garbage Collection** | Manual | Automático (SQL DELETE) |
| **Histórico** | Não | Timestamps (first/last_seen) |
| **Production Ready** | ⚠️ Para redes pequenas | ✅ Para produção |

---

## 💡 Melhorias Futuras

### 1. Metrics Endpoint

Expor estatísticas via health endpoint:
```rust
// GET /health
{
  "status": "OK",
  "peer_count": 42,
  "uptime_seconds": 3600,
  "storage": {
    "total_peers": 156,
    "total_addresses": 289,
    "stale_peers": 12
  }
}
```

### 2. Periodic Sync

Atualmente salvamos on-demand. Poderia ter sync periódico:
```rust
// A cada 5 minutos, garantir que DHT está em sync com storage
tokio::spawn(async move {
    loop {
        tokio::time::sleep(Duration::from_secs(300)).await;
        let stats = storage.get_stats().await?;
        info!("📊 Storage stats: {:?}", stats);
    }
});
```

### 3. Peer Reputation

Adicionar coluna de "reputation" baseada em uptime:
```sql
ALTER TABLE dht_peers ADD COLUMN reputation REAL DEFAULT 1.0;

-- Aumentar reputation de peers estáveis
UPDATE dht_peers
SET reputation = MIN(10.0, reputation + 0.1)
WHERE julianday('now') - julianday(last_seen, 'unixepoch') < 1;
```

### 4. Sharding (Múltiplos Bootstraps)

Para escalabilidade, múltiplos bootstraps poderiam compartilhar database:
```
bootstrap-1 → dht_shard_1.db (peers A-M)
bootstrap-2 → dht_shard_2.db (peers N-Z)
```

---

## ✅ Checklist

- [x] rusqlite adicionado ao Cargo.toml
- [x] storage.rs reescrito com rusqlite
- [x] Operações async via spawn_blocking
- [x] Schema automático com índices
- [x] UPSERT para add_peer
- [x] load_peers carrega e agrupa por peer_id
- [x] cleanup_stale remove peers antigos
- [x] Integrado em main.rs (load + save)
- [x] ConnectionEstablished salva peers
- [x] Identify salva novos endereços
- [x] Compilação sem erros
- [x] Documentação completa

---

## 🎯 Resultado Final

**Bootstrap Node agora é production-ready** com:
- ✅ Persistência SQLite
- ✅ Zero downtime em restarts
- ✅ Garbage collection automático
- ✅ Fácil debugging via SQL
- ✅ Histórico de peers com timestamps

**Próxima FASE:** TURN Server Integration

---

**Autor:** Claude Opus 4.5 + Edson Martins
**Data:** 2026-01-20
