# P3: 多线程下载 + 断点续传 详细任务链规划

> 按照乐高式开发模式：项目框架 → 任务链 → 子任务 → 步骤 → 验证

---

## 任务链总览

```
P3: 多线程下载 + 断点续传
├── T3.1 Range 请求模块 (3h)
│   ├── T3.1.1 RangeClient 结构定义 (0.5h)
│   ├── T3.1.2 Range 支持检测 (1h)
│   ├── T3.1.3 Content-Length 获取 (0.5h)
│   └── T3.1.4 Range 数据获取 (1h)
│
├── T3.2 分片管理模块 (3h)
│   ├── T3.2.1 Chunk 数据结构 (0.5h)
│   ├── T3.2.2 ChunkManager 实现 (1h)
│   ├── T3.2.3 分片策略计算 (1h)
│   └── T3.2.4 分片状态管理 (0.5h)
│
├── T3.3 线程池模块 (2h)
│   ├── T3.3.1 WorkerPool 结构 (0.5h)
│   ├── T3.3.2 并发控制实现 (1h)
│   └── T3.3.3 任务调度 (0.5h)
│
├── T3.4 分片下载 Worker (3h)
│   ├── T3.4.1 ChunkWorker 实现 (1h)
│   ├── T3.4.2 下载逻辑实现 (1h)
│   └── T3.4.3 错误重试机制 (1h)
│
├── T3.5 分片存储模块 (3h)
│   ├── T3.5.1 ChunkWriter 实现 (1h)
│   ├── T3.5.2 临时文件管理 (1h)
│   └── T3.5.3 FileMerger 实现 (1h)
│
├── T3.6 状态持久化模块 (3h)
│   ├── T3.6.1 StateManager 实现 (1h)
│   ├── T3.6.2 状态保存 (1h)
│   └── T3.6.3 状态恢复 (1h)
│
├── T3.7 事件系统 (2h)
│   ├── T3.7.1 EventEmitter 实现 (1h)
│   └── T3.7.2 Tauri 事件集成 (1h)
│
├── T3.8 进度计算模块 (2h)
│   ├── T3.8.1 进度追踪器 (1h)
│   └── T3.8.2 速度计算 (1h)
│
├── T3.9 多线程下载整合 (4h)
│   ├── T3.9.1 MultiThreadDownloader 实现 (2h)
│   ├── T3.9.2 下载流程整合 (1h)
│   └── T3.9.3 暂停/恢复功能 (1h)
│
├── T3.10 Tauri 命令集成 (2h)
│   ├── T3.10.1 命令定义 (1h)
│   └── T3.10.2 前端事件监听 (1h)
│
└── T3.11 测试与文档 (3h)
    ├── T3.11.1 单元测试 (1.5h)
    └── T3.11.2 集成测试 (1.5h)
```

**总工时**: 30 小时

---

## T3.1 Range 请求模块

### T3.1.1 RangeClient 结构定义

**时间**: 0.5h  
**依赖**: 无

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 创建 range 模块目录 | `src/range/` | 目录存在 |
| 2 | 定义 RangeClientConfig | `src/range/client.rs` | 结构定义 |
| 3 | 定义 RangeClient | `src/range/client.rs` | 结构定义 |
| 4 | 实现 new 方法 | `src/range/client.rs` | 可创建实例 |

#### 输出文件

**src/range/mod.rs**:
```rust
mod client;
mod support;

pub use client::{RangeClient, RangeClientConfig};
pub use support::RangeSupport;
```

**src/range/client.rs**:
```rust
use reqwest::Client;
use std::time::Duration;
use crate::Result;

pub struct RangeClientConfig {
    pub timeout: Duration,
    pub retry_count: u32,
    pub user_agent: String,
}

impl Default for RangeClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300),
            retry_count: 3,
            user_agent: "TurboDownload/1.0".to_string(),
        }
    }
}

pub struct RangeClient {
    inner: Client,
    config: RangeClientConfig,
}

impl RangeClient {
    pub fn new(config: RangeClientConfig) -> Result<Self> {
        let inner = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()?;
        
        Ok(Self { inner, config })
    }
    
    pub fn with_defaults() -> Result<Self> {
        Self::new(RangeClientConfig::default())
    }
}
```

#### 验收清单
- [ ] `cargo check` 通过
- [ ] RangeClient 可创建

---

### T3.1.2 Range 支持检测

**时间**: 1h  
**依赖**: T3.1.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 RangeSupport | `src/range/support.rs` | 结构定义 | 定义完整 |
| 2 | 实现 check_range_support | `src/range/client.rs` | 异步方法 | 方法可用 |
| 3 | 解析 Accept-Ranges 头 | `src/range/support.rs` | 解析逻辑 | 解析正确 |

#### 输出文件

**src/range/support.rs**:
```rust
#[derive(Debug, Clone)]
pub struct RangeSupport {
    pub supported: bool,
    pub content_length: Option<u64>,
    pub accept_ranges: Option<String>,
    pub etag: Option<String>,
}

impl RangeSupport {
    pub fn is_supported(&self) -> bool {
        self.supported && self.accept_ranges.as_deref() == Some("bytes")
    }
}
```

