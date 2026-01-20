//! VoIP module
//!
//! WebRTC voice/video calls (P2P + TURN relay).

pub mod audio;
pub mod call;
pub mod codec;
pub mod integration;
pub mod manager;
pub mod pipeline;
pub mod signaling;
pub mod turn;
pub mod webrtc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoipError {
    #[error("Call setup failed: {0}")]
    CallSetupFailed(String),

    #[error("WebRTC error: {0}")]
    WebRtcError(String),

    #[error("Codec error: {0}")]
    CodecError(String),

    #[error("Signaling error: {0}")]
    SignalingError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

pub type Result<T> = std::result::Result<T, VoipError>;

// Re-exports for convenience
pub use audio::{AudioCapture, AudioConfig, AudioPlayback, Sample};
pub use call::{Call, CallDirection, CallEndReason, CallState, CallStats};
pub use codec::{OpusCodec, OpusConfig, OpusDecoder, OpusEncoder};
pub use integration::VoIPIntegration;
pub use manager::{CallEvent, CallManager, TurnCredentials};
pub use pipeline::AudioPipeline;
pub use signaling::{SignalingCodec, SignalingMessage};
pub use turn::TurnCredentialsClient;
pub use webrtc::{build_turn_config, WebRTCPeer};
