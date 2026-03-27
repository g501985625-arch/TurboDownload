//! File system commands for Tauri

use std::path::PathBuf;

use crate::models::{Result, AppError};

/// Get the default download directory
#[tauri::command]
pub fn get_default_download_dir() -> Result<String> {
    let dir = dirs::download_dir()
        .ok_or_else(|| AppError::FileSystemError("Cannot determine download directory".to_string()))?;
    
    Ok(dir.to_string_lossy().to_string())
}

/// Get user's home directory
#[tauri::command]
pub fn get_home_dir() -> Result<String> {
    let dir = dirs::home_dir()
        .ok_or_else(|| AppError::FileSystemError("Cannot determine home directory".to_string()))?;
    
    Ok(dir.to_string_lossy().to_string())
}

/// Check if a file exists
#[tauri::command]
pub fn file_exists(path: String) -> bool {
    PathBuf::from(&path).exists()
}

/// Get file size in bytes
#[tauri::command]
pub fn get_file_size(path: String) -> Result<u64> {
    let metadata = std::fs::metadata(&path)
        .map_err(|e| AppError::FileSystemError(format!("Cannot get file metadata: {}", e)))?;
    
    Ok(metadata.len())
}

/// Delete a file
#[tauri::command]
pub fn delete_file(path: String) -> Result<()> {
    std::fs::remove_file(&path)
        .map_err(|e| AppError::FileSystemError(format!("Cannot delete file: {}", e)))?;
    
    Ok(())
}

/// Create a directory
#[tauri::command]
pub fn create_dir(path: String) -> Result<()> {
    std::fs::create_dir_all(&path)
        .map_err(|e| AppError::FileSystemError(format!("Cannot create directory: {}", e)))?;
    
    Ok(())
}

/// List files in a directory
#[tauri::command]
pub fn list_dir(path: String) -> Result<Vec<String>> {
    let entries = std::fs::read_dir(&path)
        .map_err(|e| AppError::FileSystemError(format!("Cannot read directory: {}", e)))?;
    
    let files: Vec<String> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();
    
    Ok(files)
}

/// Open file in system file manager
#[tauri::command]
pub fn open_in_file_manager(path: String) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|e| AppError::FileSystemError(format!("Cannot open file manager: {}", e)))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| AppError::FileSystemError(format!("Cannot open file manager: {}", e)))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| AppError::FileSystemError(format!("Cannot open file manager: {}", e)))?;
    }
    
    Ok(())
}