**src/range/client.rs (续)**:
```rust
impl RangeClient {
    /// 检测服务器是否支持 Range 请求
    pub async fn check_range_support(&self, url: &str) -> Result<RangeSupport> {
        let response = self.inner.head(url).send().await?;
        
        let headers = response.headers();
        
        Ok(RangeSupport {
            supported: response.status().is_success(),
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
        })
    }
}
```

#### 验收清单
- [ ] HEAD 请求可发送
- [ ] Accept-Ranges 正确解析
- [ ] is_supported() 返回正确

---

### T3.1.3 Content-Length 获取

**时间**: 0.5h  
**依赖**: T3.1.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 get_content_length | `src/range/client.rs` | 方法 | 返回文件大小 |

#### 输出代码

```rust
impl RangeClient {
    /// 获取文件总大小
    pub async fn get_content_length(&self, url: &str) -> Result<u64> {
        let support = self.check_range_support(url).await?;
        
        support.content_length
            .ok_or_else(|| DownloadError::ContentLengthUnknown)
    }
}
```

#### 验收清单
- [ ] 正确获取文件大小

---

### T3.1.4 Range 数据获取

**时间**: 1h  
**依赖**: T3.1.3

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 fetch_range | `src/range/client.rs` | 方法 | 获取指定范围数据 |
| 2 | 处理 Range 头格式 | `src/range/client.rs` | 格式化 | 格式正确 |
| 3 | 处理 206 响应 | `src/range/client.rs` | 响应处理 | 正确处理 |

#### 输出代码

```rust
use bytes::Bytes;

impl RangeClient {
    /// 下载指定范围的数据
    pub async fn fetch_range(
        &self,
        url: &str,
        start: u64,
        end: u64,
    ) -> Result<Bytes> {
        let range_header = format!("bytes={}-{}", start, end - 1);
        
        let response = self.inner
            .get(url)
            .header("Range", range_header)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() && status.as_u16() != 206 {
            return Err(DownloadError::HttpError(status.as_u16(), status.to_string()));
        }
        
        let bytes = response.bytes().await?;
        Ok(bytes)
    }
    
    /// 从指定位置开始下载（断点续传用）
    pub async fn fetch_from(&self, url: &str, start: u64) -> Result<Bytes> {
        let response = self.inner
            .get(url)
            .header("Range", format!("bytes={}-", start))
            .send()
            .await?;
        
        response.bytes().await.map_err(Into::into)
    }
}
```

#### 验收清单
- [ ] Range 请求格式正确
- [ ] 返回正确数据
- [ ] 错误处理完善

---

## T3.2 分片管理模块

### T3.2.1 Chunk 数据结构

**时间**: 0.5h  
**依赖**: 无

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 创建 chunk 模块 | `src/chunk/` | 目录 | 目录存在 |
| 2 | 定义 ChunkState | `src/chunk/state.rs` | 枚举 | 定义完整 |
| 3 | 定义 Chunk | `src/chunk/mod.rs` | 结构 | 定义完整 |

#### 输出文件

**src/chunk/state.rs**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChunkState {
    Pending,
    Downloading,
    Completed,
    Failed,
}
```

**src/chunk/mod.rs**:
```rust
mod state;
mod manager;

pub use state::ChunkState;
pub use manager::ChunkManager;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub state: ChunkState,
    pub temp_path: PathBuf,
}

impl Chunk {
    pub fn new(id: u32, start: u64, end: u64, temp_dir: &std::path::Path) -> Self {
        Self {
            id,
            start,
            end,
            downloaded: 0,
            state: ChunkState::Pending,
            temp_path: temp_dir.join(format!("chunk_{}.tmp", id)),
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
    
    pub fn progress_percent(&self) -> f64 {
        if self.size() == 0 {
            return 100.0;
        }
        (self.downloaded as f64 / self.size() as f64) * 100.0
    }
}
```

#### 验收清单
- [ ] 数据结构完整
- [ ] 方法实现正确

---

### T3.2.2 ChunkManager 实现

**时间**: 1h  
**依赖**: T3.2.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 ChunkManager | `src/chunk/manager.rs` | 结构 | 定义完整 |
| 2 | 实现 new 方法 | `src/chunk/manager.rs` | 构造 | 可创建 |
| 3 | 实现状态更新方法 | `src/chunk/manager.rs` | 更新方法 | 更新正确 |

#### 输出文件

**src/chunk/manager.rs**:
```rust
use super::{Chunk, ChunkState};
use std::path::PathBuf;

pub struct ChunkManager {
    chunks: Vec<Chunk>,
    total_size: u64,
    chunk_size: u64,
    temp_dir: PathBuf,
}

impl ChunkManager {
    pub fn new(total_size: u64, chunk_size: u64, temp_dir: PathBuf) -> Self {
        Self {
            chunks: Vec::new(),
            total_size,
            chunk_size,
            temp_dir,
        }
    }
    
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }
    
    pub fn total_size(&self) -> u64 {
        self.total_size
    }
    
    pub fn total_downloaded(&self) -> u64 {
        self.chunks.iter().map(|c| c.downloaded).sum()
    }
    
    pub fn progress_percent(&self) -> f64 {
        if self.total_size == 0 {
            return 0.0;
        }
        (self.total_downloaded() as f64 / self.total_size as f64) * 100.0
    }
}
```

#### 验收清单
- [ ] ChunkManager 可创建
- [ ] 进度计算正确

---

### T3.2.3 分片策略计算

**时间**: 1h  
**依赖**: T3.2.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 calculate_chunks | `src/chunk/manager.rs` | 方法 | 计算正确 |
| 2 | 处理边界情况 | `src/chunk/manager.rs` | 边界处理 | 处理正确 |

#### 输出代码

```rust
impl ChunkManager {
    /// 计算并创建分片
    pub fn calculate_chunks(&mut self, thread_count: u32) {
        let chunk_size = if self.chunk_size > 0 {
            self.chunk_size
        } else {
            // 自动计算：文件大小 / 线程数，最小 1MB
            (self.total_size / thread_count as u64).max(1024 * 1024)
        };
        
        let mut start = 0u64;
        let mut id = 0u32;
        
        while start < self.total_size {
            let end = (start + chunk_size).min(self.total_size);
            let chunk = Chunk::new(id, start, end, &self.temp_dir);
            self.chunks.push(chunk);
            
            start = end;
            id += 1;
        }
    }
    
