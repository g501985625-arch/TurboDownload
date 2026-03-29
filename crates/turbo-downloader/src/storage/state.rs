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
    use crate::chunk::{Chunk, ChunkState};
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

    #[test]
    fn test_download_state_complete() {
        let state = DownloadState {
            task_id: "test".to_string(),
            url: "http://test.com".to_string(),
            output_path: PathBuf::from("/tmp/test"),
            total_size: 1000,
            downloaded: 1000,
            chunks: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        assert_eq!(state.progress_percent(), 100.0);
        assert!(state.is_complete());
    }

    #[test]
    fn test_from_chunks() {
        let temp_dir = PathBuf::from("/tmp");
        let chunks = vec![
            Chunk {
                id: 0,
                start: 0,
                end: 500,
                downloaded: 500,
                state: ChunkState::Completed,
                temp_path: temp_dir.join("chunk_0.tmp"),
            },
            Chunk {
                id: 1,
                start: 500,
                end: 1000,
                downloaded: 250,
                state: ChunkState::Downloading,
                temp_path: temp_dir.join("chunk_1.tmp"),
            },
        ];
        
        let state = DownloadState::from_chunks(
            "task_123".to_string(),
            "http://example.com/file.zip".to_string(),
            PathBuf::from("/downloads/file.zip"),
            1000,
            &chunks,
        );
        
        assert_eq!(state.task_id, "task_123");
        assert_eq!(state.url, "http://example.com/file.zip");
        assert_eq!(state.total_size, 1000);
        assert_eq!(state.downloaded, 750);
        assert_eq!(state.chunks.len(), 2);
        assert_eq!(state.chunks[0].id, 0);
        assert_eq!(state.chunks[1].id, 1);
    }

    #[tokio::test]
    async fn test_state_manager_save_and_load() {
        let state_dir = PathBuf::from("/tmp/test_state_manager");
        let manager = StateManager::new(state_dir.clone());
        
        // Create state
        let state = DownloadState {
            task_id: "test_task".to_string(),
            url: "http://test.com".to_string(),
            output_path: PathBuf::from("/tmp/output"),
            total_size: 1000,
            downloaded: 500,
            chunks: vec![
                ChunkStateInfo {
                    id: 0,
                    start: 0,
                    end: 500,
                    downloaded: 500,
                    temp_path: PathBuf::from("/tmp/chunk_0.tmp"),
                },
            ],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Save
        manager.save(&state).await.unwrap();
        
        // Load
        let loaded = manager.load("test_task").await.unwrap();
        assert!(loaded.is_some());
        
        let loaded_state = loaded.unwrap();
        assert_eq!(loaded_state.task_id, "test_task");
        assert_eq!(loaded_state.total_size, 1000);
        assert_eq!(loaded_state.downloaded, 500);
        assert_eq!(loaded_state.chunks.len(), 1);
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&state_dir).await;
    }

    #[tokio::test]
    async fn test_state_manager_load_nonexistent() {
        let state_dir = PathBuf::from("/tmp/test_state_manager_2");
        let manager = StateManager::new(state_dir.clone());
        
        // Load non-existent state
        let loaded = manager.load("nonexistent_task").await.unwrap();
        assert!(loaded.is_none());
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&state_dir).await;
    }

    #[tokio::test]
    async fn test_state_manager_delete() {
        let state_dir = PathBuf::from("/tmp/test_state_manager_3");
        let manager = StateManager::new(state_dir.clone());
        
        // Create and save state
        let state = DownloadState {
            task_id: "delete_test".to_string(),
            url: "http://test.com".to_string(),
            output_path: PathBuf::from("/tmp/output"),
            total_size: 100,
            downloaded: 50,
            chunks: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        manager.save(&state).await.unwrap();
        assert!(manager.load("delete_test").await.unwrap().is_some());
        
        // Delete
        manager.delete("delete_test").await.unwrap();
        assert!(manager.load("delete_test").await.unwrap().is_none());
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&state_dir).await;
    }
}
