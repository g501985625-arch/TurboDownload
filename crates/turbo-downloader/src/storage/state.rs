use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::Result;
use crate::chunk::Chunk;

/// Chunk state information for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkStateInfo {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub temp_path: PathBuf,
}

/// Download state for resume support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadState {
    pub task_id: String,
    pub url: String,
    pub output_path: PathBuf,
    pub total_size: u64,
    pub downloaded: u64,
    pub chunks: Vec<ChunkStateInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DownloadState {
    /// Create state from chunks
    pub fn from_chunks(
        task_id: String,
        url: String,
        output_path: PathBuf,
        total_size: u64,
        chunks: &[Chunk],
    ) -> Self {
        let now = Utc::now();
        
        Self {
            task_id,
            url,
            output_path,
            total_size,
            downloaded: chunks.iter().map(|c| c.downloaded).sum(),
            chunks: chunks.iter().map(|c| ChunkStateInfo {
                id: c.id,
                start: c.start,
                end: c.end,
                downloaded: c.downloaded,
                temp_path: c.temp_path.clone(),
            }).collect(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Check if download is complete
    pub fn is_complete(&self) -> bool {
        self.downloaded >= self.total_size
    }
    
    /// Get progress percentage
    pub fn progress_percent(&self) -> f64 {
        if self.total_size == 0 {
            return 0.0;
        }
        (self.downloaded as f64 / self.total_size as f64) * 100.0
    }
}

/// State manager for persistence
pub struct StateManager {
    state_dir: PathBuf,
}

impl StateManager {
    /// Create a new state manager
    pub fn new(state_dir: PathBuf) -> Self {
        Self { state_dir }
    }
    
    /// Get state file path for a task
    fn state_path(&self, task_id: &str) -> PathBuf {
        self.state_dir.join(format!("{}.json", task_id))
    }
    
    /// Save download state
    pub async fn save(&self, state: &DownloadState) -> Result<()> {
        tokio::fs::create_dir_all(&self.state_dir).await?;
        
        let path = self.state_path(&state.task_id);
        let json = serde_json::to_string_pretty(state)?;
        
        tokio::fs::write(&path, json).await?;
        Ok(())
    }
    
    /// Load download state
    pub async fn load(&self, task_id: &str) -> Result<Option<DownloadState>> {
        let path = self.state_path(task_id);
        
        if !path.exists() {
            return Ok(None);
        }
        
        let json = tokio::fs::read_to_string(&path).await?;
        let state: DownloadState = serde_json::from_str(&json)?;
        Ok(Some(state))
    }
    
    /// Delete state file
    pub async fn delete(&self, task_id: &str) -> Result<()> {
        let path = self.state_path(task_id);
        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_download_state_progress() {
        let state = DownloadState {
            task_id: "test".to_string(),
            url: "http://test.com".to_string(),
            output_path: PathBuf::from("/tmp/test"),
            total_size: 1000,
            downloaded: 500,
            chunks: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        assert_eq!(state.progress_percent(), 50.0);
        assert!(!state.is_complete());
    }
}
