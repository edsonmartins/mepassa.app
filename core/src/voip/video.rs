//! Video capture and codec support
//!
//! Provides video codec definitions, configuration, and platform-agnostic
//! camera capture trait for WebRTC video calls.

use super::Result;

/// Video codecs supported by MePassa
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodec {
    /// H.264 (AVC) - Primary codec with hardware acceleration
    /// Supported by MediaCodec (Android) and VideoToolbox (iOS)
    H264,

    /// VP8 - Fallback software codec
    /// Mandatory in WebRTC spec, royalty-free
    VP8,

    /// VP9 - Future codec with better compression
    /// Growing hardware support
    VP9,
}

impl VideoCodec {
    /// Get MIME type string for SDP negotiation
    pub fn mime_type(&self) -> &'static str {
        match self {
            VideoCodec::H264 => "video/H264",
            VideoCodec::VP8 => "video/VP8",
            VideoCodec::VP9 => "video/VP9",
        }
    }

    /// Get SDP fmtp line for codec parameters
    pub fn fmtp_line(&self) -> String {
        match self {
            // H.264 Baseline Profile Level 3.1, packetization-mode 1
            VideoCodec::H264 => "profile-level-id=42e01f;packetization-mode=1".to_string(),
            VideoCodec::VP8 | VideoCodec::VP9 => String::new(),
        }
    }

    /// Get clock rate for RTP (standard 90kHz for video)
    pub const fn clock_rate(&self) -> u32 {
        90000
    }
}

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoResolution {
    pub width: u32,
    pub height: u32,
}

impl VideoResolution {
    /// VGA 640x480 - Default for video calls
    pub const VGA: Self = Self {
        width: 640,
        height: 480,
    };

    /// HD 1280x720 - High quality
    pub const HD: Self = Self {
        width: 1280,
        height: 720,
    };

    /// Full HD 1920x1080 - Premium quality
    pub const FHD: Self = Self {
        width: 1920,
        height: 1080,
    };

    /// QVGA 320x240 - Low bandwidth
    pub const QVGA: Self = Self {
        width: 320,
        height: 240,
    };
}

impl std::fmt::Display for VideoResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// Video configuration
#[derive(Debug, Clone)]
pub struct VideoConfig {
    /// Codec to use
    pub codec: VideoCodec,

    /// Resolution
    pub resolution: VideoResolution,

    /// Frame rate (fps)
    pub fps: u32,

    /// Target bitrate in kbps
    pub bitrate_kbps: u32,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            codec: VideoCodec::H264,
            resolution: VideoResolution::VGA,
            fps: 24,
            bitrate_kbps: 500,
        }
    }
}

impl VideoConfig {
    /// Create config optimized for low bandwidth (3G)
    pub fn low_bandwidth() -> Self {
        Self {
            codec: VideoCodec::H264,
            resolution: VideoResolution::QVGA,
            fps: 15,
            bitrate_kbps: 200,
        }
    }

    /// Create config optimized for high quality (WiFi)
    pub fn high_quality() -> Self {
        Self {
            codec: VideoCodec::H264,
            resolution: VideoResolution::HD,
            fps: 30,
            bitrate_kbps: 1500,
        }
    }
}

/// Video frame data
pub struct VideoFrame {
    /// Raw frame data (YUV or RGB)
    pub data: Vec<u8>,

    /// Frame width in pixels
    pub width: u32,

    /// Frame height in pixels
    pub height: u32,

    /// Timestamp in microseconds
    pub timestamp_us: i64,

    /// Pixel format
    pub format: PixelFormat,
}

/// Pixel formats for video frames
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// YUV 4:2:0 planar (most common)
    YUV420,

    /// NV21 (Android camera format)
    NV21,

    /// RGB 24-bit
    RGB24,

    /// RGBA 32-bit
    RGBA,
}

impl PixelFormat {
    /// Get bytes per pixel
    pub const fn bytes_per_pixel(&self) -> f32 {
        match self {
            PixelFormat::YUV420 | PixelFormat::NV21 => 1.5, // 12 bits per pixel
            PixelFormat::RGB24 => 3.0,
            PixelFormat::RGBA => 4.0,
        }
    }
}

/// Platform-agnostic camera capture trait
///
/// Implemented by platform-specific camera managers:
/// - Android: CameraX via JNI
/// - iOS: AVCaptureSession via FFI
/// - Desktop: webrtc native camera or gstreamer
pub trait VideoCapture: Send + Sync {
    /// Start camera capture
    fn start(&mut self, config: VideoConfig) -> Result<()>;

    /// Stop camera capture
    fn stop(&mut self) -> Result<()>;

    /// Get next video frame (blocking until available)
    fn next_frame(&mut self) -> Result<VideoFrame>;

    /// Switch camera (front/back) - mobile only
    fn switch_camera(&mut self) -> Result<()>;

    /// Get available cameras
    fn list_cameras(&self) -> Result<Vec<CameraInfo>>;

    /// Check if camera is currently running
    fn is_running(&self) -> bool;
}

/// Camera information
#[derive(Debug, Clone)]
pub struct CameraInfo {
    /// Platform-specific camera ID
    pub id: String,

    /// Human-readable camera name
    pub name: String,

    /// Camera position
    pub position: CameraPosition,
}

/// Camera position (mobile devices)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraPosition {
    /// Front-facing camera (selfie)
    Front,

    /// Back-facing camera (environment)
    Back,

    /// External camera (USB webcam, etc.)
    External,
}

impl std::fmt::Display for CameraPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraPosition::Front => write!(f, "Front"),
            CameraPosition::Back => write!(f, "Back"),
            CameraPosition::External => write!(f, "External"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_codec_mime_types() {
        assert_eq!(VideoCodec::H264.mime_type(), "video/H264");
        assert_eq!(VideoCodec::VP8.mime_type(), "video/VP8");
        assert_eq!(VideoCodec::VP9.mime_type(), "video/VP9");
    }

    #[test]
    fn test_video_codec_clock_rate() {
        assert_eq!(VideoCodec::H264.clock_rate(), 90000);
        assert_eq!(VideoCodec::VP8.clock_rate(), 90000);
        assert_eq!(VideoCodec::VP9.clock_rate(), 90000);
    }

    #[test]
    fn test_video_config_default() {
        let config = VideoConfig::default();
        assert_eq!(config.codec, VideoCodec::H264);
        assert_eq!(config.resolution.width, 640);
        assert_eq!(config.resolution.height, 480);
        assert_eq!(config.fps, 24);
        assert_eq!(config.bitrate_kbps, 500);
    }

    #[test]
    fn test_video_config_presets() {
        let low = VideoConfig::low_bandwidth();
        assert_eq!(low.resolution.width, 320);
        assert_eq!(low.fps, 15);
        assert_eq!(low.bitrate_kbps, 200);

        let high = VideoConfig::high_quality();
        assert_eq!(high.resolution.width, 1280);
        assert_eq!(high.fps, 30);
        assert_eq!(high.bitrate_kbps, 1500);
    }

    #[test]
    fn test_resolution_display() {
        assert_eq!(VideoResolution::VGA.to_string(), "640x480");
        assert_eq!(VideoResolution::HD.to_string(), "1280x720");
    }

    #[test]
    fn test_pixel_format_bytes_per_pixel() {
        assert_eq!(PixelFormat::YUV420.bytes_per_pixel(), 1.5);
        assert_eq!(PixelFormat::RGB24.bytes_per_pixel(), 3.0);
        assert_eq!(PixelFormat::RGBA.bytes_per_pixel(), 4.0);
    }
}
