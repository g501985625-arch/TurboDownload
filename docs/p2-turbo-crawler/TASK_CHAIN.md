# P2: turbo-crawler 详细任务链

## 任务概览

| 任务编号 | 任务名称 | 预估时间 | 依赖任务 |
|----------|----------|----------|----------|
| T2.1 | 项目初始化 | 2h | 无 |
| T2.2 | HTTP 抓取核心 | 4h | T2.1 |
| T2.3 | HTML 解析器 | 5h | T2.2 |
| T2.4 | 资源提取器 | 6h | T2.3 |
| T2.5 | 过滤与限速 | 4h | T2.2 |
| T2.6 | Cookie 管理 | 2h | T2.2 |
| T2.7 | 测试与优化 | 8h | T2.1-T2.6 |
| T2.8 | 文档与示例 | 3h | T2.7 |

**总工时**: 34h (约 4.5 个工作日)

---

## T2.1: 项目初始化

### T2.1.1: 创建 Rust crate 结构

**时间**: 0.5h  
**依赖**: 无

#### 步骤

1. **创建项目目录**
   ```bash
   cd ~/Projects/TurboDownload
   mkdir -p crates/turbo-crawler
   cd crates/turbo-crawler
   cargo init --lib
   ```

2. **创建模块文件**
   ```bash
   mkdir -p src/{fetch,parse,extract,filter,rate_limit,cookie,error}
   touch src/fetch/{mod.rs,client.rs,request.rs,response.rs}
   touch src/parse/{mod.rs,html.rs,css.rs,js.rs}
   touch src/extract/{mod.rs,links.rs,images.rs,media.rs,fonts.rs}
   touch src/filter/{mod.rs,url_filter.rs,mime_filter.rs,size_filter.rs}
   touch src/rate_limit/{mod.rs,token_bucket.rs,throttle.rs}
   touch src/cookie/{mod.rs,jar.rs,policy.rs}
   touch src/error/{mod.rs,types.rs}
   ```

3. **配置 lib.rs 入口**
   ```rust
   //! Turbo Crawler - High-performance web resource crawler
   //!
   //! # Features
   //! - HTML/CSS/JS parsing
   //! - Resource extraction
   //! - URL filtering
   //! - Rate limiting

   pub mod error;
   pub mod fetch;
   pub mod parse;
   pub mod extract;
   pub mod filter;
   pub mod rate_limit;
   pub mod cookie;

   pub use error::{CrawlError, Result};
   pub use fetch::{Crawler, CrawlConfig};
   pub use extract::{Resource, ResourceType};
   pub use filter::{ResourceFilter, UrlFilter};
   ```

#### 验收标准

- [ ] `cargo check` 通过
- [ ] 目录结构符合规范
- [ ] 模块导入无错误

---

### T2.1.2: 配置 Cargo.toml 依赖

**时间**: 0.5h  
**依赖**: T2.1.1

#### 步骤

