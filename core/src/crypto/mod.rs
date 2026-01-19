//! Cryptography module
//!
//! Implements Signal Protocol E2E encryption (Double Ratchet, X3DH).

// pub mod session;
// pub mod signal;
// pub mod ratchet;
// pub mod group;

// pub use session::{Session, SessionManager};
// pub use signal::SignalProtocol;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Session not found")]
    SessionNotFound,

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Invalid message format")]
    InvalidMessageFormat,
}

pub type Result<T> = std::result::Result<T, CryptoError>;
