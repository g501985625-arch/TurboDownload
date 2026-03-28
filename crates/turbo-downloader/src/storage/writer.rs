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

    #[tokio::test]
    async fn test_append() {
        let writer = ChunkWriter::new();
        let temp_path = PathBuf::from("/tmp/test_chunk_append.txt");
        
        // First write
        writer.write(&temp_path, b"Hello").await.unwrap();
        
        // Append
        writer.append(&temp_path, b", World!").await.unwrap();
        
        let read_data = tokio::fs::read(&temp_path).await.unwrap();
        assert_eq!(read_data, b"Hello, World!");
        
        // Cleanup
        let _ = tokio::fs::remove_file(&temp_path).await;
    }

    #[tokio::test]
    async fn test_create_temp_dir() {
        let base = PathBuf::from("/tmp");
        let task_id = "test_task_123";
        
        let temp_dir = ChunkWriter::create_temp_dir(&base, task_id).await.unwrap();
        assert!(temp_dir.exists());
        assert_eq!(temp_dir, base.join("temp").join(task_id));
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_cleanup() {
        let temp_dir = PathBuf::from("/tmp/test_cleanup_dir");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        tokio::fs::write(temp_dir.join("file.txt"), b"test").await.unwrap();
        
        assert!(temp_dir.exists());
        
        ChunkWriter::cleanup(&temp_dir).await.unwrap();
        
        assert!(!temp_dir.exists());
    }

    #[tokio::test]
    async fn test_cleanup_nonexistent() {
        let temp_dir = PathBuf::from("/tmp/test_cleanup_nonexistent");
        
        // Should not fail if directory doesn't exist
        ChunkWriter::cleanup(&temp_dir).await.unwrap();
        
        assert!(!temp_dir.exists());
    }
}
