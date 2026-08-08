//! Synchronization module (placeholder)
//!
//! Multi-device sync foi adiado: este módulo hoje só define o tipo de erro e
//! serve de ponto de entrada para a implementação futura. O anúncio de
//! "CRDTs for multi-device sync" foi removido do `lib.rs` (doc drift).
//!
//! Para implementar de fato (fora do escopo de homologação):
//! - CRDT via Automerge (`automerge = "0.5"`, hoje comentado no Cargo.toml)
//! - link de devices e protocolo de sincronização
//! - decidir o quê sincronizar (conversas, contatos, prekeys, configurações)

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Sync failed: {0}")]
    SyncFailed(String),

    #[error("Device not linked")]
    DeviceNotLinked,

    #[error("Conflict resolution failed")]
    ConflictResolutionFailed,
}

pub type Result<T> = std::result::Result<T, SyncError>;
