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
    /// 
    /// # Errors
    /// Returns an error if the semaphore is closed
    pub async fn spawn<F, T>(&self, task: F) -> crate::Result<tokio::task::JoinHandle<T>>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let permit = self.semaphore.clone().acquire_owned().await
            .map_err(|_| crate::DownloadError::PoolClosed)?;
        
        Ok(tokio::spawn(async move {
            let result = task.await;
            drop(permit);
            result
        }))
    }
    
    /// Try to spawn a task (non-blocking)
    /// 
    /// Returns None if no workers are available
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

    #[tokio::test]
    async fn test_spawn_task() {
        let pool = WorkerPool::new(2);
        
        let handle = pool.spawn(async {
            42
        }).await.unwrap();
        
        let result = handle.await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_concurrent_spawn() {
        let pool = WorkerPool::new(2);
        
        let mut handles = Vec::new();
        for i in 0..5 {
            let handle = pool.spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                i * 2
            }).await.unwrap();
            handles.push(handle);
        }
        
        let results = WorkerPool::wait_all(handles).await;
        assert_eq!(results.len(), 5);
        assert_eq!(results.iter().sum::<i32>(), 20);
    }

    #[tokio::test]
    async fn test_try_spawn() {
        let pool = WorkerPool::new(1);
        
        // First spawn should succeed
        let handle1 = pool.try_spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            1
        });
        assert!(handle1.is_some());
        
        // Second spawn should fail (no available workers)
        let handle2 = pool.try_spawn(async { 2 });
        assert!(handle2.is_none());
        
        // Wait for first task to complete
        handle1.unwrap().await.unwrap();
    }

    #[tokio::test]
    async fn test_wait_all() {
        let pool = WorkerPool::new(4);
        
        let mut handles = Vec::new();
        for i in 0..3 {
            let handle = pool.spawn(async move { i + 10 }).await.unwrap();
            handles.push(handle);
        }
        
        let results = WorkerPool::wait_all(handles).await;
        assert_eq!(results, vec![10, 11, 12]);
    }
}
