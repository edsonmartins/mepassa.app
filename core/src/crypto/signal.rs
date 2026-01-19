//! Simplified Signal Protocol Implementation
//!
//! This module implements a simplified version of the Signal Protocol for end-to-end encryption:
//! - Simplified X3DH for initial key agreement (using only X25519 prekeys)
//! - AES-GCM for message encryption
//!
//! Note: This is a simplified implementation for MVP. Production should use libsignal-protocol.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng as AeadOsRng},
    Aes256Gcm, Nonce,
};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

use crate::identity::prekeys::PreKeyBundle;
use crate::utils::error::{Result, MePassaError};

/// Simplified X3DH (Extended Triple Diffie-Hellman) for initial key agreement
///
/// This simplified version only uses X25519 prekeys, not identity keys.
/// Performs 2 Diffie-Hellman operations:
/// - DH1: alice_ephemeral × bob_signed_prekey
/// - DH2: alice_ephemeral × bob_one_time_prekey (if available)
pub struct X3DH;

impl X3DH {
    /// Initiate X3DH as Alice (sender)
    ///
    /// # Arguments
    ///
    /// * `bob_bundle` - Bob's prekey bundle
    ///
    /// # Returns
    ///
    /// Tuple of (shared_secret, ephemeral_public_key)
    pub fn initiate(bob_bundle: &PreKeyBundle) -> Result<([u8; 32], [u8; 32])> {
        // Generate ephemeral key for this session
        let ephemeral_secret = x25519_dalek::StaticSecret::random_from_rng(AeadOsRng);
        let ephemeral_public = x25519_dalek::PublicKey::from(&ephemeral_secret);

        let bob_signed_prekey_public = x25519_dalek::PublicKey::from(bob_bundle.signed_prekey);

        // DH1: alice_ephemeral × bob_signed_prekey
        let dh1 = ephemeral_secret.diffie_hellman(&bob_signed_prekey_public);

        // Concatenate DH outputs
        let mut dh_output = Vec::new();
        dh_output.extend_from_slice(dh1.as_bytes());

        // DH2: alice_ephemeral × bob_one_time_prekey (if available)
        if let Some(ref otpk) = bob_bundle.one_time_prekey {
            let bob_otpk_public = x25519_dalek::PublicKey::from(otpk.public_key);
            let dh2 = ephemeral_secret.diffie_hellman(&bob_otpk_public);
            dh_output.extend_from_slice(dh2.as_bytes());
        }

        // Derive shared secret using HKDF
        let shared_secret = Self::derive_shared_secret(&dh_output)?;

        Ok((shared_secret, ephemeral_public.to_bytes()))
    }

    /// Respond to X3DH as Bob (receiver)
    ///
    /// # Arguments
    ///
    /// * `bob_signed_prekey_secret` - Bob's signed prekey secret
    /// * `bob_one_time_prekey_secret` - Bob's one-time prekey secret (optional)
    /// * `alice_ephemeral_public` - Alice's ephemeral public key
    ///
    /// # Returns
    ///
    /// Shared secret (32 bytes)
    pub fn respond(
        bob_signed_prekey_secret: &[u8; 32],
        bob_one_time_prekey_secret: Option<&[u8; 32]>,
        alice_ephemeral_public: &[u8; 32],
    ) -> Result<[u8; 32]> {
        let bob_signed_prekey = x25519_dalek::StaticSecret::from(*bob_signed_prekey_secret);
        let alice_ephemeral_public = x25519_dalek::PublicKey::from(*alice_ephemeral_public);

        // DH1: bob_signed_prekey × alice_ephemeral
        let dh1 = bob_signed_prekey.diffie_hellman(&alice_ephemeral_public);

        let mut dh_output = Vec::new();
        dh_output.extend_from_slice(dh1.as_bytes());

        // DH2: bob_one_time_prekey × alice_ephemeral (if available)
        if let Some(otpk_secret) = bob_one_time_prekey_secret {
            let bob_otpk = x25519_dalek::StaticSecret::from(*otpk_secret);
            let dh2 = bob_otpk.diffie_hellman(&alice_ephemeral_public);
            dh_output.extend_from_slice(dh2.as_bytes());
        }

        Self::derive_shared_secret(&dh_output)
    }

    /// Derive shared secret from concatenated DH outputs using HKDF
    fn derive_shared_secret(dh_output: &[u8]) -> Result<[u8; 32]> {
        let hkdf = Hkdf::<Sha256>::new(Some(b"mepassa-x3dh-v1"), dh_output);
        let mut shared_secret = [0u8; 32];
        hkdf.expand(b"shared-secret", &mut shared_secret)
            .map_err(|e| MePassaError::Crypto(format!("HKDF expand failed: {}", e)))?;

        Ok(shared_secret)
    }
}

/// Encrypted message envelope
#[derive(Debug, Clone)]
pub struct EncryptedMessage {
    /// Nonce for AES-GCM (12 bytes)
    pub nonce: [u8; 12],
    /// Ciphertext (variable length)
    pub ciphertext: Vec<u8>,
}

