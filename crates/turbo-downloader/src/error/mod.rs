mod types;

pub use types::DownloadError;
pub type Result<T> = std::result::Result<T, DownloadError>;
