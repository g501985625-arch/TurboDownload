# P2: turbo-crawler 详细任务链规划

> 按照乐高式开发模式：项目框架 → 任务链 → 子任务 → 步骤 → 验证

---

## 任务链总览

```
P2: turbo-crawler
├── T2.1 项目初始化 (2h)
│   ├── T2.1.1 创建项目结构 (0.5h)
│   ├── T2.1.2 配置依赖 (0.5h)
│   ├── T2.1.3 创建测试框架 (0.5h)
│   └── T2.1.4 配置开发工具 (0.5h)
│
├── T2.2 HTTP 客户端封装 (2h) [可复用P1]
│   ├── T2.2.1 Client 结构定义 (0.5h)
│   ├── T2.2.2 GET 请求实现 (0.5h)
│   └── T2.2.3 响应处理 (1h)
│
├── T2.3 URL 规范化模块 (2h)
│   ├── T2.3.1 UrlNormalizer 实现 (1h)
│   ├── T2.3.2 相对路径解析 (0.5h)
│   └── T2.3.3 URL 验证 (0.5h)
│
├── T2.4 HTML 解析模块 (3h)
│   ├── T2.4.1 HtmlParser 结构 (0.5h)
│   ├── T2.4.2 DOM 解析 (1h)
│   ├── T2.4.3 选择器查询 (1h)
│   └── T2.4.4 解析测试 (0.5h)
│
├── T2.5 URL 提取模块 (3h)
│   ├── T2.5.1 UrlExtractor 结构 (0.5h)
│   ├── T2.5.2 HTML 链接提取 (1h)
│   ├── T2.5.3 CSS 资源提取 (1h)
│   └── T2.5.4 提取测试 (0.5h)
│
├── T2.6 资源分类模块 (2h)
│   ├── T2.6.1 ResourceType 定义 (0.5h)
│   ├── T2.6.2 TypeClassifier 实现 (1h)
│   └── T2.6.3 分类测试 (0.5h)
│
├── T2.7 URL 调度器 (4h)
│   ├── T2.7.1 UrlQueue 实现 (1.5h)
│   ├── T2.7.2 Scheduler 实现 (1.5h)
│   └── T2.7.3 调度策略 (1h)
│
├── T2.8 资源提取整合 (3h)
│   ├── T2.8.1 ResourceExtractor 实现 (1.5h)
│   ├── T2.8.2 整合提取逻辑 (1h)
│   └── T2.8.3 提取测试 (0.5h)
│
├── T2.9 整站扫描 (4h)
│   ├── T2.9.1 Crawler 实现 (2h)
│   ├── T2.9.2 并发控制 (1h)
│   └── T2.9.3 扫描测试 (1h)
│
└── T2.10 测试与文档 (3h)
    ├── T2.10.1 集成测试 (1.5h)
    └── T2.10.2 API 文档 (1.5h)
```

**总工时**: 28 小时

---

## T2.1 项目初始化

### T2.1.1 创建项目结构

**时间**: 0.5h  
**依赖**: 无

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 创建 crate 目录 | `crates/turbo-crawler/` | 目录存在 |
| 2 | 初始化 Cargo 项目 | `cargo init --lib` | `Cargo.toml` 存在 |
| 3 | 创建模块目录 | `src/{http,parser,extractor,scheduler,classifier,normalizer}/` | 目录存在 |
| 4 | 创建模块文件 | `src/*/mod.rs` | 文件存在 |
| 5 | 编写 lib.rs 入口 | `src/lib.rs` | `cargo check` 通过 |

#### 输出文件

**src/lib.rs**:
```rust
//! Turbo Crawler - High-performance web resource crawler

pub mod http;
pub mod parser;
pub mod extractor;
pub mod scheduler;
pub mod classifier;
pub mod normalizer;

pub use extractor::{Resource, ResourceType, ResourceExtractor};
pub use scheduler::{UrlQueue, Scheduler};
pub use normalizer::UrlNormalizer;
```

#### 验收清单
- [ ] `cargo check` 无错误
- [ ] 目录结构符合设计

---

### T2.1.2 配置依赖

**时间**: 0.5h  
**依赖**: T2.1.1

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 编辑 Cargo.toml | `Cargo.toml` | 依赖配置完整 |
| 2 | 下载依赖 | `cargo fetch` | 依赖下载成功 |

