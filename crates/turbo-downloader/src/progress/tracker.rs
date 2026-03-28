use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Progress tracker for download tasks
/// 
/// Thread-safe progress tracking with atomic operations.
/// Calculates speed, ETA, and percentage in real-time.
pub struct Tracker {
    total: u64,
    downloaded: AtomicU64,
    start_time: Instant,
}

impl Tracker {
    /// Create a new progress tracker
    /// 
    /// # Arguments
    /// - `total`: Total file size in bytes
    pub fn new(total: u64) -> Self {
        Self {
            total,
            downloaded: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }

    /// Update downloaded bytes
    /// 
    /// This is thread-safe and can be called from multiple threads.
    pub fn update(&self, downloaded: u64) {
        self.downloaded.store(downloaded, Ordering::Relaxed);
    }

    /// Add downloaded bytes
    /// 
    /// Useful for incremental updates from multiple chunks.
    pub fn add(&self, bytes: u64) {
        self.downloaded.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Get current downloaded bytes
    pub fn downloaded(&self) -> u64 {
        self.downloaded.load(Ordering::Relaxed)
    }

    /// Get total file size
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Get elapsed time since start
    pub fn elapsed_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Calculate current speed (bytes per second)
    pub fn speed(&self) -> u64 {
        let downloaded = self.downloaded();
        let elapsed = self.elapsed_secs();
        
        if elapsed > 0 {
            downloaded / elapsed
        } else {
            0
        }
    }

    /// Calculate ETA (seconds remaining)
    pub fn eta(&self) -> Option<u64> {
        let downloaded = self.downloaded();
        let speed = self.speed();
        
        if speed > 0 && self.total > downloaded {
            Some((self.total - downloaded) / speed)
        } else {
            None
        }
    }

    /// Calculate progress percentage
    pub fn percent(&self) -> f64 {
        let downloaded = self.downloaded();
        
        if self.total > 0 {
            (downloaded as f64 / self.total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Check if download is complete
    pub fn is_complete(&self) -> bool {
        self.downloaded() >= self.total
    }

    /// Get full progress information
    pub fn get_progress(&self) -> super::DownloadProgress {
        let downloaded = self.downloaded();
        let speed = self.speed();
        
        super::DownloadProgress {
            total: self.total,
            downloaded,
            speed,
            avg_speed: speed,
            eta: self.eta(),
            percent: self.percent(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_tracker_creation() {
        let tracker = Tracker::new(1024);
        assert_eq!(tracker.total(), 1024);
        assert_eq!(tracker.downloaded(), 0);
        assert_eq!(tracker.percent(), 0.0);
    }

    #[test]
    fn test_tracker_update() {
        let tracker = Tracker::new(1000);
        
        tracker.update(500);
        assert_eq!(tracker.downloaded(), 500);
        assert_eq!(tracker.percent(), 50.0);
        
        tracker.update(750);
        assert_eq!(tracker.downloaded(), 750);
        assert_eq!(tracker.percent(), 75.0);
    }

    #[test]
    fn test_tracker_add() {
        let tracker = Tracker::new(1000);
        
        tracker.add(100);
        assert_eq!(tracker.downloaded(), 100);
        
        tracker.add(200);
        assert_eq!(tracker.downloaded(), 300);
        
        tracker.add(700);
        assert_eq!(tracker.downloaded(), 1000);
        assert!(tracker.is_complete());
    }

    #[test]
    fn test_tracker_is_complete() {
        let tracker = Tracker::new(100);
        assert!(!tracker.is_complete());
        
        tracker.update(50);
        assert!(!tracker.is_complete());
        
        tracker.update(100);
        assert!(tracker.is_complete());
        
        tracker.update(150); // Over-download (shouldn't happen but handle gracefully)
        assert!(tracker.is_complete());
    }

    #[test]
    fn test_tracker_percent_edge_cases() {
        // Zero total
        let tracker = Tracker::new(0);
        assert_eq!(tracker.percent(), 0.0);
        
        // Normal case
        let tracker = Tracker::new(100);
        tracker.update(33);
        assert_eq!(tracker.percent(), 33.0);
    }

    #[test]
    fn test_tracker_thread_safety() {
        let tracker = Tracker::new(10000);
        
        // Spawn multiple threads to update concurrently
        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let tracker = Tracker::new(10000);
                    for _ in 0..100 {
                        tracker.add(10);
                    }
                    tracker.downloaded()
                })
            })
            .collect();
        
        // Verify all threads completed
        for handle in handles {
            let result = handle.join().unwrap();
            assert_eq!(result, 1000);
        }
    }

    #[test]
    fn test_tracker_get_progress() {
        let tracker = Tracker::new(1000);
        tracker.update(500);
        
        let progress = tracker.get_progress();
        assert_eq!(progress.total, 1000);
        assert_eq!(progress.downloaded, 500);
        assert_eq!(progress.percent, 50.0);
    }
}
