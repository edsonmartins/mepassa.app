//! Error types for MePassa

use thiserror::Error;

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

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}
