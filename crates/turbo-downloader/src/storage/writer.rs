use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use crate::Result;

/// Chunk writer for writing data to temporary files
pub struct ChunkWriter;

impl ChunkWriter {
    /// Create a new chunk writer
    pub fn new() -> Self {
        Self
    }
    
    /// Write data to a file (creates or truncates)
    pub async fn write(&self, path: &Path, data: &[u8]) -> Result<()> {
        let mut file = File::create(path).await?;
        file.write_all(data).await?;
        file.flush().await?;
        Ok(())
    }
    
    /// Append data to a file (creates if not exists)
    pub async fn append(&self, path: &Path, data: &[u8]) -> Result<()> {
        let mut file = File::options()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        file.write_all(data).await?;
        file.flush().await?;
        Ok(())
    }
    
    /// Create temporary directory for a task
    pub async fn create_temp_dir(base: &Path, task_id: &str) -> Result<std::path::PathBuf> {
        let temp_dir = base.join("temp").join(task_id);
        tokio::fs::create_dir_all(&temp_dir).await?;
        Ok(temp_dir)
    }
    
    /// Cleanup temporary directory
    pub async fn cleanup(temp_dir: &Path) -> Result<()> {
        if temp_dir.exists() {
            tokio::fs::remove_dir_all(temp_dir).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_write_and_read() {
        let writer = ChunkWriter::new();
        let temp_path = PathBuf::from("/tmp/test_chunk_writer.txt");
        let data = b"Hello, World!";
        
        writer.write(&temp_path, data).await.unwrap();
        
        let read_data = tokio::fs::read(&temp_path).await.unwrap();
        assert_eq!(read_data, data);
        
        // Cleanup
        let _ = tokio::fs::remove_file(&temp_path).await;
    }
}
