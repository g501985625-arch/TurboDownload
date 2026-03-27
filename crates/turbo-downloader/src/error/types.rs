use reqwest::Error as ReqwestError;
use std::io::Error as IoError;
use thiserror::Error;

/// Download errors
#[derive(Debug, Error)]
pub enum DownloadError {
    /// Network error
    #[error("Network error: {0}")]
    Network(#[from] ReqwestError),

    /// HTTP error with status code
    #[error("HTTP error {0}: {1}")]
    Http(u16, String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    /// Task not found
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    /// Range request not supported
    #[error("Range request not supported")]
    RangeNotSupported,

    /// Content length unknown
    #[error("Content length unknown")]
    ContentLengthUnknown,

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    /// Timeout
    #[error("Timeout")]
    Timeout,

    /// Cancelled
    #[error("Cancelled")]
    Cancelled,

    /// File already exists
    #[error("File already exists: {0}")]
    FileExists(String),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for DownloadError {
    fn from(err: serde_json::Error) -> Self {
        DownloadError::Internal(err.to_string())
    }
}

impl DownloadError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            DownloadError::Network(_) | DownloadError::Timeout | DownloadError::Http(500..=599, _)
        )
    }

    /// Get error code
    pub fn code(&self) -> &'static str {
        match self {
            DownloadError::Network(_) => "NETWORK",
            DownloadError::Http(_, _) => "HTTP",
            DownloadError::Io(_) => "IO",
            DownloadError::TaskNotFound(_) => "TASK_NOT_FOUND",
            DownloadError::RangeNotSupported => "RANGE_NOT_SUPPORTED",
            DownloadError::ContentLengthUnknown => "CONTENT_LENGTH_UNKNOWN",
            DownloadError::ValidationFailed(_) => "VALIDATION",
            DownloadError::Timeout => "TIMEOUT",
            DownloadError::Cancelled => "CANCELLED",
            DownloadError::FileExists(_) => "FILE_EXISTS",
            DownloadError::InvalidUrl(_) => "INVALID_URL",
            DownloadError::Internal(_) => "INTERNAL",
        }
    }
}
