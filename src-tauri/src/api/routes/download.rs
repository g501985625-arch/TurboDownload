use axum::{
    Json, Router,
    routing::{get, post, delete},
    extract::Path,
    extract::State,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use std::sync::Arc;
use turbo_downloader::{DownloadConfig, TaskState, Manager};
use std::path::PathBuf;
use uuid::Uuid;

use crate::api::server::{WsState, DownloadEvent, broadcast_event};

/// 错误响应结构
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    fn bad_request(code: &str, message: &str) -> Self {
        Self::new(code, message)
    }

    fn not_found(code: &str, message: &str) -> Self {
        Self::new(code, message)
    }

    fn internal_error(message: &str) -> Self {
        Self::new("INTERNAL_ERROR", message)
    }
}

/// API 状态 - 包含下载管理器和WebSocket状态
pub struct ApiState {
    pub download_manager: Arc<Mutex<Manager>>,
    pub ws_state: Arc<WsState>,
}

/// 下载请求
#[derive(Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub filename: Option<String>,
    pub threads: Option<u32>,
}

/// 下载响应
#[derive(Serialize)]
pub struct DownloadResponse {
    pub task_id: String,
    pub status: String,
    pub message: String,
}

/// 下载状态
#[derive(Serialize)]
pub struct DownloadStatus {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub progress: f64,
    pub speed: u64,
    pub status: String,
    pub total_size: u64,
    pub downloaded: u64,
}

/// 输入验证函数

/// 验证 URL 格式（必须 http:// 或 https://）
fn validate_url(url: &str) -> Result<(), String> {
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }

    let url_lower = url.to_lowercase();
    if !url_lower.starts_with("http://") && !url_lower.starts_with("https://") {
        return Err("URL must start with http:// or https://".to_string());
    }

    // 验证 URL 格式是否有效
    if let Err(_) = url::Url::parse(url) {
        return Err("Invalid URL format".to_string());
    }

    Ok(())
}

/// 验证线程数（1-32）
fn validate_threads(threads: Option<u32>) -> Result<u32, String> {
    match threads {
        Some(t) => {
            if t < 1 || t > 32 {
                return Err("Thread count must be between 1 and 32".to_string());
            }
            Ok(t)
        }
        None => Ok(4), // 默认 4 线程
    }
}

/// 验证文件名
fn validate_filename(filename: Option<String>) -> Result<String, String> {
    match filename {
        Some(name) => {
            if name.is_empty() {
                return Err("Filename cannot be empty".to_string());
            }

            // 检查非法字符
            let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
            for c in invalid_chars {
                if name.contains(c) {
                    return Err(format!("Filename contains invalid character: {}", c));
                }
            }

            // 检查路径遍历
            if name.contains("..") {
                return Err("Filename cannot contain path traversal".to_string());
            }

            Ok(name)
        }
        None => {
            // 如果没有提供文件名，使用默认名称
            Ok(format!("download_{}", Uuid::new_v4()))
        }
    }
}

/// 启动下载
pub async fn start_download(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<DownloadRequest>,
) -> Result<Json<DownloadResponse>, Json<ErrorResponse>> {
    // 1. 验证 URL
    if let Err(e) = validate_url(&req.url) {
        return Err(Json(ErrorResponse::bad_request("INVALID_URL", &e)));
    }

    // 2. 验证线程数
    let threads = match validate_threads(req.threads) {
        Ok(t) => t,
        Err(e) => return Err(Json(ErrorResponse::bad_request("INVALID_THREADS", &e))),
    };

    // 3. 验证文件名
    let filename = match validate_filename(req.filename) {
        Ok(f) => f,
        Err(e) => return Err(Json(ErrorResponse::bad_request("INVALID_FILENAME", &e))),
    };

    // 4. 创建下载任务
    let task_id = Uuid::new_v4().to_string();
    let output_path = PathBuf::from("./downloads").join(&filename);

    let config = DownloadConfig {
        id: task_id.clone(),
        url: req.url.clone(),
        output_path,
        threads,
        chunk_size: 1024 * 1024,
        resume_support: true,
        user_agent: Some("TurboDownload/1.0".to_string()),
        headers: std::collections::HashMap::new(),
        speed_limit: 0,
    };

    // 5. 调用 turbo-downloader 创建任务
    let result = {
        let manager = state.download_manager.lock().await;
        manager.create_task(config).await
    };

    match result {
        Ok(tid) => {
            log::info!("Download task created: {} -> {}", tid, req.url);
            
            // 发送进度事件表示下载开始
            let event = DownloadEvent::Progress {
                task_id: tid.clone(),
                downloaded: 0,
                total: 0,
                speed: 0,
            };
            let _ = broadcast_event(&state.ws_state, event);

            Ok(Json(DownloadResponse {
                task_id: tid,
                status: "created".to_string(),
                message: "Download started successfully".to_string(),
            }))
        }
        Err(e) => {
            log::error!("Failed to create download: {}", e);
            
            // 发送错误事件
            let event = DownloadEvent::Error {
                task_id: Uuid::new_v4().to_string(), // 使用新ID因为任务未创建成功
                message: e.to_string(),
            };
            let _ = broadcast_event(&state.ws_state, event);

            Err(Json(ErrorResponse::internal_error(&e.to_string())))
        }
    }
}

