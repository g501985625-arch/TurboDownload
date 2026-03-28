use super::{Chunk, ChunkState};
use std::path::PathBuf;

pub struct ChunkManager {
    chunks: Vec<Chunk>,
    total_size: u64,
    chunk_size: u64,
    temp_dir: PathBuf,
}

impl ChunkManager {
    pub fn new(total_size: u64, chunk_size: u64, temp_dir: PathBuf) -> Self {
        Self {
            chunks: Vec::new(),
            total_size,
            chunk_size,
            temp_dir,
        }
    }
    
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }
    
    pub fn total_size(&self) -> u64 {
        self.total_size
    }
    
    pub fn total_downloaded(&self) -> u64 {
        self.chunks.iter().map(|c| c.downloaded).sum()
    }
    
    pub fn progress_percent(&self) -> f64 {
        if self.total_size == 0 {
            return 0.0;
        }
        (self.total_downloaded() as f64 / self.total_size as f64) * 100.0
    }
    
    /// Calculate and create chunks
    pub fn calculate_chunks(&mut self, thread_count: u32) {
        let chunk_size = if self.chunk_size > 0 {
            self.chunk_size
        } else {
            // Auto calculate: file size / thread count, minimum 1MB
            (self.total_size / thread_count as u64).max(1024 * 1024)
        };
        
        let mut start = 0u64;
        let mut id = 0u32;
        
        while start < self.total_size {
            let end = (start + chunk_size).min(self.total_size);
            let chunk = Chunk::new(id, start, end, &self.temp_dir);
            self.chunks.push(chunk);
            
            start = end;
            id += 1;
        }
    }
    
    /// Get the next pending chunk
    pub fn get_next_pending(&mut self) -> Option<&mut Chunk> {
        self.chunks
            .iter_mut()
            .find(|c| c.state == ChunkState::Pending)
    }
    
    /// Get count of pending chunks
    pub fn pending_count(&self) -> usize {
        self.chunks
            .iter()
            .filter(|c| c.state != ChunkState::Completed)
            .count()
    }
    
    /// Update chunk state
    pub fn update_chunk(&mut self, id: u32, downloaded: u64, state: ChunkState) {
        if let Some(chunk) = self.chunks.get_mut(id as usize) {
            chunk.downloaded = downloaded;
            chunk.state = state;
        }
    }
    
    /// Mark chunk as downloading
    pub fn mark_downloading(&mut self, id: u32) {
        if let Some(chunk) = self.chunks.get_mut(id as usize) {
            chunk.state = ChunkState::Downloading;
        }
    }
    
    /// Mark chunk as completed
    pub fn mark_completed(&mut self, id: u32) {
        if let Some(chunk) = self.chunks.get_mut(id as usize) {
            chunk.downloaded = chunk.size();
            chunk.state = ChunkState::Completed;
        }
    }
    
    /// Mark chunk as failed
    pub fn mark_failed(&mut self, id: u32) {
        if let Some(chunk) = self.chunks.get_mut(id as usize) {
            chunk.state = ChunkState::Failed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_chunk_manager_creation() {
        let temp_dir = PathBuf::from("/tmp/test");
        let manager = ChunkManager::new(1024 * 1024, 256 * 1024, temp_dir);
        
        assert_eq!(manager.total_size(), 1024 * 1024);
        assert_eq!(manager.total_downloaded(), 0);
        assert_eq!(manager.progress_percent(), 0.0);
    }

    #[test]
    fn test_calculate_chunks() {
        let temp_dir = PathBuf::from("/tmp/test");
        let mut manager = ChunkManager::new(1024 * 1024, 256 * 1024, temp_dir);
        
        manager.calculate_chunks(4);
        
        // Should create 4 chunks of 256KB each
        assert_eq!(manager.chunks().len(), 4);
        
        // Check first chunk
        let first = &manager.chunks()[0];
        assert_eq!(first.id, 0);
        assert_eq!(first.start, 0);
        assert_eq!(first.end, 256 * 1024);
        
        // Check last chunk
        let last = &manager.chunks()[3];
        assert_eq!(last.id, 3);
        assert_eq!(last.start, 768 * 1024);
        assert_eq!(last.end, 1024 * 1024);
    }

    #[test]
    fn test_get_next_pending() {
        let temp_dir = PathBuf::from("/tmp/test");
        let mut manager = ChunkManager::new(1024 * 1024, 512 * 1024, temp_dir);
        manager.calculate_chunks(2);
        
        // Get first pending chunk
        let chunk = manager.get_next_pending();
        assert!(chunk.is_some());
        assert_eq!(chunk.unwrap().id, 0);
        
        // Mark first as completed
        manager.mark_completed(0);
        
        // Get next pending
        let chunk = manager.get_next_pending();
        assert!(chunk.is_some());
        assert_eq!(chunk.unwrap().id, 1);
        
        // Mark second as completed
        manager.mark_completed(1);
        
        // No more pending
        let chunk = manager.get_next_pending();
        assert!(chunk.is_none());
    }

    #[test]
    fn test_pending_count() {
        let temp_dir = PathBuf::from("/tmp/test");
        let mut manager = ChunkManager::new(1024 * 1024, 256 * 1024, temp_dir);
        manager.calculate_chunks(4);
        
        assert_eq!(manager.pending_count(), 4);
        
        manager.mark_completed(0);
        assert_eq!(manager.pending_count(), 3);
        
        manager.mark_failed(1);
        assert_eq!(manager.pending_count(), 3); // Failed is also not completed
    }

    #[test]
    fn test_progress_percent() {
        let temp_dir = PathBuf::from("/tmp/test");
        let mut manager = ChunkManager::new(1000, 250, temp_dir);
        manager.calculate_chunks(4);
        
        assert_eq!(manager.progress_percent(), 0.0);
        
        // Update first chunk to 50% complete
        manager.update_chunk(0, 125, ChunkState::Downloading);
        assert_eq!(manager.progress_percent(), 12.5);
        
        // Complete first chunk
        manager.mark_completed(0);
        assert_eq!(manager.progress_percent(), 25.0);
    }

    #[test]
    fn test_chunk_state_transitions() {
        let temp_dir = PathBuf::from("/tmp/test");
        let mut manager = ChunkManager::new(1024, 512, temp_dir);
        manager.calculate_chunks(2);
        
        // Initial state
        assert_eq!(manager.chunks()[0].state, ChunkState::Pending);
        
        // Mark as downloading
        manager.mark_downloading(0);
        assert_eq!(manager.chunks()[0].state, ChunkState::Downloading);
        
        // Mark as completed
        manager.mark_completed(0);
        assert_eq!(manager.chunks()[0].state, ChunkState::Completed);
        assert_eq!(manager.chunks()[0].downloaded, 512);
        
        // Mark as failed
        manager.mark_failed(1);
        assert_eq!(manager.chunks()[1].state, ChunkState::Failed);
    }
}