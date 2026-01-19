//! PreKey management for Signal Protocol
//!
//! This module handles X25519 prekeys used in the Extended Triple Diffie-Hellman (X3DH)
//! key agreement protocol. Prekeys enable asynchronous messaging where the recipient
//! doesn't need to be online during initial key exchange.

use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

use crate::utils::error::{Result, MePassaError};

// Custom serialization for [u8; 64] arrays
mod serde_bytes_64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(bytes)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
        if bytes.len() != 64 {
            return Err(serde::de::Error::custom(format!(
                "Expected 64 bytes, got {}",
                bytes.len()
            )));
        }
        let mut array = [0u8; 64];
        array.copy_from_slice(&bytes);
        Ok(array)
    }
}

/// A single X25519 prekey used for key agreement
#[derive(Clone)]
pub struct PreKey {
    /// Unique ID for this prekey
    pub id: u32,
    /// Private key (X25519)
    secret: StaticSecret,
    /// Public key (X25519)
    public: X25519PublicKey,
}

impl PreKey {
    /// Generate a new random prekey with given ID
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this prekey
    pub fn generate(id: u32) -> Self {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = X25519PublicKey::from(&secret);

        Self { id, secret, public }
    }

    /// Create a prekey from raw bytes (32 bytes for X25519)
    ///
    /// # Arguments
    ///
    /// * `id` - Prekey ID
    /// * `bytes` - 32-byte secret key
    ///
    /// # Errors
    ///
    /// Returns error if bytes length is not exactly 32
    pub fn from_bytes(id: u32, bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 32 {
            return Err(MePassaError::Identity(format!(
                "Invalid prekey length: expected 32 bytes, got {}",
                bytes.len()
            )));
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(bytes);

        let secret = StaticSecret::from(key_bytes);
        let public = X25519PublicKey::from(&secret);

        Ok(Self { id, secret, public })
    }

    /// Export the secret key as bytes (32 bytes)
    ///
    /// ⚠️ **WARNING**: Keep this secret!
    pub fn secret_bytes(&self) -> [u8; 32] {
        self.secret.to_bytes()
    }

    /// Get the public key as bytes (32 bytes)
    pub fn public_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }

    /// Perform Diffie-Hellman key agreement
    ///
    /// # Arguments
    ///
    /// * `their_public` - The other party's public key
    ///
    /// # Returns
    ///
    /// 32-byte shared secret
    pub fn diffie_hellman(&self, their_public: &[u8; 32]) -> [u8; 32] {
        let their_public = X25519PublicKey::from(*their_public);
        self.secret.diffie_hellman(&their_public).to_bytes()
    }
}

impl std::fmt::Debug for PreKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreKey")
            .field("id", &self.id)
            .field("public", &hex::encode(self.public_bytes()))
            .finish_non_exhaustive()
    }
}

/// Serializable prekey bundle for transmission
///
/// This is sent to other peers during key exchange (does not include secret key)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreKeyBundle {
    /// Identity key (Ed25519 public key as bytes)
    pub identity_key: [u8; 32],
    /// Signed prekey ID
    pub signed_prekey_id: u32,
    /// Signed prekey public bytes
    pub signed_prekey: [u8; 32],
    /// Signature over signed prekey
    #[serde(with = "serde_bytes_64")]
    pub signed_prekey_signature: [u8; 64],
    /// One-time prekey (optional - consumed after use)
    pub one_time_prekey: Option<OneTimePreKey>,
}

/// One-time prekey (consumed after first use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneTimePreKey {
    pub id: u32,
    pub public_key: [u8; 32],
}

/// Pool of prekeys for key agreement
///
/// Signal Protocol recommends maintaining:
/// - 1 signed prekey (rotated periodically)
/// - 100 one-time prekeys (replenished as consumed)
#[derive(Clone)]
pub struct PreKeyPool {
    /// Identity keypair (Ed25519) for signing
    identity_keypair: crate::identity::Keypair,
    /// Current signed prekey
    signed_prekey: PreKey,
    /// Signature over signed prekey
    signed_prekey_signature: [u8; 64],
    /// Pool of one-time prekeys
    one_time_prekeys: HashMap<u32, PreKey>,
    /// Next prekey ID to assign
    next_prekey_id: u32,
}

