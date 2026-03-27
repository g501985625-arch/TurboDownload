# P1: turbo-downloader 接口详细设计

## 1. 核心类型定义

### 1.1 下载配置

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    /// 下载唯一标识
    pub id: TaskId,
    /// 下载 URL
    pub url: String,
    /// 输出文件路径
    pub output_path: PathBuf,
    /// 分片数量 (0 = 自动)
    pub threads: u32,
    /// 分片大小 (0 = 自动)
    pub chunk_size: u64,
    /// 是否支持断点续传
    pub resume_support: bool,
    /// User-Agent
    pub user_agent: Option<String>,
    /// 自定义请求头
    pub headers: HashMap<String, String>,
    /// 速度限制 (0 = 无限制) 单位: bytes/s
    pub speed_limit: u64,
    /// 重试次数
    pub retry_count: u32,
    /// 超时时间 (秒)
    pub timeout: u64,
}
```

### 1.2 下载进度

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    /// 任务 ID
    pub task_id: TaskId,
    /// 总大小 (bytes)
    pub total: u64,
    /// 已下载 (bytes)
    pub downloaded: u64,
    /// 当前速度 (bytes/s)
    pub speed: u64,
    /// 预估剩余时间 (秒)
    pub eta: Option<u64>,
    /// 完成百分比 (0-100)
    pub percent: f64,
    /// 当前状态
    pub status: ProgressStatus,
    /// 错误信息 (如有)
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ProgressStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
}
```

### 1.3 下载结果

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    /// 任务 ID
    pub task_id: TaskId,
    /// 文件路径
    pub file_path: PathBuf,
    /// 文件大小
    pub file_size: u64,
    /// 总耗时 (毫秒)
    pub duration_ms: u64,
    /// 平均速度 (bytes/s)
    pub avg_speed: u64,
}
```

---

## 2. 核心接口

### 2.1 DownloaderBuilder (建造者模式)

```rust
/// 下载器建造者
pub struct DownloaderBuilder {
    max_concurrent_tasks: usize,
    default_threads: u32,
    default_chunk_size: u64,
    default_timeout: u64,
    default_retry_count: u32,
}

impl DownloaderBuilder {
    /// 创建新的建造者
    pub fn new() -> Self;
    
    /// 设置最大并发任务数
    pub fn max_concurrent_tasks(mut self, count: usize) -> Self;
    
    /// 设置默认线程数
    pub fn default_threads(mut self, threads: u32) -> Self;
    
    /// 设置默认分片大小
    pub fn default_chunk_size(mut self, size: u64) -> Self;
    
    /// 设置默认超时时间
    pub fn default_timeout(mut self, seconds: u64) -> Self;
    
    /// 设置默认重试次数
    pub fn default_retry_count(mut self, count: u32) -> Self;
    
    /// 构建下载器
    pub fn build(self) -> Result<Downloader>;
}
```

### 2.2 Downloader (主接口)

```rust
/// 下载器
pub struct Downloader {
    inner: Arc<Inner>,
}

pub struct Inner {
    client: reqwest::Client,
    manager: download::Manager,
}

impl Downloader {
    /// 创建下载任务
    /// 
    /// # Arguments
    /// * `config` - 下载配置
    /// 
    /// # Returns
    /// * `TaskId` - 任务 ID
    /// 
    /// # Example
    /// ```ignore
    /// let task_id = downloader.create_task(config).await?;
    /// ```
    pub async fn create_task(&self, config: DownloadConfig) -> Result<TaskId>;
    
    /// 开始/继续下载
    /// 
    /// # Arguments
    /// * `task_id` - 任务 ID
    /// * `callback` - 进度回调 (可选)
    /// 
    /// # Returns
    /// * `DownloadResult` - 下载结果
    /// 
    /// # Example
    /// ```ignore
    /// let result = downloader.start(&task_id, Some(Box::new(|progress| {
    ///     println!("Progress: {:.2}%", progress.percent);
    /// }))).await?;
    /// ```
    pub async fn start(
        &self,
        task_id: &TaskId,
        callback: Option<ProgressCallback>,
    ) -> Result<DownloadResult>;
    
    /// 暂停下载
    /// 
    /// # Arguments
    /// * `task_id` - 任务 ID
    pub async fn pause(&self, task_id: &TaskId) -> Result<()>;
    
    /// 恢复下载
    /// 
    /// # Arguments
    /// * `task_id` - 任务 ID
    pub async fn resume(&self, task_id: &TaskId) -> Result<()>;
    
