//! HTML parser module

use scraper::{Html, Selector, ElementRef};

/// HTML parser for extracting content
pub struct HtmlParser {
    document: Html,
}

impl HtmlParser {
    /// Parse HTML string
    pub fn new(html: &str) -> Self {
        Self {
            document: Html::parse_document(html),
        }
    }
    
    /// Select elements by CSS selector
    pub fn select(&self, selector: &str) -> Vec<ElementRef<'_>> {
        let selector = match Selector::parse(selector) {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        
        self.document.select(&selector).collect()
    }
    
    /// Get text content of an element
    pub fn text(&self, element: &ElementRef) -> String {
        element.text().collect::<Vec<_>>().join("")
    }
    
    /// Get attribute value
    pub fn attr(&self, element: &ElementRef, attr: &str) -> Option<String> {
        element.value().attr(attr).map(String::from)
    }
    
    /// Extract all links (href attributes)
    pub fn extract_links(&self) -> Vec<String> {
        let selector = Selector::parse("a[href]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("href"))
            .map(String::from)
            .collect()
    }
    
    /// Extract all images (src attributes)
    pub fn extract_images(&self) -> Vec<String> {
        let selector = Selector::parse("img[src]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("src"))
            .map(String::from)
            .collect()
    }
    
    /// Extract all scripts (src attributes)
    pub fn extract_scripts(&self) -> Vec<String> {
        let selector = Selector::parse("script[src]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("src"))
            .map(String::from)
            .collect()
    }
    
    /// Extract all stylesheets (href attributes)
    pub fn extract_stylesheets(&self) -> Vec<String> {
        let selector = Selector::parse("link[rel='stylesheet'][href]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("href"))
            .map(String::from)
            .collect()
    }
    
    /// Get page title
    pub fn title(&self) -> Option<String> {
        let selector = Selector::parse("title").ok()?;
        self.document.select(&selector).next().map(|el| el.text().collect())
    }
    
    /// Get all meta tags
    pub fn meta_tags(&self) -> Vec<(String, String)> {
        let selector = Selector::parse("meta").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| {
                let name = el.value().attr("name").or(el.value().attr("property"));
                let content = el.value().attr("content");
                match (name, content) {
                    (Some(n), Some(c)) => Some((n.to_string(), c.to_string())),
                    _ => None,
                }
            })
            .collect()
    }
    
    /// Extract all videos from <video> and <source> tags
    pub fn extract_videos(&self) -> Vec<String> {
        let mut videos = Vec::new();
        
        // Extract from <video src="...">
        if let Ok(selector) = Selector::parse("video[src]") {
            for el in self.document.select(&selector) {
                if let Some(src) = el.value().attr("src") {
                    videos.push(src.to_string());
                }
            }
        }
        
        // Extract from <video><source src="...">
        if let Ok(selector) = Selector::parse("video source[src]") {
            for el in self.document.select(&selector) {
                if let Some(src) = el.value().attr("src") {
                    if !videos.contains(&src.to_string()) {
                        videos.push(src.to_string());
                    }
                }
            }
        }
        
        // Extract from <source> inside <video>
        if let Ok(selector) = Selector::parse("video") {
            for video_el in self.document.select(&selector) {
                if let Ok(source_selector) = Selector::parse("source[src]") {
                    for source_el in video_el.select(&source_selector) {
                        if let Some(src) = source_el.value().attr("src") {
                            if !videos.contains(&src.to_string()) {
                                videos.push(src.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        videos
    }
    
    /// Extract all audios from <audio> and <source> tags
    pub fn extract_audios(&self) -> Vec<String> {
        let mut audios = Vec::new();
        
        // Extract from <audio src="...">
        if let Ok(selector) = Selector::parse("audio[src]") {
            for el in self.document.select(&selector) {
                if let Some(src) = el.value().attr("src") {
                    audios.push(src.to_string());
                }
            }
        }
        
        // Extract from <audio><source src="...">
        if let Ok(selector) = Selector::parse("audio source[src]") {
            for el in self.document.select(&selector) {
                if let Some(src) = el.value().attr("src") {
                    if !audios.contains(&src.to_string()) {
                        audios.push(src.to_string());
                    }
                }
            }
        }
        
        // Extract from <source> inside <audio>
        if let Ok(selector) = Selector::parse("audio") {
            for audio_el in self.document.select(&selector) {
                if let Ok(source_selector) = Selector::parse("source[src]") {
                    for source_el in audio_el.select(&source_selector) {
                        if let Some(src) = source_el.value().attr("src") {
                            if !audios.contains(&src.to_string()) {
                                audios.push(src.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        audios
    }
    
    /// Extract lazy-loaded images (data-src, data-lazy-src, data-original, etc.)
    pub fn extract_lazy_images(&self) -> Vec<String> {
        let lazy_attrs = [
            "data-src",
            "data-lazy-src",
            "data-original",
            "data-srcset",
            "data-lazy",
            "data-image",
            "data-bg",
            "data-thumb",
        ];
        
        let mut images = Vec::new();
        
        // Extract from img tags with lazy attributes
        for attr in &lazy_attrs {
            let selector_str = format!("img[{}]", attr);
            let sel = match Selector::parse(&selector_str) {
                Ok(s) => s,
                Err(_) => continue,
            };
            for el in self.document.select(&sel) {
                if let Some(src) = el.value().attr(attr) {
                    // Parse srcset if present
                    if *attr == "data-srcset" {
                        for src_part in src.split(',') {
                            let url = src_part.trim().split_whitespace().next()
                                .map(String::from);
                            if let Some(url) = url {
                                if !images.contains(&url) {
                                    images.push(url);
                                }
                            }
                        }
                    } else if !images.contains(&src.to_string()) {
                        images.push(src.to_string());
                    }
                }
            }
        }
        
        // Extract from picture > source elements
        if let Ok(selector) = Selector::parse("picture source[srcset]") {
            for el in self.document.select(&selector) {
                if let Some(srcset) = el.value().attr("srcset") {
                    for src_part in srcset.split(',') {
                        let url = src_part.trim().split_whitespace().next()
                            .map(String::from);
                        if let Some(url) = url {
                            if !images.contains(&url) {
                                images.push(url);
                            }
                        }
                    }
                }
            }
        }
        
        images
    }
    
    /// Extract background images from inline styles
    pub fn extract_background_images(&self) -> Vec<String> {
        let mut images = Vec::new();
        
        // Look for elements with style containing background-image
        if let Ok(selector) = Selector::parse("[style]") {
            for el in self.document.select(&selector) {
                if let Some(style) = el.value().attr("style") {
                    // Match url(...) patterns
                    let re = regex::Regex::new(r#"url\s*\(\s*['"]?([^'")]+)['"]?\s*\)"#).ok();
                    if let Some(regex) = re {
                        for cap in regex.captures_iter(style) {
                            if let Some(url) = cap.get(1) {
                                let url_str = url.as_str().to_string();
                                if !url_str.is_empty() && !url_str.starts_with("data:") {
                                    if !images.contains(&url_str) {
                                        images.push(url_str);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        images
    }
    
    /// Extract all iframes (src attributes)
    pub fn extract_iframes(&self) -> Vec<String> {
        let selector = Selector::parse("iframe[src]").unwrap();
        
        self.document.select(&selector)
            .filter_map(|el| el.value().attr("src"))
            .filter(|src| !src.starts_with("about:") && !src.starts_with("javascript:"))
            .map(String::from)
            .collect()
    }
    
    /// Extract streaming manifests (m3u8, mpd)
    pub fn extract_streaming_manifests(&self) -> Vec<String> {
        let mut manifests = Vec::new();
        
        // Extract from <video> and <audio> source tags
        if let Ok(selector) = Selector::parse("source[type]") {
            for el in self.document.select(&selector) {
                if let Some(src) = el.value().attr("src") {
                    let src_lower = src.to_lowercase();
                    if src_lower.contains(".m3u8") || src_lower.contains(".mpd") ||
                       src_lower.contains(".m3u") {
                        if !manifests.contains(&src.to_string()) {
                            manifests.push(src.to_string());
                        }
                    }
                }
                // Also check type attribute for streaming types
                if let Some(mime_type) = el.value().attr("type") {
                    if mime_type.contains("mpeg-dash") || mime_type.contains("application/vnd.apple.mpegurl") {
                        if let Some(src) = el.value().attr("src") {
                            if !manifests.contains(&src.to_string()) {
                                manifests.push(src.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        // Extract from script tags that might contain streaming URLs
        if let Ok(selector) = Selector::parse("script[src]") {
            for el in self.document.select(&selector) {
                if let Some(src) = el.value().attr("src") {
                    let src_lower = src.to_lowercase();
                    if src_lower.contains(".m3u8") || src_lower.contains(".mpd") ||
                       src_lower.contains(".m3u") {
                        if !manifests.contains(&src.to_string()) {
                            manifests.push(src.to_string());
                        }
                    }
                }
            }
        }
        
        // Extract from link tags
        if let Ok(selector) = Selector::parse("link[href]") {
            for el in self.document.select(&selector) {
                if let Some(href) = el.value().attr("href") {
                    let href_lower = href.to_lowercase();
                    if href_lower.contains(".m3u8") || href_lower.contains(".mpd") ||
                       href_lower.contains(".m3u") {
                        if !manifests.contains(&href.to_string()) {
                            manifests.push(href.to_string());
                        }
                    }
                }
            }
        }
        
        // Look for manifest URLs in inline JavaScript
        if let Ok(selector) = Selector::parse("script") {
            for el in self.document.select(&selector) {
                let script_text: String = el.text().collect();
                let re = regex::Regex::new(r#"['"]([^'"]+\.m3u8[^'"]*)['"]"#).ok();
                if let Some(regex) = re {
                    for cap in regex.captures_iter(&script_text) {
                        if let Some(url) = cap.get(1) {
                            let url_str = url.as_str().to_string();
                            if !manifests.contains(&url_str) {
                                manifests.push(url_str);
                            }
                        }
                    }
                }
                // Also match .mpd
                let re_mpd = regex::Regex::new(r#"['"]([^'"]+\.mpd[^'"]*)['"]"#).ok();
                if let Some(regex) = re_mpd {
                    for cap in regex.captures_iter(&script_text) {
                        if let Some(url) = cap.get(1) {
                            let url_str = url.as_str().to_string();
                            if !manifests.contains(&url_str) {
                                manifests.push(url_str);
                            }
                        }
                    }
                }
            }
        }
        
        manifests
    }
}