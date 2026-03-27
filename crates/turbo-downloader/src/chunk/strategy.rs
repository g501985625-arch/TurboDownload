use serde::{Deserialize, Serialize};

/// Chunk state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChunkState {
    Pending,
    Downloading,
    Completed,
    Failed,
}

/// Chunk information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub state: ChunkState,
}

impl Chunk {
    pub fn new(id: u32, start: u64, end: u64) -> Self {
        Self {
            id,
            start,
            end,
            downloaded: 0,
            state: ChunkState::Pending,
        }
    }

    pub fn size(&self) -> u64 {
        self.end - self.start
    }

    pub fn remaining(&self) -> u64 {
        self.size() - self.downloaded
    }

    pub fn is_complete(&self) -> bool {
        self.downloaded >= self.size()
    }
}

/// Chunk strategy
pub struct Strategy {
    pub chunks: Vec<Chunk>,
}

impl Strategy {
    pub fn calculate(file_size: u64, thread_count: u32, min_chunk_size: u64) -> Self {
        let threads = if thread_count == 0 {
            Self::auto_thread_count(file_size)
        } else {
            thread_count
        };

        let chunk_size = (file_size / threads as u64).max(min_chunk_size);
        #[allow(clippy::manual_div_ceil)]
        let actual_threads = ((file_size + chunk_size - 1) / chunk_size) as u32;

        let mut chunks = Vec::with_capacity(actual_threads as usize);
        let mut start = 0u64;

        for id in 0..actual_threads {
            let end = (start + chunk_size).min(file_size);
            chunks.push(Chunk::new(id, start, end));
            start = end;
        }

        Self { chunks }
    }

    fn auto_thread_count(file_size: u64) -> u32 {
        match file_size {
            0..=10_000_000 => 2,
            10_000_001..=100_000_000 => 4,
            100_000_001..=1_000_000_000 => 8,
            _ => 16,
        }
    }
}
