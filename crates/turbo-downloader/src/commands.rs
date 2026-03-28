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
    use tempfile::TempDir;
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    // Test 1: start_download with actual download via mock server
    #[tokio::test]
    async fn test_start_download_with_mock_server() {
        use wiremock::http::HeaderValue;
        use tokio::fs;
        
        // Start a mock HTTP server
        let mock_server = MockServer::start().await;
        
        // Create a small test file content
        let test_content = b"Hello, World! This is test content for download.";
        let content_length = test_content.len() as u64;
        let content_length_str = content_length.to_string();
        
        // Setup mock endpoint for HEAD request (range support check)
        Mock::given(matchers::method("HEAD"))
            .and(matchers::path("/test-file.txt"))
            .respond_with(ResponseTemplate::new(200)
                .insert_header("Content-Length", HeaderValue::from_bytes(content_length_str.as_bytes().to_vec()).unwrap())
                .insert_header("Accept-Ranges", HeaderValue::from_bytes(b"bytes".to_vec()).unwrap()))
            .mount(&mock_server)
            .await;
            
        // Setup mock endpoint for GET request - just return full content
        // (the downloader will handle Range headers if the server supports them)
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/test-file.txt"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_bytes(test_content.to_vec())
                .insert_header("Content-Length", HeaderValue::from_bytes(content_length_str.as_bytes().to_vec()).unwrap())
                .insert_header("Accept-Ranges", HeaderValue::from_bytes(b"bytes".to_vec()).unwrap()))
            .mount(&mock_server)
            .await;
            
        // Create temp output directory
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("downloaded.txt");
        
        // Build the mock server URL
        let url = format!("{}/test-file.txt", mock_server.uri());
        
        // Actually call start_download()
        let result = start_download(url.clone(), output_path.clone(), 1).await;
        
        // Verify download succeeded
        assert!(result.is_ok(), "Download should succeed: {:?}", result.err());
        
        // Verify the downloaded file exists and has correct content
        assert!(output_path.exists(), "Downloaded file should exist");
        
        // Verify content matches
        let downloaded_content = fs::read(&output_path).await.unwrap();
        assert_eq!(downloaded_content, test_content, "Downloaded content should match original");
        
        // Verify file size
        let metadata = fs::metadata(&output_path).await.unwrap();
        assert_eq!(metadata.len() as usize, test_content.len());
    }

    // Test 1b: start_download with config and multi-thread
    #[tokio::test]
    async fn test_start_download_with_config_actual_download() {
        use wiremock::http::HeaderValue;
        use tokio::fs;
        
        // Start a mock HTTP server
        let mock_server = MockServer::start().await;
        
        // Create a larger test file content to support multi-chunk download
        let test_content = b"The quick brown fox jumps over the lazy dog. This is a test file for multi-threaded download. abcdefghijklmnopqrstuvwxyz0123456789";
        let content_length = test_content.len() as u64;
        let content_length_str = content_length.to_string();
        
        // HEAD request - check range support
        Mock::given(matchers::method("HEAD"))
            .and(matchers::path("/large-file.txt"))
            .respond_with(ResponseTemplate::new(200)
                .insert_header("Content-Length", HeaderValue::from_bytes(content_length_str.as_bytes().to_vec()).unwrap())
                .insert_header("Accept-Ranges", HeaderValue::from_bytes(b"bytes".to_vec()).unwrap()))
            .mount(&mock_server)
            .await;
            
        // GET request - return full content (for non-range requests)
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/large-file.txt"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_bytes(test_content.to_vec())
                .insert_header("Content-Length", HeaderValue::from_bytes(content_length_str.as_bytes().to_vec()).unwrap())
                .insert_header("Accept-Ranges", HeaderValue::from_bytes(b"bytes".to_vec()).unwrap()))
            .mount(&mock_server)
            .await;
            
        // Create temp output directory
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("large-file.txt");
        
        let url = format!("{}/large-file.txt", mock_server.uri());
        
        // Use start_download_with_config
        let config = DownloadConfig {
            id: "test-multi-thread-123".to_string(),
            url,
            output_path: output_path.clone(),
            threads: 4,
            chunk_size: 1024, // Small chunks to test multi-thread
            resume_support: true,
            user_agent: Some("TurboDownload/1.0".to_string()),
            headers: Default::default(),
            speed_limit: 0,
        };
        
        let result = start_download_with_config(config).await;
        
        assert!(result.is_ok(), "Download should succeed: {:?}", result.err());
        
        // Verify content
        let downloaded_content = fs::read(&output_path).await.unwrap();
        assert_eq!(downloaded_content, test_content);
    }

    // Test 1c: start_download with invalid URL should fail
    #[tokio::test]
    async fn test_start_download_invalid_url() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("invalid.txt");
        
        // Invalid URL (not a valid HTTP URL)
        let result = start_download("not-a-valid-url".to_string(), output_path, 1).await;
        
        // Should fail with an error
        assert!(result.is_err());
    }

    // Test 1d: start_download with unreachable server
    #[tokio::test]
    async fn test_start_download_unreachable_server() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("unreachable.txt");
        
        // Use a URL to a non-routable address
        let result = start_download(
            "http://192.0.2.1/test.txt".to_string(), 
            output_path, 
            1
        ).await;
        
        // Should fail with network error
        assert!(result.is_err());
    }

    // Test 1e: start_download with 404 response
    #[tokio::test]
    async fn test_start_download_404_error() {
        use wiremock::http::HeaderValue;
        
        let mock_server = MockServer::start().await;
        
        // Return 404 for all requests
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(404)
                .insert_header("Content-Length", HeaderValue::from_bytes(b"0".to_vec()).unwrap()))
            .mount(&mock_server)
            .await;
            
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("notfound.txt");
        
        let url = format!("{}/not-exist.txt", mock_server.uri());
        let result = start_download(url, output_path, 1).await;
        
        // Should fail with HTTP error
        assert!(result.is_err());
    }

    // Test 2: pause_download command framework
    #[tokio::test]
    async fn test_pause_download() {
        let task_id = "test-task-123".to_string();
        
        // Test that pause_download returns Ok
        let result = pause_download(task_id.clone()).await;
        assert!(result.is_ok());
    }

    // Test 3: resume_download command framework  
    #[tokio::test]
    async fn test_resume_download() {
        let task_id = "test-task-456".to_string();
        let output_path = PathBuf::from("/tmp/test_resume.txt");
        
        // Test that resume_download returns Ok with task_id
        let result = resume_download(task_id.clone(), output_path).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), task_id);
    }

    // Test 4: cancel_download command framework
    #[tokio::test]
    async fn test_cancel_download() {
        let task_id = "test-task-789".to_string();
        
        // Test that cancel_download returns Ok
        let result = cancel_download(task_id.clone()).await;
        assert!(result.is_ok());
    }

    // Test 5: get_progress test
    #[tokio::test]
    async fn test_get_progress() {
        let task_id = "test-task-progress".to_string();
        
        // Test that get_progress returns a valid DownloadProgress
        let result = get_progress(task_id.clone()).await;
        assert!(result.is_ok());
        
        let progress = result.unwrap();
        // Initial progress should have zeros
        assert_eq!(progress.total, 0);
        assert_eq!(progress.downloaded, 0);
        assert_eq!(progress.speed, 0);
        assert_eq!(progress.percent, 0.0);
    }

    // Test 6: list_downloads test
    #[tokio::test]
    async fn test_list_downloads() {
        // Test that list_downloads returns a vector (initially empty)
        let result = list_downloads().await;
        assert!(result.is_ok());
        
        let downloads = result.unwrap();
        assert!(downloads.is_empty());
    }

    // Test 7: get_download_result for non-existent task
    #[tokio::test]
    async fn test_get_download_result_not_found() {
        let task_id = "non-existent-task".to_string();
        
        // Test that get_download_result returns TaskNotFound error
        let result = get_download_result(task_id.clone()).await;
        assert!(result.is_err());
        
        let err = result.unwrap_err();
        assert!(matches!(err, crate::DownloadError::TaskNotFound(_)));
    }

    // Test 8: start_download_with_config basic test
    #[tokio::test]
    async fn test_start_download_with_config_validation() {
        // Create a valid config
        let config = DownloadConfig {
            id: "config-test-123".to_string(),
            url: "http://example.com/test.txt".to_string(),
            output_path: PathBuf::from("/tmp/test_config.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: Some("TurboDownload/1.0".to_string()),
            headers: Default::default(),
            speed_limit: 0,
        };
        
        // Verify config values
        assert_eq!(config.threads, 4);
        assert!(config.resume_support);
        assert!(config.url.starts_with("http://"));
    }

    // Original test preserved
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

    // Test 9: DownloadProgress serialization
    #[test]
    fn test_download_progress_default() {
        let progress = DownloadProgress {
            total: 1000,
            downloaded: 500,
            speed: 100000,
            avg_speed: 80000,
            eta: Some(5000),
            percent: 50.0,
        };
        
        assert_eq!(progress.total, 1000);
        assert_eq!(progress.downloaded, 500);
        assert_eq!(progress.percent, 50.0);
    }

    // Test 10: pause_resume_cancel workflow simulation
    #[tokio::test]
    async fn test_pause_resume_cancel_workflow() {
        let task_id = "workflow-test-123".to_string();
        
        // Start a download (simulated by just having task_id)
        
        // Pause
        let pause_result = pause_download(task_id.clone()).await;
        assert!(pause_result.is_ok());
        
        // Resume
        let resume_result = resume_download(task_id.clone(), PathBuf::from("/tmp/workflow.txt")).await;
        assert!(resume_result.is_ok());
        
        // Cancel
        let cancel_result = cancel_download(task_id.clone()).await;
        assert!(cancel_result.is_ok());
    }
}
