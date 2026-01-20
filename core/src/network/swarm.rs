//! Swarm Management
//!
//! Manages the libp2p Swarm for P2P networking.

use libp2p::{
    identity::Keypair,
    swarm::{Config as SwarmConfig, Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::select;

use super::{behaviour::MePassaBehaviour, transport::build_transport};
use crate::utils::error::{MePassaError, Result};

/// P2P Network Manager
pub struct NetworkManager {
    swarm: Swarm<MePassaBehaviour>,
    local_peer_id: PeerId,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new(keypair: Keypair) -> Result<Self> {
        let local_peer_id = PeerId::from(keypair.public());

        // Build transport
        let transport = build_transport(&keypair)?;

        // Create behaviour
        let behaviour = MePassaBehaviour::new(local_peer_id, &keypair)?;

        // Create swarm
        let swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            SwarmConfig::with_tokio_executor()
                .with_idle_connection_timeout(Duration::from_secs(60)),
        );

        Ok(Self {
            swarm,
            local_peer_id,
        })
    }

    /// Get local peer ID
    pub fn local_peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }

    /// Start listening on a multiaddr
    pub fn listen_on(&mut self, addr: Multiaddr) -> Result<()> {
        self.swarm
            .listen_on(addr)
            .map_err(|e| MePassaError::Network(format!("Failed to listen: {}", e)))?;

        Ok(())
    }

    /// Dial a peer
    pub fn dial(&mut self, peer_id: PeerId, addr: Multiaddr) -> Result<()> {
        self.swarm
            .dial(addr)
            .map_err(|e| MePassaError::Network(format!("Failed to dial {}: {}", peer_id, e)))?;

        Ok(())
    }

    /// Add a peer to the DHT
    pub fn add_peer_to_dht(&mut self, peer_id: PeerId, addr: Multiaddr) {
        self.swarm
            .behaviour_mut()
            .kademlia
            .add_address(&peer_id, addr);
    }

    /// Bootstrap the DHT
    pub fn bootstrap(&mut self) -> Result<()> {
        self.swarm
            .behaviour_mut()
            .kademlia
            .bootstrap()
            .map_err(|e| MePassaError::Network(format!("Failed to bootstrap DHT: {}", e)))?;

        Ok(())
    }

    /// Get connected peers count
    pub fn connected_peers(&self) -> usize {
        self.swarm.connected_peers().count()
    }

    /// Send a message to a peer
    pub fn send_message(&mut self, peer_id: PeerId, message: crate::protocol::Message) -> Result<()> {
        let request_id = self
            .swarm
            .behaviour_mut()
            .request_response
            .send_request(&peer_id, message);

        tracing::info!("Sent message to {} (request_id: {:?})", peer_id, request_id);
        Ok(())
    }

    /// Send an ACK response to a peer
    pub fn send_ack(
        &mut self,
        channel: libp2p::request_response::ResponseChannel<crate::protocol::Message>,
        ack_message: crate::protocol::Message,
    ) -> Result<()> {
        self.swarm
            .behaviour_mut()
            .request_response
            .send_response(channel, ack_message)
            .map_err(|e| MePassaError::Network(format!("Failed to send ACK: {:?}", e)))?;

        Ok(())
    }

    /// Run the event loop (blocking)
    pub async fn run(&mut self) -> Result<()> {
        loop {
            select! {
                event = self.swarm.select_next_some() => {
                    self.handle_event(event).await?;
                }
            }
        }
    }

    /// Handle swarm events
    async fn handle_event(&mut self, event: SwarmEvent<MePassaBehaviourEvent>) -> Result<()> {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                tracing::info!("Listening on {}", address);
            }
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                tracing::info!("Connected to {} at {}", peer_id, endpoint.get_remote_address());
            }
            SwarmEvent::ConnectionClosed {
                peer_id, cause, ..
            } => {
                tracing::info!("Disconnected from {}: {:?}", peer_id, cause);
            }
            SwarmEvent::Behaviour(event) => {
                self.handle_behaviour_event(event).await?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle behaviour-specific events
    async fn handle_behaviour_event(&mut self, event: MePassaBehaviourEvent) -> Result<()> {
        match event {
            MePassaBehaviourEvent::Kademlia(kad_event) => {
                tracing::debug!("Kademlia event: {:?}", kad_event);
            }
            MePassaBehaviourEvent::Mdns(mdns_event) => {
                match mdns_event {
                    libp2p::mdns::Event::Discovered(peers) => {
                        for (peer_id, addr) in peers {
                            tracing::info!("mDNS discovered peer: {} at {}", peer_id, addr);
                            self.add_peer_to_dht(peer_id, addr);
                        }
                    }
                    libp2p::mdns::Event::Expired(peers) => {
                        for (peer_id, _) in peers {
                            tracing::info!("mDNS peer expired: {}", peer_id);
                        }
                    }
                }
            }
            MePassaBehaviourEvent::Identify(identify_event) => {
                tracing::debug!("Identify event: {:?}", identify_event);
            }
            MePassaBehaviourEvent::Ping(ping_event) => {
                tracing::trace!("Ping event: {:?}", ping_event);
            }
            MePassaBehaviourEvent::Gossipsub(gossipsub_event) => {
                tracing::debug!("GossipSub event: {:?}", gossipsub_event);
            }
            MePassaBehaviourEvent::RequestResponse(rr_event) => {
                match rr_event {
                    libp2p::request_response::Event::Message { peer, message } => {
                        match message {
                            libp2p::request_response::Message::Request {
                                request_id,
                                request,
                                channel: _,
                            } => {
                                tracing::info!(
                                    "Received message from {}: {:?} (request_id: {:?})",
                                    peer,
                                    request.id,
                                    request_id
                                );
                                // TODO: Process message and send response (FASE 4)
                                // For now, just log the request
                            }
                            libp2p::request_response::Message::Response {
                                request_id,
                                response,
                            } => {
                                tracing::info!(
                                    "Received response from {}: {:?} (request_id: {:?})",
                                    peer,
                                    response.id,
                                    request_id
                                );
                                // TODO: Process acknowledgment (FASE 4)
                            }
                        }
                    }
                    libp2p::request_response::Event::OutboundFailure {
                        peer,
                        request_id,
                        error,
                    } => {
                        tracing::warn!(
                            "Outbound request failed to {}: {:?} (request_id: {:?})",
                            peer,
                            error,
                            request_id
                        );
                    }
                    libp2p::request_response::Event::InboundFailure {
                        peer,
                        request_id,
                        error,
                    } => {
                        tracing::warn!(
                            "Inbound request failed from {}: {:?} (request_id: {:?})",
                            peer,
                            error,
                            request_id
                        );
                    }
                    libp2p::request_response::Event::ResponseSent { peer, request_id } => {
                        tracing::debug!("Response sent to {} (request_id: {:?})", peer, request_id);
                    }
                }
            }
        }

        Ok(())
    }
}

// The MePassaBehaviourEvent type is auto-generated by NetworkBehaviour derive macro
use super::behaviour::MePassaBehaviourEvent;

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity;

    #[tokio::test]
    async fn test_create_network_manager() {
        let keypair = identity::Keypair::generate_ed25519();
        let manager = NetworkManager::new(keypair);

        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_local_peer_id() {
        let keypair = identity::Keypair::generate_ed25519();
        let expected_peer_id = PeerId::from(keypair.public());

        let manager = NetworkManager::new(keypair).unwrap();

        assert_eq!(*manager.local_peer_id(), expected_peer_id);
    }

    #[tokio::test]
    async fn test_listen_on() {
        let keypair = identity::Keypair::generate_ed25519();
        let mut manager = NetworkManager::new(keypair).unwrap();

        let addr: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
        let result = manager.listen_on(addr);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_connected_peers_initially_zero() {
        let keypair = identity::Keypair::generate_ed25519();
        let manager = NetworkManager::new(keypair).unwrap();

        assert_eq!(manager.connected_peers(), 0);
    }
}
