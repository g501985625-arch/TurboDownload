# P1: turbo-downloader 代码模板

本文档提供核心代码结构和实现模板。

---

## 核心结构体定义

### 1. 下载配置 (DownloadConfig)

```rust
// src/download/config.rs

use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 下载任务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    /// 任务唯一标识
    pub id: String,
    
    /// 下载 URL
    pub url: String,
    
    /// 输出文件路径
    pub output_path: PathBuf,
    
    /// 并发线程数
    pub threads: usize,
    
    /// 分片大小 (0 表示自动)
    pub chunk_size: u64,
    
    /// 是否支持断点续传
    pub resume_support: bool,
    
    /// 自定义 User-Agent
    pub user_agent: Option<String>,
    
    /// 自定义请求头
    pub headers: HashMap<String, String>,
    
    /// 下载速度限制 (字节/秒, 0 表示不限)
    pub speed_limit: u64,
    
    /// 最大重试次数
    pub max_retries: u32,
    
    /// 连接超时 (秒)
    pub connect_timeout: u64,
    
    /// 读取超时 (秒)
    pub read_timeout: u64,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url: String::new(),
            output_path: PathBuf::new(),
            threads: 4,
            chunk_size: 0, // 自动
            resume_support: true,
            user_agent: None,
            headers: HashMap::new(),
            speed_limit: 0,
            max_retries: 3,
            connect_timeout: 30,
            read_timeout: 60,
        }
    }
}

impl DownloadConfig {
    /// 创建新的下载配置
    pub fn new(url: impl Into<String>, output_path: impl Into<PathBuf>) -> Self {
        Self {
            url: url.into(),
            output_path: output_path.into(),
            ..Default::default()
        }
    }
    
    /// 设置并发线程数
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads.max(1).min(32);
        self
    }
    
    /// 设置自定义请求头
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
    
    /// 设置速度限制
    pub fn with_speed_limit(mut self, bytes_per_sec: u64) -> Self {
        self.speed_limit = bytes_per_sec;
        self
    }
}
```

---

### 2. 下载进度 (DownloadProgress)

```rust
// src/progress/progress.rs

use std::time::{Duration, Instant};

/// 下载进度信息
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    /// 任务 ID
    pub task_id: String,
    
    /// 总字节数
    pub total: u64,
    
    /// 已下载字节数
    pub downloaded: u64,
    
    /// 当前下载速度 (字节/秒)
    pub speed: f64,
    
    /// 预计剩余时间
    pub eta: Option<Duration>,
    
    /// 已用时间
    pub elapsed: Duration,
    
    /// 完成百分比 (0.0 - 100.0)
    pub percentage: f64,
    
    /// 当前活跃线程数
    pub active_threads: usize,
    
    /// 时间戳
    pub timestamp: Instant,
}

impl DownloadProgress {
    /// 创建新的进度信息
    pub fn new(task_id: String, total: u64) -> Self {
        Self {
            task_id,
            total,
            downloaded: 0,
            speed: 0.0,
            eta: None,
            elapsed: Duration::ZERO,
            percentage: 0.0,
            active_threads: 0,
            timestamp: Instant::now(),
        }
    }
    
    /// 更新进度
    pub fn update(&mut self, downloaded: u64, speed: f64) {
        self.downloaded = downloaded;
        self.speed = speed;
        self.percentage = if self.total > 0 {
            (self.downloaded as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };
        self.eta = if speed > 0.0 {
            let remaining_bytes = self.total.saturating_sub(self.downloaded);
            Some(Duration::from_secs_f64(remaining_bytes as f64 / speed))
        } else {
            None
        };
        self.timestamp = Instant::now();
    }
    
    /// 检查是否完成
    pub fn is_completed(&self) -> bool {
        self.total > 0 && self.downloaded >= self.total
    }
}

/// 进度回调类型
pub type ProgressCallback = Box<dyn Fn(DownloadProgress) + Send + Sync>;

/// 进度回调选项
pub struct ProgressOptions {
    /// 回调间隔 (毫秒)
    pub interval_ms: u64,
    
    /// 是否在速度变化时触发
    pub on_speed_change: bool,
    
    /// 最小进度变化百分比
    pub min_progress_delta: f64,
}

impl Default for ProgressOptions {
    fn default() -> Self {
        Self {
            interval_ms: 500,
            on_speed_change: false,
            min_progress_delta: 1.0,
        }
    }
}
```

---

### 3. 下载任务状态 (TaskStatus)