#### 输出文件

**Cargo.toml**:
```toml
[package]
name = "turbo-crawler"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
reqwest = { workspace = true }
scraper = "0.18"
url = "2.5"
thiserror = { workspace = true }
tracing = { workspace = true }
futures = "0.3"
dashmap = "5.5"
robotstxt = "0.3"

[dev-dependencies]
wiremock = "0.5"
tokio-test = "0.4"
```

#### 验收清单
- [ ] `cargo fetch` 成功
- [ ] `cargo build` 成功

---

### T2.1.3 创建测试框架

**时间**: 0.5h  
**依赖**: T2.1.1

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 创建测试目录 | `tests/` | 目录存在 |
| 2 | 创建 Mock 工具 | `tests/common/mod.rs` | 文件存在 |
| 3 | 创建示例测试 | `tests/smoke_test.rs` | 测试通过 |

#### 验收清单
- [ ] `cargo test` 可运行

---

### T2.1.4 配置开发工具

**时间**: 0.5h  
**依赖**: T2.1.1

#### 步骤

| # | 操作 | 文件路径 | 验收标准 |
|---|------|----------|----------|
| 1 | 创建 rustfmt 配置 | `rustfmt.toml` | 格式化规则生效 |
| 2 | 创建 clippy 配置 | `clippy.toml` | Lint 规则生效 |

#### 验收清单
- [ ] `cargo fmt -- --check` 通过
- [ ] `cargo clippy` 无警告

---

## T2.2 HTTP 客户端封装

### T2.2.1 Client 结构定义

**时间**: 0.5h  
**依赖**: T2.1.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Client | `src/http/client.rs` | `Client` | 结构定义 |
| 2 | 定义配置 | `src/http/client.rs` | `ClientConfig` | 配置定义 |

#### 输出文件

```rust
pub struct Client {
    inner: reqwest::Client,
    config: ClientConfig,
}

pub struct ClientConfig {
    pub timeout: std::time::Duration,
    pub user_agent: String,
    pub max_redirects: usize,
}
```

#### 验收清单
- [ ] Client 可创建

---

### T2.2.2 GET 请求实现

**时间**: 0.5h  
**依赖**: T2.2.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 get 方法 | `src/http/client.rs` | `get(&self, url)` | 方法可用 |

#### 输出代码

```rust
impl Client {
    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        let response = self.inner
            .get(url)
            .send()
            .await?;
        
        Ok(HttpResponse::from(response).await?)
    }
}
```

#### 验收清单
- [ ] GET 请求可发送

---

### T2.2.3 响应处理

**时间**: 1h  
**依赖**: T2.2.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 HttpResponse | `src/http/response.rs` | `HttpResponse` | 结构定义 |
| 2 | 实现响应转换 | `src/http/response.rs` | `from()` | 转换正确 |

#### 输出文件

```rust
pub struct HttpResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: String,
    pub content_type: Option<String>,
}

impl HttpResponse {
    pub async fn from(response: reqwest::Response) -> Result<Self> {
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        let body = response.text().await?;
        
        Ok(Self { status, headers, body, content_type })
    }
}
```

#### 验收清单
- [ ] 响应正确解析

---

## T2.3 URL 规范化模块

### T2.3.1 UrlNormalizer 实现

**时间**: 1h  
**依赖**: T2.1.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 UrlNormalizer | `src/normalizer/url.rs` | `UrlNormalizer` | 结构定义 |
| 2 | 实现规范化方法 | `src/normalizer/url.rs` | `normalize()` | 规范化正确 |

#### 输出文件

```rust
use url::Url;

pub struct UrlNormalizer;

impl UrlNormalizer {
    /// 规范化 URL
    pub fn normalize(url: &str, base: Option<&str>) -> Result<String> {
        let mut parsed = if let Some(base_url) = base {
            let base = Url::parse(base_url)?;
            base.join(url)?
        } else {
            Url::parse(url)?
        };
        
        // 移除片段
        parsed.set_fragment(None);
        
        // 移除默认端口
        // ... 规范化逻辑
        
        Ok(parsed.to_string())
    }
    
    /// 判断是否同域名
    pub fn is_same_domain(url1: &str, url2: &str) -> bool {
        // 实现域名比较
    }
}
```

#### 验收清单
- [ ] URL 规范化正确
- [ ] 相对路径解析正确

