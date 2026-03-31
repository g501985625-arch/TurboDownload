use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Download configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub id: String,
    pub url: String,
    pub output_path: PathBuf,
    pub threads: u32,
    pub chunk_size: u64,
    pub resume_support: bool,
    pub user_agent: Option<String>,
    pub headers: std::collections::HashMap<String, String>,
    pub speed_limit: u64,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url: String::new(),
            output_path: PathBuf::from("."),
            threads: 4,
            chunk_size: 0,
            resume_support: true,
            user_agent: None,
            headers: std::collections::HashMap::new(),
            speed_limit: 0,
        }
    }
}

/// Download result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    pub task_id: String,
    pub output_path: PathBuf,
    pub file_size: u64,
    pub duration_ms: u64,
    pub avg_speed: u64,
}

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskState {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Download task
pub struct Task {
    pub id: String,
    pub config: DownloadConfig,
    state: parking_lot::Mutex<TaskState>,
    pub file_size: u64,
    pub downloaded: u64,
    speed: std::sync::atomic::AtomicU64,
}

impl Task {
    pub fn new(config: DownloadConfig, file_size: u64) -> Self {
        Self {
            id: config.id.clone(),
            config,
            state: parking_lot::Mutex::new(TaskState::Pending),
            file_size,
            downloaded: 0,
            speed: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn state(&self) -> TaskState {
        *self.state.lock()
    }

    pub fn set_state(&self, state: TaskState) {
        *self.state.lock() = state;
    }

    pub fn get_downloaded(&self) -> u64 {
        self.downloaded
    }

    pub fn set_downloaded(&mut self, downloaded: u64) {
        self.downloaded = downloaded;
    }

    pub fn speed(&self) -> u64 {
        self.speed.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_speed(&self, speed: u64) {
        // Note: AtomicU64::store takes &self, not &mut self
        self.speed.store(speed, std::sync::atomic::Ordering::Relaxed);
    }
}
