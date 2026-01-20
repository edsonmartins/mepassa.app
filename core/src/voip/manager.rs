//! Call Manager
//!
//! Orchestrates WebRTC calls, signaling, and audio I/O.

use super::{
    call::{Call, CallDirection, CallEndReason, CallState},
    signaling::SignalingMessage,
    webrtc::{build_turn_config, WebRTCPeer},
    Result, VoipError,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use webrtc::ice_transport::ice_server::RTCIceServer;

/// TURN credentials from server
#[derive(Debug, Clone)]
pub struct TurnCredentials {
    pub username: String,
    pub password: String,
    pub uris: Vec<String>,
}

/// Call manager events
#[derive(Debug, Clone)]
pub enum CallEvent {
    /// Incoming call received
    IncomingCall {
        call_id: String,
        from_peer_id: String,
    },

    /// Call state changed
    StateChanged {
        call_id: String,
        new_state: CallState,
    },

    /// Call ended
    Ended {
        call_id: String,
        reason: CallEndReason,
    },

    /// Remote audio received (ready to play)
    AudioReceived {
        call_id: String,
        data: Vec<u8>,
    },
}

/// Manages all active calls
pub struct CallManager {
    /// Active calls by call_id
    calls: Arc<RwLock<HashMap<String, CallState>>>,

    /// WebRTC peers by call_id
    peers: Arc<RwLock<HashMap<String, Arc<WebRTCPeer>>>>,

    /// Event sender
    event_tx: mpsc::UnboundedSender<CallEvent>,

    /// Event receiver
    event_rx: Arc<RwLock<mpsc::UnboundedReceiver<CallEvent>>>,

    /// TURN credentials (cached)
    turn_credentials: Arc<RwLock<Option<TurnCredentials>>>,
}

impl CallManager {
    /// Create a new call manager
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            calls: Arc::new(RwLock::new(HashMap::new())),
            peers: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            event_rx: Arc::new(RwLock::new(event_rx)),
            turn_credentials: Arc::new(RwLock::new(None)),
        }
    }

    /// Set TURN credentials (fetched from server)
    pub async fn set_turn_credentials(&self, credentials: TurnCredentials) {
        let mut turn = self.turn_credentials.write().await;
        *turn = Some(credentials);
        tracing::info!("✅ TURN credentials configured");
    }

    /// Get ICE servers configuration
    async fn get_ice_servers(&self) -> Vec<RTCIceServer> {
        let turn = self.turn_credentials.read().await;

        if let Some(creds) = turn.as_ref() {
            build_turn_config(
                creds.uris.clone(),
                creds.username.clone(),
                creds.password.clone(),
            )
        } else {
            // Fallback to public STUN only
            vec![RTCIceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                ..Default::default()
            }]
        }
    }

    /// Start an outgoing call
    pub async fn start_call(&self, remote_peer_id: String) -> Result<String> {
        let call = Call::new_outgoing(remote_peer_id.clone());
        let call_id = call.id.clone();

        // Create WebRTC peer connection
        let ice_servers = self.get_ice_servers().await;
        let mut peer = WebRTCPeer::new(ice_servers).await?;

        // Add audio track
        peer.add_audio_track().await?;

        // Create offer
        let offer_sdp = peer.create_offer().await?;

        // Store call and peer
        {
            let mut calls = self.calls.write().await;
            calls.insert(call_id.clone(), call.state.clone());
        }
        {
            let mut peers = self.peers.write().await;
            peers.insert(call_id.clone(), Arc::new(peer));
        }

        // Emit event
        let _ = self.event_tx.send(CallEvent::StateChanged {
            call_id: call_id.clone(),
            new_state: CallState::Initiating,
        });

        tracing::info!("📞 Starting outgoing call to {}", remote_peer_id);

        // TODO: Send offer via signaling
        // This should be done by the network layer
        // For now, we just return the call_id and SDP needs to be sent separately

        Ok(call_id)
    }

    /// Handle incoming call (from signaling)
    pub async fn handle_incoming_call(
        &self,
        call_id: String,
        remote_peer_id: String,
        offer_sdp: String,
    ) -> Result<()> {
        let call = Call::new_incoming(call_id.clone(), remote_peer_id.clone());

        // Create WebRTC peer connection
        let ice_servers = self.get_ice_servers().await;
        let peer = WebRTCPeer::new(ice_servers).await?;

        // Set remote offer
        peer.set_remote_description(offer_sdp, "offer").await?;

        // Store call and peer
        {
            let mut calls = self.calls.write().await;
            calls.insert(call_id.clone(), call.state.clone());
        }
        {
            let mut peers = self.peers.write().await;
            peers.insert(call_id.clone(), Arc::new(peer));
        }

        // Emit incoming call event
        let _ = self.event_tx.send(CallEvent::IncomingCall {
            call_id: call_id.clone(),
            from_peer_id: remote_peer_id,
        });

        tracing::info!("📲 Incoming call: {}", call_id);

        Ok(())
    }

    /// Accept an incoming call
    pub async fn accept_call(&self, call_id: String) -> Result<String> {
        let peers = self.peers.read().await;
        let peer = peers
            .get(&call_id)
            .ok_or_else(|| VoipError::InvalidState("Call not found".to_string()))?;

        // Add audio track if not already added
        // Note: This is a bit tricky because we need mutable access
        // In real implementation, we'd need to handle this better

        // Create answer
        let answer_sdp = peer.create_answer().await?;

        // Update call state
        {
            let mut calls = self.calls.write().await;
            if let Some(state) = calls.get_mut(&call_id) {
                *state = CallState::Connecting;
            }
        }

        let _ = self.event_tx.send(CallEvent::StateChanged {
            call_id: call_id.clone(),
            new_state: CallState::Connecting,
        });

        tracing::info!("✅ Accepting call: {}", call_id);

        Ok(answer_sdp)
    }

    /// Handle incoming answer (for outgoing call)
    pub async fn handle_answer(&self, call_id: String, answer_sdp: String) -> Result<()> {
        let peers = self.peers.read().await;
        let peer = peers
            .get(&call_id)
            .ok_or_else(|| VoipError::InvalidState("Call not found".to_string()))?;

        peer.set_remote_description(answer_sdp, "answer").await?;

        // Update state to connecting
        {
            let mut calls = self.calls.write().await;
            if let Some(state) = calls.get_mut(&call_id) {
                *state = CallState::Connecting;
            }
        }

        let _ = self.event_tx.send(CallEvent::StateChanged {
            call_id: call_id.clone(),
            new_state: CallState::Connecting,
        });

        tracing::info!("🔗 Call connecting: {}", call_id);

        Ok(())
    }

    /// Add ICE candidate
    pub async fn add_ice_candidate(
        &self,
        call_id: String,
        candidate: String,
    ) -> Result<()> {
        let peers = self.peers.read().await;
        let peer = peers
            .get(&call_id)
            .ok_or_else(|| VoipError::InvalidState("Call not found".to_string()))?;

        peer.add_ice_candidate(candidate).await?;

        tracing::debug!("🧊 ICE candidate added for call: {}", call_id);

        Ok(())
    }

    /// Reject an incoming call
    pub async fn reject_call(&self, call_id: String) -> Result<()> {
        // Close peer connection
        {
            let mut peers = self.peers.write().await;
            if let Some(peer) = peers.remove(&call_id) {
                let _ = peer.close().await;
            }
        }

        // Remove call
        {
            let mut calls = self.calls.write().await;
            calls.remove(&call_id);
        }

        let _ = self.event_tx.send(CallEvent::Ended {
            call_id: call_id.clone(),
            reason: CallEndReason::Rejected,
        });

        tracing::info!("❌ Call rejected: {}", call_id);

        Ok(())
    }

    /// Hang up an active call
    pub async fn hangup_call(&self, call_id: String) -> Result<()> {
        // Close peer connection
        {
            let mut peers = self.peers.write().await;
            if let Some(peer) = peers.remove(&call_id) {
                let _ = peer.close().await;
            }
        }

        // Update call state
        {
            let mut calls = self.calls.write().await;
            calls.remove(&call_id);
        }

        let _ = self.event_tx.send(CallEvent::Ended {
            call_id: call_id.clone(),
            reason: CallEndReason::Hangup,
        });

        tracing::info!("📴 Call ended: {}", call_id);

        Ok(())
    }

    /// Get current active calls
    pub async fn get_active_calls(&self) -> Vec<String> {
        let calls = self.calls.read().await;
        calls.keys().cloned().collect()
    }

    /// Get call state
    pub async fn get_call_state(&self, call_id: &str) -> Option<CallState> {
        let calls = self.calls.read().await;
        calls.get(call_id).cloned()
    }

    /// Subscribe to call events
    pub async fn subscribe_events(&self) -> mpsc::UnboundedReceiver<CallEvent> {
        // Create a new channel for this subscriber
        // Note: This is a simplified implementation
        // In production, we'd use a broadcast channel
        let (tx, rx) = mpsc::unbounded_channel();

        // TODO: Implement proper event broadcasting
        // For now, this is a placeholder

        rx
    }

    // === Compatibility wrappers for VoIPIntegration ===

    /// Handle call answer (alias for handle_answer)
    pub async fn handle_call_answer(&self, call_id: String, answer_sdp: String) -> Result<()> {
        self.handle_answer(call_id, answer_sdp).await
    }

    /// Handle ICE candidate (alias for add_ice_candidate)
    pub async fn handle_ice_candidate(
        &self,
        call_id: String,
        candidate: String,
        _sdp_mid: Option<String>,
        _sdp_m_line_index: Option<u16>,
    ) -> Result<()> {
        // Current implementation doesn't use sdp_mid/sdp_m_line_index
        // TODO: Pass these to WebRTCPeer when implementing full ICE support
        self.add_ice_candidate(call_id, candidate).await
    }

    /// End a call with specific reason
    pub async fn end_call(&self, call_id: String, reason: CallEndReason) -> Result<()> {
        // Close peer connection
        {
            let mut peers = self.peers.write().await;
            if let Some(peer) = peers.remove(&call_id) {
                let _ = peer.close().await;
            }
        }

        // Remove call
        {
            let mut calls = self.calls.write().await;
            calls.remove(&call_id);
        }

        // Emit event with specific reason
        let _ = self.event_tx.send(CallEvent::Ended {
            call_id: call_id.clone(),
            reason: reason.clone(),
        });

        tracing::info!("📴 Call {} ended: {:?}", call_id, reason);

        Ok(())
    }
}

impl Default for CallManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_manager_creation() {
        let manager = CallManager::new();
        let active = manager.get_active_calls().await;
        assert_eq!(active.len(), 0);
    }

    #[tokio::test]
    async fn test_turn_credentials() {
        let manager = CallManager::new();

        let creds = TurnCredentials {
            username: "user123".to_string(),
            password: "pass456".to_string(),
            uris: vec!["turn:example.com:3478".to_string()],
        };

        manager.set_turn_credentials(creds).await;

        let ice_servers = manager.get_ice_servers().await;
        assert_eq!(ice_servers.len(), 2); // STUN + TURN
    }
}
