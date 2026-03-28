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
}