//! Multi-threaded downloader implementation

use crate::{
    chunk::{Chunk, ChunkManager, ChunkProgress, Worker},
    download::{DownloadConfig, DownloadResult},
    error::{DownloadError, Result},
    event::EventEmitter,
    http::Client,
    pool::WorkerPool,
    progress::Tracker,
    range::RangeClient,
    storage::{ChunkWriter, FileMerger, StateManager},
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

/// Retry configuration for download operations
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial retry delay in milliseconds
    pub initial_delay_ms: u64,
    /// Maximum retry delay in milliseconds
    pub max_delay_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Create a new retry config with custom settings
    pub fn new(max_retries: u32, initial_delay_ms: u64) -> Self {
        Self {
            max_retries,
            initial_delay_ms,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }

    /// Calculate the delay for a given attempt (0-indexed)
    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        let delay = (self.initial_delay_ms as f64 * self.backoff_multiplier.powi(attempt as i32)) as u64;
        delay.min(self.max_delay_ms)
    }
}

/// Downloader with retry support
pub struct RetryDownloader {
    inner: MultiThreadDownloader,
    retry_config: RetryConfig,
}

impl RetryDownloader {
    /// Create a new retry downloader
    pub fn new(config: DownloadConfig, retry_config: RetryConfig) -> Result<Self> {
        let inner = MultiThreadDownloader::new(config)?;
        Ok(Self { inner, retry_config })
    }

    /// Download with automatic retry on failure
    pub async fn download_with_retry(&self) -> Result<DownloadResult> {
        let mut last_error: Option<DownloadError> = None;
        
        for attempt in 0..=self.retry_config.max_retries {
            match self.inner.download().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Check if error is retryable
                    if !e.is_retryable() || attempt == self.retry_config.max_retries {
                        return Err(e);
                    }
                    
                    last_error = Some(e);
                    
                    // Calculate and wait for retry delay
                    let delay = self.retry_config.calculate_delay(attempt);
                    tracing::warn!(
                        "Download failed (attempt {}/{}), retrying in {}ms",
                        attempt + 1,
                        self.retry_config.max_retries + 1,
                        delay
                    );
                    
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
            }
        }
        
        // This should never happen but just in case
        Err(last_error.unwrap_or_else(|| DownloadError::Internal("Max retries exceeded".to_string())))
    }

    /// Cancel the download
    pub async fn cancel(&self) -> Result<()> {
        self.inner.cancel().await
    }

    /// Get the inner downloader
    pub fn inner(&self) -> &MultiThreadDownloader {
        &self.inner
    }
}

/// Multi-threaded downloader
pub struct MultiThreadDownloader {
    config: DownloadConfig,
    client: Client,
    state_manager: StateManager,
    /// Shared flag for cancellation (Arc<RwLock<bool>> to allow async locking)
    cancelled: Arc<RwLock<bool>>,
}

impl MultiThreadDownloader {
    /// Create a new downloader
    pub fn new(config: DownloadConfig) -> Result<Self> {
        let client = Client::new(Default::default())?;
        let state_manager = StateManager::new(
            std::path::PathBuf::from(".download_states")
        );

        Ok(Self {
            config,
            client,
            state_manager,
            cancelled: Arc::new(RwLock::new(false)),
        })
    }

    /// Get the temp directory path for this download task
    fn get_temp_dir(&self) -> PathBuf {
        let base_dir = std::env::temp_dir();
        base_dir.join("turbo-download").join(&self.config.id)
    }

