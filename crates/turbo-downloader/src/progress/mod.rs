pub mod speed;
pub mod tracker;

pub use speed::SpeedCalculator;
pub use tracker::Tracker;
pub type ProgressCallback = Box<dyn Fn(super::DownloadProgress) + Send + Sync>;

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub total: u64,
    pub downloaded: u64,
    pub speed: u64,
    pub avg_speed: u64,
    pub eta: Option<u64>,
    pub percent: f64,
}
