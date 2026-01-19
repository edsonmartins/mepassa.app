//! Storage module
//!
//! Local SQLite storage for messages, contacts, and groups.

// pub mod database;
// pub mod schema;
// pub mod migrations;
// pub mod messages;
// pub mod contacts;
// pub mod groups;

// pub use database::Database;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Migration failed: {0}")]
    MigrationFailed(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;
