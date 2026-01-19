//! Protocol module
//!
//! Protobuf message definitions and codec.

// pub mod codec;
// pub mod validation;

// Protobuf generated code will be included here
// include!(concat!(env!("OUT_DIR"), "/mepassa.protocol.rs"));

use serde::{Deserialize, Serialize};

/// Message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Text,
    Image,
    Video,
    Audio,
    File,
    Location,
    Contact,
    Sticker,
    Reaction,
    Edit,
    Delete,
}

/// Message struct (simplified, will be replaced by Protobuf)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub message_type: MessageType,
    pub encrypted_content: Vec<u8>,
    pub timestamp: i64,
    pub signature: Vec<u8>,
}

/// Message content (decrypted)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    Text { text: String },
    Image { data: Vec<u8>, mime_type: String },
    Audio { data: Vec<u8>, duration: u32 },
}
