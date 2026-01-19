//! VoIP module
//!
//! WebRTC voice/video calls (P2P + TURN relay).

// pub mod webrtc;
// pub mod signaling;
// pub mod codec;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoipError {
    #[error("Call setup failed: {0}")]
    CallSetupFailed(String),

    #[error("WebRTC error: {0}")]
    WebRtcError(String),

    #[error("Codec error: {0}")]
    CodecError(String),
}

pub type Result<T> = std::result::Result<T, VoipError>;
