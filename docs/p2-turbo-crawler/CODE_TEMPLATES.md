# P2: turbo-crawler 代码模板

本文档提供核心代码结构和实现模板。

---

## 核心结构体定义

### 1. 爬虫配置 (CrawlConfig)

```rust
// src/fetch/config.rs

use std::time::Duration;
use url::Url;
use std::collections::HashSet;

/// 爬虫配置
#[derive(Debug, Clone)]
pub struct CrawlConfig {
    /// 起始 URL
    pub start_url: Url,
    
    /// 最大爬取深度
    pub max_depth: usize,
    
    /// 最大页面数
    pub max_pages: usize,
    
    /// 并发请求数
    pub concurrent_requests: usize,
    
    /// 请求间隔 (毫秒)
    pub request_delay_ms: u64,
    
    /// 用户代理
    pub user_agent: Option<String>,
    
    /// 是否遵守 robots.txt
    pub respect_robots_txt: bool,
    
    /// 允许的域名 (None 表示不限制)
    pub allowed_domains: Option<Vec<String>>,
    
    /// 资源过滤器
    pub resource_filter: ResourceFilter,
    
    /// 请求超时
    pub request_timeout: Duration,
    
    /// 最大重试次数
    pub max_retries: u32,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            start_url: "https://example.com".parse().unwrap(),
            max_depth: 3,
            max_pages: 100,
            concurrent_requests: 5,
            request_delay_ms: 500,
            user_agent: None,
            respect_robots_txt: true,
            allowed_domains: None,
            resource_filter: ResourceFilter::default(),
            request_timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

impl CrawlConfig {
    /// 从 URL 创建配置
    pub fn from_url(url: impl Into<String>) -> Result<Self, url::ParseError> {
        Ok(Self {
            start_url: url.into().parse()?,
            ..Default::default()
        })
    }
    
    /// 设置最大深度
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    
    /// 设置并发数
    pub fn with_concurrency(mut self, count: usize) -> Self {
        self.concurrent_requests = count;
        self
    }
    
    /// 设置请求间隔
    pub fn with_delay(mut self, ms: u64) -> Self {
        self.request_delay_ms = ms;
        self
    }
}
```

---

### 2. 资源定义 (Resource)

```rust
// src/extract/resource.rs

use url::Url;
use serde::{Deserialize, Serialize};

/// 提取的资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// 资源 URL
    pub url: Url,
    
    /// 来源页面
    pub source_url: Option<Url>,
    
    /// 资源类型
    pub resource_type: ResourceType,
    
    /// 内容类型 (MIME)
    pub content_type: Option<String>,
    
    /// 文件大小
    pub size: Option<u64>,
    
    /// 文件名
    pub filename: Option<String>,
    
    /// 替代文本 (图片)
    pub alt_text: Option<String>,
    
    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
}

/// 资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    /// HTML 页面
    Html,
    /// CSS 样式表
    Css,
    /// JavaScript 脚本
    JavaScript,
    /// 图片
    Image,
    /// 视频
    Video,
    /// 音频
    Audio,
    /// 字体
    Font,
    /// 文档
    Document,
    /// 其他
    Other,
}

impl ResourceType {
    /// 从 URL 推断资源类型
    pub fn from_url(url: &Url) -> Self {
        let path = url.path().to_lowercase();
        
        if path.ends_with(".html") || path.ends_with(".htm") {
            Self::Html
        } else if path.ends_with(".css") {
            Self::Css
        } else if path.ends_with(".js") || path.ends_with(".mjs") {
            Self::JavaScript
        } else if matches!(path.rsplit('.').next(), 
            Some("jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "ico")) {
            Self::Image
        } else if matches!(path.rsplit('.').next(),
            Some("mp4" | "webm" | "avi" | "mov")) {
            Self::Video
        } else if matches!(path.rsplit('.').next(),
            Some("mp3" | "wav" | "ogg" | "m4a")) {
            Self::Audio
        } else if matches!(path.rsplit('.').next(),
            Some("woff" | "woff2" | "ttf" | "otf" | "eot")) {
            Self::Font
        } else if matches!(path.rsplit('.').next(),
            Some("pdf" | "doc" | "docx" | "xls" | "xlsx")) {
            Self::Document
        } else {
            Self::Other
        }
    }
    
    /// 从 Content-Type 推断资源类型
    pub fn from_content_type(content_type: &str) -> Self {
        let mime = content_type.split(';').next().unwrap_or("").trim();
        
        match mime {
            "text/html" => Self::Html,
            "text/css" => Self::Css,
            "application/javascript" | "text/javascript" => Self::JavaScript,
            m if m.starts_with("image/") => Self::Image,
            m if m.starts_with("video/") => Self::Video,
            m if m.starts_with("audio/") => Self::Audio,
            m if m.starts_with("font/") => Self::Font,
            "application/pdf" => Self::Document,
            _ => Self::Other,
        }
    }
}
```

