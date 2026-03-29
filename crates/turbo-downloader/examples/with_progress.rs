//! Download with progress callback example

use std::path::PathBuf;
use turbo_downloader::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create downloader
    let downloader = DownloaderBuilder::new()
        .max_concurrent_tasks(3)
        .default_threads(4)
        .build()?;

    // Create configuration with URL
    let config = DownloadConfig {
        id: uuid::Uuid::new_v4().to_string(),
        url: "https://example.com/large-file.zip".to_string(),
        output_path: PathBuf::from("./downloads/file.zip"),
        threads: 4,
        chunk_size: 0,
        resume_support: true,
        user_agent: Some("TurboDownload/1.0".to_string()),
        headers: Default::default(),
        speed_limit: 0,
    };

    // Define progress callback
    let progress_callback = |progress: DownloadProgress| {
        println!(
            "Progress: {:.1}% | Downloaded: {} bytes | Speed: {} bytes/s | ETA: {:?}s",
            progress.percent, progress.downloaded, progress.speed, progress.eta
        );
    };

    // Create task
    let task_id = downloader.manager().create_task(config).await?;
    println!("Task created: {}", task_id);

    // In a real implementation, you would:
    // 1. Get file size via HEAD request
    // 2. Calculate chunks
    // 3. Start concurrent downloads with scheduler
    // 4. Progress updates would trigger the callback

    println!("\nTo start download with progress tracking, you would use:");
    println!("scheduler.run(strategy, url, client, temp_dir, progress_callback).await");

    Ok(())
}
