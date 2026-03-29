//! URL Extractor
//! 
//! Utilities for URL parsing and manipulation

use regex::Regex;
use url::Url;
use once_cell::sync::Lazy;

use crate::models::{ResourceType, Result, AppError};

/// Regex for matching file extensions
static EXTENSION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\.([a-zA-Z0-9]+)(?:\?|#|$)").expect("Invalid extension regex")
});

/// Regex for matching common media patterns
static _MEDIA_PATTERN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(/cdn/|/media/|/assets/|/files/|/download/|/uploads/)")
        .expect("Invalid media pattern regex")
});

/// URL encoding/decoding utilities
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }

    #[allow(dead_code)]
    pub fn decode(s: &str) -> String {
        url::form_urlencoded::parse(s.as_bytes())
            .map(|(k, _)| k.to_string())
            .next()
            .unwrap_or_default()
    }
}

/// URL extractor and parser
pub struct UrlExtractor;

impl UrlExtractor {
    /// Create a new URL extractor
    pub fn new() -> Self {
        Self
    }

    /// Parse a URL string
    pub fn parse_url(url: &str) -> Result<Url> {
        Url::parse(url).map_err(|e| AppError::InvalidUrl(format!("Invalid URL {}: {}", url, e)))
    }

    /// Get file extension from URL
    pub fn get_extension(url: &str) -> Option<String> {
        let parsed = Url::parse(url).ok()?;
        let path = parsed.path();
        
        // Try to extract extension from path
        if let Some(captures) = EXTENSION_REGEX.captures(path) {
            return captures.get(1).map(|m: regex::Match| m.as_str().to_lowercase());
        }
        
        // Try query parameters
        for (key, value) in parsed.query_pairs() {
            if key == "format" || key == "ext" || key == "type" {
                return Some(value.to_lowercase());
            }
        }

        None
    }

    /// Detect resource type from URL
    pub fn detect_resource_type(url: &str) -> ResourceType {
        let extension = Self::get_extension(url);
        let url_lower = url.to_lowercase();

        // Check by extension first
        if let Some(ext) = extension {
            match ext.as_str() {
                // Images
                "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "bmp" | "ico" | "tiff" | "tif" => {
                    return ResourceType::Image;
                }
                // Videos
                "mp4" | "webm" | "avi" | "mkv" | "mov" | "flv" | "wmv" | "m4v" | "3gp" => {
                    return ResourceType::Video;
                }
                // Audio
                "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" | "aiff" => {
                    return ResourceType::Audio;
                }
                // Documents
                "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" | "odt" => {
                    return ResourceType::Document;
                }
                // Archives
                "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => {
                    return ResourceType::Archive;
                }
                // Executables
                "exe" | "dmg" | "pkg" | "deb" | "rpm" | "msi" | "app" => {
                    return ResourceType::Executable;
                }
                _ => {}
            }
        }

        // Check URL patterns
        if url_lower.contains("/image") || url_lower.contains("/img") || url_lower.contains("/photo") {
            return ResourceType::Image;
        }
        if url_lower.contains("/video") || url_lower.contains("/movie") {
            return ResourceType::Video;
        }
        if url_lower.contains("/audio") || url_lower.contains("/music") || url_lower.contains("/sound") {
            return ResourceType::Audio;
        }
        if url_lower.contains("/document") || url_lower.contains("/doc") || url_lower.contains("/file") {
            return ResourceType::Document;
        }
        if url_lower.contains("/download") || url_lower.contains("/cdn") || url_lower.contains("/assets") {
            return ResourceType::Other;
        }

        ResourceType::Other
    }

    /// Check if URL is likely a downloadable resource
    pub fn is_downloadable(url: &str) -> bool {
        let resource_type = Self::detect_resource_type(url);
        
        // Consider media and documents as downloadable
        matches!(
            resource_type,
            ResourceType::Image
                | ResourceType::Video
                | ResourceType::Audio
                | ResourceType::Document
                | ResourceType::Archive
                | ResourceType::Executable
        )
    }

    /// Clean URL by removing fragments and normalizing
    #[allow(dead_code)]
    pub fn clean_url(url: &str) -> Option<String> {
        let mut parsed = Url::parse(url).ok()?;
        
        // Remove fragment
        parsed.set_fragment(None);
        
        // Sort query parameters for consistency
        let query_pairs: Vec<_> = parsed.query_pairs().collect();
        if !query_pairs.is_empty() {
            let mut sorted_pairs: Vec<_> = query_pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            sorted_pairs.sort_by(|a, b| a.0.cmp(&b.0));
            
            let mut query = String::new();
            for (i, (key, value)) in sorted_pairs.iter().enumerate() {
                if i > 0 {
                    query.push('&');
                }
                query.push_str(&format!("{}={}", 
                    urlencoding::encode(key),
                    urlencoding::encode(value)
                ));
            }
            parsed.set_query(Some(&query));
        }

        Some(parsed.to_string())
    }

    /// Get filename from URL
    #[allow(dead_code)]
    pub fn get_filename(url: &str) -> Option<String> {
        let parsed = Url::parse(url).ok()?;
        let path = parsed.path();
        
        // Get last segment of path
        let filename = path.rsplit('/').next()?;
        
        if filename.is_empty() {
            return None;
        }

        // Remove query string and fragment if present
        let filename = filename.split('?').next()?.split('#').next()?;
        
        Some(filename.to_string())
    }

    /// Check if URL is same origin
    #[allow(dead_code)]
    pub fn is_same_origin(url1: &str, url2: &str) -> bool {
        let parsed1 = Url::parse(url1);
        let parsed2 = Url::parse(url2);

        match (parsed1, parsed2) {
            (Ok(u1), Ok(u2)) => {
                u1.scheme() == u2.scheme() && u1.host() == u2.host() && u1.port() == u2.port()
            }
            _ => false,
        }
    }
}

impl Default for UrlExtractor {
    fn default() -> Self {
        Self::new()
    }
}