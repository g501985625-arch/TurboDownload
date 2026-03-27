use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::chunk::worker::Worker;
use crate::chunk::ChunkProgress;
use crate::chunk::Strategy;
use crate::http::Client;
use crate::progress::Tracker;
use crate::{DownloadError, DownloadProgress, Result};
use mpsc::Sender;
use tokio::sync::mpsc;

/// Download scheduler for concurrent chunk downloads
pub struct Scheduler {
    max_concurrent: usize,
}

impl Scheduler {
    pub fn new(max_concurrent: usize) -> Self {
        Self { max_concurrent }
    }

    /// Execute concurrent download of all chunks
    pub async fn run<F>(
        &self,
        strategy: Strategy,
        url: String,
        client: Client,
        temp_dir: std::path::PathBuf,
        progress_callback: F,
    ) -> Result<Vec<std::path::PathBuf>>
    where
        F: Fn(DownloadProgress) + Send + Sync + 'static,
    {
        use std::sync::Arc;
        use tokio::sync::Semaphore;
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::new();
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let tracker = Arc::new(Tracker::new(strategy.chunks.iter().map(|c| c.size()).sum()));

        let (tx, mut rx): (Sender<ChunkProgress>, _) = mpsc::channel(100);
        let temp_paths = Arc::new(parking_lot::Mutex::new(Vec::new()));

        // Spawn progress aggregator
        let tracker_clone = tracker.clone();
        let callback = Arc::new(progress_callback);
        tokio::spawn(async move {
            while let Some(progress) = rx.recv().await {
                tracker_clone.update(progress.downloaded);
                let current_progress = tracker_clone.get_progress();
                callback(current_progress);
            }
        });

        let temp_paths_clone = temp_paths.clone();

        // Spawn workers for each chunk
        let temp_dir_clone = temp_dir.clone();

        for chunk in strategy.chunks {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let url = url.clone();
            let client = client.clone();
            let tx = tx.clone();
            let temp_paths_inner = temp_paths_clone.clone();
            let temp_dir = temp_dir_clone.clone();

            join_set.spawn(async move {
                let mut worker = Worker::new(chunk, url, client, &temp_dir);
                let temp_path = worker.temp_path().to_owned();

                let result = worker.download(tx).await;
                drop(permit);

                if result.is_ok() {
                    let mut paths = temp_paths_inner.lock();
                    paths.push(temp_path);
                }

                result
            });
        }

        // Wait for all tasks
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => return Err(e),
                Err(_) => return Err(DownloadError::Internal("Task join error".to_string())),
            }
        }

        let paths = temp_paths.lock().clone();
        Ok(paths)
    }
}

/// Merge temporary chunk files
pub async fn merge_files(temp_paths: &[std::path::PathBuf], output_path: &Path) -> Result<()> {
    let mut output = tokio::fs::File::create(output_path).await?;

    for temp_path in temp_paths {
        let mut input = tokio::fs::File::open(temp_path).await?;
        let mut buffer = vec![0u8; 64 * 1024];

        loop {
            let n = input.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            output.write_all(&buffer[..n]).await?;
        }
    }

    output.flush().await?;
    Ok(())
}

/// Cleanup temporary files
pub async fn cleanup(temp_paths: &[std::path::PathBuf]) {
    for path in temp_paths {
        let _ = tokio::fs::remove_file(path).await;
    }
}
