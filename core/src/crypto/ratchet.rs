//! Double Ratchet for Forward Secrecy
//!
//! This module implements a simplified Double Ratchet algorithm inspired by
//! the Signal Protocol. It provides forward secrecy by deriving new encryption
//! keys for each message using HKDF key derivation.
//!
//! **Simplified Implementation:**
//! - Uses HKDF to derive sending and receiving keys from root keys
//! - Ratchets forward with each message
//! - Provides forward secrecy (compromise of current key doesn't affect past messages)
//!
//! **Note:** This is a simplified version for MVP. Production should use libsignal-protocol
//! which includes full Double Ratchet with Diffie-Hellman ratchet steps.

use hkdf::Hkdf;
use sha2::Sha256;

use crate::crypto::signal::{encrypt_message, decrypt_message, EncryptedMessage};
use crate::utils::error::{Result, MePassaError};

/// Ratchet State
///
/// Maintains the state of the Double Ratchet for a session.
/// Includes sending and receiving chain keys.
#[derive(Debug, Clone)]
pub struct RatchetState {
    /// Root key (32 bytes) - source of truth for key derivation
    pub root_key: [u8; 32],

    /// Sending chain key (32 bytes) - ratcheted forward with each sent message
    pub sending_chain_key: [u8; 32],

    /// Receiving chain key (32 bytes) - ratcheted forward with each received message
    pub receiving_chain_key: [u8; 32],

    /// Sending message number (counter)
    pub sending_counter: u64,

    /// Receiving message number (counter)
    pub receiving_counter: u64,
}

impl RatchetState {
    /// Create a new ratchet state from a root key (shared secret from X3DH)
    ///
    /// `is_initiator`: true for Alice (who initiated X3DH), false for Bob (responder)
    /// This ensures Alice's sending chain = Bob's receiving chain
    pub fn new(root_key: [u8; 32], is_initiator: bool) -> Result<Self> {
        // Derive initial sending and receiving chain keys from root key
        let (chain_key_0, chain_key_1) = Self::derive_chain_keys(&root_key)?;

        // Alice (initiator): sends with chain_0, receives with chain_1
        // Bob (responder): sends with chain_1, receives with chain_0
        let (sending_chain_key, receiving_chain_key) = if is_initiator {
            (chain_key_0, chain_key_1)
        } else {
            (chain_key_1, chain_key_0)
        };

        Ok(Self {
            root_key,
            sending_chain_key,
            receiving_chain_key,
            sending_counter: 0,
            receiving_counter: 0,
        })
    }

    /// Derive sending and receiving chain keys from root key using HKDF
    fn derive_chain_keys(root_key: &[u8; 32]) -> Result<([u8; 32], [u8; 32])> {
        let hkdf = Hkdf::<Sha256>::new(Some(b"mepassa-ratchet-v1"), root_key);

        let mut sending_chain_key = [0u8; 32];
        hkdf.expand(b"sending-chain", &mut sending_chain_key)
            .map_err(|e| MePassaError::Crypto(format!("HKDF expand failed: {}", e)))?;

        let mut receiving_chain_key = [0u8; 32];
        hkdf.expand(b"receiving-chain", &mut receiving_chain_key)
            .map_err(|e| MePassaError::Crypto(format!("HKDF expand failed: {}", e)))?;

        Ok((sending_chain_key, receiving_chain_key))
    }

    /// Derive a message key from a chain key
    fn derive_message_key(chain_key: &[u8; 32], counter: u64) -> Result<[u8; 32]> {
        let hkdf = Hkdf::<Sha256>::new(Some(b"mepassa-message-key-v1"), chain_key);

        let mut message_key = [0u8; 32];
        let info = format!("message-{}", counter);
        hkdf.expand(info.as_bytes(), &mut message_key)
            .map_err(|e| MePassaError::Crypto(format!("HKDF expand failed: {}", e)))?;

        Ok(message_key)
    }

    /// Ratchet forward the sending chain key
    fn ratchet_sending_chain(&mut self) -> Result<()> {
        let hkdf = Hkdf::<Sha256>::new(Some(b"mepassa-chain-ratchet-v1"), &self.sending_chain_key);

        let mut new_chain_key = [0u8; 32];
        hkdf.expand(b"next-chain", &mut new_chain_key)
            .map_err(|e| MePassaError::Crypto(format!("HKDF expand failed: {}", e)))?;

        self.sending_chain_key = new_chain_key;
        self.sending_counter += 1;

        Ok(())
    }

