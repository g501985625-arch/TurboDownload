//! Resource classifier module

use crate::extractor::{Resource, ResourceType, Platform};

/// CDN path patterns for video/audio content
const CDN_PATTERNS: &[&str] = &[
    "/hls/",
    "/dash/",
    "/live/",
    "/video/",
    "/stream/",
    "/videos/",
    "/media/",
    "/content/",
    "/playback/",
    "/vod/",
    "/mp4/",
    "/webm/",
    "/audio/",
    "/music/",
    "/podcast/",
];

/// Common CDN domain patterns
const CDN_DOMAINS: &[&str] = &[
    "cdn",
    "media",
    "static",
    "assets",
    "content",
    "video",
    "stream",
    "playback",
    "edge",
    "fastly",
    "cloudfront",
    "akamai",
    "cloudflare",
];

pub struct ResourceClassifier {
    allowed_extensions: Vec<String>,
    blocked_extensions: Vec<String>,
    allowed_domains: Vec<String>,
    blocked_domains: Vec<String>,
}

impl ResourceClassifier {
    pub fn new() -> Self {
        Self {
            allowed_extensions: vec![],
            blocked_extensions: vec![],
            allowed_domains: vec![],
            blocked_domains: vec![],
        }
    }

    pub fn with_allowed_extensions(mut self, extensions: Vec<String>) -> Self {
        self.allowed_extensions = extensions;
        self
    }

    pub fn with_blocked_extensions(mut self, extensions: Vec<String>) -> Self {
        self.blocked_extensions = extensions;
        self
    }