---

### 3. 爬取结果 (CrawlResult)

```rust
// src/fetch/result.rs

use url::Url;
use std::time::Duration;
use crate::extract::Resource;

/// 爬取结果
#[derive(Debug, Clone)]
pub struct CrawlResult {
    /// 页面 URL
    pub url: Url,
    
    /// 爬取深度
    pub depth: usize,
    
    /// 状态码
    pub status_code: u16,
    
    /// 响应时间
    pub response_time: Duration,
    
    /// 提取的资源
    pub resources: Vec<Resource>,
    
    /// 发现的链接
    pub links: Vec<Url>,
    
    /// 错误信息
    pub error: Option<String>,
}

impl CrawlResult {
    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.error.is_none() && self.status_code >= 200 && self.status_code < 300
    }
    
    /// 按类型获取资源
    pub fn resources_by_type(&self, resource_type: ResourceType) -> Vec<&Resource> {
        self.resources
            .iter()
            .filter(|r| r.resource_type == resource_type)
            .collect()
    }
    
    /// 获取所有图片
    pub fn images(&self) -> Vec<&Resource> {
        self.resources_by_type(ResourceType::Image)
    }
    
    /// 获取所有样式表
    pub fn stylesheets(&self) -> Vec<&Resource> {
        self.resources_by_type(ResourceType::Css)
    }
    
    /// 获取所有脚本
    pub fn scripts(&self) -> Vec<&Resource> {
        self.resources_by_type(ResourceType::JavaScript)
    }
}
```

---

### 4. 过滤器 (ResourceFilter)

```rust
// src/filter/resource_filter.rs

use url::Url;
use regex::Regex;
use std::collections::HashSet;

/// 资源过滤器
#[derive(Debug, Clone, Default)]
pub struct ResourceFilter {
    /// 包含的资源类型
    pub include_types: HashSet<ResourceType>,
    
    /// 排除的资源类型
    pub exclude_types: HashSet<ResourceType>,
    
    /// 包含的 URL 模式
    pub include_patterns: Vec<Regex>,
    
    /// 排除的 URL 模式
    pub exclude_patterns: Vec<Regex>,
    
    /// 最小文件大小
    pub min_size: Option<u64>,
    
    /// 最大文件大小
    pub max_size: Option<u64>,
    
    /// 排除的查询参数
    pub exclude_query_params: Vec<String>,
}

impl ResourceFilter {
    /// 创建新的过滤器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 检查资源是否应该被包含
    pub fn should_include(&self, resource: &Resource) -> bool {
        // 检查类型
        if !self.include_types.is_empty() && 
           !self.include_types.contains(&resource.resource_type) {
            return false;
        }
        
        if self.exclude_types.contains(&resource.resource_type) {
            return false;
        }
        
        // 检查大小
        if let Some(min) = self.min_size {
            if let Some(size) = resource.size {
                if size < min {
                    return false;
                }
            }
        }
        
        if let Some(max) = self.max_size {
            if let Some(size) = resource.size {
                if size > max {
                    return false;
                }
            }
        }
        
        // 检查 URL 模式
        let url_str = resource.url.as_str();
        
        for pattern in &self.exclude_patterns {
            if pattern.is_match(url_str) {
                return false;
            }
        }
        
        if !self.include_patterns.is_empty() {
            let matches_any = self.include_patterns.iter()
                .any(|p| p.is_match(url_str));
            if !matches_any {
                return false;
            }
        }
        
        true
    }
    
    /// 过滤资源列表
    pub fn filter(&self, resources: Vec<Resource>) -> Vec<Resource> {
        resources.into_iter()
            .filter(|r| self.should_include(r))
            .collect()
    }
}

/// 资源过滤器构建器
pub struct ResourceFilterBuilder {
    filter: ResourceFilter,
}

impl ResourceFilterBuilder {
    pub fn new() -> Self {
        Self {
            filter: ResourceFilter::new(),
        }
    }
    
    pub fn include_types(mut self, types: Vec<ResourceType>) -> Self {
        self.filter.include_types = types.into_iter().collect();
        self
    }
    
    pub fn exclude_types(mut self, types: Vec<ResourceType>) -> Self {
        self.filter.exclude_types = types.into_iter().collect();
        self
    }
    
    pub fn min_size(mut self, bytes: u64) -> Self {
        self.filter.min_size = Some(bytes);
        self
    }
    
    pub fn max_size(mut self, bytes: u64) -> Self {
        self.filter.max_size = Some(bytes);
        self
    }
    
    pub fn exclude_patterns(mut self, patterns: Vec<&str>) -> Self {
        self.filter.exclude_patterns = patterns
            .into_iter()
            .filter_map(|p| Regex::new(p).ok())
            .collect();
        self
    }
    
    pub fn build(self) -> ResourceFilter {
        self.filter
    }
}
```

