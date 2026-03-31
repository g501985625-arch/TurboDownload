//! YouTube platform adapter
//!
//! Provides media extraction from YouTube using yt-dlp or innertube API

use super::{MediaResource, MediaQuality, VideoFormat, PlatformAdapter, Result};
use crate::{CrawlerError, CrawlerClient, Platform};
use regex::Regex;
use std::sync::OnceLock;
use async_trait::async_trait;

/// YouTube video ID extraction regex
static VIDEO_ID_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_video_id_regex() -> &'static Regex {
    VIDEO_ID_REGEX.get_or_init(|| {
        Regex::new(r"(?:v=|/v/|/embed/|/shorts/|youtu\.be/)([a-zA-Z0-9_-]{11})").unwrap()
    })
}

/// YouTube adapter for extracting video information
pub struct YouTubeAdapter {
    /// Use yt-dlp CLI for extraction (requires yt-dlp installed)
    use_ytdlp: bool,
}

impl YouTubeAdapter {
    /// Create a new YouTube adapter
    pub fn new() -> Self {
        Self {
            use_ytdlp: false, // Default to innertube API
        }
    }

    /// Create with yt-dlp enabled
    pub fn with_ytdlp() -> Self {
        Self {
            use_ytdlp: true,
        }
    }

    /// Extract video ID from YouTube URL
    pub fn extract_video_id(url: &str) -> Option<String> {
        // Check for youtu.be short URLs (case-insensitive check, but preserve original case)
        if url.to_lowercase().contains("youtu.be/") {
            if let Some(pos) = url.find("youtu.be/") {
                let after = &url[pos + 9..]; // length of "youtu.be/"
                let id_part = after.split(|c| c == '?' || c == '&').next()?;
                if id_part.len() == 11 {
                    return Some(id_part.to_string());
                }
            }
        }
        
        // Check for standard YouTube URLs
        get_video_id_regex()
            .captures(url)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Parse resolution from quality label
    fn parse_resolution(quality: &str) -> Option<(u32, u32)> {
        match quality {
            "144p" => Some((256, 144)),
            "240p" => Some((426, 240)),
            "360p" => Some((640, 360)),
            "480p" => Some((854, 480)),
            "720p" | "720p60" => Some((1280, 720)),
            "1080p" | "1080p60" => Some((1920, 1080)),
            "1440p" | "1440p60" => Some((2560, 1440)),
            "2160p" | "2160p60" => Some((3840, 2160)),
            "4320p" => Some((7680, 4320)),
            _ => None,
        }
    }

    /// Map itag to quality
    fn itag_to_quality(itag: u32) -> MediaQuality {
        match itag {
            // Video only
            160 => MediaQuality::Low,        // 144p
            133 => MediaQuality::Low,        // 240p
            134 => MediaQuality::Medium,     // 360p
            135 => MediaQuality::Medium,     // 480p
            136 => MediaQuality::High,       // 720p
            137 => MediaQuality::FullHD,     // 1080p
            264 => MediaQuality::TwoK,       // 1440p
            271 => MediaQuality::FourK,      // 2160p
            313 => MediaQuality::FourK,      // 2160p
            // Adaptive formats with higher bitrate
            298 => MediaQuality::High,       // 720p60
            299 => MediaQuality::FullHD,     // 1080p60
            305 => MediaQuality::FullHD,     // 1080p60 HDR
            336 => MediaQuality::TwoK,       // 1440p60
            315 => MediaQuality::FourK,      // 2160p60
            _ => MediaQuality::Medium,
        }
    }

    /// Map itag to format
    fn itag_to_format(itag: u32) -> VideoFormat {
        match itag {
            // MP4 formats
            18 | 22 | 37 | 38 | 82 | 83 | 84 | 85 | 133 | 134 | 135 | 136 | 137 | 
            264 | 266 | 298 | 299 | 305 | 315 | 336 => VideoFormat::MP4,
            // WebM formats
            43 | 44 | 45 | 46 | 167 | 168 | 169 | 170 | 218 | 219 | 220 | 221 | 
            247 | 248 | 278 | 303 | 306 => VideoFormat::WebM,
            // 3GP (legacy)
            36 | 17 => VideoFormat::Unknown("3gp".to_string()),
            // HLS (live)
            96 | 95 | 94 | 93 | 92 | 91 => VideoFormat::HLS,
            _ => VideoFormat::MP4, // Default to MP4
        }
    }

    /// Extract video info using simple HTTP requests (no yt-dlp)
    async fn extract_with_api(&self, url: &str, _client: &CrawlerClient) -> Result<Vec<MediaResource>> {
        let video_id = Self::extract_video_id(url)
            .ok_or_else(|| CrawlerError::Parse("Invalid YouTube URL".to_string()))?;

        // Use oembed API to get basic video info
        let oembed_url = format!(
            "https://www.youtube.com/oembed?url=https://www.youtube.com/watch?v={}&format=json",
            video_id
        );

        // Try to get video info, but don't fail if unavailable
        let _oembed_response = reqwest::get(&oembed_url)
            .await
            .ok();

        // For now, return a placeholder that indicates we need yt-dlp for full extraction
        // In a production implementation, you would:
        // 1. Use yt-dlp --dump-json
        // 2. Or use innertube API with proper signature decryption
        // 3. Or use a library like youtube-dl-api or ytfzf

        let mut resources = Vec::new();

        // Add a note that full extraction requires yt-dlp
        resources.push(MediaResource::new(
            format!("ytvideo://{}", video_id),
            url.to_string(),
            MediaQuality::Best,
            VideoFormat::MP4,
        ).with_metadata(
            "platform".to_string(),
            "youtube".to_string(),
        ).with_metadata(
            "video_id".to_string(),
            video_id,
        ).with_metadata(
            "note".to_string(),
            "Install yt-dlp for full quality extraction".to_string(),
        ));

        Ok(resources)
    }

    /// Extract video info using yt-dlp
    #[cfg(feature = "ytdlp")]
    async fn extract_with_ytdlp(&self, url: &str) -> Result<Vec<MediaResource>> {
        use std::process::Stdio;
        use tokio::process::Command;

        let output = Command::new("yt-dlp")
            .args([
                "--dump-json",
                "--no-download",
                "--no-playlist",
                url,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| CrawlerError::Internal(format!("Failed to run yt-dlp: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CrawlerError::Internal(format!("yt-dlp failed: {}", stderr)));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| CrawlerError::Parse(format!("Failed to parse yt-dlp output: {}", e)))?;

        let mut resources = Vec::new();
        let video_id = json["id"].as_str().unwrap_or("");
        let title = json["title"].as_str().unwrap_or("Unknown");

        // Extract formats
        if let Some(formats) = json["formats"].as_array() {
            for fmt in formats {
                let url = fmt["url"].as_str().unwrap_or("");
                if url.is_empty() {
                    continue;
                }

                let itag = fmt["format_id"].as_str()
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0);

                let quality_label = fmt["format_note"].as_str()
                    .or_else(|| fmt["quality"].as_str())
                    .unwrap_or("unknown");

                let (width, height) = (
                    fmt["width"].as_u64().map(|w| w as u32),
                    fmt["height"].as_u64().map(|h| h as u32),
                );

                let filesize = fmt["filesize"].as_u64()
                    .or_else(|| fmt["filesize_approx"].as_u64());

                let mut resource = MediaResource::new(
                    url.to_string(),
                    url.to_string(),
                    Self::itag_to_quality(itag),
                    Self::itag_to_format(itag),
                );

                if let (Some(w), Some(h)) = (width, height) {
                    resource = resource.with_resolution(w, h);
                }

                if let Some(size) = filesize {
                    resource = resource.with_file_size(size);
                }

                resource = resource
                    .with_codec(fmt["vcodec"].as_str().map(String::from))
                    .with_metadata("title".to_string(), title.to_string())
                    .with_metadata("video_id".to_string(), video_id.to_string());

                resources.push(resource);
            }
        }

        // Sort by quality (best first)
        resources.sort_by(|a, b| {
            let a_pri = match a.quality {
                MediaQuality::FourK => 7,
                MediaQuality::TwoK => 6,
                MediaQuality::FullHD => 5,
                MediaQuality::High => 4,
                MediaQuality::Medium => 3,
                MediaQuality::Low => 2,
                _ => 1,
            };
            let b_pri = match b.quality {
                MediaQuality::FourK => 7,
                MediaQuality::TwoK => 6,
                MediaQuality::FullHD => 5,
                MediaQuality::High => 4,
                MediaQuality::Medium => 3,
                MediaQuality::Low => 2,
                _ => 1,
            };
            b_pri.cmp(&a_pri)
        });

        Ok(resources)
    }

    #[cfg(not(feature = "ytdlp"))]
    async fn extract_with_ytdlp(&self, _url: &str) -> Result<Vec<MediaResource>> {
        Err(CrawlerError::Internal(
            "yt-dlp feature not enabled. Rebuild with --features ytdlp".to_string()
        ))
    }
}

#[async_trait]
impl PlatformAdapter for YouTubeAdapter {
    fn can_handle(&self, url: &str) -> bool {
        let url_lower = url.to_lowercase();
        
        // Check for various YouTube URL patterns
        url_lower.contains("youtube.com") ||
        url_lower.contains("youtu.be") ||
        url_lower.contains("youtube-nocookie.com")
    }

    async fn extract_media(&self, url: &str, client: &CrawlerClient) -> Result<Vec<MediaResource>> {
        // Validate URL first
        if !self.can_handle(url) {
            return Err(CrawlerError::InvalidUrl("Not a YouTube URL".to_string()));
        }

        // Use yt-dlp if available, otherwise fall back to API
        if self.use_ytdlp {
            self.extract_with_ytdlp(url).await
        } else {
            self.extract_with_api(url, client).await
        }
    }

    fn platform_name(&self) -> &str {
        "YouTube"
    }

    fn platform(&self) -> Platform {
        Platform::YouTube
    }
}

impl Default for YouTubeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_video_id() {
        // Standard watch URL
        assert_eq!(
            YouTubeAdapter::extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );

        // Short URL
        assert_eq!(
            YouTubeAdapter::extract_video_id("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );

        // Embed URL
        assert_eq!(
            YouTubeAdapter::extract_video_id("https://www.youtube.com/embed/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );

        // Shorts URL
        assert_eq!(
            YouTubeAdapter::extract_video_id("https://www.youtube.com/shorts/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );

        // With additional params
        assert_eq!(
            YouTubeAdapter::extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PL123"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn test_can_handle() {
        let adapter = YouTubeAdapter::new();
        
        assert!(adapter.can_handle("https://www.youtube.com/watch?v=abc"));
        assert!(adapter.can_handle("https://youtu.be/abc"));
        assert!(adapter.can_handle("https://www.youtube.com/embed/abc"));
        assert!(!adapter.can_handle("https://bilibili.com/video/abc"));
    }

    #[test]
    fn test_itag_to_quality() {
        assert_eq!(YouTubeAdapter::itag_to_quality(137), MediaQuality::FullHD);
        assert_eq!(YouTubeAdapter::itag_to_quality(136), MediaQuality::High);
        assert_eq!(YouTubeAdapter::itag_to_quality(135), MediaQuality::Medium);
    }

    #[test]
    fn test_resolution_parsing() {
        assert_eq!(YouTubeAdapter::parse_resolution("1080p"), Some((1920, 1080)));
        assert_eq!(YouTubeAdapter::parse_resolution("720p"), Some((1280, 720)));
        assert_eq!(YouTubeAdapter::parse_resolution("480p"), Some((854, 480)));
    }
}