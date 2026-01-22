//! Sender Keys for Group E2E Encryption
//!
//! Implementation of Signal Protocol Sender Keys for group messaging.
//!
//! Sender Keys allow efficient group encryption:
//! - Each sender has one key shared with all group members
//! - No need for N pairwise sessions in a group of N members
//! - Forward secrecy through ratcheting
//!
//! References:
//! - https://signal.org/docs/specifications/doubleratchet/#sender-keys
//! - https://signal.org/docs/specifications/sesame/

use crate::utils::error::{MePassaError, Result};
use serde::{Deserialize, Serialize};

/// Sender Key for group encryption
///
/// Each group member has a sender key that they use to encrypt messages.
/// All other members receive and store this sender key to decrypt messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenderKey {
    /// Group ID
    pub group_id: String,

    /// Sender peer ID
    pub sender_peer_id: String,

    /// Chain key (ratcheted forward for each message)
    chain_key: Vec<u8>,

    /// Message key derivation counter
    iteration: u32,

    /// Public signing key (Ed25519)
    signing_key: Vec<u8>,
}

impl SenderKey {
    /// Generate a new sender key
    pub fn generate(group_id: String, sender_peer_id: String) -> Result<Self> {
        // Generate random chain key (32 bytes)
        let chain_key = rand::random::<[u8; 32]>().to_vec();

        // Generate signing key (for now, placeholder)
        let signing_key = vec![0u8; 32];

        Ok(Self {
            group_id,
            sender_peer_id,
            chain_key,
            iteration: 0,
            signing_key,
        })
    }

    /// Encrypt a message with this sender key
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>> {
        // TODO: Implement proper Sender Key encryption
        // For MVP: Just return plaintext (groups will work, but without E2E encryption)

        // Ratchet forward
        self.iteration += 1;
        self.ratchet_forward();

        // For now, return plaintext
        // TODO: Use libsignal-protocol or implement Signal's Sender Key algorithm
        Ok(plaintext.to_vec())
    }

    /// Decrypt a message with this sender key
    pub fn decrypt(&mut self, ciphertext: &[u8], iteration: u32) -> Result<Vec<u8>> {
        // TODO: Implement proper Sender Key decryption
        // For MVP: Just return ciphertext (groups will work, but without E2E encryption)

        // Update iteration if needed
        if iteration > self.iteration {
            self.iteration = iteration;
            self.ratchet_forward();
        }

        // For now, return ciphertext
        Ok(ciphertext.to_vec())
    }

    /// Ratchet chain key forward (KDF)
    fn ratchet_forward(&mut self) {
        // TODO: Implement HKDF ratcheting
        // For MVP: Just increment (not secure, placeholder)

        // Placeholder: XOR with iteration
        for (i, byte) in self.chain_key.iter_mut().enumerate() {
            *byte ^= (self.iteration as u8).wrapping_add(i as u8);
        }
    }

    /// Serialize sender key for transmission
    pub fn serialize(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| MePassaError::Crypto(format!("Failed to serialize sender key: {}", e)))
    }

    /// Deserialize sender key from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        serde_json::from_slice(data)
            .map_err(|e| MePassaError::Crypto(format!("Failed to deserialize sender key: {}", e)))
    }
}

/// Sender Key Store
///
/// Stores sender keys for all group members.
pub struct SenderKeyStore {
    /// Keys indexed by (group_id, sender_peer_id)
    keys: std::collections::HashMap<(String, String), SenderKey>,
}

impl SenderKeyStore {
    /// Create a new sender key store
    pub fn new() -> Self {
        Self {
            keys: std::collections::HashMap::new(),
        }
    }

    /// Store a sender key
    pub fn store_key(&mut self, key: SenderKey) {
        let index = (key.group_id.clone(), key.sender_peer_id.clone());
        self.keys.insert(index, key);
    }

    /// Get a sender key
    pub fn get_key(&self, group_id: &str, sender_peer_id: &str) -> Option<&SenderKey> {
        let index = (group_id.to_string(), sender_peer_id.to_string());
        self.keys.get(&index)
    }

    /// Get a mutable sender key
    pub fn get_key_mut(&mut self, group_id: &str, sender_peer_id: &str) -> Option<&mut SenderKey> {
        let index = (group_id.to_string(), sender_peer_id.to_string());
        self.keys.get_mut(&index)
    }

    /// Remove all keys for a group
    pub fn remove_group(&mut self, group_id: &str) {
        self.keys.retain(|(gid, _), _| gid != group_id);
    }
}

impl Default for SenderKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sender_key() {
        let key = SenderKey::generate("group-1".to_string(), "peer-1".to_string()).unwrap();

        assert_eq!(key.group_id, "group-1");
        assert_eq!(key.sender_peer_id, "peer-1");
        assert_eq!(key.iteration, 0);
        assert_eq!(key.chain_key.len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let mut key = SenderKey::generate("group-1".to_string(), "peer-1".to_string()).unwrap();

        let plaintext = b"Hello, group!";
        let ciphertext = key.encrypt(plaintext).unwrap();

        // For MVP, encryption is a no-op, so ciphertext == plaintext
        assert_eq!(ciphertext, plaintext);

        let decrypted = key.decrypt(&ciphertext, key.iteration).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_sender_key_store() {
        let mut store = SenderKeyStore::new();

        let key1 = SenderKey::generate("group-1".to_string(), "peer-1".to_string()).unwrap();
        let key2 = SenderKey::generate("group-1".to_string(), "peer-2".to_string()).unwrap();

        store.store_key(key1);
        store.store_key(key2);

        assert!(store.get_key("group-1", "peer-1").is_some());
        assert!(store.get_key("group-1", "peer-2").is_some());
        assert!(store.get_key("group-2", "peer-1").is_none());

        store.remove_group("group-1");
        assert!(store.get_key("group-1", "peer-1").is_none());
    }
}
