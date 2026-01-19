//! Database Migrations
//!
//! Manages schema migrations for the SQLite database.

use super::{Database, Result, StorageError};
use crate::storage::schema::{init_fts, init_schema, SCHEMA_VERSION};

/// Migration definition
struct Migration {
    version: i32,
    description: &'static str,
    up: fn(&Database) -> Result<()>,
}

/// All migrations in order
const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    description: "Initial schema with contacts.username support",
    up: migrate_to_v1,
}];

/// Migrate database to latest version
pub fn migrate(db: &Database) -> Result<()> {
    let current_version = db.get_version()?;

    tracing::info!(
        "Database at version {}, target version {}",
        current_version,
        SCHEMA_VERSION
    );

    if current_version == SCHEMA_VERSION {
        tracing::info!("Database is up to date");
        return Ok(());
    }

    if current_version > SCHEMA_VERSION {
        return Err(StorageError::MigrationFailed(format!(
            "Database version ({}) is newer than schema version ({}). Please update the app.",
            current_version, SCHEMA_VERSION
        )));
    }

    // Run migrations in order
    for migration in MIGRATIONS {
        if migration.version > current_version {
            tracing::info!(
                "Running migration {}: {}",
                migration.version,
                migration.description
            );

            (migration.up)(db).map_err(|e| {
                StorageError::MigrationFailed(format!(
                    "Migration {} failed: {}",
                    migration.version, e
                ))
            })?;

            db.set_version(migration.version)?;

            tracing::info!("Migration {} completed", migration.version);
        }
    }

    tracing::info!("All migrations completed successfully");
    Ok(())
}

/// Migration to version 1: Initial schema
fn migrate_to_v1(db: &Database) -> Result<()> {
    // Create all tables
    init_schema(db)?;

    // Create FTS tables
    init_fts(db)?;

    Ok(())
}

/// Check if database needs migration
pub fn needs_migration(db: &Database) -> Result<bool> {
    let current_version = db.get_version()?;
    Ok(current_version < SCHEMA_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_fresh_database() {
        let db = Database::in_memory().unwrap();
        assert_eq!(db.get_version().unwrap(), 0);

        migrate(&db).unwrap();

        assert_eq!(db.get_version().unwrap(), SCHEMA_VERSION);
        assert!(db.table_exists("contacts").unwrap());
    }

    #[test]
    fn test_migrate_already_up_to_date() {
        let db = Database::in_memory().unwrap();
        migrate(&db).unwrap();

        // Run migration again (should be no-op)
        let result = migrate(&db);
        assert!(result.is_ok());
        assert_eq!(db.get_version().unwrap(), SCHEMA_VERSION);
    }

    #[test]
    fn test_needs_migration() {
        let db = Database::in_memory().unwrap();
        assert!(needs_migration(&db).unwrap());

        migrate(&db).unwrap();
        assert!(!needs_migration(&db).unwrap());
    }

    #[test]
    fn test_migration_creates_username_column() {
        let db = Database::in_memory().unwrap();
        migrate(&db).unwrap();

        // Verify username column exists and works
        db.conn()
            .execute(
                "INSERT INTO contacts (peer_id, username, public_key) VALUES (?1, ?2, ?3)",
                rusqlite::params!["test_peer", "alice", vec![0u8; 32]],
            )
            .unwrap();

        let username: Option<String> = db
            .conn()
            .query_row(
                "SELECT username FROM contacts WHERE peer_id = ?1",
                ["test_peer"],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(username, Some("alice".to_string()));
    }
}
