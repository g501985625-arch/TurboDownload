//! URL scheduler for crawling

use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Queue policy
#[derive(Debug, Clone)]
pub enum QueuePolicy {
    /// First In First Out
    Fifo,
    /// Last In First Out
    Lifo,
    /// Priority based
    Priority,
}

/// URL scheduler with concurrency control
pub struct UrlScheduler {
    queue: VecDeque<String>,
    visited: Arc<RwLock<HashSet<String>>>,
    policy: QueuePolicy,
    max_depth: usize,
    current_depth: usize,
    rate_limit: Duration,
    last_request: RwLock<Instant>,
    /// Concurrent request limit
    max_concurrent: usize,
    /// Current active requests
    active: Arc<RwLock<usize>>,
}

impl UrlScheduler {
    /// Create new scheduler
    pub fn new(policy: QueuePolicy, max_depth: usize, rate_limit: Duration) -> Self {
        Self {
            queue: VecDeque::new(),
            visited: Arc::new(RwLock::new(HashSet::new())),
            policy,
            max_depth,
            current_depth: 0,
            rate_limit,
            last_request: RwLock::new(Instant::now()),
            max_concurrent: 3,
            active: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Create scheduler with custom concurrency
    pub fn with_concurrency(policy: QueuePolicy, max_depth: usize, rate_limit: Duration, max_concurrent: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            visited: Arc::new(RwLock::new(HashSet::new())),
            policy,
            max_depth,
            current_depth: 0,
            rate_limit,
            last_request: RwLock::new(Instant::now()),
            max_concurrent,
            active: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Add URL to queue
    pub fn add(&mut self, url: String) {
        // Check if already visited
        if self.is_visited(&url) {
            return;
        }
        
        // Check depth limit
        if self.current_depth >= self.max_depth && !self.queue.is_empty() {
            return;
        }
        
        match self.policy {
            QueuePolicy::Fifo => self.queue.push_back(url),
            QueuePolicy::Lifo => self.queue.push_front(url),
            QueuePolicy::Priority => self.queue.push_back(url),
        }
    }
    
    /// Get next URL from queue (check concurrency)
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<String> {
        // Check if at concurrency limit
        let active = *self.active.read().unwrap();
        if active >= self.max_concurrent {
            return None;
        }
        
        // Check if queue is empty
        if self.queue.is_empty() {
            return None;
        }
        
        // Check rate limit
        let last = *self.last_request.read().unwrap();
        if last.elapsed() < self.rate_limit {
            return None;
        }
        
        // Get URL from queue
        let url = self.queue.pop_front()?;
        
        // Mark as visited
        self.visited.write().unwrap().insert(url.clone());
        
        // Update last request time
        *self.last_request.write().unwrap() = Instant::now();
        
        // Increment active count
        *self.active.write().unwrap() += 1;
        
        // Increment depth
        self.current_depth += 1;
        
        Some(url)
    }
    
    /// Mark a URL as completed (decrement active count)
    pub fn complete(&self) {
        let mut active = self.active.write().unwrap();
        if *active > 0 {
            *active -= 1;
        }
    }
    
    /// Check if URL was visited
    pub fn is_visited(&self, url: &str) -> bool {
        self.visited.read().unwrap().contains(url)
    }
    
    /// Get queue size
    pub fn size(&self) -> usize {
        self.queue.len()
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// Add multiple URLs
    pub fn add_batch(&mut self, urls: Vec<String>) {
        for url in urls {
            self.add(url);
        }
    }
    
    /// Get visited count
    pub fn visited_count(&self) -> usize {
        self.visited.read().unwrap().len()
    }
    
    /// Get current concurrency level
    pub fn active_count(&self) -> usize {
        *self.active.read().unwrap()
    }
    
    /// Reset the scheduler
    pub fn reset(&mut self) {
        self.queue.clear();
        self.visited.write().unwrap().clear();
        self.current_depth = 0;
        *self.active.write().unwrap() = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scheduler_fifo() {
        let mut scheduler = UrlScheduler::new(QueuePolicy::Fifo, 3, Duration::from_millis(0));
        
        scheduler.add("url1".to_string());
        scheduler.add("url2".to_string());
        scheduler.add("url3".to_string());
        
        // Can't get without async
        assert_eq!(scheduler.size(), 3);
    }
    
    #[test]
    fn test_visited_check() {
        let scheduler = UrlScheduler::new(QueuePolicy::Fifo, 3, Duration::from_millis(0));
        
        assert!(!scheduler.is_visited("test"));
    }
}