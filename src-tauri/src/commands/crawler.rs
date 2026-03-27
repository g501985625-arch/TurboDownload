//! Crawler commands for Tauri

use crate::models::{CrawlResult, Resource, Result, AppError};
use crate::services::{HtmlParser, UrlExtractor};

/// Crawl a single page and extract resources
#[tauri::command]
pub async fn crawl_page(url: String) -> Result<CrawlResult> {
    // Validate URL
    let parsed = url::Url::parse(&url)
        .map_err(|e| AppError::InvalidUrl(format!("Invalid URL: {}", e)))?;
    
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err(AppError::InvalidUrl("Only HTTP/HTTPS URLs are supported".to_string()));
    }

    // Fetch the page
    let client = reqwest::Client::builder()
        .user_agent("TurboDownload/0.1.0 (Web Crawler)")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| AppError::NetworkError(format!("Failed to create client: {}", e)))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("Failed to fetch page: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::NetworkError(format!("HTTP error: {}", response.status())));
    }

    let html = response
        .text()
        .await
        .map_err(|e| AppError::NetworkError(format!("Failed to read response: {}", e)))?;

    // Parse HTML
    let parser = HtmlParser::new();
    parser.parse(&html, &url)
}

/// Scan a site for resources (with depth limit)
#[tauri::command]
pub async fn scan_site(
    url: String,
    max_depth: Option<u32>,
    max_pages: Option<usize>,
) -> Result<Vec<CrawlResult>> {
    let max_depth = max_depth.unwrap_or(2).min(5); // Cap at 5
    let max_pages = max_pages.unwrap_or(50).min(100); // Cap at 100

    // Validate URL
    let parsed = url::Url::parse(&url)
        .map_err(|e| AppError::InvalidUrl(format!("Invalid URL: {}", e)))?;
    
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err(AppError::InvalidUrl("Only HTTP/HTTPS URLs are supported".to_string()));
    }

    let client = reqwest::Client::builder()
        .user_agent("TurboDownload/0.1.0 (Web Crawler)")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| AppError::NetworkError(format!("Failed to create client: {}", e)))?;

    let parser = HtmlParser::new();
    let mut results = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut queue = vec![(url.clone(), 0u32)];

    while let Some((current_url, depth)) = queue.pop() {
        // Check limits
        if visited.len() >= max_pages || depth > max_depth {
            continue;
        }

        // Skip if already visited
        if visited.contains(&current_url) {
            continue;
        }
        visited.insert(current_url.clone());

        // Fetch page
        let response = match client.get(&current_url).send().await {
            Ok(r) => r,
            Err(_) => continue,
        };

        if !response.status().is_success() {
            continue;
        }

        let html = match response.text().await {
            Ok(h) => h,
            Err(_) => continue,
        };

        // Parse for resources
        let crawl_result = match parser.parse(&html, &current_url) {
            Ok(r) => r,
            Err(_) => continue,
        };

        // Extract links for further crawling
        if depth < max_depth {
            if let Ok(links) = parser.extract_links(&html, &current_url) {
                for link in links {
                    if !visited.contains(&link) {
                        queue.push((link, depth + 1));
                    }
                }
            }
        }

        results.push(crawl_result);
    }

    Ok(results)
}

/// Extract direct download links from a page
#[tauri::command]
pub async fn extract_download_links(url: String) -> Result<Vec<Resource>> {
    let result = crawl_page(url).await?;
    
    // Filter to only downloadable resources
    let resources: Vec<Resource> = result
        .resources
        .into_iter()
        .filter(|r| UrlExtractor::is_downloadable(&r.url))
        .collect();

    Ok(resources)
}

/// Get resource info (size, filename, etc.)
#[tauri::command]
pub async fn get_resource_info(url: String) -> Result<Resource> {
    // Validate URL
    let parsed = url::Url::parse(&url)
        .map_err(|e| AppError::InvalidUrl(format!("Invalid URL: {}", e)))?;
    
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err(AppError::InvalidUrl("Only HTTP/HTTPS URLs are supported".to_string()));
    }

    let client = reqwest::Client::builder()
        .user_agent("TurboDownload/0.1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| AppError::NetworkError(format!("Failed to create client: {}", e)))?;

    let response = client
        .head(&url)
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("Failed to get resource info: {}", e)))?;

    let size = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok());

    let mime_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| {
            // Remove charset and other parameters
            s.split(';').next().unwrap_or(s).trim().to_string()
        });

    let filename = response
        .headers()
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            v.split("filename=")
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string())
        })
        .or_else(|| {
            // Extract from URL
            url.split('/').last().map(|s| s.to_string())
        });

    let resource_type = mime_type
        .as_ref()
        .map(|mt| {
            if mt.starts_with("image/") {
                crate::models::ResourceType::Image
            } else if mt.starts_with("video/") {
                crate::models::ResourceType::Video
            } else if mt.starts_with("audio/") {
                crate::models::ResourceType::Audio
            } else {
                crate::models::ResourceType::from_url(&url)
            }
        })
        .unwrap_or_else(|| crate::models::ResourceType::from_url(&url));

    Ok(Resource {
        url: url.clone(),
        resource_type,
        title: filename,
        size,
        mime_type,
    })
}