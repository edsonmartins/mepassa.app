# Omega Core - Rust Library Structure

## Cargo.toml

```toml
[package]
name = "omega-core"
version = "0.1.0"
edition = "2021"
license = "AGPL-3.0"
authors = ["IntegrallTech <contato@integralltech.com.br>"]
description = "Core library for Omega P2P chat platform"
repository = "https://github.com/integralltech/omega"

[lib]
name = "omega_core"
crate-type = ["cdylib", "staticlib", "rlib"]

[dependencies]
# Networking P2P
libp2p = { version = "0.53", features = [
    "tcp",
    "quic",
    "noise",
    "yamux",
    "gossipsub",
    "kad",
    "relay",
    "dcutr",
    "identify",
    "ping",
    "mdns"
]}

# Async runtime
tokio = { version = "1.35", features = ["full"] }
futures = "0.3"

# Cryptography
libsignal-client = "0.42"
ed25519-dalek = "2.1"
x25519-dalek = "2.0"
rand = "0.8"
sha2 = "0.10"
aes-gcm = "0.10"
hkdf = "0.12"

# Storage
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# CRDTs for sync
automerge = "0.5"

# Protobuf
prost = "0.12"
prost-types = "0.12"

# FFI
uniffi = "0.25"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Utils
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = "0.4"
base64 = "0.21"
hex = "0.4"

[dev-dependencies]
proptest = "1.4"
criterion = "0.5"
tempfile = "3.8"

[build-dependencies]
prost-build = "0.12"
uniffi = { version = "0.25", features = ["build"] }

[[bench]]
name = "crypto_bench"
harness = false

[[bench]]
name = "network_bench"
harness = false
```

## Estrutura de Módulos

```
omega-core/
├── Cargo.toml
├── build.rs                 # Build script para protobuf + uniffi
├── src/
│   ├── lib.rs              # Re-exports públicos
│   │
│   ├── identity/           # Gerenciamento de identidade
│   │   ├── mod.rs
│   │   ├── keypair.rs      # Ed25519 keypairs
│   │   ├── prekeys.rs      # Signal prekeys (X25519)
│   │   └── storage.rs      # Keychain storage
│   │
│   ├── crypto/             # Criptografia E2E
│   │   ├── mod.rs
│   │   ├── signal.rs       # Signal Protocol wrapper
│   │   ├── session.rs      # Session management
│   │   ├── ratchet.rs      # Double Ratchet
│   │   └── group.rs        # Sender Keys (grupos)
│   │
│   ├── network/            # P2P networking
│   │   ├── mod.rs
│   │   ├── behaviour.rs    # libp2p behaviour
│   │   ├── transport.rs    # Transport layer
│   │   ├── dht.rs          # Kademlia DHT
│   │   ├── gossip.rs       # GossipSub
│   │   ├── relay.rs        # Relay client
│   │   └── nat.rs          # NAT traversal (STUN/TURN)
│   │
│   ├── storage/            # Armazenamento local
│   │   ├── mod.rs
│   │   ├── database.rs     # SQLite wrapper
│   │   ├── schema.rs       # Table definitions
│   │   ├── migrations.rs   # DB migrations
│   │   ├── messages.rs     # Message CRUD
│   │   ├── contacts.rs     # Contacts CRUD
│   │   └── groups.rs       # Groups CRUD
│   │
│   ├── sync/               # Multi-device sync
│   │   ├── mod.rs
│   │   ├── crdt.rs         # Automerge wrapper
│   │   ├── device.rs       # Device linking
│   │   └── protocol.rs     # Sync protocol
│   │
│   ├── protocol/           # Message protocols
│   │   ├── mod.rs
│   │   ├── messages.rs     # Protobuf messages (generated)
│   │   ├── codec.rs        # Encode/decode
│   │   └── validation.rs   # Message validation
│   │
│   ├── api/                # API pública
│   │   ├── mod.rs
│   │   ├── client.rs       # Main Client API
│   │   ├── events.rs       # Event system
│   │   └── callbacks.rs    # Callback handlers
│   │
│   ├── ffi/                # Foreign Function Interface
│   │   ├── mod.rs
│   │   ├── uniffi.udl      # UDL definitions
│   │   └── types.rs        # FFI-safe types
│   │
│   └── utils/              # Utilidades
│       ├── mod.rs
│       ├── config.rs       # Configuration
│       ├── logging.rs      # Logging setup
│       └── error.rs        # Error types
│
├── proto/                   # Protobuf definitions
│   └── messages.proto
│
├── tests/                   # Integration tests
│   ├── integration_test.rs
│   ├── e2e_test.rs
│   └── nat_traversal_test.rs
│
├── benches/                 # Benchmarks
│   ├── crypto_bench.rs
│   └── network_bench.rs
│
└── examples/                # Exemplos
    ├── simple_chat.rs
    ├── send_message.rs
    └── group_chat.rs
```