```rust
// src/download/status.rs

use serde::{Deserialize, Serialize};

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// 等待中
    Pending,
    
    /// 准备中 (获取文件信息)
    Preparing,
    
    /// 下载中
    Downloading,
    
    /// 已暂停
    Paused,
    
    /// 已完成
    Completed,
    
    /// 失败
    Failed,
    
    /// 已取消
    Cancelled,
}

impl TaskStatus {
    /// 检查是否可以开始
    pub fn can_start(&self) -> bool {
        matches!(self, Self::Pending | Self::Paused | Self::Failed)
    }
    
    /// 检查是否可以暂停
    pub fn can_pause(&self) -> bool {
        matches!(self, Self::Downloading)
    }
    
    /// 检查是否可以恢复
    pub fn can_resume(&self) -> bool {
        matches!(self, Self::Paused)
    }
    
    /// 检查是否已结束
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// 任务错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskError {
    /// 错误类型
    pub kind: ErrorKind,
    
    /// 错误消息
    pub message: String,
    
    /// 错误时间
    pub timestamp: i64,
    
    /// 是否可重试
    pub retryable: bool,
    
    /// 重试次数
    pub retry_count: u32,
}

/// 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorKind {
    /// 网络错误
    Network,
    
    /// 连接超时
    Timeout,
    
    /// DNS 解析失败
    Dns,
    
    /// HTTP 错误
    Http,
    
    /// 文件系统错误
    FileSystem,
    
    /// 配置错误
    Config,
    
    /// 内部错误
    Internal,
}
```

---

### 4. 分片定义 (Chunk)

```rust
// src/chunk/chunk.rs

use serde::{Deserialize, Serialize};

/// 分片范围
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChunkRange {
    /// 起始字节 (包含)
    pub start: u64,
    
    /// 结束字节 (包含)
    pub end: u64,
}

impl ChunkRange {
    /// 创建新的分片范围
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
    
    /// 获取分片大小
    pub fn size(&self) -> u64 {
        self.end - self.start + 1
    }
    
    /// 转换为 HTTP Range header 值
    pub fn to_range_header(&self) -> String {
        format!("bytes={}-{}", self.start, self.end)
    }
}

/// 分片状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChunkStatus {
    /// 等待下载
    Pending,
    
    /// 下载中
    Downloading,
    
    /// 已完成
    Completed,
    
    /// 失败
    Failed,
}

/// 分片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// 分片索引
    pub index: usize,
    
    /// 分片范围
    pub range: ChunkRange,
    
    /// 分片状态
    pub status: ChunkStatus,
    
    /// 已下载字节数
    pub downloaded: u64,
    
    /// 重试次数
    pub retry_count: u32,
    
    /// 最后错误
    pub last_error: Option<String>,
}

impl Chunk {
    /// 创建新分片
    pub fn new(index: usize, range: ChunkRange) -> Self {
        Self {
            index,
            range,
            status: ChunkStatus::Pending,
            downloaded: 0,
            retry_count: 0,
            last_error: None,
        }
    }
    
    /// 获取进度百分比
    pub fn progress(&self) -> f64 {
        if self.range.size() == 0 {
            return 0.0;
        }
        (self.downloaded as f64 / self.range.size() as f64) * 100.0
    }
    
    /// 标记开始下载
    pub fn start(&mut self) {
        self.status = ChunkStatus::Downloading;
    }
    
    /// 标记完成
    pub fn complete(&mut self) {
        self.status = ChunkStatus::Completed;
        self.downloaded = self.range.size();
    }
    
    /// 标记失败
    pub fn fail(&mut self, error: impl Into<String>) {
        self.status = ChunkStatus::Failed;
        self.last_error = Some(error.into());
        self.retry_count += 1;
    }
}
```

---

## 核心 Trait 定义

### 1. 分片策略 Trait

```rust
// src/chunk/strategy.rs

use crate::chunk::ChunkRange;

/// 分片策略 trait
pub trait ChunkStrategy: Send + Sync {
    /// 根据文件大小生成分片计划
    fn plan(&self, total_size: u64) -> Vec<ChunkRange>;
    
    /// 获取策略名称
    fn name(&self) -> &str;
}

/// 固定大小分片策略
pub struct FixedSizeStrategy {
    chunk_size: u64,
}

impl FixedSizeStrategy {
    pub fn new(chunk_size: u64) -> Self {
        Self { chunk_size }
    }
}

impl ChunkStrategy for FixedSizeStrategy {
    fn plan(&self, total_size: u64) -> Vec<ChunkRange> {
        let mut chunks = Vec::new();
        let mut start = 0u64;
        
        while start < total_size {
            let end = (start + self.chunk_size - 1).min(total_size - 1);
            chunks.push(ChunkRange::new(start, end));
            start = end + 1;
        }
        
        chunks
    }
    
    fn name(&self) -> &str {
        "fixed_size"
    }
}

/// 固定数量分片策略
pub struct FixedCountStrategy {
    chunk_count: usize,
}

impl FixedCountStrategy {
    pub fn new(chunk_count: usize) -> Self {
        Self { chunk_count: chunk_count.max(1) }
    }
}

impl ChunkStrategy for FixedCountStrategy {
    fn plan(&self, total_size: u64) -> Vec<ChunkRange> {
        if total_size == 0 {
            return vec![];
        }
        
        let chunk_size = total_size / self.chunk_count as u64;
        let remainder = total_size % self.chunk_count as u64;
        
        let mut chunks = Vec::with_capacity(self.chunk_count);
        let mut start = 0u64;
        
        for i in 0..self.chunk_count {
            let extra = if i < remainder as usize { 1 } else { 0 };
            let end = start + chunk_size + extra - 1;
            chunks.push(ChunkRange::new(start, end.min(total_size - 1)));
            start = end + 1;
        }
        
        chunks
    }
    
    fn name(&self) -> &str {
        "fixed_count"
    }
}

/// 自适应分片策略
pub struct AdaptiveStrategy {
    min_chunk_size: u64,
    max_chunk_size: u64,
    target_chunks: usize,
}

impl AdaptiveStrategy {
    pub fn new() -> Self {
        Self {
            min_chunk_size: 1024 * 1024,      // 1 MB
            max_chunk_size: 100 * 1024 * 1024, // 100 MB
            target_chunks: 8,
        }
    }
}

impl ChunkStrategy for AdaptiveStrategy {
    fn plan(&self, total_size: u64) -> Vec<ChunkRange> {
        if total_size == 0 {
            return vec![];
        }
        
        // 计算理想分片大小
        let ideal_chunk_size = total_size / self.target_chunks as u64;
        let chunk_size = ideal_chunk_size
            .max(self.min_chunk_size)
            .min(self.max_chunk_size);
        
        // 使用固定大小策略
        FixedSizeStrategy::new(chunk_size).plan(total_size)
    }
    
    fn name(&self) -> &str {
        "adaptive"
    }
}

impl Default for AdaptiveStrategy {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### 2. 进度追踪器 Trait

```rust
// src/progress/tracker.rs

