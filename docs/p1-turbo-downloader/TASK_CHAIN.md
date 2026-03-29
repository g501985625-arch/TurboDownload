# P1: turbo-downloader 详细任务链

## 任务概览

| 任务编号 | 任务名称 | 预估时间 | 依赖任务 |
|----------|----------|----------|----------|
| T1.1 | 项目初始化 | 2h | 无 |
| T1.2 | HTTP 客户端封装 | 3h | T1.1 |
| T1.3 | 多线程下载核心 | 8h | T1.2 |
| T1.4 | 断点续传 | 5h | T1.3 |
| T1.5 | 进度回调 | 4h | T1.3 |
| T1.6 | 错误处理与重试 | 3h | T1.2 |
| T1.7 | 测试与优化 | 8h | T1.1-T1.6 |
| T1.8 | 文档与示例 | 3h | T1.7 |

**总工时**: 36h (约 4.5 个工作日)

---

## T1.1: 项目初始化

### T1.1.1: 创建 Rust crate 结构

**时间**: 0.5h  
**依赖**: 无

#### 步骤

1. **创建项目目录**
   ```bash
   cd ~/Projects/TurboDownload
   mkdir -p crates/turbo-downloader
   cd crates/turbo-downloader
   cargo init --lib
   ```

2. **创建模块文件**
   ```bash
   mkdir -p src/{http,chunk,download,progress,resume,error}
   touch src/http/{mod.rs,client.rs,response.rs}
   touch src/chunk/{mod.rs,strategy.rs,worker.rs}
   touch src/download/{mod.rs,task.rs,manager.rs}
   touch src/progress/{mod.rs,tracker.rs,speed.rs}
   touch src/resume/{mod.rs,state.rs,recovery.rs}
   touch src/error/{mod.rs,types.rs}
   ```

3. **配置 lib.rs 入口**
   ```rust
   //! Turbo Downloader - High-performance multi-threaded download engine
   //!
   //! # Features
   //! - Multi-threaded chunk downloads
   //! - Resume support
   //! - Progress callbacks
   //! - Speed calculation

   pub mod error;
   pub mod http;
   pub mod chunk;
   pub mod download;
   pub mod progress;
   pub mod resume;

   pub use error::{DownloadError, Result};
   pub use download::{DownloadConfig, DownloadResult, Downloader, DownloaderBuilder};
   pub use progress::{DownloadProgress, ProgressCallback};
   ```

#### 验收标准

- [ ] `cargo check` 通过
- [ ] 目录结构符合规范
- [ ] 模块导入无错误

---

### T1.1.2: 配置 Cargo.toml 依赖

**时间**: 0.5h  
**依赖**: T1.1.1

#### 步骤

1. **编辑 Cargo.toml**
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

### T1.1.3: 创建测试目录结构

**时间**: 0.5h  
**依赖**: T1.1.1

#### 步骤

1. **创建测试目录**
   ```bash
   mkdir -p tests
   touch tests/{mod.rs,http_test.rs,chunk_test.rs,download_test.rs,resume_test.rs}
   ```

2. **创建测试框架**
   ```rust
   // tests/mod.rs
   pub mod http_test;
   pub mod chunk_test;
   pub mod download_test;
   pub mod resume_test;

   pub fn setup_test_env() {
       // 初始化测试环境
       let _ = tracing_subscriber::fmt::try_init();
   }
   ```

3. **创建测试工具**
   ```rust
   // tests/common/mod.rs
   use wiremock::{MockServer, Mock, ResponseTemplate};
   use wiremock::matchers::{method, path};

   pub async fn start_mock_server() -> MockServer {
       MockServer::start().await
   }

   pub fn mock_file_response(server: &MockServer, path: &str, size: u64) {
       // 配置 mock 响应
   }
   ```

#### 验收标准

- [ ] 测试目录结构完整
- [ ] 测试框架可运行
- [ ] Mock 服务器可用

---

### T1.1.4: 配置开发工具

**时间**: 0.5h  
**依赖**: T1.1.1

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

3. **创建 Justfile (可选)**
   ```just
   check:
       cargo check --all-targets

   test:
       cargo nextest run

   lint:
       cargo fmt -- --check
       cargo clippy -- -D warnings

   ci: lint test
   ```

#### 验收标准

- [ ] `cargo fmt` 格式化正确
- [ ] `cargo clippy` 无警告
- [ ] 开发工具链配置完成

---

## T1.2: HTTP 客户端封装

### T1.2.1: 设计 HttpClient 结构体

**时间**: 1h  
**依赖**: T1.1

#### 步骤

1. **定义配置结构**
   ```rust
   // src/http/client.rs
   use std::time::Duration;
   use std::collections::HashMap;

   /// HTTP 客户端配置
   #[derive(Debug, Clone)]
   pub struct HttpClientConfig {
       /// 请求超时时间
       pub timeout: Duration,
       /// 连接超时时间
       pub connect_timeout: Duration,
       /// 用户代理
       pub user_agent: String,
       /// 代理配置
       pub proxy: Option<ProxyConfig>,
       /// 默认请求头
       pub default_headers: HashMap<String, String>,
       /// 最大重定向次数
       pub max_redirects: usize,
   }

   /// 代理配置
   #[derive(Debug, Clone)]
   pub struct ProxyConfig {
       pub http: Option<String>,
       pub https: Option<String>,
   }

   impl Default for HttpClientConfig {
       fn default() -> Self {
           Self {
               timeout: Duration::from_secs(300),
               connect_timeout: Duration::from_secs(30),
               user_agent: format!("TurboDownloader/{}", env!("CARGO_PKG_VERSION")),
               proxy: None,
               default_headers: HashMap::new(),
               max_redirects: 10,
           }
       }
   }
   ```

