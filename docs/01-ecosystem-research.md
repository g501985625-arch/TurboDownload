# TurboDownload 资源抓取功能 - Rust 生态调研报告

> 创建日期: 2026-03-25  
> 版本: 1.0  
> 状态: 深度调研完成

---

## 目录

1. [HTML 解析库调研](#1-html-解析库调研)
2. [HTTP 客户端库调研](#2-http-客户端库调研)
3. [Headless 浏览器方案调研](#3-headless-浏览器方案调研)
4. [视频处理相关库调研](#4-视频处理相关库调研)
5. [综合选型建议](#5-综合选型建议)

---

## 1. HTML 解析库调研

### 1.1 scraper (推荐 ⭐⭐⭐⭐⭐)

**仓库**: https://github.com/causal-agent/scraper  
**Crates.io**: https://crates.io/crates/scraper  
**最新版本**: 0.22.0 (截至 2024)

#### 核心特性
```toml
[dependencies]
scraper = "0.22"
```

```rust
use scraper::{Html, Selector};

// 解析 HTML
let html = Html::parse_document(r#"
    <html>
        <body>
            <div class="content">
                <a href="/video/123">Video 1</a>
                <a href="/video/456">Video 2</a>
            </div>
        </body>
    </html>
"#);

// CSS 选择器查询
let selector = Selector::parse("div.content a").unwrap();
for element in html.select(&selector) {
    let text = element.text().collect::<String>();
    let href = element.value().attr("href");
    println!("Text: {}, href: {:?}", text, href);
}
```

#### 优点
| 特性 | 描述 |
|------|------|
| **CSS 选择器支持** | 完整支持 CSS3 选择器语法，学习成本低 |
| **内存效率高** | 使用 `tendril` 库，零拷贝字符串处理 |
| **API 简洁** | 类似 jQuery 的链式操作，易于上手 |
| **纯 Rust 实现** | 无系统依赖，交叉编译友好 |
| **活跃维护** | 定期更新，社区活跃 |
| **文档完善** | 丰富的示例和文档 |

#### 缺点
| 特性 | 描述 |
|------|------|
| **仅解析不渲染** | 不执行 JavaScript，动态内容需配合其他方案 |
| **容错性一般** | 对严重畸形的 HTML 可能解析失败 |
| **无流式解析** | 需要完整文档加载到内存 |

#### 性能测试
```rust
// 性能基准测试示例
use scraper::{Html, Selector};
use std::time::Instant;

fn benchmark_scraper(html_content: &str) {
    let start = Instant::now();
    
    // 解析 1MB HTML 文件
    let document = Html::parse_document(html_content);
    let parse_time = start.elapsed();
    
    let selector = Selector::parse("a[href]").unwrap();
    let links: Vec<_> = document.select(&selector).collect();
    let query_time = start.elapsed() - parse_time;
    
    println!("Parse: {:?}, Query: {:?}, Links: {}", 
             parse_time, query_time, links.len());
}
```

---

### 1.2 kuchiki

**仓库**: https://github.com/kuchiki-rs/kuchiki  
**Crates.io**: https://crates.io/crates/kuchiki

#### 核心特性
```toml
[dependencies]
kuchiki = "0.8"
```

```rust
use kuchiki::traits::*;

// 解析 HTML
let document = kuchiki::parse_html().one(r#"
    <html>
        <body>
            <video src="/video.mp4"></video>
        </body>
    </html>
"#);

// 遍历 DOM 树
for node in document.select("video").unwrap() {
    let attributes = node.attributes.borrow();
    let src = attributes.get("src");
    println!("Video source: {:?}", src);
}
```

#### 优点
- 基于 html5ever，HTML5 标准兼容性好
- 提供完整的 DOM 树操作能力
- 支持遍历和修改 DOM

#### 缺点
- 维护不活跃（最近更新较少）
- API 相对复杂
- 性能略低于 scraper

---

### 1.3 html5ever

**仓库**: https://github.com/servo/html5ever  
**Crates.io**: https://crates.io/crates/html5ever

#### 核心特性
```toml
[dependencies]
html5ever = "0.27"
```

```rust
use html5ever::{parse_document, serialize};
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;

// 低级别解析
let dom: RcDom = parse_document(RcDom::default(), Default::default())
    .one(r#"<html><body>Hello</body></html>"#.to_tendril());

// 遍历 DOM
fn walk(node: &html5ever::rcdom::Handle, depth: usize) {
    println!("{}{:?}", "  ".repeat(depth), node.data);
    for child in node.children.borrow().iter() {
        walk(child, depth + 1);
    }
}
```

#### 优点
- Servo 项目官方组件，HTML5 规范完全兼容
- 浏览器级别的容错能力
- 流式解析支持

#### 缺点
- 低级 API，使用复杂
- 没有 CSS 选择器支持（需配合其他库）
- 学习曲线陡峭

---

### 1.4 select

**仓库**: https://github.com/utkarshkukreti/select.rs  
**Crates.io**: https://crates.io/crates/select

```toml
[dependencies]
select = "0.6"
```

```rust
use select::document::Document;
use select::predicate::{Attr, Class, Name};

let html = r#"<div class="video"><a href="/v1">Link</a></div>"#;
let doc = Document::from(html);

// 多种选择方式
for node in doc.find(Class("video")) {
    if let Some(link) = node.find(Name("a")).next() {
        println!("Link: {}", link.attr("href").unwrap());
    }
}
```

#### 优点
- 轻量级，依赖少
- 支持自定义谓词匹配

#### 缺点
- 社区活跃度较低
- 功能不如 scraper 完善
- 文档较少

---

### 1.5 HTML 解析库对比总结

| 特性 | scraper | kuchiki | html5ever | select |
|------|---------|---------|-----------|--------|
| **CSS 选择器** | ✅ 完整 | ✅ 基础 | ❌ 无 | ✅ 自定义谓词 |
| **性能** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **内存效率** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **HTML5 兼容** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **容错能力** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **维护活跃度** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |
| **易用性** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| **文档质量** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |

**推荐选择**: **scraper** 作为主要 HTML 解析库，对于需要浏览器级别容错的场景可补充 html5ever。

---

## 2. HTTP 客户端库调研

### 2.1 reqwest (推荐 ⭐⭐⭐⭐⭐)

**仓库**: https://github.com/seanmonstar/reqwest  
**Crates.io**: https://crates.io/crates/reqwest

#### 核心特性
```toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "cookies", "gzip", "brotli", "stream"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use reqwest::{Client, ClientBuilder, redirect, Proxy};
use std::time::Duration;

// 创建高级客户端
let client = ClientBuilder::new()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(10))
    .redirect(redirect::Policy::limited(5))
    .cookie_store(true)
    .gzip(true)
    .brotli(true)
    .proxy(Proxy::http("http://proxy.example.com:8080")?)
    .build()?;

// GET 请求
let response = client
    .get("https://example.com/video")
    .header("Accept", "video/webm,video/mp4")
    .header("Referer", "https://example.com")
    .send()
    .await?;

// 流式下载大文件
use futures::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

let mut file = File::create("video.mp4").await?;
let mut stream = response.bytes_stream();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    file.write_all(&chunk).await?;
}

// POST 表单
let form_data = [
    ("username", "user"),
    ("password", "pass"),
];
let response = client
    .post("https://example.com/login")
    .form(&form_data)
    .send()
    .await?;
```

#### 连接池配置
```rust
use reqwest::Client;

let client = Client::builder()
    .pool_max_idle_per_host(20)  // 每个主机最大空闲连接
    .pool_idle_timeout(Duration::from_secs(60))
    .tcp_keepalive(Duration::from_secs(30))
    .tcp_nodelay(true)
    .build()?;
```

#### 优点
| 特性 | 描述 |
|------|------|
| **异步优先** | 基于 Tokio，原生异步支持 |
| **功能丰富** | Cookie、重定向、压缩、流式传输全支持 |
| **类型安全** | 强类型 API，编译时检查 |
| **中间件生态** | reqwest-middleware 支持重试、日志等 |
| **WebSocket** | 可升级为 WebSocket 连接 |
| **文档完善** | 丰富的文档和示例 |

#### 缺点
- 依赖 OpenSSL（可通过 rustls 替代）
- 编译时间较长
- 不支持同步 API（需阻塞包装）

---

### 2.2 hyper

**仓库**: https://github.com/hyperium/hyper  
**Crates.io**: https://crates.io/crates/hyper

```toml
[dependencies]
hyper = { version = "1.0", features = ["full"] }
hyper-util = { version = "0.1", features = ["full"] }
http-body-util = "0.1"
```

```rust
use hyper::{body::Incoming, Request, Response, body::Buf};
use hyper::client::conn::http1::{Builder, SendRequest};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

// 原始 HTTP/1.1 连接
let stream = TcpStream::connect("example.com:80").await?;
let io = TokioIo::new(stream);

let (mut sender, conn) = Builder::new()
    .handshake(io)
    .await?;

// 启动连接任务
tokio::spawn(async move {
    if let Err(e) = conn.await {
        eprintln!("Connection error: {:?}", e);
    }
});

// 发送请求
let request = Request::builder()
    .uri("/video.mp4")
    .header("Host", "example.com")
    .body(())?;

let response = sender.send_request(request).await?;
let body = hyper::body::to_bytes(response).await?;
```

#### 优点
- hyper 是 reqwest 的底层依赖
- 零成本抽象，性能最高
- 完全控制 HTTP 协议细节
- 支持 HTTP/2 和 HTTP/3

#### 缺点
- API 低级，需要更多代码
- 没有高级功能（Cookie、重定向等）
- 学习曲线陡峭
- 不适合简单场景

---

### 2.3 isahc

**仓库**: https://github.com/sagebind/isahc  
**Crates.io**: https://crates.io/crates/isahc

```toml
[dependencies]
isahc = "1.7"
```

```rust
use isahc::{HttpClient, Configurable, ReadResponseExt};

let client = HttpClient::builder()
    .timeout(Duration::from_secs(30))
    .redirect_policy(isahc::redirect::Policy::follow(5))
    .build()?;

let mut response = client.get("https://example.com")?;
let body = response.text()?;
```

#### 优点
- 同步 API，使用简单
- 基于 curl，功能强大
- 支持 HTTP/2、HTTP/3
- SPDY 支持

#### 缺点
- 依赖 libcurl（系统依赖）
- 异步支持有限
- 社区较小

---

### 2.4 ureq

**仓库**: https://github.com/algesten/ureq  
**Crates.io**: https://crates.io/crates/ureq

```toml
[dependencies]
ureq = { version = "2.9", features = ["json", "charset"] }
```

```rust
// 简单同步请求
let response = ureq::get("https://example.com/video")
    .set("Accept", "video/mp4")
    .call()?;

let body = response.into_string()?;

// 或流式读取
let reader = response.into_reader();
std::io::copy(&mut reader.take(1024 * 1024), &mut std::io::stdout())?;
```

#### 优点
- 纯 Rust，无系统依赖
- 轻量级，编译快
- 同步 API，使用简单
- 适合嵌入式场景

#### 缺点
- 无异步支持
- 功能不如 reqwest 丰富
- 性能略低于 reqwest

---

### 2.5 HTTP 客户端库对比总结

| 特性 | reqwest | hyper | isahc | ureq |
|------|---------|-------|-------|------|
| **异步支持** | ✅ 完整 | ✅ 完整 | ⚠️ 有限 | ❌ |
| **同步支持** | ⚠️ 需要 tokio | ❌ | ✅ 原生 | ✅ 原生 |
| **性能** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **功能丰富度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **易用性** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **依赖复杂度** | 中等 | 低 | 高(curl) | 低 |
| **文档质量** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |

**推荐选择**: **reqwest** 作为主要 HTTP 客户端，对于需要同步调用的脚本场景可选用 **ureq**。

---

## 3. Headless 浏览器方案调研

### 3.1 headless_chrome (推荐 ⭐⭐⭐⭐)

**仓库**: https://github.com/atroche/rust-headless-chrome  
**Crates.io**: https://crates.io/crates/headless_chrome

#### 核心特性
```toml
[dependencies]
headless_chrome = "1.0"
```

```rust
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::time::Duration;

// 启动浏览器
let browser = Browser::new(
    LaunchOptionsBuilder::default()
        .headless(true)
        .window_size(Some((1920, 1080)))
        .build()
        .unwrap()
)?;

// 创建标签页
let tab = browser.new_tab()?;

// 设置视口
tab.set_default_timeout(Duration::from_secs(30));
tab.navigate_to("https://example.com/video")?;

// 等待页面加载
tab.wait_until_navigated()?;

// 等待特定元素
tab.wait_for_element("video")?;

// 获取视频源
let video_src = tab.evaluate(
    r#"document.querySelector('video').src"#
)?;

// 执行 JavaScript
let result = tab.evaluate(
    r#"
    // 获取 m3u8 URL
    const scripts = document.querySelectorAll('script');
    for (let s of scripts) {
        const match = s.textContent.match(/['"]([^'"]*\.m3u8)['"]/);
        if (match) return match[1];
    }
    return null;
    "#
)?;

// 截图
let png_data = tab.capture_screenshot(
    headless_chrome::protocol::page::ScreenshotFormat::PNG,
    Some(75),
    true
)?;

// 关闭浏览器
browser.close()?;
```

#### 高级配置
```rust
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};

let browser = Browser::new(
    LaunchOptionsBuilder::default()
        .headless(true)
        .window_size(Some((1920, 1080)))
        .args(vec![
            "--disable-gpu",
            "--no-sandbox",
            "--disable-dev-shm-usage",
            "--disable-web-security",
            "--ignore-certificate-errors",
        ])
        .path(Some("/usr/bin/google-chrome".into()))
        .port(Some(9222))
        .build()?
)?;

// 使用代理
let tab = browser.new_tab()?;
tab.enable_stealth_mode()?;
tab.authenticate("user", "password")?;
```

#### 优点
| 特性 | 描述 |
|------|------|
| **纯 Rust** | 无需 Node.js 运行时 |
| **Chrome CDP** | 直接使用 Chrome DevTools Protocol |
| **功能完整** | 支持导航、点击、输入、截图等 |
| **性能较好** | 比 Selenium 轻量 |
| **Stealth 模式** | 可隐藏自动化特征 |

#### 缺点
- 需要 Chrome/Chromium 安装
- 相比 playwright 功能较少
- 文档不如 Puppeteer 完善

---

### 3.2 fantoccini (WebDriver 协议)

**仓库**: https://github.com/jonhoo/fantoccini  
**Crates.io**: https://crates.io/crates/fantoccini

```toml
[dependencies]
fantoccini = "0.21"
tokio = { version = "1", features = ["full"] }
```

```rust
use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到 WebDriver (如 geckodriver, chromedriver)
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    // 导航到页面
    client.goto("https://example.com").await?;

    // 查找元素
    let elem = client.find(Locator::Css("video")).await?;
    let src = elem.attr("src").await?;

    // 执行 JavaScript
    let result = client.execute(
        "return document.querySelector('video').currentSrc",
        vec![]
    ).await?;

    // 关闭
    client.close().await?;
    Ok(())
}
```

#### 优点
- 支持多种浏览器（Chrome、Firefox、Safari）
- WebDriver 标准 API
- 异步原生支持

#### 缺点
- 需要单独运行 WebDriver 服务
- 性能较低
- 配置复杂

---

### 3.3 thirtyfour (Selenium WebDriver)

**仓库**: https://github.com/stevepryde/thirtyfour  
**Crates.io**: https://crates.io/crates/thirtyfour

```toml
[dependencies]
thirtyfour = { version = "0.31", features = ["reqwest"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use thirtyfour::{DesiredCapabilities, WebDriver};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Chrome 配置
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-gpu")?;

    // 连接 WebDriver
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.goto("https://example.com").await?;

    // 查找元素
    let video = driver.find(By::Css("video")).await?;
    let src = video.attr("src").await?;

    // 高级等待
    driver.wait(By::Css("video.loaded"))
        .wait_for_element()
        .await?;

    driver.quit().await?;
    Ok(())
}
```

#### 优点
- 功能最完整的 Selenium 绑定
- 支持所有主流浏览器
- 活跃维护

#### 缺点
- 需要 Selenium Grid 或 WebDriver
- 性能较低
- 依赖复杂

---

### 3.4 chromiumoxide (CDP 原生)

**仓库**: https://github.com/mattsse/chromiumoxide  
**Crates.io**: https://crates.io/crates/chromiumoxide

```toml
[dependencies]
chromiumoxide = { version = "0.7", features = ["tokio-runtime"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use chromiumoxide::{Browser, BrowserConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = BrowserConfig::builder()
        .window_size(1920, 1080)
        .headless_mode(true)
        .build()?;

    let browser = Browser::launch(config).await?;
    let page = browser.new_page("about:blank").await?;

    page.goto("https://example.com").await?;

    // 等待网络空闲
    page.wait_for_navigation().await?;

    // 执行 JavaScript
    let video_url: String = page.evaluate(
        "document.querySelector('video')?.src || ''"
    ).await?;

    browser.close().await?;
    Ok(())
}
```

#### 优点
- 最现代化的 CDP 实现
- 异步优先设计
- 活跃维护

#### 缺点
- 相对较新，可能有稳定性问题
- 文档较少

---

### 3.5 Headless 浏览器方案对比

| 特性 | headless_chrome | fantoccini | thirtyfour | chromiumoxide |
|------|-----------------|------------|------------|---------------|
| **纯 Rust** | ✅ | ✅ | ✅ | ✅ |
| **外部依赖** | Chrome | WebDriver | Selenium | Chrome |
| **异步支持** | ⚠️ 部分 | ✅ 完整 | ✅ 完整 | ✅ 完整 |
| **性能** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **易用性** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **功能完整性** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **文档质量** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **维护活跃度** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |

**推荐选择**: 
- **headless_chrome** 用于简单的动态页面处理
- **chromiumoxide** 用于需要高性能和现代化 API 的场景

---

## 4. 视频处理相关库调研

### 4.1 m3u8 解析

**推荐库**: m3u8-rs
```toml
[dependencies]
m3u8-rs = "6.0"
```

```rust
use m3u8_rs::Playlist;

fn parse_m3u8(content: &str) -> Result<Playlist, Box<dyn std::error::Error>> {
    let playlist = m3u8_rs::parse_playlist(content.as_bytes())?;
    
    match playlist {
        Playlist::MasterPlaylist(master) => {
            // 主播放列表 - 包含多个分辨率选项
            for variant in master.variants {
                println!("Resolution: {:?}, URL: {}", 
                    variant.resolution, variant.uri);
            }
        }
        Playlist::MediaPlaylist(media) => {
            // 媒体播放列表 - 包含分片信息
            for segment in media.segments {
                println!("Segment: {}, Duration: {:?}", 
                    segment.uri, segment.duration);
            }
        }
    }
    
    Ok(playlist)
}
```

### 4.2 URL 处理

```toml
[dependencies]
url = "2.5"
```

```rust
use url::{Url, ParseError};

fn normalize_url(base: &str, relative: &str) -> Result<String, ParseError> {
    let base_url = Url::parse(base)?;
    let resolved = base_url.join(relative)?;
    
    // 规范化：移除片段、排序查询参数等
    let mut normalized = resolved.clone();
    normalized.set_fragment(None);
    
    // 移除默认端口
    if let Some(port) = normalized.port() {
        match normalized.scheme() {
            "http" if port == 80 => normalized.set_port(None).unwrap(),
            "https" if port == 443 => normalized.set_port(None).unwrap(),
            _ => {}
        }
    }
    
    Ok(normalized.to_string())
}

// 示例
let base = "https://example.com/videos/";
let relative = "./video.m3u8";
assert_eq!(
    normalize_url(base, relative)?,
    "https://example.com/videos/video.m3u8"
);
```

### 4.3 并发控制

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
async-channel = "2"
futures = "0.3"
```

```rust
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};

pub struct ConcurrencyController {
    semaphore: Arc<Semaphore>,
    rate_limiter: RateLimiter,
}

impl ConcurrencyController {
    pub fn new(max_concurrent: usize, requests_per_second: u32) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            rate_limiter: RateLimiter::new(requests_per_second),
        }
    }

    pub async fn acquire(&self) -> SemaphorePermit<'_> {
        self.rate_limiter.wait().await;
        self.semaphore.acquire().await.unwrap()
    }
}

pub struct RateLimiter {
    interval: tokio::time::Interval,
}

impl RateLimiter {
    pub fn new(requests_per_second: u32) -> Self {
        let interval = tokio::time::interval(
            std::time::Duration::from_millis(1000 / requests_per_second as u64)
        );
        Self { interval }
    }

    pub async fn wait(&mut self) {
        self.interval.tick().await;
    }
}
```

---

## 5. 综合选型建议

### 5.1 推荐技术栈

```
┌─────────────────────────────────────────────────────────────────┐
│                    TurboDownload 技术栈                          │
├─────────────────────────────────────────────────────────────────┤
│  HTML 解析      │  scraper (主) + html5ever (容错备用)          │
│  HTTP 客户端    │  reqwest (主) + ureq (简单脚本)               │
│  Headless      │  headless_chrome (主) + chromiumoxide (高级)   │
│  URL 处理       │  url crate                                   │
│  异步运行时     │  tokio                                        │
│  并发控制      │  tokio::sync::Semaphore + 自定义限流器         │
│  序列化        │  serde + serde_json                           │
│  错误处理      │  thiserror + anyhow                           │
│  日志          │  tracing + tracing-subscriber                  │
│  测试          │  tokio-test + wiremock                         │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Cargo.toml 核心依赖

```toml
[package]
name = "turbo-download"
version = "0.1.0"
edition = "2021"

[dependencies]
# 异步运行时
tokio = { version = "1", features = ["full"] }

# HTTP 客户端
reqwest = { version = "0.12", features = ["json", "cookies", "gzip", "brotli", "stream", "rustls-tls"] }

# HTML 解析
scraper = "0.22"
html5ever = "0.27"

# URL 处理
url = "2.5"

# M3U8 解析
m3u8-rs = "6.0"

# Headless 浏览器
headless_chrome = "1.0"

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# 错误处理
thiserror = "2"
anyhow = "1"

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 异步工具
futures = "0.3"
async-channel = "2"

# 正则表达式
regex = "1"

# 哈希（去重）
xxhash-rust = { version = "0.8", features = ["xxh3"] }

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# Robots.txt 解析
robotstxt = "0.3"

[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.6"
tempfile = "3"
```

### 5.3 项目结构建议

```
turbo-download/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # 库入口
│   ├── main.rs                   # CLI 入口
│   │
│   ├── fetcher/                  # 抓取核心模块
│   │   ├── mod.rs
│   │   ├── http.rs              # HTTP 客户端封装
│   │   ├── browser.rs           # Headless 浏览器封装
│   │   └── pool.rs              # 客户端池
│   │
│   ├── parser/                   # 解析模块
│   │   ├── mod.rs
│   │   ├── html.rs              # HTML 解析
│   │   ├── m3u8.rs              # M3U8 解析
│   │   ├── video.rs             # 视频源提取
│   │   └── url.rs               # URL 规范化
│   │
│   ├── crawler/                  # 爬虫模块
│   │   ├── mod.rs
│   │   ├── scheduler.rs         # 调度器
│   │   ├── queue.rs             # URL 队列
│   │   ├── robots.rs            # robots.txt 处理
│   │   └── dedup.rs             # 去重
│   │
│   ├── anti_detect/              # 反检测模块
│   │   ├── mod.rs
│   │   ├── user_agent.rs        # UA 轮换
│   │   ├── proxy.rs             # 代理池
│   │   └── throttle.rs          # 频率控制
│   │
│   ├── downloader/               # 下载模块
│   │   ├── mod.rs
│   │   ├── segment.rs           # 分片下载
│   │   ├── merge.rs             # 合并
│   │   └── progress.rs          # 进度条
│   │
│   └── error.rs                  # 错误定义
│
├── tests/                        # 集成测试
│   ├── integration/
│   └── fixtures/
│
└── docs/                         # 文档
    ├── architecture.md
    ├── api.md
    └── examples.md
```

---

## 附录：参考链接

- scraper 文档: https://docs.rs/scraper
- reqwest 文档: https://docs.rs/reqwest
- headless_chrome 文档: https://docs.rs/headless_chrome
- tokio 文档: https://docs.rs/tokio
- Rust HTTP Working Group: https://github.com/hyperium