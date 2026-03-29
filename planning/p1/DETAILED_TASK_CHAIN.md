# P1: turbo-downloader 详细任务链规划

> 按照乐高式开发模式：项目框架 → 任务链 → 子任务 → 步骤 → 验证

---

## 任务链总览

```
P1: turbo-downloader
├── T1.1 项目初始化 (2h)
│   ├── T1.1.1 创建项目结构 (0.5h)
│   ├── T1.1.2 配置依赖 (0.5h)
│   ├── T1.1.3 创建测试框架 (0.5h)
│   └── T1.1.4 配置开发工具 (0.5h)
│
├── T1.2 错误处理模块 (2h) [可与T1.3并行]
│   ├── T1.2.1 定义错误类型 (1h)
│   └── T1.2.2 实现错误转换 (1h)
│
├── T1.3 HTTP 客户端封装 (3h)
│   ├── T1.3.1 Client 结构定义 (1h)
│   ├── T1.3.2 HEAD 请求实现 (1h)
│   └── T1.3.3 Range 请求实现 (1h)
│
├── T1.4 分片策略模块 (3h)
│   ├── T1.4.1 Chunk 数据结构 (1h)
│   ├── T1.4.2 策略计算算法 (1h)
│   └── T1.4.3 策略测试 (1h)
│
├── T1.5 多线程下载核心 (6h)
│   ├── T1.5.1 Worker 结构设计 (1h)
│   ├── T1.5.2 单分片下载逻辑 (2h)
│   ├── T1.5.3 并发调度器 (2h)
│   └── T1.5.4 文件合并 (1h)
│
├── T1.6 进度追踪模块 (3h) [可与T1.5并行]
│   ├── T1.6.1 Tracker 实现 (1h)
│   ├── T1.6.2 速度计算 (1h)
│   └── T1.6.3 回调机制 (1h)
│
├── T1.7 断点续传模块 (4h)
│   ├── T1.7.1 状态持久化 (1.5h)
│   ├── T1.7.2 恢复逻辑 (1.5h)
│   └── T1.7.3 状态验证 (1h)
│
├── T1.8 任务管理模块 (4h)
│   ├── T1.8.1 Task 结构 (1h)
│   ├── T1.8.2 Manager 实现 (2h)
│   └── T1.8.3 Builder 模式 (1h)
│
├── T1.9 集成测试 (4h)
│   ├── T1.9.1 Mock 服务器 (1h)
│   ├── T1.9.2 端到端测试 (2h)
│   └── T1.9.3 性能测试 (1h)
│
└── T1.10 文档与示例 (2h)
    ├── T1.10.1 API 文档 (1h)
    └── T1.10.2 使用示例 (1h)
```

**总工时**: 33 小时

**并行关系**:
- T1.2 和 T1.3 可以并行
- T1.6 可以在 T1.5 进行时并行开发

---

## T1.1 项目初始化

### T1.1.1 创建项目结构

**时间**: 0.5h  
**依赖**: 无  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 创建 crate 目录 | `crates/turbo-downloader/` | 目录存在 |
| 2 | 初始化 Cargo 项目 | `cargo init --lib` | `Cargo.toml` 存在 |
| 3 | 创建模块目录 | `src/{http,chunk,download,progress,resume,error}/` | 目录存在 |
| 4 | 创建模块文件 | `src/*/mod.rs` | 文件存在 |
| 5 | 编写 lib.rs 入口 | `src/lib.rs` | `cargo check` 通过 |

#### 输出文件

**src/lib.rs**:
```rust
//! Turbo Downloader - High-performance multi-threaded download engine

pub mod error;
pub mod http;
pub mod chunk;
pub mod download;
pub mod progress;
pub mod resume;

// 重新导出主要类型
pub use error::{DownloadError, Result};
pub use download::{DownloadConfig, DownloadResult, Downloader, DownloaderBuilder};
pub use progress::{DownloadProgress, ProgressCallback};
```

#### 验收清单
- [ ] `cargo check` 无错误
- [ ] 目录结构符合设计
- [ ] 模块导入正常

---

### T1.1.2 配置依赖

**时间**: 0.5h  
**依赖**: T1.1.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 编辑 Cargo.toml | `Cargo.toml` | 依赖配置完整 |
| 2 | 添加 workspace 引用 | 根 `Cargo.toml` | 包含 turbo-downloader |
| 3 | 下载依赖 | `cargo fetch` | 依赖下载成功 |

#### 输出文件

**Cargo.toml**:
```toml
[package]
name = "turbo-downloader"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
reqwest = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
sha2 = "0.10"
futures = "0.3"
bytes = "1.5"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
parking_lot = "0.12"

[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.5"
tempfile = "3.8"
```

#### 验收清单
- [ ] `cargo fetch` 成功
- [ ] `cargo build` 成功
- [ ] 无版本冲突

---

### T1.1.3 创建测试框架

**时间**: 0.5h  
**依赖**: T1.1.1  
**并行**: 可与 T1.1.2 并行

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 创建测试目录 | `tests/` | 目录存在 |
| 2 | 创建测试模块 | `tests/mod.rs` | 文件存在 |
| 3 | 创建测试工具 | `tests/common/mod.rs` | Mock 工具可用 |
| 4 | 创建示例测试 | `tests/smoke_test.rs` | 测试通过 |

