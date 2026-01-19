//! Synchronization module
//!
//! Multi-device sync using CRDTs (Automerge).

// pub mod crdt;
// pub mod device;
// pub mod protocol;

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