impl PreKeyPool {
    /// Create a new prekey pool with initial prekeys
    ///
    /// # Arguments
    ///
    /// * `identity_keypair` - The identity keypair for signing
    /// * `pool_size` - Number of one-time prekeys to generate (default: 100)
    pub fn new(identity_keypair: crate::identity::Keypair, pool_size: usize) -> Self {
        let mut pool = Self {
            identity_keypair: identity_keypair.clone(),
            signed_prekey: PreKey::generate(1),
            signed_prekey_signature: [0u8; 64],
            one_time_prekeys: HashMap::new(),
            next_prekey_id: 2,
        };

        // Sign the signed prekey
        pool.signed_prekey_signature = identity_keypair.sign(&pool.signed_prekey.public_bytes());

        // Generate one-time prekeys
        pool.replenish_prekeys(pool_size);

        pool
    }

    /// Get the current signed prekey
    pub fn signed_prekey(&self) -> &PreKey {
        &self.signed_prekey
    }

    /// Get the signed prekey signature
    pub fn signed_prekey_signature(&self) -> [u8; 64] {
        self.signed_prekey_signature
    }

    /// Replenish one-time prekeys to reach target count
    ///
    /// # Arguments
    ///
    /// * `target_count` - Desired number of prekeys in the pool
    pub fn replenish_prekeys(&mut self, target_count: usize) {
        let current_count = self.one_time_prekeys.len();

        if current_count >= target_count {
            return;
        }

        let to_generate = target_count - current_count;

        for _ in 0..to_generate {
            let id = self.next_prekey_id;
            self.next_prekey_id += 1;

            let prekey = PreKey::generate(id);
            self.one_time_prekeys.insert(id, prekey);
        }
    }

    /// Get a prekey bundle for key exchange
    ///
    /// This consumes one one-time prekey from the pool.
    ///
    /// # Returns
    ///
    /// PreKeyBundle that can be serialized and sent to another peer
    pub fn get_bundle(&mut self) -> PreKeyBundle {
        let one_time_prekey = self.consume_one_time_prekey();

        PreKeyBundle {
            identity_key: self.identity_keypair.public_key_bytes(),
            signed_prekey_id: self.signed_prekey.id,
            signed_prekey: self.signed_prekey.public_bytes(),
            signed_prekey_signature: self.signed_prekey_signature,
            one_time_prekey: one_time_prekey.map(|pk| OneTimePreKey {
                id: pk.id,
                public_key: pk.public_bytes(),
            }),
        }
    }

    /// Consume and remove one one-time prekey from the pool
    ///
    /// Returns None if pool is empty (should trigger replenishment)
    fn consume_one_time_prekey(&mut self) -> Option<PreKey> {
        if self.one_time_prekeys.is_empty() {
            return None;
        }

        // Get any prekey (doesn't matter which one)
        let id = *self.one_time_prekeys.keys().next()?;
        self.one_time_prekeys.remove(&id)
    }

    /// Get a specific one-time prekey by ID (without consuming)
    ///
    /// Used for processing incoming X3DH messages
    pub fn get_prekey(&self, id: u32) -> Option<&PreKey> {
        self.one_time_prekeys.get(&id)
    }

    /// Remove a specific one-time prekey after use
    pub fn remove_prekey(&mut self, id: u32) -> Option<PreKey> {
        self.one_time_prekeys.remove(&id)
    }

    /// Rotate the signed prekey
    ///
    /// Should be called periodically (e.g., every 7 days) for forward secrecy
    pub fn rotate_signed_prekey(&mut self) {
        let new_id = self.next_prekey_id;
        self.next_prekey_id += 1;

        self.signed_prekey = PreKey::generate(new_id);
        self.signed_prekey_signature = self
            .identity_keypair
            .sign(&self.signed_prekey.public_bytes());
    }

    /// Get count of remaining one-time prekeys
    pub fn prekey_count(&self) -> usize {
        self.one_time_prekeys.len()
    }

    /// Check if prekey pool needs replenishment
    ///
    /// Returns true if count is below threshold (20% of recommended 100)
    pub fn needs_replenishment(&self) -> bool {
        self.prekey_count() < 20
    }
}

