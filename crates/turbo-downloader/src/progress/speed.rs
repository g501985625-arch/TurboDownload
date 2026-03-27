use std::collections::VecDeque;
use std::time::Instant;

struct Sample {
    timestamp: Instant,
    bytes: u64,
}

/// Speed calculator (sliding window)
pub struct SpeedCalculator {
    samples: VecDeque<Sample>,
    window_size: usize,
    total_bytes: u64,
}

impl SpeedCalculator {
    pub fn new(window_size: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(window_size),
            window_size,
            total_bytes: 0,
        }
    }

    pub fn add_sample(&mut self, bytes: u64) {
        let now = Instant::now();
        self.samples.push_back(Sample {
            timestamp: now,
            bytes,
        });
        self.total_bytes += bytes;

        while self.samples.len() > self.window_size {
            if let Some(old) = self.samples.pop_front() {
                self.total_bytes -= old.bytes;
            }
        }
    }

    pub fn get_speed(&self) -> u64 {
        if self.samples.len() < 2 {
            return 0;
        }

        let first = self.samples.front().unwrap();
        let last = self.samples.back().unwrap();

        let duration = last.timestamp.duration_since(first.timestamp).as_secs();
        if duration == 0 {
            return 0;
        }

        self.total_bytes / duration
    }
}
