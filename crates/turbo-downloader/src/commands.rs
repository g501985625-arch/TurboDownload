//! CLI commands for turbo-downloader

use crate::{
    download::DownloadConfig,
    downloader::MultiThreadDownloader,
    progress::DownloadProgress,
    Result,
};
use std::path::PathBuf;

/// Start a new download
/// 
/// # Arguments
/// - `url`: Download URL
/// - `output_path`: Output file path
/// - `threads`: Number of download threads (default: 4)
/// 
/// # Returns
/// - Task ID for tracking the download
pub async fn start_download(url: String, output_path: PathBuf, threads: u32) -> Result<String> {
    let config = DownloadConfig {
        id: uuid::Uuid::new_v4().to_string(),
        url,
        output_path,
        threads: if threads == 0 { 4 } else { threads },
        chunk_size: 1024 * 1024, // 1MB default
        resume_support: true,
        user_agent: Some("TurboDownload/1.0".to_string()),
        headers: Default::default(),
        speed_limit: 0,
    };

    let downloader = MultiThreadDownloader::new(config)?;
    let result = downloader.download().await?;
    
    Ok(result.task_id)
}

/// Start a download with full configuration
pub async fn start_download_with_config(config: DownloadConfig) -> Result<String> {
    let downloader = MultiThreadDownloader::new(config)?;
    let result = downloader.download().await?;
    
    Ok(result.task_id)
}

/// Pause a download
/// 
/// Saves the current download state for later resumption.
pub async fn pause_download(task_id: String) -> Result<()> {
    // TODO: Implement pause logic with state persistence
    // This requires storing the downloader instance and calling pause()
    println!("Pausing download: {}", task_id);
    Ok(())
}

/// Resume a download
/// 
/// Loads the saved state and continues downloading.
pub async fn resume_download(task_id: String, output_path: PathBuf) -> Result<String> {
    // TODO: Implement resume logic with state loading
    // For now, start a new download
    println!("Resuming download: {}", task_id);
    
    // Load state from StateManager
    // let state_manager = StateManager::new(PathBuf::from(".download_states"));
    // let state = state_manager.load(&task_id).await?;
    
    // if let Some(state) = state {
    //     // Resume from saved state
    // } else {
    //     return Err(DownloadError::TaskNotFound(task_id));
    // }
    
    Ok(task_id)
}

/// Cancel a download
/// 
/// Stops the download and cleans up temporary files.
pub async fn cancel_download(task_id: String) -> Result<()> {
    // TODO: Implement cancel logic
    // - Stop all workers
    // - Cleanup temp files
    // - Remove state file
    println!("Cancelling download: {}", task_id);
    Ok(())
}

/// Get download progress
/// 
/// # Returns
/// - Current download progress information
pub async fn get_progress(task_id: String) -> Result<DownloadProgress> {
    // TODO: Implement progress query
    // This requires storing progress in a shared location
    Ok(DownloadProgress {
        total: 0,
        downloaded: 0,
        speed: 0,
        avg_speed: 0,
        eta: None,
        percent: 0.0,
    })
}

/// List all active downloads
pub async fn list_downloads() -> Result<Vec<String>> {
    // TODO: Implement download listing
    // Return list of active task IDs
    Ok(Vec::new())
}

/// Get download result
/// 
/// Returns the final result of a completed download.
pub async fn get_download_result(task_id: String) -> Result<crate::download::DownloadResult> {
    // TODO: Implement result retrieval
    // Return the DownloadResult for a completed task
    Err(crate::DownloadError::TaskNotFound(task_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_config_creation() {
        let config = DownloadConfig {
            id: "test-123".to_string(),
            url: "http://example.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/test.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: Some("Test/1.0".to_string()),
            headers: Default::default(),
            speed_limit: 0,
        };

        assert_eq!(config.id, "test-123");
        assert_eq!(config.threads, 4);
        assert!(config.resume_support);
    }
}
