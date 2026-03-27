//! Event system for download progress and status

mod types;
mod emitter;

pub use types::{DownloadEvent, DownloadStatus};
pub use emitter::EventEmitter;
