# P1: turbo-downloader 数据结构设计

## 1. 核心数据类型

### 1.1 任务 ID

```rust
/// 任务唯一标识符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(u128);

impl TaskId {
    pub fn new() -> Self;
    pub fn from_u128(id: u128) -> Self;
    pub fn to_u128(self) -> u128;
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}
```

### 1.2 分片信息

```rust
/// 分片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    /// 分片 ID
    pub id: u32,
    /// 起始位置 (bytes)
    pub start: u64,
    /// 结束位置 (bytes)
    pub end: u64,
    /// 已下载大小 (bytes)
    pub downloaded: u64,
    /// 当前状态
    pub state: ChunkState,
    /// 临时文件路径
    pub temp_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChunkState {
    Pending,
    Downloading,
    Completed,
    Failed,
    Paused,
}
```

---

## 2. 任务状态机

### 2.1 任务状态

```rust
/// 下载任务状态
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskState {
    /// 等待中
    Pending,
    /// 下载中
    Downloading,
    /// 暂停
    Paused,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}
```

### 2.2 任务信息

```rust
/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    /// 任务 ID
    pub id: TaskId,
    /// 状态
    pub state: TaskState,
    /// URL
    pub url: String,
    /// 文件大小
    pub file_size: Option<u64>,
    /// 已下载
    pub downloaded: u64,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 开始时间
    pub started_at: Option<DateTime<Utc>>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
}
```

---

## 3. 下载进度

### 3.1 进度数据

```rust
/// 下载进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    /// 任务 ID
    pub task_id: TaskId,
    /// 总大小 (bytes)
    pub total: u64,
    /// 已下载 (bytes)
    pub downloaded: u64,
    /// 当前速度 (bytes/s)
    pub speed: u64,
    /// 平均速度 (bytes/s)
    pub avg_speed: u64,
    /// 预估剩余时间 (秒)
    pub eta: Option<u64>,
    /// 完成百分比 (0-100)
    pub percent: f64,
    /// 状态
    pub status: ProgressStatus,
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

### 3.2 速度样本

```rust
/// 速度计算样本
#[derive(Debug, Clone)]
pub struct SpeedSample {
    /// 时间戳
    pub timestamp: Instant,
    /// 已下载字节数
    pub downloaded: u64,
}

