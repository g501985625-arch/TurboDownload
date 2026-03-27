# P4: turbo-manager 代码模板

本文档提供核心代码结构和实现模板。

---

## 核心类型定义

### 1. Task 类型 (src/task/mod.rs)

```rust
// src/task/mod.rs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Downloading => write!(f, "downloading"),
            Self::Paused => write!(f, "paused"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "downloading" => Ok(Self::Downloading),
            "paused" => Ok(Self::Paused),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Unknown task status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub output_path: String,
    pub total_size: u64,
    pub downloaded_size: u64,
    pub status: TaskStatus,
    pub progress: f64,
    pub speed: u64,
    pub eta: Option<u64>,
    pub threads: u32,
    pub error: Option<String>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub metadata: Option<String>,
}

impl Task {
    pub fn new(url: String, output_path: String, filename: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url,
            filename,
            output_path,
            total_size: 0,
            downloaded_size: 0,
            status: TaskStatus::Pending,
            progress: 0.0,
            speed: 0,
            eta: None,
            threads: 4,
            error: None,
            created_at: Utc::now().timestamp(),
            started_at: None,
            completed_at: None,
            metadata: None,
        }
    }

    pub fn update_progress(&mut self, downloaded: u64, speed: u64) {
        self.downloaded_size = downloaded;
        self.speed = speed;
        
        if self.total_size > 0 {
            self.progress = (downloaded as f64 / self.total_size as f64) * 100.0;
        }

        if speed > 0 {
            let remaining = self.total_size.saturating_sub(downloaded);
            self.eta = Some(remaining / speed);
        } else {
            self.eta = None;
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskFilter {
    pub status: Option<TaskStatus>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskUpdates {
    pub status: Option<TaskStatus>,
    pub threads: Option<u32>,
    pub error: Option<String>,
    pub total_size: Option<u64>,
    pub downloaded_size: Option<u64>,
    pub speed: Option<u64>,
    pub progress: Option<f64>,
    pub eta: Option<u64>,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
}
```

---

### 2. Settings 类型 (src/config/settings.rs)

```rust
// src/config/settings.rs

use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, File};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub download: DownloadSettings,
    pub database: DatabaseSettings,
    pub logging: LoggingSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub bind_address: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub require_auth: bool,
    pub api_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
    pub max_concurrent_tasks: usize,
    pub default_threads: u32,
    pub max_speed: u64,
    pub download_dir: String,
    pub temp_dir: String,
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,
    pub enable_resume: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub file: Option<String>,
    pub json_format: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                bind_address: "127.0.0.1".to_string(),
                port: 3000,
                cors_origins: vec!["*".to_string()],
                require_auth: false,
                api_token: None,
            },
            download: DownloadSettings {
                max_concurrent_tasks: 3,
                default_threads: 4,
                max_speed: 0,
                download_dir: "./downloads".to_string(),
                temp_dir: "./temp".to_string(),
                retry_attempts: 3,
                retry_delay_ms: 1000,
                enable_resume: true,
            },
            database: DatabaseSettings {
                url: "sqlite:data/turbo.db?mode=rwc".to_string(),
                max_connections: 10,
                min_connections: 1,
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                file: None,
                json_format: false,
            },
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false))
            .build()
            .unwrap_or_default();

        config.try_deserialize().or_else(|_| Ok(Self::default()))
    }
}
```

---

## 数据库实现

### 1. 数据库连接 (src/store/database.rs)

```rust
// src/store/database.rs

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:data/turbo.db?mode=rwc".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self, StoreError> {
        std::fs::create_dir_all("data").ok();
        
        let pool = SqlitePoolOptions::new()
            .max(config.max_connections)
            .min(config.min_connections)
            .acquire_timeout(config.connect_timeout)
            .idle_timeout(Some(config.idle_timeout))
            .connect(&config.url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), StoreError> {
        self.create_tables().await?;
        Ok(())
    }

    async fn create_tables(&self) -> Result<(), StoreError> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                filename TEXT NOT NULL,
                output_path TEXT NOT NULL,
                total_size INTEGER NOT NULL DEFAULT 0,
                downloaded_size INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'pending',
                progress REAL NOT NULL DEFAULT 0.0,
                speed INTEGER NOT NULL DEFAULT 0,
                eta INTEGER,
                threads INTEGER NOT NULL DEFAULT 4,
                error TEXT,
                created_at INTEGER NOT NULL,
                started_at INTEGER,
                completed_at INTEGER,
                metadata TEXT
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query!("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)")
            .execute(&self.pool)
            .await?;

        sqlx::query!("CREATE INDEX IF NOT EXISTS idx_tasks_created ON tasks(created_at)")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
```

