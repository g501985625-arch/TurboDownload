use std::path::Path;
use crate::Result;

/// File merger for combining chunks into final file
pub struct FileMerger;

impl FileMerger {
    /// Merge multiple chunk files into a single output file
    /// 
    /// Uses streaming copy with a fixed buffer size to avoid loading
    /// large files into memory.
    pub async fn merge(
        chunk_paths: &[&Path],
        output_path: &Path,
    ) -> Result<()> {
        use tokio::fs::File;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        // Create output file
        let mut output = File::create(output_path).await?;
        
        // 64KB buffer for streaming copy
        let mut buffer = vec![0u8; 64 * 1024];
        
        // Copy each chunk to output using streaming
        for chunk_path in chunk_paths {
            let mut chunk_file = File::open(chunk_path).await?;
            
            loop {
                let n = chunk_file.read(&mut buffer).await?;
                if n == 0 {
                    break;
                }
                output.write_all(&buffer[..n]).await?;
            }
        }
        
        output.flush().await?;
        Ok(())
    }
    
    /// Merge chunks in order with progress callback
    /// 
    /// # Arguments
    /// - `chunk_paths`: Paths to chunk files in order
    /// - `output_path`: Path for the merged output file
    /// - `progress_callback`: Called after each chunk is merged with (chunk_index, bytes_written)
    pub async fn merge_with_progress<F>(
        chunk_paths: &[&Path],
        output_path: &Path,
        mut progress_callback: F,
    ) -> Result<()>
    where
        F: FnMut(usize, u64),
    {
        use tokio::fs::File;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut output = File::create(output_path).await?;
        let mut buffer = vec![0u8; 64 * 1024];
        
        for (idx, chunk_path) in chunk_paths.iter().enumerate() {
            let mut chunk_file = File::open(chunk_path).await?;
            let mut chunk_bytes = 0u64;
            
            loop {
                let n = chunk_file.read(&mut buffer).await?;
                if n == 0 {
                    break;
                }
                output.write_all(&buffer[..n]).await?;
                chunk_bytes += n as u64;
            }
            
            progress_callback(idx, chunk_bytes);
        }
        
        output.flush().await?;
        Ok(())
    }
    
    /// Merge chunks in order (for resumable downloads)
    pub async fn merge_ordered(
        chunk_paths: &[&Path],
        output_path: &Path,
        _chunk_size: u64,
    ) -> Result<()> {
        Self::merge(chunk_paths, output_path).await
    }
    
    /// Calculate SHA256 hash of merged file
    /// 
    /// This is useful for verifying file integrity after merge.
    pub async fn calculate_sha256(path: &Path) -> Result<String> {
        use tokio::fs::File;
        use tokio::io::AsyncReadExt;
        use sha2::{Sha256, Digest};

        let mut file = File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 64 * 1024];
        
        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
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

    #[tokio::test]
    async fn test_merge_single_chunk() {
        let temp_dir = PathBuf::from("/tmp/test_merge_single");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        
        let chunk_path = temp_dir.join("chunk.tmp");
        let output_path = temp_dir.join("output.txt");
        
        tokio::fs::write(&chunk_path, b"Single chunk content").await.unwrap();
        
        let chunk_paths: Vec<&Path> = vec![&chunk_path];
        FileMerger::merge(&chunk_paths, &output_path).await.unwrap();
        
        let result = tokio::fs::read(&output_path).await.unwrap();
        assert_eq!(result, b"Single chunk content");
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_merge_empty_chunks() {
        let temp_dir = PathBuf::from("/tmp/test_merge_empty");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        
        let output_path = temp_dir.join("output.txt");
        
        // Merge empty list
        let chunk_paths: Vec<&Path> = vec![];
        FileMerger::merge(&chunk_paths, &output_path).await.unwrap();
        
        let result = tokio::fs::read(&output_path).await.unwrap();
        assert!(result.is_empty());
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_merge_large_files() {
        let temp_dir = PathBuf::from("/tmp/test_merge_large");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        
        let chunk1_path = temp_dir.join("chunk1.tmp");
        let chunk2_path = temp_dir.join("chunk2.tmp");
        let output_path = temp_dir.join("output.bin");
        
        // Create large chunks (1MB each)
        let data1 = vec![0u8; 1024 * 1024];
        let data2 = vec![1u8; 1024 * 1024];
        
        tokio::fs::write(&chunk1_path, &data1).await.unwrap();
        tokio::fs::write(&chunk2_path, &data2).await.unwrap();
        
        let chunk_paths: Vec<&Path> = vec![&chunk1_path, &chunk2_path];
        FileMerger::merge(&chunk_paths, &output_path).await.unwrap();
        
        // Verify size
        let metadata = tokio::fs::metadata(&output_path).await.unwrap();
        assert_eq!(metadata.len(), 2 * 1024 * 1024);
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_merge_with_progress() {
        let temp_dir = PathBuf::from("/tmp/test_merge_progress");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        
        let chunk1_path = temp_dir.join("chunk1.tmp");
        let chunk2_path = temp_dir.join("chunk2.tmp");
        let output_path = temp_dir.join("output.txt");
        
        tokio::fs::write(&chunk1_path, b"Hello").await.unwrap();
        tokio::fs::write(&chunk2_path, b", World!").await.unwrap();
        
        let mut progress_calls = vec![];
        
        let chunk_paths: Vec<&Path> = vec![&chunk1_path, &chunk2_path];
        FileMerger::merge_with_progress(&chunk_paths, &output_path, |idx, bytes| {
            progress_calls.push((idx, bytes));
        }).await.unwrap();
        
        // Verify progress was called for each chunk
        assert_eq!(progress_calls.len(), 2);
        assert_eq!(progress_calls[0], (0, 5)); // "Hello" = 5 bytes
        assert_eq!(progress_calls[1], (1, 8)); // ", World!" = 8 bytes
        
        // Verify merged content
        let result = tokio::fs::read(&output_path).await.unwrap();
        assert_eq!(result, b"Hello, World!");
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_calculate_sha256() {
        let temp_dir = PathBuf::from("/tmp/test_sha256");
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        
        let file_path = temp_dir.join("test.txt");
        tokio::fs::write(&file_path, b"Hello, World!").await.unwrap();
        
        let hash = FileMerger::calculate_sha256(&file_path).await.unwrap();
        
        // Known SHA256 hash for "Hello, World!"
        assert_eq!(hash, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
        
        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
}
