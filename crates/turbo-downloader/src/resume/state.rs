use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Chunk resume state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResumeState {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub temp_path: PathBuf,
}

/// Task resume state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeState {
    pub task_id: String,
    pub url: String,
    pub file_size: u64,
    pub etag: Option<String>,
    pub downloaded: u64,
    pub chunks: Vec<ChunkResumeState>,
    pub output_path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResumeState {
    pub async fn save(&self, path: &std::path::Path) -> crate::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    pub async fn load(path: &std::path::Path) -> crate::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }

        let json = tokio::fs::read_to_string(path).await?;
        let state: Self = serde_json::from_str(&json)?;
        Ok(Some(state))
    }
}
