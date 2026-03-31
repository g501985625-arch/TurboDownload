//! Platform adapters for video extraction
//!
//! This module provides platform-specific adapters for extracting media
//! from various video hosting platforms like YouTube, Bilibili, etc.

mod youtube;
mod bilibili;
mod generic;

pub use youtube::YouTubeAdapter;
pub use bilibili::BilibiliAdapter;
pub use generic::GenericAdapter;

use crate::{CrawlerError, Result, CrawlerClient, Platform};
use crate::extractor::Resource;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;

/// Media quality options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaQuality {
    /// Low quality (360p)
    Low,
    /// Medium quality (480p)
    Medium,
    /// High quality (720p)
    High,
    /// Full HD (1080p)
    FullHD,
    /// 2K resolution
    TwoK,
    /// 4K resolution
    FourK,
    /// Highest available
    Best,
    /// Original/source quality
    Original,
}

impl MediaQuality {
    /// Get quality label
    pub fn label(&self) -> &str {
        match self {
            MediaQuality::Low => "360p",
            MediaQuality::Medium => "480p",
            MediaQuality::High => "720p",
            MediaQuality::FullHD => "1080p",
            MediaQuality::TwoK => "1440p",
            MediaQuality::FourK => "2160p",
            MediaQuality::Best => "Best",
            MediaQuality::Original => "Original",
        }
    }
}

/// Video format
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VideoFormat {
    /// MP4 container
    MP4,
    /// WebM container
    WebM,
    /// FLV container
    FLV,
    /// HLS stream
    HLS,
    /// DASH stream
    DASH,
    /// Unknown format
    Unknown(String),
}

