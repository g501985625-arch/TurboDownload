//! Data models for TurboDownload

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

/// Download task status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    /// Waiting to start
    Pending,
    /// Actively downloading
    Downloading,
    /// Download paused
    Paused,
    /// Download completed successfully
    Completed,
    /// Download failed with error
    Failed,
    /// Download cancelled by user
    Cancelled,
}

impl Default for DownloadStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Download task configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    /// Output filename (auto-detected from URL if not specified)
    pub filename: Option<String>,
    /// Output directory (defaults to user's download folder)
    pub output_dir: Option<String>,
    /// Number of concurrent connections
    #[serde(default = "default_connections")]
    pub connections: u8,
    /// Maximum speed in bytes/sec (0 = unlimited)
    #[serde(default)]
    pub max_speed: u64,
    /// HTTP headers
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
}

fn default_connections() -> u8 {
    4
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            filename: None,
            output_dir: None,
            connections: default_connections(),
            max_speed: 0,
            headers: std::collections::HashMap::new(),
        }
    }
}

/// Download task progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    /// Task ID
    pub id: String,
    /// Current progress percentage (0-100)
    pub progress: f64,
    /// Download speed in bytes/sec
    pub speed: u64,
    /// Total file size in bytes
    pub total_size: u64,
    /// Downloaded bytes
    pub downloaded: u64,
    /// Estimated time remaining in seconds
    pub eta: Option<u64>,
    /// Current status
    pub status: DownloadStatus,
}

/// Download task model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    /// Unique task ID
    pub id: String,
    /// Source URL
    pub url: String,
    /// Output filename
    pub filename: String,
    /// Output directory
    pub output_dir: PathBuf,
    /// Current status
    pub status: DownloadStatus,
    /// Current progress
    pub progress: f64,
    /// Download speed in bytes/sec
    pub speed: u64,
    /// Total file size in bytes (0 if unknown)
    pub total_size: u64,
    /// Downloaded bytes
    pub downloaded: u64,
    /// Error message if failed
    pub error: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Completion timestamp
    pub completed_at: Option<DateTime<Utc>>,
    /// Download configuration
    pub config: DownloadConfig,
}

impl DownloadTask {
    /// Create a new download task
    pub fn new(url: String, config: DownloadConfig) -> Self {
        let id = Uuid::new_v4().to_string();
        let filename = config.filename.clone().unwrap_or_else(|| {
            // Extract filename from URL
            url.split('/')
                .last()
                .unwrap_or("download")
                .to_string()
        });
        let output_dir = config
            .output_dir
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                dirs::download_dir().unwrap_or_else(|| PathBuf::from("."))
            });

        Self {
            id,
            url,
            filename,
            output_dir,
            status: DownloadStatus::Pending,
            progress: 0.0,
            speed: 0,
            total_size: 0,
            downloaded: 0,
            error: None,
            created_at: Utc::now(),
            completed_at: None,
            config,
        }
    }
}

/// Resource type for crawler
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    /// Image files
    Image,
    /// Video files
    Video,
    /// Audio files
    Audio,
    /// Document files
    Document,
    /// Archive files (zip, rar, etc.)
    Archive,
    /// Executable files
    Executable,
    /// Other files
    Other,
}

impl ResourceType {
    /// Determine resource type from URL or MIME type
    pub fn from_url(url: &str) -> Self {
        let url_lower = url.to_lowercase();
        
        if url_lower.ends_with(".jpg")
            || url_lower.ends_with(".jpeg")
            || url_lower.ends_with(".png")
            || url_lower.ends_with(".gif")
            || url_lower.ends_with(".webp")
            || url_lower.ends_with(".svg")
            || url_lower.ends_with(".bmp")
        {
            Self::Image
        } else if url_lower.ends_with(".mp4")
            || url_lower.ends_with(".webm")
            || url_lower.ends_with(".avi")
            || url_lower.ends_with(".mkv")
            || url_lower.ends_with(".mov")
            || url_lower.ends_with(".flv")
        {
            Self::Video
        } else if url_lower.ends_with(".mp3")
            || url_lower.ends_with(".wav")
            || url_lower.ends_with(".flac")
            || url_lower.ends_with(".aac")
            || url_lower.ends_with(".ogg")
            || url_lower.ends_with(".m4a")
        {
            Self::Audio
        } else if url_lower.ends_with(".pdf")
            || url_lower.ends_with(".doc")
            || url_lower.ends_with(".docx")
            || url_lower.ends_with(".xls")
            || url_lower.ends_with(".xlsx")
            || url_lower.ends_with(".ppt")
            || url_lower.ends_with(".pptx")
            || url_lower.ends_with(".txt")
        {
            Self::Document
        } else if url_lower.ends_with(".zip")
            || url_lower.ends_with(".rar")
            || url_lower.ends_with(".7z")
            || url_lower.ends_with(".tar")
            || url_lower.ends_with(".gz")
        {
            Self::Archive
        } else if url_lower.ends_with(".exe")
            || url_lower.ends_with(".dmg")
            || url_lower.ends_with(".app")
            || url_lower.ends_with(".deb")
            || url_lower.ends_with(".rpm")
        {
            Self::Executable
        } else {
            Self::Other
        }
    }
}

/// Crawled resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Resource URL
    pub url: String,
    /// Resource type
    pub resource_type: ResourceType,
    /// Resource title/name
    pub title: Option<String>,
    /// File size if known
    pub size: Option<u64>,
    /// MIME type if known
    pub mime_type: Option<String>,
}

/// Crawler scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    /// Source URL
    pub source_url: String,
    /// Found resources
    pub resources: Vec<Resource>,
    /// Page title
    pub title: Option<String>,
    /// Scan depth
    pub depth: u32,
    /// Timestamp
    pub crawled_at: DateTime<Utc>,
}

/// Application error types
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AppError {
    #[error("Download error: {0}")]
    DownloadError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Task already exists: {0}")]
    TaskExists(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Crawler error: {0}")]
    CrawlerError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystemError(err.to_string())
    }
}

impl From<url::ParseError> for AppError {
    fn from(err: url::ParseError) -> Self {
        AppError::InvalidUrl(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

/// Result type alias for the application
pub type Result<T> = std::result::Result<T, AppError>;