---

### 2. 任务存储 (src/store/task_store.rs)

```rust
// src/store/task_store.rs

use sqlx::SqlitePool;
use crate::task::{Task, TaskStatus, TaskFilter, TaskUpdates};
use super::StoreError;

pub struct TaskStore {
    pool: SqlitePool,
}

impl TaskStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, task: &Task) -> Result<(), StoreError> {
        sqlx::query!(
            r#"
            INSERT INTO tasks (id, url, filename, output_path, total_size,
                              downloaded_size, status, progress, speed, eta, threads,
                              error, created_at, started_at, completed_at, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            task.id,
            task.url,
            task.filename,
            task.output_path,
            task.total_size as i64,
            task.downloaded_size as i64,
            task.status.to_string(),
            task.progress,
            task.speed as i64,
            task.eta.map(|e| e as i64),
            task.threads as i32,
            task.error,
            task.created_at,
            task.started_at,
            task.completed_at,
            task.metadata
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get(&self, id: &str) -> Result<Option<Task>, StoreError> {
        let row = sqlx::query_as!(
            TaskRow,
            r#"SELECT * FROM tasks WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    pub async fn list(&self, filter: TaskFilter) -> Result<Vec<Task>, StoreError> {
        let status_filter = filter.status.map(|s| s.to_string());
        let search = filter.search.map(|s| format!("%{}%", s));
        
        let rows = sqlx::query_as!(
            TaskRow,
            r#"
            SELECT * FROM tasks
            WHERE ($1 IS NULL OR status = $1)
            AND ($2 IS NULL OR filename LIKE $2 OR url LIKE $2)
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            status_filter,
            search,
            filter.limit.map(|l| l as i32),
            filter.offset.map(|o| o as i32)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update(&self, id: &str, updates: &TaskUpdates) -> Result<(), StoreError> {
        let status = updates.status.as_ref().map(|s| s.to_string());
        
        sqlx::query!(
            r#"
            UPDATE tasks SET
                status = COALESCE(?, status),
                threads = COALESCE(?, threads),
                error = COALESCE(?, error),
                total_size = COALESCE(?, total_size),
                downloaded_size = COALESCE(?, downloaded_size),
                speed = COALESCE(?, speed),
                progress = COALESCE(?, progress),
                eta = COALESCE(?, eta),
                started_at = COALESCE(?, started_at),
                completed_at = COALESCE(?, completed_at)
            WHERE id = ?
            "#,
            status,
            updates.threads.map(|t| t as i32),
            updates.error,
            updates.total_size.map(|t| t as i64),
            updates.downloaded_size.map(|d| d as i64),
            updates.speed.map(|s| s as i64),
            updates.progress,
            updates.eta.map(|e| e as i64),
            updates.started_at,
            updates.completed_at,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), StoreError> {
        sqlx::query!("DELETE FROM tasks WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn count(&self, status: Option<TaskStatus>) -> Result<i64, StoreError> {
        let status_str = status.map(|s| s.to_string());
        
        let row = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM tasks WHERE $1 IS NULL OR status = $1"#,
            status_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.count)
    }
}

// 数据库行结构
struct TaskRow {
    id: String,
    url: String,
    filename: String,
    output_path: String,
    total_size: i64,
    downloaded_size: i64,
    status: String,
    progress: f64,
    speed: i64,
    eta: Option<i64>,
    threads: i32,
    error: Option<String>,
    created_at: i64,
    started_at: Option<i64>,
    completed_at: Option<i64>,
    metadata: Option<String>,
}

impl From<TaskRow> for Task {
    fn from(row: TaskRow) -> Self {
        Self {
            id: row.id,
            url: row.url,
            filename: row.filename,
            output_path: row.output_path,
            total_size: row.total_size as u64,
            downloaded_size: row.downloaded_size as u64,
            status: row.status.parse().unwrap_or(TaskStatus::Pending),
            progress: row.progress,
            speed: row.speed as u64,
            eta: row.eta.map(|e| e as u64),
            threads: row.threads as u32,
            error: row.error,
            created_at: row.created_at,
            started_at: row.started_at,
            completed_at: row.completed_at,
            metadata: row.metadata,
        }
    }
}
```