---

## 核心 Trait 定义

### 1. 解析器 Trait

```rust
// src/parse/mod.rs

use crate::extract::Resource;
use url::Url;

/// 内容解析器 trait
pub trait Parser: Send + Sync {
    /// 解析内容并提取资源
    fn parse(&self, content: &str, base_url: &Url) -> ParserResult;
    
    /// 获取解析器支持的 MIME 类型
    fn supported_types(&self) -> Vec<&'static str>;
    
    /// 检查是否支持该内容类型
    fn supports(&self, content_type: &str) -> bool {
        let mime = content_type.split(';').next().unwrap_or("").trim();
        self.supported_types().iter().any(|&t| t == mime)
    }
}

/// 解析结果
#[derive(Debug, Clone)]
pub struct ParserResult {
    /// 提取的资源
    pub resources: Vec<Resource>,
    
    /// 发现的链接
    pub links: Vec<Url>,
    
    /// 提取的文本
    pub text: Option<String>,
    
    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
}
```

---

### 2. 提取器 Trait

```rust
// src/extract/mod.rs

use url::Url;
use crate::parse::ParserResult;

/// 资源提取器 trait
pub trait Extractor: Send + Sync {
    /// 从解析结果中提取资源
    fn extract(&self, result: &ParserResult, source_url: &Url) -> Vec<Resource>;
    
    /// 提取器名称
    fn name(&self) -> &str;
}

/// 链接提取器
pub struct LinkExtractor;

impl Extractor for LinkExtractor {
    fn extract(&self, result: &ParserResult, source_url: &Url) -> Vec<Resource> {
        result.links.iter()
            .map(|url| Resource {
                url: url.clone(),
                source_url: Some(source_url.clone()),
                resource_type: ResourceType::from_url(url),
                content_type: None,
                size: None,
                filename: extract_filename(url),
                alt_text: None,
                metadata: std::collections::HashMap::new(),
            })
            .collect()
    }
    
    fn name(&self) -> &str {
        "link"
    }
}

fn extract_filename(url: &Url) -> Option<String> {
    url.path_segments()
        .and_then(|segments| segments.last())
        .map(|s| s.to_string())
}
```

---

### 3. 过滤器 Trait

```rust
// src/filter/mod.rs

use url::Url;
use crate::extract::Resource;

/// URL 过滤器 trait
pub trait UrlFilter: Send + Sync {
    /// 检查 URL 是否应该被爬取
    fn filter(&self, url: &Url) -> FilterResult;
}

/// 过滤结果
#[derive(Debug, Clone, PartialEq)]
pub enum FilterResult {
    /// 允许
    Allow,
    /// 拒绝
    Deny(String),
    /// 需要进一步检查
    Pending,
}

/// 资源过滤器 trait
pub trait ResourceFilterTrait: Send + Sync {
    /// 检查资源是否应该被下载
    fn filter(&self, resource: &Resource) -> FilterResult;
}
```

---

## 主要函数实现

### 1. 爬虫主逻辑

