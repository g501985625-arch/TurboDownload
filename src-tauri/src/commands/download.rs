// Download commands - Real implementation using turbo-downloader
use crate::commands::AppState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use turbo_downloader::{DownloadConfig, TaskState};

/// Download task status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub total_size: u64,
    pub downloaded: u64,
    pub status: String,
    pub speed: u64,
    pub error: Option<String>,
}

impl From<&turbo_downloader::Task> for DownloadTask {
    fn from(task: &turbo_downloader::Task) -> Self {
        let filename = task.config.output_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        Self {
            id: task.id.clone(),
            url: task.config.url.clone(),
            filename,
            total_size: task.file_size,
            downloaded: 0,
            status: format!("{:?}", task.state()),
            speed: 0,
            error: None,
        }
    }
}

/// Download progress event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgressEvent {
    pub task_id: String,
    pub downloaded: u64,
    pub total_size: u64,
    pub speed: u64,
    pub status: String,
}

/// Start a new download
#[tauri::command]
pub fn start_download(
    url: String,
    filename: String,
    app_handle: AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<DownloadTask, String> {
    log::info!("Starting download: {} -> {}", url, filename);

    let task_id = uuid::Uuid::new_v4().to_string();

    // Create download config
    let output_path = PathBuf::from("./downloads").join(&filename);
    let config = DownloadConfig {
        id: task_id.clone(),
        url: url.clone(),
        output_path,
        threads: 4,
        chunk_size: 1024 * 1024,
        resume_support: true,
        user_agent: Some("TurboDownload/1.0".to_string()),
        headers: std::collections::HashMap::new(),
        speed_limit: 0,
    };

    // Create task in manager
    let task_info = {
        let manager = state.download_manager.blocking_lock();
        
        // Create task - use block_on to run the async create_task
        let tid = tokio::runtime::Handle::current()
            .block_on(manager.create_task(config))
            .map_err(|e: turbo_downloader::DownloadError| e.to_string())?;

        let task = manager
            .get_task(&tid)
            .ok_or("Task not found after creation")?;

        DownloadTask {
            id: tid.clone(),
            url: task.config.url.clone(),
            filename: filename.clone(),
            total_size: task.file_size,
            downloaded: 0,
            status: "created".to_string(),
            speed: 0,
            error: None,
        }
    };

    // Spawn background download task
    let state_clone = state.inner().clone();
    let app_handle_clone = app_handle.clone();
    let task_id_clone = task_info.id.clone();

    tokio::spawn(async move {
        let manager = state_clone.download_manager.blocking_lock();
        if let Some(task) = manager.get_task(&task_id_clone) {
            let _ = app_handle_clone.emit(
                "download-progress",
                DownloadProgressEvent {
                    task_id: task_id_clone,
                    downloaded: 0,
                    total_size: task.file_size,
                    speed: 0,
                    status: "running".to_string(),
                },
            );
        }
    });

    Ok(task_info)
}

/// Pause a download
#[tauri::command]
pub fn pause_download(
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    log::info!("Pausing download: {}", id);

    let manager = state.download_manager.blocking_lock();
    if let Some(task) = manager.get_task(&id) {
        task.set_state(TaskState::Paused);
        Ok(())
    } else {
        Err(format!("Task not found: {}", id))
    }
}

/// Resume a paused download
#[tauri::command]
pub fn resume_download(
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    log::info!("Resuming download: {}", id);

    let manager = state.download_manager.blocking_lock();
    if let Some(task) = manager.get_task(&id) {
        task.set_state(TaskState::Downloading);
        Ok(())
    } else {
        Err(format!("Task not found: {}", id))
    }
}

/// Cancel a download
#[tauri::command]
pub fn cancel_download(
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    log::info!("Canceling download: {}", id);

    let manager = state.download_manager.blocking_lock();
    manager
        .remove_task(&id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get download status
#[tauri::command]
pub fn get_download_status(
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<DownloadTask, String> {
    let manager = state.download_manager.blocking_lock();
    let task = manager
        .get_task(&id)
        .ok_or(format!("Task not found: {}", id))?;

    Ok(DownloadTask::from(&*task))
}

/// List all downloads
#[tauri::command]
pub fn list_downloads(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<DownloadTask>, String> {
    let manager = state.download_manager.blocking_lock();
    let task_ids = manager.list_tasks();

    let mut tasks = Vec::new();
    for task_id in task_ids {
        if let Some(task) = manager.get_task(&task_id) {
            tasks.push(DownloadTask::from(&*task));
        }
    }

    Ok(tasks)
}