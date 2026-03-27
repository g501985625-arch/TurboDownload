use serde::{Deserialize, Serialize};

/// Download status
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Download events for progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DownloadEvent {
    /// Download started
    Started {
        task_id: String,
        total_size: u64,
    },
    
    /// Progress update
    Progress {
        task_id: String,
        downloaded: u64,
        speed: u64,
        percent: f64,
        eta: Option<u64>,
    },
    
    /// Chunk completed
    ChunkCompleted {
        task_id: String,
        chunk_id: u32,
    },
    
    /// Download completed
    Completed {
        task_id: String,
        file_path: String,
    },
    
    /// Download failed
    Failed {
        task_id: String,
        error: String,
    },
    
    /// Download paused
    Paused {
        task_id: String,
    },
    
    /// Download resumed
    Resumed {
        task_id: String,
    },
    
    /// Download cancelled
    Cancelled {
        task_id: String,
    },
}

impl DownloadEvent {
    /// Get task ID from event
    pub fn task_id(&self) -> &str {
        match self {
            Self::Started { task_id, .. } => task_id,
            Self::Progress { task_id, .. } => task_id,
            Self::ChunkCompleted { task_id, .. } => task_id,
            Self::Completed { task_id, .. } => task_id,
            Self::Failed { task_id, .. } => task_id,
            Self::Paused { task_id, .. } => task_id,
            Self::Resumed { task_id, .. } => task_id,
            Self::Cancelled { task_id, .. } => task_id,
        }
    }
}
