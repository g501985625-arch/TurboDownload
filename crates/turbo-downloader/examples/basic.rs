//! Basic download example

use std::path::PathBuf;
use turbo_downloader::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create downloader with default configuration
    let downloader = DownloaderBuilder::new()
        .max_concurrent_tasks(3)
        .default_threads(4)
        .build()?;

    // Create download configuration
    let config = DownloadConfig {
        id: uuid::Uuid::new_v4().to_string(),
        url: "https://example.com/file.zip".to_string(),
        output_path: PathBuf::from("./downloads/file.zip"),
        threads: 4,
        chunk_size: 0, // Auto
        resume_support: true,
        user_agent: Some("TurboDownload/1.0".to_string()),
        headers: std::collections::HashMap::new(),
        speed_limit: 0, // No limit
    };

    // Create task
    let task_id = downloader.manager().create_task(config.clone()).await?;
    println!("Created task: {}", task_id);

    println!("Download configuration created:");
    println!("  URL: {}", config.url);
    println!("  Output: {:?}", config.output_path);
    println!("  Threads: {}", config.threads);

    Ok(())
}