---

### T2.3.2 相对路径解析

**时间**: 0.5h  
**依赖**: T2.3.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现相对路径解析 | `src/normalizer/url.rs` | `resolve_relative()` | 解析正确 |

#### 输出代码

```rust
impl UrlNormalizer {
    pub fn resolve_relative(relative: &str, base: &str) -> Result<String> {
        let base_url = Url::parse(base)?;
        let resolved = base_url.join(relative)?;
        Ok(resolved.to_string())
    }
}
```

#### 验收清单
- [ ] 相对路径解析正确

---

### T2.3.3 URL 验证

**时间**: 0.5h  
**依赖**: T2.3.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 URL 验证 | `src/normalizer/url.rs` | `is_valid()` | 验证正确 |

#### 输出代码

```rust
impl UrlNormalizer {
    pub fn is_valid(url: &str) -> bool {
        Url::parse(url).is_ok()
    }
    
    pub fn is_http(url: &str) -> bool {
        if let Ok(parsed) = Url::parse(url) {
            matches!(parsed.scheme(), "http" | "https")
        } else {
            false
        }
    }
}
```

#### 验收清单
- [ ] URL 验证正确

---

## T2.4 HTML 解析模块

### T2.4.1 HtmlParser 结构

**时间**: 0.5h  
**依赖**: T2.1.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 HtmlParser | `src/parser/html.rs` | `HtmlParser` | 结构定义 |

#### 输出文件

```rust
use scraper::{Html, Selector};

pub struct HtmlParser {
    document: Html,
}

impl HtmlParser {
    pub fn from_html(html: &str) -> Self {
        Self {
            document: Html::parse_document(html),
        }
    }
}
```

#### 验收清单
- [ ] 解析器可创建

---

### T2.4.2 DOM 解析

**时间**: 1h  
**依赖**: T2.4.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 DOM 解析 | `src/parser/html.rs` | `parse()` | 解析正确 |

#### 输出代码

```rust
impl HtmlParser {
    /// 查询所有匹配元素
    pub fn select(&self, selector: &str) -> Result<Vec<Element>> {
        let selector = Selector::parse(selector)
            .map_err(|e| ParseError::InvalidSelector(format!("{:?}", e)))?;
        
        Ok(self.document
            .select(&selector)
            .map(|el| Element::from(el))
            .collect())
    }
    
    /// 获取元素属性
    pub fn get_attr(&self, element: &Element, attr: &str) -> Option<String> {
        element.attr(attr).map(String::from)
    }
}
```

#### 验收清单
- [ ] DOM 解析正确

---

### T2.4.3 选择器查询

**时间**: 1h  
**依赖**: T2.4.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现链接查询 | `src/parser/html.rs` | `query_links()` | 查询正确 |
| 2 | 实现资源查询 | `src/parser/html.rs` | `query_resources()` | 查询正确 |

#### 输出代码

```rust
impl HtmlParser {
    /// 提取所有链接
    pub fn query_links(&self) -> Vec<String> {
        self.select("a[href]")
            .unwrap_or_default()
            .iter()
            .filter_map(|el| el.attr("href"))
            .map(String::from)
            .collect()
    }
    
    /// 提取所有图片
    pub fn query_images(&self) -> Vec<String> {
        self.select("img[src]")
            .unwrap_or_default()
            .iter()
            .filter_map(|el| el.attr("src"))
            .map(String::from)
            .collect()
    }
}
```

#### 验收清单
- [ ] 选择器查询正确

---

### T2.4.4 解析测试

**时间**: 0.5h  
**依赖**: T2.4.3

#### 验收清单
- [ ] HTML 解析测试通过
- [ ] 边界情况覆盖

---

## T2.5 URL 提取模块

### T2.5.1 UrlExtractor 结构

**时间**: 0.5h  
**依赖**: T2.4

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 UrlExtractor | `src/extractor/url.rs` | `UrlExtractor` | 结构定义 |

#### 输出文件

```rust
pub struct UrlExtractor {
    normalizer: UrlNormalizer,
}

impl UrlExtractor {
    pub fn new() -> Self {
        Self {
            normalizer: UrlNormalizer,
        }
    }
}
```

#### 验收清单
- [ ] 提取器可创建

---

### T2.5.2 HTML 链接提取

