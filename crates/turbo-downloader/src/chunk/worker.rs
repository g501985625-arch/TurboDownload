use super::Chunk;
use crate::{http::Client, Result};
use std::path::Path;
use tokio::sync::mpsc;

/// Chunk progress message
#[derive(Debug, Clone)]
pub struct ChunkProgress {
    pub chunk_id: u32,
    pub downloaded: u64,
    pub total: u64,
}

/// Chunk download worker
pub struct Worker {
    chunk: Chunk,
    url: String,
    client: Client,
}

impl Worker {
    pub fn new(chunk: Chunk, url: String, client: Client) -> Self {
        Self {
            chunk,
            url,
            client,
        }
    }

    pub fn chunk_id(&self) -> u32 {
        self.chunk.id
    }

    /// Execute chunk download
    pub async fn download(&mut self, progress_tx: mpsc::Sender<ChunkProgress>) -> Result<()> {
        use tokio::fs::File;
        use tokio::io::AsyncWriteExt;

        let mut file = File::create(&self.chunk.temp_path).await?;
        let mut downloaded = self.chunk.downloaded;

        while downloaded < self.chunk.size() {
            let start = self.chunk.start + downloaded;
            let end = (start + 64 * 1024).min(self.chunk.end);
            let range = start..end;

            let bytes = self.client.get_range(&self.url, range).await?;
            file.write_all(&bytes).await?;

            downloaded += bytes.len() as u64;
            self.chunk.downloaded = downloaded;

            let _ = progress_tx
                .send(ChunkProgress {
                    chunk_id: self.chunk.id,
                    downloaded,
                    total: self.chunk.size(),
                })
                .await;
        }

        file.flush().await?;
        Ok(())
    }

    pub fn temp_path(&self) -> &Path {
        &self.chunk.temp_path
    }
}
