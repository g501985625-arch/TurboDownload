// Update commands for TurboDownload auto-update functionality
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_updater::UpdaterExt;

/// Update information returned to the frontend
#[derive(Serialize, Debug, Clone)]
pub struct UpdateInfo {
    /// Available version number (e.g., "1.1.0")
    #[serde(rename = "version")]
    pub version: String,
    /// Current app version
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    /// Release notes in markdown format
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Release date string
    #[serde(rename = "date")]
    pub date: Option<String>,
    /// Download URL
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
}

/// Check for available updates
/// 
/// Returns Some(UpdateInfo) if update is available, None if running latest version
#[tauri::command]
pub async fn check_update(app: AppHandle) -> Result<Option<UpdateInfo>, String> {
    log::info!("Checking for updates...");
    
    let updater = app.updater()
        .map_err(|e| {
            log::error!("Failed to get updater: {}", e);
            format!("Failed to initialize updater: {}", e)
        })?;
    
    match updater.check().await {
        Ok(Some(update)) => {
            log::info!("Update available: {}", update.version);
            
            // Format date
            let date: Option<String> = update.date.map(|d| {
                d.to_string()
            });
            
            Ok(Some(UpdateInfo {
                version: update.version,
                current_version: app.package_info().version.to_string(),
                notes: update.body,
                date,
                download_url: update.target,
            }))
        }
        Ok(None) => {
            log::info!("No update available, running latest version");
            Ok(None)
        }
        Err(e) => {
            log::error!("Failed to check update: {}", e);
            Err(format!("Failed to check for updates: {}", e))
        }
    }
}

use std::sync::Arc;
use tokio::sync::Mutex;

/// Structure for update progress events
#[derive(serde::Serialize, Debug, Clone)]
pub struct UpdateProgressEvent {
    pub percent: u32,
    pub downloaded: u64,
    pub total: Option<u64>,
    pub status: String,
}

/// Download the available update with progress reporting, retry logic, and background execution
/// 
/// Emits "update-download-progress" events to the window
#[tauri::command]
pub async fn download_update(
    app: AppHandle,
    window: tauri::Window,
) -> Result<(), String> {
    log::info!("Starting update download...");
    
    let updater = app.updater()
        .map_err(|e| format!("Failed to get updater: {}", e))?;
    
    // Check for available updates
    let update = updater.check().await
        .map_err(|e| format!("Failed to check for updates: {}", e))?
        .ok_or_else(|| "No update available".to_string())?;
    
    log::info!("Downloading update version: {}", update.version);
    
    // Attempt to download with retry logic (up to 3 times)
    let mut attempt = 0;
    let max_attempts = 3;
    
    loop {
        attempt += 1;
        log::info!("Attempt {} of {} to download update", attempt, max_attempts);
        
        // Create a progress tracking structure
        let progress_tracker = Arc::new(Mutex::new(UpdateProgressTracker {
            downloaded: 0,
            total: 0,
            window: window.clone(),
        }));

        let progress_clone = progress_tracker.clone();
        let window_clone = window.clone();
        
        match update.download_and_install(
            // Progress callback: FnMut(chunk_length, Option<content_length>)
            move |chunk_length: usize, content_length: Option<u64>| {
                let mut tracker = progress_clone.try_lock().unwrap();
                
                tracker.downloaded += chunk_length as u64;
                if let Some(total) = content_length {
                    tracker.total = total;
                }
                
                let percent = if tracker.total > 0 { 
                    ((tracker.downloaded as f64 / tracker.total as f64) * 100.0) as u32 
                } else { 
                    0 
                };
                
                // Emit progress event
                let _ = tracker.window.emit("update-download-progress", UpdateProgressEvent {
                    percent,
                    downloaded: tracker.downloaded,
                    total: Some(tracker.total),
                    status: "downloading".to_string(),
                });
                
                log::debug!("Download progress: {}% ({} / {:?})", percent, tracker.downloaded, content_length);
            },
            // Completion callback: FnOnce() - called when download is complete
            move || {
                log::info!("Download and install completed");
                // Emit final progress event
                let _ = window_clone.emit("update-download-progress", UpdateProgressEvent {
                    percent: 100,
                    downloaded: 0, // At this point, we don't have exact final size
                    total: None,
                    status: "completed".to_string(),
                });
            }
        ).await {
            Ok(_) => {
                log::info!("Update downloaded and installed successfully");
                return Ok(());
            }
            Err(e) => {
                log::error!("Failed to download update on attempt {}: {}", attempt, e);
                
                // Emit error event
                let _ = window.emit("update-download-progress", UpdateProgressEvent {
                    percent: 0,
                    downloaded: 0,
                    total: None,
                    status: "error".to_string(),
                });
                
                if attempt >= max_attempts {
                    return Err(format!("Failed to download update after {} attempts: {}", max_attempts, e));
                }
                
                // Wait before retrying (exponential backoff)
                tokio::time::sleep(tokio::time::Duration::from_secs(2_u64.pow(attempt))).await;
            }
        }
    }
}

/// Helper struct to track download progress
struct UpdateProgressTracker {
    downloaded: u64,
    total: u64,
    window: tauri::Window,
}

/// Get the current application version
#[tauri::command]
pub fn get_current_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

/// Install an already downloaded update (if downloaded externally)
/// 
/// Note: In Tauri v2, the updater handles installation automatically after download.
/// This command is kept for consistency but mainly just checks if an update is ready.
#[tauri::command]
pub async fn install_update(app: AppHandle) -> Result<(), String> {
    log::info!("Installing update...");
    
    // In Tauri v2, the download_and_install handles both download and install
    // This is a no-op if no update has been downloaded
    // We check if there's an update available and trigger the install
    
    let updater = app.updater()
        .map_err(|e| format!("Failed to get updater: {}", e))?;
    
    // Try to check for update - if one exists, download and install it
    if let Some(update) = updater.check().await
        .map_err(|e| format!("Failed to check for updates: {}", e))? {
        
        log::info!("Found update version {}, downloading and installing...", update.version);
        
        // Download and install
        update.download_and_install(
            |_chunk_length, _content_length| {
                // Progress handled elsewhere if needed
            },
            || {
                log::info!("Installation complete");
            }
        ).await
        .map_err(|e| format!("Failed to install update: {}", e))?;
    }
    
    log::info!("Update installation complete");
    Ok(())
}