impl std::fmt::Debug for PreKeyPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreKeyPool")
            .field("signed_prekey_id", &self.signed_prekey.id)
            .field("one_time_prekey_count", &self.one_time_prekeys.len())
            .field("next_prekey_id", &self.next_prekey_id)
            .finish_non_exhaustive()
    }
}

// Placeholder for compatibility with old code
pub type PreKeyStore = PreKeyPool;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prekey_generation() {
        let prekey = PreKey::generate(1);
        assert_eq!(prekey.id, 1);
        assert_eq!(prekey.public_bytes().len(), 32);
    }

    #[test]
    fn test_prekey_from_bytes() {
        let prekey1 = PreKey::generate(1);
        let bytes = prekey1.secret_bytes();

        let prekey2 = PreKey::from_bytes(1, &bytes).unwrap();

        assert_eq!(prekey1.public_bytes(), prekey2.public_bytes());
    }

    #[test]
    fn test_diffie_hellman() {
        let alice_prekey = PreKey::generate(1);
        let bob_prekey = PreKey::generate(2);

        let alice_shared = alice_prekey.diffie_hellman(&bob_prekey.public_bytes());
        let bob_shared = bob_prekey.diffie_hellman(&alice_prekey.public_bytes());

        assert_eq!(alice_shared, bob_shared);
    }

    #[test]
    fn test_prekey_pool_creation() {
        let identity = crate::identity::Keypair::generate();
        let pool = PreKeyPool::new(identity, 100);

        assert_eq!(pool.prekey_count(), 100);
        assert_eq!(pool.signed_prekey().id, 1);
    }

    #[test]
    fn test_prekey_bundle() {
        let identity = crate::identity::Keypair::generate();
        let mut pool = PreKeyPool::new(identity.clone(), 10);

        let bundle = pool.get_bundle();

        assert_eq!(bundle.identity_key, identity.public_key_bytes());
        assert!(bundle.one_time_prekey.is_some());
        assert_eq!(pool.prekey_count(), 9); // One consumed
    }

    #[test]
    fn test_prekey_consumption() {
        let identity = crate::identity::Keypair::generate();
        let mut pool = PreKeyPool::new(identity, 5);

        assert_eq!(pool.prekey_count(), 5);

        let _bundle1 = pool.get_bundle();
        assert_eq!(pool.prekey_count(), 4);

        let _bundle2 = pool.get_bundle();
        assert_eq!(pool.prekey_count(), 3);
    }

    #[test]
    fn test_prekey_replenishment() {
        let identity = crate::identity::Keypair::generate();
        let mut pool = PreKeyPool::new(identity, 10);

        // Consume all prekeys
        for _ in 0..10 {
            let _ = pool.get_bundle();
        }

        assert_eq!(pool.prekey_count(), 0);

        // Replenish
        pool.replenish_prekeys(100);
        assert_eq!(pool.prekey_count(), 100);
    }

    #[test]
    fn test_needs_replenishment() {
        let identity = crate::identity::Keypair::generate();
        let mut pool = PreKeyPool::new(identity, 100);

        assert!(!pool.needs_replenishment());

        // Consume until below threshold
        for _ in 0..85 {
            let _ = pool.get_bundle();
        }

        assert!(pool.needs_replenishment());
    }

    #[test]
    fn test_rotate_signed_prekey() {
        let identity = crate::identity::Keypair::generate();
        let mut pool = PreKeyPool::new(identity, 10);

        let old_id = pool.signed_prekey().id;
        let old_public = pool.signed_prekey().public_bytes();

        pool.rotate_signed_prekey();

        let new_id = pool.signed_prekey().id;
        let new_public = pool.signed_prekey().public_bytes();

        assert_ne!(old_id, new_id);
        assert_ne!(old_public, new_public);
    }

    #[test]
    fn test_bundle_serialization() {
        let identity = crate::identity::Keypair::generate();
        let mut pool = PreKeyPool::new(identity, 10);

        let bundle = pool.get_bundle();

        // Serialize to JSON
        let json = serde_json::to_string(&bundle).unwrap();
        assert!(!json.is_empty());

        // Deserialize back
        let deserialized: PreKeyBundle = serde_json::from_str(&json).unwrap();
        assert_eq!(bundle.identity_key, deserialized.identity_key);
        assert_eq!(bundle.signed_prekey_id, deserialized.signed_prekey_id);
    }
}
