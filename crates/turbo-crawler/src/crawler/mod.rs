//! Crawler implementation

use crate::{CrawlerClient, Result, Resource, ResourceExtractor, ResourceClassifier, UrlScheduler, QueuePolicy};
use std::collections::HashSet;
use std::time::Duration;

/// Crawl configuration
#[derive(Debug, Clone)]
pub struct CrawlConfig {
    /// Maximum concurrent requests
    pub max_concurrent: usize,
    /// Maximum pages to crawl
    pub max_pages: usize,
    /// Maximum depth
    pub max_depth: usize,
    /// Rate limit between requests
    pub rate_limit: Duration,
    /// Follow external links
    pub follow_external: bool,
    /// User agent
    pub user_agent: String,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 3,
            max_pages: 100,
            max_depth: 3,
            rate_limit: Duration::from_millis(500),
            follow_external: false,
            user_agent: "TurboCrawler/1.0".to_string(),
        }
    }
}

/// Crawl result
#[derive(Debug, Clone)]
pub struct CrawlResult {
    pub url: String,
    pub resources: Vec<Resource>,
    pub pages_scanned: usize,
    pub duration_ms: u64,
}

/// Web crawler
pub struct Crawler {
    client: CrawlerClient,
    config: CrawlConfig,
}

impl Crawler {
    /// Create new crawler
    pub fn new(config: CrawlConfig) -> Result<Self> {
        let client = CrawlerClient::with_defaults()?;
        
        Ok(Self { client, config })
    }
    
    /// Crawl a single URL
    pub async fn crawl(&self, url: &str) -> Result<CrawlResult> {
        let start = std::time::Instant::now();
        
        // Fetch HTML
        let html = self.client.fetch(url).await?;
        
        // Extract resources
        let extractor = ResourceExtractor::new(url);
        let resources = extractor.extract(&html)?;
        
        // Filter resources
        let classifier = ResourceClassifier::new();
        let resources = classifier.filter(resources);
        
        let duration = start.elapsed();
        
        Ok(CrawlResult {
            url: url.to_string(),
            resources,
            pages_scanned: 1,
            duration_ms: duration.as_millis() as u64,
        })
    }
    
    /// Crawl multiple URLs
    pub async fn crawl_batch(&self, urls: Vec<String>) -> Result<Vec<CrawlResult>> {
        let mut results = Vec::new();
        
        for url in urls.into_iter().take(self.config.max_pages) {
            match self.crawl(&url).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    tracing::warn!("Failed to crawl {}: {}", url, e);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Scan a site recursively
    pub async fn scan_site(&self, start_url: &str) -> Result<CrawlResult> {
        let start = std::time::Instant::now();
        
        let mut scheduler = UrlScheduler::new(
            QueuePolicy::Fifo,
            self.config.max_depth,
            self.config.rate_limit,
        );
        
        let mut all_resources = Vec::new();
        let mut pages_scanned = 0;
        let mut seen_urls = HashSet::new();
        
        // Get the base domain for filtering
        let start_host = url::Url::parse(start_url)
            .ok()
            .and_then(|u| u.host_str().map(String::from));
        
        scheduler.add(start_url.to_string());
        
        while let Some(url) = scheduler.next() {
            if pages_scanned >= self.config.max_pages {
                break;
            }
            
            if seen_urls.contains(&url) {
                continue;
            }
            seen_urls.insert(url.clone());
            
            // Fetch page
            match self.client.fetch(&url).await {
                Ok(html) => {
                    pages_scanned += 1;
                    
                    // Extract resources
                    let extractor = ResourceExtractor::new(&url);
                    let resources = extractor.extract(&html)?;
                    
                    // Add new URLs to scheduler
                    for resource in &resources {
                        if let Ok(parsed) = url::Url::parse(&resource.url) {
                            // Only add URLs from same domain
                            if let Some(host) = parsed.host_str() {
                                if let Some(ref start) = start_host {
                                    if host.contains(start) || host == start {
                                        scheduler.add(resource.url.clone());
                                    }
                                }
                            }
                        }
                    }
                    
                    all_resources.extend(resources);
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch {}: {}", url, e);
                }
            }
        }
        
        // Filter resources
        let classifier = ResourceClassifier::new();
        all_resources = classifier.filter(all_resources);
        
        let duration = start.elapsed();
        
        Ok(CrawlResult {
            url: start_url.to_string(),
            resources: all_resources,
            pages_scanned,
            duration_ms: duration.as_millis() as u64,
        })
    }
}