    /// 取消下载
    /// 
    /// # Arguments
    /// * `task_id` - 任务 ID
    pub async fn cancel(&self, task_id: &TaskId) -> Result<()>;
    
    /// 获取下载进度
    /// 
    /// # Arguments
    /// * `task_id` - 任务 ID
    /// 
    /// # Returns
    /// * `DownloadProgress` - 进度信息
    pub fn get_progress(&self, task_id: &TaskId) -> Result<DownloadProgress>;
    
    /// 列出所有任务
    pub fn list_tasks(&self) -> Vec<TaskInfo>;
    
    /// 删除任务记录
    pub async fn remove_task(&self, task_id: &TaskId) -> Result<()>;
}
```

### 2.3 ProgressCallback (进度回调)

```rust
/// 进度回调函数
/// 
/// # Arguments
/// * `progress` - 当前进度
pub type ProgressCallback = Box<dyn Fn(DownloadProgress) + Send + Sync>;

/// 简化回调创建
pub fn progress_callback<F>(f: F) -> Option<ProgressCallback>
where
    F: Fn(DownloadProgress) + Send + Sync + 'static,
{
    Some(Box::new(f))
}
```

---

## 3. 模块级接口

### 3.1 HTTP 模块

```rust
pub mod http {
    /// HTTP 客户端
    pub struct Client {
        client: reqwest::Client,
    }
    
    impl Client {
        /// HEAD 请求 (获取文件信息)
        pub async fn head(&self, url: &str) -> Result<HeadResponse>;
        
        /// 分片下载请求
        pub async fn get_range(
            &self,
            url: &str,
            range: Range<u64>,
        ) -> Result<Bytes>;
    }
    
    /// HEAD 响应
    pub struct HeadResponse {
        /// 文件大小
        pub content_length: Option<u64>,
        /// 支持范围请求
        pub accept_ranges: Option<String>,
        /// ETag
        pub etag: Option<String>,
        /// 内容类型
        pub content_type: Option<String>,
        /// 最后修改时间
        pub last_modified: Option<DateTime<Utc>>,
    }
}
```

### 3.2 分片模块

```rust
pub mod chunk {
    /// 分片策略
    pub struct Strategy;
    
    impl Strategy {
        /// 计算分片信息
        /// 
        /// # Arguments
        /// * `file_size` - 文件总大小
        /// * `thread_count` - 线程数
        /// * `min_chunk_size` - 最小分片大小
        pub fn calculate(
            file_size: u64,
            thread_count: u32,
            min_chunk_size: u64,
        ) -> Vec<Chunk>;
    }
    
    /// 分片
    #[derive(Debug, Clone)]
    pub struct Chunk {
        /// 分片 ID
        pub id: u32,
        /// 起始字节
        pub start: u64,
        /// 结束字节
        pub end: u64,
        /// 当前已下载
        pub downloaded: u64,
    }
    
    /// 分片工作器
    pub struct Worker {
        chunk: Chunk,
        url: String,
        client: http::Client,
    }
    
    impl Worker {
        /// 执行分片下载
        pub async fn download(
            &mut self,
            output_path: &Path,
            progress_tx: mpsc::Sender<ChunkProgress>,
        ) -> Result<()>;
    }
    
    /// 分片进度
    #[derive(Debug, Clone)]
    pub struct ChunkProgress {
        pub chunk_id: u32,
        pub downloaded: u64,
    }
}
```

### 3.3 进度模块

```rust
pub mod progress {
    /// 进度追踪器
    pub struct Tracker {
        total: u64,
        downloaded: AtomicU64,
        start_time: Instant,
    }
    
    impl Tracker {
        pub fn new(total: u64) -> Self;
        
        /// 更新分片进度
        pub fn update(&self, chunk_id: u32, downloaded: u64);
        
        /// 获取当前进度
        pub fn get_progress(&self) -> DownloadProgress;
        
        /// 计算当前速度
        pub fn calculate_speed(&self) -> u64;
    }
    
    /// 速度计算器
    pub struct SpeedCalculator {
        samples: Vec<SpeedSample>,
        window_size: usize,
    }
    
    impl SpeedCalculator {
        pub fn new(window_size: usize) -> Self;
        pub fn add_sample(&mut self, bytes: u64);
        pub fn get_speed(&self) -> u64;
    }
}
```

### 3.4 断点续传模块

```rust
pub mod resume {
    /// 状态持久化
    pub struct State;
    
