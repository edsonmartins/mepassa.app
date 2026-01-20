//! Networking module
//!
//! Implements P2P networking using libp2p (Kademlia DHT, GossipSub, Relay).

pub mod behaviour;
pub mod messaging;
pub mod swarm;
pub mod transport;
// pub mod dht;
// pub mod gossip;
// pub mod relay;
// pub mod nat;

pub use behaviour::MePassaBehaviour;
pub use messaging::MePassaCodec;
pub use swarm::NetworkManager;

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

impl From<NetworkError> for crate::utils::error::MePassaError {
    fn from(err: NetworkError) -> Self {
        crate::utils::error::MePassaError::Network(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, NetworkError>;
