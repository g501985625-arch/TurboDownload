use std::path::Path;
use crate::Result;

/// File merger for combining chunks into final file
pub struct FileMerger;

impl FileMerger {
    /// Merge multiple chunk files into a single output file
    pub async fn merge(
        chunk_paths: &[&Path],
        output_path: &Path,
    ) -> Result<()> {
        use tokio::fs::File;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        // Create output file
        let mut output = File::create(output_path).await?;
        
        // Copy each chunk to output
        for chunk_path in chunk_paths {
            let mut chunk_file = File::open(chunk_path).await?;
            let mut buffer = vec![];
            chunk_file.read_to_end(&mut buffer).await?;
            output.write_all(&buffer).await?;
        }
        
        output.flush().await?;
        Ok(())
    }
    
    /// Merge chunks in order (for resumable downloads)
    pub async fn merge_ordered(
        chunk_paths: &[&Path],
        output_path: &Path,
        chunk_size: u64,
    ) -> Result<()> {
        // For now, same as merge (chunks should be in order)
        Self::merge(chunk_paths, output_path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_merge() {
        let temp_dir = PathBuf::from("/tmp/test_merge");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        
        // Create test chunks
        let chunk1_path = temp_dir.join("chunk1.tmp");
        let chunk2_path = temp_dir.join("chunk2.tmp");
        let output_path = temp_dir.join("output.txt");
        
        tokio::fs::write(&chunk1_path, b"Hello, ").await.unwrap();
        tokio::fs::write(&chunk2_path, b"World!").await.unwrap();
        
        // Merge
        let chunk_paths: Vec<&Path> = vec![&chunk1_path, &chunk2_path];
        FileMerger::merge(&chunk_paths, &output_path).await.unwrap();
        
        // Verify
        let result = tokio::fs::read(&output_path).await.unwrap();
        assert_eq!(result, b"Hello, World!");
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
}