## Código Exemplo: lib.rs

```rust
// src/lib.rs

// Re-exports públicos
pub use api::{Client, ClientBuilder, Event};
pub use identity::{Identity, Keypair};
pub use protocol::{Message, MessageContent, MessageType};
pub use crypto::{EncryptionSession, SessionError};
pub use storage::{Database, StorageError};

pub mod api;
pub mod crypto;
pub mod identity;
pub mod network;
pub mod protocol;
pub mod storage;
pub mod sync;
pub mod utils;

// FFI exports (gerado por uniffi)
#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "ffi")]
uniffi::include_scaffolding!("omega");

// Versão da biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Inicializa o sistema de logging
pub fn init_logging(level: utils::LogLevel) {
    utils::logging::init(level);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
```

## Código Exemplo: identity/keypair.rs

```rust
// src/identity/keypair.rs

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Invalid key format")]
    InvalidKeyFormat,
    
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    
    #[error("Cryptographic error: {0}")]
    CryptoError(String),
}

/// Identity keypair (Ed25519)
#[derive(Clone)]
pub struct Keypair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl Keypair {
    /// Gera novo keypair aleatório
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        Self {
            signing_key,
            verifying_key,
        }
    }
    
    /// Carrega keypair de bytes
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, IdentityError> {
        let signing_key = SigningKey::from_bytes(bytes);
        let verifying_key = signing_key.verifying_key();
        
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }
    
    /// Exporta chave privada (use com cuidado!)
    pub fn to_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }
    
    /// Retorna chave pública
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            key: self.verifying_key,
        }
    }
    
    /// Assina mensagem
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.signing_key.sign(message).to_bytes().to_vec()
    }
    
    /// Gera ID do peer (hash da chave pública)
    pub fn peer_id(&self) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(self.verifying_key.as_bytes());
        let hash = hasher.finalize();
        format!("omega_{}", hex::encode(&hash[..16]))
    }
}

/// Chave pública
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PublicKey {
    key: VerifyingKey,
}

impl PublicKey {
    /// Verifica assinatura
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), IdentityError> {
        let sig = Signature::from_slice(signature)
            .map_err(|e| IdentityError::CryptoError(e.to_string()))?;
        
        self.key.verify(message, &sig)
            .map_err(|_| IdentityError::SignatureVerificationFailed)
    }
    
    /// Exporta para bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        self.key.to_bytes()
    }
    
    /// Importa de bytes
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, IdentityError> {
        let key = VerifyingKey::from_bytes(bytes)
            .map_err(|e| IdentityError::CryptoError(e.to_string()))?;
        Ok(Self { key })
    }
    
    /// Gera ID legível
    pub fn to_id(&self) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(self.key.as_bytes());
        let hash = hasher.finalize();
        format!("omega_{}", hex::encode(&hash[..16]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = Keypair::generate();
        assert_eq!(kp.to_bytes().len(), 32);
    }

    #[test]
    fn test_sign_verify() {
        let kp = Keypair::generate();
        let message = b"Hello, Omega!";
        let signature = kp.sign(message);
        
        let public = kp.public_key();
        assert!(public.verify(message, &signature).is_ok());
    }

    #[test]
    fn test_peer_id_generation() {
        let kp = Keypair::generate();
        let id = kp.peer_id();
        assert!(id.starts_with("omega_"));
        assert_eq!(id.len(), "omega_".len() + 32); // omega_ + 16 bytes hex
    }
}
```

## Código Exemplo: api/client.rs