2. **定义客户端结构**
   ```rust
   /// HTTP 客户端
   #[derive(Debug, Clone)]
   pub struct HttpClient {
       client: reqwest::Client,
       config: HttpClientConfig,
   }

   impl HttpClient {
       /// 创建新的 HTTP 客户端
       pub fn new(config: HttpClientConfig) -> Result<Self, DownloadError> {
           let mut builder = reqwest::Client::builder()
               .timeout(config.timeout)
               .connect_timeout(config.connect_timeout)
               .user_agent(&config.user_agent)
               .redirect(reqwest::redirect::Policy::limited(config.max_redirects));

           // 配置代理
           if let Some(proxy_config) = &config.proxy {
               if let Some(http_proxy) = &proxy_config.http {
                   builder = builder.proxy(reqwest::Proxy::http(http_proxy)?);
               }
               if let Some(https_proxy) = &proxy_config.https {
                   builder = builder.proxy(reqwest::Proxy::https(https_proxy)?);
               }
           }

           // 添加默认请求头
           for (key, value) in &config.default_headers {
               builder = builder.header(key, value);
           }

           let client = builder.build()?;

           Ok(Self { client, config })
       }

       /// 获取配置
       pub fn config(&self) -> &HttpClientConfig {
           &self.config
       }
   }
   ```

3. **定义构建器**
   ```rust
   /// HTTP 客户端构建器
   pub struct HttpClientBuilder {
       config: HttpClientConfig,
   }

   impl HttpClientBuilder {
       pub fn new() -> Self {
           Self {
               config: HttpClientConfig::default(),
           }
       }

       pub fn timeout(mut self, timeout: Duration) -> Self {
           self.config.timeout = timeout;
           self
       }

       pub fn connect_timeout(mut self, timeout: Duration) -> Self {
           self.config.connect_timeout = timeout;
           self
       }

       pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
           self.config.user_agent = user_agent.into();
           self
       }

       pub fn proxy(mut self, proxy: ProxyConfig) -> Self {
           self.config.proxy = Some(proxy);
           self
       }

       pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
           self.config.default_headers.insert(key.into(), value.into());
           self
       }

       pub fn build(self) -> Result<HttpClient, DownloadError> {
           HttpClient::new(self.config)
       }
   }

   impl Default for HttpClientBuilder {
       fn default() -> Self {
           Self::new()
       }
   }
   ```

#### 验收标准

- [ ] HttpClient 可正常创建
- [ ] 配置选项全部可用
- [ ] 单元测试覆盖配置构建

---

### T1.2.2: 实现 HEAD 请求方法

**时间**: 1h  
**依赖**: T1.2.1

#### 步骤

1. **定义响应结构**
   ```rust
   // src/http/response.rs
   use serde::{Deserialize, Serialize};

   /// HEAD 请求响应
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct HeadResponse {
       /// 文件大小 (Content-Length)
       pub content_length: Option<u64>,
       /// 是否支持 Range 请求
       pub accept_ranges: bool,
       /// ETag 标识
       pub etag: Option<String>,
       /// 最后修改时间
       pub last_modified: Option<String>,
       /// 内容类型
       pub content_type: Option<String>,
       /// 文件名 (从 Content-Disposition 解析)
       pub filename: Option<String>,
       /// 支持的 HTTP 方法
       pub allow: Option<Vec<String>>,
   }
   ```

2. **实现 HEAD 方法**
   ```rust
   impl HttpClient {
       /// 发送 HEAD 请求获取文件信息
       pub async fn head(&self, url: &str) -> Result<HeadResponse, DownloadError> {
           let response = self.client
               .head(url)
               .send()
               .await
               .map_err(|e| DownloadError::Network(e.to_string()))?;

           let status = response.status();
           if !status.is_success() {
               return Err(DownloadError::ServerError(
                   status.as_u16(),
                   status.to_string(),
               ));
           }

           let headers = response.headers();

           Ok(HeadResponse {
               content_length: headers
                   .get(reqwest::header::CONTENT_LENGTH)
                   .and_then(|v| v.to_str().ok())
                   .and_then(|v| v.parse().ok()),
               accept_ranges: headers
                   .get(reqwest::header::ACCEPT_RANGES)
                   .map(|v| v.to_str().unwrap_or("") == "bytes")
                   .unwrap_or(false),
               etag: headers
                   .get(reqwest::header::ETAG)
                   .and_then(|v| v.to_str().ok())
                   .map(|s| s.to_string()),
               last_modified: headers
                   .get(reqwest::header::LAST_MODIFIED)
                   .and_then(|v| v.to_str().ok())
                   .map(|s| s.to_string()),
               content_type: headers
                   .get(reqwest::header::CONTENT_TYPE)
                   .and_then(|v| v.to_str().ok())
                   .map(|s| s.to_string()),
               filename: Self::parse_filename(headers),
               allow: headers
                   .get(reqwest::header::ALLOW)
                   .and_then(|v| v.to_str().ok())
                   .map(|s| s.split(',').map(|m| m.trim().to_string()).collect()),
           })
       }

       /// 从 Content-Disposition 解析文件名
       fn parse_filename(headers: &reqwest::header::HeaderMap) -> Option<String> {
           let disposition = headers
               .get(reqwest::header::CONTENT_DISPOSITION)?
               .to_str().ok()?;

           // 解析 filename="xxx" 或 filename=xxx
           for part in disposition.split(';') {
               let part = part.trim();
               if part.starts_with("filename=") {
                   let filename = part.strip_prefix("filename=")?;
                   let filename = filename.trim_matches('"');
                   return Some(filename.to_string());
               }
           }
           None
       }
   }
   ```

#### 验收标准

- [ ] 正确解析 Content-Length
- [ ] 正确解析 Accept-Ranges
- [ ] 正确解析 ETag 和 Last-Modified
- [ ] 处理缺失头部的情况
- [ ] 单元测试覆盖

---

### T1.2.3: 实现 GET 和 Range 请求

**时间**: 1.5h  
**依赖**: T1.2.2

#### 步骤

1. **实现完整 GET 请求**
   ```rust
   impl HttpClient {
       /// 发送 GET 请求
       pub async fn get(&self, url: &str) -> Result<reqwest::Response, DownloadError> {
           let response = self.client
               .get(url)
               .send()
               .await
               .map_err(|e| DownloadError::Network(e.to_string()))?;

           let status = response.status();
           if !status.is_success() {
               return Err(DownloadError::ServerError(
                   status.as_u16(),
                   status.to_string(),
               ));
           }

           Ok(response)
       }
   }
   ```

