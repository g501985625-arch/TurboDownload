//! Resource extractor and classifier

use serde::{Deserialize, Serialize};
use crate::Result;

/// Streaming format types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StreamFormat {
    /// HLS (HTTP Live Streaming) - .m3u8
    HLS,
    /// DASH (Dynamic Adaptive Streaming over HTTP) - .mpd
    DASH,
    /// Smooth Streaming - .ism
    SmoothStreaming,
    /// Unknown streaming format
    Unknown,
}

impl StreamFormat {
    /// Detect streaming format from URL
    pub fn from_url(url: &str) -> Self {
        let url_lower = url.to_lowercase();
        
        if url_lower.contains(".m3u8") {
            StreamFormat::HLS
        } else if url_lower.contains(".mpd") {
            StreamFormat::DASH
        } else if url_lower.contains(".ism") {
            StreamFormat::SmoothStreaming
        } else {
            StreamFormat::Unknown
        }
    }
    
    /// Get MIME type for this streaming format
    pub fn mime_type(&self) -> Option<&'static str> {
        match self {
            StreamFormat::HLS => Some("application/vnd.apple.mpegurl"),
            StreamFormat::DASH => Some("application/dash+xml"),
            StreamFormat::SmoothStreaming => Some("application/vnd.ms-sstr+xml"),
            StreamFormat::Unknown => None,
        }
    }
    
    /// Check if this is a valid streaming format
    pub fn is_valid(&self) -> bool {
        !matches!(self, StreamFormat::Unknown)
    }
}

/// Platform detection for video sites
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Platform {
    /// YouTube
    YouTube,
    /// Bilibili
    Bilibili,
    /// Vimeo
    Vimeo,
    /// Dailymotion
    Dailymotion,
    /// Twitch
    Twitch,
    /// Facebook Video
    Facebook,
    /// Instagram
    Instagram,
    /// Twitter/X Video
    Twitter,
    /// TikTok
    TikTok,
    /// Other/unknown platform
    Other(String),
}

impl Platform {
    /// Detect platform from URL
    pub fn detect(url: &str) -> Option<Self> {
        let url_lower = url.to_lowercase();
        
        // YouTube patterns - youtu.be short URLs just have video ID as path
        if url_lower.contains("youtube.com") || url_lower.contains("youtu.be") ||
           url_lower.contains("youtube-nocookie.com") {
            // Check for valid video path - youtu.be URLs just have video ID as path segment
            // e.g., youtu.be/dQw4w9WgXcQ or youtube.com/shorts/xxx or youtube.com/watch?v=xxx
            if url_lower.contains("/watch") || url_lower.contains("/v/") ||
               url_lower.contains("/embed/") || url_lower.contains("/shorts/") ||
               url_lower.contains("/live/") || 
               url_lower.contains("v=") ||
               // youtu.be format: just video ID after domain
               (url_lower.contains("youtu.be/") && url_lower.split("youtu.be/").nth(1).map_or(false, |s| !s.is_empty() && !s.contains('/'))) {
                return Some(Platform::YouTube);
            }
        }
        
        // Bilibili patterns - b23.tv is the short URL format
        if url_lower.contains("bilibili.com") || url_lower.contains("b23.tv") {
            // Check for full video URLs
            if url_lower.contains("/video/") || url_lower.contains("/av") ||
               url_lower.contains("/bv") || url_lower.contains("/play/") {
                return Some(Platform::Bilibili);
            }
            // For b23.tv short URLs, we assume they are Bilibili links
            if url_lower.contains("b23.tv/") {
                return Some(Platform::Bilibili);
            }
        }
        
        // Vimeo patterns
        if url_lower.contains("vimeo.com") {
            if url_lower.contains("/video/") || url_lower.contains("/") && 
               url_lower.split('/').filter(|s| !s.is_empty()).count() >= 2 {
                return Some(Platform::Vimeo);
            }
        }
        
        // Dailymotion patterns
        if url_lower.contains("dailymotion.com") || url_lower.contains("dai.ly") {
            return Some(Platform::Dailymotion);
        }
        
        // Twitch patterns
        if url_lower.contains("twitch.tv") {
            if url_lower.contains("/videos/") || url_lower.contains("/clips/") ||
               url_lower.contains("/live/") {
                return Some(Platform::Twitch);
            }
        }
        
        // Facebook Video patterns
        if url_lower.contains("facebook.com") && 
           (url_lower.contains("/watch/") || url_lower.contains("/video/")) {
            return Some(Platform::Facebook);
        }
        
        // Instagram patterns
        if url_lower.contains("instagram.com") && 
           (url_lower.contains("/reel/") || url_lower.contains("/tv/")) {
            return Some(Platform::Instagram);
        }
        
        // Twitter/X Video patterns
        if url_lower.contains("x.com") || url_lower.contains("twitter.com") {
            if url_lower.contains("/status/") && url_lower.contains("/video/") {
                return Some(Platform::Twitter);
            }
        }
        
        // TikTok patterns
        if url_lower.contains("tiktok.com") || url_lower.contains("tiktok.com") {
            if url_lower.contains("/video/") {
                return Some(Platform::TikTok);
            }
        }
        
        None
    }
    