#### 输出文件

**tests/common/mod.rs**:
```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

pub async fn start_mock_server() -> MockServer {
    MockServer::start().await
}

pub fn mock_file_response(server: &MockServer, path: &str, size: u64) {
    // TODO: 实现 mock 配置
}
```

#### 验收清单
- [ ] `cargo test` 可运行
- [ ] Mock 服务器可启动

---

### T1.1.4 配置开发工具

**时间**: 0.5h  
**依赖**: T1.1.1  
**并行**: 可与 T1.1.2/T1.1.3 并行

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 创建 rustfmt 配置 | `rustfmt.toml` | 格式化规则生效 |
| 2 | 创建 clippy 配置 | `clippy.toml` | Lint 规则生效 |
| 3 | 创建 VSCode 配置 | `.vscode/settings.json` | IDE 配置正确 |

#### 输出文件

**rustfmt.toml**:
```toml
edition = "2021"
max_width = 100
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

#### 验收清单
- [ ] `cargo fmt -- --check` 通过
- [ ] `cargo clippy` 无警告

---

## T1.2 错误处理模块

### T1.2.1 定义错误类型

**时间**: 1h  
**依赖**: T1.1.1  
**并行**: 可与 T1.3 并行

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义错误枚举 | `src/error/types.rs` | `DownloadError` | 枚举完整 |
| 2 | 实现错误 Display | `src/error/types.rs` | `impl Display` | 格式化输出 |
| 3 | 定义 Result 类型 | `src/error/mod.rs` | `Result<T>` | 类型可用 |

#### 输出文件

**src/error/types.rs**:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("HTTP 错误 {0}: {1}")]
    Http(u16, String),
    
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("任务不存在: {0}")]
    TaskNotFound(String),
    
    #[error("不支持范围请求")]
    RangeNotSupported,
    
    #[error("校验失败: {0}")]
    ValidationFailed(String),
    
    #[error("超时")]
    Timeout,
    
    #[error("已取消")]
    Cancelled,
}
```

**src/error/mod.rs**:
```rust
mod types;

pub use types::DownloadError;

pub type Result<T> = std::result::Result<T, DownloadError>;
```

#### 验收清单
- [ ] `cargo check` 通过
- [ ] 错误类型覆盖所有场景
- [ ] `thiserror` 宏正常工作

---

### T1.2.2 实现错误转换

**时间**: 1h  
**依赖**: T1.2.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | reqwest 错误转换 | `src/error/types.rs` | `#[from]` | 自动转换 |
| 2 | IO 错误转换 | `src/error/types.rs` | `#[from]` | 自动转换 |
| 3 | 自定义错误方法 | `src/error/types.rs` | `is_retryable()` | 方法可用 |

#### 输出代码

```rust
impl DownloadError {
    /// 判断是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            DownloadError::Network(_) | 
            DownloadError::Timeout |
            DownloadError::Http(500..=599, _)
        )
    }
    
    /// 获取错误代码
    pub fn code(&self) -> &'static str {
        match self {
            DownloadError::Network(_) => "NETWORK",
            DownloadError::Http(_, _) => "HTTP",
            DownloadError::Io(_) => "IO",
            DownloadError::TaskNotFound(_) => "TASK_NOT_FOUND",
            DownloadError::RangeNotSupported => "RANGE_NOT_SUPPORTED",
            DownloadError::ValidationFailed(_) => "VALIDATION",
            DownloadError::Timeout => "TIMEOUT",
            DownloadError::Cancelled => "CANCELLED",
        }
    }
}
```

#### 验收清单
- [ ] 错误转换编译通过
- [ ] `is_retryable()` 逻辑正确
- [ ] 单元测试通过

---

## T1.3 HTTP 客户端封装

### T1.3.1 Client 结构定义

**时间**: 1h  
**依赖**: T1.1.2, T1.2.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Client 结构 | `src/http/client.rs` | `Client` | 结构定义 |
| 2 | 定义 ClientBuilder | `src/http/client.rs` | `ClientBuilder` | Builder 模式 |
| 3 | 实现 Default | `src/http/client.rs` | `impl Default` | 默认配置 |

#### 输出文件

**src/http/client.rs**:
```rust
use reqwest::Client as ReqwestClient;
use crate::Result;

/// HTTP 客户端配置
pub struct ClientConfig {
    pub timeout: std::time::Duration,
    pub user_agent: String,
    pub max_connections: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: std::time::Duration::from_secs(300),
            user_agent: "TurboDownload/1.0".to_string(),
            max_connections: 32,
        }
    }
}

/// HTTP 客户端
pub struct Client {
    inner: ReqwestClient,
    config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let inner = ReqwestClient::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .pool_max_idle_per_host(config.max_connections)
            .build()?;
        
        Ok(Self { inner, config })
    }
    
    pub fn with_defaults() -> Result<Self> {
        Self::new(ClientConfig::default())
    }
}
```

#### 验收清单
- [ ] `cargo check` 通过
- [ ] Client 可创建
- [ ] 默认配置合理

---

### T1.3.2 HEAD 请求实现

**时间**: 1h  
**依赖**: T1.3.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 HeadResponse | `src/http/response.rs` | `HeadResponse` | 结构完整 |
| 2 | 实现 head 方法 | `src/http/client.rs` | `head(&self, url)` | 方法可用 |
| 3 | 解析响应头 | `src/http/response.rs` | 解析函数 | 解析正确 |