```rust
// src/api/client.rs

use crate::{
    identity::{Keypair, PublicKey},
    network::Network,
    storage::Database,
    protocol::{Message, MessageContent},
    crypto::SessionManager,
};
use anyhow::Result;
use tokio::sync::mpsc;
use std::path::PathBuf;

/// Eventos que o cliente pode emitir
#[derive(Debug, Clone)]
pub enum Event {
    /// Nova mensagem recebida
    MessageReceived {
        sender: String,
        message: Message,
    },
    
    /// Mensagem enviada com sucesso
    MessageSent {
        message_id: String,
    },
    
    /// Peer conectado
    PeerConnected {
        peer_id: String,
    },
    
    /// Peer desconectado
    PeerDisconnected {
        peer_id: String,
    },
    
    /// Erro
    Error {
        message: String,
    },
}

/// Cliente principal Omega
pub struct Client {
    identity: Keypair,
    database: Database,
    network: Network,
    sessions: SessionManager,
    event_tx: mpsc::Sender<Event>,
}

impl Client {
    /// Envia mensagem de texto
    pub async fn send_text(
        &mut self,
        recipient: &str,
        text: String,
    ) -> Result<String> {
        // 1. Buscar chave pública do destinatário
        let recipient_key = self.lookup_public_key(recipient).await?;
        
        // 2. Criar ou recuperar sessão E2E
        let session = self.sessions.get_or_create(recipient_key).await?;
        
        // 3. Criar conteúdo da mensagem
        let content = MessageContent::Text { text };
        
        // 4. Criptografar
        let encrypted = session.encrypt(&content)?;
        
        // 5. Criar envelope de mensagem
        let message = Message {
            id: uuid::Uuid::new_v4().to_string(),
            sender: self.identity.peer_id(),
            recipient: recipient.to_string(),
            encrypted_content: encrypted,
            timestamp: chrono::Utc::now().timestamp(),
            signature: vec![], // TODO: assinar
        };
        
        // 6. Salvar localmente
        self.database.save_message(&message)?;
        
        // 7. Enviar via rede
        self.network.send_message(recipient, &message).await?;
        
        Ok(message.id)
    }
    
    /// Busca chave pública via DHT
    async fn lookup_public_key(&self, peer_id: &str) -> Result<PublicKey> {
        // TODO: implementar lookup no DHT
        todo!("DHT lookup")
    }
    
    /// Processa mensagem recebida
    async fn handle_incoming_message(&mut self, message: Message) -> Result<()> {
        // 1. Verificar assinatura
        // TODO: verificar signature
        
        // 2. Descriptografar
        let sender_key = self.lookup_public_key(&message.sender).await?;
        let session = self.sessions.get_or_create(sender_key).await?;
        let content = session.decrypt(&message.encrypted_content)?;
        
        // 3. Salvar no banco
        self.database.save_message(&message)?;
        
        // 4. Emitir evento
        self.event_tx.send(Event::MessageReceived {
            sender: message.sender.clone(),
            message,
        }).await?;
        
        Ok(())
    }
}

/// Builder para criar Client
pub struct ClientBuilder {
    data_dir: PathBuf,
    identity: Option<Keypair>,
    bootstrap_peers: Vec<String>,
}

impl ClientBuilder {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            identity: None,
            bootstrap_peers: vec![],
        }
    }
    
    pub fn identity(mut self, keypair: Keypair) -> Self {
        self.identity = Some(keypair);
        self
    }
    
    pub fn bootstrap_peer(mut self, addr: String) -> Self {
        self.bootstrap_peers.push(addr);
        self
    }
    
    pub async fn build(self) -> Result<(Client, mpsc::Receiver<Event>)> {
        // Criar diretório de dados
        std::fs::create_dir_all(&self.data_dir)?;
        
        // Gerar ou carregar identidade
        let identity = self.identity.unwrap_or_else(|| Keypair::generate());
        
        // Inicializar database
        let db_path = self.data_dir.join("omega.db");
        let database = Database::open(db_path)?;
        
        // Inicializar network
        let network = Network::new(
            identity.clone(),
            self.bootstrap_peers,
        ).await?;
        
        // Criar canal de eventos
        let (event_tx, event_rx) = mpsc::channel(100);
        
        let client = Client {
            identity,
            database,
            network,
            sessions: SessionManager::new(),
            event_tx,
        };
        
        Ok((client, event_rx))
    }
}
```

## Código Exemplo: FFI (uniffi.udl)

```
// src/ffi/omega.udl

namespace omega {
    // Inicializa o sistema
    void init_logging(LogLevel level);
};

enum LogLevel {
    "Trace",
    "Debug",
    "Info",
    "Warn",
    "Error",
};

enum MessageType {
    "Text",
    "Image",
    "Audio",
    "Video",
    "File",
};

// Keypair
dictionary KeypairData {
    sequence<u8> private_key;
    sequence<u8> public_key;
    string peer_id;
};

interface OmegaKeypair {
    constructor();
    
    [Name=from_bytes]
    constructor(sequence<u8> bytes);
    
    sequence<u8> to_bytes();
    sequence<u8> public_key_bytes();
    string peer_id();
    sequence<u8> sign(sequence<u8> message);
};

// Client
[Error]
enum ClientError {
    "NetworkError",
    "StorageError",
    "CryptoError",
    "InvalidInput",
};

interface OmegaClient {
    [Throws=ClientError]
    constructor(string data_dir, OmegaKeypair? identity);
    
    [Throws=ClientError]
    string send_text_message(string recipient, string text);
    
    [Throws=ClientError]
    sequence<MessageData> get_messages(string conversation_id, u32 limit);
};

dictionary MessageData {
    string id;
    string sender;
    string recipient;
    MessageType type;
    string content;
    i64 timestamp;
    boolean delivered;
    boolean read;
};

callback interface EventCallback {
    void on_message_received(MessageData message);
    void on_message_sent(string message_id);
    void on_peer_connected(string peer_id);
    void on_error(string error);
};
```

