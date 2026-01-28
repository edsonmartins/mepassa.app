//! Message Handler
//!
//! Handles incoming messages from the network:
//! 1. Validates message format
//! 2. Decrypts content (if encrypted)
//! 3. Stores message in database
//! 4. Emits events to application layer
//! 5. Sends acknowledgment back to sender

use libp2p::PeerId;
use std::sync::Arc;

use crate::{
    protocol::{
        pb::message::Payload, AckMessage, AckStatus, Message, MessageType, ReadReceipt,
        TextMessage, TypingIndicator,
    },
    storage::{Database, MessageStatus, NewMessage, UpdateMessage},
    utils::error::{MePassaError, Result},
};

/// Message handler
///
/// Processes incoming messages and coordinates between network, storage, and crypto layers.
pub struct MessageHandler {
    /// Local peer ID
    local_peer_id: String,

    /// Database for storing messages (thread-safe via internal Mutex)
    database: Arc<Database>,

    /// Event callback for notifying UI
    event_tx: Option<tokio::sync::mpsc::UnboundedSender<MessageEvent>>,
}

impl MessageHandler {
    /// Create a new message handler
    pub fn new(
        local_peer_id: String,
        database: Arc<Database>,
        event_tx: Option<tokio::sync::mpsc::UnboundedSender<MessageEvent>>,
    ) -> Self {
        Self {
            local_peer_id,
            database,
            event_tx,
        }
    }