#### 输出文件

**src/http/response.rs**:
```rust
use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;

/// HEAD 请求响应
#[derive(Debug, Clone)]
pub struct HeadResponse {
    pub status: u16,
    pub content_length: Option<u64>,
    pub accept_ranges: Option<String>,
    pub etag: Option<String>,
    pub content_type: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
}

impl HeadResponse {
    pub fn from_headers(status: u16, headers: &HeaderMap) -> Self {
        Self {
            status,
            content_length: headers
                .get("content-length")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
            accept_ranges: headers
                .get("accept-ranges")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            etag: headers
                .get("etag")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            content_type: headers
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            last_modified: headers
                .get("last-modified")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| chrono::DateTime::parse_from_rfc2822(v).ok())
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }
    
    /// 是否支持范围请求
    pub fn supports_range(&self) -> bool {
        self.accept_ranges.as_deref() == Some("bytes")
    }
}
```

#### 验收清单
- [ ] HEAD 请求可发送
- [ ] 响应头正确解析
- [ ] `supports_range()` 正确判断

---

### T1.3.3 Range 请求实现

**时间**: 1h  
**依赖**: T1.3.2  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 get_range | `src/http/client.rs` | `get_range()` | 方法可用 |
| 2 | 处理 Range 头 | `src/http/client.rs` | Range 格式 | 格式正确 |
| 3 | 处理响应体 | `src/http/client.rs` | Bytes 处理 | 正确处理 |

#### 输出代码

```rust
use bytes::Bytes;
use std::ops::Range;

impl Client {
    /// 获取指定范围的数据
    pub async fn get_range(
        &self,
        url: &str,
        range: Range<u64>,
    ) -> Result<Bytes> {
        let range_header = format!("bytes={}-{}", range.start, range.end - 1);
        
        let response = self.inner
            .get(url)
            .header("Range", range_header)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() && status.as_u16() != 206 {
            return Err(DownloadError::Http(
                status.as_u16(),
                status.to_string()
            ));
        }
        
        let bytes = response.bytes().await?;
        Ok(bytes)
    }
}
```

#### 验收清单
- [ ] Range 请求格式正确
- [ ] 响应正确处理
- [ ] 错误情况覆盖

---

## T1.4 分片策略模块

### T1.4.1 Chunk 数据结构

**时间**: 1h  
**依赖**: T1.1.1  
**并行**: 可与 T1.3 并行

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Chunk | `src/chunk/strategy.rs` | `Chunk` | 结构定义 |
| 2 | 定义 ChunkState | `src/chunk/strategy.rs` | `ChunkState` | 枚举定义 |
| 3 | 实现序列化 | `src/chunk/strategy.rs` | `Serialize/Deserialize` | 可序列化 |

#### 输出文件

**src/chunk/strategy.rs**:
```rust
use serde::{Deserialize, Serialize};

/// 分片状态
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChunkState {
    Pending,
    Downloading,
    Completed,
    Failed,
}

/// 分片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub state: ChunkState,
}

impl Chunk {
    pub fn new(id: u32, start: u64, end: u64) -> Self {
        Self {
            id,
            start,
            end,
            downloaded: 0,
            state: ChunkState::Pending,
        }
    }
    
    pub fn size(&self) -> u64 {
        self.end - self.start
    }
    
    pub fn remaining(&self) -> u64 {
        self.size() - self.downloaded
    }
    
    pub fn is_complete(&self) -> bool {
        self.downloaded >= self.size()
    }
}
```

#### 验收清单
- [ ] 结构定义完整
- [ ] 方法实现正确
- [ ] 序列化正常工作

---

### T1.4.2 策略计算算法

**时间**: 1h  
**依赖**: T1.4.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 Strategy | `src/chunk/strategy.rs` | `Strategy` | 结构定义 |
| 2 | 实现计算方法 | `src/chunk/strategy.rs` | `calculate()` | 算法正确 |
| 3 | 边界处理 | `src/chunk/strategy.rs` | 边界情况 | 处理完善 |

#### 输出代码

```rust
/// 分片策略
pub struct Strategy {
    pub chunks: Vec<Chunk>,
}

impl Strategy {
    /// 计算分片策略
    ///
    /// # Arguments
    /// * `file_size` - 文件总大小
    /// * `thread_count` - 线程数 (0 = 自动)
    /// * `min_chunk_size` - 最小分片大小
    pub fn calculate(
        file_size: u64,
        thread_count: u32,
        min_chunk_size: u64,
    ) -> Self {
        // 自动计算线程数
        let threads = if thread_count == 0 {
            Self::auto_thread_count(file_size)
        } else {
            thread_count
        };
        
        let chunk_size = (file_size / threads as u64).max(min_chunk_size);
        let actual_threads = ((file_size + chunk_size - 1) / chunk_size) as u32;
        
        let mut chunks = Vec::with_capacity(actual_threads as usize);
        let mut start = 0u64;
        
        for id in 0..actual_threads {
            let end = (start + chunk_size).min(file_size);
            chunks.push(Chunk::new(id, start, end));
            start = end;
        }
        
        Self { chunks }
    }
    
    fn auto_thread_count(file_size: u64) -> u32 {
        match file_size {
            0..=10_000_000 => 2,        // < 10MB
            10_000_001..=100_000_000 => 4, // < 100MB
            100_000_001..=1_000_000_000 => 8, // < 1GB
            _ => 16, // >= 1GB
        }
    }
}
```

