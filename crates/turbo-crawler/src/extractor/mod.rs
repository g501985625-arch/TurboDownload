//! Resource extractor and classifier

use serde::{Deserialize, Serialize};
use crate::Result;

/// Resource type enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    Image,
    Video,
    Audio,
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
        
        // Images
        if url_lower.ends_with(".jpg") || url_lower.ends_with(".jpeg") ||
           url_lower.ends_with(".png") || url_lower.ends_with(".gif") ||
           url_lower.ends_with(".webp") || url_lower.ends_with(".svg") ||
           url_lower.ends_with(".bmp") || url_lower.ends_with(".ico") {
            return ResourceType::Image;
        }
        
        // Videos
        if url_lower.ends_with(".mp4") || url_lower.ends_with(".webm") ||
           url_lower.ends_with(".avi") || url_lower.ends_with(".mov") ||
           url_lower.ends_with(".mkv") {
            return ResourceType::Video;
        }
        
        // Audio
        if url_lower.ends_with(".mp3") || url_lower.ends_with(".wav") ||
           url_lower.ends_with(".ogg") || url_lower.ends_with(".flac") ||
           url_lower.ends_with(".aac") {
            return ResourceType::Audio;
        }
        
        // Documents
        if url_lower.ends_with(".pdf") || url_lower.ends_with(".doc") ||
           url_lower.ends_with(".docx") || url_lower.ends_with(".txt") ||
           url_lower.ends_with(".md") {
            return ResourceType::Document;
        }
        
        // Archives
        if url_lower.ends_with(".zip") || url_lower.ends_with(".rar") ||
           url_lower.ends_with(".7z") || url_lower.ends_with(".tar") ||
           url_lower.ends_with(".gz") {
            return ResourceType::Archive;
        }
        
        // Scripts
        if url_lower.ends_with(".js") || url_lower.ends_with(".jsx") ||
           url_lower.ends_with(".ts") || url_lower.ends_with(".tsx") {
            return ResourceType::Script;
        }
        
        // Stylesheets
        if url_lower.ends_with(".css") || url_lower.ends_with(".scss") ||
           url_lower.ends_with(".less") {
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
    
    /// Check if downloadable
    pub fn is_downloadable(&self) -> bool {
        matches!(
            self,
            ResourceType::Image | 
            ResourceType::Video | 
            ResourceType::Audio |
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
}

impl Resource {
    /// Create new resource from URL
    pub fn new(url: String, source_url: String) -> Self {
        let resource_type = ResourceType::from_url(&url);
        let filename = url.split('/').next_back().map(String::from);
        let downloadable = resource_type.is_downloadable();
        
        Self {
            url,
            resource_type,
            filename,
            size: None,
            mime_type: None,
            source_url,
            downloadable,
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
        
        // Extract images
        for src in parser.extract_images() {
            let url = self.normalize_url(&src);
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