    /// Handle an incoming message request
    ///
    /// Returns an acknowledgment message to send back to the sender.
    pub async fn handle_incoming_message(
        &self,
        from_peer: PeerId,
        message: Message,
    ) -> Result<AckMessage> {
        tracing::info!(
            "📨 Processing message {} from {} (type: {:?})",
            message.id,
            from_peer,
            MessageType::try_from(message.r#type).unwrap_or(MessageType::Unspecified)
        );

        // Validate message
        if let Err(e) = self.validate_message(&message) {
            tracing::warn!("Invalid message {}: {}", message.id, e);
            return Ok(self.create_ack(&message.id, AckStatus::Error, Some(e.to_string())));
        }

        // Process based on message type
        let result = match message.payload {
            Some(Payload::Text(ref text_msg)) => {
                self.handle_text_message(&message, text_msg).await
            }
            Some(Payload::Ack(ref ack_msg)) => {
                self.handle_ack_message(&message, ack_msg).await
            }
            Some(Payload::Typing(ref typing_msg)) => {
                self.handle_typing_indicator(&message, typing_msg).await
            }
            Some(Payload::ReadReceipt(ref read_msg)) => {
                self.handle_read_receipt(&message, read_msg).await
            }
            None => {
                tracing::warn!("Message {} has no payload", message.id);
                Err(MePassaError::Protocol(
                    "Message has no payload".to_string(),
                ))
            }
        };

        match result {
            Ok(_) => Ok(self.create_ack(&message.id, AckStatus::Received, None)),
            Err(e) => {
                tracing::error!("Failed to process message {}: {}", message.id, e);
                Ok(self.create_ack(&message.id, AckStatus::Error, Some(e.to_string())))
            }
        }
    }

    /// Handle acknowledgment for an outgoing message
    pub async fn handle_outgoing_ack(&self, ack: AckMessage) -> Result<()> {
        tracing::info!(
            "✅ Received ACK for message {} - status: {:?}",
            ack.message_id,
            AckStatus::try_from(ack.status).unwrap_or(AckStatus::Unspecified)
        );

        // Update message status in database
        let status = match AckStatus::try_from(ack.status) {
            Ok(AckStatus::Received) => MessageStatus::Delivered,
            Ok(AckStatus::Error) => MessageStatus::Failed,
            _ => return Ok(()), // Ignore other statuses
        };

        {
            let update = UpdateMessage {
                status: Some(status),
                ..Default::default()
            };
            if let Err(e) = self.database.update_message(&ack.message_id, &update) {
                tracing::warn!("Failed to update message status: {}", e);
            }
        }

        // Emit event (include recipient when available)
        let to_peer_id = self
            .database
            .get_message(&ack.message_id)
            .ok()
            .and_then(|msg| msg.recipient_peer_id);

        self.emit_event(MessageEvent::MessageDelivered {
            message_id: ack.message_id.clone(),
            status,
            to_peer_id,
        });

        Ok(())
    }

    /// Validate message format
    fn validate_message(&self, message: &Message) -> Result<()> {
        // Check message ID
        if message.id.is_empty() {
            return Err(MePassaError::Protocol("Empty message ID".to_string()));
        }

        // Check sender
        if message.sender_peer_id.is_empty() {
            return Err(MePassaError::Protocol("Empty sender peer ID".to_string()));
        }

        // Check recipient (should be us)
        if message.recipient_peer_id != self.local_peer_id {
            return Err(MePassaError::Protocol(format!(
                "Message not addressed to us (expected: {}, got: {})",
                self.local_peer_id, message.recipient_peer_id
            )));
        }

        // Check timestamp is not too old (> 7 days)
        let now = chrono::Utc::now().timestamp_millis();
        let age_ms = now - message.timestamp;
        if age_ms > 7 * 24 * 60 * 60 * 1000 {
            tracing::warn!(
                "Message {} is very old ({} days), but accepting anyway",
                message.id,
                age_ms / (24 * 60 * 60 * 1000)
            );
        }

        Ok(())
    }

    /// Handle text message
    async fn handle_text_message(&self, message: &Message, text: &TextMessage) -> Result<()> {
        tracing::debug!("📝 Received text: \"{}\"", text.content);

        // Get or create conversation (Database has internal Mutex for thread-safety)
        let conversation_id = self.database.get_or_create_conversation(&message.sender_peer_id)?;

        // Store message in database
        let new_msg = NewMessage {
            message_id: message.id.clone(),
            conversation_id: conversation_id.clone(),
            sender_peer_id: message.sender_peer_id.clone(),
            recipient_peer_id: Some(message.recipient_peer_id.clone()),
            message_type: "text".to_string(),
            content_encrypted: None, // TODO: Support E2E encryption
            content_plaintext: Some(text.content.clone()),
            status: MessageStatus::Delivered,
            parent_message_id: if text.reply_to_id.is_empty() {
                None
            } else {
                Some(text.reply_to_id.clone())
            },
        };

        self.database.insert_message(&new_msg)?;

        // Update conversation last message
        self.database.update_conversation_last_message(&conversation_id, &message.id)?;

        tracing::info!("💾 Stored message {} in conversation {}", message.id, conversation_id);

        // Emit event to UI
        self.emit_event(MessageEvent::MessageReceived {
            message_id: message.id.clone(),
            from_peer_id: message.sender_peer_id.clone(),
            conversation_id: conversation_id.clone(),
            content: text.content.clone(),
            message: message.clone(),
        });

        Ok(())
    }

    /// Handle acknowledgment message
    async fn handle_ack_message(&self, _message: &Message, ack: &AckMessage) -> Result<()> {
        // This is an ACK for one of our messages
        self.handle_outgoing_ack(ack.clone()).await
    }

    /// Handle typing indicator
    async fn handle_typing_indicator(
        &self,
        message: &Message,
        typing: &TypingIndicator,
    ) -> Result<()> {
        tracing::debug!(
            "⌨️ Typing indicator from {}: {}",
            message.sender_peer_id,
            typing.is_typing
        );

        // Emit event to UI
        self.emit_event(MessageEvent::TypingIndicator {
            from_peer_id: message.sender_peer_id.clone(),
            is_typing: typing.is_typing,
        });

        Ok(())
    }

    /// Handle read receipt
    async fn handle_read_receipt(&self, message: &Message, read: &ReadReceipt) -> Result<()> {
        tracing::debug!(
            "✓✓ Read receipt from {} for message {}",
            message.sender_peer_id,
            read.message_id
        );

        // Update message status in database
        {
            let update = UpdateMessage {
                status: Some(MessageStatus::Read),
                read_at: Some(read.read_at),
                ..Default::default()
            };
            if let Err(e) = self.database.update_message(&read.message_id, &update) {
                tracing::warn!("Failed to update message read status: {}", e);
            }
        }

        // Emit event to UI
        self.emit_event(MessageEvent::MessageRead {
            message_id: read.message_id.clone(),
            by_peer_id: message.sender_peer_id.clone(),
            read_at: read.read_at,
        });

        Ok(())
    }

    /// Create an acknowledgment message
    fn create_ack(&self, message_id: &str, status: AckStatus, error: Option<String>) -> AckMessage {
        AckMessage {
            message_id: message_id.to_string(),
            status: status as i32,
            error: error.unwrap_or_default(),
        }
    }

    /// Emit an event to the application layer
    fn emit_event(&self, event: MessageEvent) {
        if let Some(ref tx) = self.event_tx {
            if let Err(e) = tx.send(event) {
                tracing::warn!("Failed to emit message event: {}", e);
            }
        }
    }
}

/// Message events emitted to application layer
#[derive(Debug, Clone)]
pub enum MessageEvent {
    /// New message received
    MessageReceived {
        message_id: String,
        from_peer_id: String,
        conversation_id: String,
        content: String,
        message: Message,
    },

