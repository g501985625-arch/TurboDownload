use super::ResumeState;
use crate::{
    chunk::Chunk,
    error::{DownloadError, Result},
    http::Client,
};

/// Resume recovery
pub struct Recovery {
    client: Client,
}

impl Recovery {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Try to recover download
    pub async fn try_recover(&self, state: ResumeState) -> Result<Option<Vec<Chunk>>> {
        // Verify ETag
        if let Some(ref etag) = state.etag {
            let head = self.client.head(&state.url).await?;
            if head.etag.as_ref() != Some(etag) {
                return Err(DownloadError::ValidationFailed("ETag mismatch".to_string()));
            }
        }

        // Convert to chunks
        let temp_dir = state.output_path.parent().unwrap_or(std::path::Path::new(".")).to_path_buf();
        let chunks: Vec<Chunk> = state
            .chunks
            .into_iter()
            .filter_map(|c| {
                if c.downloaded < (c.end - c.start) {
                    Some(Chunk::new(c.id, c.start + c.downloaded, c.end, &temp_dir))
                } else {
                    None
                }
            })
            .collect();

        if chunks.is_empty() {
            Ok(None)
        } else {
            Ok(Some(chunks))
        }
    }
}