2. **实现 Range 请求**
   ```rust
   impl HttpClient {
       /// 发送 Range 请求
       pub async fn get_range(
           &self,
           url: &str,
           start: u64,
           end: u64,
       ) -> Result<reqwest::Response, DownloadError> {
           let range_header = format!("bytes={}-{}", start, end);

           let response = self.client
               .get(url)
               .header(reqwest::header::RANGE, range_header)
               .send()
               .await
               .map_err(|e| DownloadError::Network(e.to_string()))?;

           let status = response.status();
           if status != reqwest::StatusCode::PARTIAL_CONTENT {
               if status == reqwest::StatusCode::OK {
                   // 服务器不支持 Range，返回完整内容
                   return Err(DownloadError::ResumeNotSupported);
               }
               return Err(DownloadError::ServerError(
                   status.as_u16(),
                   status.to_string(),
               ));
           }

           Ok(response)
       }
   }
   ```

3. **添加自定义请求头支持**
   ```rust
   impl HttpClient {
       /// 发送带自定义头的 GET 请求
       pub async fn get_with_headers(
           &self,
           url: &str,
           headers: HashMap<String, String>,
       ) -> Result<reqwest::Response, DownloadError> {
           let mut request = self.client.get(url);

           for (key, value) in headers {
               request = request.header(&key, &value);
           }

           let response = request
               .send()
               .await
               .map_err(|e| DownloadError::Network(e.to_string()))?;

           let status = response.status();
           if !status.is_success() {
               return Err(DownloadError::ServerError(
                   status.as_u16(),
                   status.to_string(),
               ));
           }

           Ok(response)
       }
   }
   ```

#### 验收标准

- [ ] GET 请求正常工作
- [ ] Range 请求返回 206
- [ ] 自定义请求头支持
- [ ] 错误状态码处理
- [ ] 集成测试覆盖

---

## T1.3: 多线程下载核心

### T1.3.1: 设计分片策略

**时间**: 2h  
**依赖**: T1.2

#### 步骤

1. **定义分片数据结构**
   ```rust
   // src/chunk/strategy.rs
   use serde::{Deserialize, Serialize};

   /// 分片范围
   #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
   pub struct ChunkRange {
       /// 分片索引
       pub index: usize,
       /// 起始字节
       pub start: u64,
       /// 结束字节 (包含)
       pub end: u64,
   }

   impl ChunkRange {
       /// 分片大小
       pub fn size(&self) -> u64 {
           self.end - self.start + 1
       }
   }

   /// 分片策略配置
   #[derive(Debug, Clone)]
   pub struct ChunkStrategy {
       /// 最小分片大小 (字节)
       pub min_chunk_size: u64,
       /// 最大分片数
       pub max_chunks: usize,
       /// 默认分片大小
       pub default_chunk_size: u64,
   }

   impl Default for ChunkStrategy {
       fn default() -> Self {
           Self {
               min_chunk_size: 1024 * 1024, // 1MB
               max_chunks: 16,
               default_chunk_size: 10 * 1024 * 1024, // 10MB
           }
       }
   }
   ```

2. **实现分片计算算法**
   ```rust
   impl ChunkStrategy {
       /// 计算分片方案
       ///
       /// # Arguments
       /// * `file_size` - 文件总大小
       /// * `threads` - 线程数
       /// * `supports_range` - 服务器是否支持 Range
       pub fn calculate(
           &self,
           file_size: u64,
           threads: usize,
           supports_range: bool,
       ) -> Vec<ChunkRange> {
           // 不支持 Range 或文件太小，使用单线程
           if !supports_range || file_size < self.min_chunk_size {
               return vec![ChunkRange {
                   index: 0,
                   start: 0,
                   end: file_size.saturating_sub(1),
               }];
           }

           // 计算最优分片数
           let ideal_chunks = (file_size / self.min_chunk_size).min(self.max_chunks as u64);
           let actual_chunks = ideal_chunks.max(1) as usize;
           let actual_threads = threads.min(actual_chunks);

           // 计算每个分片的大小
           let chunk_size = file_size / actual_threads as u64;

           // 生成分片范围
           (0..actual_threads)
               .map(|i| {
                   let start = i as u64 * chunk_size;
                   let end = if i == actual_threads - 1 {
                       file_size - 1
                   } else {
                       start + chunk_size - 1
                   };
                   ChunkRange {
                       index: i,
                       start,
                       end,
                   }
               })
               .collect()
       }
   }
   ```

3. **添加验证方法**
   ```rust
   impl ChunkStrategy {
       /// 验证分片方案
       pub fn validate(chunks: &[ChunkRange], file_size: u64) -> bool {
           if chunks.is_empty() {
               return false;
           }

           // 检查分片是否覆盖整个文件
           let mut covered = vec![false; file_size as usize];
           for chunk in chunks {
               for i in chunk.start..=chunk.end {
                   if i >= file_size {
                       return false;
                   }
                   let idx = i as usize;
                   if covered[idx] {
                       return false; // 重叠
                   }
                   covered[idx] = true;
               }
           }

           covered.iter().all(|&b| b)
       }
   }
   ```

#### 验收标准

- [ ] 正确处理不支持 Range 的情况
- [ ] 分片大小计算合理
- [ ] 边界情况处理正确
- [ ] 单元测试覆盖率 > 90%

---

### T1.3.2: 实现单分片下载

**时间**: 2h  
**依赖**: T1.3.1

#### 步骤

1. **定义分片下载器**
   ```rust
   // src/chunk/worker.rs
   use std::fs::File;
   use std::io::{Seek, SeekFrom, Write};
   use std::sync::Arc;
   use tokio::sync::Mutex;
   use crate::http::HttpClient;
   use crate::progress::ProgressTracker;
   use super::ChunkRange;

   /// 分片下载器
   pub struct ChunkWorker {
       http_client: Arc<HttpClient>,
       range: ChunkRange,
       output_file: Arc<Mutex<File>>,
       progress: Arc<ProgressTracker>,
   }

   /// 分片下载结果
   #[derive(Debug)]
   pub struct ChunkDownloadResult {
       pub range: ChunkRange,
       pub bytes_downloaded: u64,
       pub duration_ms: u64,
   }
   ```