---

## API 实现

### 1. 路由定义 (src/api/routes.rs)

```rust
// src/api/routes.rs

use axum::{
    Router,
    routing::{get, post, put, delete},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::sync::Arc;
use crate::api::handlers;
use crate::task::TaskManager;
use crate::config::Settings;
use crate::events::Broadcaster;

pub struct AppState {
    pub task_manager: Arc<TaskManager>,
    pub settings: Arc<Settings>,
    pub broadcaster: Arc<Broadcaster>,
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // 任务管理
        .route("/api/tasks", get(handlers::list_tasks))
        .route("/api/tasks", post(handlers::create_task))
        .route("/api/tasks/:id", get(handlers::get_task))
        .route("/api/tasks/:id", put(handlers::update_task))
        .route("/api/tasks/:id", delete(handlers::delete_task))
        // 任务操作
        .route("/api/tasks/:id/start", post(handlers::start_task))
        .route("/api/tasks/:id/pause", post(handlers::pause_task))
        .route("/api/tasks/:id/cancel", post(handlers::cancel_task))
        .route("/api/tasks/:id/retry", post(handlers::retry_task))
        // 配置管理
        .route("/api/settings", get(handlers::get_settings))
        .route("/api/settings", put(handlers::update_settings))
        // 系统信息
        .route("/api/system/info", get(handlers::system_info))
        // WebSocket
        .route("/ws", get(handlers::ws_handler))
        // 应用状态和中间件
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
```

---

### 2. 请求处理 (src/api/handlers.rs)

```rust
// src/api/handlers.rs

use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::task::{Task, TaskFilter, TaskManager};
use crate::config::Settings;
use crate::events::Broadcaster;
use super::{AppState, ws::handle_socket};

// 请求类型
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub url: String,
    pub output_path: Option<String>,
    pub filename: Option<String>,
    pub threads: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub threads: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    #[serde(flatten)]
    pub settings: serde_json::Value,
}

// 响应类型
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub output_path: String,
    pub total_size: u64,
    pub downloaded_size: u64,
    pub status: String,
    pub progress: f64,
    pub speed: u64,
    pub eta: Option<u64>,
    pub threads: u32,
    pub created_at: i64,
    pub error: Option<String>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            url: task.url,
            filename: task.filename,
            output_path: task.output_path,
            total_size: task.total_size,
            downloaded_size: task.downloaded_size,
            status: task.status.to_string(),
            progress: task.progress,
            speed: task.speed,
            eta: task.eta,
            threads: task.threads,
            created_at: task.created_at,
            error: task.error,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub version: String,
    pub active_downloads: usize,
    pub total_downloads: usize,
    pub download_dir: String,
}

// 错误类型
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ApiResponse::<()>::error(self.message)),
        )
        .into_response()
    }
}

// 处理函数
pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<TaskFilter>,
) -> Result<Json<ApiResponse<Vec<TaskResponse>>>, ApiError> {
    let tasks = state
        .task_manager
        .list_tasks(filter)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        })?;

    Ok(Json(ApiResponse::success(
        tasks.into_iter().map(TaskResponse::from).collect(),
    )))
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
    let output_path = req.output_path.unwrap_or_else(|| {
        state.settings.download.download_dir.clone()
    });

    let task = state
        .task_manager
        .create_task(req.url, output_path, req.filename, req.threads)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        })?;

    Ok(Json(ApiResponse::success(TaskResponse::from(task))))
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
    let task = state
        .task_manager
        .get_task(&id)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        })?
        .ok_or_else(|| ApiError {
            status: StatusCode::NOT_FOUND,
            message: "Task not found".to_string(),
        })?;

    Ok(Json(ApiResponse::success(TaskResponse::from(task))))
}

pub async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
    let task = state
        .task_manager
        .update_task(&id, req.threads)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        })?;

    Ok(Json(ApiResponse::success(TaskResponse::from(task))))
}

pub async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .task_manager
        .delete_task(&id)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        })?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn start_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .task_manager
        .start_task(&id)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        })?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn pause_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .task_manager
        .pause_task(&id)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        })?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn cancel_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .task_manager
        .cancel_task(&id)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        })?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn retry_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .task_manager
        .retry_task(&id)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        })?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn get_settings(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Settings>> {
    Json(ApiResponse::success(state.settings.as_ref().clone()))
}

pub async fn update_settings(
    State(state): State<Arc<AppState>>,
    Json(_req): Json<UpdateSettingsRequest>,
) -> Result<Json<ApiResponse<Settings>>, ApiError> {
    // TODO: 实现设置更新
    Ok(Json(ApiResponse::success(state.settings.as_ref().clone())))
}

pub async fn system_info(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<SystemInfo>>, ApiError> {
    let active = state.task_manager.count_active().await;
    let total = state.task_manager.count_total().await;

    Ok(Json(ApiResponse::success(SystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        active_downloads: active,
        total_downloads: total,
        download_dir: state.settings.download.download_dir.clone(),
    })))
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state.broadcaster.clone()))
}
```