    /// Get platform name as string
    pub fn name(&self) -> &str {
        match self {
            Platform::YouTube => "YouTube",
            Platform::Bilibili => "Bilibili",
            Platform::Vimeo => "Vimeo",
            Platform::Dailymotion => "Dailymotion",
            Platform::Twitch => "Twitch",
            Platform::Facebook => "Facebook",
            Platform::Instagram => "Instagram",
            Platform::Twitter => "Twitter/X",
            Platform::TikTok => "TikTok",
            Platform::Other(name) => name,
        }
    }
}

/// Resource type enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    Image,
    Video,
    Audio,
    Streaming,
    Document,
    Archive,
    Script,
    Stylesheet,
    Font,
    Html,
    Other(String),
}

impl ResourceType {
    /// Classify from file extension or URL
    pub fn from_url(url: &str) -> Self {
        let url_lower = url.to_lowercase();
        
        // Handle blob: URLs
        if url_lower.starts_with("blob:") {
            // Try to determine type from context (usually video/audio)
            return ResourceType::Video;
        }
        
        // Handle data: URLs
        if url_lower.starts_with("data:") {
            if url_lower.starts_with("data:image/") {
                return ResourceType::Image;
            }
            if url_lower.starts_with("data:video/") {
                return ResourceType::Video;
            }
            if url_lower.starts_with("data:audio/") {
                return ResourceType::Audio;
            }
            // Base64 data without MIME type - assume image (most common)
            if url_lower.contains("base64,") {
                return ResourceType::Image;
            }
        }
        
        // First check for platform-specific URLs
        if let Some(platform) = Platform::detect(url) {
            match platform {
                Platform::YouTube | Platform::Bilibili | Platform::Vimeo |
                Platform::Dailymotion | Platform::Twitch | Platform::Facebook |
                Platform::Instagram | Platform::Twitter | Platform::TikTok => {
                    return ResourceType::Video;
                }
                Platform::Other(_) => {}
            }
        }
        
        // Streaming manifests (check before video/audio for .m3u8 extension)
        if url_lower.contains(".m3u8") || url_lower.contains(".mpd") ||
           url_lower.contains(".ism") || url_lower.contains(".ismc") {
            return ResourceType::Streaming;
        }
        
        // CDN path patterns for streaming
        let cdn_patterns = [
            "/hls/", "/dash/", "/live/", "/video/", "/stream/",
            "/videos/", "/media/", "/content/", "/playback/",
            "/vod/", "/mp4/", "/webm/", "/audio/", "/music/",
        ];
        
        let is_cdn_path = cdn_patterns.iter().any(|p| url_lower.contains(p));
        
        if is_cdn_path {
            // Check if it's a known video/audio extension
            if url_lower.ends_with(".mp4") || url_lower.ends_with(".webm") ||
               url_lower.ends_with(".m3u8") || url_lower.ends_with(".mpd") ||
               url_lower.ends_with(".mov") || url_lower.ends_with(".avi") ||
               url_lower.ends_with(".mkv") || url_lower.ends_with(".flv") ||
               url_lower.ends_with(".wmv") || url_lower.ends_with(".m4v") {
                return ResourceType::Video;
            }
            if url_lower.ends_with(".mp3") || url_lower.ends_with(".m3u") ||
               url_lower.ends_with(".aac") || url_lower.ends_with(".wav") ||
               url_lower.ends_with(".ogg") || url_lower.ends_with(".flac") ||
               url_lower.ends_with(".m4a") {
                return ResourceType::Audio;
            }
        }
        
        // Images
        if url_lower.ends_with(".jpg") || url_lower.ends_with(".jpeg") ||
           url_lower.ends_with(".png") || url_lower.ends_with(".gif") ||
           url_lower.ends_with(".webp") || url_lower.ends_with(".svg") ||
           url_lower.ends_with(".bmp") || url_lower.ends_with(".ico") ||
           url_lower.ends_with(".tiff") || url_lower.ends_with(".tif") ||
           url_lower.ends_with(".avif") || url_lower.ends_with(".apng") {
            return ResourceType::Image;
        }
        
        // Videos
        if url_lower.ends_with(".mp4") || url_lower.ends_with(".webm") ||
           url_lower.ends_with(".avi") || url_lower.ends_with(".mov") ||
           url_lower.ends_with(".mkv") || url_lower.ends_with(".flv") ||
           url_lower.ends_with(".wmv") || url_lower.ends_with(".m4v") ||
           url_lower.ends_with(".3gp") || url_lower.ends_with(".ogv") {
            return ResourceType::Video;
        }
        
        // Audio
        if url_lower.ends_with(".mp3") || url_lower.ends_with(".wav") ||
           url_lower.ends_with(".ogg") || url_lower.ends_with(".flac") ||
           url_lower.ends_with(".aac") || url_lower.ends_with(".m4a") ||
           url_lower.ends_with(".wma") || url_lower.ends_with(".opus") {
            return ResourceType::Audio;
        }
        
        // Documents
        if url_lower.ends_with(".pdf") || url_lower.ends_with(".doc") ||
           url_lower.ends_with(".docx") || url_lower.ends_with(".txt") ||
           url_lower.ends_with(".md") || url_lower.ends_with(".xls") ||
           url_lower.ends_with(".xlsx") || url_lower.ends_with(".ppt") ||
           url_lower.ends_with(".pptx") || url_lower.ends_with(".odt") ||
           url_lower.ends_with(".ods") || url_lower.ends_with(".odp") ||
           url_lower.ends_with(".epub") {
            return ResourceType::Document;
        }
        
        // Archives
        if url_lower.ends_with(".zip") || url_lower.ends_with(".rar") ||
           url_lower.ends_with(".7z") || url_lower.ends_with(".tar") ||
           url_lower.ends_with(".gz") || url_lower.ends_with(".bz2") ||
           url_lower.ends_with(".xz") || url_lower.ends_with(".iso") {
            return ResourceType::Archive;
        }
        
        // Scripts
        if url_lower.ends_with(".js") || url_lower.ends_with(".jsx") ||
           url_lower.ends_with(".ts") || url_lower.ends_with(".tsx") ||
           url_lower.ends_with(".mjs") || url_lower.ends_with(".cjs") {
            return ResourceType::Script;
        }
        
        // Stylesheets
        if url_lower.ends_with(".css") || url_lower.ends_with(".scss") ||
           url_lower.ends_with(".less") || url_lower.ends_with(".sass") {
            return ResourceType::Stylesheet;
        }
        
        // Fonts
        if url_lower.ends_with(".woff") || url_lower.ends_with(".woff2") ||
           url_lower.ends_with(".ttf") || url_lower.ends_with(".otf") ||
           url_lower.ends_with(".eot") {
            return ResourceType::Font;
        }
        
        // HTML
        if url_lower.ends_with(".html") || url_lower.ends_with(".htm") ||
           url_lower.ends_with("/") || !url_lower.contains('.') {
            return ResourceType::Html;
        }
        
        ResourceType::Other(url.to_string())
    }
    