2. **实现下载逻辑**
   ```rust
   impl ChunkWorker {
       pub fn new(
           http_client: Arc<HttpClient>,
           range: ChunkRange,
           output_file: Arc<Mutex<File>>,
           progress: Arc<ProgressTracker>,
       ) -> Self {
           Self {
               http_client,
               range,
               output_file,
               progress,
           }
       }

       /// 执行分片下载
       pub async fn download(
           &self,
           url: &str,
       ) -> Result<ChunkDownloadResult, DownloadError> {
           let start_time = std::time::Instant::now();

           // 发送 Range 请求
           let response = self.http_client
               .get_range(url, self.range.start, self.range.end)
               .await?;

           // 流式读取响应
           let mut stream = response.bytes_stream();
           let mut bytes_downloaded = 0u64;

           {
               let mut file = self.output_file.lock().await;
               file.seek(SeekFrom::Start(self.range.start))?;

               while let Some(chunk) = stream.next().await {
                   let data = chunk.map_err(|e| {
                       DownloadError::Network(e.to_string())
                   })?;

                   file.write_all(&data)?;
                   bytes_downloaded += data.len() as u64;

                   // 更新进度
                   self.progress.update_chunk(
                       self.range.index,
                       bytes_downloaded,
                   );
               }
           }

           Ok(ChunkDownloadResult {
               range: self.range,
               bytes_downloaded,
               duration_ms: start_time.elapsed().as_millis() as u64,
           })
       }
   }
   ```

3. **添加取消支持**
   ```rust
   use std::sync::atomic::{AtomicBool, Ordering};

   impl ChunkWorker {
       /// 带取消支持的下载
       pub async fn download_with_cancel(
           &self,
           url: &str,
           cancel_flag: Arc<AtomicBool>,
       ) -> Result<ChunkDownloadResult, DownloadError> {
           let start_time = std::time::Instant::now();
           let response = self.http_client
               .get_range(url, self.range.start, self.range.end)
               .await?;

           let mut stream = response.bytes_stream();
           let mut bytes_downloaded = 0u64;

           {
               let mut file = self.output_file.lock().await;
               file.seek(SeekFrom::Start(self.range.start))?;

               while let Some(chunk) = stream.next().await {
                   // 检查取消标志
                   if cancel_flag.load(Ordering::Relaxed) {
                       return Err(DownloadError::Cancelled);
                   }

                   let data = chunk.map_err(|e| {
                       DownloadError::Network(e.to_string())
                   })?;

                   file.write_all(&data)?;
                   bytes_downloaded += data.len() as u64;
                   self.progress.update_chunk(
                       self.range.index,
                       bytes_downloaded,
                   );
               }
           }

           Ok(ChunkDownloadResult {
               range: self.range,
               bytes_downloaded,
               duration_ms: start_time.elapsed().as_millis() as u64,
           })
       }
   }
   ```

#### 验收标准

- [ ] Range 请求正确发送
- [ ] 数据写入正确位置
- [ ] 进度更新正常
- [ ] 取消功能正常
- [ ] 单元测试覆盖

---

### T1.3.3: 实现多分片并行下载

**时间**: 3h  
**依赖**: T1.3.2

#### 步骤

1. **定义并行下载器**
   ```rust
   // src/download/manager.rs
   use std::path::PathBuf;
   use std::sync::Arc;
   use std::collections::HashMap;
   use tokio::sync::Mutex;
   use tokio::task::JoinSet;
   use crate::http::HttpClient;
   use crate::chunk::{ChunkRange, ChunkWorker, ChunkStrategy};
   use crate::progress::ProgressTracker;
   use crate::resume::ResumeState;

   /// 并行下载管理器
   pub struct ParallelDownloader {
       http_client: Arc<HttpClient>,
       chunk_strategy: ChunkStrategy,
   }

   /// 下载配置
   #[derive(Debug, Clone)]
   pub struct DownloadConfig {
       pub id: String,
       pub url: String,
       pub output_path: PathBuf,
       pub threads: usize,
       pub chunk_size: u64,
       pub resume_support: bool,
       pub user_agent: Option<String>,
       pub headers: HashMap<String, String>,
       pub speed_limit: u64,
   }
   ```

2. **实现并行下载**
   ```rust
   impl ParallelDownloader {
       pub fn new(http_client: Arc<HttpClient>) -> Self {
           Self {
               http_client,
               chunk_strategy: ChunkStrategy::default(),
           }
       }

       /// 执行并行下载
       pub async fn download(
           &self,
           config: DownloadConfig,
           progress: Arc<ProgressTracker>,
           cancel_flag: Arc<AtomicBool>,
       ) -> Result<DownloadResult, DownloadError> {
           // 1. 获取文件信息
           let head = self.http_client.head(&config.url).await?;
           let file_size = head.content_length.ok_or(
               DownloadError::InvalidUrl("Cannot determine file size".into())
           )?;

           // 2. 计算分片
           let chunks = self.chunk_strategy.calculate(
               file_size,
               config.threads,
               head.accept_ranges,
           );

           // 3. 创建输出文件
           std::fs::create_dir_all(config.output_path.parent().unwrap())?;
           let file = File::create(&config.output_path)?;
           file.set_len(file_size)?;
           let file = Arc::new(Mutex::new(file));

           // 4. 初始化进度
           progress.init(&chunks, file_size);

           // 5. 并行下载
           let mut tasks = JoinSet::new();

           for chunk in chunks {
               let worker = ChunkWorker::new(
                   self.http_client.clone(),
                   chunk,
                   file.clone(),
                   progress.clone(),
               );
               let url = config.url.clone();
               let cancel = cancel_flag.clone();

               tasks.spawn(async move {
                   worker.download_with_cancel(&url, cancel).await
               });
           }

           // 6. 等待所有任务完成
           let mut results = Vec::new();
           while let Some(result) = tasks.join_next().await {
               match result {
                   Ok(Ok(r)) => results.push(r),
                   Ok(Err(e)) => {
                       cancel_flag.store(true, Ordering::Relaxed);
                       return Err(e);
                   }
                   Err(e) => {
                       return Err(DownloadError::Internal(e.to_string()));
                   }
               }
           }

           // 7. 返回结果
           Ok(DownloadResult {
               task_id: config.id,
               output_path: config.output_path,
               file_size,
               duration_ms: 0, // 计算总耗时
               avg_speed: 0,   // 计算平均速度
           })
       }
   }
   ```

