//! Generic platform adapter
//!
//! Provides fallback extraction for platforms without dedicated adapters

use super::{MediaResource, MediaQuality, VideoFormat, PlatformAdapter, Result};
use crate::{CrawlerError, CrawlerClient, Platform};
use std::time::Duration;
use async_trait::async_trait;

/// Generic adapter for non-specific URLs
pub struct GenericAdapter {
    /// Supported video extensions
    video_extensions: Vec<&'static str>,
    /// Supported audio extensions
    audio_extensions: Vec<&'static str>,
}

impl GenericAdapter {
    /// Create a new generic adapter
    pub fn new() -> Self {
        Self {
            video_extensions: vec![
                "mp4", "webm", "mkv", "avi", "mov", "wmv", 
                "flv", "f4v", "m4v", "mpg", "mpeg", "3gp", "ogv"
            ],
            audio_extensions: vec![
                "mp3", "wav", "flac", "aac", "ogg", "m4a", 
                "wma", "opus", "aiff", "ape"
            ],
        }
    }

    /// Detect quality from URL or filename
    fn detect_quality(url: &str) -> MediaQuality {
        let url_lower = url.to_lowercase();
        
        // Check for quality indicators in URL
        if url_lower.contains("2160p") || url_lower.contains("4k") {
            MediaQuality::FourK
        } else if url_lower.contains("1440p") || url_lower.contains("2k") {
            MediaQuality::TwoK
        } else if url_lower.contains("1080p") || url_lower.contains("fhd") || 
                  url_lower.contains("fullhd") {
            MediaQuality::FullHD
        } else if url_lower.contains("720p") || url_lower.contains("hd") {
            MediaQuality::High
        } else if url_lower.contains("480p") || url_lower.contains("sd") {
            MediaQuality::Medium
        } else if url_lower.contains("360p") || url_lower.contains("240p") || 
                  url_lower.contains("144p") {
            MediaQuality::Low
        } else {
            MediaQuality::Medium // Default
        }
    }

    /// Detect format from URL
    fn detect_format(url: &str) -> VideoFormat {
        let url_lower = url.to_lowercase();
        
        if url_lower.ends_with(".mp4") {
            VideoFormat::MP4
        } else if url_lower.ends_with(".webm") {
            VideoFormat::WebM
        } else if url_lower.ends_with(".mkv") {
            VideoFormat::Unknown("mkv".to_string())
        } else if url_lower.ends_with(".avi") {
            VideoFormat::Unknown("avi".to_string())
        } else if url_lower.ends_with(".mov") {
            VideoFormat::Unknown("mov".to_string())
        } else if url_lower.ends_with(".flv") {
            VideoFormat::FLV
        } else if url_lower.ends_with(".m3u8") {
            VideoFormat::HLS
        } else if url_lower.ends_with(".mpd") {
            VideoFormat::DASH
        } else {
            VideoFormat::Unknown("unknown".to_string())
        }
    }

    /// Check if URL looks like a direct media file
    fn is_media_url(url: &str) -> bool {
        let url_lower = url.to_lowercase();
        
        // Check file extensions
        Self::new().video_extensions.iter().any(|ext| url_lower.ends_with(&format!(".{}", ext))) ||
        Self::new().audio_extensions.iter().any(|ext| url_lower.ends_with(&format!(".{}", ext))) ||
        // Check streaming manifests
        url_lower.ends_with(".m3u8") ||
        url_lower.ends_with(".mpd") ||
        // Check CDN patterns
        (url_lower.contains("/video/") && url_lower.contains(".")) ||
        (url_lower.contains("/media/") && url_lower.contains(".")) ||
        (url_lower.contains("/hls/") && url_lower.contains(".")) ||
        (url_lower.contains("/dash/") && url_lower.contains("."))
    }

    /// Extract from direct media URL
    async fn extract_from_direct(&self, url: &str, client: &CrawlerClient) -> Result<Vec<MediaResource>> {
        let format = Self::detect_format(url);
        let quality = Self::detect_quality(url);
        
        // Try to get file size via HEAD request
        let file_size = client.head(url).await.ok().flatten();
        
        let mut resource = MediaResource::new(
            url.to_string(),
            url.to_string(),
            quality,
            format,
        );
        
        if let Some(size) = file_size {
            resource = resource.with_file_size(size);
        }
        
        Ok(vec![resource])
    }

