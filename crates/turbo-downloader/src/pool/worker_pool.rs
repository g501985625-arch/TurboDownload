use std::sync::Arc;
use std::future::Future;
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
    
    /// Spawn a task to the worker pool
    pub async fn spawn<F, T>(&self, task: F) -> tokio::task::JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        
        tokio::spawn(async move {
            let result = task.await;
            drop(permit);
            result
        })
    }
    
    /// Try to spawn a task (non-blocking)
    pub fn try_spawn<F, T>(&self, task: F) -> Option<tokio::task::JoinHandle<T>>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        match self.semaphore.clone().try_acquire_owned() {
            Ok(permit) => {
                Some(tokio::spawn(async move {
                    let result = task.await;
                    drop(permit);
                    result
                }))
            }
            Err(_) => None,
        }
    }
    
    /// Wait for all tasks to complete
    pub async fn wait_all<T>(handles: Vec<tokio::task::JoinHandle<T>>) -> Vec<T>
    where
        T: Send + 'static,
    {
        let mut results = Vec::with_capacity(handles.len());
        
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }
        
        results
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
