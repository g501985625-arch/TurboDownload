//! Download Manager Service
//! 
//! Manages multiple download tasks with state persistence

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{DownloadConfig, DownloadProgress, DownloadStatus, DownloadTask, Result, AppError};
use super::http_downloader::HttpDownloader;

/// Download manager state
pub struct DownloadManager {
    /// Active download tasks
    tasks: Arc<RwLock<HashMap<String, DownloadTask>>>,
    /// Cancel flags for each task
    cancel_flags: Arc<RwLock<HashMap<String, Arc<RwLock<bool>>>>>,
    /// HTTP downloader
    downloader: HttpDownloader,
}

impl DownloadManager {
    /// Create a new download manager
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            cancel_flags: Arc::new(RwLock::new(HashMap::new())),
            downloader: HttpDownloader::new(),
        }
    }

    /// Add a new download task
    pub async fn add_task(&self, url: String, config: DownloadConfig) -> Result<String> {
        // Validate URL
        let parsed = url::Url::parse(&url)
            .map_err(|e| AppError::InvalidUrl(format!("Invalid URL: {}", e)))?;
        
        if parsed.scheme() != "http" && parsed.scheme() != "https" {
            return Err(AppError::InvalidUrl("Only HTTP/HTTPS URLs are supported".to_string()));
        }

        // Create task
        let task = DownloadTask::new(url, config);
        let task_id = task.id.clone();

        // Check for duplicates
        let tasks = self.tasks.read().await;
        if tasks.contains_key(&task_id) {
            return Err(AppError::TaskExists(task_id));
        }
        drop(tasks);

        // Store task
        self.tasks.write().await.insert(task_id.clone(), task.clone());

        // Create cancel flag
        self.cancel_flags.write().await.insert(task_id.clone(), Arc::new(RwLock::new(false)));

        Ok(task_id)
    }

    /// Start a download task
    pub async fn start_task(&self, task_id: &str) -> Result<()> {
        let task = {
            let tasks = self.tasks.read().await;
            tasks.get(task_id).cloned()
                .ok_or_else(|| AppError::TaskNotFound(task_id.to_string()))?
        };

        // Update status
        {
            let mut tasks = self.tasks.write().await;
            if let Some(t) = tasks.get_mut(task_id) {
                t.status = DownloadStatus::Downloading;
            }
        }

        // Get cancel flag
        let cancel_flag = {
            let flags = self.cancel_flags.read().await;
            flags.get(task_id).cloned()
                .ok_or_else(|| AppError::TaskNotFound(task_id.to_string()))?
        };

        // Reset cancel flag
        *cancel_flag.write().await = false;

        // Build output path
        let output_path = task.output_dir.join(&task.filename);
        let url = task.url.clone();
        let config = task.config.clone();
        let task_id_owned = task_id.to_string();
        let tasks = self.tasks.clone();

        // Spawn download task
        tokio::spawn(async move {
            let downloader = HttpDownloader::new();
            let result = downloader.download(&url, output_path, &config, cancel_flag, task_id_owned.clone()).await;
            
            // Update task status
            let mut tasks = tasks.write().await;
            if let Some(t) = tasks.get_mut(&task_id_owned) {
                match result {
                    Ok(()) => {
                        t.status = DownloadStatus::Completed;
                        t.progress = 100.0;
                        t.completed_at = Some(chrono::Utc::now());
                    }
                    Err(e) => {
                        // Check if cancelled
                        if e.to_string().contains("cancelled") {
                            t.status = DownloadStatus::Cancelled;
                        } else {
                            t.status = DownloadStatus::Failed;
                            t.error = Some(e.to_string());
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Pause a download task
    pub async fn pause_task(&self, task_id: &str) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        let task = tasks.get_mut(task_id)
            .ok_or_else(|| AppError::TaskNotFound(task_id.to_string()))?;
        
        if task.status != DownloadStatus::Downloading {
            return Err(AppError::DownloadError("Task is not downloading".to_string()));
        }

        // Set cancel flag (will stop the download)
        if let Some(flag) = self.cancel_flags.read().await.get(task_id) {
            *flag.write().await = true;
        }

        task.status = DownloadStatus::Paused;
        Ok(())
    }

    /// Resume a paused task
    pub async fn resume_task(&self, task_id: &str) -> Result<()> {
        let tasks = self.tasks.read().await;
        let task = tasks.get(task_id)
            .ok_or_else(|| AppError::TaskNotFound(task_id.to_string()))?;
        
        if task.status != DownloadStatus::Paused {
            return Err(AppError::DownloadError("Task is not paused".to_string()));
        }

        drop(tasks);
        self.start_task(task_id).await
    }

    /// Cancel a download task
    pub async fn cancel_task(&self, task_id: &str) -> Result<()> {
        // Set cancel flag
        if let Some(flag) = self.cancel_flags.read().await.get(task_id) {
            *flag.write().await = true;
        }

        // Update status
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = DownloadStatus::Cancelled;
        }

        Ok(())
    }

    /// Remove a task
    pub async fn remove_task(&self, task_id: &str) -> Result<()> {
        // Cancel first if downloading
        self.cancel_task(task_id).await.ok();
        
        // Remove from maps
        self.tasks.write().await.remove(task_id);
        self.cancel_flags.write().await.remove(task_id);

        Ok(())
    }

    /// Get task by ID
    pub async fn get_task(&self, task_id: &str) -> Option<DownloadTask> {
        self.tasks.read().await.get(task_id).cloned()
    }

    /// Get all tasks
    pub async fn get_all_tasks(&self) -> Vec<DownloadTask> {
        self.tasks.read().await.values().cloned().collect()
    }

    /// Get task progress
    pub async fn get_progress(&self, task_id: &str) -> Result<DownloadProgress> {
        let task = self.tasks.read().await.get(task_id).cloned()
            .ok_or_else(|| AppError::TaskNotFound(task_id.to_string()))?;

        Ok(DownloadProgress {
            id: task.id,
            progress: task.progress,
            speed: task.speed,
            total_size: task.total_size,
            downloaded: task.downloaded,
            eta: None,
            status: task.status,
        })
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}