3. **添加重试逻辑**
   ```rust
   impl ParallelDownloader {
       /// 带重试的下载
       pub async fn download_with_retry(
           &self,
           config: DownloadConfig,
           progress: Arc<ProgressTracker>,
           cancel_flag: Arc<AtomicBool>,
           max_retries: usize,
       ) -> Result<DownloadResult, DownloadError> {
           let mut last_error = None;

           for attempt in 0..=max_retries {
               if attempt > 0 {
                   // 等待后重试
                   tokio::time::sleep(Duration::from_secs(2u64.pow(attempt as u32))).await;
               }

               match self.download(config.clone(), progress.clone(), cancel_flag.clone()).await {
                   Ok(result) => return Ok(result),
                   Err(e) => {
                       match &e {
                           DownloadError::Cancelled => return Err(e),
                           DownloadError::ServerError(code, _) if *code >= 500 => {
                               last_error = Some(e);
                               continue;
                           }
                           _ => return Err(e),
                       }
                   }
               }
           }

           Err(last_error.unwrap_or(DownloadError::Internal("Max retries exceeded".into())))
       }
   }
   ```

#### 验收标准

- [ ] 多线程并行下载正常
- [ ] 进度汇总正确
- [ ] 任务取消正常
- [ ] 重试逻辑正确
- [ ] 集成测试覆盖

---

### T1.3.4: 实现分片合并与清理

**时间**: 1h  
**依赖**: T1.3.3

#### 步骤

1. **定义临时文件管理**
   ```rust
   // src/download/temp.rs
   use std::path::{Path, PathBuf};

   /// 临时文件管理器
   pub struct TempFileManager {
       base_dir: PathBuf,
       task_id: String,
   }

   impl TempFileManager {
       pub fn new(base_dir: PathBuf, task_id: String) -> Self {
           Self { base_dir, task_id }
       }

       /// 获取分片临时文件路径
       pub fn chunk_path(&self, index: usize) -> PathBuf {
           self.base_dir
               .join(&self.task_id)
               .join(format!("chunk_{}.tmp", index))
       }

       /// 获取状态文件路径
       pub fn state_path(&self) -> PathBuf {
           self.base_dir
               .join(&self.task_id)
               .join("state.json")
       }

       /// 清理所有临时文件
       pub fn cleanup(&self) -> std::io::Result<()> {
           let dir = self.base_dir.join(&self.task_id);
           if dir.exists() {
               std::fs::remove_dir_all(dir)?;
           }
           Ok(())
       }
   }
   ```

2. **实现合并逻辑 (如需要)**
   ```rust
   impl TempFileManager {
       /// 合并分片文件
       pub fn merge_chunks(
           &self,
           output_path: &Path,
           total_chunks: usize,
       ) -> std::io::Result<()> {
           let mut output = File::create(output_path)?;

           for i in 0..total_chunks {
               let chunk_path = self.chunk_path(i);
               if chunk_path.exists() {
                   let data = std::fs::read(&chunk_path)?;
                   output.write_all(&data)?;
               }
           }

           output.sync_all()?;
           self.cleanup()?;

           Ok(())
       }
   }
   ```

#### 验收标准

- [ ] 临时文件正确命名
- [ ] 合并逻辑正确
- [ ] 清理完整
- [ ] 异常情况下清理

---

## T1.4: 断点续传

### T1.4.1: 设计状态持久化格式

**时间**: 1h  
**依赖**: T1.3

#### 步骤

1. **定义状态结构**
   ```rust
   // src/resume/state.rs
   use serde::{Deserialize, Serialize};
   use chrono::{DateTime, Utc};

   /// 下载状态
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct ResumeState {
       /// 版本号
       pub version: u32,
       /// 任务 ID
       pub task_id: String,
       /// 下载 URL
       pub url: String,
       /// 输出路径
       pub output_path: String,
       /// 文件总大小
       pub file_size: u64,
       /// 分片状态
       pub chunks: Vec<ChunkState>,
       /// 文件标识
       pub file_identity: FileIdentity,
       /// 创建时间
       pub created_at: DateTime<Utc>,
       /// 更新时间
       pub updated_at: DateTime<Utc>,
   }

   /// 分片状态
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct ChunkState {
       pub index: usize,
       pub start: u64,
       pub end: u64,
       pub downloaded: u64,
       pub completed: bool,
   }

   /// 文件标识 (用于验证)
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct FileIdentity {
       pub etag: Option<String>,
       pub last_modified: Option<String>,
       pub content_length: u64,
   }
   ```

2. **定义 JSON 序列化格式**
   ```json
   {
     "version": 1,
     "task_id": "550e8400-e29b-41d4-a716-446655440000",
     "url": "https://example.com/file.zip",
     "output_path": "/downloads/file.zip",
     "file_size": 104857600,
     "chunks": [
       {
         "index": 0,
         "start": 0,
         "end": 52428799,
         "downloaded": 10485760,
         "completed": false
       },
       {
         "index": 1,
         "start": 52428800,
         "end": 104857599,
         "downloaded": 0,
         "completed": false
       }
     ],
     "file_identity": {
       "etag": "\"abc123\"",
       "last_modified": "Wed, 25 Mar 2026 10:00:00 GMT",
       "content_length": 104857600
     },
     "created_at": "2026-03-25T10:00:00Z",
     "updated_at": "2026-03-25T11:30:00Z"
   }
   ```

#### 验收标准

- [ ] 状态结构完整
- [ ] 序列化/反序列化正确
- [ ] 向后兼容性设计

---

### T1.4.2: 实现状态保存与加载

**时间**: 2h  
**依赖**: T1.4.1

#### 步骤

