//! Storage encryption helpers
//!
//! Encrypt/decrypt message content for at-rest storage using AES-GCM.

use serde::{Deserialize, Serialize};

use crate::crypto::signal::{decrypt_message, encrypt_message, EncryptedMessage};
use crate::utils::error::{MePassaError, Result};

#[derive(Debug, Serialize, Deserialize)]
struct StorageEnvelope {
    nonce: [u8; 12],
    ciphertext: Vec<u8>,
}

pub fn encrypt_for_storage(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
    let encrypted = encrypt_message(plaintext, key)?;
    let envelope = StorageEnvelope {
        nonce: encrypted.nonce,
        ciphertext: encrypted.ciphertext,
    };
    bincode::serialize(&envelope)
        .map_err(|e| MePassaError::Crypto(format!("Storage encrypt serialize failed: {}", e)))
}

pub fn decrypt_for_storage(key: &[u8; 32], blob: &[u8]) -> Result<Vec<u8>> {
    let envelope: StorageEnvelope = bincode::deserialize(blob)
        .map_err(|e| MePassaError::Crypto(format!("Storage decrypt deserialize failed: {}", e)))?;
    let encrypted = EncryptedMessage {
        nonce: envelope.nonce,
        ciphertext: envelope.ciphertext,
    };
    decrypt_message(&encrypted, key)
}
