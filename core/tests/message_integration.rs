//! Message Integration Tests
//!
//! End-to-end tests for message exchange with full stack:
//! - NetworkManager with MessageHandler
//! - Database persistence
//! - Event emission
//! - ACK handling
//! - Status updates

use libp2p::{identity::Keypair, Multiaddr, PeerId};
use mepassa_core::{
    network::{MessageEvent, MessageHandler, NetworkManager},
    protocol::{pb::message::Payload, AckStatus, Message, MessageType, TextMessage},
    storage::{schema::init_schema, Database, MessageStatus},
};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};
use uuid::Uuid;

/// Test helper to create a peer with full stack
struct TestPeer {
    peer_id: PeerId,
    network: Arc<RwLock<NetworkManager>>,
    database: Arc<RwLock<Database>>,
    message_handler: Arc<MessageHandler>,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<MessageEvent>,
}

impl TestPeer {
    async fn new(port: u16) -> Self {
        // Create keypair and peer ID
        let keypair = Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());

        // Create database
        let db = Database::in_memory().expect("Failed to create database");
        init_schema(&db).expect("Failed to init schema");
        let database = Arc::new(RwLock::new(db));

        // Create event channel
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();

        // Create message handler
        let message_handler = Arc::new(MessageHandler::new(
            peer_id.to_string(),
            Arc::clone(&database),
            Some(event_tx),
        ));

        // Create network manager
        let mut network = NetworkManager::new(keypair).expect("Failed to create network");

        // Set message handler
        network.set_message_handler(Arc::clone(&message_handler));

        let network = Arc::new(RwLock::new(network));

        // Start listening
        let addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/{}", port)
            .parse()
            .expect("Invalid address");

        {
            let mut net = network.write().await;
            net.listen_on(addr.clone()).expect("Failed to listen");
        }

        tracing::info!("✅ Created peer {} on port {}", peer_id, port);

        Self {
            peer_id,
            network,
            database,
            message_handler,
            event_rx,
        }
    }

    async fn connect_to(&self, other_peer_id: PeerId, other_addr: Multiaddr) {
        let mut network = self.network.write().await;
        network.add_peer_to_dht(other_peer_id, other_addr.clone());
        network.dial(other_peer_id, other_addr).expect("Failed to dial");
    }

    async fn send_text_message(&self, to_peer_id: &PeerId, content: &str) -> String {
        let message_id = Uuid::new_v4().to_string();

        let message = Message {
            id: message_id.clone(),
            sender_peer_id: self.peer_id.to_string(),
            recipient_peer_id: to_peer_id.to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            r#type: MessageType::Text as i32,
            payload: Some(Payload::Text(TextMessage {
                content: content.to_string(),
                reply_to_id: String::new(),
                metadata: std::collections::HashMap::new(),
            })),
        };

        let mut network = self.network.write().await;
        network
            .send_message(to_peer_id.clone(), message)
            .expect("Failed to send message");

        tracing::info!("📤 Sent message {} to {}", message_id, to_peer_id);

        message_id
    }

    async fn get_message(&self, message_id: &str) -> Option<mepassa_core::storage::messages::Message> {
        let db = self.database.read().await;
        db.get_message(message_id).ok()
    }

    async fn get_conversation_messages(&self, conversation_id: &str) -> Vec<mepassa_core::storage::messages::Message> {
        let db = self.database.read().await;
        db.get_conversation_messages(conversation_id, None, None)
            .unwrap_or_default()
    }

    async fn wait_for_event(&mut self, timeout: Duration) -> Option<MessageEvent> {
        tokio::time::timeout(timeout, self.event_rx.recv())
            .await
            .ok()
            .flatten()
    }
}

