//! TurboDownload Library
//!
//! Core functionality for the download manager

pub mod commands;
pub mod models;
pub mod services;

// Re-export commonly used types
pub use models::{DownloadTask, DownloadConfig, DownloadStatus, DownloadProgress, Resource, ResourceType};