1. **实现状态管理器**
   ```rust
   // src/resume/manager.rs
   use std::path::{Path, PathBuf};
   use std::io::Write;
   use tokio::fs;

   /// 状态管理器
   pub struct StateManager {
       state_dir: PathBuf,
   }

   impl StateManager {
       pub fn new(state_dir: PathBuf) -> Self {
           Self { state_dir }
       }

       /// 保存状态
       pub async fn save(&self, state: &ResumeState) -> Result<(), DownloadError> {
           fs::create_dir_all(&self.state_dir).await?;

           let path = self.state_path(&state.task_id);
           let json = serde_json::to_string_pretty(state)?;

           // 原子写入: 先写临时文件，再重命名
           let temp_path = path.with_extension("tmp");
           fs::write(&temp_path, json).await?;
           fs::rename(&temp_path, &path).await?;

           Ok(())
       }

       /// 加载状态
       pub async fn load(&self, task_id: &str) -> Result<Option<ResumeState>, DownloadError> {
           let path = self.state_path(task_id);

           if !path.exists() {
               return Ok(None);
           }

           let content = fs::read_to_string(&path).await?;
           let state: ResumeState = serde_json::from_str(&content)?;

           Ok(Some(state))
       }

       /// 删除状态
       pub async fn delete(&self, task_id: &str) -> Result<(), DownloadError> {
           let path = self.state_path(task_id);
           if path.exists() {
               fs::remove_file(&path).await?;
           }
           Ok(())
       }

       fn state_path(&self, task_id: &str) -> PathBuf {
           self.state_dir.join(format!("{}.json", task_id))
       }
   }
   ```

2. **实现状态更新**
   ```rust
   impl StateManager {
       /// 更新分片进度
       pub async fn update_chunk_progress(
           &self,
           task_id: &str,
           chunk_index: usize,
           downloaded: u64,
       ) -> Result<(), DownloadError> {
           let mut state = self.load(task_id).await?
               .ok_or(DownloadError::TaskNotFound(task_id.to_string()))?;

           if let Some(chunk) = state.chunks.get_mut(chunk_index) {
               chunk.downloaded = downloaded;
               chunk.completed = downloaded >= chunk.end - chunk.start + 1;
           }

           state.updated_at = Utc::now();
           self.save(&state).await
       }
   }
   ```

#### 验收标准

- [ ] 状态保存正确
- [ ] 原子写入正常
- [ ] 状态加载正确
- [ ] 状态删除正常

---

### T1.4.3: 实现断点恢复

**时间**: 2h  
**依赖**: T1.4.2

#### 步骤

1. **实现文件验证**
   ```rust
   // src/resume/recovery.rs
   use crate::http::HttpClient;

   /// 断点恢复器
   pub struct ResumeRecovery {
       http_client: Arc<HttpClient>,
       state_manager: StateManager,
   }

   impl ResumeRecovery {
       /// 验证文件是否可续传
       pub async fn verify_resume(
           &self,
           state: &ResumeState,
       ) -> Result<bool, DownloadError> {
           // 1. 检查本地文件是否存在
           let output_path = Path::new(&state.output_path);
           if !output_path.exists() {
               return Ok(false);
           }

           // 2. 检查文件大小
           let metadata = std::fs::metadata(output_path)?;
           if metadata.len() != state.file_size {
               return Ok(false);
           }

           // 3. 验证远程文件未变更
           let head = self.http_client.head(&state.url).await?;

           // 检查 ETag
           if let Some(etag) = &state.file_identity.etag {
               if head.etag.as_ref() != Some(etag) {
                   return Ok(false);
               }
           }

           // 检查 Last-Modified
           if let Some(last_modified) = &state.file_identity.last_modified {
               if head.last_modified.as_ref() != Some(last_modified) {
                   return Ok(false);
               }
           }

           // 检查 Content-Length
           if head.content_length != Some(state.file_identity.content_length) {
               return Ok(false);
           }

           Ok(true)
       }
   }
   ```

2. **实现恢复下载**
   ```rust
   impl ResumeRecovery {
       /// 恢复未完成的下载
       pub async fn resume(
           &self,
           task_id: &str,
           progress: Arc<ProgressTracker>,
           cancel_flag: Arc<AtomicBool>,
       ) -> Result<DownloadResult, DownloadError> {
           // 1. 加载状态
           let state = self.state_manager.load(task_id).await?
               .ok_or(DownloadError::TaskNotFound(task_id.to_string()))?;

           // 2. 验证可续传
           if !self.verify_resume(&state).await? {
               return Err(DownloadError::ResumeNotSupported);
           }

           // 3. 计算未完成的分片
           let pending_chunks: Vec<_> = state.chunks.iter()
               .filter(|c| !c.completed)
               .cloned()
               .collect();

           if pending_chunks.is_empty() {
               // 已完成，直接返回结果
               return Ok(DownloadResult {
                   task_id: state.task_id,
                   output_path: PathBuf::from(&state.output_path),
                   file_size: state.file_size,
                   duration_ms: 0,
                   avg_speed: 0,
               });
           }

           // 4. 恢复下载进度
           progress.restore(&state.chunks, state.file_size);

           // 5. 继续下载未完成的分片
           // ... (类似 T1.3.3 的逻辑)

           Ok(DownloadResult { /* ... */ })
       }
   }
   ```

#### 验收标准

- [ ] 文件验证正确
- [ ] 恢复逻辑正确
- [ ] 远程文件变更检测
- [ ] 单元测试覆盖

---

## T1.5: 进度回调

### T1.5.1: 设计进度追踪器

**时间**: 1h  
**依赖**: T1.3

#### 步骤

