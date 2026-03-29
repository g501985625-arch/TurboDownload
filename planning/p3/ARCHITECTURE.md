# P3: 多线程下载 + 断点续传 技术架构设计

## 1. 设计目标

基于 P1 (turbo-downloader) 实现高性能多线程分片下载与断点续传功能。

### 功能需求

| 需求 | 说明 |
|------|------|
| Range 请求 | HTTP Range 头实现分片下载 |
| 多线程下载 | 可配置线程数并行下载 |
| 分片合并 | 下载完成后合并为完整文件 |
| 断点续传 | 持久化状态，支持中断恢复 |
| 进度事件 | Tauri 事件推送实时进度 |

### 非功能性需求

| 指标 | 目标值 |
|------|--------|
| 最大线程数 | 32 |
| 分片大小 | 可配置，默认 1MB |
| 状态持久化 | JSON 文件 |
| 进度更新频率 | 100ms |
| 重试次数 | 可配置，默认 3 次 |

---

## 2. 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                     MultiThreadDownloader                    │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  下载配置    │  │  状态管理    │  │  事件发射器  │       │
│  │ DownloadConfig│  │ StateManager │  │EventEmitter │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
├─────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────┐  │
│  │                  下载引擎 (DownloadEngine)              │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │  │
│  │  │ RangeClient │  │ ChunkManager│  │WorkerPool   │    │  │
│  │  │ (HTTP Range)│  │ (分片管理)  │  │ (线程池)    │    │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘    │  │
│  └───────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────┐  │
│  │                   存储层 (Storage)                      │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │  │
│  │  │ ChunkWriter │  │ FileMerger  │  │StatePersist │    │  │
│  │  │ (分片写入)  │  │ (文件合并)  │  │(状态持久化) │    │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘    │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 核心模块设计

### 3.1 RangeClient (Range 请求客户端)

**职责**: 发送 HTTP Range 请求，获取指定范围的数据

```rust
pub struct RangeClient {
    client: reqwest::Client,
    config: RangeClientConfig,
}

pub struct RangeClientConfig {
    pub timeout: Duration,
    pub retry_count: u32,
    pub user_agent: String,
}

impl RangeClient {
    /// 检测服务器是否支持 Range 请求
    pub async fn check_range_support(&self, url: &str) -> Result<RangeSupport>;
    
    /// 获取文件总大小
    pub async fn get_content_length(&self, url: &str) -> Result<u64>;
    
    /// 下载指定范围的数据
    pub async fn fetch_range(&self, url: &str, start: u64, end: u64) -> Result<Bytes>;
}
```

### 3.2 ChunkManager (分片管理器)

**职责**: 计算分片策略，管理分片状态

```rust
pub struct ChunkManager {
    chunks: Vec<Chunk>,
    total_size: u64,
    chunk_size: u64,
}

pub struct Chunk {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub state: ChunkState,
    pub temp_path: PathBuf,
}

pub enum ChunkState {
    Pending,
    Downloading,
    Completed,
    Failed,
}

impl ChunkManager {
    /// 创建分片管理器
    pub fn new(total_size: u64, chunk_size: u64) -> Self;
    
    /// 计算分片
    pub fn calculate_chunks(&mut self) -> Vec<Chunk>;
    
    /// 获取下一个待下载的分片
    pub fn get_next_pending(&self) -> Option<&Chunk>;
    
    /// 更新分片状态
    pub fn update_chunk(&mut self, id: u32, downloaded: u64, state: ChunkState);
}
```

### 3.3 WorkerPool (工作线程池)

**职责**: 管理并发下载任务

```rust
pub struct WorkerPool {
    max_workers: usize,
    workers: JoinSet<DownloadTask>,
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    /// 创建线程池
    pub fn new(max_workers: usize) -> Self;
    
    /// 提交下载任务
    pub fn submit(&mut self, task: DownloadTask);
    
    /// 等待所有任务完成
    pub async fn wait_all(&mut self) -> Result<Vec<DownloadResult>>;
}
```

### 3.4 ChunkWriter (分片写入器)

**职责**: 将下载的数据写入临时文件