    /// Message delivered (ACK received)
    MessageDelivered {
        message_id: String,
        status: MessageStatus,
        to_peer_id: Option<String>,
    },

    /// Message read by recipient
    MessageRead {
        message_id: String,
        by_peer_id: String,
        read_at: i64,
    },

    /// Typing indicator
    TypingIndicator {
        from_peer_id: String,
        is_typing: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{contacts::NewContact, schema::init_schema};
    use libp2p::PeerId;

    #[tokio::test]
    async fn test_handle_text_message() {
        let db = Database::in_memory().unwrap();
        init_schema(&db).unwrap();

        // Generate valid PeerId for sender
        let sender_peer = PeerId::random();
        let sender_peer_id = sender_peer.to_string();
        let local_peer_id = "local-peer".to_string();

        // Insert test contact (required for foreign keys)
        let contact = NewContact {
            peer_id: sender_peer_id.clone(),
            username: None,
            display_name: Some("Sender".to_string()),
            public_key: vec![1, 2, 3],
            prekey_bundle_json: None,
        };
        db.insert_contact(&contact).unwrap();

        let db_arc = Arc::new(db);

        let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();

        let handler = MessageHandler::new(
            local_peer_id.clone(),
            db_arc,
            Some(event_tx),
        );

        // Create test message
        let message = Message {
            id: "msg-123".to_string(),
            sender_peer_id: sender_peer_id,
            recipient_peer_id: local_peer_id,
            timestamp: chrono::Utc::now().timestamp_millis(),
            r#type: MessageType::Text as i32,
            payload: Some(Payload::Text(TextMessage {
                content: "Hello, World!".to_string(),
                reply_to_id: String::new(),
                metadata: std::collections::HashMap::new(),
            })),
        };

        // Handle message
        let ack = handler
            .handle_incoming_message(sender_peer, message)
            .await
            .unwrap();

        // Verify ACK
        assert_eq!(ack.message_id, "msg-123");
        assert_eq!(ack.status, AckStatus::Received as i32);

        // Verify event emitted
        let event = event_rx.recv().await.unwrap();
        match event {
            MessageEvent::MessageReceived {
                message_id,
                content,
                message,
                ..
            } => {
                assert_eq!(message_id, "msg-123");
                assert_eq!(content, "Hello, World!");
                assert_eq!(message.id, "msg-123");
            }
            _ => panic!("Expected MessageReceived event"),
        }
    }

    #[tokio::test]
    async fn test_handle_ack() {
        let db = Database::in_memory().unwrap();
        init_schema(&db).unwrap();

        // Insert test contacts (required for foreign keys)
        let local_peer_id = "local-peer".to_string();
        let remote_peer_id = PeerId::random().to_string();

        // Insert local peer as contact
        let local_contact = NewContact {
            peer_id: local_peer_id.clone(),
            username: None,
            display_name: Some("Local".to_string()),
            public_key: vec![1, 2, 3],
            prekey_bundle_json: None,
        };
        db.insert_contact(&local_contact).unwrap();

        // Insert remote peer as contact
        let remote_contact = NewContact {
            peer_id: remote_peer_id.clone(),
            username: None,
            display_name: Some("Remote".to_string()),
            public_key: vec![4, 5, 6],
            prekey_bundle_json: None,
        };
        db.insert_contact(&remote_contact).unwrap();

        // Create conversation first
        let conversation_id = db.get_or_create_conversation(&remote_peer_id).unwrap();

        // Insert a message first
        let new_msg = NewMessage {
            message_id: "msg-456".to_string(),
            conversation_id,
            sender_peer_id: local_peer_id.clone(),
            recipient_peer_id: Some(remote_peer_id),
            message_type: "text".to_string(),
            content_encrypted: None,
            content_plaintext: Some("Test".to_string()),
            status: MessageStatus::Sent,
            parent_message_id: None,
        };
        db.insert_message(&new_msg).unwrap();

        let db_arc = Arc::new(db);

        let handler = MessageHandler::new(
            local_peer_id,
            Arc::clone(&db_arc),
            None,
        );

        // Create ACK message
        let ack = AckMessage {
            message_id: "msg-456".to_string(),
            status: AckStatus::Received as i32,
            error: String::new(),
        };

        // Handle ACK
        handler.handle_outgoing_ack(ack).await.unwrap();

        // Verify message status updated
        {
            let message = db_arc.get_message("msg-456").unwrap();
            assert_eq!(message.status, MessageStatus::Delivered);
        }
    }
}