    /// 获取下一个待下载的分片
    pub fn get_next_pending(&mut self) -> Option<&mut Chunk> {
        self.chunks
            .iter_mut()
            .find(|c| c.state == ChunkState::Pending)
    }
    
    /// 获取未完成的分片数量
    pub fn pending_count(&self) -> usize {
        self.chunks
            .iter()
            .filter(|c| c.state != ChunkState::Completed)
            .count()
    }
}
```

#### 验收清单
- [ ] 分片计算正确
- [ ] 边界处理完善

---

### T3.2.4 分片状态管理

**时间**: 0.5h  
**依赖**: T3.2.3

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现更新方法 | `src/chunk/manager.rs` | update 方法 | 更新正确 |
| 2 | 实现查询方法 | `src/chunk/manager.rs` | 查询方法 | 查询正确 |

#### 输出代码

```rust
impl ChunkManager {
    /// 更新分片状态
    pub fn update_chunk(&mut self, id: u32, downloaded: u64, state: ChunkState) {
        if let Some(chunk) = self.chunks.get_mut(id as usize) {
            chunk.downloaded = downloaded;
            chunk.state = state;
        }
    }
    
    /// 标记分片为下载中
    pub fn mark_downloading(&mut self, id: u32) {
        if let Some(chunk) = self.chunks.get_mut(id as usize) {
            chunk.state = ChunkState::Downloading;
        }
    }
    
    /// 标记分片为完成
    pub fn mark_completed(&mut self, id: u32) {
        if let Some(chunk) = self.chunks.get_mut(id as usize) {
            chunk.downloaded = chunk.size();
            chunk.state = ChunkState::Completed;
        }
    }
    
    /// 标记分片为失败
    pub fn mark_failed(&mut self, id: u32) {
        if let Some(chunk) = self.chunks.get_mut(id as usize) {
            chunk.state = ChunkState::Failed;
        }
    }
}
```

#### 验收清单
- [ ] 状态更新正确
- [ ] 方法可用

---

## T3.3 线程池模块

### T3.3.1 WorkerPool 结构

**时间**: 0.5h  
**依赖**: 无

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 创建 pool 模块 | `src/pool/` | 目录 | 目录存在 |
| 2 | 定义 WorkerPool | `src/pool/worker_pool.rs` | 结构 | 定义完整 |

#### 输出文件

**src/pool/mod.rs**:
```rust
mod worker_pool;

pub use worker_pool::WorkerPool;
```

**src/pool/worker_pool.rs**:
```rust
use tokio::task::JoinSet;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct WorkerPool {
    max_workers: usize,
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            semaphore: Arc::new(Semaphore::new(max_workers)),
        }
    }
    
    pub fn max_workers(&self) -> usize {
        self.max_workers
    }
}
```

#### 验收清单
- [ ] WorkerPool 可创建

---

### T3.3.2 并发控制实现

**时间**: 1h  
**依赖**: T3.3.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现信号量控制 | `src/pool/worker_pool.rs` | acquire | 控制正确 |
| 2 | 实现 spawn 方法 | `src/pool/worker_pool.rs` | spawn | 可提交任务 |

#### 输出代码

```rust
use futures::future::BoxFuture;
use std::future::Future;

impl WorkerPool {
    /// 提交任务到线程池
    pub async fn spawn<F, T>(&self, task: F) -> tokio::task::JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        
        tokio::spawn(async move {
            let result = task.await;
            drop(permit);
            result
        })
    }
    
    /// 尝试提交任务（非阻塞）
    pub fn try_spawn<F, T>(&self, task: F) -> Option<tokio::task::JoinHandle<T>>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        match self.semaphore.clone().try_acquire_owned() {
            Ok(permit) => {
                Some(tokio::spawn(async move {
                    let result = task.await;
                    drop(permit);
                    result
                }))
            }
            Err(_) => None,
        }
    }
}
```

#### 验收清单
- [ ] 并发限制生效
- [ ] 任务可提交

---

### T3.3.3 任务调度

**时间**: 0.5h  
**依赖**: T3.3.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 wait_all | `src/pool/worker_pool.rs` | 等待方法 | 等待正确 |

#### 输出代码

```rust
impl WorkerPool {
    /// 等待所有任务完成
    pub async fn wait_all<T>(handles: Vec<tokio::task::JoinHandle<T>>) -> Vec<T>
    where
        T: Send + 'static,
    {
        let mut results = Vec::with_capacity(handles.len());
        
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }
        