```rust
pub struct ChunkWriter {
    temp_dir: PathBuf,
}

impl ChunkWriter {
    /// 写入分片数据
    pub async fn write_chunk(&self, chunk: &Chunk, data: &[u8]) -> Result<()>;
    
    /// 读取已下载的分片数据
    pub async fn read_chunk(&self, chunk: &Chunk) -> Result<Option<Vec<u8>>>;
}
```

### 3.5 FileMerger (文件合并器)

**职责**: 合并所有分片为完整文件

```rust
pub struct FileMerger;

impl FileMerger {
    /// 合并分片文件
    pub async fn merge(
        chunks: &[Chunk],
        output_path: &Path,
        on_progress: impl Fn(u64, u64),
    ) -> Result<()>;
    
    /// 验证文件完整性
    pub async fn verify(&self, file_path: &Path, expected_size: u64) -> Result<bool>;
}
```

### 3.6 StateManager (状态管理器)

**职责**: 持久化和恢复下载状态

```rust
pub struct StateManager {
    state_dir: PathBuf,
}

pub struct DownloadState {
    pub task_id: String,
    pub url: String,
    pub total_size: u64,
    pub downloaded: u64,
    pub chunks: Vec<ChunkState>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl StateManager {
    /// 保存状态
    pub async fn save(&self, state: &DownloadState) -> Result<()>;
    
    /// 加载状态
    pub async fn load(&self, task_id: &str) -> Result<Option<DownloadState>>;
    
    /// 删除状态
    pub async fn remove(&self, task_id: &str) -> Result<()>;
}
```

### 3.7 EventEmitter (事件发射器)

**职责**: 通过 Tauri 事件系统推送进度

```rust
pub struct EventEmitter {
    app_handle: AppHandle,
}

pub enum DownloadEvent {
    Started { task_id: String, total_size: u64 },
    Progress { task_id: String, downloaded: u64, speed: u64, percent: f64 },
    ChunkCompleted { task_id: String, chunk_id: u32 },
    Completed { task_id: String, file_path: String },
    Failed { task_id: String, error: String },
    Paused { task_id: String },
    Resumed { task_id: String },
}

impl EventEmitter {
    /// 发送事件到前端
    pub fn emit(&self, event: DownloadEvent) -> Result<()>;
}
```

---

## 4. 数据结构设计

### 4.1 下载配置

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub task_id: String,
    pub url: String,
    pub output_path: PathBuf,
    pub threads: u32,          // 线程数
    pub chunk_size: u64,       // 分片大小 (bytes)
    pub retry_count: u32,      // 重试次数
    pub timeout: u64,          // 超时时间 (秒)
    pub resume_enabled: bool,  // 是否启用断点续传
}
```

### 4.2 下载进度

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub task_id: String,
    pub total_size: u64,
    pub downloaded: u64,
    pub speed: u64,            // bytes/s
    pub percent: f64,          // 0.0 - 100.0
    pub eta: Option<u64>,      // 预估剩余时间 (秒)
    pub active_threads: u32,
}
```

### 4.3 分片状态

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkStateInfo {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub status: String,
    pub temp_file: String,
}
```

---

## 5. 核心流程

### 5.1 下载流程

```
┌─────────────────────────────────────────────────────────────┐
│                      下载流程                                │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. 接收下载请求                                             │
│     ↓                                                        │
│  2. 检测 Range 支持 (HEAD 请求)                              │
│     ↓                                                        │
│  3. 计算分片策略                                             │
│     ↓                                                        │
│  4. 检查是否有未完成的断点                                   │
│     ↓                                                        │
│  5. 启动线程池                                               │
│     ↓                                                        │
│  6. 并发下载分片                                             │
│     │                                                        │
│     ├── Worker 1 → 下载分片 1 → 写入临时文件                │
│     ├── Worker 2 → 下载分片 2 → 写入临时文件                │
│     └── Worker N → 下载分片 N → 写入临时文件                │
│     ↓                                                        │
│  7. 合并分片                                                 │
│     ↓                                                        │
│  8. 验证文件完整性                                           │
│     ↓                                                        │
│  9. 清理临时文件                                             │
│     ↓                                                        │
│  10. 发送完成事件                                            │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 断点续传流程