    /// Ratchet forward the receiving chain key
    fn ratchet_receiving_chain(&mut self) -> Result<()> {
        let hkdf = Hkdf::<Sha256>::new(Some(b"mepassa-chain-ratchet-v1"), &self.receiving_chain_key);

        let mut new_chain_key = [0u8; 32];
        hkdf.expand(b"next-chain", &mut new_chain_key)
            .map_err(|e| MePassaError::Crypto(format!("HKDF expand failed: {}", e)))?;

        self.receiving_chain_key = new_chain_key;
        self.receiving_counter += 1;

        Ok(())
    }

    /// Encrypt a message using the ratchet
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<EncryptedMessage> {
        // Derive message key from current sending chain key
        let message_key = Self::derive_message_key(&self.sending_chain_key, self.sending_counter)?;

        // Encrypt message
        let encrypted = encrypt_message(plaintext, &message_key)?;

        // Ratchet forward sending chain
        self.ratchet_sending_chain()?;

        Ok(encrypted)
    }

    /// Decrypt a message using the ratchet
    pub fn decrypt(&mut self, encrypted: &EncryptedMessage) -> Result<Vec<u8>> {
        // Derive message key from current receiving chain key
        let message_key = Self::derive_message_key(&self.receiving_chain_key, self.receiving_counter)?;

        // Decrypt message
        let plaintext = decrypt_message(encrypted, &message_key)?;

        // Ratchet forward receiving chain
        self.ratchet_receiving_chain()?;

        Ok(plaintext)
    }