    impl State {
        /// 保存状态
        pub async fn save(&self, task_id: &TaskId, state: &ResumeState) -> Result<()>;
        
        /// 加载状态
        pub async fn load(&self, task_id: &TaskId) -> Result<Option<ResumeState>>;
        
        /// 删除状态
        pub async fn remove(&self, task_id: &TaskId) -> Result<()>;
        
        /// 列出所有保存的状态
        pub async fn list(&self) -> Result<Vec<TaskId>>;
    }
    
    /// 恢复状态
    #[derive(Serialize, Deserialize)]
    pub struct ResumeState {
        pub task_id: TaskId,
        pub url: String,
        pub file_size: u64,
        pub downloaded: u64,
        pub chunks: Vec<ChunkState>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }
    
    /// 分片状态
    #[derive(Serialize, Deserialize)]
    pub struct ChunkState {
        pub id: u32,
        pub start: u64,
        pub end: u64,
        pub downloaded: u64,
        pub temp_path: PathBuf,
    }
}
```

---

## 4. 错误类型

```rust
pub mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum DownloadError {
        #[error("网络错误: {0}")]
        Network(#[from] reqwest::Error),
        
        #[error("HTTP 错误: {0} - {1}")]
        Http(u16, String),
        
        #[error("IO 错误: {0}")]
        Io(#[from] std::io::Error),
        
        #[error("任务不存在: {0}")]
        TaskNotFound(TaskId),
        
        #[error("任务已存在: {0}")]
        TaskAlreadyExists(TaskId),
        
        #[error("任务已暂停")]
        TaskPaused,
        
        #[error("任务已完成")]
        TaskCompleted,
        
        #[error("不支持的范围请求")]
        RangeNotSupported,
        
        #[error("文件校验失败: {0}")]
        ValidationFailed(String),
        
        #[error("超时: {0}")]
        Timeout(String),
        
        #[error("取消请求")]
        Cancelled,
        
        #[error("其他错误: {0}")]
        Other(String),
    }
    
    pub type Result<T> = std::result::Result<T, DownloadError>;
}
```

---

## 5. 使用示例

### 5.1 基础下载

```rust
use turbo_downloader::{DownloaderBuilder, DownloadConfig, DownloadProgress};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建下载器
    let downloader = DownloaderBuilder::new()
        .max_concurrent_tasks(3)
        .default_threads(4)
        .build()?;
    
    // 创建下载配置
    let config = DownloadConfig {
        id: uuid::Uuid::new_v4(),
        url: "https://example.com/file.zip".to_string(),
        output_path: PathBuf::from("./downloads/file.zip"),
        threads: 4,
        chunk_size: 0,
        resume_support: true,
        user_agent: Some("TurboDownload/1.0".to_string()),
        headers: Default::default(),
        speed_limit: 0,
        retry_count: 3,
        timeout: 300,
    };
    
    // 创建任务
    let task_id = downloader.create_task(config).await?;
    
    // 开始下载 (带进度回调)
    let result = downloader.start(
        &task_id,
        Some(Box::new(|progress: DownloadProgress| {
            println!(
                "Progress: {:.1}% | Speed: {} KB/s | ETA: {:?}",
                progress.percent,
                progress.speed / 1024,
                progress.eta.map(|s| format!("{}s", s))
            );
        }))
    ).await?;
    
    println!("Download completed: {:?}", result);
    Ok(())
}
```

### 5.2 断点续传

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = DownloaderBuilder::new().build()?;
    
    // 恢复之前暂停的任务
    let task_id = TaskId::from_u128(previous_task_id);
    
    // 检查状态是否存在
    if let Some(progress) = downloader.get_progress(&task_id).ok() {
        if progress.status == ProgressStatus::Paused {
            // 恢复下载
            downloader.resume(&task_id).await?;
        }
    }
    
    Ok(())
}
```

---

## 6. 设计原则

1. **建造者模式**: `DownloaderBuilder` 提供灵活的配置
2. **异步优先**: 所有 I/O 操作使用 `async/await`
3. **错误透明**: 使用 `thiserror` 提供清晰的错误信息
4. **进度解耦**: 进度追踪与下载逻辑分离
5. **状态可恢复**: 断点信息持久化，支持崩溃恢复

---

*接口设计版本: v0.1.0*
*设计日期: 2026-03-26*