#[tokio::test]
#[ignore] // TODO: Requires event loop to be running for full end-to-end test
async fn test_end_to_end_message_exchange() {
    // Setup logging
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();

    tracing::info!("🧪 Starting end-to-end message exchange test");

    // Create two peers
    let mut peer_a = TestPeer::new(20001).await;
    let mut peer_b = TestPeer::new(20002).await;

    tracing::info!("👤 Peer A: {}", peer_a.peer_id);
    tracing::info!("👤 Peer B: {}", peer_b.peer_id);

    // Connect peers
    let addr_b: Multiaddr = "/ip4/127.0.0.1/tcp/20002".parse().unwrap();
    peer_a.connect_to(peer_b.peer_id, addr_b).await;

    // Wait for connection
    sleep(Duration::from_secs(2)).await;

    // Check connection
    {
        let net_a = peer_a.network.read().await;
        let net_b = peer_b.network.read().await;
        tracing::info!("🔗 Peer A connections: {}", net_a.connected_peers());
        tracing::info!("🔗 Peer B connections: {}", net_b.connected_peers());

        assert!(net_a.connected_peers() > 0, "Peer A should be connected");
        assert!(net_b.connected_peers() > 0, "Peer B should be connected");
    }

    // Send message from A to B
    let message_id = peer_a
        .send_text_message(&peer_b.peer_id, "Hello from Peer A!")
        .await;

    tracing::info!("📤 Sent message: {}", message_id);

    // Wait for message to be delivered and processed
    sleep(Duration::from_secs(3)).await;

    // TODO: Verify Peer B received and stored the message
    // This requires running the event loop which we'll implement next

    tracing::info!("✅ Test completed");
}

#[tokio::test]
async fn test_message_handler_processing() {
    // Setup logging
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();

    tracing::info!("🧪 Testing MessageHandler processing");

    // Create test database
    let db = Database::in_memory().expect("Failed to create database");
    init_schema(&db).expect("Failed to init schema");

    // Generate sender peer ID first
    let sender_peer = PeerId::random();
    let sender_peer_id = sender_peer.to_string();

    // Insert test contact with the SAME peer ID
    use mepassa_core::storage::contacts::NewContact;
    let contact = NewContact {
        peer_id: sender_peer_id.clone(),
        username: None,
        display_name: Some("Test Sender".to_string()),
        public_key: vec![1, 2, 3],
        prekey_bundle_json: None,
    };
    db.insert_contact(&contact).expect("Failed to insert contact");

    let db_arc = Arc::new(RwLock::new(db));

    // Create event channel
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();

    // Create message handler
    let handler = MessageHandler::new(
        "local-peer".to_string(),
        Arc::clone(&db_arc),
        Some(event_tx),
    );

    // Create test message
    let message_id = Uuid::new_v4().to_string();

    let message = Message {
        id: message_id.clone(),
        sender_peer_id: sender_peer.to_string(),
        recipient_peer_id: "local-peer".to_string(),
        timestamp: chrono::Utc::now().timestamp_millis(),
        r#type: MessageType::Text as i32,
        payload: Some(Payload::Text(TextMessage {
            content: "Test message content".to_string(),
            reply_to_id: String::new(),
            metadata: std::collections::HashMap::new(),
        })),
    };

    tracing::info!("📨 Processing message {}", message_id);

    // Process message
    let ack = handler
        .handle_incoming_message(sender_peer, message)
        .await
        .expect("Failed to handle message");

    // Verify ACK
    assert_eq!(ack.message_id, message_id);
    assert_eq!(ack.status, AckStatus::Received as i32);
    tracing::info!("✅ ACK verified: status = Received");

    // Verify event was emitted
    let event = tokio::time::timeout(Duration::from_secs(1), event_rx.recv())
        .await
        .expect("Timeout waiting for event")
        .expect("No event received");

    match event {
        MessageEvent::MessageReceived {
            message_id: recv_msg_id,
            content,
            ..
        } => {
            assert_eq!(recv_msg_id, message_id);
            assert_eq!(content, "Test message content");
            tracing::info!("✅ Event verified: MessageReceived");
        }
        _ => panic!("Expected MessageReceived event"),
    }

    // Verify message was stored in database
    {
        let db = db_arc.read().await;
        let stored_msg = db
            .get_message(&message_id)
            .expect("Failed to get message");

        assert_eq!(stored_msg.message_id, message_id);
        assert_eq!(stored_msg.content_plaintext, Some("Test message content".to_string()));
        assert_eq!(stored_msg.status, MessageStatus::Delivered);
        tracing::info!("✅ Database verified: message stored with status Delivered");
    }

    tracing::info!("✅ MessageHandler processing test passed");
}