1. **定义进度追踪结构**
   ```rust
   // src/progress/tracker.rs
   use std::sync::Arc;
   use std::sync::atomic::{AtomicU64, Ordering};
   use parking_lot::RwLock;

   /// 进度追踪器
   pub struct ProgressTracker {
       /// 总字节数
       total: AtomicU64,
       /// 已下载字节数
       downloaded: AtomicU64,
       /// 分片进度
       chunks: RwLock<Vec<ChunkProgress>>,
       /// 回调函数
       callback: RwLock<Option<Box<dyn Fn(&DownloadProgress) + Send + Sync>>>,
       /// 更新频率限制 (毫秒)
       update_interval_ms: u64,
       /// 上次更新时间
       last_update: RwLock<std::time::Instant>,
   }

   /// 分片进度
   #[derive(Debug, Clone)]
   pub struct ChunkProgress {
       pub index: usize,
       pub downloaded: u64,
       pub completed: bool,
   }

   /// 下载进度
   #[derive(Debug, Clone)]
   pub struct DownloadProgress {
       pub task_id: String,
       pub downloaded: u64,
       pub total: u64,
       pub speed: u64,
       pub eta: Option<u64>,
       pub state: DownloadState,
       pub chunks: Vec<ChunkProgress>,
   }
   ```

2. **实现进度更新**
   ```rust
   impl ProgressTracker {
       pub fn new(update_interval_ms: u64) -> Self {
           Self {
               total: AtomicU64::new(0),
               downloaded: AtomicU64::new(0),
               chunks: RwLock::new(Vec::new()),
               callback: RwLock::new(None),
               update_interval_ms,
               last_update: RwLock::new(std::time::Instant::now()),
           }
       }

       /// 初始化进度
       pub fn init(&self, chunks: &[ChunkRange], total: u64) {
           self.total.store(total, Ordering::SeqCst);
           let chunk_progress: Vec<_> = chunks.iter().map(|c| ChunkProgress {
               index: c.index,
               downloaded: 0,
               completed: false,
           }).collect();
           *self.chunks.write() = chunk_progress;
       }

       /// 更新分片进度
       pub fn update_chunk(&self, index: usize, downloaded: u64) {
           let mut chunks = self.chunks.write();
           if let Some(chunk) = chunks.get_mut(index) {
               chunk.downloaded = downloaded;
           }
           drop(chunks);

           // 汇总总进度
           let total_downloaded: u64 = self.chunks.read()
               .iter()
               .map(|c| c.downloaded)
               .sum();
           self.downloaded.store(total_downloaded, Ordering::SeqCst);

           // 触发回调 (频率限制)
           self.maybe_notify();
       }

       fn maybe_notify(&self) {
           let mut last_update = self.last_update.write();
           let now = std::time::Instant::now();
           if now.duration_since(*last_update).as_millis() as u64 >= self.update_interval_ms {
               *last_update = now;
               if let Some(callback) = self.callback.read().as_ref() {
                   // 构建进度并调用回调
                   let progress = self.build_progress();
                   callback(&progress);
               }
           }
       }
   }
   ```

#### 验收标准

- [ ] 进度计算正确
- [ ] 回调频率控制
- [ ] 线程安全

---

### T1.5.2: 实现速度计算

**时间**: 1h  
**依赖**: T1.5.1

#### 步骤

1. **定义速度计算器**
   ```rust
   // src/progress/speed.rs
   use std::collections::VecDeque;
   use std::time::Instant;

   /// 速度计算器 (滑动窗口)
   pub struct SpeedCalculator {
       /// 历史记录 (时间, 字节数)
       history: VecDeque<(Instant, u64)>,
       /// 窗口大小 (秒)
       window_size_secs: u64,
       /// 当前总下载量
       total_downloaded: u64,
   }

   impl SpeedCalculator {
       pub fn new(window_size_secs: u64) -> Self {
           Self {
               history: VecDeque::new(),
               window_size_secs,
               total_downloaded: 0,
           }
       }

       /// 记录下载量
       pub fn record(&mut self, bytes: u64) {
           let now = Instant::now();
           self.total_downloaded += bytes;
           self.history.push_back((now, self.total_downloaded));

           // 清理过期记录
           let cutoff = now - std::time::Duration::from_secs(self.window_size_secs);
           while let Some((time, _)) = self.history.front() {
               if *time < cutoff {
                   self.history.pop_front();
               } else {
                   break;
               }
           }
       }

       /// 计算当前速度 (字节/秒)
       pub fn speed(&self) -> u64 {
           if self.history.len() < 2 {
               return 0;
           }

           let (oldest_time, oldest_bytes) = self.history.front().unwrap();
           let (newest_time, newest_bytes) = self.history.back().unwrap();

           let duration = newest_time.duration_since(*oldest_time).as_secs_f64();
           if duration == 0.0 {
               return 0;
           }

           let diff = newest_bytes - oldest_bytes;
           (diff as f64 / duration) as u64
       }

       /// 计算 ETA (秒)
       pub fn eta(&self, remaining: u64) -> Option<u64> {
           let speed = self.speed();
           if speed == 0 {
               return None;
           }
           Some(remaining / speed)
       }
   }
   ```

2. **集成到进度追踪器**
   ```rust
   impl ProgressTracker {
       /// 获取完整进度信息
       pub fn build_progress(&self) -> DownloadProgress {
           let downloaded = self.downloaded.load(Ordering::SeqCst);
           let total = self.total.load(Ordering::SeqCst);
           let speed = self.speed_calculator.read().speed();
           let eta = self.speed_calculator.read().eta(total.saturating_sub(downloaded));

           DownloadProgress {
               task_id: self.task_id.clone(),
               downloaded,
               total,
               speed,
               eta,
               state: self.state.clone(),
               chunks: self.chunks.read().clone(),
           }
       }
   }
   ```

#### 验收标准

- [ ] 速度计算准确
- [ ] ETA 估算合理
- [ ] 滑动窗口正常工作

---

## T1.6: 错误处理与重试

### T1.6.1: 定义错误类型

**时间**: 1h  
**依赖**: T1.1

#### 步骤

```rust
// src/error/types.rs
use thiserror::Error;

/// 下载错误类型
#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("File already exists: {0}")]
    FileExists(String),

    #[error("Server error ({0}): {1}")]
    ServerError(u16, String),

    #[error("Resume not supported")]
    ResumeNotSupported,

    #[error("Task already running: {0}")]
    TaskAlreadyRunning(String),

    #[error("Download cancelled")]
    Cancelled,

    #[error("Timeout")]
    Timeout,

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// 下载结果类型
pub type Result<T> = std::result::Result<T, DownloadError>;
```

#### 验收标准