        results
    }
}
```

#### 验收清单
- [ ] 等待逻辑正确

---

## T3.4 分片下载 Worker

### T3.4.1 ChunkWorker 实现

**时间**: 1h  
**依赖**: T3.1, T3.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 ChunkWorker | `src/chunk/worker.rs` | 结构 | 定义完整 |
| 2 | 实现 new 方法 | `src/chunk/worker.rs` | 构造 | 可创建 |

#### 输出文件

**src/chunk/worker.rs**:
```rust
use super::{Chunk, ChunkState};
use crate::range::RangeClient;
use crate::storage::ChunkWriter;
use crate::{Result, DownloadError};
use tokio::sync::mpsc::Sender;

pub struct ChunkProgress {
    pub chunk_id: u32,
    pub downloaded: u64,
}

pub struct ChunkWorker {
    chunk: Chunk,
    client: RangeClient,
    writer: ChunkWriter,
    url: String,
}

impl ChunkWorker {
    pub fn new(
        chunk: Chunk,
        client: RangeClient,
        writer: ChunkWriter,
        url: String,
    ) -> Self {
        Self {
            chunk,
            client,
            writer,
            url,
        }
    }
    
    pub fn chunk_id(&self) -> u32 {
        self.chunk.id
    }
}
```

#### 验收清单
- [ ] Worker 可创建

---

### T3.4.2 下载逻辑实现

**时间**: 1h  
**依赖**: T3.4.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 download 方法 | `src/chunk/worker.rs` | 下载方法 | 下载正确 |
| 2 | 实现进度报告 | `src/chunk/worker.rs` | 进度发送 | 发送正确 |

#### 输出代码

```rust
impl ChunkWorker {
    /// 执行分片下载
    pub async fn download(
        &mut self,
        progress_tx: Sender<ChunkProgress>,
    ) -> Result<()> {
        let start = self.chunk.start + self.chunk.downloaded;
        let end = self.chunk.end;
        
        if start >= end {
            return Ok(()); // 已完成
        }
        
        // 下载分片数据
        let data = self.client.fetch_range(&self.url, start, end).await?;
        
        // 写入临时文件
        self.writer.write(&self.chunk.temp_path, &data).await?;
        
        // 更新进度
        self.chunk.downloaded = self.chunk.size();
        
        // 发送进度
        let _ = progress_tx.send(ChunkProgress {
            chunk_id: self.chunk.id,
            downloaded: self.chunk.downloaded,
        }).await;
        
        Ok(())
    }
    