#[tokio::test]
async fn test_ack_handling() {
    // Setup logging
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();

    tracing::info!("🧪 Testing ACK handling");

    // Create test database
    let db = Database::in_memory().expect("Failed to create database");
    init_schema(&db).expect("Failed to init schema");

    // Insert test contacts
    use mepassa_core::storage::contacts::NewContact;
    let local_peer_id = "local-peer".to_string();
    let remote_peer_id = PeerId::random().to_string();

    let local_contact = NewContact {
        peer_id: local_peer_id.clone(),
        username: None,
        display_name: Some("Local".to_string()),
        public_key: vec![1, 2, 3],
        prekey_bundle_json: None,
    };
    db.insert_contact(&local_contact).expect("Failed to insert contact");

    let remote_contact = NewContact {
        peer_id: remote_peer_id.clone(),
        username: None,
        display_name: Some("Remote".to_string()),
        public_key: vec![4, 5, 6],
        prekey_bundle_json: None,
    };
    db.insert_contact(&remote_contact).expect("Failed to insert contact");

    // Create conversation and insert message
    let conversation_id = db.get_or_create_conversation(&remote_peer_id)
        .expect("Failed to create conversation");

    use mepassa_core::storage::messages::NewMessage;
    let message_id = Uuid::new_v4().to_string();
    let new_msg = NewMessage {
        message_id: message_id.clone(),
        conversation_id,
        sender_peer_id: local_peer_id.clone(),
        recipient_peer_id: Some(remote_peer_id.clone()),
        message_type: "text".to_string(),
        content_encrypted: None,
        content_plaintext: Some("Test message".to_string()),
        status: MessageStatus::Sent,
        parent_message_id: None,
    };
    db.insert_message(&new_msg).expect("Failed to insert message");

    let db_arc = Arc::new(RwLock::new(db));

    // Create event channel
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();

    // Create message handler
    let handler = MessageHandler::new(
        local_peer_id,
        Arc::clone(&db_arc),
        Some(event_tx),
    );

    // Create ACK
    use mepassa_core::protocol::AckMessage;
    let ack = AckMessage {
        message_id: message_id.clone(),
        status: AckStatus::Received as i32,
        error: String::new(),
    };

    tracing::info!("📨 Processing ACK for message {}", message_id);

    // Process ACK
    handler
        .handle_outgoing_ack(ack)
        .await
        .expect("Failed to handle ACK");

    // Verify message status was updated
    {
        let db = db_arc.read().await;
        let message = db.get_message(&message_id).expect("Failed to get message");
        assert_eq!(message.status, MessageStatus::Delivered);
        tracing::info!("✅ Message status updated to Delivered");
    }

    // Verify event was emitted
    let event = tokio::time::timeout(Duration::from_secs(1), event_rx.recv())
        .await
        .expect("Timeout waiting for event")
        .expect("No event received");

    match event {
        MessageEvent::MessageDelivered {
            message_id: delivered_msg_id,
            status,
        } => {
            assert_eq!(delivered_msg_id, message_id);
            assert_eq!(status, MessageStatus::Delivered);
            tracing::info!("✅ Event verified: MessageDelivered");
        }
        _ => panic!("Expected MessageDelivered event"),
    }

    tracing::info!("✅ ACK handling test passed");
}
