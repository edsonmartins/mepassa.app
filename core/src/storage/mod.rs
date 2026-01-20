//! Storage module
//!
//! Local SQLite storage for messages, contacts, and groups.

pub mod contacts;
pub mod database;
pub mod groups;
pub mod messages;
pub mod migrations;
pub mod schema;

pub use contacts::{Contact, NewContact, UpdateContact};
pub use database::Database;
pub use groups::{Group, GroupMember, MemberRole, NewGroup, NewGroupMember};
pub use messages::{Conversation, Message, MessageStatus, NewMessage, UpdateMessage};
pub use migrations::{migrate, needs_migration};
pub use schema::{init_fts, init_schema, SCHEMA_VERSION};

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

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::DatabaseError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, StorageError>;
