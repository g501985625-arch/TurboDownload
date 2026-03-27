pub mod manager;
pub mod scheduler;
pub mod task;

pub use manager::{Downloader, DownloaderBuilder, Manager};
pub use scheduler::{cleanup, merge_files, Scheduler};
pub use task::{DownloadConfig, DownloadResult, Task, TaskState};
