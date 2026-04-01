// Crawler commands - Real implementation using turbo-crawler
use crate::commands::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use turbo_crawler::{Resource, ResourceType, CrawlerError};

/// Crawl result for a single URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    pub url: String,
    pub title: String,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub resources: Vec<ResourceInfo>,
    pub pages_scanned: usize,
    pub duration_ms: u64,
}

/// Resource information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub url: String,
    pub resource_type: String,
    pub filename: Option<String>,
    pub size: Option<u64>,
    pub downloadable: bool,
}

impl From<Resource> for ResourceInfo {
    fn from(resource: Resource) -> Self {
        Self {
            url: resource.url,
            resource_type: format!("{:?}", resource.resource_type),
            filename: resource.filename,
            size: resource.size,
            downloadable: resource.downloadable,
        }
    }
}

/// Helper to convert raw crawl result to CrawlResult
fn convert_crawl_result(url: String, result: turbo_crawler::CrawlResult) -> CrawlResult {
    let title = result
        .resources
        .iter()
        .find(|r| matches!(r.resource_type, ResourceType::Html))
        .map(|r| r.filename.clone().unwrap_or_else(|| "Untitled".to_string()))
        .unwrap_or_else(|| "Untitled".to_string());

    let links: Vec<String> = result
        .resources
        .iter()
        .filter(|r| matches!(r.resource_type, ResourceType::Html))
        .map(|r| r.url.clone())
        .collect();

    let images: Vec<String> = result
        .resources
        .iter()
        .filter(|r| matches!(r.resource_type, ResourceType::Image))
        .map(|r| r.url.clone())
        .collect();

    let resources: Vec<ResourceInfo> = result.resources.into_iter().map(ResourceInfo::from).collect();

    CrawlResult {
        url,
        title,
        links,
        images,
        resources,
        pages_scanned: result.pages_scanned,
        duration_ms: result.duration_ms,
    }
}

/// Crawl a single URL
#[tauri::command]
pub async fn crawl_url(
    url: String,
    state: State<'_, Arc<AppState>>,
) -> Result<CrawlResult, String> {
    log::info!("Crawling URL: {}", url);

    // Clone the URL for use in blocking task
    let url_clone = url.clone();
    
    // Use tokio to run the blocking crawl operation
    let result = tokio::task::spawn_blocking(move || -> Result<turbo_crawler::CrawlResult, CrawlerError> {
        // Create a new crawler instance for this blocking operation
        let crawler = turbo_crawler::Crawler::new(
            turbo_crawler::CrawlConfig::default()
        )?;
        
        // Use block_on to run the async crawl method synchronously
        tokio::runtime::Handle::current().block_on(crawler.crawl(&url_clone))
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e: CrawlerError| e.to_string())?;

    Ok(convert_crawl_result(url, result))
}

/// Crawl multiple URLs
#[tauri::command]
pub async fn crawl_batch(
    urls: Vec<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<CrawlResult>, String> {
    log::info!("Crawling {} URLs", urls.len());
    
    // Placeholder - return empty results
    Ok(vec![])
}

/// Scan a website recursively
#[tauri::command]
pub async fn scan_site(
    url: String,
    max_pages: Option<usize>,
    state: State<'_, Arc<AppState>>,
) -> Result<CrawlResult, String> {
    log::info!("Scanning site: {} (max_pages: {:?})", url, max_pages);
    
    // Placeholder - return empty result
    Ok(CrawlResult {
        url: url.clone(),
        title: "Untitled".to_string(),
        links: vec![],
        images: vec![],
        resources: vec![],
        pages_scanned: 0,
        duration_ms: 0,
    })
}

/// Scan URL for downloadable resources - returns resources as frontend ResourceItem format
#[tauri::command]
pub async fn scan_url(
    url: String,
) -> Result<Vec<ResourceInfo>, String> {
    log::info!("Scanning URL for resources: {}", url);
    
    let url_clone = url.clone();
    
    // Use tokio to run the blocking crawl operation
    let result = tokio::task::spawn_blocking(move || -> Result<turbo_crawler::CrawlResult, CrawlerError> {
        let crawler = turbo_crawler::Crawler::new(
            turbo_crawler::CrawlConfig::default()
        )?;
        
        // Use block_on to run the async crawl method synchronously
        tokio::runtime::Handle::current().block_on(crawler.crawl(&url_clone))
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e: CrawlerError| e.to_string())?;

    // Convert resources to ResourceInfo format with deduplication
    let mut seen_urls = std::collections::HashSet::new();
    let mut resources: Vec<ResourceInfo> = Vec::new();
    
    for resource in result.resources {
        // Skip duplicates by URL
        if seen_urls.contains(&resource.url) {
            continue;
        }
        seen_urls.insert(resource.url.clone());
        
        // Only include downloadable resources (images, videos, audio, documents)
        let is_downloadable = matches!(
            resource.resource_type,
            ResourceType::Image | ResourceType::Video | ResourceType::Audio | ResourceType::Document
        );
        
        if is_downloadable {
            resources.push(ResourceInfo::from(resource));
        }
    }

    Ok(resources)
}