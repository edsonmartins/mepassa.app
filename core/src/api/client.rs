//! Client API
//!
//! Public API for MePassa client.

use libp2p::{Multiaddr, PeerId};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::events::{ClientEvent, EventCallback};
use crate::{
    identity::Identity,
    network::NetworkManager,
    protocol::{pb::message::Payload, Message, MessageType, TextMessage},
    storage::{Database, NewMessage, MessageStatus},
    utils::error::{MePassaError, Result},
};
#[cfg(feature = "voip")]
use crate::voip::{CallManager, VoIPIntegration};

/// MePassa Client
///
/// Main entry point for using the MePassa P2P messaging platform.
pub struct Client {
    /// Local peer ID (libp2p)
    peer_id: PeerId,
    /// Local identity (keypair + prekeys)
    identity: Identity,
    /// Network manager (P2P networking)
    network: Arc<RwLock<NetworkManager>>,
    /// Local storage (SQLite)
    database: Database,
    /// Event callbacks
    callbacks: Arc<RwLock<Vec<Box<dyn EventCallback>>>>,
    /// Data directory
    data_dir: PathBuf,
    /// Call manager (VoIP)
    #[cfg(feature = "voip")]
    call_manager: Arc<CallManager>,
    /// VoIP integration (network ↔ calls)
    #[cfg(feature = "voip")]
    voip_integration: Arc<VoIPIntegration>,
    /// Group manager (FASE 15)
    group_manager: Arc<crate::group::GroupManager>,
}

impl Client {
    /// Create a new client (use ClientBuilder instead)
    pub(crate) fn new(
        peer_id: PeerId,
        identity: Identity,
        network: Arc<RwLock<NetworkManager>>,
        database: Database,
        data_dir: PathBuf,
        #[cfg(feature = "voip")]
        call_manager: Arc<CallManager>,
        #[cfg(feature = "voip")]
        voip_integration: Arc<VoIPIntegration>,
        group_manager: Arc<crate::group::GroupManager>,
    ) -> Self {
        Self {
            peer_id,
            identity,
            network,
            database,
            callbacks: Arc::new(RwLock::new(Vec::new())),
            data_dir,
            #[cfg(feature = "voip")]
            call_manager,
            #[cfg(feature = "voip")]
            voip_integration,
            group_manager,
        }
    }

    /// Get local peer ID
    pub fn local_peer_id(&self) -> PeerId {
        self.peer_id
    }

    /// Get local identity
    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    /// Get database
    pub fn database(&self) -> &Database {
        &self.database
    }

    /// Get data directory
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    /// Register an event callback
    pub async fn register_callback<C>(&self, callback: C)
    where
        C: EventCallback + 'static,
    {
        let mut callbacks = self.callbacks.write().await;
        callbacks.push(Box::new(callback));
    }

    /// Emit an event to all callbacks
    async fn emit_event(&self, event: ClientEvent) {
        let callbacks = self.callbacks.read().await;
        for callback in callbacks.iter() {
            callback.on_event(event.clone());
        }
    }

    /// Start listening on a multiaddr
    pub async fn listen_on(&self, addr: Multiaddr) -> Result<()> {
        let mut network = self.network.write().await;
        network.listen_on(addr)
    }

    /// Connect to a peer
    pub async fn connect_to_peer(&self, peer_id: PeerId, addr: Multiaddr) -> Result<()> {
        let mut network = self.network.write().await;
        network.add_peer_to_dht(peer_id, addr.clone());
        network.dial(peer_id, addr)?;

        self.emit_event(ClientEvent::PeerConnected { peer_id }).await;
        Ok(())
    }

