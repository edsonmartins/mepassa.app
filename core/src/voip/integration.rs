//! VoIP Network Integration
//!
//! Coordinates VoIP signaling between NetworkManager and CallManager.
//! Bridges libp2p network layer with WebRTC call management.

use libp2p::PeerId;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use super::{
    call::{CallDirection, CallEndReason},
    manager::{CallEvent, CallManager},
    signaling::SignalingMessage,
    Result, // Use voip::Result instead of utils::error::Result
};
use crate::network::swarm::NetworkManager;
use crate::utils::error::MePassaError;

/// VoIP network integration coordinator
///
/// Manages the flow of WebRTC signaling messages between:
/// - NetworkManager (libp2p P2P layer)
/// - CallManager (WebRTC call management)
pub struct VoIPIntegration {
    network_manager: Arc<RwLock<NetworkManager>>,
    call_manager: Arc<CallManager>,

    // Event channels
    signaling_rx: mpsc::UnboundedReceiver<(PeerId, SignalingMessage)>,
    signaling_tx: mpsc::UnboundedSender<(PeerId, SignalingMessage)>,
}

impl VoIPIntegration {
    /// Create a new VoIP integration coordinator
    pub fn new(
        network_manager: Arc<RwLock<NetworkManager>>,
        call_manager: Arc<CallManager>,
    ) -> Self {
        let (signaling_tx, signaling_rx) = mpsc::unbounded_channel();

        Self {
            network_manager,
            call_manager,
            signaling_rx,
            signaling_tx,
        }
    }

    /// Get a sender for signaling messages (for NetworkManager to use)
    pub fn signaling_sender(&self) -> mpsc::UnboundedSender<(PeerId, SignalingMessage)> {
        self.signaling_tx.clone()
    }

    /// Run the integration event loop
    ///
    /// Processes:
    /// - Incoming signaling messages from network
    /// - Outgoing signaling messages from CallManager (TODO)
    /// - Call state changes and events (TODO)
    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("🔗 VoIP integration started");

        loop {
            // Handle incoming signaling from network
            if let Some((peer_id, signal)) = self.signaling_rx.recv().await {
                if let Err(e) = self.handle_incoming_signal(peer_id, signal).await {
                    tracing::error!("❌ Failed to handle incoming signal: {}", e);
                }
            } else {
                tracing::warn!("⚠️ Signaling channel closed");
                break;
            }

            // TODO: Add call event handling when CallManager exposes event_rx
            // For now, we only handle incoming network signals
        }

        Ok(())
    }

    /// Handle incoming signaling message from network
    async fn handle_incoming_signal(
        &self,
        peer_id: PeerId,
        signal: SignalingMessage,
    ) -> Result<()> {
        let peer_id_str = peer_id.to_string();

        tracing::info!("📞 Handling signal from {}: {:?}", peer_id_str, signal);

        match &signal {
            SignalingMessage::CallOffer { call_id, sdp } => {
                // Incoming call offer
                tracing::info!("📲 Incoming call offer from {} (call: {})", peer_id_str, call_id);

                // Create incoming call (call_id, remote_peer_id, offer_sdp)
                self.call_manager
                    .handle_incoming_call(call_id.clone(), peer_id_str.clone(), sdp.clone())
                    .await?;

                tracing::info!("✅ Incoming call created: {}", call_id);
            }

            SignalingMessage::CallAnswer { call_id, sdp } => {
                // Remote peer answered our call
                tracing::info!("✅ Call answered by {} (call: {})", peer_id_str, call_id);

                self.call_manager
                    .handle_call_answer(call_id.clone(), sdp.clone())
                    .await?;
            }

            SignalingMessage::IceCandidate {
                call_id,
                candidate,
                sdp_mid,
                sdp_m_line_index,
            } => {
                // ICE candidate from remote peer
                tracing::debug!(
                    "🧊 ICE candidate from {} for call {}: {}",
                    peer_id_str,
                    call_id,
                    candidate
                );

                self.call_manager
                    .handle_ice_candidate(
                        call_id.clone(),
                        candidate.clone(),
                        sdp_mid.clone(),
                        *sdp_m_line_index,
                    )
                    .await?;
            }

            SignalingMessage::CallReject { call_id, reason } => {
                // Remote peer rejected our call
                tracing::warn!(
                    "❌ Call rejected by {} (call: {}): {:?}",
                    peer_id_str,
                    call_id,
                    reason
                );

                self.call_manager
                    .end_call(call_id.clone(), CallEndReason::Rejected)
                    .await?;
            }

            SignalingMessage::CallHangup { call_id } => {
                // Remote peer hung up
                tracing::info!("📴 Call hung up by {} (call: {})", peer_id_str, call_id);

                self.call_manager
                    .end_call(call_id.clone(), CallEndReason::RemoteHangup)
                    .await?;
            }

            SignalingMessage::CallAccept { call_id } => {
                // Remote peer accepted call (acknowledgment)
                tracing::info!("✅ Call accepted by {} (call: {})", peer_id_str, call_id);
            }
        }

        Ok(())
    }

    // TODO: Add call event handling when CallManager exposes event_rx
    // This would process events like:
    // - IncomingCall: Already handled via network signals
    // - StateChanged: Log state transitions
    // - Ended: Send hangup signal to remote peer
    // - AudioReceived: Forward to audio playback

    /// Send signaling message via network
    pub async fn send_signal(&self, peer_id: PeerId, signal: SignalingMessage) -> Result<()> {
        let mut network = self.network_manager.write().await;
        network
            .send_voip_signal(peer_id, signal)
            .map_err(|e| super::VoipError::NetworkError(e.to_string()))
    }

    /// Initiate a call to a peer
    pub async fn start_call(&self, to_peer_id: String) -> Result<String> {
        tracing::info!("📞 Starting call to {}", to_peer_id);

        // Start call via CallManager
        let call_id = self.call_manager.start_call(to_peer_id.clone()).await?;

        // CallManager will generate offer and emit it as event
        // Integration will send it via network when ready
        // TODO: Listen for SignalingMessage from CallManager and send via network

        Ok(call_id)
    }

    /// Accept an incoming call
    pub async fn accept_call(&self, call_id: String) -> Result<()> {
        tracing::info!("✅ Accepting call {}", call_id);

        self.call_manager.accept_call(call_id).await?;

        // CallManager will generate answer and we'll send it via network
        // TODO: Listen for answer signal and send via network

        Ok(())
    }

    /// Reject an incoming call
    pub async fn reject_call(&self, call_id: String, reason: Option<String>) -> Result<()> {
        tracing::info!("❌ Rejecting call {}: {:?}", call_id, reason);

        self.call_manager
            .end_call(call_id.clone(), CallEndReason::Rejected)
            .await?;

        // Send rejection signal to remote peer
        // TODO: Get peer_id for call_id and send CallReject
        tracing::warn!("⚠️ TODO: Send CallReject signal to remote peer");

        Ok(())
    }

    /// Hangup an active call
    pub async fn hangup_call(&self, call_id: String) -> Result<()> {
        tracing::info!("📴 Hanging up call {}", call_id);

        self.call_manager
            .end_call(call_id.clone(), CallEndReason::LocalHangup)
            .await?;

        // Send hangup signal to remote peer
        // TODO: Get peer_id for call_id and send CallHangup
        tracing::warn!("⚠️ TODO: Send CallHangup signal to remote peer");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires full NetworkManager + CallManager setup
    async fn test_integration_creation() {
        // This test would require proper initialization
        // of both NetworkManager and CallManager
        // which is complex in unit test environment
    }
}