---

### 3. WebSocket 处理 (src/api/ws.rs)

```rust
// src/api/ws.rs

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use crate::events::Broadcaster;

pub async fn handle_socket(socket: WebSocket, broadcaster: Arc<Broadcaster>) {
    let (mut sender, mut receiver) = socket.split();

    // 订阅广播
    let mut rx = broadcaster.subscribe();

    // 发送任务
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // 接收任务
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // 处理客户端消息（如订阅特定任务）
                tracing::debug!("Received WebSocket message: {}", text);
            }
        }
    });

    // 等待任一任务完成
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
```

---

## 任务管理器

### TaskManager (src/task/manager.rs)

```rust
// src/task/manager.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::store::{Database, TaskStore};
use crate::events::Broadcaster;
use super::{Task, TaskStatus, TaskFilter, TaskUpdates};

pub struct TaskManager {
    store: TaskStore,
    broadcaster: Arc<Broadcaster>,
    downloader: Arc<crate::downloader::Downloader>,
}

impl TaskManager {
    pub fn new(db: &Database, broadcaster: Arc<Broadcaster>, downloader: Arc<crate::downloader::Downloader>) -> Self {
        Self {
            store: TaskStore::new(db.pool().clone()),
            broadcaster,
            downloader,
        }
    }

    pub async fn create_task(
        &self,
        url: String,
        output_path: String,
        filename: Option<String>,
        threads: Option<u32>,
    ) -> Result<Task, String> {
        // 获取文件名
        let filename = filename.unwrap_or_else(|| {
            url.split('/')
                .last()
                .unwrap_or("download")
                .to_string()
        });

        let mut task = Task::new(url, output_path, filename);
        if let Some(t) = threads {
            task.threads = t;
        }

        self.store.create(&task).await
            .map_err(|e| e.to_string())?;

        // 广播创建事件
        self.broadcast_event("task_created", &task.id);

        Ok(task)
    }

    pub async fn get_task(&self, id: &str) -> Result<Option<Task>, String> {
        self.store.get(id).await
            .map_err(|e| e.to_string())
    }

    pub async fn list_tasks(&self, filter: TaskFilter) -> Result<Vec<Task>, String> {
        self.store.list(filter).await
            .map_err(|e| e.to_string())
    }

    pub async fn update_task(&self, id: &str, threads: Option<u32>) -> Result<Task, String> {
        let updates = TaskUpdates {
            threads,
            ..Default::default()
        };

        self.store.update(id, &updates).await
            .map_err(|e| e.to_string())?;

        self.get_task(id).await?
            .ok_or_else(|| "Task not found".to_string())
    }

    pub async fn delete_task(&self, id: &str) -> Result<(), String> {
        // 先取消任务
        let _ = self.cancel_task_internal(id).await;

        self.store.delete(id).await
            .map_err(|e| e.to_string())?;

        self.broadcast_event("task_deleted", id);

        Ok(())
    }

    pub async fn start_task(&self, id: &str) -> Result<(), String> {
        let task = self.get_task(id).await?
            .ok_or_else(|| "Task not found".to_string())?;

        if task.status != TaskStatus::Pending && task.status != TaskStatus::Paused {
            return Err("Task cannot be started".to_string());
        }

        // 更新状态
        let updates = TaskUpdates {
            status: Some(TaskStatus::Downloading),
            started_at: Some(chrono::Utc::now().timestamp()),
            ..Default::default()
        };

        self.store.update(id, &updates).await
            .map_err(|e| e.to_string())?;

        // 启动下载
        self.downloader.start_download(task).await?;

        self.broadcast_event("task_started", id);

        Ok(())
    }

    pub async fn pause_task(&self, id: &str) -> Result<(), String> {
        let task = self.get_task(id).await?
            .ok_or_else(|| "Task not found".to_string())?;

        if task.status != TaskStatus::Downloading {
            return Err("Task is not downloading".to_string());
        }

        // 暂停下载
        self.downloader.pause_download(id).await?;

        // 更新状态
        let updates = TaskUpdates {
            status: Some(TaskStatus::Paused),
            ..Default::default()
        };

        self.store.update(id, &updates).await
            .map_err(|e| e.to_string())?;

        self.broadcast_event("task_paused", id);

        Ok(())
    }

    pub async fn cancel_task(&self, id: &str) -> Result<(), String> {
        self.cancel_task_internal(id).await
    }

    async fn cancel_task_internal(&self, id: &str) -> Result<(), String> {
        let task = self.get_task(id).await?;

        if let Some(task) = task {
            if task.status == TaskStatus::Downloading {
                self.downloader.cancel_download(id).await?;
            }
        }

        let updates = TaskUpdates {
            status: Some(TaskStatus::Cancelled),
            ..Default::default()
        };

        self.store.update(id, &updates).await
            .map_err(|e| e.to_string())?;

        self.broadcast_event("task_cancelled", id);

        Ok(())
    }

    pub async fn retry_task(&self, id: &str) -> Result<(), String> {
        let task = self.get_task(id).await?
            .ok_or_else(|| "Task not found".to_string())?;

        if task.status != TaskStatus::Failed {
            return Err("Only failed tasks can be retried".to_string());
        }

        // 重置状态
        let updates = TaskUpdates {
            status: Some(TaskStatus::Pending),
            error: None,
            ..Default::default()
        };

        self.store.update(id, &updates).await
            .map_err(|e| e.to_string())?;

        // 重新启动
        self.start_task(id).await
    }

    pub async fn count_active(&self) -> usize {
        // TODO: 实现计数
        0
    }

    pub async fn count_total(&self) -> usize {
        // TODO: 实现计数
        0
    }

    pub async fn update_progress(&self, id: &str, downloaded: u64, speed: u64) -> Result<(), String> {
        let mut task = self.get_task(id).await?
            .ok_or_else(|| "Task not found".to_string())?;

        task.update_progress(downloaded, speed);

        let updates = TaskUpdates {
            downloaded_size: Some(downloaded),
            speed: Some(speed),
            progress: Some(task.progress),
            eta: task.eta,
            ..Default::default()
        };

        self.store.update(id, &updates).await
            .map_err(|e| e.to_string())?;

        // 广播进度更新
        self.broadcast_progress(id, &task);

        Ok(())
    }

    fn broadcast_event(&self, event_type: &str, task_id: &str) {
        let event = serde_json::json!({
            "type": event_type,
            "taskId": task_id,
        });
        self.broadcaster.broadcast(&event.to_string());
    }

    fn broadcast_progress(&self, task_id: &str, task: &Task) {
        let event = serde_json::json!({
            "type": "progress",
            "taskId": task_id,
            "data": {
                "downloaded": task.downloaded_size,
                "total": task.total_size,
                "speed": task.speed,
                "progress": task.progress,
                "eta": task.eta,
            }
        });
        self.broadcaster.broadcast(&event.to_string());
    }
}
```

