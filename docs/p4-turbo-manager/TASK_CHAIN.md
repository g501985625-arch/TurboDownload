# P4: turbo-manager 详细任务链

## 任务概览

| 任务编号 | 任务名称 | 预估时间 | 依赖任务 |
|----------|----------|----------|----------|
| T4.1 | 项目初始化 | 3h | 无 |
| T4.2 | 数据存储层 | 6h | T4.1 |
| T4.3 | REST API 实现 | 8h | T4.2 |
| T4.4 | WebSocket 实现 | 4h | T4.3 |
| T4.5 | 任务调度器 | 8h | T4.2 |
| T4.6 | 配置管理 | 4h | T4.1 |
| T4.7 | 日志系统 | 3h | T4.1 |
| T4.8 | IPC 桥接 | 5h | T4.3 |
| T4.9 | 测试与优化 | 10h | T4.1-T4.8 |
| T4.10 | 文档与示例 | 4h | T4.9 |

**总工时**: 55h (约 7 个工作日)

---

## T4.1: 项目初始化

### T4.1.1: 创建 Rust crate 结构

**时间**: 1h  
**依赖**: 无

#### 步骤

1. **创建项目目录**
   ```bash
   cd ~/Projects/TurboDownload
   mkdir -p crates/turbo-manager
   cd crates/turbo-manager
   cargo init --lib
   ```

2. **创建模块文件**
   ```bash
   mkdir -p src/{api,scheduler,store,config,logging,ipc,task,events}
   touch src/api/{mod.rs,routes.rs,handlers.rs,middleware.rs,ws.rs}
   touch src/scheduler/{mod.rs,queue.rs,worker.rs,priority.rs}
   touch src/store/{mod.rs,database.rs,task_store.rs,config_store.rs}
   touch src/config/{mod.rs,settings.rs,loader.rs}
   touch src/logging/{mod.rs,setup.rs,query.rs}
   touch src/ipc/{mod.rs,commands.rs,events.rs}
   touch src/task/{mod.rs,manager.rs,monitor.rs}
   touch src/events/{mod.rs,bus.rs}
   ```

3. **配置 lib.rs 入口**
   ```rust
   //! Turbo Manager - Backend management service
   //!
   //! # Features
   //! - REST API
   //! - WebSocket
   //! - Task scheduling
   //! - Configuration management

   pub mod api;
   pub mod scheduler;
   pub mod store;
   pub mod config;
   pub mod logging;
   pub mod ipc;
   pub mod task;
   pub mod events;

   pub use api::Manager;
   pub use config::Settings;
   pub use task::TaskManager;
   ```

#### 验收标准

- [ ] `cargo check` 通过
- [ ] 目录结构符合规范
- [ ] 模块导入无错误

---

### T4.1.2: 配置 Cargo.toml 依赖

**时间**: 1h  
**依赖**: T4.1.1

#### 步骤

1. **编辑 Cargo.toml**
   ```toml
   [package]
   name = "turbo-manager"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   tokio = { workspace = true }
   axum = { version = "0.7", features = ["ws", "macros"] }
   tower = { version = "0.4", features = ["util", "timeout"] }
   tower-http = { version = "0.5", features = ["fs", "cors", "trace"] }
   serde = { workspace = true }
   serde_json = { workspace = true }
   sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
   thiserror = { workspace = true }
   tracing = { workspace = true }
   chrono = { version = "0.4", features = ["serde"] }
   uuid = { version = "1.0", features = ["v4", "serde"] }
   config = "0.14"
   parking_lot = "0.12"
   
   turbo-downloader = { path = "../turbo-downloader" }
   turbo-crawler = { path = "../turbo-crawler" }
   ```

2. **验证依赖**
   ```bash
   cargo fetch
   cargo check
   ```

#### 验收标准

- [ ] 所有依赖下载成功
- [ ] 版本冲突已解决
- [ ] `cargo build` 成功

---

### T4.1.3: 创建测试目录结构

**时间**: 0.5h  
**依赖**: T4.1.1

#### 步骤

1. **创建测试目录**
   ```bash
   mkdir -p tests
   touch tests/{mod.rs,api_test.rs,scheduler_test.rs,store_test.rs}
   ```

