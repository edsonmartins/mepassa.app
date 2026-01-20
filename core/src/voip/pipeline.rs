//! Audio Pipeline
//!
//! Integrates audio capture, Opus encoding, WebRTC, decoding, and playback.

use super::{
    audio::{AudioCapture, AudioConfig, AudioPlayback, Sample},
    codec::{OpusCodec, OpusConfig},
    webrtc::WebRTCPeer,
    Result, VoipError,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::TrackLocalWriter;

/// Audio pipeline for a call
///
/// Manages the full audio flow:
/// - Capture → Opus Encode → RTP Send
/// - RTP Receive → Opus Decode → Playback
pub struct AudioPipeline {
    // Components
    _audio_capture: AudioCapture,
    _audio_playback: AudioPlayback,
    codec: Arc<tokio::sync::Mutex<OpusCodec>>,

    // Tasks
    encode_task: Option<JoinHandle<()>>,
    decode_task: Option<JoinHandle<()>>,

    // Channels
    _encoded_tx: mpsc::UnboundedSender<Vec<u8>>,
    decode_tx: mpsc::UnboundedSender<Vec<u8>>,
}

impl AudioPipeline {
    /// Start the audio pipeline for a call
    ///
    /// # Arguments
    /// * `peer` - WebRTC peer connection
    ///
    /// # Returns
    /// AudioPipeline managing the audio flow
    pub async fn start(peer: Arc<WebRTCPeer>) -> Result<Self> {
        // Configuration
        let audio_config = AudioConfig::default();
        let opus_config = OpusConfig::default();

        // Create codec
        let codec = Arc::new(tokio::sync::Mutex::new(OpusCodec::with_config(opus_config)?));

        // Start audio capture
        let (audio_capture, mut sample_rx) =
            AudioCapture::start(audio_config.clone()).map_err(|e| {
                VoipError::CodecError(format!("Failed to start audio capture: {}", e))
            })?;

        // Start audio playback
        let (audio_playback, playback_tx) = AudioPlayback::start(audio_config).map_err(|e| {
            VoipError::CodecError(format!("Failed to start audio playback: {}", e))
        })?;

        // Channel for encoded packets (to be sent via RTP)
        let (encoded_tx, mut encoded_rx) = mpsc::unbounded_channel::<Vec<u8>>();

        // Channel for received packets (from RTP)
        let (decode_tx, decode_rx) = mpsc::unbounded_channel::<Vec<u8>>();

        // Get audio track from peer
        let audio_track = peer
            .audio_track()
            .ok_or_else(|| VoipError::CallSetupFailed("No audio track available".to_string()))?;

        // === ENCODE TASK: Capture → Opus → RTP ===
        let codec_enc = Arc::clone(&codec);
        let track = Arc::clone(&audio_track);
        let encode_task = tokio::spawn(async move {
            while let Some(samples) = sample_rx.recv().await {
                // Encode samples
                let mut codec = codec_enc.lock().await;
                match codec.encoder.encode(&samples) {
                    Ok(Some(packet)) => {
                        // Send encoded packet via RTP
                        if let Err(e) = send_rtp_packet(&track, &packet).await {
                            tracing::error!("❌ Failed to send RTP packet: {}", e);
                        }
                    }
                    Ok(None) => {
                        // Buffering, not enough samples yet
                    }
                    Err(e) => {
                        tracing::error!("❌ Encoding error: {}", e);
                    }
                }
            }

            tracing::info!("🛑 Encode task stopped");
        });

        // === DECODE TASK: RTP → Opus → Playback ===
        let codec_dec = Arc::clone(&codec);
        let mut decode_rx_mut = decode_rx;
        let decode_task = tokio::spawn(async move {
            while let Some(packet) = decode_rx_mut.recv().await {
                // Decode packet
                let mut codec = codec_dec.lock().await;
                match codec.decoder.decode(&packet) {
                    Ok(samples) => {
                        // Send to playback
                        if let Err(e) = playback_tx.send(samples) {
                            tracing::error!("❌ Failed to send to playback: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::error!("❌ Decoding error: {}", e);
                    }
                }
            }

            tracing::info!("🛑 Decode task stopped");
        });

        tracing::info!("✅ Audio pipeline started");

        Ok(Self {
            _audio_capture: audio_capture,
            _audio_playback: audio_playback,
            codec,
            encode_task: Some(encode_task),
            decode_task: Some(decode_task),
            _encoded_tx: encoded_tx,
            decode_tx,
        })
    }

    /// Feed received RTP packet to decoder
    ///
    /// This should be called by WebRTC's on_track callback
    pub async fn receive_packet(&mut self, packet: Vec<u8>) -> Result<()> {
        self.decode_tx
            .send(packet)
            .map_err(|e| VoipError::NetworkError(format!("Failed to send packet to decoder: {}", e)))?;
        Ok(())
    }

    /// Stop the pipeline
    pub async fn stop(mut self) {
        // Abort tasks
        if let Some(task) = self.encode_task.take() {
            task.abort();
        }
        if let Some(task) = self.decode_task.take() {
            task.abort();
        }

        tracing::info!("🛑 Audio pipeline stopped");
    }
}

/// Send encoded audio via RTP
///
/// TODO: This is a simplified placeholder. In real implementation:
/// - Use proper RTP packet construction (sequence number, timestamp, etc.)
/// - Handle packet fragmentation for large Opus packets
/// - Implement proper timing based on sample rate
async fn send_rtp_packet(
    track: &Arc<TrackLocalStaticRTP>,
    opus_packet: &[u8],
) -> Result<()> {
    // Write Opus packet to RTP track
    // WebRTC will handle RTP packetization, sequencing, and timestamps
    track
        .write(opus_packet)
        .await
        .map_err(|e| VoipError::NetworkError(format!("Failed to write RTP: {}", e)))?;

    tracing::trace!("📤 Sent RTP packet: {} bytes", opus_packet.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_components() {
        // Test that all components can be created
        let audio_config = AudioConfig::default();
        let opus_config = OpusConfig::default();

        assert_eq!(audio_config.sample_rate, 48000);
        assert_eq!(opus_config.sample_rate, 48000);
        assert_eq!(audio_config.buffer_size, opus_config.frame_size());
    }

    // Note: Full integration test would require a WebRTC peer
    // which is complex to set up in unit tests
}