```rust
// src/fetch/crawler.rs

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};
use url::Url;
use crate::error::Result;
use crate::parse::HtmlParser;
use crate::extract::{Resource, ResourceType};

/// 爬虫
pub struct Crawler {
    config: CrawlConfig,
    client: FetchClient,
    visited: Arc<RwLock<HashSet<Url>>>,
    queue: Arc<RwLock<VecDeque<CrawlItem>>>,
    semaphore: Arc<Semaphore>,
}

/// 爬取项
#[derive(Debug, Clone)]
struct CrawlItem {
    url: Url,
    depth: usize,
}

impl Crawler {
    /// 创建新爬虫
    pub fn new(config: CrawlConfig) -> Result<Self> {
        let client_config = FetchClientConfig {
            timeout: config.request_timeout,
            user_agent: config.user_agent.clone().unwrap_or_else(|| {
                format!("TurboCrawler/{}", env!("CARGO_PKG_VERSION"))
            }),
            ..Default::default()
        };
        
        let start_url = config.start_url.clone();
        
        Ok(Self {
            config,
            client: FetchClient::new(client_config)?,
            visited: Arc::new(RwLock::new(HashSet::new())),
            queue: Arc::new(RwLock::new(VecDeque::from(vec![
                CrawlItem { url: start_url, depth: 0 }
            ]))),
            semaphore: Arc::new(Semaphore::new(5)),
        })
    }
    
    /// 执行爬取
    pub async fn crawl(&self) -> Result<Vec<CrawlResult>> {
        let mut results = Vec::new();
        let mut pages_crawled = 0;
        
        while pages_crawled < self.config.max_pages {
            // 获取下一个 URL
            let item = {
                let mut queue = self.queue.write().await;
                queue.pop_front()
            };
            
            let item = match item {
                Some(i) => i,
                None => break, // 队列为空
            };
            
            // 检查深度
            if item.depth > self.config.max_depth {
                continue;
            }
            
            // 检查是否已访问
            {
                let visited = self.visited.read().await;
                if visited.contains(&item.url) {
                    continue;
                }
            }
            
            // 标记为已访问
            self.visited.write().await.insert(item.url.clone());
            
            // 爬取页面
            let result = self.crawl_page(&item).await?;
            
            // 添加新链接到队列
            if item.depth < self.config.max_depth {
                let mut queue = self.queue.write().await;
                for link in &result.links {
                    if !self.visited.read().await.contains(link) {
                        queue.push_back(CrawlItem {
                            url: link.clone(),
                            depth: item.depth + 1,
                        });
                    }
                }
            }
            
            results.push(result);
            pages_crawled += 1;
            
            // 请求间隔
            if self.config.request_delay_ms > 0 {
                tokio::time::sleep(
                    std::time::Duration::from_millis(self.config.request_delay_ms)
                ).await;
            }
        }
        
        Ok(results)
    }
    
    /// 爬取单个页面
    async fn crawl_page(&self, item: &CrawlItem) -> Result<CrawlResult> {
        let start = std::time::Instant::now();
        
        // 发送请求
        let response = self.client.get(&item.url).await?;
        
        let response_time = start.elapsed();
        
        // 检查状态码
        if !response.status.is_success() {
            return Ok(CrawlResult {
                url: item.url.clone(),
                depth: item.depth,
                status_code: response.status,
                response_time,
                resources: Vec::new(),
                links: Vec::new(),
                error: Some(format!("HTTP {}", response.status)),
            });
        }
        
        // 解析内容
        let content = String::from_utf8_lossy(&response.body);
        let parser = HtmlParser::from_response(&response)?;
        
        // 提取资源
        let resources = self.extract_resources(&parser, &item.url);
        
        // 提取链接
        let links = self.extract_links(&parser, &item.url);
        
        Ok(CrawlResult {
            url: item.url.clone(),
            depth: item.depth,
            status_code: response.status,
            response_time,
            resources,
            links,
            error: None,
        })
    }
    
    /// 提取资源
    fn extract_resources(&self, parser: &HtmlParser, url: &Url) -> Vec<Resource> {
        let mut resources = Vec::new();
        
        // 提取图片
        for img in parser.select("img[src]").unwrap_or_default() {
            if let Some(img_url) = img.abs_url("src") {
                resources.push(Resource {
                    url: img_url,
                    source_url: Some(url.clone()),
                    resource_type: ResourceType::Image,
                    content_type: None,
                    size: None,
                    filename: img.attr("src").and_then(extract_filename_from_path),
                    alt_text: img.attr("alt"),
                    metadata: std::collections::HashMap::new(),
                });
            }
        }
        
        // 提取样式表
        for link in parser.select("link[rel='stylesheet']").unwrap_or_default() {
            if let Some(css_url) = link.abs_url("href") {
                resources.push(Resource {
                    url: css_url,
                    source_url: Some(url.clone()),
                    resource_type: ResourceType::Css,
                    content_type: None,
                    size: None,
                    filename: link.attr("href").and_then(extract_filename_from_path),
                    alt_text: None,
                    metadata: std::collections::HashMap::new(),
                });
            }
        }
        
        // 提取脚本
        for script in parser.select("script[src]").unwrap_or_default() {
            if let Some(js_url) = script.abs_url("src") {
                resources.push(Resource {
                    url: js_url,
                    source_url: Some(url.clone()),
                    resource_type: ResourceType::JavaScript,
                    content_type: None,
                    size: None,
                    filename: script.attr("src").and_then(extract_filename_from_path),
                    alt_text: None,
                    metadata: std::collections::HashMap::new(),
                });
            }
        }
        
        resources
    }
    
    /// 提取链接
    fn extract_links(&self, parser: &HtmlParser, base_url: &Url) -> Vec<Url> {
        parser.select("a[href]")
            .unwrap_or_default()
            .into_iter()
            .filter_map(|a| a.abs_url("href"))
            .filter(|url| {
                // 过滤非 HTTP(S) 链接
                matches!(url.scheme(), "http" | "https")
            })
            .filter(|url| {
                // 过滤域名
                if let Some(allowed) = &self.config.allowed_domains {
                    if let Some(host) = url.host_str() {
                        return allowed.iter().any(|d| host.ends_with(d));
                    }
                }
                true
            })
            .collect()
    }
}

fn extract_filename_from_path(path: &str) -> Option<String> {
    path.rsplit('/').next().map(|s| s.to_string())
}
```