2. **创建测试框架**
   ```rust
   // tests/mod.rs
   pub mod api_test;
   pub mod scheduler_test;
   pub mod store_test;

   pub fn setup_test_env() {
       let _ = tracing_subscriber::fmt::try_init();
   }
   ```

#### 验收标准

- [ ] 测试目录结构完整
- [ ] 测试框架可运行

---

### T4.1.4: 配置开发工具

**时间**: 0.5h  
**依赖**: T4.1.1

#### 步骤

1. **创建 rustfmt.toml**
   ```toml
   edition = "2021"
   max_width = 100
   use_small_heuristics = "Default"
   imports_granularity = "Crate"
   group_imports = "StdExternalCrate"
   ```

2. **创建 .cargo/config.toml**
   ```toml
   [build]
   rustflags = ["-W", "clippy::all"]

   [alias]
   nextest = "nextest run"
   lint = "clippy -- -D warnings"
   ```

#### 验收标准

- [ ] `cargo fmt` 格式化正确
- [ ] `cargo clippy` 无警告

---

## T4.2: 数据存储层

### T4.2.1: 设计数据库模式

**时间**: 2h  
**依赖**: T4.1

#### 步骤

1. **定义任务表**
   ```sql
   -- schema/tasks.sql
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
       threads INTEGER NOT NULL DEFAULT 4,
       error TEXT,
       created_at INTEGER NOT NULL,
       started_at INTEGER,
       completed_at INTEGER,
       metadata TEXT
   );

   CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
   CREATE INDEX IF NOT EXISTS idx_tasks_created ON tasks(created_at);
   ```

2. **定义配置表**
   ```sql
   -- schema/settings.sql
   CREATE TABLE IF NOT EXISTS settings (
       key TEXT PRIMARY KEY,
       value TEXT NOT NULL,
       updated_at INTEGER NOT NULL
   );
   ```

3. **定义日志表**
   ```sql
   -- schema/logs.sql
   CREATE TABLE IF NOT EXISTS logs (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       level TEXT NOT NULL,
       message TEXT NOT NULL,
       task_id TEXT,
       timestamp INTEGER NOT NULL,
       metadata TEXT
   );

   CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON logs(timestamp);
   CREATE INDEX IF NOT EXISTS idx_logs_task ON logs(task_id);
   ```

#### 验收标准

- [ ] 表结构设计合理
- [ ] 索引正确创建
- [ ] 迁移脚本可用

---

### T4.2.2: 实现数据库连接池

**时间**: 2h  
**依赖**: T4.2.1

#### 步骤

1. **定义数据库配置**
   ```rust
   // src/store/database.rs
   use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
   use std::time::Duration;

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
   ```

2. **实现连接池**
   ```rust
   pub struct Database {
       pool: SqlitePool,
   }

   impl Database {
       pub async fn new(config: DatabaseConfig) -> Result<Self, StoreError> {
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
           sqlx::migrate!("./schema").run(&self.pool).await?;
           Ok(())
       }

       pub fn pool(&self) -> &SqlitePool {
           &self.pool
       }
   }
   ```

#### 验收标准

- [ ] 连接池创建成功
- [ ] 迁移运行正常
- [ ] 连接复用正常

---

### T4.2.3: 实现任务存储

**时间**: 2h  
**依赖**: T4.2.2

#### 步骤

1. **定义任务存储接口**
   ```rust
   // src/store/task_store.rs
   use sqlx::SqlitePool;
   use crate::task::{Task, TaskStatus};

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
                                  downloaded_size, status, progress, speed, threads,
                                  created_at, metadata)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               "#,
               task.id, task.url, task.filename, task.output_path,
               task.total_size, task.downloaded_size, task.status.to_string(),
               task.progress, task.speed, task.threads, task.created_at, task.metadata
           )
           .execute(&self.pool)
           .await?;

           Ok(())
       }

       pub async fn get(&self, id: &str) -> Result<Option<Task>, StoreError> {
           let row = sqlx::query_as!(
               Task,
               r#"SELECT * FROM tasks WHERE id = ?"#,
               id
           )
           .fetch_optional(&self.pool)
           .await?;

           Ok(row)
       }

       pub async fn list(&self, filter: TaskFilter) -> Result<Vec<Task>, StoreError> {
           // 实现列表查询
       }

       pub async fn update(&self, id: &str, updates: &TaskUpdates) -> Result<(), StoreError> {
           // 实现更新
       }

       pub async fn delete(&self, id: &str) -> Result<(), StoreError> {
           // 实现删除
       }
   }
   ```