#### 验收清单
- [ ] 分片计算正确
- [ ] 自动线程数合理
- [ ] 边界情况处理

---

### T1.4.3 策略测试

**时间**: 1h  
**依赖**: T1.4.2  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 基础测试 | `tests/chunk_test.rs` | 测试用例 | 测试通过 |
| 2 | 边界测试 | `tests/chunk_test.rs` | 边界情况 | 覆盖完整 |
| 3 | 性能测试 | `benches/` | Benchmark | 性能达标 |

#### 测试用例

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_small_file() {
        let strategy = Strategy::calculate(5_000_000, 0, 1_000_000);
        assert_eq!(strategy.chunks.len(), 2);
    }
    
    #[test]
    fn test_large_file() {
        let strategy = Strategy::calculate(500_000_000, 0, 1_000_000);
        assert_eq!(strategy.chunks.len(), 8);
    }
    
    #[test]
    fn test_chunk_boundaries() {
        let strategy = Strategy::calculate(100, 3, 10);
        let total: u64 = strategy.chunks.iter().map(|c| c.size()).sum();
        assert_eq!(total, 100);
    }
}
```

#### 验收清单
- [ ] 所有测试通过
- [ ] 边界情况覆盖
- [ ] 代码覆盖率 > 80%

---

## T1.5 多线程下载核心

### T1.5.1 Worker 结构设计

**时间**: 1h  
**依赖**: T1.3, T1.4  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Worker | `src/chunk/worker.rs` | `Worker` | 结构定义 |
| 2 | 定义进度消息 | `src/chunk/worker.rs` | `ChunkProgress` | 消息定义 |
| 3 | 实现 Clone | `src/chunk/worker.rs` | `impl Clone` | 可克隆 |

#### 输出文件

**src/chunk/worker.rs**:
```rust
use crate::http::Client;
use crate::error::Result;
use super::Chunk;
use bytes::Bytes;
use std::path::Path;

/// 分片进度消息
#[derive(Debug, Clone)]
pub struct ChunkProgress {
    pub chunk_id: u32,
    pub downloaded: u64,
    pub total: u64,
}

/// 分片下载工作器
pub struct Worker {
    chunk: Chunk,
    url: String,
    client: Client,
    temp_path: std::path::PathBuf,
}

impl Worker {
    pub fn new(
        chunk: Chunk,
        url: String,
        client: Client,
        temp_dir: &Path,
    ) -> Self {
        let temp_path = temp_dir.join(format!("chunk_{}.tmp", chunk.id));
        Self {
            chunk,
            url,
            client,
            temp_path,
        }
    }
    
    pub fn chunk_id(&self) -> u32 {
        self.chunk.id
    }
}
```

#### 验收清单
- [ ] 结构定义完整
- [ ] 字段合理
- [ ] 构造函数正常

---

### T1.5.2 单分片下载逻辑

**时间**: 2h  
**依赖**: T1.5.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现下载方法 | `src/chunk/worker.rs` | `download()` | 方法可用 |
| 2 | 实现文件写入 | `src/chunk/worker.rs` | 写入逻辑 | 写入正确 |
| 3 | 实现进度报告 | `src/chunk/worker.rs` | 进度发送 | 进度更新 |

#### 输出代码

```rust
use tokio::sync::mpsc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

impl Worker {
    /// 执行分片下载
    pub async fn download(
        &mut self,
        progress_tx: mpsc::Sender<ChunkProgress>,
    ) -> Result<()> {
        let mut file = File::create(&self.temp_path).await?;
        let mut downloaded = self.chunk.downloaded;
        
        while downloaded < self.chunk.size() {
            let start = self.chunk.start + downloaded;
            let end = (start + 64 * 1024).min(self.chunk.end); // 64KB chunks
            let range = start..end;
            
            let bytes = self.client.get_range(&self.url, range).await?;
            file.write_all(&bytes).await?;
            
            downloaded += bytes.len() as u64;
            self.chunk.downloaded = downloaded;
            
            // 发送进度
            let _ = progress_tx.send(ChunkProgress {
                chunk_id: self.chunk.id,
                downloaded,
                total: self.chunk.size(),
            }).await;
        }
        
        file.flush().await?;
        Ok(())
    }
    
    pub fn temp_path(&self) -> &Path {
        &self.temp_path
    }
}
```

#### 验收清单
- [ ] 下载逻辑正确
- [ ] 文件写入正确
- [ ] 进度发送正常

---

### T1.5.3 并发调度器

**时间**: 2h  
**依赖**: T1.5.2  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义调度器 | `src/download/manager.rs` | `Scheduler` | 结构定义 |
| 2 | 实现并发控制 | `src/download/manager.rs` | 并发逻辑 | 控制正确 |
| 3 | 实现任务收集 | `src/download/manager.rs` | JoinSet | 收集正常 |

#### 输出代码

```rust
use tokio::task::JoinSet;
use tokio::sync::mpsc;

pub struct Scheduler {
    max_concurrent: usize,
}

impl Scheduler {
    pub fn new(max_concurrent: usize) -> Self {
        Self { max_concurrent }
    }
    
