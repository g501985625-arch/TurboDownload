//! HTML Parser
//! 
//! Parses HTML content to extract metadata and links

use scraper::{Html, Selector};
use url::Url;
use chrono::Utc;

use crate::models::{CrawlResult, Resource, ResourceType};

/// HTML parser for extracting links and metadata
pub struct HtmlParser {
    /// Selector for title element
    title_selector: Selector,
    /// Selector for anchor tags
    a_selector: Selector,
    /// Selector for image tags
    img_selector: Selector,
    /// Selector for video tags
    video_selector: Selector,
    /// Selector for audio tags
    audio_selector: Selector,
    /// Selector for source tags
    source_selector: Selector,
    /// Selector for link tags
    link_selector: Selector,
}

impl HtmlParser {
    /// Create a new HTML parser
    pub fn new() -> Self {
        Self {
            title_selector: Selector::parse("title").expect("Invalid title selector"),
            a_selector: Selector::parse("a[href]").expect("Invalid a selector"),
            img_selector: Selector::parse("img[src]").expect("Invalid img selector"),
            video_selector: Selector::parse("video[src], video source[src]").expect("Invalid video selector"),
            audio_selector: Selector::parse("audio[src], audio source[src]").expect("Invalid audio selector"),
            source_selector: Selector::parse("source[src]").expect("Invalid source selector"),
            link_selector: Selector::parse("link[href]").expect("Invalid link selector"),
        }
    }

    /// Parse HTML and extract resources into a CrawlResult
    pub fn parse(&self, html: &str, url: &str) -> crate::models::Result<CrawlResult> {
        let base_url = Url::parse(url)
            .map_err(|e| crate::models::AppError::InvalidUrl(format!("Invalid base URL: {}", e)))?;
        
        let title = self.extract_title(html);
        let raw_urls = self.extract_urls(html, &base_url);

        // Convert URLs to resources with type detection
        let resources: Vec<Resource> = raw_urls
            .into_iter()
            .map(|url| {
                let resource_type = ResourceType::from_url(&url);
                Resource {
                    url,
                    resource_type,
                    title: None,
                    size: None,
                    mime_type: None,
                }
            })
            .collect();

        Ok(CrawlResult {
            source_url: url.to_string(),
            resources,
            title,
            depth: 0,
            crawled_at: Utc::now(),
        })
    }

    /// Extract links for further crawling (same-domain links only)
    pub fn extract_links(&self, html: &str, url: &str) -> crate::models::Result<Vec<String>> {
        let base_url = Url::parse(url)
            .map_err(|e| crate::models::AppError::InvalidUrl(format!("Invalid base URL: {}", e)))?;
        
        Ok(self.extract_link_urls(html, &base_url))
    }

    /// Extract page title
    pub fn extract_title(&self, html: &str) -> Option<String> {
        let document = Html::parse_document(html);
        
        document
            .select(&self.title_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
    }

    /// Extract all URLs from the HTML
    pub fn extract_urls(&self, html: &str, base_url: &Url) -> Vec<String> {
        let document = Html::parse_document(html);
        let mut urls = std::collections::HashSet::new();

        // Extract from anchor tags
        for element in document.select(&self.a_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Some(url) = self.resolve_url(href, base_url) {
                    urls.insert(url);
                }
            }
        }

        // Extract from image tags
        for element in document.select(&self.img_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Some(url) = self.resolve_url(src, base_url) {
                    urls.insert(url);
                }
            }
            // Also check srcset
            if let Some(srcset) = element.value().attr("srcset") {
                for url in self.parse_srcset(srcset, base_url) {
                    urls.insert(url);
                }
            }
        }

        // Extract from video tags
        for element in document.select(&self.video_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Some(url) = self.resolve_url(src, base_url) {
                    urls.insert(url);
                }
            }
        }

        // Extract from audio tags
        for element in document.select(&self.audio_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Some(url) = self.resolve_url(src, base_url) {
                    urls.insert(url);
                }
            }
        }

        // Extract from source tags
        for element in document.select(&self.source_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Some(url) = self.resolve_url(src, base_url) {
                    urls.insert(url);
                }
            }
        }

        // Extract from link tags (stylesheets, icons, etc.)
        for element in document.select(&self.link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Some(url) = self.resolve_url(href, base_url) {
                    // Only include downloadable resources
                    let rel = element.value().attr("rel").unwrap_or("");
                    if rel.contains("icon") || rel.contains("stylesheet") {
                        urls.insert(url);
                    }
                }
            }
        }

        urls.into_iter().collect()
    }

    /// Resolve a relative URL to absolute URL
    fn resolve_url(&self, href: &str, base_url: &Url) -> Option<String> {
        // Skip JavaScript and mailto links
        if href.starts_with("javascript:") || href.starts_with("mailto:") || href.starts_with("#") {
            return None;
        }

        // Skip data URLs
        if href.starts_with("data:") {
            return None;
        }

        base_url
            .join(href)
            .ok()
            .map(|url| url.to_string())
    }

    /// Parse srcset attribute
    fn parse_srcset(&self, srcset: &str, base_url: &Url) -> Vec<String> {
        srcset
            .split(',')
            .filter_map(|part| {
                let part = part.trim();
                let url_part = part.split_whitespace().next()?;
                self.resolve_url(url_part, base_url)
            })
            .collect()
    }

    /// Extract meta description
    pub fn extract_meta_description(&self, html: &str) -> Option<String> {
        let document = Html::parse_document(html);
        
        let selector = Selector::parse("meta[name=\"description\"]").ok()?;
        
        document
            .select(&selector)
            .next()
            .and_then(|el| el.value().attr("content").map(|s| s.to_string()))
    }

    /// Extract all image URLs
    pub fn extract_image_urls(&self, html: &str, base_url: &Url) -> Vec<String> {
        let document = Html::parse_document(html);
        let mut urls = std::collections::HashSet::new();

        for element in document.select(&self.img_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Some(url) = self.resolve_url(src, base_url) {
                    urls.insert(url);
                }
            }
        }

        urls.into_iter().collect()
    }

    /// Extract all link URLs (for further crawling)
    pub fn extract_link_urls(&self, html: &str, base_url: &Url) -> Vec<String> {
        let document = Html::parse_document(html);
        let mut urls = std::collections::HashSet::new();

        for element in document.select(&self.a_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Some(url) = self.resolve_url(href, base_url) {
                    // Only include HTTP/HTTPS URLs from the same domain
                    if let Ok(parsed) = Url::parse(&url) {
                        if parsed.scheme() == "http" || parsed.scheme() == "https" {
                            if parsed.domain() == base_url.domain() {
                                urls.insert(url);
                            }
                        }
                    }
                }
            }
        }

        urls.into_iter().collect()
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}