**时间**: 1h  
**依赖**: T2.5.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现链接提取 | `src/extractor/url.rs` | `extract_from_html()` | 提取正确 |

#### 输出代码

```rust
impl UrlExtractor {
    pub fn extract_from_html(&self, html: &str, base_url: &str) -> Result<Vec<UrlEntry>> {
        let parser = HtmlParser::from_html(html);
        let mut urls = Vec::new();
        
        // 提取 <a href>
        for href in parser.query_links() {
            if let Ok(normalized) = self.normalizer.normalize(&href, Some(base_url)) {
                urls.push(UrlEntry::new(normalized, ResourceType::Html));
            }
        }
        
        // 提取 <img src>
        for src in parser.query_images() {
            if let Ok(normalized) = self.normalizer.normalize(&src, Some(base_url)) {
                urls.push(UrlEntry::new(normalized, ResourceType::Image));
            }
        }
        
        // 提取 <link href>
        // 提取 <script src>
        // ...
        
        Ok(urls)
    }
}
```

#### 验收清单
- [ ] 链接提取正确
- [ ] 资源类型正确

---

### T2.5.3 CSS 资源提取

**时间**: 1h  
**依赖**: T2.5.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现 CSS 解析 | `src/parser/css.rs` | `CssParser` | 解析器定义 |
| 2 | 实现 URL 提取 | `src/parser/css.rs` | `extract_urls()` | 提取正确 |

#### 输出文件

```rust
use regex::Regex;

pub struct CssParser;

impl CssParser {
    /// 从 CSS 中提取 URL
    pub fn extract_urls(css: &str, base_url: &str) -> Vec<String> {
        let url_pattern = Regex::new(r"url\(['\"]?([^'\"()]+)['\"]?\)").unwrap();
        
        url_pattern
            .captures_iter(css)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
}
```

#### 验收清单
- [ ] CSS URL 提取正确

---

### T2.5.4 提取测试

**时间**: 0.5h  
**依赖**: T2.5.3

#### 验收清单
- [ ] 提取测试通过

---

## T2.6 资源分类模块

### T2.6.1 ResourceType 定义

**时间**: 0.5h  
**依赖**: T2.1.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 ResourceType | `src/classifier/types.rs` | `ResourceType` | 枚举定义 |

#### 输出文件

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Html,
    Css,
    Js,
    Image,
    Font,
    Video,
    Audio,
    Document,
    Other,
}

impl ResourceType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "html" | "htm" => ResourceType::Html,
            "css" => ResourceType::Css,
            "js" | "mjs" => ResourceType::Js,
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => ResourceType::Image,
            "woff" | "woff2" | "ttf" | "otf" => ResourceType::Font,
            "mp4" | "webm" => ResourceType::Video,
            "mp3" | "wav" => ResourceType::Audio,
            "pdf" | "doc" | "docx" => ResourceType::Document,
            _ => ResourceType::Other,
        }
    }
    
    pub fn from_mime(mime: &str) -> Self {
        match mime.split('/').next() {
            Some("text") => ResourceType::Html,
            Some("image") => ResourceType::Image,
            Some("video") => ResourceType::Video,
            Some("audio") => ResourceType::Audio,
            Some("font") => ResourceType::Font,
            _ => ResourceType::Other,
        }
    }
}
```

#### 验收清单
- [ ] 类型定义完整

---

### T2.6.2 TypeClassifier 实现

**时间**: 1h  
**依赖**: T2.6.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 TypeClassifier | `src/classifier/mod.rs` | `TypeClassifier` | 结构定义 |
| 2 | 实现分类方法 | `src/classifier/mod.rs` | `classify()` | 分类正确 |

#### 输出代码

```rust
pub struct TypeClassifier;

impl TypeClassifier {
    pub fn classify(url: &str, content_type: Option<&str>) -> ResourceType {
        // 优先使用 Content-Type
        if let Some(mime) = content_type {
            return ResourceType::from_mime(mime);
        }
        
        // 使用 URL 扩展名
        if let Some(ext) = url.rsplit('.').next() {
            return ResourceType::from_extension(ext);
        }
        
        ResourceType::Other
    }
}
```

#### 验收清单
- [ ] 分类正确

---

### T2.6.3 分类测试

**时间**: 0.5h  
**依赖**: T2.6.2

#### 验收清单
- [ ] 分类测试通过

---

## T2.7 URL 调度器

### T2.7.1 UrlQueue 实现

**时间**: 1.5h  
**依赖**: T2.3

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 UrlQueue | `src/scheduler/queue.rs` | `UrlQueue` | 结构定义 |
| 2 | 实现入队出队 | `src/scheduler/queue.rs` | `push/pop` | 操作正确 |
| 3 | 实现去重 | `src/scheduler/queue.rs` | 去重逻辑 | 去重正确 |

#### 输出文件

```rust
use std::collections::{HashSet, VecDeque};
use parking_lot::Mutex;

