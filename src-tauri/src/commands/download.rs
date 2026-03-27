//! Download commands for Tauri

use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

use crate::models::{DownloadConfig, DownloadProgress, DownloadTask, Result};
use crate::services::DownloadManager;

/// Application state
pub struct AppState {
    pub download_manager: Arc<RwLock<DownloadManager>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            download_manager: Arc::new(RwLock::new(DownloadManager::new())),
        }
    }
}

/// Add a new download task
#[tauri::command]
pub async fn add_download(
    url: String,
    config: Option<DownloadConfig>,
    state: State<'_, AppState>,
) -> Result<String> {
    let config = config.unwrap_or_default();
    let manager = state.download_manager.read().await;
    manager.add_task(url, config).await
}

/// Start a download task
#[tauri::command]
pub async fn start_download(
    id: String,
    state: State<'_, AppState>,
) -> Result<()> {
    let manager = state.download_manager.read().await;
    manager.start_task(&id).await
}

/// Pause a download task
#[tauri::command]
pub async fn pause_download(
    id: String,
    state: State<'_, AppState>,
) -> Result<()> {
    let manager = state.download_manager.read().await;
    manager.pause_task(&id).await
}

/// Resume a paused download task
#[tauri::command]
pub async fn resume_download(
    id: String,
    state: State<'_, AppState>,
) -> Result<()> {
    let manager = state.download_manager.read().await;
    manager.resume_task(&id).await
}

/// Cancel a download task
#[tauri::command]
pub async fn cancel_download(
    id: String,
    state: State<'_, AppState>,
) -> Result<()> {
    let manager = state.download_manager.read().await;
    manager.cancel_task(&id).await
}

/// Remove a download task
#[tauri::command]
pub async fn remove_download(
    id: String,
    state: State<'_, AppState>,
) -> Result<()> {
    let manager = state.download_manager.read().await;
    manager.remove_task(&id).await
}

/// Get download task by ID
#[tauri::command]
pub async fn get_download(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<DownloadTask>> {
    let manager = state.download_manager.read().await;
    Ok(manager.get_task(&id).await)
}

/// Get all download tasks
#[tauri::command]
pub async fn get_all_downloads(
    state: State<'_, AppState>,
) -> Result<Vec<DownloadTask>> {
    let manager = state.download_manager.read().await;
    Ok(manager.get_all_tasks().await)
}

/// Get download progress
#[tauri::command]
pub async fn get_download_progress(
    id: String,
    state: State<'_, AppState>,
) -> Result<DownloadProgress> {
    let manager = state.download_manager.read().await;
    manager.get_progress(&id).await
}