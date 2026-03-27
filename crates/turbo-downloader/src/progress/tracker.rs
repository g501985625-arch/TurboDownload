use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Progress tracker
pub struct Tracker {
    total: u64,
    downloaded: AtomicU64,
    start_time: Instant,
}

impl Tracker {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            downloaded: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }

    pub fn update(&self, downloaded: u64) {
        self.downloaded.store(downloaded, Ordering::Relaxed);
    }

    pub fn get_progress(&self) -> super::DownloadProgress {
        let downloaded = self.downloaded.load(Ordering::Relaxed);
        let elapsed = self.start_time.elapsed().as_secs();

        let avg_speed = if elapsed > 0 { downloaded / elapsed } else { 0 };

        let eta = if avg_speed > 0 && self.total > downloaded {
            Some((self.total - downloaded) / avg_speed)
        } else {
            None
        };

        let percent = if self.total > 0 {
            (downloaded as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        super::DownloadProgress {
            total: self.total,
            downloaded,
            speed: avg_speed,
            avg_speed,
            eta,
            percent,
        }
    }
}