pub struct UrlQueue {
    pending: Mutex<VecDeque<UrlEntry>>,
    visited: Mutex<HashSet<String>>,
}

impl UrlQueue {
    pub fn new() -> Self {
        Self {
            pending: Mutex::new(VecDeque::new()),
            visited: Mutex::new(HashSet::new()),
        }
    }
    
    pub fn push(&self, entry: UrlEntry) -> bool {
        let mut visited = self.visited.lock();
        if visited.contains(&entry.url) {
            return false;
        }
        visited.insert(entry.url.clone());
        
        self.pending.lock().push_back(entry);
        true
    }
    
    pub fn pop(&self) -> Option<UrlEntry> {
        self.pending.lock().pop_front()
    }
    
    pub fn len(&self) -> usize {
        self.pending.lock().len()
    }
}
```

#### 验收清单
- [ ] 队列操作正确
- [ ] 去重有效

---

### T2.7.2 Scheduler 实现

**时间**: 1.5h  
**依赖**: T2.7.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Scheduler | `src/scheduler/mod.rs` | `Scheduler` | 结构定义 |
| 2 | 实现调度逻辑 | `src/scheduler/mod.rs` | 调度方法 | 调度正确 |

#### 输出文件

```rust
use crate::http::Client;
use crate::extractor::UrlExtractor;

pub struct Scheduler {
    queue: UrlQueue,
    client: Client,
    max_depth: u32,
    concurrency: usize,
}

impl Scheduler {
    pub fn new(client: Client, max_depth: u32, concurrency: usize) -> Self {
        Self {
            queue: UrlQueue::new(),
            client,
            max_depth,
            concurrency,
        }
    }
    
    pub fn add_seed(&self, url: String) {
        self.queue.push(UrlEntry::new(url, ResourceType::Html));
    }
}
```

#### 验收清单
- [ ] 调度器正常工作

---

### T2.7.3 调度策略

**时间**: 1h  
**依赖**: T2.7.2

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现深度限制 | `src/scheduler/policy.rs` | 深度策略 | 限制生效 |
| 2 | 实现域名限制 | `src/scheduler/policy.rs` | 域名策略 | 限制生效 |

#### 输出代码

```rust
pub struct CrawlPolicy {
    pub max_depth: u32,
    pub allowed_domains: Option<Vec<String>>,
    pub excluded_paths: Vec<String>,
}

impl CrawlPolicy {
    pub fn should_crawl(&self, url: &str, depth: u32) -> bool {
        if depth > self.max_depth {
            return false;
        }
        
        // 域名检查
        // 路径排除检查
        
        true
    }
}
```

#### 验收清单
- [ ] 策略生效

---

## T2.8 资源提取整合

### T2.8.1 ResourceExtractor 实现

**时间**: 1.5h  
**依赖**: T2.5, T2.6

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 ResourceExtractor | `src/extractor/resource.rs` | `ResourceExtractor` | 结构定义 |
| 2 | 实现提取方法 | `src/extractor/resource.rs` | `extract()` | 提取正确 |

#### 输出文件

```rust
pub struct ResourceExtractor {
    url_extractor: UrlExtractor,
    classifier: TypeClassifier,
}

impl ResourceExtractor {
    pub fn new() -> Self {
        Self {
            url_extractor: UrlExtractor::new(),
            classifier: TypeClassifier,
        }
    }
    