```
┌─────────────────────────────────────────────────────────────┐
│                    断点续传流程                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. 加载保存的状态文件                                       │
│     ↓                                                        │
│  2. 验证 URL 和文件大小是否匹配                              │
│     ↓                                                        │
│  3. 筛选未完成的分片                                         │
│     ↓                                                        │
│  4. 从断点位置继续下载                                       │
│     ↓                                                        │
│  5. 合并所有分片                                             │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 5.3 进度更新流程

```
┌─────────────────────────────────────────────────────────────┐
│                    进度更新流程                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Worker 下载分片数据                                         │
│     ↓                                                        │
│  更新 ChunkManager 分片状态                                  │
│     ↓                                                        │
│  计算总进度和速度                                            │
│     ↓                                                        │
│  发送 ProgressEvent                                          │
│     ↓                                                        │
│  前端接收并更新 UI                                           │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. 错误处理

### 6.1 错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("服务器不支持 Range 请求")]
    RangeNotSupported,
    
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("分片 {0} 下载失败: {1}")]
    ChunkFailed(u32, String),
    
    #[error("文件合并失败: {0}")]
    MergeFailed(String),
    
    #[error("状态文件损坏: {0}")]
    StateCorrupted(String),
    
    #[error("任务已取消")]
    Cancelled,
}
```

### 6.2 重试策略

```rust
pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl RetryPolicy {
    pub async fn execute<F, T, E>(&self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> std::pin::Pin<Box<dyn Future<Output = Result<T, E>>>>,
    {
        let mut attempt = 0;
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < self.max_retries => {
                    let delay = self.calculate_delay(attempt);
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

---

## 7. 性能优化

### 7.1 并发控制

- 使用 `tokio::sync::Semaphore` 限制最大并发数
- 每个 Worker 独立的异步任务

### 7.2 内存优化

- 流式下载，避免一次性加载大文件到内存
- 分片缓冲区大小可配置

### 7.3 磁盘 I/O 优化

- 使用 `tokio::fs` 异步文件操作
- 批量写入减少 I/O 次数

---

## 8. 目录结构

```
crates/turbo-downloader/src/
├── lib.rs
├── range/
│   ├── mod.rs
│   ├── client.rs       # RangeClient
│   └── support.rs      # Range 支持检测
├── chunk/
│   ├── mod.rs
│   ├── manager.rs      # ChunkManager
│   ├── worker.rs       # ChunkWorker
│   └── state.rs        # ChunkState
├── pool/
│   ├── mod.rs
│   └── worker_pool.rs  # WorkerPool
├── storage/
│   ├── mod.rs
│   ├── writer.rs       # ChunkWriter
│   ├── merger.rs       # FileMerger
│   └── state.rs        # StateManager
├── event/
│   ├── mod.rs
│   └── emitter.rs      # EventEmitter
└── error.rs
```

---

## 9. Tauri 集成

### 9.1 命令定义

```rust
#[tauri::command]
async fn start_download(
    config: DownloadConfig,
    app_handle: AppHandle,
) -> Result<String, String>;

#[tauri::command]
async fn pause_download(task_id: String) -> Result<(), String>;

#[tauri::command]
async fn resume_download(task_id: String) -> Result<(), String>;

#[tauri::command]
async fn cancel_download(task_id: String) -> Result<(), String>;

#[tauri::command]
async fn get_download_progress(task_id: String) -> Result<DownloadProgress, String>;
```

### 9.2 事件定义

| 事件名 | 载荷 | 说明 |
|--------|------|------|
| `download:started` | `{ task_id, total_size }` | 下载开始 |
| `download:progress` | `{ task_id, downloaded, speed, percent }` | 进度更新 |
| `download:completed` | `{ task_id, file_path }` | 下载完成 |
| `download:failed` | `{ task_id, error }` | 下载失败 |
| `download:paused` | `{ task_id }` | 下载暂停 |
| `download:resumed` | `{ task_id }` | 下载恢复 |

---

*架构设计版本: v1.0*
*设计日期: 2026-03-27*