impl VideoFormat {
    /// Get MIME type
    pub fn mime_type(&self) -> String {
        match self {
            VideoFormat::MP4 => "video/mp4".to_string(),
            VideoFormat::WebM => "video/webm".to_string(),
            VideoFormat::FLV => "video/x-flv".to_string(),
            VideoFormat::HLS => "application/vnd.apple.mpegurl".to_string(),
            VideoFormat::DASH => "application/dash+xml".to_string(),
            VideoFormat::Unknown(ext) => format!("video/{}", ext),
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &str {
        match self {
            VideoFormat::MP4 => "mp4",
            VideoFormat::WebM => "webm",
            VideoFormat::FLV => "flv",
            VideoFormat::HLS => "m3u8",
            VideoFormat::DASH => "mpd",
            VideoFormat::Unknown(ext) => ext,
        }
    }
}

/// Extracted media resource with platform-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaResource {
    /// Direct URL to the media
    pub url: String,
    /// Resource type (re-use from extractor)
    pub resource: Resource,
    /// Video quality
    pub quality: MediaQuality,
    /// Video format
    pub format: VideoFormat,
    /// File size in bytes (if known)
    pub file_size: Option<u64>,
    /// Duration in seconds (if known)
    pub duration: Option<f64>,
    /// Bitrate in bps (if known)
    pub bitrate: Option<u64>,
    /// Resolution width (if video)
    pub width: Option<u32>,
    /// Resolution height (if video)
    pub height: Option<u32>,
    /// Codec information
    pub codec: Option<String>,
    /// Whether this is a direct download link
    pub is_direct: bool,
    /// Whether this is a streaming manifest
    pub is_streaming: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl MediaResource {
    /// Create a new media resource
    pub fn new(url: String, source_url: String, quality: MediaQuality, format: VideoFormat) -> Self {
        let is_streaming = matches!(format, VideoFormat::HLS | VideoFormat::DASH);
        let is_direct = !is_streaming;

        Self {
            url: url.clone(),
            resource: Resource::new(url, source_url),
            quality,
            format,
            file_size: None,
            duration: None,
            bitrate: None,
            width: None,
            height: None,
            codec: None,
            is_direct,
            is_streaming,
            metadata: HashMap::new(),
        }
    }

    /// Set file size
    pub fn with_file_size(mut self, size: u64) -> Self {
        self.file_size = Some(size);
        self
    }

    /// Set duration
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set resolution
    pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set codec
    pub fn with_codec(mut self, codec: String) -> Self {
        self.codec = Some(codec);
        self
    }

    /// Set bitrate
    pub fn with_bitrate(mut self, bitrate: u64) -> Self {
        self.bitrate = Some(bitrate);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get resolution string (e.g., "1920x1080")
    pub fn resolution(&self) -> Option<String> {
        self.width.and_then(|w| self.height.map(|h| format!("{}x{}", w, h)))
    }
}

/// Platform adapter trait
///
/// Implement this trait to add support for a new video platform.
#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// Check if this adapter can handle the given URL
    fn can_handle(&self, url: &str) -> bool;

    /// Extract media resources from the URL
    async fn extract_media(&self, url: &str, client: &CrawlerClient) -> Result<Vec<MediaResource>>;

    /// Get the platform name
    fn platform_name(&self) -> &str;

    /// Get the supported platform enum
    fn platform(&self) -> Platform;
}

/// Adapter registry for managing platform adapters
pub struct AdapterRegistry {
    adapters: Vec<Arc<dyn PlatformAdapter>>,
}

impl AdapterRegistry {
    /// Create a new adapter registry with default adapters
    pub fn new() -> Self {
        let mut registry = Self {
            adapters: Vec::new(),
        };
        
        // Register built-in adapters
        registry.register(Arc::new(YouTubeAdapter::new()) as Arc<dyn PlatformAdapter>);
        registry.register(Arc::new(BilibiliAdapter::new()) as Arc<dyn PlatformAdapter>);
        registry.register(Arc::new(GenericAdapter::new()) as Arc<dyn PlatformAdapter>);
        
        registry
    }

    /// Register a new adapter
    pub fn register(&mut self, adapter: Arc<dyn PlatformAdapter>) {
        self.adapters.push(adapter);
    }

    /// Find the appropriate adapter for a URL
    pub fn find_adapter(&self, url: &str) -> Option<Arc<dyn PlatformAdapter>> {
        for adapter in &self.adapters {
            if adapter.can_handle(url) {
                return Some(Arc::clone(adapter));
            }
        }
        None
    }

    /// Extract media using the appropriate adapter
    pub async fn extract(&self, url: &str, client: &CrawlerClient) -> Result<Vec<MediaResource>> {
        let adapter = self.find_adapter(url)
            .ok_or_else(|| CrawlerError::InvalidUrl(format!("No adapter found for URL: {}", url)))?;
        
        adapter.extract_media(url, client).await
    }

    /// Get all registered adapters
    pub fn adapters(&self) -> &[Arc<dyn PlatformAdapter>] {
        &self.adapters
    }

    /// Get adapter by platform name
    pub fn get_by_platform(&self, platform: &Platform) -> Option<Arc<dyn PlatformAdapter>> {
        for adapter in &self.adapters {
            if adapter.platform() == *platform {
                return Some(Arc::clone(adapter));
            }
        }
        None
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_resource_creation() {
        let resource = MediaResource::new(
            "https://example.com/video.mp4".to_string(),
            "https://youtube.com/watch?v=abc".to_string(),
            MediaQuality::High,
            VideoFormat::MP4,
        );

        assert_eq!(resource.url, "https://example.com/video.mp4");
        assert_eq!(resource.quality, MediaQuality::High);
        assert_eq!(resource.format, VideoFormat::MP4);
        assert!(resource.is_direct);
        assert!(!resource.is_streaming);
    }

    #[test]
    fn test_media_resource_resolution() {
        let resource = MediaResource::new(
            "https://example.com/video.mp4".to_string(),
            "https://youtube.com/watch?v=abc".to_string(),
            MediaQuality::FullHD,
            VideoFormat::MP4,
        ).with_resolution(1920, 1080);

        assert_eq!(resource.resolution(), Some("1920x1080".to_string()));
    }

    #[test]
    fn test_adapter_registry() {
        let registry = AdapterRegistry::new();
        
        // Should find YouTube adapter
        assert!(registry.find_adapter("https://www.youtube.com/watch?v=abc").is_some());
        
        // Should find Bilibili adapter
        assert!(registry.find_adapter("https://www.bilibili.com/video/BV1xx411c7XD").is_some());
        
        // Should fall back to generic
        assert!(registry.find_adapter("https://example.com/video.mp4").is_some());
    }
}