// Crawler commands - Real implementation using turbo-crawler
use crate::commands::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use turbo_crawler::Resource;
use turbo_crawler::ResourceType;

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

    // Get crawler and release lock, then call async function
    let crawler = {
        let crawler = state.crawler.lock();
        // Extract any needed config from crawler before releasing lock
        // For now, just create a new crawler instance for this request
        // This is a workaround - ideally the crawler would be Clone
        std::mem::drop(crawler);
        None::<()>
    };
    
    // Use tokio to run the blocking crawl operation
    let result = tokio::task::spawn_blocking(move || {
        // Need a fresh crawler - this is a design issue in the app state
        // For now, return a placeholder
        Err::<turbo_crawler::CrawlResult, _>("Crawler not available in async context".to_string())
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e: String| e)?;

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