use crate::progress::DownloadProgress;
use std::time::Duration;

/// 进度追踪器 trait
pub trait ProgressTracker: Send + Sync {
    /// 更新进度
    fn update(&self, downloaded: u64, speed: f64);
    
    /// 获取当前进度
    fn progress(&self) -> DownloadProgress;
    
    /// 重置进度
    fn reset(&self);
    
    /// 设置回调
    fn set_callback(&mut self, callback: ProgressCallback, interval: Duration);
}

/// 进度回调类型
pub type ProgressCallback = Box<dyn Fn(DownloadProgress) + Send + Sync>;

/// 基础进度追踪器实现
pub struct BasicProgressTracker {
    task_id: String,
    total: u64,
    downloaded: std::sync::atomic::AtomicU64,
    speed: std::sync::atomic::AtomicU64, // 存储为整数避免浮点精度问题
    start_time: std::time::Instant,
    callback: Option<ProgressCallback>,
    callback_interval: Duration,
    last_callback: std::sync::Mutex<std::time::Instant>,
}

impl BasicProgressTracker {
    pub fn new(task_id: String, total: u64) -> Self {
        Self {
            task_id,
            total,
            downloaded: std::sync::atomic::AtomicU64::new(0),
            speed: std::sync::atomic::AtomicU64::new(0),
            start_time: std::time::Instant::now(),
            callback: None,
            callback_interval: Duration::from_millis(500),
            last_callback: std::sync::Mutex::new(std::time::Instant::now()),
        }
    }
}

impl ProgressTracker for BasicProgressTracker {
    fn update(&self, downloaded: u64, speed: f64) {
        self.downloaded.store(downloaded, std::sync::atomic::Ordering::Relaxed);
        self.speed.store(speed as u64, std::sync::atomic::Ordering::Relaxed);
        
        // 触发回调
        if let Some(ref callback) = self.callback {
            let mut last = self.last_callback.lock().unwrap();
            if last.elapsed() >= self.callback_interval {
                callback(self.progress());
                *last = std::time::Instant::now();
            }
        }
    }
    
    fn progress(&self) -> DownloadProgress {
        let downloaded = self.downloaded.load(std::sync::atomic::Ordering::Relaxed);
        let speed = self.speed.load(std::sync::atomic::Ordering::Relaxed) as f64;
        
        DownloadProgress {
            task_id: self.task_id.clone(),
            total: self.total,
            downloaded,
            speed,
            eta: if speed > 0.0 {
                Some(Duration::from_secs_f64(
                    (self.total.saturating_sub(downloaded)) as f64 / speed
                ))
            } else {
                None
            },
            elapsed: self.start_time.elapsed(),
            percentage: if self.total > 0 {
                (downloaded as f64 / self.total as f64) * 100.0
            } else {
                0.0
            },
            active_threads: 0, // 由外部更新
            timestamp: std::time::Instant::now(),
        }
    }
    
    fn reset(&self) {
        self.downloaded.store(0, std::sync::atomic::Ordering::Relaxed);
        self.speed.store(0, std::sync::atomic::Ordering::Relaxed);
    }
    
    fn set_callback(&mut self, callback: ProgressCallback, interval: Duration) {
        self.callback = Some(callback);
        self.callback_interval = interval;
    }
}
```

---

### 3. 重试策略 Trait

```rust
// src/retry/strategy.rs

use std::time::Duration;
use crate::error::DownloadError;

/// 重试策略 trait
pub trait RetryStrategy: Send + Sync {
    /// 判断是否应该重试
    fn should_retry(&self, error: &DownloadError, attempt: u32) -> bool;
    
    /// 获取下次重试的等待时间
    fn delay(&self, attempt: u32) -> Duration;
    