    /// 并发执行所有 Worker
    pub async fn run(
        &self,
        workers: Vec<Worker>,
        progress_tx: mpsc::Sender<ChunkProgress>,
    ) -> Result<Vec<std::path::PathBuf>> {
        let mut join_set = JoinSet::new();
        let mut temp_paths = Vec::new();
        
        // 限制并发数
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.max_concurrent));
        
        for mut worker in workers {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let tx = progress_tx.clone();
            
            temp_paths.push(worker.temp_path().to_owned());
            
            join_set.spawn(async move {
                let result = worker.download(tx).await;
                drop(permit);
                result
            });
        }
        
        // 等待所有任务完成
        while let Some(result) = join_set.join_next().await {
            result??; // 传播错误
        }
        
        Ok(temp_paths)
    }
}
```

#### 验收清单
- [ ] 并发控制正确
- [ ] 任务全部完成
- [ ] 错误传播正确

---

### T1.5.4 文件合并

**时间**: 1h  
**依赖**: T1.5.3  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现合并函数 | `src/download/task.rs` | `merge_files()` | 合并正确 |
| 2 | 实现清理函数 | `src/download/task.rs` | `cleanup()` | 清理正常 |
| 3 | 处理错误情况 | `src/download/task.rs` | 错误处理 | 处理完善 |

#### 输出代码

```rust
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// 合并临时文件
pub async fn merge_files(
    temp_paths: &[std::path::PathBuf],
    output_path: &Path,
) -> Result<()> {
    let mut output = fs::File::create(output_path).await?;
    
    for temp_path in temp_paths {
        let mut input = fs::File::open(temp_path).await?;
        let mut buffer = vec![0u8; 64 * 1024];
        
        loop {
            let n = input.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            output.write_all(&buffer[..n]).await?;
        }
    }
    
    output.flush().await?;
    Ok(())
}

/// 清理临时文件
pub async fn cleanup(temp_paths: &[std::path::PathBuf]) {
    for path in temp_paths {
        let _ = fs::remove_file(path).await;
    }
}
```

#### 验收清单
- [ ] 合并结果正确
- [ ] 清理彻底
- [ ] 大文件测试通过

---

## T1.6 进度追踪模块

### T1.6.1 Tracker 实现

**时间**: 1h  
**依赖**: T1.1.1  
**并行**: 可与 T1.5 并行

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Tracker | `src/progress/tracker.rs` | `Tracker` | 结构定义 |
| 2 | 实现更新方法 | `src/progress/tracker.rs` | `update()` | 更新正确 |
| 3 | 实现查询方法 | `src/progress/tracker.rs` | `get_progress()` | 查询正确 |

#### 输出文件

**src/progress/tracker.rs**:
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use super::speed::SpeedCalculator;
use crate::progress::DownloadProgress;

/// 进度追踪器
pub struct Tracker {
    total: u64,
    downloaded: AtomicU64,
    start_time: Instant,
    speed_calc: SpeedCalculator,
}

impl Tracker {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            downloaded: AtomicU64::new(0),
            start_time: Instant::now(),
            speed_calc: SpeedCalculator::new(10),
        }
    }
    
    pub fn update(&self, chunk_id: u32, downloaded: u64) {
        // 原子更新
        self.downloaded.fetch_add(downloaded, Ordering::Relaxed);
        self.speed_calc.add_sample(downloaded);
    }
    
    pub fn get_progress(&self) -> DownloadProgress {
        let downloaded = self.downloaded.load(Ordering::Relaxed);
        let speed = self.speed_calc.get_speed();
        let elapsed = self.start_time.elapsed().as_secs();
        
        let avg_speed = if elapsed > 0 {
            downloaded / elapsed
        } else {
            0
        };
        
        let eta = if speed > 0 {
            Some((self.total - downloaded) / speed)
        } else {
            None
        };
        
        let percent = if self.total > 0 {
            (downloaded as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };
        
        DownloadProgress {
            total: self.total,
            downloaded,
            speed,
            avg_speed,
            eta,
            percent,
        }
    }
}
```

#### 验收清单
- [ ] 原子操作正确
- [ ] 进度计算准确
- [ ] 线程安全

---

### T1.6.2 速度计算

**时间**: 1h  
**依赖**: T1.6.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 SpeedCalculator | `src/progress/speed.rs` | 结构定义 | 结构完整 |
| 2 | 实现滑动窗口 | `src/progress/speed.rs` | 窗口算法 | 算法正确 |
| 3 | 实现速度计算 | `src/progress/speed.rs` | `get_speed()` | 计算正确 |

#### 输出文件

**src/progress/speed.rs**:
```rust
use std::collections::VecDeque;
use std::time::Instant;

/// 速度样本
struct Sample {
    timestamp: Instant,
    bytes: u64,
}

/// 速度计算器 (滑动窗口)
pub struct SpeedCalculator {
    samples: VecDeque<Sample>,
    window_size: usize,
    total_bytes: u64,
}

impl SpeedCalculator {
    pub fn new(window_size: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(window_size),
            window_size,
            total_bytes: 0,
        }
    }
    
    pub fn add_sample(&mut self, bytes: u64) {
        let now = Instant::now();
        self.samples.push_back(Sample {
            timestamp: now,
            bytes,
        });
        self.total_bytes += bytes;
        
        // 移除旧样本
        while self.samples.len() > self.window_size {
            if let Some(old) = self.samples.pop_front() {
                self.total_bytes -= old.bytes;
            }
        }
    }
    
    pub fn get_speed(&self) -> u64 {
        if self.samples.len() < 2 {
            return 0;
        }
        
        let first = self.samples.front().unwrap();
        let last = self.samples.back().unwrap();
        
        let duration = last.timestamp.duration_since(first.timestamp).as_secs();
        if duration == 0 {
            return 0;
        }
        
        self.total_bytes / duration
    }
}
```

