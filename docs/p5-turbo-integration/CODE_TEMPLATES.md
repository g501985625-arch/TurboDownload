# P5: turbo-integration 代码模板

## 概述

本文档提供核心代码模板，可直接复制使用。

---

## 1. 错误类型模板

### 文件: `src/error.rs`

```rust
use thiserror::Error;

/// 系统集成层错误类型
#[derive(Debug, Error)]
pub enum IntegrationError {
    /// 命令执行错误
    #[error("Command error: {0}")]
    CommandError(String),

    /// 配置错误
    #[error("Config error: {0}")]
    ConfigError(String),

    /// 文件系统错误
    #[error("File system error: {0}")]
    FileSystemError(String),

    /// 通知错误
    #[error("Notification error: {0}")]
    NotificationError(String),

    /// 任务不存在
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    /// 内部错误
    #[error("Internal error: {0}")]
    InternalError(String),
}

// 实现 From 转换
impl From<std::io::Error> for IntegrationError {
    fn from(err: std::io::Error) -> Self {
        IntegrationError::FileSystemError(err.to_string())
    }
}

impl From<toml::de::Error> for IntegrationError {
    fn from(err: toml::de::Error) -> Self {
        IntegrationError::ConfigError(err.to_string())
    }
}

impl From<serde_json::Error> for IntegrationError {
    fn from(err: serde_json::Error) -> Self {
        IntegrationError::InternalError(err.to_string())
    }
}

/// Tauri 命令结果类型
pub type CommandResult<T> = Result<T, IntegrationError>;
```

---

## 2. 下载命令模板

### 文件: `src/commands/download.rs`

```rust
use tauri::{AppHandle, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use turbo_manager::{DownloadManager, DownloadConfig, TaskInfo, DownloadProgress};
use crate::error::{IntegrationError, CommandResult};

/// 下载配置 JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfigJson {
    pub output_path: Option<String>,
    pub threads: Option<usize>,
    pub chunk_size: Option<u64>,
    pub resume_support: Option<bool>,
    pub user_agent: Option<String>,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub speed_limit: Option<u64>,
}

/// 任务信息 JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfoJson {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub output_path: String,
    pub total_size: u64,
    pub downloaded_size: u64,
    pub state: String,
    pub speed: u64,
    pub eta: Option<u64>,
    pub error: Option<String>,
}

/// 添加下载任务
#[tauri::command]
pub async fn add_download(
    url: String,
    config: Option<DownloadConfigJson>,
    manager: State<'_, Arc<dyn DownloadManager>>,
) -> CommandResult<String> {
    let download_config = config.map(|c| DownloadConfig {
        id: uuid::Uuid::new_v4().to_string(),
        url: url.clone(),
        output_path: c.output_path
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                let filename = url.split('/').last().unwrap_or("download");
                std::env::current_dir().unwrap().join(filename)
            }),
        threads: c.threads.unwrap_or(4),
        chunk_size: c.chunk_size.unwrap_or(0),
        resume_support: c.resume_support.unwrap_or(true),
        user_agent: c.user_agent,
        headers: c.headers.unwrap_or_default(),
        speed_limit: c.speed_limit.unwrap_or(0),
    }).unwrap_or_default();

    let task_id = manager.add_task(download_config).await
        .map_err(|e| IntegrationError::CommandError(e.to_string()))?;
    
    Ok(task_id)
}

/// 开始下载
#[tauri::command]
pub async fn start_download(
    task_id: String,
    manager: State<'_, Arc<dyn DownloadManager>>,
) -> CommandResult<()> {
    manager.start_task(&task_id).await
        .map_err(|e| IntegrationError::CommandError(e.to_string()))
}

/// 暂停下载
#[tauri::command]
pub async fn pause_download(
    task_id: String,
    manager: State<'_, Arc<dyn DownloadManager>>,
) -> CommandResult<()> {
    manager.pause_task(&task_id).await
        .map_err(|e| IntegrationError::CommandError(e.to_string()))
}

/// 恢复下载
#[tauri::command]
pub async fn resume_download(
    task_id: String,
    manager: State<'_, Arc<dyn DownloadManager>>,
) -> CommandResult<()> {
    manager.resume_task(&task_id).await
        .map_err(|e| IntegrationError::CommandError(e.to_string()))
}

/// 取消下载
#[tauri::command]
pub async fn cancel_download(
    task_id: String,
    manager: State<'_, Arc<dyn DownloadManager>>,
) -> CommandResult<()> {
    manager.cancel_task(&task_id).await
        .map_err(|e| IntegrationError::CommandError(e.to_string()))
}

/// 移除下载任务
#[tauri::command]
pub async fn remove_download(
    task_id: String,
    manager: State<'_, Arc<dyn DownloadManager>>,
) -> CommandResult<()> {
    manager.remove_task(&task_id).await
        .map_err(|e| IntegrationError::CommandError(e.to_string()))
}

/// 获取下载进度
#[tauri::command]
pub async fn get_download_progress(
    task_id: String,
    manager: State<'_, Arc<dyn DownloadManager>>,
) -> CommandResult<Option<DownloadProgressJson>> {
    let progress = manager.get_progress(&task_id).await
        .map_err(|e| IntegrationError::CommandError(e.to_string()))?;
    
    Ok(progress.map(|p| DownloadProgressJson {
        task_id: p.task_id,
        downloaded: p.downloaded,
        total: p.total,
        speed: p.speed,
        eta: p.eta,
        state: format!("{:?}", p.state),
    }))
}

/// 下载进度 JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgressJson {
    pub task_id: String,
    pub downloaded: u64,
    pub total: u64,
    pub speed: u64,
    pub eta: Option<u64>,
    pub state: String,
}

/// 获取所有下载任务
#[tauri::command]
pub async fn get_all_downloads(
    manager: State<'_, Arc<dyn DownloadManager>>,
) -> CommandResult<Vec<TaskInfoJson>> {
    let tasks = manager.list_tasks().await
        .map_err(|e| IntegrationError::CommandError(e.to_string()))?;
    
    Ok(tasks.into_iter().map(|t| TaskInfoJson {
        id: t.id,
        url: t.config.url,
        filename: t.config.output_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        output_path: t.config.output_path.to_string_lossy().to_string(),
        total_size: t.progress.as_ref().map(|p| p.total).unwrap_or(0),
        downloaded_size: t.progress.as_ref().map(|p| p.downloaded).unwrap_or(0),
        state: format!("{:?}", t.state),
        speed: t.progress.as_ref().map(|p| p.speed).unwrap_or(0),
        eta: t.progress.and_then(|p| p.eta),
        error: None,
    }).collect())
}
```