---

### 2. 批量资源抓取

```rust
// src/fetch/batch.rs

use std::sync::Arc;
use tokio::sync::Semaphore;
use url::Url;
use crate::error::Result;
use crate::extract::Resource;

/// 批量抓取器
pub struct BatchFetcher {
    client: FetchClient,
    concurrency: usize,
}

impl BatchFetcher {
    pub fn new(client: FetchClient, concurrency: usize) -> Self {
        Self { client, concurrency }
    }
    
    /// 批量获取资源信息
    pub async fn fetch_info(&self, resources: &[Resource]) -> Vec<(Url, Result<ResourceInfo>)> {
        let semaphore = Arc::new(Semaphore::new(self.concurrency));
        let mut tasks = Vec::new();
        
        for resource in resources {
            let client = self.client.clone();
            let semaphore = semaphore.clone();
            let url = resource.url.clone();
            
            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let result = client.head(&url).await;
                (url, result.map(|info| ResourceInfo {
                    content_type: info.content_type,
                    size: info.content_length,
                }))
            }));
        }
        
        let results: Vec<_> = futures::future::join_all(tasks).await;
        results.into_iter()
            .filter_map(|r| r.ok())
            .collect()
    }
}

/// 资源信息
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub content_type: Option<String>,
    pub size: Option<u64>,
}
```

---

## 测试用例

### 1. HTML 解析器测试

```rust
// tests/parse_test.rs

use turbo_crawler::parse::HtmlParser;
use url::Url;

#[test]
fn test_html_parser_basic() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Test Page</title>
            <meta name="description" content="Test description">
        </head>
        <body>
            <h1>Welcome</h1>
            <a href="/page1">Page 1</a>
            <img src="/img/logo.png" alt="Logo">
        </body>
        </html>
    "#;
    
    let base_url: Url = "https://example.com".parse().unwrap();
    let parser = HtmlParser::from_html(html, Some(base_url));
    
    // 测试标题
    assert_eq!(parser.title(), Some("Test Page".to_string()));
    
    // 测试 meta
    assert_eq!(parser.meta("description"), Some("Test description".to_string()));
    
    // 测试链接提取
    let links = parser.select("a[href]").unwrap();
    assert_eq!(links.len(), 1);
    
    let link = &links[0];
    assert_eq!(link.text(), "Page 1");
    assert_eq!(
        link.abs_url("href").unwrap().as_str(),
        "https://example.com/page1"
    );
    
    // 测试图片提取
    let images = parser.select("img[src]").unwrap();
    assert_eq!(images.len(), 1);
    
    let img = &images[0];
    assert_eq!(img.attr("alt"), Some("Logo".to_string()));
}

#[test]
fn test_relative_url_resolution() {
    let html = r#"<a href="../page.html">Link</a>"#;
    let base_url: Url = "https://example.com/docs/current/".parse().unwrap();
    let parser = HtmlParser::from_html(html, Some(base_url));
    
    let links = parser.select("a[href]").unwrap();
    assert_eq!(
        links[0].abs_url("href").unwrap().as_str(),
        "https://example.com/docs/page.html"
    );
}
```