    /// Get stream format if this is a streaming resource
    pub fn stream_format(&self) -> Option<StreamFormat> {
        if matches!(self, ResourceType::Streaming) {
            None // Caller should use StreamFormat::from_url with actual URL
        } else {
            None
        }
    }
    
    /// Check if downloadable
    pub fn is_downloadable(&self) -> bool {
        matches!(
            self,
            ResourceType::Image | 
            ResourceType::Video | 
            ResourceType::Audio |
            ResourceType::Streaming |
            ResourceType::Document |
            ResourceType::Archive |
            ResourceType::Script |
            ResourceType::Stylesheet |
            ResourceType::Font
        )
    }
}

/// Extracted resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub url: String,
    pub resource_type: ResourceType,
    pub filename: Option<String>,
    pub size: Option<u64>,
    pub mime_type: Option<String>,
    pub source_url: String,
    pub downloadable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_format: Option<StreamFormat>,
}

impl Resource {
    /// Create new resource from URL
    pub fn new(url: String, source_url: String) -> Self {
        let resource_type = ResourceType::from_url(&url);
        let filename = url.split('/').next_back().map(String::from);
        let downloadable = resource_type.is_downloadable();
        let platform = Platform::detect(&url);
        
        // Get stream format for streaming resources
        let stream_format = if matches!(resource_type, ResourceType::Streaming) {
            Some(StreamFormat::from_url(&url))
        } else {
            None
        };
        
        Self {
            url,
            resource_type,
            filename,
            size: None,
            mime_type: None,
            source_url,
            downloadable,
            platform,
            stream_format,
        }
    }
    
