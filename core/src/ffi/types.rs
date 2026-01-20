//! FFI-safe types for UniFFI bindings

use crate::storage::{Conversation, Message, MessageStatus as InternalMessageStatus};

/// FFI-safe error type (rich error with messages)
#[derive(Debug, thiserror::Error)]
pub enum MePassaFfiError {
    #[error("Identity error: {message}")]
    Identity { message: String },

    #[error("Crypto error: {message}")]
    Crypto { message: String },

    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Storage error: {message}")]
    Storage { message: String },

    #[error("Protocol error: {message}")]
    Protocol { message: String },

    #[error("IO error: {message}")]
    Io { message: String },

    #[error("Other error: {message}")]
    Other { message: String },
}

impl From<crate::utils::error::MePassaError> for MePassaFfiError {
    fn from(err: crate::utils::error::MePassaError) -> Self {
        match err {
            crate::utils::error::MePassaError::Identity(s) => {
                MePassaFfiError::Identity { message: s }
            }
            crate::utils::error::MePassaError::Crypto(s) => MePassaFfiError::Crypto { message: s },
            crate::utils::error::MePassaError::Network(s) => {
                MePassaFfiError::Network { message: s }
            }
            crate::utils::error::MePassaError::Storage(s) => {
                MePassaFfiError::Storage { message: s }
            }
            crate::utils::error::MePassaError::Protocol(s) => {
                MePassaFfiError::Protocol { message: s }
            }
            crate::utils::error::MePassaError::Io(e) => MePassaFfiError::Io {
                message: e.to_string(),
            },
            crate::utils::error::MePassaError::Other(s) => MePassaFfiError::Other { message: s },
        }
    }
}

/// FFI-safe message status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
}

impl From<InternalMessageStatus> for MessageStatus {
    fn from(status: InternalMessageStatus) -> Self {
        match status {
            InternalMessageStatus::Pending => MessageStatus::Pending,
            InternalMessageStatus::Sent => MessageStatus::Sent,
            InternalMessageStatus::Delivered => MessageStatus::Delivered,
            InternalMessageStatus::Read => MessageStatus::Read,
            InternalMessageStatus::Failed => MessageStatus::Failed,
        }
    }
}

/// FFI-safe message record
#[derive(Debug, Clone)]
pub struct FfiMessage {
    pub message_id: String,
    pub conversation_id: String,
    pub sender_peer_id: String,
    pub recipient_peer_id: Option<String>,
    pub message_type: String,
    pub content_plaintext: Option<String>,
    pub created_at: i64,
    pub sent_at: Option<i64>,
    pub received_at: Option<i64>,
    pub read_at: Option<i64>,
    pub status: MessageStatus,
    pub is_deleted: bool,
}

impl From<Message> for FfiMessage {
    fn from(msg: Message) -> Self {
        Self {
            message_id: msg.message_id,
            conversation_id: msg.conversation_id,
            sender_peer_id: msg.sender_peer_id,
            recipient_peer_id: msg.recipient_peer_id,
            message_type: msg.message_type,
            content_plaintext: msg.content_plaintext,
            created_at: msg.created_at,
            sent_at: msg.sent_at,
            received_at: msg.received_at,
            read_at: msg.read_at,
            status: msg.status.into(),
            is_deleted: msg.is_deleted,
        }
    }
}

/// FFI-safe conversation record
#[derive(Debug, Clone)]
pub struct FfiConversation {
    pub id: String,
    pub conversation_type: String,
    pub peer_id: Option<String>,
    pub display_name: Option<String>,
    pub last_message_id: Option<String>,
    pub last_message_at: Option<i64>,
    pub unread_count: i32,
    pub is_muted: bool,
    pub is_archived: bool,
    pub created_at: i64,
}

impl From<Conversation> for FfiConversation {
    fn from(conv: Conversation) -> Self {
        Self {
            id: conv.id,
            conversation_type: conv.conversation_type,
            peer_id: conv.peer_id,
            display_name: conv.display_name,
            last_message_id: conv.last_message_id,
            last_message_at: conv.last_message_at,
            unread_count: conv.unread_count,
            is_muted: conv.is_muted,
            is_archived: conv.is_archived,
            created_at: conv.created_at,
        }
    }
}
