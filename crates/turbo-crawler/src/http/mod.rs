//! HTTP client for crawler

use crate::{CrawlerError, Result};
use std::ops::Range;
use std::time::Duration;

/// HTTP client for crawling
pub struct CrawlerClient {
    client: reqwest::Client,
}

impl CrawlerClient {
    /// Create new crawler client
    pub fn new(timeout: Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .user_agent("TurboCrawler/1.0")
            .build()
            .map_err(|e: reqwest::Error| CrawlerError::Network(e.to_string()))?;
        
        Ok(Self { client })
    }
    
    /// Create with default settings
    pub fn with_defaults() -> Result<Self> {
        Self::new(Duration::from_secs(30))
    }
    
    /// Fetch a URL and return HTML content
    pub async fn fetch(&self, url: &str) -> Result<String> {
        let response = self.client.get(url)
            .send()
            .await
            .map_err(|e: reqwest::Error| CrawlerError::Network(e.to_string()))?;
        
        let status = response.status();
        if !status.is_success() {
            return Err(CrawlerError::Http(status.as_u16(), status.to_string()));
        }
        
        let text = response.text().await
            .map_err(|e: reqwest::Error| CrawlerError::Network(e.to_string()))?;
        
        Ok(text)
    }
    
    /// Get response headers only (HEAD request)
    pub async fn head(&self, url: &str) -> Result<Option<u64>> {
        let response = self.client.head(url)
            .send()
            .await
            .map_err(|e: reqwest::Error| CrawlerError::Network(e.to_string()))?;
        
        Ok(response.content_length())
    }
    
    /// Fetch a range of bytes from URL (for partial downloads)
    pub async fn fetch_range(&self, url: &str, range: Range<u64>) -> Result<Vec<u8>> {
        let range_header = format!("bytes={}-{}", range.start, range.end.saturating_sub(1));
        
        let response = self.client.get(url)
            .header("Range", range_header)
            .send()
            .await
            .map_err(|e: reqwest::Error| CrawlerError::Network(e.to_string()))?;
        
        let status = response.status();
        // Accept 200 (full) or 206 (partial)
        if !status.is_success() && status.as_u16() != 206 {
            return Err(CrawlerError::Http(status.as_u16(), status.to_string()));
        }
        
        let bytes = response.bytes().await
            .map_err(|e: reqwest::Error| CrawlerError::Network(e.to_string()))?;
        
        Ok(bytes.to_vec())
    }
    
    /// Check if server supports range requests
    pub async fn supports_range(&self, url: &str) -> Result<bool> {
        let response = self.client.head(url)
            .send()
            .await
            .map_err(|e: reqwest::Error| CrawlerError::Network(e.to_string()))?;
        
        Ok(response.headers()
            .get("accept-ranges")
            .and_then(|v| v.to_str().ok())
            .map(|v| v == "bytes")
            .unwrap_or(false))
    }
}