    pub async fn extract(&self, url: &str, html: &str) -> Result<Vec<Resource>> {
        let entries = self.url_extractor.extract_from_html(html, url)?;
        
        let resources = entries
            .into_iter()
            .map(|entry| Resource {
                url: entry.url,
                resource_type: entry.resource_type,
                source_page: url.to_string(),
            })
            .collect();
        
        Ok(resources)
    }
}
```

#### 验收清单
- [ ] 提取整合正确

---

### T2.8.2 整合提取逻辑

**时间**: 1h  
**依赖**: T2.8.1

#### 验收清单
- [ ] 整合逻辑正确

---

### T2.8.3 提取测试

**时间**: 0.5h  
**依赖**: T2.8.2

#### 验收清单
- [ ] 测试通过

---

## T2.9 整站扫描

### T2.9.1 Crawler 实现

**时间**: 2h  
**依赖**: T2.7, T2.8

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 定义 Crawler | `src/lib.rs` | `Crawler` | 结构定义 |
| 2 | 实现爬取方法 | `src/lib.rs` | `crawl()` | 爬取正确 |

#### 输出文件

```rust
pub struct Crawler {
    scheduler: Scheduler,
    extractor: ResourceExtractor,
}

impl Crawler {
    pub fn new(config: CrawlerConfig) -> Self {
        // 初始化
    }
    
    pub async fn crawl(&self, seed: &str) -> Result<Vec<Resource>> {
        self.scheduler.add_seed(seed.to_string());
        
        let mut resources = Vec::new();
        
        while let Some(entry) = self.scheduler.queue.pop() {
            let response = self.scheduler.client.get(&entry.url).await?;
            let extracted = self.extractor.extract(&entry.url, &response.body).await?;
            resources.extend(extracted);
        }
        
        Ok(resources)
    }
}
```

#### 验收清单
- [ ] 爬取正常

---

### T2.9.2 并发控制

**时间**: 1h  
**依赖**: T2.9.1

#### 步骤

| # | 操作 | 文件路径 | 函数/类型 | 验收标准 |
|---|------|----------|-----------|----------|
| 1 | 实现并发爬取 | `src/lib.rs` | 并发逻辑 | 并发正确 |
| 2 | 实现速率限制 | `src/lib.rs` | 限速逻辑 | 限速生效 |

#### 输出代码

```rust
impl Crawler {
    pub async fn crawl_concurrent(&self, seeds: Vec<String>) -> Result<Vec<Resource>> {
        let mut tasks = JoinSet::new();
        let resources = Arc<Mutex<Vec<Resource>>>;
        
        for seed in seeds {
            let crawler = self.clone();
            tasks.spawn(async move {
                crawler.crawl(&seed).await
            });
        }
        
        while let Some(result) = tasks.join_next().await {
            if let Ok(page_resources) = result? {
                resources.lock().extend(page_resources);
            }
        }
        
        Ok(resources.lock().clone())
    }
}
```

#### 验收清单
- [ ] 并发控制正确

---

### T2.9.3 扫描测试

**时间**: 1h  
**依赖**: T2.9.2

#### 验收清单
- [ ] 扫描测试通过

---

## T2.10 测试与文档

### T2.10.1 集成测试

**时间**: 1.5h  
**依赖**: T2.9

#### 验收清单
- [ ] 集成测试通过

---

### T2.10.2 API 文档

**时间**: 1.5h  
**依赖**: T2.9

#### 验收清单
- [ ] API 文档完整

---

## 任务依赖关系图

```
T2.1 ──┬── T2.2 ────────────────────────┐
        │                                │
        ├── T2.3 ────────────────────────┤
        │                                │
        └── T2.4 ─── T2.5 ─── T2.6 ──────┤
                                         │
T2.7 ───────────────────────────────────┤
                                         │
T2.8 ◀───────────────────────────────────┘
 │
 └── T2.9 ─── T2.10
```

---

## 工时汇总

| 任务 | 时间 | 并行机会 |
|------|------|----------|
| T2.1 项目初始化 | 2h | T2.1.2-4 部分并行 |
| T2.2 HTTP 客户端 | 2h | - |
| T2.3 URL 规范化 | 2h | 与 T2.4 并行 |
| T2.4 HTML 解析 | 3h | 与 T2.3 并行 |
| T2.5 URL 提取 | 3h | - |
| T2.6 资源分类 | 2h | 与 T2.5 并行 |
| T2.7 URL 调度器 | 4h | - |
| T2.8 资源提取整合 | 3h | - |
| T2.9 整站扫描 | 4h | - |
| T2.10 测试文档 | 3h | - |
| **总计** | **28h** | **优化后约 22h** |

---

*任务链规划版本: v1.0*
*规划日期: 2026-03-26*