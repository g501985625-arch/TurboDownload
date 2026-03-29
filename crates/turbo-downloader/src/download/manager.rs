use super::{DownloadConfig, Task};
use crate::{
    error::{DownloadError, Result},
    http::Client,
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// Download manager
pub struct Manager {
    client: Client,
    tasks: RwLock<HashMap<String, Arc<Task>>>,
    #[allow(dead_code)]
    max_concurrent: usize,
}

impl Manager {
    pub fn new(client: Client, max_concurrent: usize) -> Self {
        Self {
            client,
            tasks: RwLock::new(HashMap::new()),
            max_concurrent,
        }
    }

    /// Create download task
    pub async fn create_task(&self, config: DownloadConfig) -> Result<String> {
        let task_id = config.id.clone();

        // Check if already exists
        {
            let tasks = self.tasks.read();
            if tasks.contains_key(&task_id) {
                return Err(DownloadError::TaskNotFound(task_id));
            }
        }

        // Get file info
        let head = self.client.head(&config.url).await?;
        let file_size = head
            .content_length
            .ok_or(DownloadError::RangeNotSupported)?;

        // Create task
        let task = Arc::new(Task::new(config, file_size));

        {
            let mut tasks = self.tasks.write();
            tasks.insert(task_id.clone(), task);
        }

        Ok(task_id)
    }

    /// Get task
    pub fn get_task(&self, task_id: &str) -> Option<Arc<Task>> {
        let tasks = self.tasks.read();
        tasks.get(task_id).cloned()
    }

    /// List all tasks
    pub fn list_tasks(&self) -> Vec<String> {
        let tasks = self.tasks.read();
        tasks.keys().cloned().collect()
    }

    /// Remove task
    pub fn remove_task(&self, task_id: &str) -> Result<()> {
        let mut tasks = self.tasks.write();
        tasks
            .remove(task_id)
            .ok_or(DownloadError::TaskNotFound(task_id.to_string()))?;
        Ok(())
    }
}

/// Downloader
pub struct Downloader {
    manager: Manager,
}

impl Downloader {
    pub fn manager(&self) -> &Manager {
        &self.manager
    }
}

/// Downloader builder
pub struct DownloaderBuilder {
    max_concurrent_tasks: usize,
    default_threads: u32,
    timeout: std::time::Duration,
}

impl DownloaderBuilder {
    pub fn new() -> Self {
        Self {
            max_concurrent_tasks: 3,
            default_threads: 4,
            timeout: std::time::Duration::from_secs(300),
        }
    }

    pub fn max_concurrent_tasks(mut self, count: usize) -> Self {
        self.max_concurrent_tasks = count;
        self
    }

    pub fn default_threads(mut self, threads: u32) -> Self {
        self.default_threads = threads;
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = std::time::Duration::from_secs(secs);
        self
    }

    pub fn build(self) -> Result<Downloader> {
        let client = Client::new(crate::http::ClientConfig {
            timeout: self.timeout,
            ..Default::default()
        })?;

        Ok(Downloader {
            manager: Manager::new(client, self.max_concurrent_tasks),
        })
    }
}

impl Default for DownloaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
