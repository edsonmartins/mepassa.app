//! Video encoding/decoding pipeline
//!
//! Handles the flow of video frames from camera → encoding → RTP transmission
//! and RTP reception → decoding → rendering.
//!
//! For MVP, actual encoding/decoding happens in platform layer (Android MediaCodec,
//! iOS VideoToolbox) for optimal hardware acceleration.

use super::rtp_video::{RtpDepacketizer, RtpPacket, RtpPacketizer};
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
    packetizer: RtpPacketizer,
    running: bool,
}

impl VideoEncoderPipeline {
    /// Create a new video encoder pipeline
    pub fn new(
        config: VideoConfig,
        frame_rx: mpsc::Receiver<VideoFrame>,
        rtp_tx: mpsc::Sender<Vec<u8>>,
    ) -> Self {
        // Generate random SSRC for this stream
        let ssrc = rand::random();
        let packetizer = RtpPacketizer::new(ssrc, config.codec);

        Self {
            config,
            frame_rx,
            rtp_tx,
            packetizer,
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
        let mut packet_count = 0u64;

        while let Some(frame) = self.frame_rx.recv().await {
            if !self.running {
                break;
            }

            // For MVP: assume frame.data is already encoded (H.264 NALUs or VP8 frames)
            // Platform layer (Android MediaCodec / iOS VideoToolbox) does encoding

            // Convert timestamp from microseconds to RTP timestamp (90kHz clock)
            let rtp_timestamp = ((frame.timestamp_us * 90) / 1000) as u32;

            // Packetize frame into RTP packets
            let rtp_packets = self.packetizer.packetize(&frame.data, rtp_timestamp);

            tracing::debug!(
                "📦 Frame {} ({} bytes) packetized into {} RTP packets",
                frame_count,
                frame.data.len(),
                rtp_packets.len()
            );

            // Send all RTP packets for this frame
            for packet in rtp_packets {
                let packet_bytes = packet.to_bytes();
                if let Err(e) = self.rtp_tx.send(packet_bytes).await {
                    tracing::warn!("Failed to send RTP packet: {}", e);
                    break;
                }
                packet_count += 1;
            }

            frame_count += 1;

            if frame_count % 100 == 0 {
                tracing::debug!(
                    "📤 Sent {} video frames ({} RTP packets)",
                    frame_count,
                    packet_count
                );
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
    depacketizer: RtpDepacketizer,
    running: bool,
}

impl VideoDecoderPipeline {
    /// Create a new video decoder pipeline
    pub fn new(
        config: VideoConfig,
        rtp_rx: mpsc::Receiver<Vec<u8>>,
        frame_tx: mpsc::Sender<VideoFrame>,
    ) -> Self {
        let depacketizer = RtpDepacketizer::new(config.codec);

        Self {
            config,
            rtp_rx,
            frame_tx,
            depacketizer,
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
        let mut frame_count = 0u64;

        while let Some(rtp_packet_bytes) = self.rtp_rx.recv().await {
            if !self.running {
                break;
            }

            packet_count += 1;

            // Parse RTP packet from bytes
            let rtp_packet = match RtpPacket::from_bytes(&rtp_packet_bytes) {
                Ok(packet) => packet,
                Err(e) => {
                    tracing::warn!("Failed to parse RTP packet: {}", e);
                    continue;
                }
            };

            // Depacketize - may return None if frame is not yet complete
            match self.depacketizer.depacketize(&rtp_packet) {
                Ok(Some(frame_data)) => {
                    // Complete frame assembled!
                    frame_count += 1;

                    // Convert RTP timestamp (90kHz) back to microseconds
                    let timestamp_us = (rtp_packet.header.timestamp as i64 * 1000) / 90;

                    let frame = VideoFrame {
                        data: frame_data,
                        width: self.config.resolution.width,
                        height: self.config.resolution.height,
                        timestamp_us,
                        format: PixelFormat::YUV420, // Assume YUV420 for H.264
                    };

                    tracing::debug!(
                        "📦 Frame {} reassembled ({} bytes) from RTP packets",
                        frame_count,
                        frame.data.len()
                    );

                    if let Err(e) = self.frame_tx.send(frame).await {
                        tracing::warn!("Failed to send decoded frame: {}", e);
                        break;
                    }
                }
                Ok(None) => {
                    // Frame not yet complete, waiting for more packets
                }
                Err(e) => {
                    tracing::warn!("Failed to depacketize RTP packet: {}", e);
                    // Continue processing next packet
                }
            }

            if packet_count % 100 == 0 {
                tracing::debug!(
                    "📥 Received {} RTP packets ({} complete frames)",
                    packet_count,
                    frame_count
                );
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