---

## 事件广播

### Broadcaster (src/events/bus.rs)

```rust
// src/events/bus.rs

use tokio::sync::broadcast;

pub struct Broadcaster {
    sender: broadcast::Sender<String>,
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Broadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(256);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }

    pub fn broadcast(&self, message: &str) {
        let _ = self.sender.send(message.to_string());
    }
}
```

---

## 主程序入口

### main.rs

```rust
// src/main.rs

use std::net::SocketAddr;
use std::sync::Arc;
use turbo_manager::{
    api::{create_routes, AppState},
    config::Settings,
    events::Broadcaster,
    store::Database,
    task::TaskManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let settings = Settings::load().unwrap_or_default();
    tracing::info!("Loaded settings: {:?}", settings);

    // 初始化数据库
    let db = Database::new(settings.database.clone().into()).await?;
    db.run_migrations().await?;
    tracing::info!("Database initialized");

    // 创建组件
    let broadcaster = Arc::new(Broadcaster::new());
    let downloader = Arc::new(turbo_downloader::Downloader::new(Default::default())?);
    let task_manager = Arc::new(TaskManager::new(&db, broadcaster.clone(), downloader));

    // 创建应用状态
    let state = Arc::new(AppState {
        task_manager,
        settings: Arc::new(settings.clone()),
        broadcaster,
    });

    // 创建路由
    let app = create_routes(state);

    // 启动服务器
    let addr: SocketAddr = format!("{}:{}", settings.server.bind_address, settings.server.port)
        .parse()?;
    
    tracing::info!("Server starting at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

---

## 测试用例

### API 测试 (tests/api_test.rs)

```rust
// tests/api_test.rs

