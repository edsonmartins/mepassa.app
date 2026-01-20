//! Audio Capture and Playback
//!
//! Handles microphone input and speaker output using cpal.

use super::{Result, VoipError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Audio sample format (f32 for compatibility)
pub type Sample = f32;

/// Audio configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: usize,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000, // Opus standard
            channels: 1,        // Mono for voice
            buffer_size: 960,   // 20ms at 48kHz
        }
    }
}

/// Audio capture from microphone
pub struct AudioCapture {
    _device: Device,
    _stream: Stream,
    sample_tx: mpsc::UnboundedSender<Vec<Sample>>,
}

impl AudioCapture {
    /// Start capturing audio from default microphone
    pub fn start(config: AudioConfig) -> Result<(Self, mpsc::UnboundedReceiver<Vec<Sample>>)> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| VoipError::CodecError("No input device available".to_string()))?;

        tracing::info!("🎤 Using input device: {:?}", device.name());

        let stream_config = StreamConfig {
            channels: config.channels,
            sample_rate: cpal::SampleRate(config.sample_rate),
            buffer_size: cpal::BufferSize::Fixed(config.buffer_size as u32),
        };

        let (sample_tx, sample_rx) = mpsc::unbounded_channel();
        let tx = sample_tx.clone();

        // Build input stream
        let stream = device
            .build_input_stream(
                &stream_config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    // Send captured samples
                    let _ = tx.send(data.to_vec());
                },
                |err| {
                    tracing::error!("❌ Audio capture error: {}", err);
                },
                None,
            )
            .map_err(|e| VoipError::CodecError(format!("Failed to build input stream: {}", e)))?;

        stream
            .play()
            .map_err(|e| VoipError::CodecError(format!("Failed to start stream: {}", e)))?;

        tracing::info!("✅ Audio capture started");

        Ok((
            Self {
                _device: device,
                _stream: stream,
                sample_tx,
            },
            sample_rx,
        ))
    }

    /// Stop capturing (automatically done on drop)
    pub fn stop(self) {
        tracing::info!("🛑 Audio capture stopped");
    }
}

/// Audio playback to speakers
pub struct AudioPlayback {
    _device: Device,
    _stream: Stream,
    sample_tx: Arc<tokio::sync::Mutex<mpsc::UnboundedSender<Vec<Sample>>>>,
}

impl AudioPlayback {
    /// Start audio playback to default speakers
    pub fn start(config: AudioConfig) -> Result<(Self, mpsc::UnboundedSender<Vec<Sample>>)> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| VoipError::CodecError("No output device available".to_string()))?;

        tracing::info!("🔊 Using output device: {:?}", device.name());

        let stream_config = StreamConfig {
            channels: config.channels,
            sample_rate: cpal::SampleRate(config.sample_rate),
            buffer_size: cpal::BufferSize::Fixed(config.buffer_size as u32),
        };

        let (sample_tx, mut sample_rx) = mpsc::unbounded_channel::<Vec<Sample>>();
        let sample_tx_clone = sample_tx.clone();

        // Buffer for accumulating samples
        let mut buffer: Vec<Sample> = Vec::new();

        // Build output stream
        let stream = device
            .build_output_stream(
                &stream_config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Try to receive new samples (non-blocking)
                    while let Ok(samples) = sample_rx.try_recv() {
                        buffer.extend_from_slice(&samples);
                    }

                    // Fill output buffer
                    let to_copy = data.len().min(buffer.len());
                    if to_copy > 0 {
                        data[..to_copy].copy_from_slice(&buffer[..to_copy]);
                        buffer.drain(..to_copy);

                        // Fill remaining with silence
                        if to_copy < data.len() {
                            data[to_copy..].fill(0.0);
                        }
                    } else {
                        // No data available, output silence
                        data.fill(0.0);
                    }
                },
                |err| {
                    tracing::error!("❌ Audio playback error: {}", err);
                },
                None,
            )
            .map_err(|e| VoipError::CodecError(format!("Failed to build output stream: {}", e)))?;

        stream
            .play()
            .map_err(|e| VoipError::CodecError(format!("Failed to start stream: {}", e)))?;

        tracing::info!("✅ Audio playback started");

        Ok((
            Self {
                _device: device,
                _stream: stream,
                sample_tx: Arc::new(tokio::sync::Mutex::new(sample_tx_clone)),
            },
            sample_tx,
        ))
    }

    /// Stop playback (automatically done on drop)
    pub fn stop(self) {
        tracing::info!("🛑 Audio playback stopped");
    }
}

/// List available audio devices
pub fn list_audio_devices() -> Result<()> {
    let host = cpal::default_host();

    tracing::info!("🎧 Available audio devices:");

    // Input devices
    tracing::info!("  Input devices:");
    if let Ok(devices) = host.input_devices() {
        for (idx, device) in devices.enumerate() {
            if let Ok(name) = device.name() {
                tracing::info!("    {}. {}", idx + 1, name);
            }
        }
    }

    // Output devices
    tracing::info!("  Output devices:");
    if let Ok(devices) = host.output_devices() {
        for (idx, device) in devices.enumerate() {
            if let Ok(name) = device.name() {
                tracing::info!("    {}. {}", idx + 1, name);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_config_default() {
        let config = AudioConfig::default();
        assert_eq!(config.sample_rate, 48000);
        assert_eq!(config.channels, 1);
        assert_eq!(config.buffer_size, 960);
    }

    #[test]
    #[ignore] // Requires audio hardware
    fn test_list_devices() {
        let result = list_audio_devices();
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires audio hardware
    async fn test_audio_capture() {
        let config = AudioConfig::default();
        let result = AudioCapture::start(config);
        assert!(result.is_ok());

        let (capture, mut rx) = result.unwrap();

        // Wait for some samples
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {},
            _ = rx.recv() => {},
        }

        capture.stop();
    }

    #[tokio::test]
    #[ignore] // Requires audio hardware
    async fn test_audio_playback() {
        let config = AudioConfig::default();
        let result = AudioPlayback::start(config);
        assert!(result.is_ok());

        let (playback, tx) = result.unwrap();

        // Send some test samples (silence)
        let samples = vec![0.0f32; 960];
        let _ = tx.send(samples);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        playback.stop();
    }
}