#### 验收清单
- [ ] 滑动窗口正确
- [ ] 速度计算准确
- [ ] 边界情况处理

---

### T1.6.3 回调机制

**时间**: 1h  
**依赖**: T1.6.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义回调类型 | `src/progress/mod.rs` | `ProgressCallback` | 类型定义 |
| 2 | 实现回调触发 | `src/download/manager.rs` | 触发逻辑 | 触发正确 |
| 3 | 实现异步回调 | `src/progress/mod.rs` | 异步支持 | 支持异步 |

#### 输出代码

**src/progress/mod.rs**:
```rust
mod tracker;
mod speed;

pub use tracker::Tracker;
pub use speed::SpeedCalculator;

/// 进度回调函数
pub type ProgressCallback = Box<dyn Fn(DownloadProgress) + Send + Sync>;

/// 下载进度
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub total: u64,
    pub downloaded: u64,
    pub speed: u64,
    pub avg_speed: u64,
    pub eta: Option<u64>,
    pub percent: f64,
}

/// 创建进度回调
pub fn progress_callback<F>(f: F) -> Option<ProgressCallback>
where
    F: Fn(DownloadProgress) + Send + Sync + 'static,
{
    Some(Box::new(f))
}
```

#### 验收清单
- [ ] 回调类型正确
- [ ] 异步触发正常
- [ ] 线程安全

---

## T1.7 断点续传模块

### T1.7.1 状态持久化

**时间**: 1.5h  
**依赖**: T1.4, T1.5  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 ResumeState | `src/resume/state.rs` | 状态结构 | 结构完整 |
| 2 | 实现 save | `src/resume/state.rs` | `save()` | 保存正确 |
| 3 | 实现 load | `src/resume/state.rs` | `load()` | 加载正确 |

#### 输出文件

**src/resume/state.rs**:
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use crate::chunk::Chunk;

/// 分片恢复状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResumeState {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub temp_path: PathBuf,
}

/// 任务恢复状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeState {
    pub task_id: String,
    pub url: String,
    pub file_size: u64,
    pub etag: Option<String>,
    pub downloaded: u64,
    pub chunks: Vec<ChunkResumeState>,
    pub output_path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResumeState {
    /// 保存状态到文件
    pub async fn save(&self, path: &std::path::Path) -> crate::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }
    
    /// 从文件加载状态
    pub async fn load(path: &std::path::Path) -> crate::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        
        let json = tokio::fs::read_to_string(path).await?;
        let state: Self = serde_json::from_str(&json)?;
        Ok(Some(state))
    }
}
```

#### 验收清单
- [ ] 序列化正确
- [ ] 文件保存成功
- [ ] 文件加载成功

---

### T1.7.2 恢复逻辑

**时间**: 1.5h  
**依赖**: T1.7.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Recovery | `src/resume/recovery.rs` | 恢复器 | 结构定义 |
| 2 | 实现恢复方法 | `src/resume/recovery.rs` | `recover()` | 恢复正确 |
| 3 | 实现 ETag 验证 | `src/resume/recovery.rs` | 验证逻辑 | 验证正确 |

#### 输出代码

**src/resume/recovery.rs**:
```rust
use super::ResumeState;
use crate::http::Client;
use crate::error::{DownloadError, Result};

/// 断点续传恢复器
pub struct Recovery {
    client: Client,
}

impl Recovery {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    
    /// 尝试恢复下载
    pub async fn try_recover(
        &self,
        state: ResumeState,
    ) -> Result<Option<Vec<crate::chunk::Chunk>>> {
        // 验证 ETag
        if let Some(ref etag) = state.etag {
            let head = self.client.head(&state.url).await?;
            if head.etag.as_ref() != Some(etag) {
                return Err(DownloadError::ValidationFailed(
                    "ETag mismatch".to_string()
                ));
            }
        }
        
        // 转换为 Chunk
        let chunks: Vec<crate::chunk::Chunk> = state
            .chunks
            .into_iter()
            .filter_map(|c| {
                if c.downloaded < (c.end - c.start) {
                    Some(crate::chunk::Chunk::new(
                        c.id,
                        c.start + c.downloaded,
                        c.end,
                    ))
                } else {
                    None
                }
            })
            .collect();
        
        if chunks.is_empty() {
            Ok(None) // 已完成
        } else {
            Ok(Some(chunks))
        }
    }
}
```

#### 验收清单
- [ ] ETag 验证正确
- [ ] 恢复逻辑正确
- [ ] 完成状态处理

---

### T1.7.3 状态验证

**时间**: 1h  
**依赖**: T1.7.2  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现完整性验证 | `src/resume/recovery.rs` | 验证函数 | 验证正确 |
| 2 | 实现损坏检测 | `src/resume/recovery.rs` | 检测逻辑 | 检测正确 |
| 3 | 实现自动修复 | `src/resume/recovery.rs` | 修复逻辑 | 修复正确 |

#### 验收清单
- [ ] 完整性验证正确
- [ ] 损坏检测正确
- [ ] 自动修复正常

---

## T1.8 任务管理模块

### T1.8.1 Task 结构

**时间**: 1h  
**依赖**: T1.1.1  
**并行**: 可与 T1.5 并行

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Task | `src/download/task.rs` | `Task` | 结构定义 |
| 2 | 定义 TaskState | `src/download/task.rs` | `TaskState` | 状态枚举 |
| 3 | 实现状态转换 | `src/download/task.rs` | 转换方法 | 转换正确 |

#### 输出文件

**src/download/task.rs**:
```rust
use std::sync::Arc;
use parking_lot::Mutex;
use crate::progress::Tracker;

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// 下载任务
pub struct Task {
    pub id: String,
    pub config: DownloadConfig,
    state: Mutex<TaskState>,
    tracker: Tracker,
}

