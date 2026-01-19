//! Secure storage for identity and prekeys
//!
//! This module provides secure storage of cryptographic keys using
//! platform-specific secure storage (Keychain on iOS/macOS, Keystore on Android).
//!
//! **Note**: This is the Rust interface. Platform-specific implementations
//! are provided via FFI from the host application.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::identity::{Keypair, PreKeyPool};
use crate::utils::error::{Result, MePassaError};

/// Identity with keypair and prekey pool
#[derive(Clone)]
pub struct Identity {
    /// Main signing keypair (Ed25519)
    keypair: Keypair,
    /// Peer ID derived from keypair
    peer_id: String,
    /// PreKey pool for Signal Protocol
    prekey_pool: Option<PreKeyPool>,
}

impl Identity {
    /// Generate a new identity with prekeys
    ///
    /// # Arguments
    ///
    /// * `prekey_count` - Number of one-time prekeys to generate (default: 100)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mepassa_core::identity::Identity;
    ///
    /// let identity = Identity::generate(100);
    /// println!("Peer ID: {}", identity.peer_id());
    /// ```
    pub fn generate(prekey_count: usize) -> Self {
        let keypair = Keypair::generate();
        let peer_id = keypair.peer_id();
        let prekey_pool = Some(PreKeyPool::new(keypair.clone(), prekey_count));

        Self {
            keypair,
            peer_id,
            prekey_pool,
        }
    }

    /// Create identity from existing keypair
    pub fn from_keypair(keypair: Keypair) -> Self {
        let peer_id = keypair.peer_id();

        Self {
            keypair,
            peer_id,
            prekey_pool: None,
        }
    }

    /// Get peer ID
    pub fn peer_id(&self) -> &str {
        &self.peer_id
    }

    /// Get keypair reference
    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    /// Get mutable prekey pool reference
    pub fn prekey_pool_mut(&mut self) -> Option<&mut PreKeyPool> {
        self.prekey_pool.as_mut()
    }

    /// Get prekey pool reference
    pub fn prekey_pool(&self) -> Option<&PreKeyPool> {
        self.prekey_pool.as_ref()
    }

    /// Initialize prekey pool if not already initialized
    pub fn init_prekey_pool(&mut self, prekey_count: usize) {
        if self.prekey_pool.is_none() {
            self.prekey_pool = Some(PreKeyPool::new(self.keypair.clone(), prekey_count));
        }
    }

    /// Sign a message with this identity
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.keypair.sign(message)
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        self.keypair.verify(message, signature)
    }
}

impl std::fmt::Debug for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Identity")
            .field("peer_id", &self.peer_id)
            .field("has_prekey_pool", &self.prekey_pool.is_some())
            .finish_non_exhaustive()
    }
}

/// Serializable identity data for persistence
#[derive(Serialize, Deserialize)]
struct IdentityData {
    keypair_bytes: Vec<u8>,
    peer_id: String,
    // Prekey pool is persisted separately due to size
}

/// Storage interface for identity persistence
///
/// This trait defines the interface for storing and retrieving identities.
/// Implementations should use platform-specific secure storage:
/// - iOS/macOS: Keychain
/// - Android: Keystore + EncryptedSharedPreferences
/// - Desktop: OS keyring (keyring-rs)
pub trait IdentityStorage: Send + Sync {
    /// Save identity to secure storage
    fn save_identity(&self, identity: &Identity) -> Result<()>;

    /// Load identity from secure storage
    fn load_identity(&self) -> Result<Option<Identity>>;

    /// Delete identity from secure storage
    fn delete_identity(&self) -> Result<()>;

    /// Check if identity exists in storage
    fn has_identity(&self) -> Result<bool>;
}

/// File-based identity storage (for development/testing)
///
/// ⚠️ **WARNING**: This stores keys in plaintext. DO NOT use in production!
/// Use platform-specific secure storage instead.
pub struct FileIdentityStorage {
    data_dir: PathBuf,
}

impl FileIdentityStorage {
    /// Create a new file-based storage
    ///
    /// # Arguments
    ///
    /// * `data_dir` - Directory for storing identity file
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Self {
        Self {
            data_dir: data_dir.as_ref().to_path_buf(),
        }
    }

    fn identity_path(&self) -> PathBuf {
        self.data_dir.join("identity.json")
    }
}

impl IdentityStorage for FileIdentityStorage {
    fn save_identity(&self, identity: &Identity) -> Result<()> {
        // Ensure data directory exists
        std::fs::create_dir_all(&self.data_dir).map_err(|e| {
            MePassaError::Storage(format!("Failed to create data directory: {}", e))
        })?;

        let data = IdentityData {
            keypair_bytes: identity.keypair.to_bytes().to_vec(),
            peer_id: identity.peer_id.clone(),
        };

        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| MePassaError::Storage(format!("Failed to serialize identity: {}", e)))?;

        std::fs::write(self.identity_path(), json)
            .map_err(|e| MePassaError::Storage(format!("Failed to write identity: {}", e)))?;