    /// Send a text message to a peer
    pub async fn send_text_message(&self, to: PeerId, content: String) -> Result<String> {
        // Generate message ID
        let message_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp_millis();

        // Create protocol message
        let proto_message = Message {
            id: message_id.clone(),
            sender_peer_id: self.local_peer_id().to_string(),
            recipient_peer_id: to.to_string(),
            timestamp,
            r#type: MessageType::Text as i32,
            payload: Some(Payload::Text(TextMessage {
                content: content.clone(),
                reply_to_id: String::new(),
                metadata: std::collections::HashMap::new(),
            })),
        };

        // Send via network
        {
            let mut network = self.network.write().await;
            network.send_message(to, proto_message)?;
        }

        // Store in database
        let conversation_id = self.database.get_or_create_conversation(&to.to_string())?;
        let new_msg = NewMessage {
            message_id: message_id.clone(),
            conversation_id: conversation_id.clone(),
            sender_peer_id: self.local_peer_id().to_string(),
            recipient_peer_id: Some(to.to_string()),
            message_type: "text".to_string(),
            content_encrypted: None,
            content_plaintext: Some(content),
            status: MessageStatus::Sent,
            parent_message_id: None,
        };
        self.database.insert_message(&new_msg)?;
        self.database
            .update_conversation_last_message(&conversation_id, &message_id)?;

        // Emit event
        self.emit_event(ClientEvent::MessageSent {
            message_id: message_id.clone(),
            to,
        })
        .await;

        Ok(message_id)
    }

