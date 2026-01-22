//! Error types for MePassa

use thiserror::Error;
use crate::storage::StorageError;

/// Result type alias
pub type Result<T> = std::result::Result<T, MePassaError>;

/// Main error type for MePassa
#[derive(Error, Debug)]
pub enum MePassaError {
    #[error("Identity error: {0}")]
    Identity(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<StorageError> for MePassaError {
    fn from(err: StorageError) -> Self {
        MePassaError::Storage(err.to_string())
    }
}

impl From<rusqlite::Error> for MePassaError {
    fn from(err: rusqlite::Error) -> Self {
        MePassaError::Storage(err.to_string())
    }
}