    /// 获取策略名称
    fn name(&self) -> &str;
}

/// 固定间隔重试策略
pub struct FixedRetry {
    max_attempts: u32,
    delay: Duration,
}

impl FixedRetry {
    pub fn new(max_attempts: u32, delay: Duration) -> Self {
        Self { max_attempts, delay }
    }
}

impl RetryStrategy for FixedRetry {
    fn should_retry(&self, error: &DownloadError, attempt: u32) -> bool {
        attempt < self.max_attempts && error.is_retryable()
    }
    
    fn delay(&self, _attempt: u32) -> Duration {
        self.delay
    }
    
    fn name(&self) -> &str {
        "fixed"
    }
}

/// 指数退避重试策略
pub struct ExponentialBackoff {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
}

impl ExponentialBackoff {
    pub fn new() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
    
    pub fn max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }
    
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }
    
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }
}

impl RetryStrategy for ExponentialBackoff {
    fn should_retry(&self, error: &DownloadError, attempt: u32) -> bool {
        attempt < self.max_attempts && error.is_retryable()
    }
    
    fn delay(&self, attempt: u32) -> Duration {
        let delay_ms = self.initial_delay.as_millis() as f64
            * self.multiplier.powi(attempt as i32);
        
        Duration::from_millis(delay_ms.min(self.max_delay.as_millis() as f64) as u64)
    }
    