## Uso no Android (Kotlin)

```kotlin
// android/app/src/main/kotlin/com/omega/chat/OmegaService.kt

import omega.*

class OmegaService : Service() {
    private lateinit var client: OmegaClient
    private var keypair: OmegaKeypair? = null
    
    override fun onCreate() {
        super.onCreate()
        
        // Inicializar logging
        initLogging(LogLevel.DEBUG)
        
        // Carregar ou gerar keypair
        keypair = loadOrGenerateKeypair()
        
        // Criar cliente
        val dataDir = filesDir.absolutePath
        client = OmegaClient(dataDir, keypair)
        
        // Registrar callback
        client.setEventCallback(object : EventCallback {
            override fun onMessageReceived(message: MessageData) {
                // Notificar UI
                sendBroadcast(Intent("OMEGA_MESSAGE_RECEIVED").apply {
                    putExtra("message", message)
                })
            }
            
            override fun onMessageSent(messageId: String) {
                Log.d("Omega", "Message sent: $messageId")
            }
            
            override fun onPeerConnected(peerId: String) {
                Log.d("Omega", "Peer connected: $peerId")
            }
            
            override fun onError(error: String) {
                Log.e("Omega", "Error: $error")
            }
        })
    }
    
    fun sendMessage(recipient: String, text: String) {
        try {
            val messageId = client.sendTextMessage(recipient, text)
            Log.d("Omega", "Sent message: $messageId")
        } catch (e: ClientException) {
            Log.e("Omega", "Failed to send: ${e.message}")
        }
    }
    
    private fun loadOrGenerateKeypair(): OmegaKeypair {
        val prefs = getSharedPreferences("omega_keys", MODE_PRIVATE)
        val privateKeyHex = prefs.getString("private_key", null)
        
        return if (privateKeyHex != null) {
            // Carregar existente
            val bytes = hexToBytes(privateKeyHex)
            OmegaKeypair.fromBytes(bytes)
        } else {
            // Gerar novo
            val kp = OmegaKeypair()
            
            // Salvar
            prefs.edit().apply {
                putString("private_key", bytesToHex(kp.toBytes()))
                apply()
            }
            
            kp
        }
    }
}
```

## Uso no iOS (Swift)

```swift
// ios/Omega/OmegaManager.swift

import Foundation
import omega // Gerado por uniffi

class OmegaManager: ObservableObject {
    @Published var messages: [MessageData] = []
    
    private var client: OmegaClient?
    private var keypair: OmegaKeypair?
    
    init() {
        // Inicializar
        initLogging(level: .debug)
        
        // Carregar ou gerar keypair
        keypair = loadOrGenerateKeypair()
        
        // Criar cliente
        let dataDir = FileManager.default
            .urls(for: .documentDirectory, in: .userDomainMask)[0]
            .path
        
        do {
            client = try OmegaClient(dataDir: dataDir, identity: keypair)
        } catch {
            print("Failed to create client: \(error)")
        }
    }
    
    func sendMessage(recipient: String, text: String) {
        do {
            let messageId = try client?.sendTextMessage(
                recipient: recipient,
                text: text
            )
            print("Sent message: \(messageId ?? "unknown")")
        } catch {
            print("Failed to send: \(error)")
        }
    }
    
    private func loadOrGenerateKeypair() -> OmegaKeypair {
        let keychain = KeychainWrapper.standard
        
        if let privateKeyData = keychain.data(forKey: "omega_private_key") {
            // Carregar existente
            return OmegaKeypair.fromBytes(bytes: [UInt8](privateKeyData))
        } else {
            // Gerar novo
            let kp = OmegaKeypair()
            
            // Salvar no Keychain
            keychain.set(Data(kp.toBytes()), forKey: "omega_private_key")
            
            return kp
        }
    }
}

// EventCallback implementation
class OmegaEventHandler: EventCallback {
    var onMessageReceived: ((MessageData) -> Void)?
    
    func onMessageReceived(message: MessageData) {
        DispatchQueue.main.async {
            self.onMessageReceived?(message)
        }
    }
    
    func onMessageSent(messageId: String) {
        print("Message sent: \(messageId)")
    }
    
    func onPeerConnected(peerId: String) {
        print("Peer connected: \(peerId)")
    }
    
    func onError(error: String) {
        print("Error: \(error)")
    }
}
```

---

Esses são os arquivos base para começar o desenvolvimento do **omega-core**. A estrutura modular permite trabalhar em cada componente independentemente e ir testando progressivamente.

**Próximo passo sugerido:** Implementar `identity` e `crypto` modules primeiro, depois `network`, depois `storage`, e por último `sync`.