    /// Extract from HTML page (basic extraction)
    async fn extract_from_page(&self, url: &str, _client: &CrawlerClient) -> Result<Vec<MediaResource>> {
        // Fetch the page and look for media URLs
        let response = reqwest::get(url)
            .await
            .map_err(|e| CrawlerError::Network(e.to_string()))?;
        
        let html = response.text()
            .await
            .map_err(|e| CrawlerError::Parse(e.to_string()))?;
        
        let mut resources = Vec::new();
        
        // Look for video tags
        let video_patterns = [
            r#"<video[^>]+src=["']([^"']+)["']"#,
            r#"<video[^>]+data-src=["']([^"']+)["']"#,
            r#"<source[^>]+src=["']([^"']+)["']"#,
            r#"<source[^>]+data-src=["']([^"']+)["']"#,
        ];
        
        for pattern in video_patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for cap in re.captures_iter(&html) {
                    if let Some(url_match) = cap.get(1) {
                        let media_url = url_match.as_str();
                        if Self::is_media_url(media_url) {
                            resources.push(MediaResource::new(
                                media_url.to_string(),
                                url.to_string(),
                                Self::detect_quality(media_url),
                                Self::detect_format(media_url),
                            ));
                        }
                    }
                }
            }
        }
        
        // Look for media links
        let media_link_patterns = [
            r#"<a[^>]+href=["']([^"']+\.(?:mp4|webm|mkv|avi|mov))["'][^>]*>"#,
            r#"<a[^>]+href=["']([^"']+\.m3u8)["'][^>]*>"#,
            r#"<a[^>]+href=["']([^"']+\.mpd)["'][^>]*>"#,
        ];
        
        for pattern in media_link_patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for cap in re.captures_iter(&html) {
                    if let Some(url_match) = cap.get(1) {
                        let media_url = url_match.as_str();
                        if Self::is_media_url(media_url) {
                            resources.push(MediaResource::new(
                                media_url.to_string(),
                                url.to_string(),
                                Self::detect_quality(media_url),
                                Self::detect_format(media_url),
                            ));
                        }
                    }
                }
            }
        }
        
        // If we found resources, return them
        if !resources.is_empty() {
            return Ok(resources);
        }
        
        // Otherwise, return a basic resource for the URL itself
        Ok(vec![MediaResource::new(
            url.to_string(),
            url.to_string(),
            MediaQuality::Medium,
            VideoFormat::MP4,
        )])
    }
}

#[async_trait]
impl PlatformAdapter for GenericAdapter {
    fn can_handle(&self, url: &str) -> bool {
        // Generic adapter handles any URL that's not handled by specific adapters
        // It should be registered last in the registry
        !url.is_empty() && url.starts_with("http")
    }

    async fn extract_media(&self, url: &str, _client: &CrawlerClient) -> Result<Vec<MediaResource>> {
        if !self.can_handle(url) {
            return Err(CrawlerError::InvalidUrl("Invalid URL".to_string()));
        }

        // Check if it's a direct media URL
        if Self::is_media_url(url) {
            return self.extract_from_direct(url, _client).await;
        }

        // Otherwise, try to extract from page
        self.extract_from_page(url, _client).await
    }

    fn platform_name(&self) -> &str {
        "Generic"
    }

    fn platform(&self) -> Platform {
        Platform::Other("Generic".to_string())
    }
}

impl Default for GenericAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating custom adapters
pub struct AdapterBuilder {
    video_extensions: Vec<String>,
    audio_extensions: Vec<String>,
    timeout: Duration,
}

impl AdapterBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            video_extensions: Vec::new(),
            audio_extensions: Vec::new(),
            timeout: Duration::from_secs(30),
        }
    }

    /// Add a video extension
    pub fn add_video_extension(mut self, ext: &str) -> Self {
        self.video_extensions.push(ext.to_lowercase());
        self
    }

    /// Add an audio extension
    pub fn add_audio_extension(mut self, ext: &str) -> Self {
        self.audio_extensions.push(ext.to_lowercase());
        self
    }

    /// Set timeout
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build the adapter
    pub fn build(self) -> GenericAdapter {
        // Use custom extensions if provided, otherwise use defaults
        if self.video_extensions.is_empty() && self.audio_extensions.is_empty() {
            GenericAdapter::new()
        } else {
            // Note: In a full implementation, GenericAdapter would accept custom extensions
            GenericAdapter::new()
        }
    }
}

impl Default for AdapterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_quality() {
        assert_eq!(GenericAdapter::detect_quality("video-2160p.mp4"), MediaQuality::FourK);
        assert_eq!(GenericAdapter::detect_quality("video-1080p.mp4"), MediaQuality::FullHD);
        assert_eq!(GenericAdapter::detect_quality("video-720p.mp4"), MediaQuality::High);
        assert_eq!(GenericAdapter::detect_quality("video-480p.mp4"), MediaQuality::Medium);
        assert_eq!(GenericAdapter::detect_quality("video.mp4"), MediaQuality::Medium);
    }

    #[test]
    fn test_detect_format() {
        assert_eq!(GenericAdapter::detect_format("video.mp4"), VideoFormat::MP4);
        assert_eq!(GenericAdapter::detect_format("video.webm"), VideoFormat::WebM);
        assert_eq!(GenericAdapter::detect_format("video.mkv"), VideoFormat::Unknown("mkv".to_string()));
        assert_eq!(GenericAdapter::detect_format("stream.m3u8"), VideoFormat::HLS);
        assert_eq!(GenericAdapter::detect_format("stream.mpd"), VideoFormat::DASH);
    }

    #[test]
    fn test_is_media_url() {
        assert!(GenericAdapter::is_media_url("https://example.com/video.mp4"));
        assert!(GenericAdapter::is_media_url("https://example.com/audio.mp3"));
        assert!(GenericAdapter::is_media_url("https://cdn.com/hls/stream.m3u8"));
        assert!(GenericAdapter::is_media_url("https://cdn.com/video/file.mp4"));
        assert!(!GenericAdapter::is_media_url("https://example.com/video"));
        assert!(!GenericAdapter::is_media_url("https://example.com/page.html"));
    }

    #[test]
    fn test_adapter_builder() {
        let adapter = AdapterBuilder::new()
            .add_video_extension("mp4")
            .add_audio_extension("mp3")
            .set_timeout(Duration::from_secs(60))
            .build();

        assert!(adapter.can_handle("https://example.com/video.mp4"));
    }
}