/// 获取下载状态
pub async fn get_download_status(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> Result<Json<DownloadStatus>, Json<ErrorResponse>> {
    let manager = state.download_manager.lock().await;

    let task = manager
        .get_task(&id)
        .ok_or_else(|| Json(ErrorResponse::not_found("TASK_NOT_FOUND", &format!("Task not found: {}", id))))?;

    let filename = task.config.output_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let state_str = format!("{:?}", task.state());
    let progress = if task.file_size > 0 {
        (task.downloaded as f64 / task.file_size as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(DownloadStatus {
        id: task.id.clone(),
        url: task.config.url.clone(),
        filename,
        progress,
        speed: task.speed(),
        status: state_str,
        total_size: task.file_size,
        downloaded: task.downloaded,
    }))
}

/// 暂停下载
pub async fn pause_download(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> Result<Json<DownloadResponse>, Json<ErrorResponse>> {
    let manager = state.download_manager.lock().await;

    let task = manager
        .get_task(&id)
        .ok_or_else(|| Json(ErrorResponse::not_found("TASK_NOT_FOUND", &format!("Task not found: {}", id))))?;

    task.set_state(TaskState::Paused);

    log::info!("Download paused: {}", id);

    // 发送暂停事件到WebSocket客户端
    let event = DownloadEvent::Paused {
        task_id: id.clone(),
    };
    broadcast_event(&state.ws_state, event);

    Ok(Json(DownloadResponse {
        task_id: id,
        status: "paused".to_string(),
        message: "Download paused successfully".to_string(),
    }))
}

/// 恢复下载
pub async fn resume_download(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> Result<Json<DownloadResponse>, Json<ErrorResponse>> {
    let manager = state.download_manager.lock().await;

    let task = manager
        .get_task(&id)
        .ok_or_else(|| Json(ErrorResponse::not_found("TASK_NOT_FOUND", &format!("Task not found: {}", id))))?;

    task.set_state(TaskState::Downloading);

    log::info!("Download resumed: {}", id);

    // 发送恢复事件到WebSocket客户端
    let event = DownloadEvent::Resumed {
        task_id: id.clone(),
    };
    broadcast_event(&state.ws_state, event);

    Ok(Json(DownloadResponse {
        task_id: id,
        status: "resumed".to_string(),
        message: "Download resumed successfully".to_string(),
    }))
}

/// 取消下载
pub async fn cancel_download(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> Result<Json<DownloadResponse>, Json<ErrorResponse>> {
    let result = {
        let manager = state.download_manager.lock().await;
        manager.remove_task(&id)
    };

    match result {
        Ok(_) => {
            log::info!("Download canceled: {}", id);
            
            // 发送错误事件到WebSocket客户端表示已取消
            let event = DownloadEvent::Error {
                task_id: id.clone(),
                message: "Download canceled".to_string(),
            };
            let _ = broadcast_event(&state.ws_state, event);

            Ok(Json(DownloadResponse {
                task_id: id,
                status: "canceled".to_string(),
                message: "Download canceled successfully".to_string(),
            }))
        }
        Err(e) => {
            log::error!("Failed to cancel download: {}", e);
            Err(Json(ErrorResponse::not_found("TASK_NOT_FOUND", &e.to_string())))
        }
    }
}

/// 列出所有下载
pub async fn list_downloads(
    State(state): State<Arc<ApiState>>,
) -> Result<Json<Vec<DownloadStatus>>, Json<ErrorResponse>> {
    let manager = state.download_manager.lock().await;
    let task_ids = manager.list_tasks();

    let mut tasks = Vec::new();
    for task_id in task_ids {
        if let Some(task) = manager.get_task(&task_id) {
            let filename = task.config.output_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let state_str = format!("{:?}", task.state());
            let progress = if task.file_size > 0 {
                (task.downloaded as f64 / task.file_size as f64) * 100.0
            } else {
                0.0
            };

            tasks.push(DownloadStatus {
                id: task.id.clone(),
                url: task.config.url.clone(),
                filename,
                progress,
                speed: task.speed(),
                status: state_str,
                total_size: task.file_size,
                downloaded: task.downloaded,
            });
        }
    }

    Ok(Json(tasks))
}