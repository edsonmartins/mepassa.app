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

use super::{
    behaviour::MePassaBehaviour,
    connection::{ConnectionManager, ConnectionType},
    relay::RelayManager,
    retry::RetryPolicy,
    transport::build_transport,
};
use crate::utils::error::{MePassaError, Result};

/// P2P Network Manager
pub struct NetworkManager {
    swarm: Swarm<MePassaBehaviour>,
    local_peer_id: PeerId,
    connection_manager: ConnectionManager,
    relay_manager: RelayManager,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new(keypair: Keypair) -> Result<Self> {
        Self::with_relay(keypair, None, None)
    }

    /// Create a new network manager with optional relay configuration
    pub fn with_relay(
        keypair: Keypair,
        bootstrap_relay_peer: Option<PeerId>,
        relay_addr: Option<Multiaddr>,
    ) -> Result<Self> {
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

        // Create connection manager with default retry policy
        let connection_manager = ConnectionManager::new(RetryPolicy::default());

        // Create relay manager
        let relay_manager = RelayManager::new(bootstrap_relay_peer, relay_addr);

        Ok(Self {
            swarm,
            local_peer_id,
            connection_manager,
            relay_manager,
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

    /// Dial a peer with automatic relay fallback
    pub fn dial(&mut self, peer_id: PeerId, addr: Multiaddr) -> Result<()> {
        // Check if we should try relay based on connection history
        if self.connection_manager.should_try_relay(&peer_id) {
            tracing::info!("🔄 Attempting relay connection to {}", peer_id);
            return self.dial_via_relay(peer_id);
        }

        // Try direct connection first
        tracing::debug!("📞 Attempting direct connection to {} at {}", peer_id, addr);
        match self.swarm.dial(addr.clone()) {
            Ok(_) => {
                // Connection initiated, will track result in events
                Ok(())
            }
            Err(e) => {
                tracing::warn!("⚠️ Direct dial failed to {}: {}", peer_id, e);
                self.connection_manager.record_failure(peer_id);

                // Try relay if available and we should fallback
                if self.connection_manager.should_try_relay(&peer_id) {
                    tracing::info!("🔄 Falling back to relay for {}", peer_id);
                    self.dial_via_relay(peer_id)
                } else {
                    Err(MePassaError::Network(format!("Failed to dial {}: {}", peer_id, e)))
                }
            }
        }
    }

    /// Dial a peer via relay
    fn dial_via_relay(&mut self, peer_id: PeerId) -> Result<()> {
        if let Some(circuit_addr) = self.relay_manager.circuit_addr(&peer_id) {
            tracing::info!("🌉 Dialing {} via relay circuit", peer_id);
            self.swarm
                .dial(circuit_addr)
                .map_err(|e| MePassaError::Network(format!("Failed to dial via relay: {}", e)))?;
            Ok(())
        } else {
            Err(MePassaError::Network(
                "No relay configuration available".to_string(),
            ))
        }
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

    /// Get connection state for a peer
    pub fn connection_state(
        &self,
        peer_id: &PeerId,
    ) -> super::connection::ConnectionState {
        self.connection_manager.get_state(peer_id)
    }

    /// Check if relay is available
    pub fn has_relay(&self) -> bool {
        self.relay_manager.has_reservation()
    }

    /// Attempt to reserve relay slot
    pub fn reserve_relay_slot(&mut self) -> Result<()> {
        if let Some(relay_peer) = self.relay_manager.bootstrap_relay_peer {
            if let Some(relay_addr) = &self.relay_manager.relay_addr {
                tracing::info!("🔗 Requesting relay reservation from {}", relay_peer);

                // Connect to relay first if not connected
                self.add_peer_to_dht(relay_peer, relay_addr.clone());

                // Mark reservation as pending
                // Note: In libp2p 0.53, relay reservation is handled at transport level
                // We're marking it here for state tracking
                // TODO: Integrate with actual relay transport API
                tracing::warn!("⚠️ Relay reservation requires transport-level integration (libp2p 0.53)");

                Ok(())
            } else {
                Err(MePassaError::Network("No relay address configured".to_string()))
            }
        } else {
            Err(MePassaError::Network("No relay peer configured".to_string()))
        }
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

    /// Send a VoIP signaling message to a peer
    pub fn send_voip_signal(
        &mut self,
        peer_id: PeerId,
        signal: crate::voip::signaling::SignalingMessage,
    ) -> Result<()> {
        let request_id = self
            .swarm
            .behaviour_mut()
            .voip_signaling
            .send_request(&peer_id, signal.clone());

        tracing::info!("📞 Sent VoIP signal to {} (request_id: {:?}): {:?}", peer_id, request_id, signal);
        Ok(())
    }

    /// Send a VoIP signaling response to a peer
    pub fn send_voip_response(
        &mut self,
        channel: libp2p::request_response::ResponseChannel<crate::voip::signaling::SignalingMessage>,
        response: crate::voip::signaling::SignalingMessage,
    ) -> Result<()> {
        self.swarm
            .behaviour_mut()
            .voip_signaling
            .send_response(channel, response)
            .map_err(|e| MePassaError::Network(format!("Failed to send VoIP response: {:?}", e)))?;

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
                let addr = endpoint.get_remote_address();
                tracing::info!("✅ Connected to {} at {}", peer_id, addr);

                // Determine connection type and record success
                let connection_type = if addr.to_string().contains("p2p-circuit") {
                    ConnectionType::Relayed
                } else {
                    // TODO: Detect if connection was upgraded via DCUtR
                    ConnectionType::Direct
                };

                self.connection_manager
                    .record_success(peer_id, connection_type);
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
            MePassaBehaviourEvent::VoipSignaling(voip_event) => {
                match voip_event {
                    libp2p::request_response::Event::Message { peer, message } => {
                        match message {
                            libp2p::request_response::Message::Request {
                                request_id,
                                request,
                                channel,
                            } => {
                                tracing::info!(
                                    "📞 Received VoIP signal from {}: {:?} (request_id: {:?})",
                                    peer,
                                    request,
                                    request_id
                                );
                                // TODO: Forward to CallManager via event channel (FASE 12)
                                // For now, send automatic ACK
                                let ack = crate::voip::signaling::SignalingMessage::CallAccept {
                                    call_id: "temp".to_string(),
                                };
                                let _ = self.send_voip_response(channel, ack);
                            }
                            libp2p::request_response::Message::Response {
                                request_id,
                                response,
                            } => {
                                tracing::info!(
                                    "📞 Received VoIP response from {}: {:?} (request_id: {:?})",
                                    peer,
                                    response,
                                    request_id
                                );
                                // TODO: Forward to CallManager (FASE 12)
                            }
                        }
                    }
                    libp2p::request_response::Event::OutboundFailure {
                        peer,
                        request_id,
                        error,
                    } => {
                        tracing::warn!(
                            "📞 VoIP signal outbound failed to {}: {:?} (request_id: {:?})",
                            peer,
                            error,
                            request_id
                        );
                        // TODO: Notify CallManager of failure (FASE 12)
                    }
                    libp2p::request_response::Event::InboundFailure {
                        peer,
                        request_id,
                        error,
                    } => {
                        tracing::warn!(
                            "📞 VoIP signal inbound failed from {}: {:?} (request_id: {:?})",
                            peer,
                            error,
                            request_id
                        );
                    }
                    libp2p::request_response::Event::ResponseSent { peer, request_id } => {
                        tracing::debug!("📞 VoIP response sent to {} (request_id: {:?})", peer, request_id);
                    }
                }
            }
            MePassaBehaviourEvent::Dcutr(dcutr_event) => {
                // DCUtR hole punching events
                // Note: Event structure varies in libp2p 0.53, using debug for now
                // TODO: Pattern match specific events when API is stable:
                //   - RemoteInitiatedDirectConnectionUpgrade
                //   - DirectConnectionUpgradeSucceeded
                //   - DirectConnectionUpgradeFailed
                tracing::debug!("🎯 DCUtR event: {:?}", dcutr_event);

                // If this is a successful upgrade event, we'd update connection type to HolePunch
                // self.connection_manager.record_success(peer_id, ConnectionType::HolePunch);
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
