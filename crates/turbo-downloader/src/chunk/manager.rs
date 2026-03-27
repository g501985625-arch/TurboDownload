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