impl Task {
    pub fn new(config: DownloadConfig, file_size: u64) -> Self {
        Self {
            id: config.id.clone(),
            config,
            state: Mutex::new(TaskState::Pending),
            tracker: Tracker::new(file_size),
        }
    }
    
    pub fn state(&self) -> TaskState {
        *self.state.lock()
    }
    
    pub fn set_state(&self, state: TaskState) {
        *self.state.lock() = state;
    }
    
    pub fn tracker(&self) -> &Tracker {
        &self.tracker
    }
}
```

#### 验收清单
- [ ] 结构定义完整
- [ ] 状态转换正确
- [ ] 线程安全

---

### T1.8.2 Manager 实现

**时间**: 2h  
**依赖**: T1.8.1, T1.5, T1.7  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Manager | `src/download/manager.rs` | `Manager` | 结构定义 |
| 2 | 实现 create_task | `src/download/manager.rs` | 创建任务 | 方法可用 |
| 3 | 实现 start/pause/resume | `src/download/manager.rs` | 控制方法 | 方法可用 |

#### 输出代码

**src/download/manager.rs**:
```rust
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::error::{DownloadError, Result};
use crate::http::Client;
use super::{Task, DownloadConfig, DownloadResult};

/// 下载管理器
pub struct Manager {
    client: Client,
    tasks: RwLock<HashMap<String, Arc<Task>>>,
    max_concurrent: usize,
}

impl Manager {
    pub fn new(client: Client, max_concurrent: usize) -> Self {
        Self {
            client,
            tasks: RwLock::new(HashMap::new()),
            max_concurrent,
        }
    }
    
    /// 创建下载任务
    pub async fn create_task(&self, config: DownloadConfig) -> Result<String> {
        let task_id = config.id.clone();
        
        // 检查是否已存在
        {
            let tasks = self.tasks.read();
            if tasks.contains_key(&task_id) {
                return Err(DownloadError::TaskNotFound(task_id));
            }
        }
        
        // 获取文件信息
        let head = self.client.head(&config.url).await?;
        let file_size = head.content_length
            .ok_or(DownloadError::RangeNotSupported)?;
        
        // 创建任务
        let task = Arc::new(Task::new(config, file_size));
        
        {
            let mut tasks = self.tasks.write();
            tasks.insert(task_id.clone(), task);
        }
        
        Ok(task_id)
    }
    
    /// 获取任务
    pub fn get_task(&self, task_id: &str) -> Option<Arc<Task>> {
        let tasks = self.tasks.read();
        tasks.get(task_id).cloned()
    }
}
```

#### 验收清单
- [ ] 任务创建正确
- [ ] 并发访问安全
- [ ] 错误处理完善

---

### T1.8.3 Builder 模式

**时间**: 1h  
**依赖**: T1.8.2  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 DownloaderBuilder | `src/download/mod.rs` | Builder | 结构定义 |
| 2 | 实现链式方法 | `src/download/mod.rs` | 链式调用 | 调用正确 |
| 3 | 实现 build | `src/download/mod.rs` | 构建方法 | 构建正确 |

#### 输出代码

**src/download/mod.rs**:
```rust
mod task;
mod manager;

pub use task::{Task, TaskState};
pub use manager::Manager;

use crate::error::Result;
use crate::http::Client;

/// 下载器建造者
pub struct DownloaderBuilder {
    max_concurrent_tasks: usize,
    default_threads: u32,
    timeout: std::time::Duration,
}

impl DownloaderBuilder {
    pub fn new() -> Self {
        Self {
            max_concurrent_tasks: 3,
            default_threads: 4,
            timeout: std::time::Duration::from_secs(300),
        }
    }
    
    pub fn max_concurrent_tasks(mut self, count: usize) -> Self {
        self.max_concurrent_tasks = count;
        self
    }
    
    pub fn default_threads(mut self, threads: u32) -> Self {
        self.default_threads = threads;
        self
    }
    
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = std::time::Duration::from_secs(secs);
        self
    }
    
    pub fn build(self) -> Result<Downloader> {
        let client = Client::new(crate::http::ClientConfig {
            timeout: self.timeout,
            ..Default::default()
        })?;
        
        Ok(Downloader {
            manager: Manager::new(client, self.max_concurrent_tasks),
        })
    }
}