- [ ] 错误类型完整
- [ ] 错误消息清晰
- [ ] 支持错误链

---

### T1.6.2: 实现重试策略

**时间**: 2h  
**依赖**: T1.6.1

#### 步骤

1. **定义重试策略**
   ```rust
   // src/retry.rs
   use std::time::Duration;

   /// 重试策略
   #[derive(Debug, Clone)]
   pub struct RetryStrategy {
       /// 最大重试次数
       pub max_retries: usize,
       /// 初始延迟
       pub initial_delay: Duration,
       /// 最大延迟
       pub max_delay: Duration,
       /// 延迟倍数
       pub multiplier: f64,
   }

   impl Default for RetryStrategy {
       fn default() -> Self {
           Self {
               max_retries: 3,
               initial_delay: Duration::from_secs(1),
               max_delay: Duration::from_secs(60),
               multiplier: 2.0,
           }
       }
   }

   impl RetryStrategy {
       /// 计算第 n 次重试的延迟
       pub fn delay_for_attempt(&self, attempt: usize) -> Duration {
           let delay = self.initial_delay.as_secs_f64()
               * self.multiplier.powi(attempt as i32);
           Duration::from_secs(delay.min(self.max_delay.as_secs_f64()) as u64)
       }

       /// 判断错误是否可重试
       pub fn is_retryable(error: &DownloadError) -> bool {
           matches!(
               error,
               DownloadError::Network(_)
               | DownloadError::Timeout
               | DownloadError::ServerError(code, _) if *code >= 500
           )
       }
   }
   ```

2. **实现重试执行器**
   ```rust
   pub async fn with_retry<F, T, Fut>(
       strategy: &RetryStrategy,
       mut operation: F,
   ) -> Result<T, DownloadError>
   where
       F: FnMut() -> Fut,
       Fut: std::future::Future<Output = Result<T, DownloadError>>,
   {
       let mut last_error = None;

       for attempt in 0..=strategy.max_retries {
           if attempt > 0 {
               let delay = strategy.delay_for_attempt(attempt - 1);
               tokio::time::sleep(delay).await;
           }

           match operation().await {
               Ok(result) => return Ok(result),
               Err(e) => {
                   if !RetryStrategy::is_retryable(&e) {
                       return Err(e);
                   }
                   last_error = Some(e);
               }
           }
       }

       Err(last_error.unwrap_or(DownloadError::Internal("Max retries exceeded".into())))
   }
   ```

#### 验收标准

- [ ] 重试逻辑正确
- [ ] 延迟计算正确
- [ ] 可重试错误判断正确

---

## T1.7: 测试与优化

### T1.7.1: 单元测试

**时间**: 4h  
**依赖**: T1.1-T1.6

#### 测试清单

```rust
// tests/http_test.rs
#[tokio::test]
async fn test_head_request() { /* ... */ }
#[tokio::test]
async fn test_range_request() { /* ... */ }
#[tokio::test]
async fn test_custom_headers() { /* ... */ }

// tests/chunk_test.rs
#[test]
fn test_chunk_calculation() { /* ... */ }
#[test]
fn test_chunk_validation() { /* ... */ }

// tests/download_test.rs
#[tokio::test]
async fn test_single_thread_download() { /* ... */ }
#[tokio::test]
async fn test_multi_thread_download() { /* ... */ }
#[tokio::test]
async fn test_cancel_download() { /* ... */ }

// tests/resume_test.rs
#[tokio::test]
async fn test_state_save_load() { /* ... */ }
#[tokio::test]
async fn test_resume_download() { /* ... */ }
```

#### 验收标准

- [ ] 单元测试覆盖率 > 80%
- [ ] 所有测试通过
- [ ] Mock 服务器正常工作

---

### T1.7.2: 性能测试

**时间**: 2h  
**依赖**: T1.7.1

#### 步骤

1. **创建基准测试**
   ```rust
   // benches/download_benchmark.rs
   use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

   fn benchmark_chunk_calculation(c: &mut Criterion) {
       let strategy = ChunkStrategy::default();

       c.bench_function("chunk_calc_100mb", |b| {
           b.iter(|| strategy.calculate(100 * 1024 * 1024, 4, true))
       });
   }

   criterion_group!(benches, benchmark_chunk_calculation);
   criterion_main!(benches);
   ```

2. **运行性能测试**
   ```bash
   cargo bench
   ```

#### 验收标准

- [ ] 基准测试结果记录
- [ ] 性能瓶颈分析

---

### T1.7.3: 内存优化

**时间**: 2h  
**依赖**: T1.7.2

#### 步骤

1. **分析内存使用**
   ```bash
   cargo install cargo-flamegraph
   cargo flamegraph --root
   ```

2. **优化点**
   - 减少缓冲区大小
   - 使用流式处理
   - 避免不必要的 clone

#### 验收标准

- [ ] 内存使用稳定
- [ ] 无内存泄漏

---

## T1.8: 文档与示例

### T1.8.1: 编写 API 文档

**时间**: 2h  
**依赖**: T1.7

#### 步骤

```bash
cargo doc --no-deps --open
```

确保所有公开 API 都有文档注释。

### T1.8.2: 编写示例代码

**时间**: 1h  
**依赖**: T1.8.1

创建多个示例：
- `examples/basic_download.rs`
- `examples/resume_download.rs`
- `examples/multi_thread.rs`

#### 验收标准

- [ ] 所有 API 有文档
- [ ] 示例可运行
- [ ] README 完整

---

## 任务依赖图

```
T1.1 项目初始化
  ├── T1.2 HTTP 客户端
  │     ├── T1.3 多线程下载
  │     │     ├── T1.4 断点续传
  │     │     └── T1.5 进度回调
  │     └── T1.6 错误处理
  └── T1.7 测试优化
        └── T1.8 文档示例
```

---

## 里程碑

| 里程碑 | 完成任务 | 预计时间 |
|--------|----------|----------|
| M1 | T1.1, T1.2 | Day 1 |
| M2 | T1.3, T1.6 | Day 2-3 |
| M3 | T1.4, T1.5 | Day 3-4 |
| M4 | T1.7, T1.8 | Day 5 |