    pub fn with_allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.allowed_domains = domains;
        self
    }

    pub fn with_blocked_domains(mut self, domains: Vec<String>) -> Self {
        self.blocked_domains = domains;
        self
    }

    /// Check if URL is a streaming manifest (HLS/DASH/Smooth Streaming)
    pub fn is_streaming_manifest(url: &str) -> bool {
        let url_lower = url.to_lowercase();
        url_lower.contains(".m3u8") 
            || url_lower.contains(".mpd")
            || url_lower.contains(".ism")
            || url_lower.contains(".ismc")
            || (url_lower.contains(".m3u") && !url_lower.ends_with(".m3u8"))
    }

    /// Check if URL is a blob: or data: URL
    pub fn is_special_url(url: &str) -> bool {
        url.starts_with("blob:") || url.starts_with("data:")
    }

    /// Check if URL matches CDN path patterns
    pub fn matches_cdn_pattern(url: &str) -> bool {
        let url_lower = url.to_lowercase();
        for pattern in CDN_PATTERNS {
            if url_lower.contains(pattern) {
                return true;
            }
        }
        false
    }

    /// Check if URL is from a known CDN domain
    pub fn is_cdn_domain(url: &str) -> bool {
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                let host_lower = host.to_lowercase();
                for cdn in CDN_DOMAINS {
                    if host_lower.contains(cdn) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Smart detection for URLs without extensions
    pub fn classify_extensionless_url(url: &str) -> Option<ResourceType> {
        let url_lower = url.to_lowercase();
        
        // Check for platform-specific patterns first
        if Platform::detect(url).is_some() {
            return Some(ResourceType::Video);
        }
        
        // Check CDN patterns - this will catch /hls/, /dash/, /stream, /media etc
        if Self::matches_cdn_pattern(url) || Self::is_cdn_domain(url) {
            // Further classify based on query parameters or path
            // Check for streaming manifest patterns
            if url_lower.contains("/hls/") || url_lower.contains("/dash/") || url_lower.contains("/stream") {
                // Could be streaming or video
                if url_lower.ends_with("/stream") || url_lower.ends_with("/hls") || url_lower.ends_with("/dash") {
                    return Some(ResourceType::Streaming);
                }
            }
            if url_lower.contains("video") || url_lower.contains("play") {
                return Some(ResourceType::Video);
            }
            if url_lower.contains("audio") || url_lower.contains("music") || url_lower.contains("sound") {
                return Some(ResourceType::Audio);
            }
        }
        
        // Check for common path segments - do this BEFORE api/json checks
        // to ensure specific media paths are matched first
        if url_lower.contains("/image/") || url_lower.contains("/img/") || url_lower.contains("/photo/") {
            return Some(ResourceType::Image);
        }
        
        // Check for common video path segments
        if url_lower.contains("/clip/") || url_lower.contains("/watch/") || url_lower.contains("/embed/") {
            return Some(ResourceType::Video);
        }
        
        // Check for common URL patterns for JSON/API
        if url_lower.contains("/api/") && !url_lower.contains("/image/") {
            return Some(ResourceType::Other("json".to_string()));
        }
        
        None
    }

    /// Check if URL is likely a media file based on URL structure
    pub fn is_likely_media(url: &str) -> Option<ResourceType> {
        let url_lower = url.to_lowercase();
        
        // Check path segments for media hints - be more flexible with matching
        let media_segments = [
            "video", "audio", "music", "movie", "clip", "stream", 
            "media", "videos", "songs", "podcast", "recording"
        ];
        
        for segment in &media_segments {
            // Match segment as path component, query param, or standalone word
            if url_lower.contains(&format!("/{}/", segment)) || 
               url_lower.contains(&format!("?{}=", segment)) ||
               url_lower.contains(&format!("-{}.", segment)) ||
               url_lower.contains(&format!("_{}.", segment)) ||
               // Also match when segment is at end of URL path
               url_lower.ends_with(&format!("/{}", segment)) ||
               url_lower.contains(&format!("/{}", segment)) {
                if url_lower.contains("audio") || url_lower.contains("music") || 
                   url_lower.contains("song") || url_lower.contains("podcast") {
                    return Some(ResourceType::Audio);
                }
                return Some(ResourceType::Video);
            }
        }
        
        None
    }

    pub fn should_include(&self, resource: &Resource) -> bool {
        // Check for special URLs (blob:, data:)
        if Self::is_special_url(&resource.url) {
            // Include blob URLs for media, skip data URLs unless they're images
            if resource.url.starts_with("blob:") {
                return true;
            }
            if resource.url.starts_with("data:") {
                return matches!(resource.resource_type, ResourceType::Image);
            }
            return false;
        }

        if let Ok(url) = url::Url::parse(&resource.url) {
            if let Some(host) = url.host_str() {
                for blocked in &self.blocked_domains {
                    if host.contains(blocked) {
                        return false;
                    }
                }

                if !self.allowed_domains.is_empty() {
                    let mut is_allowed = false;
                    for domain in &self.allowed_domains {
                        if host.contains(domain) {
                            is_allowed = true;
                            break;
                        }
                    }
                    if !is_allowed {
                        return false;
                    }
                }
            }
        }

        for blocked in &self.blocked_extensions {
            if resource.url.to_lowercase().ends_with(&format!(".{}", blocked)) {
                return false;
            }
        }

        if !self.allowed_extensions.is_empty() {
            let url_lower = resource.url.to_lowercase();
            let mut is_allowed = false;
            for ext in &self.allowed_extensions {
                if url_lower.ends_with(&format!(".{}", ext)) {
                    is_allowed = true;
                    break;
                }
            }
            if !is_allowed {
                return false;
            }
        }

        true
    }

    pub fn filter(&self, resources: Vec<Resource>) -> Vec<Resource> {
        resources.into_iter()
            .filter(|r| self.should_include(r))
            .collect()
    }
}

impl Default for ResourceClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_streaming_manifest() {
        assert!(ResourceClassifier::is_streaming_manifest("https://example.com/video.m3u8"));
        assert!(ResourceClassifier::is_streaming_manifest("https://example.com/manifest.mpd"));
        assert!(ResourceClassifier::is_streaming_manifest("https://example.com/playlist.m3u"));
        assert!(ResourceClassifier::is_streaming_manifest("https://example.com/stream.ism"));
        assert!(!ResourceClassifier::is_streaming_manifest("https://example.com/video.mp4"));
    }

    #[test]
    fn test_is_special_url() {
        assert!(ResourceClassifier::is_special_url("blob:https://example.com/1234-5678"));
        assert!(ResourceClassifier::is_special_url("data:image/png;base64,..."));
        assert!(!ResourceClassifier::is_special_url("https://example.com/image.png"));
    }

    #[test]
    fn test_matches_cdn_pattern() {
        assert!(ResourceClassifier::matches_cdn_pattern("https://cdn.example.com/hls/stream.m3u8"));
        assert!(ResourceClassifier::matches_cdn_pattern("https://media.example.com/dash/manifest.mpd"));
        assert!(ResourceClassifier::matches_cdn_pattern("https://example.com/video/test.mp4"));
        assert!(!ResourceClassifier::matches_cdn_pattern("https://example.com/page.html"));
    }

    #[test]
    fn test_is_cdn_domain() {
        assert!(ResourceClassifier::is_cdn_domain("https://cdn.example.com/video.mp4"));
        assert!(ResourceClassifier::is_cdn_domain("https://media-server.example.com/stream.m3u8"));
        assert!(ResourceClassifier::is_cdn_domain("https://static-cdn.example.com/image.png"));
        assert!(!ResourceClassifier::is_cdn_domain("https://example.com/page.html"));
    }

    #[test]
    fn test_classify_extensionless_url() {
        // Platform detection
        let result = ResourceClassifier::classify_extensionless_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        );
        assert_eq!(result, Some(ResourceType::Video));
        
        let result = ResourceClassifier::classify_extensionless_url(
            "https://www.bilibili.com/video/BV1xx411c7XD"
        );
        assert_eq!(result, Some(ResourceType::Video));
        
        // CDN pattern detection
        let result = ResourceClassifier::classify_extensionless_url(
            "https://cdn.example.com/hls/stream"
        );
        assert!(result.is_some());
        
        // Path segment detection
        let result = ResourceClassifier::classify_extensionless_url(
            "https://example.com/api/image/12345"
        );
        assert_eq!(result, Some(ResourceType::Image));
    }
}