#### 验收标准

- [ ] CRUD 操作正常
- [ ] 查询性能良好
- [ ] 事务处理正确

---

## T4.3: REST API 实现

### T4.3.1: 定义 API 路由

**时间**: 2h  
**依赖**: T4.2

#### 步骤

1. **定义路由结构**
   ```rust
   // src/api/routes.rs
   use axum::{
       Router,
       routing::{get, post, put, delete},
   };
   use crate::api::handlers;

   pub fn create_routes() -> Router {
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
           .route("/api/system/logs", get(handlers::query_logs))
           // WebSocket
           .route("/ws", get(handlers::ws_handler))
   }
   ```

2. **应用中间件**
   ```rust
   use tower_http::{cors::CorsLayer, trace::TraceLayer};

   pub fn create_app() -> Router {
       create_routes()
           .layer(CorsLayer::permissive())
           .layer(TraceLayer::new_for_http())
   }
   ```

#### 验收标准

- [ ] 路由定义完整
- [ ] 中间件正确应用
- [ ] OpenAPI 文档可用

---

### T4.3.2: 实现任务处理函数

**时间**: 3h  
**依赖**: T4.3.1

#### 步骤

1. **定义请求/响应类型**
   ```rust
   // src/api/handlers.rs
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Deserialize)]
   pub struct CreateTaskRequest {
       pub url: String,
       pub output_path: Option<String>,
       pub filename: Option<String>,
       pub threads: Option<u32>,
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
       pub threads: u32,
       pub created_at: i64,
       pub error: Option<String>,
   }

   #[derive(Debug, Serialize)]
   pub struct ApiResponse<T> {
       pub success: bool,
       pub data: Option<T>,
       pub error: Option<String>,
   }
   ```

2. **实现处理函数**
   ```rust
   pub async fn create_task(
       State(manager): State<Arc<TaskManager>>,
       Json(req): Json<CreateTaskRequest>,
   ) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
       let task = manager.create_task(req).await?;
       
       Ok(Json(ApiResponse {
           success: true,
           data: Some(task.into()),
           error: None,
       }))
   }

   pub async fn list_tasks(
       State(manager): State<Arc<TaskManager>>,
       Query(filter): Query<TaskFilter>,
   ) -> Result<Json<ApiResponse<Vec<TaskResponse>>>, ApiError> {
       let tasks = manager.list_tasks(filter).await?;
       
       Ok(Json(ApiResponse {
           success: true,
           data: Some(tasks.into_iter().map(Into::into).collect()),
           error: None,
       }))
   }

   pub async fn get_task(
       State(manager): State<Arc<TaskManager>>,
       Path(id): Path<String>,
   ) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
       let task = manager.get_task(&id).await?
           .ok_or(ApiError::NotFound)?;
       
       Ok(Json(ApiResponse {
           success: true,
           data: Some(task.into()),
           error: None,
       }))
   }
   ```

#### 验收标准

- [ ] 所有端点正确实现
- [ ] 错误处理完整
- [ ] 响应格式一致

---

### T4.3.3: 实现中间件

**时间**: 3h  
**依赖**: T4.3.2

#### 步骤

1. **实现认证中间件**
   ```rust
   // src/api/middleware.rs
   use axum::{
       extract::{Request, State},
       middleware::Next,
       response::Response,
   };

   pub async fn auth_middleware(
       State(config): State<Arc<Config>>,
       request: Request,
       next: Next,
   ) -> Result<Response, ApiError> {
       // 如果启用认证
       if config.require_auth {
           let token = request
               .headers()
               .get("Authorization")
               .and_then(|h| h.to_str().ok())
               .and_then(|h| h.strip_prefix("Bearer "))
               .ok_or(ApiError::Unauthorized)?;

           if token != config.api_token {
               return Err(ApiError::Unauthorized);
           }
       }

       Ok(next.run(request).await)
   }
   ```

