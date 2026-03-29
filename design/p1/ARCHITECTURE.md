# P1: turbo-downloader 技术架构设计

## 1. 设计目标

高性能、多线程下载引擎，支持断点续传、进度回调、速度计算等核心功能。

### 非功能性需求

| 需求 | 目标值 |
|------|--------|
| 并发下载 | 支持 1-32 线程 |
| 断点续传 | 支持 |
| 内存效率 | 单文件 < 10MB 内存 |
| 错误恢复 | 自动重试 3 次 |
| 进度更新 | 100ms 粒度 |

---

## 2. 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                    TurboDownloader                        │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
│  │   任务管理   │  │  进度追踪   │  │  状态持久化 │      │
│  │  Download   │  │  Progress   │  │   Resume    │
│  │   Manager   │  │   Tracker   │  │   Module    │
│  └─────────────┘  └─────────────┘  └─────────────┘      │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐   │
│  │              下载引擎 (Download Engine)           │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌──────────┐  │   │
│  │  │ HTTP Client │  │ Chunk Worker│  │ 策略调度 │  │   │
│  │  │   (reqwest) │  │  (多线程)   │  │ (Strategy)│  │   │
│  │  └─────────────┘  └─────────────┘  └──────────┘  │   │
│  └─────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐   │
│  │                  错误处理层                       │   │
│  │              (Error Module)                       │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

## 3. 模块划分

### 3.1 核心模块

| 模块 | 职责 | 关键组件 |
|------|------|----------|
| `http` | HTTP/HTTPS 请求 | Client, Response |
| `chunk` | 分片下载管理 | Strategy, Worker |
| `download` | 任务调度 | Task, Manager |
| `progress` | 进度追踪 | Tracker, Speed |
| `resume` | 断点续传 | State, Recovery |
| `error` | 错误处理 | Error types |

### 3.2 模块关系

```
Downloader
    │
    ├── http::Client ──────────→ HTTP 请求
    │       │
    │       └──→ chunk::Strategy → 分片策略计算
    │               │
    │               └──→ chunk::Worker → 并发下载
    │                       │
    │                       └──→ progress::Tracker → 进度更新
    │
    ├── download::Task ──────→ 下载任务
    │       │
    │       └──→ resume::Recovery → 断点恢复
    │
    └── download::Manager ───→ 任务管理
            │
            └──→ progress::Speed → 速度计算
```

---

## 4. 数据流设计

### 4.1 下载流程

```
1. create_task(config)
       ↓
2. 获取文件大小 (HEAD 请求)
       ↓
3. 计算分片策略 (ChunkStrategy)
       ↓
4. 启动 N 个 ChunkWorker 并发下载
       ↓
5. 每个 Worker:
   - 下载分片数据
   - 写入临时文件
   - 报告进度
       ↓
6. 合并临时文件 → 目标文件
       ↓
7. 清理临时文件 → 完成
```

### 4.2 进度回调流程

```
ChunkWorker ──→ ProgressUpdate ──→ Tracker ──→ Callback
                      │                  │
                      ↓                  ↓
                 已下载字节           计算速度
                 当前分片            预估时间
```

---

## 5. 并发模型

### 5.1 Tokio 运行时

- 使用 `tokio` 异步运行时
- 每个分片一个 Task
- 使用 `tokio::sync` 进行进度同步

### 5.2 线程池

```
主线程 (Tokio Main)
    │
    ├── HTTP Client (reqwest)
    │
    ├── Chunk Worker 1 ──→ async task
    ├── Chunk Worker 2 ──→ async task
    ├── Chunk Worker 3 ──→ async task
    └── ...
```

---

## 6. 模块详细设计

### 6.1 http 模块

```rust
pub mod http {
    pub struct Client {
        client: reqwest::Client,
    }
    
    impl Client {
        pub async fn head(&self, url: &str) -> Result<HeadResponse>;
        pub async fn get_range(&self, url: &str, range: Range<u64>) -> Result<Bytes>;
    }
}
```

### 6.2 chunk 模块

```rust
pub mod chunk {
    pub struct Strategy {
        pub chunks: Vec<Chunk>,
    }
    
    pub struct Chunk {
        pub id: u32,
        pub start: u64,
        pub end: u64,
        pub downloaded: u64,
    }
    
    pub struct Worker {
        // 分片下载工作器
    }
}
```

### 6.3 download 模块

```rust
pub mod download {
    pub struct Manager {
        tasks: HashMap<TaskId, Task>,
    }
    
    pub struct Task {
        pub id: TaskId,
        pub config: DownloadConfig,
        pub status: TaskStatus,
    }
}
```

### 6.4 progress 模块

```rust
pub mod progress {
    pub struct Tracker {
        total: u64,
        downloaded: u64,
        speed_calc: SpeedCalculator,
    }
}
```

### 6.5 resume 模块

```rust
pub mod resume {
    pub struct State {
        pub task_id: TaskId,
        pub url: String,
        pub chunks: Vec<ChunkState>,
        pub created_at: DateTime<Utc>,
    }
}
```

---

## 7. 错误处理

### 7.1 错误分类

| 错误类型 | 说明 | 处理策略 |
|----------|------|----------|
| NetworkError | 网络错误 | 重试 |
| HttpError | HTTP 错误 | 重试/终止 |
| IoError | 文件 IO 错误 | 重试/终止 |
| ValidateError | 校验失败 | 重试/终止 |

### 7.2 重试策略

- 最大重试次数: 3
- 重试间隔: 指数退避 (1s, 2s, 4s)
- 断网自动暂停，恢复后继续

---

## 8. 性能优化

### 8.1 内存优化

- 流式写入磁盘，避免全量内存
- 分片缓冲区: 64KB
- 进度快照: 异步非阻塞

### 8.2 网络优化

- 连接复用 (reqwest default)
- HTTP/2 多路复用
- Gzip 解压支持

### 8.3 并发控制

- 可配置最大并发数
- 动态调整分片数
- 背压控制

---

## 9. 目录结构

```
crates/turbo-downloader/
├── src/
│   ├── lib.rs              # 模块入口
│   ├── http/
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   └── response.rs
│   ├── chunk/
│   │   ├── mod.rs
│   │   ├── strategy.rs
│   │   └── worker.rs
│   ├── download/
│   │   ├── mod.rs
│   │   ├── task.rs
│   │   └── manager.rs
│   ├── progress/
│   │   ├── mod.rs
│   │   ├── tracker.rs
│   │   └── speed.rs
│   ├── resume/
│   │   ├── mod.rs
│   │   ├── state.rs
│   │   └── recovery.rs
│   └── error/
│       ├── mod.rs
│       └── types.rs
├── tests/
└── examples/
```

---

## 10. API 概览

```rust
// 核心 API
pub fn new() -> DownloaderBuilder;
pub async fn create_task(&self, config: DownloadConfig) -> TaskId;
pub async fn start(&self, task_id: &TaskId, callback: ProgressCallback);
pub async fn pause(&self, task_id: &TaskId);
pub async fn resume(&self, task_id: &TaskId);
pub async fn cancel(&self, task_id: &TaskId);
pub fn get_progress(&self, task_id: &TaskId) -> DownloadProgress;
```

---

*设计完成日期: 2026-03-26*
*版本: v0.1.0*