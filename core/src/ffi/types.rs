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
            crate::utils::error::MePassaError::NotFound(s) => {
                MePassaFfiError::Other { message: format!("Not found: {}", s) }
            }
            crate::utils::error::MePassaError::Permission(s) => {
                MePassaFfiError::Other { message: format!("Permission denied: {}", s) }
            }
            crate::utils::error::MePassaError::AlreadyExists(s) => {
                MePassaFfiError::Other { message: format!("Already exists: {}", s) }
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

// ========== VoIP Types ==========

#[cfg(feature = "voip")]
use crate::voip::{Call, CallDirection as InternalCallDirection, CallEndReason as InternalCallEndReason, CallState as InternalCallState, CallStats};

/// FFI-safe call state enum
#[cfg(feature = "voip")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiCallState {
    Initiating,
    Ringing,
    Connecting,
    Active,
    Ending,
    Ended,
}

#[cfg(feature = "voip")]
impl From<InternalCallState> for FfiCallState {
    fn from(state: InternalCallState) -> Self {
        match state {
            InternalCallState::Initiating => FfiCallState::Initiating,
            InternalCallState::Ringing => FfiCallState::Ringing,
            InternalCallState::Connecting => FfiCallState::Connecting,
            InternalCallState::Active => FfiCallState::Active,
            InternalCallState::Ending => FfiCallState::Ending,
            InternalCallState::Ended { .. } => FfiCallState::Ended,
        }
    }
}

/// FFI-safe call direction enum
#[cfg(feature = "voip")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiCallDirection {
    Outgoing,
    Incoming,
}

#[cfg(feature = "voip")]
impl From<InternalCallDirection> for FfiCallDirection {
    fn from(dir: InternalCallDirection) -> Self {
        match dir {
            InternalCallDirection::Outgoing => FfiCallDirection::Outgoing,
            InternalCallDirection::Incoming => FfiCallDirection::Incoming,
        }
    }
}

/// FFI-safe call end reason enum
#[cfg(feature = "voip")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiCallEndReason {
    Hangup,
    Rejected,
    LocalHangup,
    RemoteHangup,
    ConnectionFailed,
    Timeout,
    NetworkError,
}

#[cfg(feature = "voip")]
impl From<InternalCallEndReason> for FfiCallEndReason {
    fn from(reason: InternalCallEndReason) -> Self {
        match reason {
            InternalCallEndReason::Hangup => FfiCallEndReason::Hangup,
            InternalCallEndReason::Rejected => FfiCallEndReason::Rejected,
            InternalCallEndReason::LocalHangup => FfiCallEndReason::LocalHangup,
            InternalCallEndReason::RemoteHangup => FfiCallEndReason::RemoteHangup,
            InternalCallEndReason::ConnectionFailed => FfiCallEndReason::ConnectionFailed,
            InternalCallEndReason::Timeout => FfiCallEndReason::Timeout,
            InternalCallEndReason::NetworkError => FfiCallEndReason::NetworkError,
        }
    }
}

/// FFI-safe call record
#[cfg(feature = "voip")]
#[derive(Debug, Clone)]
pub struct FfiCall {
    pub id: String,
    pub remote_peer_id: String,
    pub direction: FfiCallDirection,
    pub state: FfiCallState,
    pub started_at: i64,
    pub connected_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub audio_muted: bool,
    pub speakerphone: bool,
    pub video_enabled: bool,
    pub video_codec: Option<FfiVideoCodec>,
}

#[cfg(feature = "voip")]
impl From<Call> for FfiCall {
    fn from(call: Call) -> Self {
        Self {
            id: call.id,
            remote_peer_id: call.remote_peer_id,
            direction: call.direction.into(),
            state: call.state.into(),
            started_at: call.started_at.timestamp(),
            connected_at: call.connected_at.map(|t| t.timestamp()),
            ended_at: call.ended_at.map(|t| t.timestamp()),
            audio_muted: call.audio_muted,
            speakerphone: call.speakerphone,
            video_enabled: false, // TODO: get from CallManager
            video_codec: None,    // TODO: get from CallManager
        }
    }
}

/// FFI-safe call statistics
#[cfg(feature = "voip")]
#[derive(Debug, Clone)]
pub struct FfiCallStats {
    pub avg_rtt_ms: u32,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub packets_lost: u64,
    pub jitter_ms: u32,
    pub audio_bitrate_kbps: u32,
}

#[cfg(feature = "voip")]
impl From<CallStats> for FfiCallStats {
    fn from(stats: CallStats) -> Self {
        Self {
            avg_rtt_ms: stats.avg_rtt_ms,
            packets_sent: stats.packets_sent,
            packets_received: stats.packets_received,
            packets_lost: stats.packets_lost,
            jitter_ms: stats.jitter_ms,
            audio_bitrate_kbps: stats.audio_bitrate_kbps,
        }
    }
}

// ========== Video Types (FASE 14) ==========

#[cfg(feature = "voip")]
use crate::voip::VideoCodec as InternalVideoCodec;

/// FFI-safe video codec enum
#[cfg(feature = "voip")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiVideoCodec {
    H264,
    VP8,
    VP9,
}

#[cfg(feature = "voip")]
impl From<InternalVideoCodec> for FfiVideoCodec {
    fn from(codec: InternalVideoCodec) -> Self {
        match codec {
            InternalVideoCodec::H264 => FfiVideoCodec::H264,
            InternalVideoCodec::VP8 => FfiVideoCodec::VP8,
            InternalVideoCodec::VP9 => FfiVideoCodec::VP9,
        }
    }
}

#[cfg(feature = "voip")]
impl From<FfiVideoCodec> for InternalVideoCodec {
    fn from(codec: FfiVideoCodec) -> Self {
        match codec {
            FfiVideoCodec::H264 => InternalVideoCodec::H264,
            FfiVideoCodec::VP8 => InternalVideoCodec::VP8,
            FfiVideoCodec::VP9 => InternalVideoCodec::VP9,
        }
    }
}

/// FFI-safe video resolution
#[cfg(feature = "voip")]
#[derive(Debug, Clone, Copy)]
pub struct FfiVideoResolution {
    pub width: u32,
    pub height: u32,
}

/// FFI-safe camera position enum
#[cfg(feature = "voip")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiCameraPosition {
    Front,
    Back,
    External,
}

/// FFI-safe video statistics
#[cfg(feature = "voip")]
#[derive(Debug, Clone)]
pub struct FfiVideoStats {
    pub resolution: FfiVideoResolution,
    pub fps: u32,
    pub bitrate_kbps: u32,
    pub frames_sent: u64,
    pub frames_received: u64,
    pub frames_dropped: u64,
}
