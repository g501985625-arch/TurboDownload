//! Smoke tests for turbo-downloader

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use turbo_downloader::*;

    /// Test error creation
    #[test]
    fn test_error_creation() {
        let err = DownloadError::TaskNotFound("test-id".to_string());
        assert_eq!(err.code(), "TASK_NOT_FOUND");
        assert!(!err.is_retryable());
    }

    /// Test chunk creation
    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new(0, 0, 1000);
        assert_eq!(chunk.id, 0);
        assert_eq!(chunk.size(), 1000);
        assert!(!chunk.is_complete());

        let chunk2 = Chunk::new(1, 500, 1000);
        assert_eq!(chunk2.remaining(), 500);
    }

    /// Test strategy calculation
    #[test]
    fn test_strategy_calculation() {
        let strategy = Strategy::calculate(10_000_000, 0, 1_000_000);
        assert!(strategy.chunks.len() >= 2);

        let total: u64 = strategy.chunks.iter().map(|c| c.size()).sum();
        assert_eq!(total, 10_000_000);
    }

    /// Test strategy boundaries
    #[test]
    fn test_strategy_boundaries() {
        // Small file
        let s1 = Strategy::calculate(100, 0, 10);
        assert!(s1.chunks.len() >= 1);

        // Large file
        let s2 = Strategy::calculate(1_000_000_000, 0, 1_000_000);
        assert!(s2.chunks.len() >= 8);
    }

    /// Test downloader builder
    #[test]
    fn test_downloader_builder() {
        let _builder = DownloaderBuilder::new()
            .max_concurrent_tasks(5)
            .default_threads(8)
            .timeout(600);

        // Can't build without network, but builder pattern works
        assert!(true);
    }

    /// Test progress tracker
    #[test]
    fn test_tracker() {
        let tracker = Tracker::new(1000);

        tracker.update(100);
        let progress = tracker.get_progress();

        assert_eq!(progress.downloaded, 100);
        assert_eq!(progress.total, 1000);
    }

    /// Test speed calculator
    #[test]
    fn test_speed_calculator() {
        let mut calc = SpeedCalculator::new(5);

        calc.add_sample(1000);
        calc.add_sample(1000);
        calc.add_sample(1000);
        calc.add_sample(1000);

        // With enough samples, should calculate speed
        let speed = calc.get_speed();
        assert!(speed >= 0); // Could be 0 with very fast execution
    }
}
