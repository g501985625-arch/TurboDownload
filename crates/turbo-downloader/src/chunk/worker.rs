use super::Chunk;
use crate::{http::Client, Result};
use std::path::Path;
use tokio::io::AsyncWriteExt;
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

    /// Execute chunk download with retry
    pub async fn download(&mut self, progress_tx: mpsc::Sender<ChunkProgress>) -> Result<()> {
        self.download_with_retry(progress_tx, 3).await
    }

    /// Execute chunk download with configurable retry
    pub async fn download_with_retry(
        &mut self,
        progress_tx: mpsc::Sender<ChunkProgress>,
        max_retries: u32,
    ) -> Result<()> {
        let mut retries = 0;

        loop {
            match self.try_download(&progress_tx).await {
                Ok(()) => return Ok(()),
                Err(_e) if retries < max_retries => {
                    retries += 1;
                    let delay = std::time::Duration::from_secs(2u64.pow(retries));
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Try to download chunk (single attempt)
    async fn try_download(&mut self, progress_tx: &mpsc::Sender<ChunkProgress>) -> Result<()> {
        use tokio::fs::OpenOptions;

        // Open file for append (resume support)
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.chunk.temp_path)
            .await?;

        let mut downloaded = self.chunk.downloaded;
        let buffer_size = 64 * 1024; // 64KB chunks

        while downloaded < self.chunk.size() {
            let start = self.chunk.start + downloaded;
            let end = (start + buffer_size).min(self.chunk.end);

            let bytes = self.client.get_range(&self.url, start..end).await?;
            file.write_all(&bytes).await?;

            downloaded += bytes.len() as u64;
            self.chunk.downloaded = downloaded;

            // Send progress update (ignore send error if receiver is dropped)
            let _ = progress_tx
                .try_send(ChunkProgress {
                    chunk_id: self.chunk.id,
                    downloaded,
                    total: self.chunk.size(),
                });
        }

        file.flush().await?;
        Ok(())
    }

    pub fn temp_path(&self) -> &Path {
        &self.chunk.temp_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;

    #[test]
    fn test_chunk_progress_creation() {
        let progress = ChunkProgress {
            chunk_id: 1,
            downloaded: 100,
            total: 1000,
        };
        assert_eq!(progress.chunk_id, 1);
        assert_eq!(progress.downloaded, 100);
        assert_eq!(progress.total, 1000);
    }

    #[test]
    fn test_worker_creation() {
        let temp_dir = std::env::temp_dir();
        let chunk = Chunk::new(0, 0, 1024, &temp_dir);
        let client = Client::new(Default::default()).unwrap();
        let worker = Worker::new(chunk, "http://example.com/file.txt".to_string(), client);
        
        assert_eq!(worker.chunk_id(), 0);
    }
}
