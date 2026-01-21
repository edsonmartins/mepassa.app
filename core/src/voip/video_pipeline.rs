//! Video encoding/decoding pipeline
//!
//! Handles the flow of video frames from camera → encoding → RTP transmission
//! and RTP reception → decoding → rendering.
//!
//! For MVP, actual encoding/decoding happens in platform layer (Android MediaCodec,
//! iOS VideoToolbox) for optimal hardware acceleration.

use super::video::*;
use super::Result;
use tokio::sync::mpsc;

/// Video encoder pipeline
///
/// Receives raw video frames from camera and outputs encoded RTP packets.
/// Platform-specific encoding (H.264/VP8) happens before frames enter this pipeline.
pub struct VideoEncoderPipeline {
    config: VideoConfig,
    frame_rx: mpsc::Receiver<VideoFrame>,
    rtp_tx: mpsc::Sender<Vec<u8>>,
    running: bool,
}

impl VideoEncoderPipeline {
    /// Create a new video encoder pipeline
    pub fn new(
        config: VideoConfig,
        frame_rx: mpsc::Receiver<VideoFrame>,
        rtp_tx: mpsc::Sender<Vec<u8>>,
    ) -> Self {
        Self {
            config,
            frame_rx,
            rtp_tx,
            running: false,
        }
    }

    /// Run the encoder pipeline
    ///
    /// This runs indefinitely until the frame_rx channel is closed.
    /// For MVP, assumes frames are pre-encoded by platform layer.
    pub async fn run(&mut self) -> Result<()> {
        self.running = true;

        tracing::info!(
            "📹 Video encoder pipeline started - codec: {:?}, resolution: {}, fps: {}",
            self.config.codec,
            self.config.resolution,
            self.config.fps
        );

        let mut frame_count = 0u64;

        while let Some(frame) = self.frame_rx.recv().await {
            if !self.running {
                break;
            }

            // For MVP: assume frame.data is already encoded (H.264 NALUs or VP8 frames)
            // Platform layer (Android MediaCodec / iOS VideoToolbox) does encoding

            // TODO: Add RTP packetization here for large frames
            // For now, send frame data directly
            if let Err(e) = self.rtp_tx.send(frame.data).await {
                tracing::warn!("Failed to send encoded frame to RTP: {}", e);
                break;
            }

            frame_count += 1;

            if frame_count % 100 == 0 {
                tracing::debug!("📤 Sent {} video frames", frame_count);
            }
        }

        self.running = false;

        tracing::info!(
            "📹 Video encoder pipeline stopped - total frames: {}",
            frame_count
        );

        Ok(())
    }

    /// Stop the encoder pipeline
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if pipeline is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get current configuration
    pub fn config(&self) -> &VideoConfig {
        &self.config
    }
}

/// Video decoder pipeline
///
/// Receives encoded RTP packets and outputs decoded video frames.
/// Platform-specific decoding (H.264/VP8) happens after frames exit this pipeline.
pub struct VideoDecoderPipeline {
    config: VideoConfig,
    rtp_rx: mpsc::Receiver<Vec<u8>>,
    frame_tx: mpsc::Sender<VideoFrame>,
    running: bool,
}

impl VideoDecoderPipeline {
    /// Create a new video decoder pipeline
    pub fn new(
        config: VideoConfig,
        rtp_rx: mpsc::Receiver<Vec<u8>>,
        frame_tx: mpsc::Sender<VideoFrame>,
    ) -> Self {
        Self {
            config,
            rtp_rx,
            frame_tx,
            running: false,
        }
    }

    /// Run the decoder pipeline
    ///
    /// This runs indefinitely until the rtp_rx channel is closed.
    /// For MVP, sends raw RTP data to platform layer for decoding.
    pub async fn run(&mut self) -> Result<()> {
        self.running = true;

        tracing::info!(
            "📹 Video decoder pipeline started - codec: {:?}, resolution: {}",
            self.config.codec,
            self.config.resolution
        );

        let mut packet_count = 0u64;

        while let Some(rtp_packet) = self.rtp_rx.recv().await {
            if !self.running {
                break;
            }

            // For MVP: pass RTP data to platform layer for decoding
            // Platform will decode (H.264/VP8) and render directly

            // TODO: Add RTP depacketization here
            // TODO: Handle frame assembly from multiple RTP packets
            // TODO: Handle packet loss and FEC

            // For now, create VideoFrame with raw data
            let frame = VideoFrame {
                data: rtp_packet,
                width: self.config.resolution.width,
                height: self.config.resolution.height,
                timestamp_us: 0, // TODO: extract from RTP timestamp
                format: PixelFormat::YUV420, // Assume YUV420 for H.264
            };

            if let Err(e) = self.frame_tx.send(frame).await {
                tracing::warn!("Failed to send decoded frame: {}", e);
                break;
            }

            packet_count += 1;

            if packet_count % 100 == 0 {
                tracing::debug!("📥 Received {} video packets", packet_count);
            }
        }

        self.running = false;

        tracing::info!(
            "📹 Video decoder pipeline stopped - total packets: {}",
            packet_count
        );

        Ok(())
    }

    /// Stop the decoder pipeline
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if pipeline is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get current configuration
    pub fn config(&self) -> &VideoConfig {
        &self.config
    }
}

/// Video statistics
#[derive(Debug, Clone, Default)]
pub struct VideoStats {
    /// Total frames sent
    pub frames_sent: u64,

    /// Total frames received
    pub frames_received: u64,

    /// Frames dropped due to network issues
    pub frames_dropped: u64,

    /// Current frame rate (fps)
    pub current_fps: u32,

    /// Current bitrate (kbps)
    pub current_bitrate_kbps: u32,

    /// Average encode time (ms)
    pub avg_encode_time_ms: f32,

    /// Average decode time (ms)
    pub avg_decode_time_ms: f32,
}

impl VideoStats {
    /// Create empty stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset all counters
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_stats_creation() {
        let stats = VideoStats::new();
        assert_eq!(stats.frames_sent, 0);
        assert_eq!(stats.frames_received, 0);
        assert_eq!(stats.frames_dropped, 0);
    }

    #[test]
    fn test_video_stats_reset() {
        let mut stats = VideoStats {
            frames_sent: 100,
            frames_received: 95,
            frames_dropped: 5,
            current_fps: 30,
            current_bitrate_kbps: 500,
            avg_encode_time_ms: 10.5,
            avg_decode_time_ms: 8.2,
        };

        stats.reset();

        assert_eq!(stats.frames_sent, 0);
        assert_eq!(stats.frames_received, 0);
        assert_eq!(stats.frames_dropped, 0);
        assert_eq!(stats.current_fps, 0);
    }

    #[tokio::test]
    async fn test_encoder_pipeline_creation() {
        let config = VideoConfig::default();
        let (_frame_tx, frame_rx) = mpsc::channel(10);
        let (rtp_tx, _rtp_rx) = mpsc::channel(10);

        let pipeline = VideoEncoderPipeline::new(config.clone(), frame_rx, rtp_tx);

        assert!(!pipeline.is_running());
        assert_eq!(pipeline.config().codec, config.codec);
    }

    #[tokio::test]
    async fn test_decoder_pipeline_creation() {
        let config = VideoConfig::default();
        let (_rtp_tx, rtp_rx) = mpsc::channel(10);
        let (frame_tx, _frame_rx) = mpsc::channel(10);

        let pipeline = VideoDecoderPipeline::new(config.clone(), rtp_rx, frame_tx);

        assert!(!pipeline.is_running());
        assert_eq!(pipeline.config().codec, config.codec);
    }
}