    /// Get a snapshot of current counters for debugging
    pub fn counters(&self) -> (u64, u64) {
        (self.sending_counter, self.receiving_counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::X3DH;
    use crate::identity::Identity;

    #[test]
    fn test_ratchet_state_creation() {
        let root_key = [42u8; 32];
        let ratchet = RatchetState::new(root_key, true).unwrap();

        assert_eq!(ratchet.root_key, root_key);
        assert_eq!(ratchet.sending_counter, 0);
        assert_eq!(ratchet.receiving_counter, 0);
        assert_ne!(ratchet.sending_chain_key, root_key); // Should be derived
        assert_ne!(ratchet.receiving_chain_key, root_key); // Should be derived
        assert_ne!(ratchet.sending_chain_key, ratchet.receiving_chain_key); // Should be different
    }

    #[test]
    fn test_ratchet_encrypt_decrypt() {
        let root_key = [42u8; 32];
        let mut alice_ratchet = RatchetState::new(root_key, true).unwrap(); // Alice is initiator
        let mut bob_ratchet = RatchetState::new(root_key, false).unwrap(); // Bob is responder

        let plaintext = b"Hello with forward secrecy!";

        // Alice encrypts
        let encrypted = alice_ratchet.encrypt(plaintext).unwrap();
        assert_eq!(alice_ratchet.sending_counter, 1);

        // Bob decrypts
        let decrypted = bob_ratchet.decrypt(&encrypted).unwrap();
        assert_eq!(bob_ratchet.receiving_counter, 1);

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_ratchet_multiple_messages() {
        let root_key = [42u8; 32];
        let mut alice_ratchet = RatchetState::new(root_key, true).unwrap();
        let mut bob_ratchet = RatchetState::new(root_key, false).unwrap();

        // Send 10 messages
        for i in 0..10 {
            let msg = format!("Message {}", i);

            // Alice encrypts
            let encrypted = alice_ratchet.encrypt(msg.as_bytes()).unwrap();

            // Bob decrypts
            let decrypted = bob_ratchet.decrypt(&encrypted).unwrap();

            assert_eq!(msg.as_bytes(), decrypted.as_slice());
        }

        assert_eq!(alice_ratchet.sending_counter, 10);
        assert_eq!(bob_ratchet.receiving_counter, 10);
    }

    #[test]
    fn test_ratchet_forward_secrecy() {
        let root_key = [42u8; 32];
        let mut alice_ratchet = RatchetState::new(root_key, true).unwrap();
        let mut bob_ratchet = RatchetState::new(root_key, false).unwrap();

        // Send first message
        let msg1 = b"First message";
        let encrypted1 = alice_ratchet.encrypt(msg1).unwrap();
        let decrypted1 = bob_ratchet.decrypt(&encrypted1).unwrap();
        assert_eq!(msg1, decrypted1.as_slice());

        // Save chain keys before second message
        let alice_chain_key_before = alice_ratchet.sending_chain_key;
        let bob_chain_key_before = bob_ratchet.receiving_chain_key;

        // Send second message
        let msg2 = b"Second message";
        let encrypted2 = alice_ratchet.encrypt(msg2).unwrap();
        let decrypted2 = bob_ratchet.decrypt(&encrypted2).unwrap();
        assert_eq!(msg2, decrypted2.as_slice());

        // Chain keys should have changed (forward secrecy)
        assert_ne!(alice_ratchet.sending_chain_key, alice_chain_key_before);
        assert_ne!(bob_ratchet.receiving_chain_key, bob_chain_key_before);

        // Old keys can't decrypt new messages (forward secrecy test)
        let msg3 = b"Third message";
        let encrypted3 = alice_ratchet.encrypt(msg3).unwrap();

        // Try to decrypt with old chain key (should fail)
        let old_message_key = RatchetState::derive_message_key(&bob_chain_key_before, 2).unwrap();
        let result = decrypt_message(&encrypted3, &old_message_key);
        assert!(result.is_err()); // Should fail because key ratcheted forward
    }

    #[test]
    fn test_ratchet_different_root_keys() {
        let root_key1 = [42u8; 32];
        let root_key2 = [99u8; 32];

        let mut alice_ratchet = RatchetState::new(root_key1, true).unwrap();
        let mut bob_ratchet = RatchetState::new(root_key2, false).unwrap();

        let plaintext = b"This should fail";

        // Alice encrypts with root_key1
        let encrypted = alice_ratchet.encrypt(plaintext).unwrap();

        // Bob tries to decrypt with root_key2 (should fail)
        let result = bob_ratchet.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_e2e_with_x3dh_and_ratchet() {
        // Bob generates identity
        let bob = Identity::generate(0);
        let mut bob_mut = bob.clone();

        let bob_signed_prekey_secret = bob_mut
            .prekey_pool()
            .unwrap()
            .signed_prekey()
            .secret_bytes();

        let bob_bundle = bob_mut.prekey_pool_mut().unwrap().get_bundle();

        // Alice initiates X3DH
        let (alice_shared_secret, alice_ephemeral_pub) = X3DH::initiate(&bob_bundle).unwrap();

        // Bob responds to X3DH
        let bob_shared_secret =
            X3DH::respond(&bob_signed_prekey_secret, None, &alice_ephemeral_pub).unwrap();

        // Both should have same shared secret
        assert_eq!(alice_shared_secret, bob_shared_secret);

        // Create ratchets from shared secret
        let mut alice_ratchet = RatchetState::new(alice_shared_secret, true).unwrap(); // Alice is initiator
        let mut bob_ratchet = RatchetState::new(bob_shared_secret, false).unwrap(); // Bob is responder

        // Alice sends encrypted message to Bob using ratchet
        let alice_message = b"Secret with X3DH + Double Ratchet!";
        let encrypted = alice_ratchet.encrypt(alice_message).unwrap();

        // Bob decrypts using ratchet
        let decrypted = bob_ratchet.decrypt(&encrypted).unwrap();

        assert_eq!(alice_message, decrypted.as_slice());

        // Send multiple messages to verify ratchet works
        for i in 0..5 {
            let msg = format!("Message {}", i);
            let enc = alice_ratchet.encrypt(msg.as_bytes()).unwrap();
            let dec = bob_ratchet.decrypt(&enc).unwrap();
            assert_eq!(msg.as_bytes(), dec.as_slice());
        }

        assert_eq!(alice_ratchet.sending_counter, 6); // 1 + 5
        assert_eq!(bob_ratchet.receiving_counter, 6);
    }

    #[test]
    fn test_counters() {
        let root_key = [42u8; 32];
        let mut alice_ratchet = RatchetState::new(root_key, true).unwrap();
        let mut bob_ratchet = RatchetState::new(root_key, false).unwrap();

        assert_eq!(alice_ratchet.counters(), (0, 0));
        assert_eq!(bob_ratchet.counters(), (0, 0));

        // Alice encrypts and sends 5 messages
        let mut encrypted_messages = Vec::new();
        for _ in 0..5 {
            encrypted_messages.push(alice_ratchet.encrypt(b"test").unwrap());
        }

        assert_eq!(alice_ratchet.counters(), (5, 0));

        // Bob receives and decrypts first 3 messages
        for i in 0..3 {
            bob_ratchet.decrypt(&encrypted_messages[i]).unwrap();
        }

        assert_eq!(alice_ratchet.counters(), (5, 0)); // Alice sent 5
        assert_eq!(bob_ratchet.counters(), (0, 3)); // Bob received 3
    }
}