1. **编辑 Cargo.toml**
   ```toml
   [package]
   name = "turbo-crawler"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   tokio = { workspace = true }
   reqwest = { workspace = true }
   scraper = "0.18"
   url = { workspace = true }
   serde = { workspace = true }
   thiserror = { workspace = true }
   tracing = { workspace = true }
   regex = "1.10"
   cssparser = "0.33"
   robotstxt = "0.3"
   chrono = { version = "0.4", features = ["serde"] }
   parking_lot = "0.12"
   mime = "0.3"
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

### T2.1.3: 创建测试目录结构

**时间**: 0.5h  
**依赖**: T2.1.1

#### 步骤

1. **创建测试目录**
   ```bash
   mkdir -p tests
   touch tests/{mod.rs,fetch_test.rs,parse_test.rs,extract_test.rs,filter_test.rs}
   ```

2. **创建测试框架**
   ```rust
   // tests/mod.rs
   pub mod fetch_test;
   pub mod parse_test;
   pub mod extract_test;
   pub mod filter_test;

   pub fn setup_test_env() {
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

   pub fn mock_html_response(server: &MockServer, path: &str, html: &str) {
       // 配置 mock HTML 响应
   }
   ```

#### 验收标准

- [ ] 测试目录结构完整
- [ ] 测试框架可运行
- [ ] Mock 服务器可用

---

### T2.1.4: 配置开发工具

**时间**: 0.5h  
**依赖**: T2.1.1

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

#### 验收标准

- [ ] `cargo fmt` 格式化正确
- [ ] `cargo clippy` 无警告
- [ ] 开发工具链配置完成

---

## T2.2: HTTP 抓取核心

### T2.2.1: 设计 FetchClient 结构体

**时间**: 1.5h  
**依赖**: T2.1

#### 步骤

1. **定义配置结构**
   ```rust
   // src/fetch/client.rs
   use std::time::Duration;
   use std::collections::HashMap;

   /// 抓取客户端配置
   #[derive(Debug, Clone)]
   pub struct FetchClientConfig {
       /// 请求超时时间
       pub timeout: Duration,
       /// 连接超时时间
       pub connect_timeout: Duration,
       /// 用户代理
       pub user_agent: String,
       /// 默认请求头
       pub default_headers: HashMap<String, String>,
       /// 是否跟随重定向
       pub follow_redirects: bool,
       /// 最大重定向次数
       pub max_redirects: usize,
       /// 代理配置
       pub proxy: Option<String>,
   }

   impl Default for FetchClientConfig {
       fn default() -> Self {
           Self {
               timeout: Duration::from_secs(30),
               connect_timeout: Duration::from_secs(10),
               user_agent: format!("TurboCrawler/{}", env!("CARGO_PKG_VERSION")),
               default_headers: HashMap::new(),
               follow_redirects: true,
               max_redirects: 10,
               proxy: None,
           }
       }
   }
   ```

2. **定义客户端结构**
   ```rust
   /// HTTP 抓取客户端
   #[derive(Debug, Clone)]
   pub struct FetchClient {
       client: reqwest::Client,
       config: FetchClientConfig,
   }

   impl FetchClient {
       /// 创建新的抓取客户端
       pub fn new(config: FetchClientConfig) -> Result<Self, CrawlError> {
           let mut builder = reqwest::Client::builder()
               .timeout(config.timeout)
               .connect_timeout(config.connect_timeout)
               .user_agent(&config.user_agent);

           if config.follow_redirects {
               builder = builder.redirect(
                   reqwest::redirect::Policy::limited(config.max_redirects)
               );
           }

           if let Some(ref proxy) = config.proxy {
               builder = builder.proxy(
                   reqwest::Proxy::all(proxy)?
               );
           }

           for (key, value) in &config.default_headers {
               builder = builder.header(key, value);
           }

           let client = builder.build()?;
           Ok(Self { client, config })
       }
   }
   ```

3. **定义构建器**
   ```rust
   /// 抓取客户端构建器
   pub struct FetchClientBuilder {
       config: FetchClientConfig,
   }

   impl FetchClientBuilder {
       pub fn new() -> Self {
           Self {
               config: FetchClientConfig::default(),
           }
       }

       pub fn timeout(mut self, timeout: Duration) -> Self {
           self.config.timeout = timeout;
           self
       }

       pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
           self.config.user_agent = user_agent.into();
           self
       }

       pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
           self.config.proxy = Some(proxy.into());
           self
       }

       pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
           self.config.default_headers.insert(key.into(), value.into());
           self
       }

       pub fn build(self) -> Result<FetchClient, CrawlError> {
           FetchClient::new(self.config)
       }
   }
   ```

#### 验收标准

- [ ] FetchClient 可正常创建
- [ ] 配置选项全部可用
- [ ] 单元测试覆盖配置构建

---

### T2.2.2: 实现 GET 请求与响应处理

**时间**: 1.5h  
**依赖**: T2.2.1

#### 步骤

1. **定义响应结构**
   ```rust
   // src/fetch/response.rs
   use url::Url;

   /// 抓取响应
   #[derive(Debug, Clone)]
   pub struct FetchResponse {
       /// 最终 URL (可能经过重定向)
       pub final_url: Url,
       /// 状态码
       pub status: u16,
       /// 响应头
       pub headers: HashMap<String, String>,
       /// 内容类型
       pub content_type: Option<String>,
       /// 响应体
       pub body: Vec<u8>,
       /// 响应时间 (毫秒)
       pub response_time_ms: u64,
   }
   ```

2. **实现请求方法**
   ```rust
   impl FetchClient {
       /// 发送 GET 请求
       pub async fn get(&self, url: &Url) -> Result<FetchResponse, CrawlError> {
           let start = std::time::Instant::now();
           
           let response = self.client
               .get(url.as_str())
               .send()
               .await
               .map_err(|e| CrawlError::Network(e.to_string()))?;

           let status = response.status().as_u16();
           let final_url = response.url().clone();
           
           let headers: HashMap<String, String> = response
               .headers()
               .iter()
               .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
               .collect();

           let content_type = headers.get("content-type").cloned();

           let body = response.bytes().await
               .map_err(|e| CrawlError::Network(e.to_string()))?
               .to_vec();

           Ok(FetchResponse {
               final_url,
               status,
               headers,
               content_type,
               body,
               response_time_ms: start.elapsed().as_millis() as u64,
           })
       }

       /// 检查 URL 是否可访问
       pub async fn head(&self, url: &Url) -> Result<HeadInfo, CrawlError> {
           let response = self.client
               .head(url.as_str())
               .send()
               .await
               .map_err(|e| CrawlError::Network(e.to_string()))?;

           Ok(HeadInfo {
               status: response.status().as_u16(),
               content_type: response.headers()
                   .get("content-type")
                   .and_then(|v| v.to_str().ok())
                   .map(String::from),
               content_length: response.headers()
                   .get("content-length")
                   .and_then(|v| v.to_str().ok())
                   .and_then(|v| v.parse().ok()),
           })
       }
   }

   /// HEAD 请求信息
   #[derive(Debug, Clone)]
   pub struct HeadInfo {
       pub status: u16,
       pub content_type: Option<String>,
       pub content_length: Option<u64>,
   }
   ```

3. **添加请求构建器**
   ```rust
   // src/fetch/request.rs
   use url::Url;
   use std::collections::HashMap;

   /// 请求构建器
   pub struct RequestBuilder {
       url: Url,
       method: Method,
       headers: HashMap<String, String>,
       body: Option<Vec<u8>>,
   }

   #[derive(Debug, Clone, Copy)]
   pub enum Method {
       GET,
       POST,
       HEAD,
   }

   impl RequestBuilder {
       pub fn new(url: Url) -> Self {
           Self {
               url,
               method: Method::GET,
               headers: HashMap::new(),
               body: None,
           }
       }

       pub fn method(mut self, method: Method) -> Self {
           self.method = method;
           self
       }

       pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
           self.headers.insert(key.into(), value.into());
           self
       }

       pub fn body(mut self, body: Vec<u8>) -> Self {
           self.body = Some(body);
           self
       }
   }
   ```

#### 验收标准

- [ ] GET 请求正常工作
- [ ] HEAD 请求正常工作
- [ ] 重定向处理正确
- [ ] 响应解析正确
- [ ] 单元测试覆盖

---

### T2.2.3: 实现并发请求控制

**时间**: 1h  
**依赖**: T2.2.2

#### 步骤

1. **定义并发控制器**
   ```rust
   // src/fetch/concurrency.rs
   use std::sync::Arc;
   use tokio::sync::Semaphore;

   /// 并发控制器
   #[derive(Clone)]
   pub struct ConcurrencyControl {
       semaphore: Arc<Semaphore>,
   }

   impl ConcurrencyControl {
       pub fn new(max_concurrent: usize) -> Self {
           Self {
               semaphore: Arc::new(Semaphore::new(max_concurrent)),
           }
       }

       pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
           self.semaphore.acquire().await.unwrap()
       }
   }
   ```

2. **集成到客户端**
   ```rust
   impl FetchClient {
       /// 批量获取 URL
       pub async fn get_many(
           &self,
           urls: &[Url],
           concurrency: usize,
       ) -> Vec<(Url, Result<FetchResponse, CrawlError>)> {
           let control = ConcurrencyControl::new(concurrency);
           let mut tasks = Vec::new();

           for url in urls {
               let client = self.clone();
               let control = control.clone();
               let url = url.clone();

               tasks.push(tokio::spawn(async move {
                   let _permit = control.acquire().await;
                   let result = client.get(&url).await;
                   (url, result)
               }));
           }

           let results: Vec<_> = futures::future::join_all(tasks).await;
           results.into_iter()
               .filter_map(|r| r.ok())
               .collect()
       }
   }
   ```

#### 验收标准

- [ ] 并发数限制有效
- [ ] 批量请求正常工作
- [ ] 无资源泄漏

---

## T2.3: HTML 解析器

### T2.3.1: 实现 HTML 解析核心

**时间**: 2h  
**依赖**: T2.2

#### 步骤

1. **定义解析器结构**
   ```rust
   // src/parse/html.rs
   use scraper::{Html, Selector};
   use url::Url;

   /// HTML 解析器
   pub struct HtmlParser {
       document: Html,
       base_url: Option<Url>,
   }

   impl HtmlParser {
       /// 从 HTML 字符串创建解析器
       pub fn from_html(html: &str, base_url: Option<Url>) -> Self {
           Self {
               document: Html::parse_document(html),
               base_url,
           }
       }

       /// 从响应创建解析器
       pub fn from_response(response: &FetchResponse) -> Result<Self, CrawlError> {
           let html = String::from_utf8_lossy(&response.body);
           Ok(Self {
               document: Html::parse_document(&html),
               base_url: Some(response.final_url.clone()),
           })
       }
   }
   ```

2. **实现元素选择**
   ```rust
   impl HtmlParser {
       /// 选择所有匹配元素
       pub fn select(&self, selector: &str) -> Result<Vec<Element>, CrawlError> {
           let selector = Selector::parse(selector)
               .map_err(|e| CrawlError::Parse(format!("Invalid selector: {:?}", e)))?;

           Ok(self.document
               .select(&selector)
               .map(|element| Element::from_scraper(element, self.base_url.as_ref()))
               .collect())
       }

       /// 选择第一个匹配元素
       pub fn select_first(&self, selector: &str) -> Result<Option<Element>, CrawlError> {
           let elements = self.select(selector)?;
           Ok(elements.into_iter().next())
       }

       /// 获取标题
       pub fn title(&self) -> Option<String> {
           self.select_first("title")
               .ok()
               .flatten()
               .map(|e| e.text())
       }

       /// 获取 meta 信息
       pub fn meta(&self, name: &str) -> Option<String> {
           self.select_first(&format!("meta[name=\"{}\"]", name))
               .ok()
               .flatten()
               .and_then(|e| e.attr("content"))
       }
   }
   ```

3. **定义元素结构**
   ```rust
   /// HTML 元素
   #[derive(Debug, Clone)]
   pub struct Element {
       /// 标签名
       pub tag: String,
       /// 属性
       pub attrs: HashMap<String, String>,
       /// 文本内容
       pub text: String,
       /// 基础 URL
       base_url: Option<Url>,
   }

   impl Element {
       fn from_scraper(element: scraper::ElementRef, base_url: Option<&Url>) -> Self {
           let el = element.value();
           
           Self {
               tag: el.name().to_string(),
               attrs: el.attrs()
                   .map(|(k, v)| (k.to_string(), v.to_string()))
                   .collect(),
               text: element.text().collect::<String>(),
               base_url: base_url.cloned(),
           }
       }

       /// 获取属性
       pub fn attr(&self, name: &str) -> Option<String> {
           self.attrs.get(name).cloned()
       }

       /// 获取文本内容
       pub fn text(&self) -> String {
           self.text.trim().to_string()
       }

       /// 获取绝对 URL
       pub fn abs_url(&self, attr: &str) -> Option<Url> {
           self.attr(attr)
               .and_then(|v| {
                   if let Some(base) = &self.base_url {
                       base.join(&v).ok()
                   } else {
                       v.parse().ok()
                   }
               })
       }
   }
   ```

#### 验收标准

- [ ] CSS 选择器工作正常
- [ ] 元素属性提取正确
- [ ] URL 解析正确
- [ ] 单元测试覆盖

---

### T2.3.2: 实现 CSS 解析

**时间**: 1.5h  
**依赖**: T2.3.1

#### 步骤

1. **定义 CSS 解析器**
   ```rust
   // src/parse/css.rs
   use cssparser::{Parser, ParserInput, Token};

   /// CSS 解析器
   pub struct CssParser {
       base_url: Option<Url>,
   }

   impl CssParser {
       pub fn new(base_url: Option<Url>) -> Self {
           Self { base_url }
       }

       /// 从 CSS 文本中提取 URL
       pub fn extract_urls(&self, css: &str) -> Vec<CssUrl> {
           let mut urls = Vec::new();
           let mut input = ParserInput::new(css);
           let mut parser = Parser::new(&mut input);

           while let Ok(token) = parser.next_including_whitespace_and_comments() {
               if let Token::UnquotedUrl(url) = token {
                   if let Some(absolute) = self.resolve_url(url) {
                       urls.push(CssUrl {
                           url: absolute,
                           context: UrlContext::UrlDeclaration,
                       });
                   }
               }
           }

           urls
       }

       fn resolve_url(&self, relative: &str) -> Option<Url> {
           if let Some(base) = &self.base_url {
               base.join(relative).ok()
           } else {
               relative.parse().ok()
           }
       }
   }

   /// CSS 中的 URL
   #[derive(Debug, Clone)]
   pub struct CssUrl {
       pub url: Url,
       pub context: UrlContext,
   }

   #[derive(Debug, Clone, Copy)]
   pub enum UrlContext {
       UrlDeclaration,
       Import,
       FontFace,
   }
   ```

2. **实现 @import 和 @font-face 解析**
   ```rust
   impl CssParser {
       /// 解析 @import 规则
       pub fn parse_imports(&self, css: &str) -> Vec<Url> {
           let mut imports = Vec::new();
           // 使用正则匹配 @import 规则
           let re = regex::Regex::new(r"@import\s+(?:url\()?['\"]?([^'\"\)]+)['\"]?\)?").unwrap();
           
           for cap in re.captures_iter(css) {
               if let Some(url_str) = cap.get(1) {
                   if let Some(url) = self.resolve_url(url_str.as_str()) {
                       imports.push(url);
                   }
               }
           }

           imports
       }

       /// 解析 @font-face 中的字体 URL
       pub fn parse_font_faces(&self, css: &str) -> Vec<FontSource> {
           let mut fonts = Vec::new();
           // 简化实现，实际应使用完整解析器
           let re = regex::Regex::new(r"url\(['\"]?([^'\"\)]+)['\"]?\)").unwrap();
           
           for cap in re.captures_iter(css) {
               if let Some(url_str) = cap.get(1) {
                   if let Some(url) = self.resolve_url(url_str.as_str()) {
                       fonts.push(FontSource { url, format: None });
                   }
               }
           }

           fonts
       }
   }

   #[derive(Debug, Clone)]
   pub struct FontSource {
       pub url: Url,
       pub format: Option<String>,
   }
   ```

#### 验收标准

- [ ] url() 解析正确
- [ ] @import 解析正确
- [ ] @font-face 解析正确
- [ ] 相对 URL 转换正确

---

### T2.3.3: 实现 JS 资源解析

**时间**: 1.5h  
**依赖**: T2.3.1

#### 步骤

1. **定义 JS 解析器**
   ```rust
   // src/parse/js.rs
   use regex::Regex;

   /// JS 资源解析器
   pub struct JsParser {
       base_url: Option<Url>,
   }

   impl JsParser {
       pub fn new(base_url: Option<Url>) -> Self {
           Self { base_url }
       }

       /// 提取 JS 中的资源 URL
       pub fn extract_resource_urls(&self, js: &str) -> Vec<ResourceMatch> {
           let mut results = Vec::new();

           // 匹配常见模式
           let patterns = vec![
               // fetch/axios
               (r#"fetch\s*\(\s*['"`]([^'"`]+)['"`]"#, "fetch"),
               // 动态 import
               (r#"import\s*\(\s*['"`]([^'"`]+)['"`]\s*\)"#, "dynamic_import"),
               // new URL
               (r#"new\s+URL\s*\(\s*['"`]([^'"`]+)['"`]"#, "new_url"),
               // 赋值语句中的字符串
               (r#"(?:src|href|url)\s*[=:]\s*['"`]([^'"`]+)['"`]"#, "assignment"),
           ];

           for (pattern, context) in patterns {
               let re = Regex::new(pattern).unwrap();
               for cap in re.captures_iter(js) {
                   if let Some(url_str) = cap.get(1) {
                       if let Some(url) = self.resolve_url(url_str.as_str()) {
                           results.push(ResourceMatch {
                               url,
                               context: context.to_string(),
                               line: self.find_line(js, cap.get(0).unwrap().start()),
                           });
                       }
                   }
               }
           }

           results
       }

       fn resolve_url(&self, relative: &str) -> Option<Url> {
           // 过滤掉明显不是 URL 的字符串
           if relative.starts_with("data:") || relative.starts_with("javascript:") {
               return None;
           }
           if relative.starts_with("${") || relative.contains("${") {
               return None; // 模板字符串
           }

           if let Some(base) = &self.base_url {
               base.join(relative).ok()
           } else {
               relative.parse().ok()
           }
       }

       fn find_line(&self, text: &str, pos: usize) -> usize {
           text[..pos].matches('\n').count() + 1
       }
   }

   #[derive(Debug, Clone)]
   pub struct ResourceMatch {
       pub url: Url,
       pub context: String,
       pub line: usize,
   }
   ```

#### 验收标准

- [ ] fetch 调用识别正确
- [ ] 动态 import 识别正确
- [ ] URL 模式匹配正确
- [ ] 过滤非 URL 字符串

---

## T2.4: 资源提取器

### T2.4.1: 实现链接提取器

**时间**: 1.5h  
**依赖**: T2.3

#### 步骤

1. **定义链接提取器**
   ```rust
   // src/extract/links.rs
   use crate::parse::HtmlParser;
   use url::Url;

   /// 链接提取器
   pub struct LinkExtractor {
       parser: HtmlParser,
   }

   impl LinkExtractor {
       pub fn new(parser: HtmlParser) -> Self {
           Self { parser }
       }

       /// 提取所有链接
       pub fn extract_all(&self) -> Vec<Link> {
           let mut links = Vec::new();

           // <a href="...">
           links.extend(self.extract_by_selector("a[href]", "href", LinkType::Anchor));
           
           // <link href="...">
           links.extend(self.extract_by_selector("link[href]", "href", LinkType::Resource));
           
           // <area href="...">
           links.extend(self.extract_by_selector("area[href]", "href", LinkType::Area));

           links
       }

       fn extract_by_selector(&self, selector: &str, attr: &str, link_type: LinkType) -> Vec<Link> {
           self.parser
               .select(selector)
               .unwrap_or_default()
               .into_iter()
               .filter_map(|element| {
                   element.abs_url(attr).map(|url| Link {
                       url,
                       text: element.text(),
                       link_type,
                       rel: element.attr("rel"),
                   })
               })
               .collect()
       }
   }

   #[derive(Debug, Clone)]
   pub struct Link {
       pub url: Url,
       pub text: String,
       pub link_type: LinkType,
       pub rel: Option<String>,
   }

   #[derive(Debug, Clone, Copy, PartialEq)]
   pub enum LinkType {
       Anchor,
       Resource,
       Area,
   }
   ```

#### 验收标准

- [ ] <a> 链接提取正确
- [ ] <link> 链接提取正确
- [ ] 相对 URL 转换正确

---

### T2.4.2: 实现图片提取器

**时间**: 1.5h  
**依赖**: T2.4.1

#### 步骤

1. **定义图片提取器**
   ```rust
   // src/extract/images.rs
   use crate::parse::{HtmlParser, CssParser};

   /// 图片提取器
   pub struct ImageExtractor {
       html_parser: HtmlParser,
       css_parser: Option<CssParser>,
   }

   impl ImageExtractor {
       pub fn new(html_parser: HtmlParser, css_parser: Option<CssParser>) -> Self {
           Self { html_parser, css_parser }
       }

       /// 提取所有图片资源
       pub fn extract_all(&self) -> Vec<ImageResource> {
           let mut images = Vec::new();

           // <img src="...">
           images.extend(self.extract_img_tags());
           
           // <picture> <source srcset="...">
           images.extend(self.extract_picture_tags());
           
           // CSS background-image
           if let Some(css) = &self.css_parser {
               images.extend(self.extract_css_images(css));
           }

           // srcset 属性
           images.extend(self.extract_srcset());

           images
       }

       fn extract_img_tags(&self) -> Vec<ImageResource> {
           self.html_parser
               .select("img[src]")
               .unwrap_or_default()
               .into_iter()
               .filter_map(|element| {
                   element.abs_url("src").map(|url| ImageResource {
                       url,
                       alt: element.attr("alt"),
                       width: element.attr("width").and_then(|w| w.parse().ok()),
                       height: element.attr("height").and_then(|h| h.parse().ok()),
                       source: ImageSource::ImgTag,
                   })
               })
               .collect()
       }

       fn extract_picture_tags(&self) -> Vec<ImageResource> {
           self.html_parser
               .select("picture source[srcset]")
               .unwrap_or_default()
               .into_iter()
               .flat_map(|element| {
                   self.parse_srcset(&element)
               })
               .collect()
       }

       fn parse_srcset(&self, element: &Element) -> Vec<ImageResource> {
           let mut images = Vec::new();
           
           if let Some(srcset) = element.attr("srcset") {
               for part in srcset.split(',') {
                   let parts: Vec<&str> = part.trim().split_whitespace().collect();
                   if let Some(url_str) = parts.first() {
                       if let Some(url) = element.abs_url_from(url_str) {
                           images.push(ImageResource {
                               url,
                               alt: None,
                               width: None,
                               height: None,
                               source: ImageSource::Srcset,
                           });
                       }
                   }
               }
           }

           images
       }
   }

   #[derive(Debug, Clone)]
   pub struct ImageResource {
       pub url: Url,
       pub alt: Option<String>,
       pub width: Option<u32>,
       pub height: Option<u32>,
       pub source: ImageSource,
   }

   #[derive(Debug, Clone, Copy)]
   pub enum ImageSource {
       ImgTag,
       Srcset,
       CssBackground,
       Picture,
   }
   ```

#### 验收标准

- [ ] <img> 提取正确
- [ ] srcset 解析正确
- [ ] CSS 图片提取正确
- [ ] picture/source 处理正确

---

### T2.4.3: 实现媒体资源提取器

**时间**: 1.5h  
**依赖**: T2.4.2

#### 步骤

1. **定义媒体提取器**
   ```rust
   // src/extract/media.rs
   use crate::parse::HtmlParser;

   /// 媒体资源提取器
   pub struct MediaExtractor {
       parser: HtmlParser,
   }

   impl MediaExtractor {
       pub fn new(parser: HtmlParser) -> Self {
           Self { parser }
       }

       /// 提取视频资源
       pub fn extract_videos(&self) -> Vec<MediaResource> {
           let mut videos = Vec::new();

           // <video src="...">
           videos.extend(self.extract_video_tags());
           
           // <video> <source src="...">
           videos.extend(self.extract_source_tags("video"));

           // <iframe> (嵌入视频)
           videos.extend(self.extract_iframe_videos());

           videos
       }

       /// 提取音频资源
       pub fn extract_audio(&self) -> Vec<MediaResource> {
           let mut audio = Vec::new();

           // <audio src="...">
           audio.extend(self.extract_audio_tags());
           
           // <audio> <source src="...">
           audio.extend(self.extract_source_tags("audio"));

           audio
       }

       fn extract_video_tags(&self) -> Vec<MediaResource> {
           self.parser
               .select("video[src], video > source[src]")
               .unwrap_or_default()
               .into_iter()
               .filter_map(|element| {
                   element.abs_url("src").map(|url| MediaResource {
                       url,
                       media_type: MediaType::Video,
                       format: element.attr("type"),
                       poster: element.attr("poster"),
                   })
               })
               .collect()
       }

       fn extract_iframe_videos(&self) -> Vec<MediaResource> {
           // 识别常见视频平台嵌入
           self.parser
               .select("iframe[src]")
               .unwrap_or_default()
               .into_iter()
               .filter_map(|element| {
                   element.abs_url("src").and_then(|url| {
                       if self.is_video_embed(&url) {
                           Some(MediaResource {
                               url,
                               media_type: MediaType::EmbeddedVideo,
                               format: None,
                               poster: None,
                           })
                       } else {
                           None
                       }
                   })
               })
               .collect()
       }

       fn is_video_embed(&self, url: &Url) -> bool {
           let host = url.host_str().unwrap_or("");
           host.contains("youtube.com") 
               || host.contains("vimeo.com")
               || host.contains("bilibili.com")
       }
   }

   #[derive(Debug, Clone)]
   pub struct MediaResource {
       pub url: Url,
       pub media_type: MediaType,
       pub format: Option<String>,
       pub poster: Option<String>,
   }

   #[derive(Debug, Clone, Copy, PartialEq)]
   pub enum MediaType {
       Video,
       Audio,
       EmbeddedVideo,
   }
   ```

#### 验收标准

- [ ] video 标签提取正确
- [ ] audio 标签提取正确
- [ ] source 标签处理正确
- [ ] iframe 视频识别正确

---

### T2.4.4: 实现字体提取器

**时间**: 1.5h  
**依赖**: T2.4.3

#### 步骤

```rust
   // src/extract/fonts.rs
   use crate::parse::{HtmlParser, CssParser};

   /// 字体提取器
   pub struct FontExtractor {
       html_parser: HtmlParser,
       css_parser: Option<CssParser>,
   }

   impl FontExtractor {
       pub fn new(html_parser: HtmlParser, css_parser: Option<CssParser>) -> Self {
           Self { html_parser, css_parser }
       }

       /// 提取所有字体资源
       pub fn extract_all(&self) -> Vec<FontResource> {
           let mut fonts = Vec::new();

           // <link rel="stylesheet"> 中的字体
           // CSS @font-face
           if let Some(css) = &self.css_parser {
               fonts.extend(self.extract_from_css(css));
           }

           fonts
       }

       fn extract_from_css(&self, css: &CssParser) -> Vec<FontResource> {
           // 需要先获取 CSS 内容，然后解析
           Vec::new() // 简化实现
       }
   }

   #[derive(Debug, Clone)]
   pub struct FontResource {
       pub url: Url,
       pub font_family: Option<String>,
       pub font_weight: Option<String>,
       pub font_style: Option<String>,
       pub format: Option<String>,
   }
   ```

#### 验收标准

- [ ] @font-face 解析正确
- [ ] 字体格式识别正确
- [ ] 字体属性提取正确

---

## T2.5: 过滤与限速

### T2.5.1: 实现 URL 过滤器

**时间**: 1.5h  
**依赖**: T2.2

#### 步骤

1. **定义 URL 过滤器**
   ```rust
   // src/filter/url_filter.rs
   use url::Url;
   use regex::Regex;

   /// URL 过滤器
   #[derive(Debug, Clone, Default)]
   pub struct UrlFilter {
       /// 允许的域名
       pub allowed_domains: Option<Vec<String>>,
       /// 禁止的域名
       pub blocked_domains: Vec<String>,
       /// 允许的路径模式
       pub allowed_patterns: Vec<Regex>,
       /// 禁止的路径模式
       pub blocked_patterns: Vec<Regex>,
       /// 允许的协议
       pub allowed_schemes: Vec<String>,
       /// 最大 URL 长度
       pub max_url_length: usize,
   }

   impl UrlFilter {
       /// 检查 URL 是否应该被过滤
       pub fn should_allow(&self, url: &Url) -> FilterResult {
           // 检查协议
           if !self.allowed_schemes.contains(&url.scheme().to_string()) {
               return FilterResult::Denied("Unsupported scheme".into());
           }

           // 检查 URL 长度
           if url.as_str().len() > self.max_url_length {
               return FilterResult::Denied("URL too long".into());
           }

           // 检查域名
           if let Some(host) = url.host_str() {
               // 检查禁止域名
               if self.blocked_domains.iter().any(|d| host.ends_with(d)) {
                   return FilterResult::Denied("Domain blocked".into());
               }

               // 检查允许域名
               if let Some(allowed) = &self.allowed_domains {
                   if !allowed.iter().any(|d| host.ends_with(d)) {
                       return FilterResult::Denied("Domain not allowed".into());
                   }
               }
           }

           // 检查路径模式
           let path = url.path();
           
           for pattern in &self.blocked_patterns {
               if pattern.is_match(path) {
                   return FilterResult::Denied("Pattern blocked".into());
               }
           }

           for pattern in &self.allowed_patterns {
               if pattern.is_match(path) {
                   return FilterResult::Allowed;
               }
           }

           FilterResult::Allowed
       }
   }

   #[derive(Debug, Clone, PartialEq)]
   pub enum FilterResult {
       Allowed,
       Denied(String),
   }
   ```

2. **定义过滤规则构建器**
   ```rust
   /// URL 过滤规则构建器
   pub struct UrlFilterBuilder {
       filter: UrlFilter,
   }

   impl UrlFilterBuilder {
       pub fn new() -> Self {
           Self {
               filter: UrlFilter {
                   allowed_schemes: vec!["http".into(), "https".into()],
                   max_url_length: 2048,
                   ..Default::default()
               },
           }
       }

       pub fn allow_domains(mut self, domains: Vec<String>) -> Self {
           self.filter.allowed_domains = Some(domains);
           self
       }

       pub fn block_domains(mut self, domains: Vec<String>) -> Self {
           self.filter.blocked_domains = domains;
           self
       }

       pub fn block_patterns(mut self, patterns: Vec<&str>) -> Self {
           self.filter.blocked_patterns = patterns
               .into_iter()
               .filter_map(|p| Regex::new(p).ok())
               .collect();
           self
       }

       pub fn build(self) -> UrlFilter {
           self.filter
       }
   }
   ```

#### 验收标准

- [ ] 域名过滤正确
- [ ] 正则匹配正确
- [ ] 协议过滤正确
- [ ] 默认规则合理

---

### T2.5.2: 实现 MIME 类型过滤器

**时间**: 1h  
**依赖**: T2.5.1

#### 步骤

```rust
   // src/filter/mime_filter.rs

   /// MIME 类型过滤器
   #[derive(Debug, Clone)]
   pub struct MimeFilter {
       /// 允许的 MIME 类型
       pub allowed_types: Vec<MimeType>,
       /// 禁止的 MIME 类型
       pub blocked_types: Vec<MimeType>,
   }

   impl MimeFilter {
       /// 检查 MIME 类型是否应该被过滤
       pub fn should_allow(&self, content_type: Option<&str>) -> FilterResult {
           let mime = match content_type {
               Some(ct) => MimeType::from_content_type(ct),
               None => MimeType::Unknown,
           };

           if self.blocked_types.contains(&mime) {
               return FilterResult::Denied("MIME type blocked".into());
           }

           if self.allowed_types.is_empty() || self.allowed_types.contains(&mime) {
               FilterResult::Allowed
           } else {
               FilterResult::Denied("MIME type not allowed".into())
           }
       }
   }

   #[derive(Debug, Clone, Copy, PartialEq)]
   pub enum MimeType {
       Html,
       Css,
       JavaScript,
       Image,
       Video,
       Audio,
       Font,
       Json,
       Xml,
       Pdf,
       Unknown,
   }

   impl MimeType {
       pub fn from_content_type(content_type: &str) -> Self {
           let mime = content_type.split(';').next().unwrap_or("").trim();
           
           match mime {
               "text/html" => Self::Html,
               "text/css" => Self::Css,
               "application/javascript" | "text/javascript" => Self::JavaScript,
               "application/json" => Self::Json,
               "application/pdf" => Self::Pdf,
               m if m.starts_with("image/") => Self::Image,
               m if m.starts_with("video/") => Self::Video,
               m if m.starts_with("audio/") => Self::Audio,
               m if m.starts_with("font/") => Self::Font,
               _ => Self::Unknown,
           }
       }

       pub fn from_extension(path: &str) -> Self {
           let ext = path.rsplit('.').next().unwrap_or("");
           
           match ext.to_lowercase().as_str() {
               "html" | "htm" => Self::Html,
               "css" => Self::Css,
               "js" | "mjs" => Self::JavaScript,
               "json" => Self::Json,
               "pdf" => Self::Pdf,
               "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" => Self::Image,
               "mp4" | "webm" | "avi" => Self::Video,
               "mp3" | "wav" | "ogg" => Self::Audio,
               "woff" | "woff2" | "ttf" | "otf" => Self::Font,
               _ => Self::Unknown,
           }
       }
   }
   ```

#### 验收标准

- [ ] MIME 类型识别正确
- [ ] 扩展名识别正确
- [ ] 过滤逻辑正确

---

### T2.5.3: 实现请求限速

**时间**: 1.5h  
**依赖**: T2.5.2

#### 步骤

1. **实现令牌桶算法**
   ```rust
   // src/rate_limit/token_bucket.rs
   use std::sync::atomic::{AtomicU64, Ordering};
   use std::time::{Duration, Instant};

   /// 令牌桶限速器
   pub struct TokenBucket {
       /// 桶容量
       capacity: u64,
       /// 当前令牌数
       tokens: AtomicU64,
       /// 每秒补充令牌数
       refill_rate: u64,
       /// 上次补充时间
       last_refill: std::sync::Mutex<Instant>,
   }

   impl TokenBucket {
       pub fn new(capacity: u64, refill_rate: u64) -> Self {
           Self {
               capacity,
               tokens: AtomicU64::new(capacity),
               refill_rate,
               last_refill: std::sync::Mutex::new(Instant::now()),
           }
       }

       /// 尝试获取令牌
       pub fn try_acquire(&self) -> bool {
           self.refill();
           
           loop {
               let current = self.tokens.load(Ordering::Relaxed);
               if current == 0 {
                   return false;
               }
               if self.tokens.compare_exchange(
                   current,
                   current - 1,
                   Ordering::Relaxed,
                   Ordering::Relaxed,
               ).is_ok() {
                   return true;
               }
           }
       }

       /// 等待并获取令牌
       pub async fn acquire(&self) {
           while !self.try_acquire() {
               tokio::time::sleep(Duration::from_millis(10)).await;
           }
       }

       fn refill(&self) {
           let mut last = self.last_refill.lock().unwrap();
           let now = Instant::now();
           let elapsed = now.duration_since(*last);
           
           let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate as f64) as u64;
           if tokens_to_add > 0 {
               let current = self.tokens.load(Ordering::Relaxed);
               let new_tokens = (current + tokens_to_add).min(self.capacity);
               self.tokens.store(new_tokens, Ordering::Relaxed);
               *last = now;
           }
       }
   }
   ```

2. **实现请求节流器**
   ```rust
   // src/rate_limit/throttle.rs
   use std::collections::HashMap;
   use std::sync::Arc;
   use url::Url;

   /// 请求节流器
   pub struct RequestThrottle {
       /// 全局限速器
       global: Arc<TokenBucket>,
       /// 每域名限速器
       per_domain: HashMap<String, Arc<TokenBucket>>,
       /// 域名限速配置
       domain_rate: u64,
       /// 最小请求间隔
       min_interval: Duration,
       /// 上次请求时间
       last_request: std::sync::Mutex<HashMap<String, Instant>>,
   }

   impl RequestThrottle {
       pub fn new(global_rate: u64, domain_rate: u64, min_interval: Duration) -> Self {
           Self {
               global: Arc::new(TokenBucket::new(global_rate, global_rate)),
               per_domain: HashMap::new(),
               domain_rate,
               min_interval,
               last_request: std::sync::Mutex::new(HashMap::new()),
           }
       }

       /// 等待许可
       pub async fn wait_for_permission(&self, url: &Url) {
           // 全局限速
           self.global.acquire().await;

           // 域名限速
           if let Some(host) = url.host_str() {
               let domain_bucket = self.get_or_create_domain_bucket(host);
               domain_bucket.acquire().await;

               // 最小间隔
               let mut last = self.last_request.lock().unwrap();
               if let Some(last_time) = last.get(host) {
                   let elapsed = last_time.elapsed();
                   if elapsed < self.min_interval {
                       tokio::time::sleep(self.min_interval - elapsed).await;
                   }
               }
               last.insert(host.to_string(), Instant::now());
           }
       }

       fn get_or_create_domain_bucket(&self, domain: &str) -> Arc<TokenBucket> {
           // 简化实现，实际应使用 RwLock
           Arc::new(TokenBucket::new(self.domain_rate, self.domain_rate))
       }
   }
   ```

#### 验收标准

- [ ] 令牌桶工作正常
- [ ] 域名级限速正常
- [ ] 最小间隔控制正确

---

## T2.6: Cookie 管理

### T2.6.1: 实现 Cookie 存储

**时间**: 1h  
**依赖**: T2.2

#### 步骤

```rust
   // src/cookie/jar.rs
   use std::collections::HashMap;
   use url::Url;

   /// Cookie 存储
   pub struct CookieJar {
       cookies: HashMap<String, Vec<Cookie>>,
   }

   impl CookieJar {
       pub fn new() -> Self {
           Self {
               cookies: HashMap::new(),
           }
       }

       /// 添加 Cookie
       pub fn add(&mut self, cookie: Cookie) {
           let domain = cookie.domain.clone();
           self.cookies
               .entry(domain)
               .or_insert_with(Vec::new)
               .push(cookie);
       }

       /// 获取 URL 相关的 Cookie
       pub fn get_for_url(&self, url: &Url) -> String {
           let host = url.host_str().unwrap_or("");
           let path = url.path();

           let mut relevant: Vec<&Cookie> = self.cookies
               .iter()
               .filter(|(domain, _)| host.ends_with(*domain))
               .flat_map(|(_, cookies)| cookies)
               .filter(|cookie| {
                   path.starts_with(&cookie.path) && !cookie.is_expired()
               })
               .collect();

           relevant.sort_by(|a, b| b.creation_time.cmp(&a.creation_time));

           relevant
               .iter()
               .map(|c| format!("{}={}", c.name, c.value))
               .collect::<Vec<_>>()
               .join("; ")
       }

       /// 清理过期 Cookie
       pub fn cleanup(&mut self) {
           for cookies in self.cookies.values_mut() {
               cookies.retain(|c| !c.is_expired());
           }
       }
   }

   #[derive(Debug, Clone)]
   pub struct Cookie {
       pub name: String,
       pub value: String,
       pub domain: String,
       pub path: String,
       pub expires: Option<i64>,
       pub max_age: Option<i64>,
       pub secure: bool,
       pub http_only: bool,
       pub creation_time: i64,
   }

   impl Cookie {
       pub fn parse(set_cookie: &str, default_domain: &str) -> Option<Self> {
           // 解析 Set-Cookie 头
           let parts: Vec<&str> = set_cookie.split(';').collect();
           let name_value: Vec<&str> = parts.first()?.splitn(2, '=').collect();
           
           if name_value.len() != 2 {
               return None;
           }

           let mut cookie = Self {
               name: name_value[0].trim().to_string(),
               value: name_value[1].trim().to_string(),
               domain: default_domain.to_string(),
               path: "/".to_string(),
               expires: None,
               max_age: None,
               secure: false,
               http_only: false,
               creation_time: chrono::Utc::now().timestamp(),
           };

           for part in parts.iter().skip(1) {
               let part = part.trim().to_lowercase();
               if part == "secure" {
                   cookie.secure = true;
               } else if part == "httponly" {
                   cookie.http_only = true;
               } else if let Some(v) = part.strip_prefix("domain=") {
                   cookie.domain = v.to_string();
               } else if let Some(v) = part.strip_prefix("path=") {
                   cookie.path = v.to_string();
               }
           }

           Some(cookie)
       }

       pub fn is_expired(&self) -> bool {
           if let Some(max_age) = self.max_age {
               return max_age <= 0;
           }
           if let Some(expires) = self.expires {
               return expires < chrono::Utc::now().timestamp();
           }
           false
       }
   }
   ```

#### 验收标准

- [ ] Cookie 解析正确
- [ ] Cookie 存储正确
- [ ] 过期处理正确

---

## T2.7: 测试与优化

### T2.7.1: 单元测试

**时间**: 4h  
**依赖**: T2.1-T2.6

#### 测试清单

```rust
// tests/fetch_test.rs
#[tokio::test]
async fn test_fetch_html() { /* ... */ }
#[tokio::test]
async fn test_concurrent_fetch() { /* ... */ }

// tests/parse_test.rs
#[test]
fn test_html_parsing() { /* ... */ }
#[test]
fn test_css_url_extraction() { /* ... */ }

// tests/extract_test.rs
#[test]
fn test_link_extraction() { /* ... */ }
#[test]
fn test_image_extraction() { /* ... */ }

// tests/filter_test.rs
#[test]
fn test_url_filter() { /* ... */ }
#[test]
fn test_mime_filter() { /* ... */ }
```

#### 验收标准

- [ ] 单元测试覆盖率 > 80%
- [ ] 所有测试通过
- [ ] Mock 服务器正常工作

---

### T2.7.2: 性能测试

**时间**: 2h  
**依赖**: T2.7.1

#### 步骤

1. **创建基准测试**
   ```rust
   // benches/crawl_benchmark.rs
   use criterion::{criterion_group, criterion_main, Criterion};

   fn benchmark_html_parsing(c: &mut Criterion) {
       let html = include_str!("../tests/fixtures/sample.html");
       c.bench_function("html_parse", |b| {
           b.iter(|| HtmlParser::from_html(html, None))
       });
   }

   criterion_group!(benches, benchmark_html_parsing);
   criterion_main!(benches);
   ```

#### 验收标准

- [ ] 基准测试结果记录
- [ ] 性能瓶颈分析

---

### T2.7.3: 内存优化

**时间**: 2h  
**依赖**: T2.7.2

#### 验收标准

- [ ] 内存使用稳定
- [ ] 无内存泄漏

---

## T2.8: 文档与示例

### T2.8.1: 编写 API 文档

**时间**: 2h  
**依赖**: T2.7

#### 步骤

```bash
cargo doc --no-deps --open
```

### T2.8.2: 编写示例代码

**时间**: 1h  
**依赖**: T2.8.1

创建示例：
- `examples/basic_crawl.rs`
- `examples/depth_crawl.rs`
- `examples/resource_extract.rs`

#### 验收标准

- [ ] 所有 API 有文档
- [ ] 示例可运行
- [ ] README 完整

---

## 任务依赖图

```
T2.1 项目初始化
  ├── T2.2 HTTP 抓取核心
  │     ├── T2.3 HTML 解析器
  │     │     └── T2.4 资源提取器
  │     ├── T2.5 过滤与限速
  │     └── T2.6 Cookie 管理
  └── T2.7 测试优化
        └── T2.8 文档示例
```

---

## 里程碑

| 里程碑 | 完成任务 | 预计时间 |
|--------|----------|----------|
| M1 | T2.1, T2.2 | Day 1 |
| M2 | T2.3, T2.4 | Day 2-3 |
| M3 | T2.5, T2.6 | Day 3-4 |
| M4 | T2.7, T2.8 | Day 5 |