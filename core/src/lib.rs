//! MePassa Core Library
//!
//! Core library for MePassa P2P chat platform.
//!
//! ## Architecture: HYBRID P2P + Server
//!
//! - **80% P2P direct:** Messages go peer-to-peer (max privacy, zero cost)
//! - **15% TURN relay:** Fallback when symmetric NAT/firewall
//! - **5% Store & Forward:** Recipient offline (PostgreSQL, 14d TTL)
//!
//! ## Modules
//!
//! - `identity`: Ed25519 keypairs and peer identity management
//! - `crypto`: Signal Protocol E2E encryption
//! - `network`: libp2p P2P networking
//! - `storage`: SQLite local storage
//! - `sync`: CRDTs for multi-device sync
//! - `voip`: WebRTC voice/video calls
//! - `protocol`: Protobuf message definitions
//! - `api`: Public Client API
//! - `ffi`: UniFFI bindings for Kotlin/Swift

// Re-export public API
// TODO: Uncomment when modules are implemented
// pub use api::{Client, ClientBuilder, Event};
pub use identity::{Identity, Keypair, PublicKey};
// pub use protocol::{Message, MessageContent, MessageType};

// Re-export FFI types (required by UniFFI scaffolding)
pub use ffi::{
    FfiConversation, FfiMessage, MePassaClient, MePassaFfiError, MessageStatus,
};

// Public modules
pub mod api;
pub mod crypto;
pub mod ffi;
pub mod group;
pub mod identity;
pub mod identity_client;
pub mod network;
pub mod protocol;
pub mod storage;
pub mod sync;
#[cfg(feature = "voip")]
pub mod voip;
pub mod utils;

// Include UniFFI scaffolding (must be at crate root)
uniffi::include_scaffolding!("mepassa");

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging system
pub fn init_logging(level: utils::LogLevel) {
    utils::logging::init(level);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(VERSION, "0.1.0");
    }
}