    /// Create a resource with explicit type (for extensionless URLs)
    pub fn with_type(url: String, source_url: String, resource_type: ResourceType) -> Self {
        let filename = url.split('/').next_back().map(String::from);
        let downloadable = resource_type.is_downloadable();
        let platform = Platform::detect(&url);
        let stream_format = if matches!(resource_type, ResourceType::Streaming) {
            Some(StreamFormat::from_url(&url))
        } else {
            None
        };
        
        Self {
            url,
            resource_type,
            filename,
            size: None,
            mime_type: None,
            source_url,
            downloadable,
            platform,
            stream_format,
        }
    }
}

/// Resource extractor
pub struct ResourceExtractor {
    base_url: String,
}

impl ResourceExtractor {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
    
    /// Extract all resources from HTML
    pub fn extract(&self, html: &str) -> Result<Vec<Resource>> {
        let parser = crate::parser::HtmlParser::new(html);
        let mut resources = Vec::new();
        
        // Extract links
        for href in parser.extract_links() {
            let url = self.normalize_url(&href);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract images (standard src)
        for src in parser.extract_images() {
            let url = self.normalize_url(&src);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract lazy-loaded images
        for src in parser.extract_lazy_images() {
            let url = self.normalize_url(&src);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract background images from inline styles
        for src in parser.extract_background_images() {
            let url = self.normalize_url(&src);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract videos
        for src in parser.extract_videos() {
            let url = self.normalize_url(&src);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract audios
        for src in parser.extract_audios() {
            let url = self.normalize_url(&src);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract streaming manifests
        for src in parser.extract_streaming_manifests() {
            let url = self.normalize_url(&src);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract iframes (not downloadable but may contain resources)
        for src in parser.extract_iframes() {
            let url = self.normalize_url(&src);
            // Add iframe as non-downloadable resource (duplicates will be removed later)
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract scripts
        for src in parser.extract_scripts() {
            let url = self.normalize_url(&src);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract stylesheets
        for href in parser.extract_stylesheets() {
            let url = self.normalize_url(&href);
            resources.push(Resource::new(url, self.base_url.clone()));
        }
        
        // Extract blob URLs from JavaScript
        for src in parser.extract_blob_urls() {
            resources.push(Resource::new(src, self.base_url.clone()));
        }
        
        // Deduplicate resources
        let mut seen = std::collections::HashSet::new();
        resources.retain(|r| seen.insert(r.url.clone()));
        
        Ok(resources)
    }
    
    /// Normalize relative URLs to absolute
    fn normalize_url(&self, url: &str) -> String {
        if url.starts_with("http://") || url.starts_with("https://") {
            return url.to_string();
        }
        
        if url.starts_with("//") {
            return format!("https:{}", url);
        }
        
        if url.starts_with('/') {
            // Absolute path - need base URL
            if let Ok(base) = url::Url::parse(&self.base_url) {
                if let Some(host) = base.host_str() {
                    let scheme = base.scheme();
                    return format!("{}://{}{}", scheme, host, url);
                }
            }
        }
        
        // Relative path
        if let Ok(base) = url::Url::parse(&self.base_url) {
            if let Ok(absolute) = base.join(url) {
                return absolute.to_string();
            }
        }
        
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_format_detection() {
        assert_eq!(StreamFormat::from_url("https://example.com/video.m3u8"), StreamFormat::HLS);
        assert_eq!(StreamFormat::from_url("https://example.com/manifest.mpd"), StreamFormat::DASH);
        assert_eq!(StreamFormat::from_url("https://example.com/stream.ism"), StreamFormat::SmoothStreaming);
        assert_eq!(StreamFormat::from_url("https://example.com/video.mp4"), StreamFormat::Unknown);
    }

    #[test]
    fn test_platform_detection() {
        // YouTube
        assert_eq!(Platform::detect("https://www.youtube.com/watch?v=dQw4w9WgXcQ"), Some(Platform::YouTube));
        assert_eq!(Platform::detect("https://youtu.be/dQw4w9WgXcQ"), Some(Platform::YouTube));
        assert_eq!(Platform::detect("https://www.youtube.com/embed/dQw4w9WgXcQ"), Some(Platform::YouTube));
        
        // Bilibili
        assert_eq!(Platform::detect("https://www.bilibili.com/video/BV1xx411c7XD"), Some(Platform::Bilibili));
        assert_eq!(Platform::detect("https://b23.tv/abc123"), Some(Platform::Bilibili));
        
        // Vimeo
        assert_eq!(Platform::detect("https://vimeo.com/123456789"), Some(Platform::Vimeo));
        
        // Twitch
        assert_eq!(Platform::detect("https://www.twitch.tv/videos/123456"), Some(Platform::Twitch));
        
        // Other platforms
        assert_eq!(Platform::detect("https://www.tiktok.com/@user/video/123"), Some(Platform::TikTok));
    }

    #[test]
    fn test_resource_type_from_url() {
        // Standard types
        assert_eq!(ResourceType::from_url("https://example.com/image.png"), ResourceType::Image);
        assert_eq!(ResourceType::from_url("https://example.com/video.mp4"), ResourceType::Video);
        assert_eq!(ResourceType::from_url("https://example.com/audio.mp3"), ResourceType::Audio);
        assert_eq!(ResourceType::from_url("https://example.com/stream.m3u8"), ResourceType::Streaming);
        assert_eq!(ResourceType::from_url("https://example.com/manifest.mpd"), ResourceType::Streaming);
        
        // Streaming formats
        assert_eq!(ResourceType::from_url("https://example.com/stream.ism"), ResourceType::Streaming);
        
        // Platform detection
        assert_eq!(ResourceType::from_url("https://www.youtube.com/watch?v=abc"), ResourceType::Video);
        assert_eq!(ResourceType::from_url("https://www.bilibili.com/video/BV1xx411c7XD"), ResourceType::Video);
        
        // Blob URLs
        assert_eq!(ResourceType::from_url("blob:https://example.com/1234-5678"), ResourceType::Video);
        
        // Data URLs
        assert_eq!(ResourceType::from_url("data:image/png;base64,abc"), ResourceType::Image);
        assert_eq!(ResourceType::from_url("data:text/html,<html></html>"), ResourceType::Html);
    }

    #[test]
    fn test_resource_creation() {
        let resource = Resource::new(
            "https://example.com/video.m3u8".to_string(),
            "https://example.com/page.html".to_string()
        );
        
        assert_eq!(resource.resource_type, ResourceType::Streaming);
        assert_eq!(resource.stream_format, Some(StreamFormat::HLS));
        assert!(resource.downloadable);
    }

    #[test]
    fn test_cdn_path_detection() {
        // Test CDN path patterns in resource type
        let url = "https://cdn.example.com/hls/stream.m3u8";
        assert_eq!(ResourceType::from_url(url), ResourceType::Streaming);
        
        let url = "https://media.example.com/dash/video.mp4";
        assert_eq!(ResourceType::from_url(url), ResourceType::Video);
        
        let url = "https://static.example.com/audio/podcast.mp3";
        assert_eq!(ResourceType::from_url(url), ResourceType::Audio);
    }
}