/// 速度计算器
pub struct SpeedCalculator {
    /// 样本窗口大小
    window_size: usize,
    /// 速度样本
    samples: Vec<SpeedSample>,
}
```

---

## 4. 断点续传

### 4.1 恢复状态

```rust
/// 恢复状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeState {
    /// 任务 ID
    pub task_id: TaskId,
    /// URL
    pub url: String,
    /// 文件大小
    pub file_size: u64,
    /// ETag
    pub etag: Option<String>,
    /// 已下载大小
    pub downloaded: u64,
    /// 分片状态
    pub chunks: Vec<ChunkResumeInfo>,
    /// 文件路径
    pub output_path: PathBuf,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 分片恢复信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResumeInfo {
    /// 分片 ID
    pub id: u32,
    /// 起始位置
    pub start: u64,
    /// 结束位置
    pub end: u64,
    /// 已下载
    pub downloaded: u64,
    /// 临时文件路径
    pub temp_path: PathBuf,
}
```

---

## 5. HTTP 响应

### 5.1 HEAD 响应

```rust
/// HEAD 请求响应
#[derive(Debug, Clone)]
pub struct HeadResponse {
    /// HTTP 状态码
    pub status: u16,
    /// 内容长度
    pub content_length: Option<u64>,
    /// 是否支持范围请求
    pub accept_ranges: Option<String>,
    /// ETag
    pub etag: Option<String>,
    /// 内容类型
    pub content_type: Option<String>,
    /// 最后修改时间
    pub last_modified: Option<DateTime<Utc>>,
    /// 接受的编码
    pub accept_encoding: Option<String>,
    /// 内容编码
    pub content_encoding: Option<String>,
}
```

### 5.2 分片数据

```rust
/// 分片数据
#[derive(Debug)]
pub struct ChunkData {
    /// 分片 ID
    pub chunk_id: u32,
    /// 数据
    pub data: Bytes,
    /// 范围
    pub range: Range<u64>,
}
```

---

## 6. 下载配置

### 6.1 用户配置

```rust
/// 下载配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    /// 任务 ID
    pub id: TaskId,
    /// 下载 URL
    pub url: String,
    /// 输出路径
    pub output_path: PathBuf,
    /// 分片数量 (0 = 自动)
    pub threads: u32,
    /// 分片大小 (0 = 自动)
    pub chunk_size: u64,
    /// 支持断点续传
    pub resume_support: bool,
    /// User-Agent
    pub user_agent: Option<String>,
    /// 自定义请求头
    pub headers: HashMap<String, String>,
    /// 速度限制 (0 = 无限制)
    pub speed_limit: u64,
    /// 重试次数
    pub retry_count: u32,
    /// 超时时间 (秒)
    pub timeout: u64,
    /// 是否验证 ETag
    pub validate_etag: bool,
}
```

### 6.2 默认配置

```rust
impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            id: TaskId::new(),
            url: String::new(),
            output_path: PathBuf::new(),
            threads: 0,  // 自动
            chunk_size: 0,  // 自动
            resume_support: true,
            user_agent: Some("TurboDownload/1.0".to_string()),
            headers: HashMap::new(),
            speed_limit: 0,
            retry_count: 3,
            timeout: 300,
            validate_etag: true,
        }
    }
}
```

---

## 7. 下载结果

### 7.1 结果数据

```rust
/// 下载结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    /// 任务 ID
    pub task_id: TaskId,
    /// 文件路径
    pub file_path: PathBuf,
    /// 文件大小 (bytes)
    pub file_size: u64,
    /// 总耗时 (毫秒)
    pub duration_ms: u64,
    /// 平均速度 (bytes/s)
    pub avg_speed: u64,
    /// 分片数
    pub chunk_count: u32,
    /// 重试次数
    pub retry_count: u32,
}
```

---

## 8. 错误信息

### 8.1 错误详情

```rust
/// 下载错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadError {
    /// 错误代码
    pub code: ErrorCode,
    /// 错误消息
    pub message: String,
    /// 原始错误
    pub source: Option<String>,
    /// 任务 ID
    pub task_id: Option<TaskId>,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ErrorCode {
    NetworkError,
    HttpError,
    IoError,
    TaskNotFound,
    TaskAlreadyExists,
    TaskPaused,
    TaskCompleted,
    RangeNotSupported,
    ValidationFailed,
    Timeout,
    Cancelled,
}
```

---

## 9. 配置存储格式

### 9.1 状态文件格式

```json
{
  "task_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "url": "https://example.com/file.zip",
  "file_size": 104857600,
  "etag": "\"abc123\"",
  "downloaded": 52428800,
  "chunks": [
    {
      "id": 0,
      "start": 0,
      "end": 26214399,
      "downloaded": 26214399,
      "temp_path": "/tmp/td_chunk_0.tmp"
    },
    {
      "id": 1,
      "start": 26214400,
      "end": 52428799,
      "downloaded": 26214400,
      "temp_path": "/tmp/td_chunk_1.tmp"
    }
  ],
  "output_path": "/downloads/file.zip",
  "created_at": "2026-03-26T10:00:00Z",
  "updated_at": "2026-03-26T10:05:00Z"
}
```

---

## 10. 内存布局

### 10.1 进度追踪内存

```
Progress Tracker (per task):
├── total: u64 (8 bytes)
├── downloaded: AtomicU64 (8 bytes)
├── start_time: Instant (16 bytes)
└── SpeedCalculator
    ├── window_size: usize (8 bytes)
    └── samples: Vec<SpeedSample> (24 bytes)
        └── [SpeedSample; N]
            ├── timestamp: Instant (16 bytes)
            └── downloaded: u64 (8 bytes)

Estimated: ~200 bytes per task
```

### 10.2 分片内存

```
Per Chunk Worker:
├── chunk: ChunkInfo (栈, ~80 bytes)
├── buffer: BytesMut (堆, 64KB default)
└── temp file: File (OS handles)
```

---

## 11. 数据流图

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  User Code  │────▶│  Downloader │────▶│  HTTP Client│
└──────────────┘     └──────────────┘     └──────────────┘
                           │                    │
                           ▼                    ▼
                    ┌──────────────┐     ┌──────────────┐
                    │    Task      │     │  Response   │
                    │   Manager    │     │   (HEAD)    │
                    └──────────────┘     └──────────────┘
                           │                    │
                           ▼                    ▼
                    ┌──────────────┐     ┌──────────────┐
                    │    Chunk     │     │ Chunk Info  │
                    │  Strategy    │     │  (ranges)   │
                    └──────────────┘     └──────────────┘
                           │                    │
          ┌────────────────┼────────────────┐    │
          ▼                ▼                ▼    ▼
   ┌────────────┐   ┌────────────┐   ┌────────────┐
   │  Worker 1  │   │  Worker 2  │   │  Worker N  │
   │ (async)    │   │ (async)    │   │ (async)    │
   └────────────┘   └────────────┘   └────────────┘
          │                │                │
          └────────────────┼────────────────┘
                           ▼
                    ┌──────────────┐
                    │   Progress   │
                    │   Tracker    │
                    └──────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │   Callback   │
                    │  (user fn)   │
                    └──────────────┘
```

---

*数据结构设计版本: v0.1.0*
*设计日期: 2026-03-26*