---

### 2. 资源提取测试

```rust
// tests/extract_test.rs

use turbo_crawler::parse::HtmlParser;
use turbo_crawler::extract::{ImageExtractor, LinkExtractor};
use url::Url;

#[test]
fn test_image_extraction() {
    let html = r#"
        <img src="/img1.png" alt="Image 1">
        <img srcset="/img2-small.png 400w, /img2-large.png 800w">
        <img data-src="/lazy.png">
    "#;
    
    let base_url: Url = "https://example.com".parse().unwrap();
    let parser = HtmlParser::from_html(html, Some(base_url));
    let extractor = ImageExtractor::new(parser, None);
    
    let images = extractor.extract_all();
    
    // 应该提取 2 个图片 (img src 和 srcset)
    assert!(images.len() >= 2);
    
    // 检查第一个图片
    let first = images.iter().find(|i| i.url.path() == "/img1.png");
    assert!(first.is_some());
    assert_eq!(first.unwrap().alt_text, Some("Image 1".to_string()));
}

#[test]
fn test_link_extraction() {
    let html = r#"
        <nav>
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="https://other.com">External</a>
            <a href="mailto:test@example.com">Email</a>
        </nav>
    "#;
    
    let base_url: Url = "https://example.com".parse().unwrap();
    let parser = HtmlParser::from_html(html, Some(base_url));
    let extractor = LinkExtractor::new(parser);
    
    let links = extractor.extract_all();
    
    // 应该有 3 个 HTTP 链接 (排除 mailto)
    let http_links: Vec<_> = links.iter()
        .filter(|l| matches!(l.url.scheme(), "http" | "https"))
        .collect();
    
    assert_eq!(http_links.len(), 3);
}
```

---

### 3. 过滤器测试

```rust
// tests/filter_test.rs

use turbo_crawler::filter::{UrlFilter, UrlFilterBuilder, FilterResult};
use url::Url;

#[test]
fn test_url_filter_domains() {
    let filter = UrlFilterBuilder::new()
        .allow_domains(vec!["example.com".to_string()])
        .build();
    
    let allowed: Url = "https://example.com/page".parse().unwrap();
    let subdomain: Url = "https://sub.example.com/page".parse().unwrap();
    let blocked: Url = "https://other.com/page".parse().unwrap();
    
    assert_eq!(filter.should_allow(&allowed), FilterResult::Allowed);
    assert_eq!(filter.should_allow(&subdomain), FilterResult::Allowed);
    assert!(matches!(filter.should_allow(&blocked), FilterResult::Denied(_)));
}

#[test]
fn test_url_filter_patterns() {
    let filter = UrlFilterBuilder::new()
        .block_patterns(vec![r"\.pdf$", r"\.zip$"])
        .build();
    
    let html: Url = "https://example.com/page.html".parse().unwrap();
    let pdf: Url = "https://example.com/doc.pdf".parse().unwrap();
    let zip: Url = "https://example.com/archive.zip".parse().unwrap();
    
    assert_eq!(filter.should_allow(&html), FilterResult::Allowed);
    assert!(matches!(filter.should_allow(&pdf), FilterResult::Denied(_)));
    assert!(matches!(filter.should_allow(&zip), FilterResult::Denied(_)));
}
```

---

### 4. 限速测试