    fn name(&self) -> &str {
        "exponential_backoff"
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 主要函数实现

### 1. HTTP 客户端

```rust
// src/http/client.rs

use reqwest::{Client, Response, header};
use std::time::Duration;
use crate::error::{DownloadError, Result};

/// HTTP 客户端配置
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub user_agent: String,
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub max_redirects: usize,
    pub proxy: Option<String>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            user_agent: format!("TurboDownloader/{}", env!("CARGO_PKG_VERSION")),
            connect_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(60),
            max_redirects: 10,
            proxy: None,
        }
    }
}

/// HTTP 客户端
pub struct HttpClient {
    client: Client,
    config: HttpClientConfig,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new(config: HttpClientConfig) -> Result<Self> {
        let mut builder = Client::builder()
            .user_agent(&config.user_agent)
            .connect_timeout(config.connect_timeout)
            .timeout(config.read_timeout)
            .redirect(reqwest::redirect::Policy::limited(config.max_redirects));
        
        // 配置代理
        if let Some(ref proxy) = config.proxy {
            builder = builder.proxy(reqwest::Proxy::all(proxy)
                .map_err(|e| DownloadError::Config(e.to_string()))?);
        }
        
        let client = builder.build()
            .map_err(|e| DownloadError::Config(e.to_string()))?;
        
        Ok(Self { client, config })
    }
    
    /// 发送 HEAD 请求获取文件信息
    pub async fn head(&self, url: &str) -> Result<FileInfo> {
        let response = self.client
            .head(url)
            .send()
            .await
            .map_err(|e| DownloadError::Network(e.to_string()))?;
        
        self.parse_file_info(response).await
    }
    
    /// 发送带范围的 GET 请求
    pub async fn get_range(&self, url: &str, start: u64, end: u64) -> Result<Response> {
        let range_header = format!("bytes={}-{}", start, end);
        
        self.client
            .get(url)
            .header(header::RANGE, range_header)
            .send()
            .await
            .map_err(|e| DownloadError::Network(e.to_string()))
    }
    
    /// 获取完整文件
    pub async fn get(&self, url: &str) -> Result<Response> {
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| DownloadError::Network(e.to_string()))
    }
    
    /// 解析文件信息
    async fn parse_file_info(&self, response: Response) -> Result<FileInfo> {
        let headers = response.headers();
        
        let content_length = headers
            .get(header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        
        let accept_ranges = headers
            .get(header::ACCEPT_RANGES)
            .and_then(|v| v.to_str().ok())
            .map(|v| v == "bytes")
            .unwrap_or(false);
        
        let etag = headers
            .get(header::ETAG)
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        
        let last_modified = headers
            .get(header::LAST_MODIFIED)
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        
        Ok(FileInfo {
            size: content_length,
            supports_range: accept_ranges,
            etag,
            last_modified,
        })
    }
}

/// 文件信息
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub size: u64,
    pub supports_range: bool,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}
```

---

### 2. 下载管理器

```rust
// src/download/manager.rs

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::error::Result;
use crate::http::HttpClient;
use crate::chunk::{Chunk, ChunkStrategy, AdaptiveStrategy};
use crate::progress::{DownloadProgress, ProgressTracker, BasicProgressTracker};
use crate::resume::{ResumeState, ResumeManager};
use crate::download::{DownloadConfig, DownloadTask, TaskStatus};

/// 下载管理器构建器
pub struct DownloaderBuilder {
    max_concurrent_tasks: usize,
    default_threads: usize,
    download_dir: PathBuf,
    temp_dir: PathBuf,
    enable_resume: bool,
}

impl DownloaderBuilder {
    pub fn new() -> Self {
        Self {
            max_concurrent_tasks: 3,
            default_threads: 4,
            download_dir: PathBuf::from("./downloads"),
            temp_dir: PathBuf::from("./temp"),
            enable_resume: true,
        }
    }
    
    pub fn max_concurrent_tasks(mut self, count: usize) -> Self {
        self.max_concurrent_tasks = count;
        self
    }
    
    pub fn default_threads(mut self, threads: usize) -> Self {
        self.default_threads = threads;
        self
    }
    
    pub fn download_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.download_dir = path.into();
        self
    }
    
    pub fn build(self) -> Result<Downloader> {
        Downloader::new(self)
    }
}

impl Default for DownloaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 下载管理器
pub struct Downloader {
    config: DownloaderConfig,
    http_client: Arc<HttpClient>,
    tasks: Arc<RwLock<Vec<Arc<DownloadTask>>>>,
    resume_manager: Option<ResumeManager>,
}

/// 下载管理器配置
struct DownloaderConfig {
    max_concurrent_tasks: usize,
    default_threads: usize,
    download_dir: PathBuf,
    temp_dir: PathBuf,
    enable_resume: bool,
}

impl Downloader {
    fn new(builder: DownloaderBuilder) -> Result<Self> {
        let config = DownloaderConfig {
            max_concurrent_tasks: builder.max_concurrent_tasks,
            default_threads: builder.default_threads,
            download_dir: builder.download_dir,
            temp_dir: builder.temp_dir,
            enable_resume: builder.enable_resume,
        };
        
        let http_client = Arc::new(HttpClient::new(Default::default())?);
        let resume_manager = if config.enable_resume {
            Some(ResumeManager::new(&config.temp_dir)?)
        } else {
            None
        };
        
        Ok(Self {
            config,
            http_client,
            tasks: Arc::new(RwLock::new(Vec::new())),
            resume_manager,
        })
    }
    
    /// 创建下载任务
    pub async fn create_task(&self, config: DownloadConfig) -> Result<String> {
        // 验证 URL
        let file_info = self.http_client.head(&config.url).await?;
        
        // 创建任务
        let task_id = config.id.clone();
        let task = DownloadTask::new(config, file_info)?;
        
        // 添加到任务列表
        self.tasks.write().await.push(Arc::new(task));
        
        Ok(task_id)
    }
    
    /// 开始下载
    pub async fn start(&self, task_id: &str, progress_callback: Option<ProgressCallback>) -> Result<()> {
        let task = self.get_task(task_id).await?;
        task.start(self.http_client.clone(), progress_callback).await
    }
    
    /// 暂停下载
    pub async fn pause(&self, task_id: &str) -> Result<()> {
        let task = self.get_task(task_id).await?;
        task.pause().await
    }
    
    /// 恢复下载
    pub async fn resume(&self, task_id: &str) -> Result<()> {
        let task = self.get_task(task_id).await?;
        task.resume().await
    }
    
    /// 取消下载
    pub async fn cancel(&self, task_id: &str) -> Result<()> {
        let task = self.get_task(task_id).await?;
        task.cancel().await
    }
    
    /// 获取任务状态
    pub async fn status(&self, task_id: &str) -> Result<TaskStatus> {
        let task = self.get_task(task_id).await?;
        Ok(task.status())
    }
    
    /// 获取任务进度
    pub async fn progress(&self, task_id: &str) -> Result<DownloadProgress> {
        let task = self.get_task(task_id).await?;
        Ok(task.progress())
    }
    
    /// 获取所有任务
    pub async fn list_tasks(&self) -> Vec<TaskInfo> {
        let tasks = self.tasks.read().await;
        tasks.iter().map(|t| t.info()).collect()
    }
    
    async fn get_task(&self, task_id: &str) -> Result<Arc<DownloadTask>> {
        let tasks = self.tasks.read().await;
        tasks.iter()
            .find(|t| t.id() == task_id)
            .cloned()
            .ok_or_else(|| DownloadError::NotFound(task_id.to_string()))
    }
}
```

---

### 3. 断点续传管理器

```rust
// src/resume/manager.rs

use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::error::{DownloadError, Result};
use crate::resume::ResumeState;
use crate::chunk::ChunkRange;

/// 断点续传管理器
pub struct ResumeManager {
    state_dir: PathBuf,
}

impl ResumeManager {
    pub fn new(state_dir: impl AsRef<Path>) -> Result<Self> {
        let state_dir = state_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&state_dir)
            .map_err(|e| DownloadError::FileSystem(e.to_string()))?;
        
        Ok(Self { state_dir })
    }
    
    /// 保存下载状态
    pub fn save(&self, state: &ResumeState) -> Result<()> {
        let path = self.state_path(&state.task_id);
        
        let json = serde_json::to_string_pretty(state)
            .map_err(|e| DownloadError::Serialization(e.to_string()))?;
        
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| DownloadError::FileSystem(e.to_string()))?;
        
        file.write_all(json.as_bytes())
            .map_err(|e| DownloadError::FileSystem(e.to_string()))?;
        
        Ok(())
    }
    
    /// 加载下载状态
    pub fn load(&self, task_id: &str) -> Result<Option<ResumeState>> {
        let path = self.state_path(task_id);
        
        if !path.exists() {
            return Ok(None);
        }
        
        let mut file = File::open(&path)
            .map_err(|e| DownloadError::FileSystem(e.to_string()))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| DownloadError::FileSystem(e.to_string()))?;
        
        let state: ResumeState = serde_json::from_str(&content)
            .map_err(|e| DownloadError::Serialization(e.to_string()))?;
        
        Ok(Some(state))
    }
    
    /// 删除下载状态
    pub fn delete(&self, task_id: &str) -> Result<()> {
        let path = self.state_path(task_id);
        
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| DownloadError::FileSystem(e.to_string()))?;
        }
        
        Ok(())
    }
    
    /// 检查是否存在保存的状态
    pub fn exists(&self, task_id: &str) -> bool {
        self.state_path(task_id).exists()
    }
    
    fn state_path(&self, task_id: &str) -> PathBuf {
        self.state_dir.join(format!("{}.json", task_id))
    }
}

/// 下载状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResumeState {
    /// 版本号
    pub version: u32,
    
    /// 任务 ID
    pub task_id: String,
    
    /// 下载 URL
    pub url: String,
    
    /// 输出文件路径
    pub output_path: PathBuf,
    
    /// 文件总大小
    pub total_size: u64,
    
    /// 已下载大小
    pub downloaded: u64,
    
    /// 文件 ETag
    pub etag: Option<String>,
    
    /// 最后修改时间
    pub last_modified: Option<String>,
    
    /// 分片信息
    pub chunks: Vec<ChunkState>,
    
    /// 创建时间
    pub created_at: i64,
    
    /// 更新时间
    pub updated_at: i64,
}

/// 分片状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChunkState {
    /// 分片索引
    pub index: usize,
    
    /// 分片范围
    pub range: ChunkRange,
    
    /// 已下载字节数
    pub downloaded: u64,
    
    /// 是否完成
    pub completed: bool,
}

impl ResumeState {
    pub fn new(
        task_id: String,
        url: String,
        output_path: PathBuf,
        total_size: u64,
        chunks: Vec<ChunkRange>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        
        Self {
            version: 1,
            task_id,
            url,
            output_path,
            total_size,
            downloaded: 0,
            etag: None,
            last_modified: None,
            chunks: chunks.into_iter().enumerate().map(|(index, range)| {
                ChunkState {
                    index,
                    range,
                    downloaded: 0,
                    completed: false,
                }
            }).collect(),
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_chunk(&mut self, index: usize, downloaded: u64) {
        if let Some(chunk) = self.chunks.get_mut(index) {
            chunk.downloaded = downloaded;
            chunk.completed = downloaded >= chunk.range.size();
        }
        
        self.downloaded = self.chunks.iter().map(|c| c.downloaded).sum();
        self.updated_at = chrono::Utc::now().timestamp();
    }
    
    pub fn is_completed(&self) -> bool {
        self.chunks.iter().all(|c| c.completed)
    }
    
    pub fn remaining_chunks(&self) -> Vec<&ChunkState> {
        self.chunks.iter().filter(|c| !c.completed).collect()
    }
}
```

---

## 测试用例

### 1. HTTP 客户端测试

```rust
// tests/http_test.rs

use turbo_downloader::http::{HttpClient, HttpClientConfig};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_head_request() {
    let mut server = MockServer::start().await;
    
    Mock::given(method("HEAD"))
        .respond_with(ResponseTemplate::new(200)
            .insert_header("content-length", "1024")
            .insert_header("accept-ranges", "bytes"))
        .mount(&server)
        .await;
    
    let client = HttpClient::new(HttpClientConfig::default()).unwrap();
    let info = client.head(&server.uri()).await.unwrap();
    
    assert_eq!(info.size, 1024);
    assert!(info.supports_range);
}

#[tokio::test]
async fn test_range_request() {
    let mut server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(header("range", "bytes=0-99"))
        .respond_with(ResponseTemplate::new(206)
            .body(vec![0u8; 100]))
        .mount(&server)
        .await;
    
    let client = HttpClient::new(HttpClientConfig::default()).unwrap();
    let response = client.get_range(&server.uri(), 0, 99).await.unwrap();
    
    assert_eq!(response.status(), 206);
}

#[tokio::test]
async fn test_timeout() {
    let mut server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(10)))
        .mount(&server)
        .await;
    
    let config = HttpClientConfig {
        read_timeout: Duration::from_millis(100),
        ..Default::default()
    };
    let client = HttpClient::new(config).unwrap();
    
    let result = client.get(&server.uri()).await;
    assert!(result.is_err());
}
```

---

### 2. 分片策略测试

```rust
// tests/chunk_test.rs

use turbo_downloader::chunk::{ChunkStrategy, FixedSizeStrategy, FixedCountStrategy, AdaptiveStrategy};

#[test]
fn test_fixed_size_strategy() {
    let strategy = FixedSizeStrategy::new(100);
    let chunks = strategy.plan(350);
    
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[0].size(), 100);
    assert_eq!(chunks[1].size(), 100);
    assert_eq!(chunks[2].size(), 100);
    assert_eq!(chunks[3].size(), 50);
}

#[test]
fn test_fixed_count_strategy() {
    let strategy = FixedCountStrategy::new(3);
    let chunks = strategy.plan(100);
    
    assert_eq!(chunks.len(), 3);
    
    // 验证所有分片覆盖整个范围
    let total: u64 = chunks.iter().map(|c| c.size()).sum();
    assert_eq!(total, 100);
    
    // 验证分片之间没有重叠
    for i in 0..chunks.len() - 1 {
        assert_eq!(chunks[i].end + 1, chunks[i + 1].start);
    }
}

#[test]
fn test_adaptive_strategy() {
    let strategy = AdaptiveStrategy::new();
    
    // 小文件
    let chunks = strategy.plan(500_000);
    assert_eq!(chunks.len(), 1);
    
    // 中等文件
    let chunks = strategy.plan(50_000_000);
    assert!(chunks.len() >= 2);
    
    // 大文件
    let chunks = strategy.plan(1_000_000_000);
    assert!(chunks.len() >= 8);
}

#[test]
fn test_empty_file() {
    let strategy = FixedSizeStrategy::new(100);
    let chunks = strategy.plan(0);
    assert!(chunks.is_empty());
}
```

---

### 3. 下载任务测试

```rust
// tests/download_test.rs

use turbo_downloader::download::{DownloadConfig, DownloaderBuilder};
use std::path::PathBuf;
use tempfile::tempdir;

#[tokio::test]
async fn test_create_task() {
    let dir = tempdir().unwrap();
    
    let downloader = DownloaderBuilder::new()
        .download_dir(dir.path())
        .build()
        .unwrap();
    
    let config = DownloadConfig::new(
        "https://httpbin.org/bytes/1024",
        dir.path().join("test.bin")
    );
    
    let task_id = downloader.create_task(config).await.unwrap();
    assert!(!task_id.is_empty());
}

#[tokio::test]
async fn test_task_status() {
    let dir = tempdir().unwrap();
    let downloader = DownloaderBuilder::new()
        .download_dir(dir.path())
        .build()
        .unwrap();
    
    let config = DownloadConfig::new(
        "https://httpbin.org/bytes/1024",
        dir.path().join("test.bin")
    );
    
    let task_id = downloader.create_task(config).await.unwrap();
    let status = downloader.status(&task_id).await.unwrap();
    
    assert_eq!(status, TaskStatus::Pending);
}

#[tokio::test]
async fn test_download_and_pause() {
    let dir = tempdir().unwrap();
    let downloader = DownloaderBuilder::new()
        .download_dir(dir.path())
        .build()
        .unwrap();
    
    let config = DownloadConfig::new(
        "https://httpbin.org/bytes/10240",
        dir.path().join("test.bin")
    ).with_threads(2);
    
    let task_id = downloader.create_task(config).await.unwrap();
    
    // 开始下载
    downloader.start(&task_id, None).await.unwrap();
    
    // 等待一小段时间
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // 暂停
    downloader.pause(&task_id).await.unwrap();
    let status = downloader.status(&task_id).await.unwrap();
    assert_eq!(status, TaskStatus::Paused);
}
```

---

### 4. 断点续传测试

```rust
// tests/resume_test.rs

use turbo_downloader::resume::{ResumeManager, ResumeState};
use tempfile::tempdir;

#[test]
fn test_save_and_load_state() {
    let dir = tempdir().unwrap();
    let manager = ResumeManager::new(dir.path()).unwrap();
    
    let state = ResumeState::new(
        "test-task".to_string(),
        "https://example.com/file".to_string(),
        dir.path().join("test.bin"),
        1024,
        vec![
            ChunkRange::new(0, 511),
            ChunkRange::new(512, 1023),
        ],
    );
    
    manager.save(&state).unwrap();
    
    let loaded = manager.load("test-task").unwrap();
    assert!(loaded.is_some());
    
    let loaded = loaded.unwrap();
    assert_eq!(loaded.task_id, "test-task");
    assert_eq!(loaded.total_size, 1024);
    assert_eq!(loaded.chunks.len(), 2);
}

#[test]
fn test_state_update() {
    let mut state = ResumeState::new(
        "test-task".to_string(),
        "https://example.com/file".to_string(),
        PathBuf::from("test.bin"),
        100,
        vec![
            ChunkRange::new(0, 49),
            ChunkRange::new(50, 99),
        ],
    );
    
    // 更新第一个分片
    state.update_chunk(0, 50);
    assert_eq!(state.downloaded, 50);
    assert!(state.chunks[0].completed);
    assert!(!state.chunks[1].completed);
    
    // 更新第二个分片
    state.update_chunk(1, 50);
    assert_eq!(state.downloaded, 100);
    assert!(state.is_completed());
}

#[test]
fn test_delete_state() {
    let dir = tempdir().unwrap();
    let manager = ResumeManager::new(dir.path()).unwrap();
    
    let state = ResumeState::new(
        "test-task".to_string(),
        "https://example.com/file".to_string(),
        dir.path().join("test.bin"),
        1024,
        vec![ChunkRange::new(0, 1023)],
    );
    
    manager.save(&state).unwrap();
    assert!(manager.exists("test-task"));
    
    manager.delete("test-task").unwrap();
    assert!(!manager.exists("test-task"));
}
```

---

### 5. 进度追踪测试

```rust
// tests/progress_test.rs

use turbo_downloader::progress::{ProgressTracker, BasicProgressTracker};
use std::time::Duration;

#[test]
fn test_progress_calculation() {
    let tracker = BasicProgressTracker::new("test".to_string(), 1000);
    
    tracker.update(500, 100.0);
    let progress = tracker.progress();
    
    assert_eq!(progress.downloaded, 500);
    assert_eq!(progress.percentage, 50.0);
    assert_eq!(progress.speed, 100.0);
    assert_eq!(progress.eta, Some(Duration::from_secs(5)));
}

#[test]
fn test_progress_complete() {
    let tracker = BasicProgressTracker::new("test".to_string(), 1000);
    
    tracker.update(1000, 100.0);
    let progress = tracker.progress();
    
    assert!(progress.is_completed());
    assert_eq!(progress.percentage, 100.0);
}

#[test]
fn test_progress_callback() {
    use std::sync::{Arc, Mutex};
    
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();
    
    let mut tracker = BasicProgressTracker::new("test".to_string(), 1000);
    tracker.set_callback(
        Box::new(move |_| {
            *counter_clone.lock().unwrap() += 1;
        }),
        Duration::from_millis(0),
    );
    
    tracker.update(100, 100.0);
    tracker.update(200, 100.0);
    
    // 回调应该被触发
    assert!(*counter.lock().unwrap() > 0);
}
```

---

## 示例代码

### 基础下载示例

```rust
// examples/basic_download.rs

use turbo_downloader::{DownloaderBuilder, DownloadConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建下载器
    let downloader = DownloaderBuilder::new()
        .max_concurrent_tasks(3)
        .default_threads(4)
        .build()?;
    
    // 创建下载配置
    let config = DownloadConfig::new(
        "https://example.com/largefile.zip",
        PathBuf::from("./downloads/largefile.zip")
    )
    .with_threads(4)
    .with_header("Authorization", "Bearer token");
    
    // 创建任务
    let task_id = downloader.create_task(config).await?;
    println!("Created task: {}", task_id);
    
    // 开始下载
    downloader.start(&task_id, Some(Box::new(|progress| {
        println!(
            "Progress: {:.2}% - Speed: {:.2} KB/s",
            progress.percentage,
            progress.speed / 1024.0
        );
    }))).await?;
    
    println!("Download completed!");
    Ok(())
}
```

### 断点续传示例

```rust
// examples/resume_download.rs

use turbo_downloader::{DownloaderBuilder, DownloadConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = DownloaderBuilder::new()
        .download_dir("./downloads")
        .build()?;
    
    let config = DownloadConfig::new(
        "https://example.com/largefile.zip",
        PathBuf::from("./downloads/largefile.zip")
    );
    
    let task_id = downloader.create_task(config).await?;
    
    // 开始下载
    let downloader_clone = downloader.clone();
    let task_id_clone = task_id.clone();
    
    tokio::spawn(async move {
        // 模拟暂停
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        downloader_clone.pause(&task_id_clone).await.unwrap();
        println!("Paused!");
        
        // 模拟恢复
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        downloader_clone.resume(&task_id_clone).await.unwrap();
        println!("Resumed!");
    });
    
    downloader.start(&task_id, Some(Box::new(|progress| {
        println!("Progress: {:.2}%", progress.percentage);
    }))).await?;
    
    Ok(())
}
```

### 多任务管理示例

```rust
// examples/multi_task.rs

use turbo_downloader::{DownloaderBuilder, DownloadConfig, TaskStatus};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = DownloaderBuilder::new()
        .max_concurrent_tasks(3)
        .build()?;
    
    // 创建多个下载任务
    let urls = vec![
        "https://example.com/file1.zip",
        "https://example.com/file2.zip",
        "https://example.com/file3.zip",
    ];
    
    let mut task_ids = Vec::new();
    
    for (i, url) in urls.iter().enumerate() {
        let config = DownloadConfig::new(
            *url,
            PathBuf::from(format!("./downloads/file{}.zip", i + 1))
        );
        
        let task_id = downloader.create_task(config).await?;
        task_ids.push(task_id);
    }
    
    // 并发启动所有任务
    for task_id in &task_ids {
        downloader.start(task_id, None).await?;
    }
    
    // 监控所有任务状态
    loop {
        let tasks = downloader.list_tasks().await;
        let all_done = tasks.iter().all(|t| t.status.is_terminal());
        
        for task in &tasks {
            println!("Task {}: {:?} - {:.2}%", 
                task.id, task.status, task.progress.percentage);
        }
        
        if all_done {
            break;
        }
        
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    println!("All downloads completed!");
    Ok(())
}
```