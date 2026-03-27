use std::sync::Arc;
use tokio::sync::Semaphore;

/// Worker pool for controlling concurrent download tasks
pub struct WorkerPool {
    max_workers: usize,
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    /// Create a new worker pool with max concurrent workers
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            semaphore: Arc::new(Semaphore::new(max_workers)),
        }
    }
    
    /// Get max workers
    pub fn max_workers(&self) -> usize {
        self.max_workers
    }
    
    /// Get available workers
    pub fn available(&self) -> usize {
        self.semaphore.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_pool_creation() {
        let pool = WorkerPool::new(4);
        assert_eq!(pool.max_workers(), 4);
        assert_eq!(pool.available(), 4);
    }
}
