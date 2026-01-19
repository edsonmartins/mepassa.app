//! Cryptography module
//!
//! Implements Signal Protocol E2E encryption (Double Ratchet, X3DH).

pub mod signal;
pub mod session;
pub mod ratchet;
pub mod group;

pub use signal::{X3DH, EncryptedMessage, encrypt_message, decrypt_message};
pub use session::{Session, SessionManager};
pub use ratchet::RatchetState;
pub use group::{SenderKey, GroupSession, GroupSessionManager};

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