    /// 获取分片引用
    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }
    
    /// 获取分片可变引用
    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }
}
```

#### 验收清单
- [ ] 下载逻辑正确
- [ ] 进度报告正常

---

### T3.4.3 错误重试机制

**时间**: 1h  
**依赖**: T3.4.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现重试逻辑 | `src/chunk/worker.rs` | retry 方法 | 重试正确 |
| 2 | 实现退避策略 | `src/chunk/worker.rs` | 退避计算 | 退避正确 |

#### 输出代码

```rust
impl ChunkWorker {
    /// 带重试的下载
    pub async fn download_with_retry(
        &mut self,
        progress_tx: Sender<ChunkProgress>,
        max_retries: u32,
    ) -> Result<()> {
        let mut attempt = 0;
        
        loop {
            match self.download(progress_tx.clone()).await {
                Ok(()) => return Ok(()),
                Err(e) if attempt < max_retries => {
                    attempt += 1;
                    let delay = std::time::Duration::from_secs(2u64.pow(attempt));
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

#### 验收清单
- [ ] 重试逻辑正确
- [ ] 退避策略生效

---

## T3.5 分片存储模块

### T3.5.1 ChunkWriter 实现

**时间**: 1h  
**依赖**: 无

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 创建 storage 模块 | `src/storage/` | 目录 | 目录存在 |
| 2 | 定义 ChunkWriter | `src/storage/writer.rs` | 结构 | 定义完整 |
| 3 | 实现 write 方法 | `src/storage/writer.rs` | 写入方法 | 写入正确 |

#### 输出文件

**src/storage/writer.rs**:
```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use crate::Result;

pub struct ChunkWriter;

impl ChunkWriter {
    pub fn new() -> Self {
        Self
    }
    
    /// 写入分片数据到临时文件
    pub async fn write(&self, path: &Path, data: &[u8]) -> Result<()> {
        let mut file = File::create(path).await?;
        file.write_all(data).await?;
        file.flush().await?;
        Ok(())
    }
    
    /// 追加写入数据
    pub async fn append(&self, path: &Path, data: &[u8]) -> Result<()> {
        let mut file = File::options()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        file.write_all(data).await?;
        Ok(())
    }
}
```

#### 验收清单
- [ ] 写入正确
- [ ] 文件创建成功

---

### T3.5.2 临时文件管理

**时间**: 1h  
**依赖**: T3.5.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 创建临时目录 | `src/storage/writer.rs` | 创建方法 | 创建正确 |
| 2 | 清理临时文件 | `src/storage/writer.rs` | 清理方法 | 清理正确 |

#### 输出代码

```rust
impl ChunkWriter {
    /// 创建临时目录
    pub async fn create_temp_dir(base: &Path, task_id: &str) -> Result<std::path::PathBuf> {
        let temp_dir = base.join("temp").join(task_id);
        tokio::fs::create_dir_all(&temp_dir).await?;
        Ok(temp_dir)
    }
    
    /// 清理临时文件
    pub async fn cleanup(temp_dir: &Path) -> Result<()> {
        if temp_dir.exists() {
            tokio::fs::remove_dir_all(temp_dir).await?;
        }
        Ok(())
    }
}
```

#### 验收清单
- [ ] 临时目录创建正确
- [ ] 清理正常

---

### T3.5.3 FileMerger 实现

**时间**: 1h  
**依赖**: T3.5.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 FileMerger | `src/storage/merger.rs` | 结构 | 定义完整 |
| 2 | 实现 merge 方法 | `src/storage/merger.rs` | 合并方法 | 合并正确 |
| 3 | 实现验证方法 | `src/storage/merger.rs` | 验证方法 | 验证正确 |

#### 输出文件

**src/storage/merger.rs**:
```rust
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use crate::{Result, DownloadError};
use super::Chunk;

pub struct FileMerger;

impl FileMerger {
    /// 合并分片文件
    pub async fn merge(
        chunks: &[Chunk],
        output_path: &Path,
    ) -> Result<()> {
        // 确保输出目录存在
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let mut output = File::create(output_path).await?;
        let mut buffer = vec![0u8; 64 * 1024];
        
        for chunk in chunks {
            let mut input = File::open(&chunk.temp_path).await?;
            
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
    
    /// 验证文件大小
    pub async fn verify_size(path: &Path, expected_size: u64) -> Result<bool> {
        let metadata = tokio::fs::metadata(path).await?;
        Ok(metadata.len() == expected_size)
    }
}
```

#### 验收清单
- [ ] 合并正确
- [ ] 验证正常

---

## T3.6 状态持久化模块

### T3.6.1 StateManager 实现

**时间**: 1h  
**依赖**: T3.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 StateManager | `src/storage/state.rs` | 结构 | 定义完整 |
| 2 | 定义 DownloadState | `src/storage/state.rs` | 状态结构 | 定义完整 |

#### 输出文件

**src/storage/state.rs**:
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::chunk::Chunk;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadState {
    pub task_id: String,
    pub url: String,
    pub output_path: PathBuf,
    pub total_size: u64,
    pub downloaded: u64,
    pub chunks: Vec<ChunkStateInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkStateInfo {
    pub id: u32,
    pub start: u64,
    pub end: u64,
    pub downloaded: u64,
    pub temp_path: PathBuf,
}

pub struct StateManager {
    state_dir: PathBuf,
}

impl StateManager {
    pub fn new(state_dir: PathBuf) -> Self {
        Self { state_dir }
    }
    
    fn state_path(&self, task_id: &str) -> PathBuf {
        self.state_dir.join(format!("{}.json", task_id))
    }
}
```

#### 验收清单
- [ ] 结构定义完整

---

### T3.6.2 状态保存

**时间**: 1h  
**依赖**: T3.6.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 save 方法 | `src/storage/state.rs` | 保存方法 | 保存正确 |
| 2 | 实现 from_chunks 转换 | `src/storage/state.rs` | 转换方法 | 转换正确 |

#### 输出代码

```rust
impl StateManager {
    /// 保存下载状态
    pub async fn save(&self, state: &DownloadState) -> Result<()> {
        tokio::fs::create_dir_all(&self.state_dir).await?;
        
        let path = self.state_path(&state.task_id);
        let json = serde_json::to_string_pretty(state)?;
        
        tokio::fs::write(&path, json).await?;
        Ok(())
    }
}

impl DownloadState {
    pub fn from_chunks(
        task_id: String,
        url: String,
        output_path: PathBuf,
        total_size: u64,
        chunks: &[Chunk],
    ) -> Self {
        let now = Utc::now();
        
        Self {
            task_id,
            url,
            output_path,
            total_size,
            downloaded: chunks.iter().map(|c| c.downloaded).sum(),
            chunks: chunks.iter().map(|c| ChunkStateInfo {
                id: c.id,
                start: c.start,
                end: c.end,
                downloaded: c.downloaded,
                temp_path: c.temp_path.clone(),
            }).collect(),
            created_at: now,
            updated_at: now,
        }
    }
}
```

#### 验收清单
- [ ] 状态保存正确
- [ ] JSON 格式正确

---

### T3.6.3 状态恢复

**时间**: 1h  
**依赖**: T3.6.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 load 方法 | `src/storage/state.rs` | 加载方法 | 加载正确 |
| 2 | 实现 remove 方法 | `src/storage/state.rs` | 删除方法 | 删除正确 |
| 3 | 实现验证方法 | `src/storage/state.rs` | 验证方法 | 验证正确 |

#### 输出代码

```rust
impl StateManager {
    /// 加载下载状态
    pub async fn load(&self, task_id: &str) -> Result<Option<DownloadState>> {
        let path = self.state_path(task_id);
        
        if !path.exists() {
            return Ok(None);
        }
        
        let json = tokio::fs::read_to_string(&path).await?;
        let state: DownloadState = serde_json::from_str(&json)?;
        
        Ok(Some(state))
    }
    
    /// 删除下载状态
    pub async fn remove(&self, task_id: &str) -> Result<()> {
        let path = self.state_path(task_id);
        
        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }
        
        Ok(())
    }
    
    /// 检查是否有未完成的下载
    pub async fn has_pending(&self, task_id: &str) -> bool {
        self.load(task_id).await.ok().flatten().is_some()
    }
}

impl DownloadState {
    /// 更新时间戳
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
    
    /// 获取未完成的分片
    pub fn incomplete_chunks(&self) -> Vec<&ChunkStateInfo> {
        self.chunks
            .iter()
            .filter(|c| c.downloaded < (c.end - c.start))
            .collect()
    }
}
```

#### 验收清单
- [ ] 加载正确
- [ ] 恢复逻辑正确

---

## T3.7 事件系统

### T3.7.1 EventEmitter 实现

**时间**: 1h  
**依赖**: 无

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 创建 event 模块 | `src/event/` | 目录 | 目录存在 |
| 2 | 定义 DownloadEvent | `src/event/emitter.rs` | 枚举 | 定义完整 |
| 3 | 实现 EventEmitter | `src/event/emitter.rs` | 结构 | 定义完整 |

#### 输出文件

**src/event/emitter.rs**:
```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum DownloadEvent {
    #[serde(rename = "started")]
    Started {
        task_id: String,
        total_size: u64,
    },
    
    #[serde(rename = "progress")]
    Progress {
        task_id: String,
        downloaded: u64,
        speed: u64,
        percent: f64,
        eta: Option<u64>,
    },
    
    #[serde(rename = "chunk_completed")]
    ChunkCompleted {
        task_id: String,
        chunk_id: u32,
    },
    
    #[serde(rename = "completed")]
    Completed {
        task_id: String,
        file_path: String,
    },
    
    #[serde(rename = "failed")]
    Failed {
        task_id: String,
        error: String,
    },
    
    #[serde(rename = "paused")]
    Paused {
        task_id: String,
    },
    
    #[serde(rename = "resumed")]
    Resumed {
        task_id: String,
    },
    
    #[serde(rename = "cancelled")]
    Cancelled {
        task_id: String,
    },
}

pub struct EventEmitter {
    task_id: String,
}

impl EventEmitter {
    pub fn new(task_id: String) -> Self {
        Self { task_id }
    }
    
    pub fn started(&self, total_size: u64) -> DownloadEvent {
        DownloadEvent::Started {
            task_id: self.task_id.clone(),
            total_size,
        }
    }
    
    pub fn progress(&self, downloaded: u64, speed: u64, percent: f64, eta: Option<u64>) -> DownloadEvent {
        DownloadEvent::Progress {
            task_id: self.task_id.clone(),
            downloaded,
            speed,
            percent,
            eta,
        }
    }
    
    pub fn completed(&self, file_path: String) -> DownloadEvent {
        DownloadEvent::Completed {
            task_id: self.task_id.clone(),
            file_path,
        }
    }
    
    pub fn failed(&self, error: String) -> DownloadEvent {
        DownloadEvent::Failed {
            task_id: self.task_id.clone(),
            error,
        }
    }
}
```

#### 验收清单
- [ ] 事件定义完整
- [ ] 序列化正确

---

### T3.7.2 Tauri 事件集成

**时间**: 1h  
**依赖**: T3.7.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 添加 Tauri 依赖 | `Cargo.toml` | 依赖 | 依赖添加 |
| 2 | 实现 emit 方法 | `src/event/emitter.rs` | 发送方法 | 发送正确 |

#### 输出代码

```rust
use tauri::AppHandle;
use crate::Result;

impl EventEmitter {
    /// 发送事件到前端
    pub async fn emit(&self, event: DownloadEvent, app_handle: &AppHandle) -> Result<()> {
        app_handle
            .emit("download:event", &event)
            .map_err(|e| crate::DownloadError::EventError(e.to_string()))?;
        Ok(())
    }
}
```

#### 验收清单
- [ ] 事件发送正确
- [ ] 前端可接收

---

## T3.8 进度计算模块

### T3.8.1 进度追踪器

**时间**: 1h  
**依赖**: 无

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 ProgressTracker | `src/progress/tracker.rs` | 结构 | 定义完整 |
| 2 | 实现进度计算 | `src/progress/tracker.rs` | 计算方法 | 计算正确 |

#### 输出文件

**src/progress/tracker.rs**:
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

pub struct ProgressTracker {
    total: u64,
    downloaded: AtomicU64,
    start_time: Instant,
}

impl ProgressTracker {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            downloaded: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }
    
    pub fn add_downloaded(&self, bytes: u64) {
        self.downloaded.fetch_add(bytes, Ordering::Relaxed);
    }
    
    pub fn set_downloaded(&self, bytes: u64) {
        self.downloaded.store(bytes, Ordering::Relaxed);
    }
    
    pub fn downloaded(&self) -> u64 {
        self.downloaded.load(Ordering::Relaxed)
    }
    
    pub fn total(&self) -> u64 {
        self.total
    }
    
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.downloaded() as f64 / self.total as f64) * 100.0
    }
    
    pub fn elapsed_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}
```

#### 验收清单
- [ ] 进度追踪正确
- [ ] 线程安全

---

### T3.8.2 速度计算

**时间**: 1h  
**依赖**: T3.8.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现速度计算 | `src/progress/tracker.rs` | 计算方法 | 计算正确 |
| 2 | 实现 ETA 计算 | `src/progress/tracker.rs` | 计算方法 | 计算正确 |

#### 输出代码

```rust
impl ProgressTracker {
    pub fn speed(&self) -> u64 {
        let elapsed = self.elapsed_secs();
        if elapsed == 0 {
            return 0;
        }
        self.downloaded() / elapsed
    }
    
    pub fn eta(&self) -> Option<u64> {
        let speed = self.speed();
        if speed == 0 {
            return None;
        }
        
        let remaining = self.total.saturating_sub(self.downloaded());
        Some(remaining / speed)
    }
    
    pub fn progress_info(&self) -> ProgressInfo {
        ProgressInfo {
            total: self.total,
            downloaded: self.downloaded(),
            speed: self.speed(),
            percent: self.percent(),
            eta: self.eta(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgressInfo {
    pub total: u64,
    pub downloaded: u64,
    pub speed: u64,
    pub percent: f64,
    pub eta: Option<u64>,
}
```

#### 验收清单
- [ ] 速度计算正确
- [ ] ETA 合理

---

## T3.9 多线程下载整合

### T3.9.1 MultiThreadDownloader 实现

**时间**: 2h  
**依赖**: T3.1-T3.8

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 MultiThreadDownloader | `src/lib.rs` | 结构 | 定义完整 |
| 2 | 实现 new 方法 | `src/lib.rs` | 构造 | 可创建 |
| 3 | 实现 download 方法 | `src/lib.rs` | 下载方法 | 下载正确 |

#### 输出文件

**src/lib.rs (核心部分)**:
```rust
use range::RangeClient;
use chunk::ChunkManager;
use pool::WorkerPool;
use storage::{ChunkWriter, FileMerger, StateManager};
use event::EventEmitter;
use progress::ProgressTracker;

pub struct MultiThreadDownloader {
    config: DownloadConfig,
    client: RangeClient,
    state_manager: StateManager,
}

impl MultiThreadDownloader {
    pub fn new(config: DownloadConfig) -> Result<Self> {
        let client = RangeClient::with_defaults()?;
        let state_manager = StateManager::new(config.state_dir.clone());
        
        Ok(Self {
            config,
            client,
            state_manager,
        })
    }
    
    pub async fn download(&self) -> Result<String> {
        // 1. 检测 Range 支持
        let support = self.client.check_range_support(&self.config.url).await?;
        
        if !support.is_supported() {
            return Err(DownloadError::RangeNotSupported);
        }
        
        let total_size = support.content_length
            .ok_or(DownloadError::ContentLengthUnknown)?;
        
        // 2. 创建分片管理器
        let mut chunk_manager = ChunkManager::new(
            total_size,
            self.config.chunk_size,
            self.config.temp_dir.clone(),
        );
        
        chunk_manager.calculate_chunks(self.config.threads);
        
        // 3. 启动并发下载
        // ... 详见完整实现
        
        Ok(self.config.output_path.to_string_lossy().to_string())
    }
}
```

#### 验收清单
- [ ] 下载器可创建
- [ ] 下载流程正确

---

### T3.9.2 下载流程整合

**时间**: 1h  
**依赖**: T3.9.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 整合并发下载逻辑 | `src/lib.rs` | 整合方法 | 逻辑正确 |
| 2 | 整合进度更新 | `src/lib.rs` | 更新逻辑 | 更新正确 |

#### 输出代码

```rust
impl MultiThreadDownloader {
    pub async fn download(&self) -> Result<String> {
        // ... 前面的步骤
        
        // 4. 创建进度追踪器
        let tracker = Arc::new(ProgressTracker::new(total_size));
        
        // 5. 创建线程池
        let pool = WorkerPool::new(self.config.threads as usize);
        
        // 6. 并发下载分片
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(100);
        
        let mut handles = Vec::new();
        
        while let Some(chunk) = chunk_manager.get_next_pending() {
            let worker = self.create_worker(chunk.clone()).await?;
            let tx = progress_tx.clone();
            
            handles.push(pool.spawn(async move {
                worker.download_with_retry(tx, self.config.retry_count).await
            }));
        }
        
        // 7. 等待所有任务完成
        let results = WorkerPool::wait_all(handles).await;
        
        // 8. 合并文件
        FileMerger::merge(chunk_manager.chunks(), &self.config.output_path).await?;
        
        // 9. 清理临时文件
        ChunkWriter::cleanup(&self.config.temp_dir).await?;
        
        Ok(self.config.output_path.to_string_lossy().to_string())
    }
}
```

#### 验收清单
- [ ] 下载流程完整
- [ ] 错误处理完善

---

### T3.9.3 暂停/恢复功能

**时间**: 1h  
**依赖**: T3.9.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 pause 方法 | `src/lib.rs` | 暂停方法 | 暂停正确 |
| 2 | 实现 resume 方法 | `src/lib.rs` | 恢复方法 | 恢复正确 |

#### 输出代码

```rust
impl MultiThreadDownloader {
    pub async fn pause(&self) -> Result<()> {
        // 保存当前状态
        let state = self.create_state().await?;
        self.state_manager.save(&state).await?;
        
        // 取消所有下载任务
        self.cancel_token.cancel();
        
        Ok(())
    }
    
    pub async fn resume(&self) -> Result<String> {
        // 加载保存的状态
        let state = self.state_manager.load(&self.config.task_id).await?
            .ok_or(DownloadError::StateNotFound)?;
        
        // 恢复下载
        self.restore_from_state(state).await?;
        
        self.download().await
    }
}
```

#### 验收清单
- [ ] 暂停功能正常
- [ ] 恢复功能正常

---

## T3.10 Tauri 命令集成

### T3.10.1 命令定义

**时间**: 1h  
**依赖**: T3.9

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Tauri 命令 | `src/commands.rs` | 命令函数 | 定义完整 |
| 2 | 注册命令 | `src/lib.rs` | 注册 | 注册正确 |

#### 输出文件

**src/commands.rs**:
```rust
use tauri::AppHandle;
use crate::{DownloadConfig, MultiThreadDownloader, DownloadProgress};

#[tauri::command]
pub async fn start_download(
    config: DownloadConfig,
    app_handle: AppHandle,
) -> Result<String, String> {
    let downloader = MultiThreadDownloader::new(config)
        .map_err(|e| e.to_string())?;
    
    downloader.download().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pause_download(task_id: String) -> Result<(), String> {
    // 实现暂停逻辑
    Ok(())
}

#[tauri::command]
pub async fn resume_download(task_id: String) -> Result<(), String> {
    // 实现恢复逻辑
    Ok(())
}

#[tauri::command]
pub async fn cancel_download(task_id: String) -> Result<(), String> {
    // 实现取消逻辑
    Ok(())
}

#[tauri::command]
pub async fn get_download_progress(task_id: String) -> Result<DownloadProgress, String> {
    // 实现进度查询
    Err("Not implemented".to_string())
}
```

#### 验收清单
- [ ] 命令定义完整
- [ ] 命令注册正确

---

### T3.10.2 前端事件监听

**时间**: 1h  
**依赖**: T3.10.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 编写前端监听示例 | 文档 | TypeScript | 示例完整 |

#### 输出代码

**前端监听示例**:
```typescript
import { listen } from '@tauri-apps/api/event';

// 监听下载事件
const unlisten = await listen('download:event', (event) => {
  const data = event.payload;
  
  switch (data.type) {
    case 'started':
      console.log('下载开始:', data.total_size);
      break;
    case 'progress':
      console.log('下载进度:', data.percent.toFixed(2) + '%');
      break;
    case 'completed':
      console.log('下载完成:', data.file_path);
      break;
    case 'failed':
      console.error('下载失败:', data.error);
      break;
  }
});

// 取消监听
unlisten();
```

#### 验收清单
- [ ] 事件监听正常
- [ ] 数据格式正确

---

## T3.11 测试与文档

### T3.11.1 单元测试

**时间**: 1.5h  
**依赖**: T3.1-T3.10

#### 验收清单
- [ ] Range 请求测试通过
- [ ] 分片计算测试通过
- [ ] 状态持久化测试通过
- [ ] 进度计算测试通过

---

### T3.11.2 集成测试

**时间**: 1.5h  
**依赖**: T3.11.1

#### 验收清单
- [ ] 完整下载流程测试通过
- [ ] 断点续传测试通过
- [ ] 多线程并发测试通过
- [ ] 错误恢复测试通过

---

## 任务依赖关系图

```
T3.1 ──┬── T3.2 ─── T3.4 ──┐
        │                    │
        └── T3.3 ────────────┤
                             │
T3.5 ────────────────────────┤
                             │
T3.6 ────────────────────────┤
                             │
T3.7 ────────────────────────┤
                             │
T3.8 ────────────────────────┤
                             │
T3.9 ◀───────────────────────┘
 │
 ├── T3.10
 │
 └── T3.11
```

---

## 工时汇总

| 任务 | 时间 | 并行机会 |
|------|------|----------|
| T3.1 Range 请求模块 | 3h | - |
| T3.2 分片管理模块 | 3h | 与 T3.3 并行 |
| T3.3 线程池模块 | 2h | 与 T3.2 并行 |
| T3.4 分片下载 Worker | 3h | - |
| T3.5 分片存储模块 | 3h | 与 T3.6 并行 |
| T3.6 状态持久化模块 | 3h | 与 T3.5 并行 |
| T3.7 事件系统 | 2h | 与 T3.8 并行 |
| T3.8 进度计算模块 | 2h | 与 T3.7 并行 |
| T3.9 多线程下载整合 | 4h | - |
| T3.10 Tauri 命令集成 | 2h | - |
| T3.11 测试与文档 | 3h | - |
| **总计** | **30h** | **优化后约 24h** |

---

*任务链规划版本: v1.0*
*规划日期: 2026-03-27*