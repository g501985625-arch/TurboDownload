//! Crawler Service Module
//! 
//! Web crawling and resource extraction

pub mod html_parser;
pub mod url_extractor;

use reqwest::Client;
use chrono::Utc;

use crate::models::{CrawlResult, Resource, ResourceType, Result, AppError};
pub use html_parser::HtmlParser;
pub use url_extractor::UrlExtractor;

/// Crawler service for web resource extraction
pub struct CrawlerService {
    /// HTTP client
    client: Client,
    /// HTML parser
    parser: HtmlParser,
    /// URL extractor
    extractor: UrlExtractor,
}

impl CrawlerService {
    /// Create a new crawler service
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("TurboDownload/0.1.0 (Web Crawler)")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            parser: HtmlParser::new(),
            extractor: UrlExtractor::new(),
        }
    }

    /// Crawl a URL and extract resources
    pub async fn crawl(&self, url: &str, depth: u32) -> Result<CrawlResult> {
        // Validate URL
        let base_url = UrlExtractor::parse_url(url)?;

        // Fetch page content
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Failed to fetch page: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::CrawlerError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let _content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let html = response
            .text()
            .await
            .map_err(|e| AppError::CrawlerError(format!("Failed to read response: {}", e)))?;

        // Parse HTML and extract resources
        let title = self.parser.extract_title(&html);
        let raw_urls = self.parser.extract_urls(&html, &base_url);

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
            depth,
            crawled_at: Utc::now(),
        })
    }

    /// Crawl multiple URLs concurrently
    pub async fn crawl_multiple(&self, urls: &[String], depth: u32) -> Vec<Result<CrawlResult>> {
        let mut results = Vec::with_capacity(urls.len());
        
        for url in urls {
            results.push(self.crawl(url, depth).await);
        }

        results
    }
}

impl Default for CrawlerService {
    fn default() -> Self {
        Self::new()
    }
}