2. **实现请求限流**
   ```rust
   use std::sync::Arc;
   use tokio::sync::Semaphore;

   pub async fn rate_limit_middleware(
       State(limiter): State<Arc<RateLimiter>>,
       request: Request,
       next: Next,
   ) -> Result<Response, ApiError> {
       limiter.acquire().await?;
       Ok(next.run(request).await)
   }

   pub struct RateLimiter {
       semaphore: Arc<Semaphore>,
   }

   impl RateLimiter {
       pub fn new(max_requests: usize) -> Self {
           Self {
               semaphore: Arc::new(Semaphore::new(max_requests)),
           }
       }

       pub async fn acquire(&self) -> Result<(), ApiError> {
           self.semaphore.acquire().await?;
           Ok(())
       }
   }
   ```

#### 验收标准

- [ ] 认证中间件工作正常
- [ ] 限流中间件工作正常
- [ ] 日志中间件工作正常

---

## T4.4: WebSocket 实现

### T4.4.1: 实现 WebSocket 处理

**时间**: 2h  
**依赖**: T4.3

#### 步骤

1. **定义 WebSocket 升级**
   ```rust
   // src/api/ws.rs
   use axum::{
       extract::{ws::WebSocketUpgrade, WebSocket, State, ConnectInfo},
       response::Response,
   };

   pub async fn ws_handler(
       ws: WebSocketUpgrade,
       State(broadcaster): State<Arc<Broadcaster>>,
   ) -> Response {
       ws.on_upgrade(|socket| handle_socket(socket, broadcaster))
   }

   async fn handle_socket(socket: WebSocket, broadcaster: Arc<Broadcaster>) {
       let (mut sender, mut receiver) = socket.split();

       // 订阅广播
       let mut rx = broadcaster.subscribe();

       // 发送任务
       let send_task = async move {
           while let Ok(msg) = rx.recv().await {
               if sender.send(Message::Text(msg)).await.is_err() {
                   break;
               }
           }
       };

       // 接收任务
       let recv_task = async move {
           while let Some(Ok(msg)) = receiver.next().await {
               // 处理客户端消息
               if let Message::Text(text) = msg {
                   // 解析命令
               }
           }
       };

       tokio::select! {
           _ = send_task => {}
           _ = recv_task => {}
       }
   }
   ```

2. **实现广播器**
   ```rust
   use tokio::sync::broadcast;

   pub struct Broadcaster {
       sender: broadcast::Sender<String>,
   }

   impl Broadcaster {
       pub fn new() -> Self {
           let (sender, _) = broadcast::channel(100);
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

#### 验收标准

- [ ] WebSocket 连接正常
- [ ] 广播消息正确
- [ ] 断开连接处理正确

---

### T4.4.2: 实现实时进度推送

**时间**: 2h  
**依赖**: T4.4.1

#### 步骤

```rust
   // 定义进度事件
   #[derive(Debug, Serialize)]
   pub struct ProgressEvent {
       pub event_type: String,
       pub task_id: String,
       pub data: ProgressData,
   }

   #[derive(Debug, Serialize)]
   pub struct ProgressData {
       pub downloaded: u64,
       pub total: u64,
       pub speed: u64,
       pub progress: f64,
       pub eta: Option<u64>,
   }

   impl TaskManager {
       pub async fn broadcast_progress(&self, task_id: &str, progress: ProgressData) {
           let event = ProgressEvent {
               event_type: "progress".to_string(),
               task_id: task_id.to_string(),
               data: progress,
           };

           let json = serde_json::to_string(&event).unwrap();
           self.broadcaster.broadcast(&json);
       }
   }