---

## 3. 配置类型模板

### 文件: `src/config/types.rs`

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 默认下载目录
    pub download_dir: PathBuf,
    /// 最大并发下载数
    pub max_concurrent_downloads: usize,
    /// 默认线程数
    pub default_threads: usize,
    /// 默认分片大小 (字节)
    pub default_chunk_size: u64,
    /// 速度限制 (字节/秒)
    pub speed_limit: u64,
    /// 通知配置
    pub notifications: NotificationConfig,
    /// 爬虫配置
    pub crawler: CrawlerConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            download_dir: dirs::download_dir()
                .unwrap_or_else(|| PathBuf::from(".")),
            max_concurrent_downloads: 3,
            default_threads: 4,
            default_chunk_size: 0, // 自动
            speed_limit: 0,        // 不限制
            notifications: NotificationConfig::default(),
            crawler: CrawlerConfig::default(),
        }
    }
}

/// 通知配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// 是否启用通知
    pub enabled: bool,
    /// 完成时通知
    pub on_complete: bool,
    /// 错误时通知
    pub on_error: bool,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            on_complete: true,
            on_error: true,
        }
    }
}

/// 爬虫配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerConfig {
    /// 请求超时 (秒)
    pub timeout: u64,
    /// 最大深度
    pub max_depth: usize,
    /// 最大页面数
    pub max_pages: usize,
    /// User-Agent
    pub user_agent: String,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        Self {
            timeout: 30,
            max_depth: 3,
            max_pages: 100,
            user_agent: format!(
                "TurboDownload/0.1.0 (Rust)"
            ),
        }
    }
}
```

---

## 4. 配置管理器模板

### 文件: `src/config/manager.rs`

```rust
use std::path::PathBuf;
use std::fs;
use directories::ProjectDirs;
use crate::config::types::AppConfig;
use crate::error::{IntegrationError, CommandResult};

/// 获取配置文件路径
pub fn get_config_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("com", "turbodownload", "TurboDownload")
        .expect("Could not determine config directory");
    
    project_dirs.config_dir().join("config.toml")
}

/// 加载配置
pub fn load() -> CommandResult<AppConfig> {
    let path = get_config_path();
    
    if !path.exists() {
        // 返回默认配置
        return Ok(AppConfig::default());
    }
    
    let content = fs::read_to_string(&path)?;
    let config: AppConfig = toml::from_str(&content)?;
    
    Ok(config)
}

/// 保存配置
pub fn save(config: &AppConfig) -> CommandResult<()> {
    let path = get_config_path();
    
    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    
    Ok(())
}

/// 获取配置命令
#[tauri::command]
pub async fn get_config() -> CommandResult<AppConfig> {
    load()
}

/// 保存配置命令
#[tauri::command]
pub async fn save_config(config: AppConfig) -> CommandResult<()> {
    save(&config)
}
```

---

## 5. 文件系统命令模板

### 文件: `src/fs/dialog.rs`

```rust
use tauri::AppHandle;
use crate::error::{IntegrationError, CommandResult};

