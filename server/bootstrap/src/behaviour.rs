use libp2p::{
    identify, kad, ping,
    swarm::NetworkBehaviour,
    PeerId,
};
use std::time::Duration;

/// Custom NetworkBehaviour for the Bootstrap Node
///
/// Combines Kademlia DHT for peer discovery with Identify and Ping protocols.
#[derive(NetworkBehaviour)]
pub struct BootstrapBehaviour {
    /// Kademlia DHT for peer discovery
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,

    /// Identify protocol for peer information exchange
    pub identify: identify::Behaviour,

    /// Ping protocol for keep-alive
    pub ping: ping::Behaviour,
}

impl BootstrapBehaviour {
    /// Create a new BootstrapBehaviour instance
    pub fn new(local_peer_id: PeerId, local_public_key: libp2p::identity::PublicKey) -> Self {
        // Kademlia DHT configuration
        let mut kad_config = kad::Config::default();
        kad_config.set_query_timeout(Duration::from_secs(60));
        kad_config.set_replication_factor(20.try_into().unwrap());

        let store = kad::store::MemoryStore::new(local_peer_id);
        let kademlia = kad::Behaviour::with_config(local_peer_id, store, kad_config);

        // Identify configuration
        let identify = identify::Behaviour::new(
            identify::Config::new(
                "/mepassa/1.0.0".to_string(),
                local_public_key,
            )
        );

        // Ping (keep-alive)
        let ping = ping::Behaviour::new(ping::Config::new());

        Self {
            kademlia,
            identify,
            ping,
        }
    }
}