```

#### 验收标准

- [ ] 进度事件正确推送
- [ ] 事件格式正确
- [ ] 多客户端同步正常

---

## T4.5: 任务调度器

### T4.5.1: 实现任务队列

**时间**: 3h  
**依赖**: T4.2

#### 步骤

1. **定义任务队列**
   ```rust
   // src/scheduler/queue.rs
   use std::collections::BinaryHeap;
   use parking_lot::RwLock;

   pub struct TaskQueue {
       pending: RwLock<BinaryHeap<PrioritizedTask>>,
       active: RwLock<HashMap<String, Task>>,
       max_concurrent: usize,
   }

   #[derive(Eq, PartialEq)]
   struct PrioritizedTask {
       priority: u32,
       task: Task,
   }

   impl Ord for PrioritizedTask {
       fn cmp(&self, other: &Self) -> std::cmp::Ordering {
           self.priority.cmp(&other.priority)
       }
   }

   impl PartialOrd for PrioritizedTask {
       fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
           Some(self.cmp(other))
       }
   }

   impl TaskQueue {
       pub fn new(max_concurrent: usize) -> Self {
           Self {
               pending: RwLock::new(BinaryHeap::new()),
               active: RwLock::new(HashMap::new()),
               max_concurrent,
           }
       }

       pub fn enqueue(&self, task: Task, priority: u32) {
           self.pending.write().push(PrioritizedTask { priority, task });
       }

       pub fn dequeue(&self) -> Option<Task> {
           self.pending.write().pop().map(|pt| pt.task)
       }

       pub fn can_start(&self) -> bool {
           self.active.read().len() < self.max_concurrent
       }
   }
   ```

#### 验收标准

- [ ] 队列操作正确
- [ ] 优先级排序正确
- [ ] 并发限制有效

---

### T4.5.2: 实现工作线程池

**时间**: 3h  
**依赖**: T4.5.1

#### 步骤

```rust
   // src/scheduler/worker.rs
   use tokio::task::JoinHandle;

   pub struct WorkerPool {
       workers: Vec<JoinHandle<()>>,
       task_queue: Arc<TaskQueue>,
       downloader: Arc<Downloader>,
   }

   impl WorkerPool {
       pub fn new(
           pool_size: usize,
           task_queue: Arc<TaskQueue>,
           downloader: Arc<Downloader>,
       ) -> Self {
           let workers = (0..pool_size)
               .map(|_| {
                   let queue = task_queue.clone();
                   let dl = downloader.clone();
                   
                   tokio::spawn(async move {
                       loop {
                           if queue.can_start() {
                               if let Some(task) = queue.dequeue() {
                                   queue.mark_active(&task.id);
                                   if let Err(e) = dl.download(task).await {
                                       tracing::error!("Download failed: {}", e);
                                   }
                                   queue.mark_completed(&task.id);
                               }
                           }
                           tokio::time::sleep(Duration::from_millis(100)).await;
                       }
                   })
               })
               .collect();

           Self { workers, task_queue, downloader }
       }
   }
```

#### 验收标准

- [ ] 线程池正常工作
- [ ] 任务执行正确
- [ ] 错误处理完整

---

### T4.5.3: 实现优先级调度

**时间**: 2h  
**依赖**: T4.5.2

#### 步骤

```rust
   // src/scheduler/priority.rs
   pub enum Priority {
       Low = 0,
       Normal = 1,
       High = 2,
       Urgent = 3,
   }

   impl Task {
       pub fn calculate_priority(&self) -> u32 {
           match self.status {
               TaskStatus::Pending => {
                   let age = chrono::Utc::now().timestamp() - self.created_at;
                   // 等待越久优先级越高
                   (age / 3600) as u32 + 1
               }
               _ => Priority::Normal as u32,
           }
       }
   }
```

#### 验收标准

- [ ] 优先级计算正确
- [ ] 调度顺序合理

---

## T4.6: 配置管理

### T4.6.1: 实现配置加载

**时间**: 2h  
**依赖**: T4.1

#### 步骤

```rust
   // src/config/settings.rs
   use serde::Deserialize;
   use config::{Config, ConfigError, File};

   #[derive(Debug, Deserialize, Clone)]
   pub struct Settings {
       pub server: ServerSettings,
       pub download: DownloadSettings,
       pub database: DatabaseSettings,
       pub logging: LoggingSettings,
   }

   #[derive(Debug, Deserialize, Clone)]
   pub struct ServerSettings {
       pub bind_address: String,
       pub port: u16,
       pub cors_origins: Vec<String>,
   }

   impl Settings {
       pub fn load() -> Result<Self, ConfigError> {
           let config = Config::builder()
               .add_source(File::with_name("config/default"))
               .add_source(File::with_name("config/local").required(false))
               .build()?;

           config.try_deserialize()
       }
   }
