//! Public API module
//!
//! Main Client API for MePassa.

// TODO: Implement these modules
// pub mod client;
// pub mod events;
// pub mod callbacks;

// pub use client::{Client, ClientBuilder};
// pub use events::Event;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Client not initialized")]
    NotInitialized,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Crypto error: {0}")]
    CryptoError(String),

    #[error("Storage error: {0}")]
    StorageError(String),
}

pub type Result<T> = std::result::Result<T, ApiError>;