use axum_test::TestServer;
use turbo_manager::{api::create_routes, config::Settings, store::Database, events::Broadcaster, task::TaskManager};

async fn create_test_server() -> TestServer {
    let settings = Settings::default();
    let db = Database::new(settings.database.clone().into()).await.unwrap();
    db.run_migrations().await.unwrap();

    let broadcaster = Arc::new(Broadcaster::new());
    let downloader = Arc::new(turbo_downloader::Downloader::new(Default::default()).unwrap());
    let task_manager = Arc::new(TaskManager::new(&db, broadcaster.clone(), downloader));

    let state = Arc::new(AppState {
        task_manager,
        settings: Arc::new(settings),
        broadcaster,
    });

    TestServer::new(create_routes(state)).unwrap()
}

#[tokio::test]
async fn test_list_tasks_empty() {
    let server = create_test_server().await;

    let response = server.get("/api/tasks").await;

    assert_eq!(response.status_code(), 200);
    
    let body: ApiResponse<Vec<TaskResponse>> = response.json();
    assert!(body.success);
    assert!(body.data.unwrap().is_empty());
}

#[tokio::test]
async fn test_create_task() {
    let server = create_test_server().await;

    let response = server
        .post("/api/tasks")
        .json(&CreateTaskRequest {
            url: "https://example.com/file.zip".to_string(),
            output_path: None,
            filename: None,
            threads: None,
        })
        .await;

    assert_eq!(response.status_code(), 200);
    
    let body: ApiResponse<TaskResponse> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());
    
    let task = body.data.unwrap();
    assert_eq!(task.url, "https://example.com/file.zip");
    assert_eq!(task.status, "pending");
}

#[tokio::test]
async fn test_get_task_not_found() {
    let server = create_test_server().await;

    let response = server.get("/api/tasks/nonexistent").await;

    assert_eq!(response.status_code(), 404);
}
```

---

## 示例代码

### 基础服务器 (examples/basic_server.rs)

```rust
// examples/basic_server.rs

use std::net::SocketAddr;
use std::sync::Arc;
use turbo_manager::{
    api::{create_routes, AppState},
    config::Settings,
    events::Broadcaster,
    store::Database,
    task::TaskManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let settings = Settings::load().unwrap_or_default();

    // 初始化数据库
    let db = Database::new(settings.database.clone().into()).await?;
    db.run_migrations().await?;

    // 创建组件
    let broadcaster = Arc::new(Broadcaster::new());
    let downloader = Arc::new(turbo_downloader::Downloader::new(Default::default())?);
    let task_manager = Arc::new(TaskManager::new(&db, broadcaster.clone(), downloader));

    // 创建应用状态
    let state = Arc::new(AppState {
        task_manager,
        settings: Arc::new(settings.clone()),
        broadcaster,
    });

    // 创建路由并启动
    let app = create_routes(state);
    let addr: SocketAddr = "127.0.0.1:3000".parse()?;

    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```