```

#### 验收标准

- [ ] 配置加载正常
- [ ] 默认值合理
- [ ] 配置验证正确

---

### T4.6.2: 实现配置持久化

**时间**: 2h  
**依赖**: T4.6.1

#### 步骤

```rust
   // src/store/config_store.rs
   pub struct ConfigStore {
       pool: SqlitePool,
   }

   impl ConfigStore {
       pub async fn get(&self, key: &str) -> Result<Option<String>, StoreError> {
           let row = sqlx::query!(
               "SELECT value FROM settings WHERE key = ?",
               key
           )
           .fetch_optional(&self.pool)
           .await?;

           Ok(row.map(|r| r.value))
       }

       pub async fn set(&self, key: &str, value: &str) -> Result<(), StoreError> {
           sqlx::query!(
               r#"
               INSERT INTO settings (key, value, updated_at)
               VALUES (?, ?, ?)
               ON CONFLICT(key) DO UPDATE SET value = ?, updated_at = ?
               "#,
               key, value, chrono::Utc::now().timestamp(),
               value, chrono::Utc::now().timestamp()
           )
           .execute(&self.pool)
           .await?;

           Ok(())
       }
   }
```

#### 验收标准

- [ ] 配置读写正常
- [ ] 原子更新正确

---

## T4.7: 日志系统

### T4.7.1: 配置日志输出

**时间**: 2h  
**依赖**: T4.1

#### 步骤

```rust
   // src/logging/setup.rs
   use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

   pub fn setup_logging(level: &str) {
       tracing_subscriber::registry()
           .with(
               tracing_subscriber::EnvFilter::try_from_default_env()
                   .unwrap_or_else(|_| level.into()),
           )
           .with(tracing_subscriber::fmt::layer())
           .with(JsonLayer::new(std::io::stdout()))
           .init();
   }
```

#### 验收标准

- [ ] 日志输出正常
- [ ] 格式正确

---

### T4.7.2: 实现日志查询

**时间**: 1h  
**依赖**: T4.7.1

#### 步骤

```rust
   // src/logging/query.rs
   pub struct LogQuery {
       pool: SqlitePool,
   }

   impl LogQuery {
       pub async fn query(
           &self,
           filter: LogFilter,
       ) -> Result<Vec<LogEntry>, StoreError> {
           sqlx::query_as!(
               LogEntry,
               r#"
               SELECT * FROM logs
               WHERE ($1 IS NULL OR level = $1)
               AND ($2 IS NULL OR task_id = $2)
               AND timestamp >= $3
               ORDER BY timestamp DESC
               LIMIT $4
               "#,
               filter.level,
               filter.task_id,
               filter.since,
               filter.limit,
           )
           .fetch_all(&self.pool)
           .await
           .map_err(Into::into)
       }
   }
```

#### 验收标准

- [ ] 查询正常
- [ ] 过滤正确

---

## T4.8: IPC 桥接

### T4.8.1: 定义 IPC 命令

**时间**: 2h  
**依赖**: T4.3

#### 步骤

```rust
   // src/ipc/commands.rs
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Serialize, Deserialize)]
   #[serde(tag = "type")]
   pub enum IpcCommand {
       CreateTask { url: String, output_path: String },
       StartTask { id: String },
       PauseTask { id: String },
       CancelTask { id: String },
       GetTask { id: String },
       ListTasks { filter: TaskFilter },
       GetSettings,
       UpdateSettings { settings: SettingsUpdate },
   }

   #[derive(Debug, Serialize, Deserialize)]
   pub struct IpcResponse<T> {
       pub success: bool,
       pub data: Option<T>,
       pub error: Option<String>,
   }
