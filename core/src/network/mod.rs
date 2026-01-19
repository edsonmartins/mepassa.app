//! Networking module
//!
//! Implements P2P networking using libp2p (Kademlia DHT, GossipSub, Relay).

// pub mod transport;
// pub mod behaviour;
// pub mod dht;
// pub mod gossip;
// pub mod relay;
// pub mod nat;

// pub use transport::MePassaTransport;
// pub use behaviour::MePassaBehaviour;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    #[error("NAT traversal failed")]
    NatTraversalFailed,

    #[error("Timeout")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, NetworkError>;