        Ok(())
    }

    fn load_identity(&self) -> Result<Option<Identity>> {
        let path = self.identity_path();

        if !path.exists() {
            return Ok(None);
        }

        let json = std::fs::read_to_string(&path)
            .map_err(|e| MePassaError::Storage(format!("Failed to read identity: {}", e)))?;

        let data: IdentityData = serde_json::from_str(&json)
            .map_err(|e| MePassaError::Storage(format!("Failed to deserialize identity: {}", e)))?;

        let keypair = Keypair::from_bytes(&data.keypair_bytes)?;
        let identity = Identity::from_keypair(keypair);

        Ok(Some(identity))
    }

    fn delete_identity(&self) -> Result<()> {
        let path = self.identity_path();

        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| MePassaError::Storage(format!("Failed to delete identity: {}", e)))?;
        }

        Ok(())
    }

    fn has_identity(&self) -> Result<bool> {
        Ok(self.identity_path().exists())
    }
}

/// In-memory identity storage (for testing)
pub struct MemoryIdentityStorage {
    identity: std::sync::Mutex<Option<Identity>>,
}

impl MemoryIdentityStorage {
    /// Create a new in-memory storage
    pub fn new() -> Self {
        Self {
            identity: std::sync::Mutex::new(None),
        }
    }
}

impl Default for MemoryIdentityStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityStorage for MemoryIdentityStorage {
    fn save_identity(&self, identity: &Identity) -> Result<()> {
        let mut guard = self
            .identity
            .lock()
            .map_err(|e| MePassaError::Storage(format!("Lock poisoned: {}", e)))?;

        *guard = Some(identity.clone());
        Ok(())
    }

    fn load_identity(&self) -> Result<Option<Identity>> {
        let guard = self
            .identity
            .lock()
            .map_err(|e| MePassaError::Storage(format!("Lock poisoned: {}", e)))?;

        Ok(guard.clone())
    }

    fn delete_identity(&self) -> Result<()> {
        let mut guard = self
            .identity
            .lock()
            .map_err(|e| MePassaError::Storage(format!("Lock poisoned: {}", e)))?;

        *guard = None;
        Ok(())
    }

    fn has_identity(&self) -> Result<bool> {
        let guard = self
            .identity
            .lock()
            .map_err(|e| MePassaError::Storage(format!("Lock poisoned: {}", e)))?;

        Ok(guard.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_generation() {
        let identity = Identity::generate(10);
        assert!(identity.peer_id().starts_with("mepassa_"));
        assert!(identity.prekey_pool().is_some());
    }

    #[test]
    fn test_identity_from_keypair() {
        let keypair = Keypair::generate();
        let identity = Identity::from_keypair(keypair);

        assert!(identity.peer_id().starts_with("mepassa_"));
        assert!(identity.prekey_pool().is_none());
    }

    #[test]
    fn test_init_prekey_pool() {
        let keypair = Keypair::generate();
        let mut identity = Identity::from_keypair(keypair);

        assert!(identity.prekey_pool().is_none());

        identity.init_prekey_pool(50);

        assert!(identity.prekey_pool().is_some());
        assert_eq!(identity.prekey_pool().unwrap().prekey_count(), 50);
    }

    #[test]
    fn test_memory_storage() {
        let storage = MemoryIdentityStorage::new();

        // Initially empty
        assert!(!storage.has_identity().unwrap());
        assert!(storage.load_identity().unwrap().is_none());

        // Save identity
        let identity = Identity::generate(10);
        let peer_id = identity.peer_id().to_string();

        storage.save_identity(&identity).unwrap();

        // Verify saved
        assert!(storage.has_identity().unwrap());

        let loaded = storage.load_identity().unwrap().unwrap();
        assert_eq!(loaded.peer_id(), peer_id);

        // Delete
        storage.delete_identity().unwrap();
        assert!(!storage.has_identity().unwrap());
    }

    #[test]
    fn test_file_storage() {
        let temp_dir = std::env::temp_dir().join("mepassa_test_identity");
        let storage = FileIdentityStorage::new(&temp_dir);

        // Clean up before test
        let _ = storage.delete_identity();

        // Initially empty
        assert!(!storage.has_identity().unwrap());

        // Save identity
        let identity = Identity::generate(10);
        let peer_id = identity.peer_id().to_string();

        storage.save_identity(&identity).unwrap();

        // Verify file exists
        assert!(temp_dir.join("identity.json").exists());

        // Load identity
        let loaded = storage.load_identity().unwrap().unwrap();
        assert_eq!(loaded.peer_id(), peer_id);

        // Clean up
        storage.delete_identity().unwrap();
        assert!(!storage.has_identity().unwrap());

        // Clean up directory
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_identity_sign_verify() {
        let identity = Identity::generate(10);
        let message = b"Hello, MePassa!";

        let signature = identity.sign(message);
        let result = identity.verify(message, &signature);

        assert!(result.is_ok());
    }
}