/// 选择目录对话框
#[tauri::command]
pub async fn select_directory(
    app: AppHandle,
) -> CommandResult<Option<String>> {
    let (tx, rx) = std::sync::mpsc::channel();
    
    tauri::api::dialog::FileDialogBuilder::new()
        .set_title("选择下载目录")
        .pick_folder(move |folder| {
            let _ = tx.send(folder);
        });
    
    let result = rx.recv()
        .map_err(|_| IntegrationError::InternalError("Dialog cancelled".into()))?;
    
    Ok(result.map(|p| p.to_string_lossy().to_string()))
}

/// 获取默认下载目录
#[tauri::command]
pub async fn get_default_download_dir() -> CommandResult<String> {
    let path = dirs::download_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    
    Ok(path.to_string_lossy().to_string())
}

/// 检查文件是否存在
#[tauri::command]
pub async fn file_exists(path: String) -> CommandResult<bool> {
    Ok(std::path::Path::new(&path).exists())
}

/// 确保目录存在
#[tauri::command]
pub async fn ensure_directory(path: String) -> CommandResult<()> {
    std::fs::create_dir_all(&path)?;
    Ok(())
}
```

---

## 6. 事件发送模板

### 文件: `src/events/emitter.rs`

```rust
use tauri::{AppHandle, Manager};
use serde::Serialize;

/// 事件名称常量
pub mod events {
    pub const DOWNLOAD_PROGRESS: &str = "download:progress";
    pub const DOWNLOAD_COMPLETED: &str = "download:completed";
    pub const DOWNLOAD_FAILED: &str = "download:failed";
    pub const TASK_STATE_CHANGED: &str = "task:state_changed";
    pub const CRAWL_PROGRESS: &str = "crawl:progress";
    pub const CRAWL_COMPLETED: &str = "crawl:completed";
}

/// 发送下载进度事件
pub fn emit_progress(app: &AppHandle, task_id: &str, downloaded: u64, total: u64, speed: u64) {
    let payload = serde_json::json!({
        "task_id": task_id,
        "downloaded": downloaded,
        "total": total,
        "speed": speed,
        "percent": if total > 0 { (downloaded as f64 / total as f64 * 100.0) as u8 } else { 0 },
    });
    
    let _ = app.emit(events::DOWNLOAD_PROGRESS, payload);
}

/// 发送下载完成事件
pub fn emit_completed(app: &AppHandle, task_id: &str, output_path: &str) {
    let payload = serde_json::json!({
        "task_id": task_id,
        "output_path": output_path,
    });
    
    let _ = app.emit(events::DOWNLOAD_COMPLETED, payload);
}

/// 发送下载失败事件
pub fn emit_failed(app: &AppHandle, task_id: &str, error: &str) {
    let payload = serde_json::json!({
        "task_id": task_id,
        "error": error,
    });
    
    let _ = app.emit(events::DOWNLOAD_FAILED, payload);
}

/// 发送任务状态变更事件
pub fn emit_state_changed(app: &AppHandle, task_id: &str, state: &str) {
    let payload = serde_json::json!({
        "task_id": task_id,
        "state": state,
    });
    
    let _ = app.emit(events::TASK_STATE_CHANGED, payload);
}
```

---

## 7. 通知模板

### 文件: `src/notification/notify.rs`

```rust
use tauri::AppHandle;
use crate::error::{IntegrationError, CommandResult};

/// 显示系统通知
#[tauri::command]
pub async fn show_notification(
    title: String,
    body: String,
    app: AppHandle,
) -> CommandResult<()> {
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map_err(|e| IntegrationError::NotificationError(e.to_string()))?;
    
    Ok(())
}
```

---

## 8. lib.rs 入口模板

### 文件: `src/lib.rs`

```rust
//! TurboDownload 系统集成层
//! 
//! 提供 Tauri 命令封装、文件操作、配置管理等功能。

pub mod commands;
pub mod events;
pub mod config;
pub mod fs;
pub mod notification;
pub mod error;

// 重导出常用类型
pub use error::{IntegrationError, CommandResult};
pub use config::types::AppConfig;
pub use events::emitter;

/// 注册所有 Tauri 命令
pub fn register_commands() -> impl Fn(tauri::invoke_handler::InvokeHandler) {
    tauri::generate_handler![
        // 下载命令
        commands::download::add_download,
        commands::download::start_download,
        commands::download::pause_download,
        commands::download::resume_download,
        commands::download::cancel_download,
        commands::download::remove_download,
        commands::download::get_download_progress,
        commands::download::get_all_downloads,
        // 爬虫命令
        commands::crawler::crawl_url,
        commands::crawler::scan_site,
        commands::crawler::cancel_scan,
        // 系统命令
        commands::system::get_config,
        commands::system::save_config,
        commands::system::select_directory,
        commands::system::get_default_download_dir,
        commands::system::file_exists,
        commands::system::show_notification,
    ]
}
```