```

#### 验收标准

- [ ] 命令定义完整
- [ ] 序列化正确

---

### T4.8.2: 实现 IPC 处理器

**时间**: 3h  
**依赖**: T4.8.1

#### 步骤

```rust
   // src/ipc/handler.rs
   pub struct IpcHandler {
       task_manager: Arc<TaskManager>,
       settings: Arc<Settings>,
   }

   impl IpcHandler {
       pub async fn handle(&self, command: IpcCommand) -> IpcResponse<Value> {
           match command {
               IpcCommand::CreateTask { url, output_path } => {
                   match self.task_manager.create_task(url, output_path).await {
                       Ok(task) => IpcResponse::success(task),
                       Err(e) => IpcResponse::error(e.to_string()),
                   }
               }
               // ... 其他命令
           }
       }
   }
```

#### 验收标准

- [ ] 命令处理正确
- [ ] 错误处理完整

---

## T4.9: 测试与优化

### T4.9.1: 单元测试

**时间**: 4h  
**依赖**: T4.1-T4.8

#### 测试清单

```rust
// tests/api_test.rs
#[tokio::test]
async fn test_create_task() { /* ... */ }
#[tokio::test]
async fn test_list_tasks() { /* ... */ }

// tests/scheduler_test.rs
#[tokio::test]
async fn test_queue_operations() { /* ... */ }
#[tokio::test]
async fn test_priority_scheduling() { /* ... */ }

// tests/store_test.rs
#[tokio::test]
async fn test_task_crud() { /* ... */ }
#[tokio::test]
async fn test_config_store() { /* ... */ }
```

#### 验收标准

- [ ] 单元测试覆盖率 > 80%
- [ ] 所有测试通过

---

### T4.9.2: 集成测试

**时间**: 3h  
**依赖**: T4.9.1

#### 步骤

1. **创建集成测试**
   ```rust
   // tests/integration_test.rs
   #[tokio::test]
   async fn test_full_task_lifecycle() {
       let app = create_test_app().await;
       
       // 创建任务
       let response = app.post("/api/tasks")
           .json(&CreateTaskRequest { url: "https://example.com/file.zip", .. })
           .send()
           .await;
       
       assert_eq!(response.status(), 200);
   }
   ```

#### 验收标准

- [ ] 集成测试通过
- [ ] 端到端流程正常

---

### T4.9.3: 性能测试

**时间**: 3h  
**依赖**: T4.9.2

#### 步骤

1. **创建基准测试**
   ```rust
   // benches/api_benchmark.rs
   use criterion::{criterion_group, criterion_main, Criterion};

   fn benchmark_create_task(c: &mut Criterion) {
       c.bench_function("create_task", |b| {
           b.to_async(tokio::runtime::Runtime::new().unwrap())
               .iter(|| async {
                   // 测试创建任务
               })
       });
   }

   criterion_group!(benches, benchmark_create_task);
   criterion_main!(benches);
   ```

#### 验收标准

- [ ] 基准测试结果记录
- [ ] 性能瓶颈分析

---

## T4.10: 文档与示例

### T4.10.1: 编写 API 文档

**时间**: 2h  
**依赖**: T4.9

#### 步骤

```bash
cargo doc --no-deps --open
```

### T4.10.2: 编写示例代码

**时间**: 2h  
**依赖**: T4.10.1

创建示例：
- `examples/basic_server.rs`
- `examples/with_frontend.rs`

#### 验收标准

- [ ] 所有 API 有文档
- [ ] 示例可运行
- [ ] README 完整

---

## 任务依赖图

```
T4.1 项目初始化
  ├── T4.2 数据存储层
  │     ├── T4.3 REST API
  │     │     ├── T4.4 WebSocket
  │     │     └── T4.8 IPC 桥接
  │     └── T4.5 任务调度器
  ├── T4.6 配置管理
  └── T4.7 日志系统
        └── T4.9 测试优化
              └── T4.10 文档示例
```

---

## 里程碑

| 里程碑 | 完成任务 | 预计时间 |
|--------|----------|----------|
| M1 | T4.1, T4.2 | Day 1-2 |
| M2 | T4.3, T4.4 | Day 2-3 |
| M3 | T4.5, T4.6 | Day 3-4 |
| M4 | T4.7, T4.8 | Day 4-5 |
| M5 | T4.9, T4.10 | Day 5-7 |