```rust
// tests/rate_limit_test.rs

use turbo_crawler::rate_limit::{TokenBucket, RequestThrottle};
use std::time::Duration;
use url::Url;

#[tokio::test]
async fn test_token_bucket() {
    let bucket = TokenBucket::new(3, 1); // 容量 3，每秒补充 1
    
    // 可以获取 3 个令牌
    assert!(bucket.try_acquire());
    assert!(bucket.try_acquire());
    assert!(bucket.try_acquire());
    
    // 第 4 个应该失败
    assert!(!bucket.try_acquire());
    
    // 等待 1 秒后应该能获取 1 个
    tokio::time::sleep(Duration::from_millis(1100)).await;
    assert!(bucket.try_acquire());
}

#[tokio::test]
async fn test_request_throttle() {
    let throttle = RequestThrottle::new(
        10,  // 全局 10/秒
        2,   // 每域名 2/秒
        Duration::from_millis(100),  // 最小间隔 100ms
    );
    
    let url: Url = "https://example.com/page".parse().unwrap();
    
    let start = std::time::Instant::now();
    
    // 发送 3 个请求
    throttle.wait_for_permission(&url).await;
    throttle.wait_for_permission(&url).await;
    throttle.wait_for_permission(&url).await;
    
    let elapsed = start.elapsed();
    
    // 由于最小间隔 100ms，3 个请求应该至少 200ms
    assert!(elapsed >= Duration::from_millis(200));
}
```

---

## 示例代码

### 基础爬取示例

```rust
// examples/basic_crawl.rs

use turbo_crawler::{Crawler, CrawlConfig};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let config = CrawlConfig::from_url("https://example.com")?
        .with_max_depth(2)
        .with_concurrency(3)
        .with_delay(500);
    
    // 创建爬虫
    let crawler = Crawler::new(config)?;
    
    // 执行爬取
    let results = crawler.crawl().await?;
    
    // 输出结果
    println!("Crawled {} pages", results.len());
    
    for result in results {
        println!("\n=== {} (depth: {}) ===", result.url, result.depth);
        println!("Status: {}", result.status_code);
        println!("Resources: {}", result.resources.len());
        println!("Links: {}", result.links.len());
        
        if !result.images().is_empty() {
            println!("\nImages:");
            for img in result.images() {
                println!("  - {}", img.url);
            }
        }
    }
    
    Ok(())
}
```

### 资源提取示例

```rust
// examples/resource_extract.rs

use turbo_crawler::{FetchClient, HtmlParser};
use turbo_crawler::extract::{ImageExtractor, LinkExtractor, ResourceType};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: Url = "https://example.com".parse()?;
    
    // 获取页面
    let client = FetchClient::new(Default::default())?;
    let response = client.get(&url).await?;
    
    // 解析 HTML
    let parser = HtmlParser::from_response(&response)?;
    
    // 提取图片
    let img_extractor = ImageExtractor::new(parser.clone(), None);
    let images = img_extractor.extract_all();
    
    println!("Found {} images:", images.len());
    for img in images {
        println!("  - {} ({:?})", img.url, img.source);
    }
    
    // 提取链接
    let link_extractor = LinkExtractor::new(parser);
    let links = link_extractor.extract_all();
    
    println!("\nFound {} links:", links.len());
    for link in links {
        println!("  - {} ({:?})", link.url, link.link_type);
    }
    
    Ok(())
}
```

### 过滤爬取示例

```rust
// examples/filtered_crawl.rs

use turbo_crawler::{Crawler, CrawlConfig};
use turbo_crawler::filter::{ResourceFilterBuilder, ResourceType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建资源过滤器 - 只下载图片和 CSS
    let resource_filter = ResourceFilterBuilder::new()
        .include_types(vec![ResourceType::Image, ResourceType::Css])
        .max_size(5 * 1024 * 1024)  // 最大 5MB
        .exclude_patterns(vec![r"ads\.", r"tracking\."])
        .build();
    
    let config = CrawlConfig::from_url("https://example.com")?
        .with_max_depth(2)
        .with_concurrency(5);
    
    // 设置资源过滤器
    let config = CrawlConfig {
        resource_filter,
        ..config
    };
    
    let crawler = Crawler::new(config)?;
    let results = crawler.crawl().await?;
    
    // 统计资源
    let mut total_images = 0;
    let mut total_css = 0;
    
    for result in &results {
        total_images += result.images().len();
        total_css += result.stylesheets().len();
    }
    
    println!("Pages: {}", results.len());
    println!("Images: {}", total_images);
    println!("CSS files: {}", total_css);
    
    Ok(())
}
```