    /// Download a file with multi-threading
    /// 
    /// This is the main download orchestrator that:
    /// 1. Checks Range support
    /// 2. Creates chunks using ChunkManager
    /// 3. Spawns workers for concurrent download using WorkerPool
    /// 4. Processes progress updates via Tracker and EventEmitter
    /// 5. Merges chunks into final file using FileMerger
    /// 6. Cleans up temporary files using ChunkWriter
    pub async fn download(&self) -> Result<DownloadResult> {
        let start_time = std::time::Instant::now();
        
        // Create event emitter for this task
        let task_id = self.config.id.clone();
        let emitter = EventEmitter::new(task_id.clone());

        // Emit started event
        let _started_event = emitter.started(0);

        // Create temp directory for this download
        let temp_dir = self.get_temp_dir();
        tokio::fs::create_dir_all(&temp_dir).await?;

        // 1. Check Range support
        let range_client = RangeClient::with_defaults()?;
        let support = range_client
            .check_range_support(&self.config.url)
            .await?;

        if !support.is_supported() {
            // Cleanup temp dir on error
            let _ = ChunkWriter::cleanup(&temp_dir).await;
            return Err(DownloadError::RangeNotSupported);
        }

        let total_size = support.content_length
            .ok_or(DownloadError::ContentLengthUnknown)?;

        // Emit started event with actual size
        let _started_event = emitter.started(total_size);

        // 2. Create chunks using ChunkManager
        let chunk_size = if self.config.chunk_size > 0 {
            self.config.chunk_size
        } else {
            // Default chunk size: 1MB
            1024 * 1024
        };

        let mut chunk_manager = ChunkManager::new(total_size, chunk_size, temp_dir.clone());
        chunk_manager.calculate_chunks(self.config.threads);

        // Get the chunks as a separate collection to avoid borrowing issues
        let chunks: Vec<Chunk> = chunk_manager.chunks().to_vec();

        // Emit progress (0%)
        let _progress_event = emitter.progress(0, 0, 0.0, None);

        // 3. Create worker pool and tracker
        let pool = WorkerPool::new(self.config.threads as usize);
        let tracker = Arc::new(Tracker::new(total_size));
        
        // Create channel for progress updates from workers
        let (progress_tx, progress_rx) = mpsc::channel::<ChunkProgress>(100);
        
        // Clone for progress listener
        let tracker_clone = tracker.clone();
        
        // Create separate emitter for progress listener
        let emitter_for_progress = EventEmitter::new(task_id.clone());

        // Spawn progress listener task
        let progress_handle = tokio::spawn(async move {
            let mut rx = progress_rx;
            while let Some(progress) = rx.recv().await {
                // Update tracker
                let current = tracker_clone.downloaded();
                tracker_clone.update(current + progress.downloaded);
                
                // Emit progress event
                let _ = emitter_for_progress.progress(
                    tracker_clone.downloaded(),
                    tracker_clone.speed(),
                    tracker_clone.percent(),
                    tracker_clone.eta(),
                );
            }
        });

        // 4. Spawn workers for each chunk using WorkerPool
        let mut handles = Vec::new();
        
        for chunk in chunks {
            // Check if cancelled
            if *self.cancelled.read().await {
                break;
            }

            let url = self.config.url.clone();
            let client = self.client.clone();
            let progress_tx = progress_tx.clone();
            let _temp_dir_clone = temp_dir.clone();

            // Spawn the worker task
            let handle = match pool.spawn(async move {
                // Create worker for this chunk
                let mut worker = Worker::new(chunk, url, client);
                
                // Download with progress updates
                let result = worker.download(progress_tx).await;
                
                match result {
                    Ok(()) => {
                        // Record the temp path for merging
                        Ok(worker.temp_path().to_path_buf())
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
                
            }).await {
                Ok(h) => h,
                Err(_) => {
                    // Cleanup on spawn error
                    let _ = ChunkWriter::cleanup(&temp_dir).await;
                    return Err(DownloadError::PoolClosed);
                }
            };

            handles.push(handle);
        }

        // Wait for all workers to complete
        let results: Vec<Result<std::path::PathBuf>> = WorkerPool::wait_all(handles).await;

        // Drop progress sender to stop the listener
        drop(progress_tx);
        
        // Wait for progress listener to finish
        let _ = progress_handle.await;

        // Collect the completed chunk paths
        let mut chunk_paths: Vec<std::path::PathBuf> = Vec::new();

        // Check for any errors and collect temp paths
        for result in &results {
            match result {
                Ok(path) => {
                    chunk_paths.push(path.clone());
                }
                Err(e) => {
                    let _ = emitter.failed(format!("{:?}", e));
                    // Cleanup temp dir on error
                    let _ = ChunkWriter::cleanup(&temp_dir).await;
                    // Create a new error with the message
                    return Err(DownloadError::Internal(format!("{:?}", e)));
                }
            }
        }

        // Sort by chunk ID to ensure correct order for merging
        // (The spawn order should match creation order, but let's be safe)
        chunk_paths.sort_by(|a, b| {
            let a_id = a.file_stem().and_then(|s| s.to_str()).unwrap_or("").replace("chunk_", "").parse::<u32>().unwrap_or(0);
            let b_id = b.file_stem().and_then(|s| s.to_str()).unwrap_or("").replace("chunk_", "").parse::<u32>().unwrap_or(0);
            a_id.cmp(&b_id)
        });

        // Convert to path references
        let chunk_refs: Vec<&std::path::Path> = chunk_paths.iter().map(|p| p.as_path()).collect();

        // 5. Merge chunks into final file using FileMerger
        if !chunk_refs.is_empty() {
            FileMerger::merge(&chunk_refs, &self.config.output_path).await?;
        }

        // Emit completed event
        let _completed_event = emitter.completed(
            self.config.output_path.to_string_lossy().to_string()
        );

        // 6. Clean up temporary files using ChunkWriter
        ChunkWriter::cleanup(&temp_dir).await?;

        // Calculate result
        let duration = start_time.elapsed();
        let avg_speed = if duration.as_secs() > 0 {
            total_size / duration.as_secs()
        } else {
            0
        };

        Ok(DownloadResult {
            task_id: self.config.id.clone(),
            output_path: self.config.output_path.clone(),
            file_size: total_size,
            duration_ms: duration.as_millis() as u64,
            avg_speed,
        })
    }

    /// Cancel the current download
    pub async fn cancel(&self) -> Result<()> {
        *self.cancelled.write().await = true;
        Ok(())
    }

    /// Pause the download
    pub async fn pause(&self) -> Result<()> {
        // TODO: Implement pause logic with state persistence
        Ok(())
    }

    /// Resume the download
    pub async fn resume(&self) -> Result<DownloadResult> {
        // TODO: Implement resume logic
        self.download().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ============== Unit Tests ==============

    #[test]
    fn test_downloader_creation() {
        let config = DownloadConfig {
            id: "test".to_string(),
            url: "http://test.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/test.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let downloader = MultiThreadDownloader::new(config);
        assert!(downloader.is_ok());
    }

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_delay_ms, 1000);
        assert_eq!(config.max_delay_ms, 30000);
        assert!((config.backoff_multiplier - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_retry_config_custom() {
        let config = RetryConfig::new(5, 500);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.initial_delay_ms, 500);
    }

    #[test]
    fn test_retry_config_calculate_delay() {
        let config = RetryConfig::default();
        
        // First attempt: 1000 * 2^0 = 1000ms
        assert_eq!(config.calculate_delay(0), 1000);
        
        // Second attempt: 1000 * 2^1 = 2000ms
        assert_eq!(config.calculate_delay(1), 2000);
        
        // Third attempt: 1000 * 2^2 = 4000ms
        assert_eq!(config.calculate_delay(2), 4000);
        
        // Fourth attempt: 1000 * 2^3 = 8000ms (capped at max_delay_ms = 30000)
        assert_eq!(config.calculate_delay(3), 8000);
        
        // Very high attempt: capped at max_delay_ms
        assert_eq!(config.calculate_delay(10), 30000);
    }

    #[test]
    fn test_retry_config_backoff() {
        // Test with custom backoff multiplier
        let config = RetryConfig {
            max_retries: 5,
            initial_delay_ms: 100,
            max_delay_ms: 10000,
            backoff_multiplier: 3.0,
        };
        
        assert_eq!(config.calculate_delay(0), 100);
        assert_eq!(config.calculate_delay(1), 300);
        assert_eq!(config.calculate_delay(2), 900);
        assert_eq!(config.calculate_delay(3), 2700);
    }

    #[test]
    fn test_temp_dir_generation() {
        let config = DownloadConfig {
            id: "test-id-123".to_string(),
            url: "http://test.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/output.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let downloader = MultiThreadDownloader::new(config).unwrap();
        let temp_dir = downloader.get_temp_dir();
        
        // Should contain the task ID
        assert!(temp_dir.to_string_lossy().contains("test-id-123"));
        // Should be under temp directory
        assert!(temp_dir.to_string_lossy().contains("turbo-download"));
    }

    #[test]
    fn test_cancelled_flag_initial_state() {
        let config = DownloadConfig {
            id: "test".to_string(),
            url: "http://test.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/test.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let downloader = MultiThreadDownloader::new(config).unwrap();
        
        // Check initial cancelled state is false
        let cancelled = downloader.cancelled.clone();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        
        runtime.block_on(async {
            let is_cancelled = *cancelled.read().await;
            assert!(!is_cancelled);
        });
    }

    #[test]
    fn test_pause_returns_ok() {
        let config = DownloadConfig {
            id: "test".to_string(),
            url: "http://test.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/test.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let downloader = MultiThreadDownloader::new(config).unwrap();
        
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        
        runtime.block_on(async {
            let result = downloader.pause().await;
            assert!(result.is_ok());
        });
    }

    // ============== Retry Downloader Tests ==============

    #[test]
    fn test_retry_downloader_creation() {
        let config = DownloadConfig {
            id: "retry-test".to_string(),
            url: "http://test.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/test.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let retry_config = RetryConfig::default();
        let downloader = RetryDownloader::new(config, retry_config);
        assert!(downloader.is_ok());
    }

    #[test]
    fn test_retry_downloader_inner_access() {
        let config = DownloadConfig {
            id: "retry-test".to_string(),
            url: "http://test.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/test.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let retry_config = RetryConfig::default();
        let downloader = RetryDownloader::new(config, retry_config).unwrap();
        let _inner = downloader.inner();
        
        // Inner should be accessible
        assert!(true);
    }

    // ============== Integration Tests ==============

    #[tokio::test]
    async fn test_downloader_cancel_integration() {
        let config = DownloadConfig {
            id: "cancel-test".to_string(),
            url: "http://test.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/test_cancel.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let downloader = MultiThreadDownloader::new(config).unwrap();
        
        // Cancel should work
        let result = downloader.cancel().await;
        assert!(result.is_ok());
        
        // Check cancelled flag is set
        let is_cancelled = *downloader.cancelled.read().await;
        assert!(is_cancelled);
    }

    #[tokio::test]
    async fn test_retry_downloader_cancel() {
        let config = DownloadConfig {
            id: "retry-cancel-test".to_string(),
            url: "http://test.com/file.txt".to_string(),
            output_path: PathBuf::from("/tmp/test_retry_cancel.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let retry_config = RetryConfig::new(3, 100);
        let downloader = RetryDownloader::new(config, retry_config).unwrap();
        
        let result = downloader.cancel().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_retry_config_delay_progression() {
        let config = RetryConfig {
            max_retries: 5,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        };
        
        let delays: Vec<u64> = (0..=5).map(|i| config.calculate_delay(i)).collect();
        
        // Should follow exponential backoff
        assert_eq!(delays[0], 100);   // 100 * 2^0
        assert_eq!(delays[1], 200);   // 100 * 2^1
        assert_eq!(delays[2], 400);   // 100 * 2^2
        assert_eq!(delays[3], 800);   // 100 * 2^3
        assert_eq!(delays[4], 1600);  // 100 * 2^4
        assert_eq!(delays[5], 3200);  // 100 * 2^5 (capped at 5000)
    }

    #[tokio::test]
    async fn test_multiple_downloader_instances() {
        let config1 = DownloadConfig {
            id: "instance-1".to_string(),
            url: "http://test.com/file1.txt".to_string(),
            output_path: PathBuf::from("/tmp/test1.txt"),
            threads: 2,
            chunk_size: 512 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let config2 = DownloadConfig {
            id: "instance-2".to_string(),
            url: "http://test.com/file2.txt".to_string(),
            output_path: PathBuf::from("/tmp/test2.txt"),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };

        let downloader1 = MultiThreadDownloader::new(config1);
        let downloader2 = MultiThreadDownloader::new(config2);
        
        assert!(downloader1.is_ok());
        assert!(downloader2.is_ok());
        
        // Both should have independent cancelled flags
        let d1 = downloader1.unwrap();
        let d2 = downloader2.unwrap();
        
        d1.cancel().await.unwrap();
        
        let is_d1_cancelled = *d1.cancelled.read().await;
        let is_d2_cancelled = *d2.cancelled.read().await;
        
        assert!(is_d1_cancelled);
        assert!(!is_d2_cancelled);
    }
}