    /// Get messages for a conversation
    pub fn get_conversation_messages(
        &self,
        peer_id: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<crate::storage::Message>> {
        let conversation_id = format!("1:1:{}", peer_id);
        self.database
            .get_conversation_messages(&conversation_id, limit, offset)
            .map_err(|e| MePassaError::Storage(e.to_string()))
    }

    /// List all conversations
    pub fn list_conversations(&self) -> Result<Vec<crate::storage::Conversation>> {
        self.database
            .list_conversations()
            .map_err(|e| MePassaError::Storage(e.to_string()))
    }

    /// Search messages
    pub fn search_messages(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<Vec<crate::storage::Message>> {
        self.database
            .search_messages(query, limit)
            .map_err(|e| MePassaError::Storage(e.to_string()))
    }

    /// Mark conversation as read
    pub fn mark_conversation_read(&self, peer_id: &str) -> Result<()> {
        let conversation_id = format!("1:1:{}", peer_id);
        self.database
            .mark_conversation_read(&conversation_id)
            .map_err(|e| MePassaError::Storage(e.to_string()))
    }

    /// Get connected peers count
    pub async fn connected_peers_count(&self) -> usize {
        let network = self.network.read().await;
        network.connected_peers()
    }

    /// Bootstrap DHT
    pub async fn bootstrap(&self) -> Result<()> {
        let mut network = self.network.write().await;
        network.bootstrap()
    }

    // === VoIP Methods ===
    #[cfg(feature = "voip")]
    /// Start a voice call to a peer
    pub async fn start_call(&self, to_peer_id: String) -> Result<String> {
        self.voip_integration
            .start_call(to_peer_id)
            .await
            .map_err(|e| MePassaError::Other(format!("VoIP error: {}", e)))
    }

    #[cfg(feature = "voip")]
    /// Accept an incoming call
    pub async fn accept_call(&self, call_id: String) -> Result<()> {
        self.voip_integration
            .accept_call(call_id)
            .await
            .map_err(|e| MePassaError::Other(format!("VoIP error: {}", e)))
    }

    #[cfg(feature = "voip")]
    /// Reject an incoming call
    pub async fn reject_call(&self, call_id: String, reason: Option<String>) -> Result<()> {
        self.voip_integration
            .reject_call(call_id, reason)
            .await
            .map_err(|e| MePassaError::Other(format!("VoIP error: {}", e)))
    }

    #[cfg(feature = "voip")]
    /// Hang up an active call
    pub async fn hangup_call(&self, call_id: String) -> Result<()> {
        self.voip_integration
            .hangup_call(call_id)
            .await
            .map_err(|e| MePassaError::Other(format!("VoIP error: {}", e)))
    }

    #[cfg(feature = "voip")]
    /// Toggle audio mute for a call
    pub async fn toggle_mute(&self, call_id: String) -> Result<()> {
        let call_manager = self
            .call_manager
            .as_ref()
            .ok_or_else(|| MePassaError::InvalidState("CallManager not initialized".to_string()))?;

        call_manager
            .toggle_mute(call_id)
            .await
            .map_err(|e| MePassaError::Other(format!("VoIP error: {}", e)))
    }

    #[cfg(feature = "voip")]
    /// Toggle speakerphone for a call
    pub async fn toggle_speakerphone(&self, call_id: String) -> Result<()> {
        let call_manager = self
            .call_manager
            .as_ref()
            .ok_or_else(|| MePassaError::InvalidState("CallManager not initialized".to_string()))?;

        call_manager
            .toggle_speakerphone(call_id)
            .await
            .map_err(|e| MePassaError::Other(format!("VoIP error: {}", e)))
    }

    // ========== Video Methods (FASE 14) ==========

    #[cfg(feature = "voip")]
    /// Enable video for an active call
    pub async fn enable_video(
        &self,
        call_id: String,
        codec: crate::voip::VideoCodec,
    ) -> Result<()> {
        self.call_manager
            .enable_video(&call_id, codec)
            .await
            .map_err(|e| MePassaError::Other(format!("Failed to enable video: {}", e)))
    }

    #[cfg(feature = "voip")]
    /// Disable video for an active call
    pub async fn disable_video(&self, call_id: String) -> Result<()> {
        self.call_manager
            .disable_video(&call_id)
            .await
            .map_err(|e| MePassaError::Other(format!("Failed to disable video: {}", e)))
    }

    #[cfg(feature = "voip")]
    /// Send video frame to remote peer
    ///
    /// Frame data should be pre-encoded (H.264 NALUs or VP8/VP9 frames)
    pub async fn send_video_frame(
        &self,
        call_id: String,
        frame_data: &[u8],
        _width: u32,
        _height: u32,
    ) -> Result<()> {
        self.call_manager
            .send_video_frame(&call_id, frame_data)
            .await
            .map_err(|e| MePassaError::Other(format!("Failed to send video frame: {}", e)))
    }

    // ========== Group Methods (FASE 15) ==========

    /// Create a new group
    pub async fn create_group(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<crate::ffi::FfiGroup> {
        use crate::ffi::FfiGroup;

        let (group, _topic_hash) = self
            .group_manager
            .create_group(name, description)
            .await
            .map_err(|e| MePassaError::Other(format!("Failed to create group: {}", e)))?;

        Ok(FfiGroup::from_group(&group, &self.local_peer_id().to_string()))
    }

    /// Join an existing group
    pub async fn join_group(&self, group_id: String, group_name: String) -> Result<()> {
        let _topic_hash = self.group_manager
            .join_group(group_id, group_name)
            .await
            .map_err(|e| MePassaError::Other(format!("Failed to join group: {}", e)))?;
        Ok(())
    }

    /// Leave a group
    pub async fn leave_group(&self, group_id: String) -> Result<()> {
        self.group_manager
            .leave_group(&group_id)
            .await
            .map_err(|e| MePassaError::Other(format!("Failed to leave group: {}", e)))
    }

    /// Add a member to a group (admin only)
    pub async fn add_group_member(&self, group_id: String, peer_id: String) -> Result<()> {
        self.group_manager
            .add_member(&group_id, &peer_id)
            .await
            .map_err(|e| MePassaError::Other(format!("Failed to add member: {}", e)))
    }

    /// Remove a member from a group (admin only)
    pub async fn remove_group_member(&self, group_id: String, peer_id: String) -> Result<()> {
        self.group_manager
            .remove_member(&group_id, &peer_id)
            .await
            .map_err(|e| MePassaError::Other(format!("Failed to remove member: {}", e)))
    }

    /// Get all groups
    pub async fn get_groups(&self) -> Result<Vec<crate::ffi::FfiGroup>> {
        use crate::ffi::FfiGroup;

        let groups = self.group_manager.get_all_groups().await;
        let local_peer_id = self.local_peer_id().to_string();

        Ok(groups
            .iter()
            .map(|g| FfiGroup::from_group(g, &local_peer_id))
            .collect())
    }

    // /// Run event loop (blocking)
    // pub async fn run(&self) -> Result<()> {
    //     let mut network = self.network.write().await;
    //     network.run().await
    // }
}

#[cfg(test)]
mod tests {
    use crate::api::ClientBuilder;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_client() {
        let temp_dir = TempDir::new().unwrap();
        let data_dir = temp_dir.path().to_path_buf();

        let client = ClientBuilder::new()
            .data_dir(data_dir)
            .build()
            .await
            .unwrap();

        assert!(client.local_peer_id().to_string().len() > 0);
    }

    #[tokio::test]
    async fn test_list_conversations() {
        let temp_dir = TempDir::new().unwrap();
        let data_dir = temp_dir.path().to_path_buf();

        let client = ClientBuilder::new()
            .data_dir(data_dir)
            .build()
            .await
            .unwrap();

        let conversations = client.list_conversations().unwrap();
        assert_eq!(conversations.len(), 0);
    }
}