impl Default for DownloaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
```

#### 验收清单
- [ ] Builder 模式正确
- [ ] 链式调用正常
- [ ] 构建结果正确

---

## T1.9 集成测试

### T1.9.1 Mock 服务器

**时间**: 1h  
**依赖**: T1.1.3  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 配置 wiremock | `tests/common/mock.rs` | Mock 配置 | 配置正确 |
| 2 | 实现 mock 文件服务 | `tests/common/mock.rs` | mock 函数 | 服务可用 |
| 3 | 实现错误模拟 | `tests/common/mock.rs` | 错误模拟 | 模拟正确 |

#### 验收清单
- [ ] Mock 服务器可启动
- [ ] 响应可配置
- [ ] 错误可模拟

---

### T1.9.2 端到端测试

**时间**: 2h  
**依赖**: T1.9.1, T1.8  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 完整下载测试 | `tests/e2e_test.rs` | 测试用例 | 测试通过 |
| 2 | 断点续传测试 | `tests/e2e_test.rs` | 测试用例 | 测试通过 |
| 3 | 错误恢复测试 | `tests/e2e_test.rs` | 测试用例 | 测试通过 |

#### 验收清单
- [ ] 完整流程测试通过
- [ ] 断点续传测试通过
- [ ] 错误恢复测试通过

---

### T1.9.3 性能测试

**时间**: 1h  
**依赖**: T1.9.2  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 创建 Benchmark | `benches/download_bench.rs` | 基准测试 | 测试可运行 |
| 2 | 多线程性能 | `benches/download_bench.rs` | 并发测试 | 性能达标 |
| 3 | 内存分析 | 手动测试 | 内存测试 | 内存合理 |

#### 验收清单
- [ ] Benchmark 可运行
- [ ] 性能达标
- [ ] 内存占用合理

---

## T1.10 文档与示例

### T1.10.1 API 文档

**时间**: 1h  
**依赖**: T1.8  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 添加文档注释 | 所有源文件 | `///` 注释 | 注释完整 |
| 2 | 生成文档 | `cargo doc` | API 文档 | 生成成功 |
| 3 | 示例代码 | 文档注释 | 示例 | 示例正确 |

#### 验收清单
- [ ] 所有公开 API 有文档
- [ ] 示例代码正确
- [ ] `cargo doc` 无警告

---

### T1.10.2 使用示例

**时间**: 1h  
**依赖**: T1.10.1  
**并行**: 否

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 基础示例 | `examples/basic.rs` | 示例代码 | 可运行 |
| 2 | 进度回调示例 | `examples/with_progress.rs` | 示例代码 | 可运行 |
| 3 | 断点续传示例 | `examples/resume.rs` | 示例代码 | 可运行 |

#### 验收清单
- [ ] 所有示例可运行
- [ ] 示例覆盖主要用例
- [ ] 示例代码清晰易懂

---

## 任务依赖关系图

```
T1.1.1 ──┬── T1.1.2 ──────────────────────────────────────────┐
          │                                                   │
          ├── T1.1.3 ─────────────────────────────────────────┤
          │                                                   │
          └── T1.1.4 ─────────────────────────────────────────┤
                                                              │
T1.2.1 ──── T1.2.2 ────────────────────────────────────────────┤
     │                                                        │
     └──(并行)─────────────────────────────────────────────────┤
                                                              │
T1.3.1 ─── T1.3.2 ─── T1.3.3 ─────────────────────────────────┤
                                                              │
T1.4.1 ─── T1.4.2 ─── T1.4.3 ─────────────────────────────────┤
     │                                                        │
     └──(并行)─────────────────────────────────────────────────┤
                                                              │
T1.5.1 ─── T1.5.2 ─── T1.5.3 ─── T1.5.4 ──────────────────────┤
                                                              │
T1.6.1 ─── T1.6.2 ─── T1.6.3 ─(与T1.5并行)────────────────────┤
                                                              │
T1.7.1 ─── T1.7.2 ─── T1.7.3 ─────────────────────────────────┤
                                                              │
T1.8.1 ─── T1.8.2 ─── T1.8.3 ─────────────────────────────────┘
                                                              │
T1.9.1 ─── T1.9.2 ─── T1.9.3 ◀───────────────────────────────┘
                                                              │
T1.10.1 ─── T1.10.2 ◀────────────────────────────────────────┘
```

---

## 工时汇总

| 任务 | 时间 | 并行机会 |
|------|------|----------|
| T1.1 项目初始化 | 2h | T1.1.2-4 可部分并行 |
| T1.2 错误处理 | 2h | 与 T1.3 并行 |
| T1.3 HTTP 客户端 | 3h | - |
| T1.4 分片策略 | 3h | 与 T1.3 并行 |
| T1.5 多线程下载 | 6h | - |
| T1.6 进度追踪 | 3h | 与 T1.5 并行 |
| T1.7 断点续传 | 4h | - |
| T1.8 任务管理 | 4h | T1.8.1 可与 T1.5 并行 |
| T1.9 集成测试 | 4h | - |
| T1.10 文档示例 | 2h | - |
| **总计** | **33h** | **优化后约 25h** |

---

*任务链规划版本: v1.0*
*规划日期: 2026-03-26*