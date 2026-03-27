//! Turbo Downloader - High-performance multi-threaded download engine
//!
//! A Rust library for high-performance multi-threaded downloads with
//! resume support, progress tracking, and chunk management.
//!
//! ## Features
//!
//! - Multi-threaded chunk downloads
//! - Resume support with state persistence
//! - Real-time progress tracking
//! - Configurable concurrency
//! - Clean API design

pub mod error;
pub mod http;
pub mod range;
pub mod chunk;
pub mod commands;
pub mod download;
pub mod downloader;
pub mod event;
pub mod pool;
pub mod progress;
pub mod resume;
pub mod storage;

// Re-export main types
pub use error::{DownloadError, Result};
pub use http::{Client, ClientConfig, HeadResponse};
pub use range::{RangeClient, RangeClientConfig, RangeSupport};
pub use chunk::{Chunk, ChunkManager, ChunkProgress, ChunkState, Strategy, Worker};
pub use pool::WorkerPool;
pub use event::{DownloadEvent, DownloadStatus, EventEmitter};
pub use storage::{ChunkWriter, FileMerger, StateManager, DownloadState};
pub use download::{
    cleanup, merge_files, DownloadConfig, DownloadResult, Downloader, DownloaderBuilder, Manager,
    Scheduler, Task, TaskState,
};
pub use progress::{DownloadProgress, ProgressCallback, SpeedCalculator, Tracker};
pub use resume::{Recovery, ResumeState};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum supported thread count
pub const MAX_THREADS: u32 = 32;

/// Default chunk size (bytes)
pub const DEFAULT_CHUNK_SIZE: u64 = 1024 * 1024; // 1MB

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}