/// Encrypt a message using AES-256-GCM
///
/// # Arguments
///
/// * `plaintext` - Message to encrypt
/// * `key` - 32-byte encryption key
///
/// # Returns
///
/// EncryptedMessage with nonce and ciphertext
pub fn encrypt_message(plaintext: &[u8], key: &[u8; 32]) -> Result<EncryptedMessage> {
    let cipher = Aes256Gcm::new(key.into());

    // Generate random nonce (12 bytes for AES-GCM)
    let mut nonce_bytes = [0u8; 12];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| MePassaError::Crypto(format!("Encryption failed: {}", e)))?;

    Ok(EncryptedMessage {
        nonce: nonce_bytes,
        ciphertext,
    })
}

/// Decrypt a message using AES-256-GCM
///
/// # Arguments
///
/// * `encrypted` - Encrypted message envelope
/// * `key` - 32-byte encryption key
///
/// # Returns
///
/// Decrypted plaintext
pub fn decrypt_message(encrypted: &EncryptedMessage, key: &[u8; 32]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(&encrypted.nonce);

    let plaintext = cipher
        .decrypt(nonce, encrypted.ciphertext.as_ref())
        .map_err(|e| MePassaError::Crypto(format!("Decryption failed: {}", e)))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Identity;

    #[test]
    fn test_x3dh_key_agreement() {
        // Bob generates identity with prekeys
        let bob = Identity::generate(10);
        let mut bob_mut = bob.clone();

        // In a real scenario, Bob would have these secrets stored locally
        // We save them before consuming the prekey in the bundle
        let pool = bob_mut.prekey_pool().unwrap();
        let bob_signed_prekey_secret = pool.signed_prekey().secret_bytes();

        // Get first available one-time prekey secret
        let bob_one_time_prekey_secret: Option<[u8; 32]> = pool
            .prekey_count()
            .gt(&0)
            .then(|| {
                // In real scenario, Bob would know which prekey was used from the bundle
                // For test, we simulate by using any available prekey
                // This works because get_bundle() will use one of the available prekeys
                pool.get_prekey(2) // Pool starts with prekey IDs starting from 2
                    .map(|pk| pk.secret_bytes())
            })
            .flatten();

        // Now get the bundle (which consumes one prekey)
        let bob_bundle = bob_mut.prekey_pool_mut().unwrap().get_bundle();

        // Alice initiates X3DH
        let (alice_shared_secret, alice_ephemeral_pub) = X3DH::initiate(&bob_bundle).unwrap();

        // Bob responds to X3DH using the saved secrets
        let bob_shared_secret = X3DH::respond(
            &bob_signed_prekey_secret,
            bob_one_time_prekey_secret.as_ref(),
            &alice_ephemeral_pub,
        )
        .unwrap();

        // Both should derive the same shared secret
        assert_eq!(alice_shared_secret, bob_shared_secret);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = [42u8; 32];
        let plaintext = b"Hello, MePassa!";

        let encrypted = encrypt_message(plaintext, &key).unwrap();
        let decrypted = decrypt_message(&encrypted, &key).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_encrypt_decrypt_different_key_fails() {
        let key1 = [42u8; 32];
        let key2 = [99u8; 32];
        let plaintext = b"Secret message";

        let encrypted = encrypt_message(plaintext, &key1).unwrap();
        let result = decrypt_message(&encrypted, &key2);

        assert!(result.is_err());
    }

    #[test]
    fn test_nonce_randomness() {
        let key = [42u8; 32];
        let plaintext = b"Test message";

        let encrypted1 = encrypt_message(plaintext, &key).unwrap();
        let encrypted2 = encrypt_message(plaintext, &key).unwrap();

        // Nonces should be different (randomized)
        assert_ne!(encrypted1.nonce, encrypted2.nonce);
        // But both should decrypt correctly
        assert_eq!(decrypt_message(&encrypted1, &key).unwrap(), plaintext);
        assert_eq!(decrypt_message(&encrypted2, &key).unwrap(), plaintext);
    }

    #[test]
    fn test_e2e_alice_to_bob() {
        // Setup: Bob generates identity and publishes prekey bundle
        let bob = Identity::generate(0); // No one-time prekeys for simplicity
        let mut bob_mut = bob.clone();

        // Bob saves his signed prekey secret before publishing bundle
        let bob_signed_prekey_secret = bob_mut
            .prekey_pool()
            .unwrap()
            .signed_prekey()
            .secret_bytes();

        // Publish the bundle (without one-time prekey)
        let bob_bundle = bob_mut.prekey_pool_mut().unwrap().get_bundle();

        // Alice wants to send encrypted message to Bob
        let alice_message = b"Secret message from Alice to Bob!";

        // Step 1: Alice performs X3DH to establish shared secret
        let (shared_secret, alice_ephemeral_pub) = X3DH::initiate(&bob_bundle).unwrap();

        // Step 2: Alice encrypts message with shared secret
        let encrypted = encrypt_message(alice_message, &shared_secret).unwrap();

        // Step 3: Bob receives Alice's ephemeral public key and encrypted message
        // Bob performs X3DH on his side to derive same shared secret
        let bob_shared_secret = X3DH::respond(
            &bob_signed_prekey_secret,
            None, // No one-time prekey
            &alice_ephemeral_pub,
        )
        .unwrap();

        // Step 4: Bob decrypts message
        let decrypted = decrypt_message(&encrypted, &bob_shared_secret).unwrap();

        // Verify: Bob received Alice's original message
        assert_eq!(alice_message, decrypted.as_slice());
    }
}
