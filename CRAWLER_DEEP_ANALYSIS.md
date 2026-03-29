# TurboDownload 资源抓取功能 - 深度技术分析

> 生成时间: 2025-03-25
> 状态: 已验证

---

## 1. 原始代码审查

### 1.1 视频抓取函数

```rust
// 原始代码
pub fn videos(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse("video[src], source[src]").unwrap();
    doc.select(&sel)
        .filter_map(|e| e.value().attr("src"))
        .map(|s| s.to_string())
        .collect()
}
```

**问题清单：**

| 问题 | 严重性 | 说明 |
|------|--------|------|
| 相对URL未处理 | 🔴 高 | `/video.mp4` 无法直接使用 |
| 缺少上下文 | 🔴 高 | 无法获取 base URL |
| data URI 未过滤 | 🟠 中 | `data:video/mp4;base64,...` 被误提取 |
| 动态加载视频 | 🔴 高 | JS 插入的 video 标签无法获取 |
| blob URL | 🟠 中 | `blob:` URL 无法直接下载 |
| lazy loading | 🟠 中 | `data-src` 属性被忽略 |
| .unwrap() panic | 🔴 高 | 选择器解析失败会导致程序崩溃 |
| 无去重 | 🟡 低 | 同一 URL 可能重复出现 |

### 1.2 m3u8 提取函数

```rust
// 原始代码
pub fn m3u8(html: &str) -> Vec<String> {
    let re = Regex::new(r"https?://[^\s\"']+\.m3u8").unwrap();
    re.find_iter(html).map(|m| m.as_str().to_string()).collect()
}
```

**问题清单：**

| 问题 | 严重性 | 说明 |
|------|--------|------|
| 正则过于简单 | 🔴 高 | 无法匹配 `?token=xxx` 查询参数 |
| 误匹配风险 | 🟠 中 | 可能匹配注释/脚本中的非真实 URL |
| URL 编码问题 | 🟠 中 | 未处理 URL 编码 |
| 无去重 | 🟡 低 | 同一 URL 可能出现多次 |
| 相对路径 | 🔴 高 | 无法处理相对路径 m3u8 |

---

## 2. 技术选型验证

### 2.1 组件评估

| 组件 | 选择 | 评分 | 评估 |
|------|------|------|------|
| HTML解析 | `scraper` | ⭐⭐⭐⭐ | ✅ 合适，基于 CSS 选择器，性能好，内存安全 |
| HTTP客户端 | `reqwest` | ⭐⭐⭐⭐⭐ | ✅ 最佳选择，异步支持，功能完整 |
| Headless | `headless_chrome` | ⭐⭐⭐ | ⚠️ 有更好替代 |

### 2.2 headless_chrome 问题

- 维护不够活跃，issue 响应慢
- 缺少现代 CDP (Chrome DevTools Protocol) 特性支持
- 不支持 WebSocket 通信（某些网站需要）

**推荐替代：**

```toml
[dependencies]
# 推荐: chromiumoxide - 更现代的 CDP 实现
chromiumoxide = { version = "0.5", features = ["tokio-runtime"] }

# 或: fantoccini - 通过 WebDriver，支持多种浏览器
fantoccini = "0.19"
```

---

## 3. 优化后代码

### 3.1 完整资源提取器

```rust
use scraper::{Html, Selector};
use regex::Regex;
use url::Url;
use std::collections::HashSet;

/// 资源提取器配置
#[derive(Debug, Clone)]
pub struct ExtractorConfig {
    pub base_url: Option<Url>,
    pub filter_data_uri: bool,
    pub filter_blob: bool,
}

impl Default for ExtractorConfig {
    fn default() -> Self {
        Self {
            base_url: None,
            filter_data_uri: true,
            filter_blob: true,
        }
    }
}

/// 提取结果
#[derive(Debug, Clone)]
pub struct ExtractedUrl {
    pub url: String,
    pub source: UrlSource,
    pub is_relative: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum UrlSource {
    VideoTag,
    SourceTag,
    M3u8Pattern,
    DataSrc,
}

/// 优化的视频提取
pub fn videos_optimized(
    html: &str,
    config: &ExtractorConfig,
) -> Result<Vec<ExtractedUrl>, Box<dyn std::error::Error>> {
    let doc = Html::parse_document(html);
    let mut results = Vec::new();
    let mut seen = HashSet::new();
    
    // 主选择器 - 包含 lazy loading 支持
    let video_sel = Selector::parse("video[src], video[data-src]")?;
    let source_sel = Selector::parse("source[src], source[data-src]")?;
    
    // 处理 video 标签
    for elem in doc.select(&video_sel) {
        if let Some(url) = extract_url_from_elem(&elem, config, UrlSource::VideoTag) {
            if seen.insert(url.url.clone()) {
                results.push(url);
            }
        }
    }
    
    // 处理 source 标签
    for elem in doc.select(&source_sel) {
        if let Some(url) = extract_url_from_elem(&elem, config, UrlSource::SourceTag) {
            if seen.insert(url.url.clone()) {
                results.push(url);
            }
        }
    }
    
    Ok(results)
}

fn extract_url_from_elem(
    elem: &scraper::ElementRef,
    config: &ExtractorConfig,
    default_source: UrlSource,
) -> Option<ExtractedUrl> {
    let value = elem.value();
    
    // 尝试 src 和 data-src
    let (raw_url, source) = value
        .attr("src")
        .map(|s| (s, default_source))
        .or_else(|| value.attr("data-src").map(|s| (s, UrlSource::DataSrc)))?;
    
    // 过滤 data URI
    if config.filter_data_uri && raw_url.starts_with("data:") {
        return None;
    }
    
    // 过滤 blob URL
    if config.filter_blob && raw_url.starts_with("blob:") {
        return None;
    }
    
    // 解析相对路径
    let is_relative = !raw_url.starts_with("http") && !raw_url.starts_with("//");
    let absolute_url = if is_relative {
        if let Some(base) = &config.base_url {
            base.join(raw_url).ok()?.to_string()
        } else {
            return None;
        }
    } else if raw_url.starts_with("//") {
        format!("https:{}", raw_url)
    } else {
        raw_url.to_string()
    };
    
    Some(ExtractedUrl {
        url: absolute_url,
        source,
        is_relative,
    })
}

/// 优化的 m3u8 提取
pub fn m3u8_optimized(
    html: &str,
    config: &ExtractorConfig,
) -> Result<Vec<ExtractedUrl>, Box<dyn std::error::Error>> {
    // 更健壮的正则，支持查询参数
    let re = Regex::new(
        r#"https?://[^\s"'<>]+?\.m3u8(?:\?[^\s"'<>]*)?"#
    )?;
    
    let mut results = Vec::new();
    let mut seen = HashSet::new();
    
    for cap in re.captures_iter(html) {
        let url = cap[0].to_string();
        
        // URL 解码
        let decoded = urlencoding::decode(&url)?.into_owned();
        
        if seen.insert(decoded.clone()) {
            results.push(ExtractedUrl {
                url: decoded,
                source: UrlSource::M3u8Pattern,
                is_relative: false,
            });
        }
    }
    
    Ok(results)
}

/// 合并所有视频资源提取
pub fn extract_all_video_resources(
    html: &str,
    base_url: Option<&str>,
) -> Result<Vec<ExtractedUrl>, Box<dyn std::error::Error>> {
    let config = ExtractorConfig {
        base_url: base_url.and_then(|u| Url::parse(u).ok()),
        ..Default::default()
    };
    
    let mut videos = videos_optimized(html, &config)?;
    let m3u8s = m3u8_optimized(html, &config)?;
    
    videos.extend(m3u8s);
    Ok(videos)
}
```

### 3.2 HTTP 客户端配置

```rust
use reqwest::{Client, ClientBuilder, redirect::Policy};
use std::time::Duration;

pub fn create_http_client() -> Result<Client, reqwest::Error> {
    ClientBuilder::new()
        // 用户代理
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        
        // 超时设置
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        
        // 连接池配置
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(60))
        
        // TCP 优化
        .tcp_keepalive(Duration::from_secs(30))
        .tcp_nodelay(true)
        
        // 重定向策略
        .redirect(Policy::limited(5))
        
        // Cookies 支持
        .cookie_store(true)
        
        // HTTPS 配置
        .danger_accept_invalid_certs(false)
        
        .build()
}

/// 带重试的请求
pub async fn fetch_with_retry(
    client: &Client,
    url: &str,
    max_retries: u32,
) -> Result<String, reqwest::Error> {
    let mut last_error = None;
    
    for attempt in 0..max_retries {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return response.text().await;
                }
                last_error = Some(format!("HTTP {}", response.status()));
            }
            Err(e) => {
                last_error = Some(e.to_string());
                if attempt < max_retries - 1 {
                    tokio::time::sleep(Duration::from_millis(100 * (1 << attempt))).await;
                }
            }
        }
    }
    
    Err(reqwest::Error::new(
        reqwest::error::Kind::Request,
        last_error.unwrap_or_else(|| "Unknown error".to_string()),
    ))
}
```

### 3.3 Headless 浏览器配置

```rust
use chromiumoxide::{Browser, BrowserConfig};
use std::time::Duration;

pub async fn create_browser() -> Result<Browser, Box<dyn std::error::Error>> {
    let config = BrowserConfig::builder()
        // 窗口大小
        .window_size(1920, 1080)
        
        // 反检测
        .disable_blink_features(&["AutomationControlled"])
        
        // 稳定性参数
        .args(&[
            "--disable-blink-features=AutomationControlled",
            "--disable-dev-shm-usage",
            "--no-sandbox",
            "--disable-gpu",
            "--disable-software-rasterizer",
        ])
        
        // 超时
        .request_timeout(Duration::from_secs(30))
        
        .build()?;
    
    Browser::launch(config).await
}

/// 等待动态内容加载
pub async fn fetch_dynamic_content(
    browser: &Browser,
    url: &str,
    wait_selector: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let page = browser.new_page("about:blank").await?;
    
    // 导航到目标页面
    page.goto(url).await?.wait_for_navigation().await?;
    
    // 等待特定元素
    if let Some(selector) = wait_selector {
        page.wait_for_element(selector).await?;
    } else {
        // 默认等待网络空闲
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    // 获取渲染后的 HTML
    let html = page.content().await?;
    
    page.close().await?;
    
    Ok(html)
}
```

---

## 4. 潜在问题与风险

### 4.1 严重问题

#### 反爬检测

| 问题 | 影响 | 解决方案 |
|------|------|----------|
| 无 User-Agent 轮换 | 被识别为爬虫 | UA 池轮换 |
| 无请求速率限制 | IP 被封 | 随机延时 + 令牌桶 |
| 无代理支持 | 单点故障 | 代理池 |
| 无 Cookie 管理 | 登录态丢失 | Cookie 持久化 |
| JS 指纹暴露 | 浏览器指纹检测 | stealth 插件 |

#### 内存安全

```rust
// ❌ 危险: 大文件直接加载
let html = response.text().await?;

// ✅ 安全: 限制大小
let html = response
    .text()
    .await
    .map_err(|_| "Response too large")?;
if html.len() > 10 * 1024 * 1024 { // 10MB 限制
    return Err("HTML exceeds size limit".into());
}
```

#### 错误处理

```rust
// ❌ 危险: unwrap 会 panic
let sel = Selector::parse("...").unwrap();

// ✅ 安全: 正确传播错误
let sel = Selector::parse("...")
    .map_err(|e| format!("Invalid selector: {}", e))?;
```

### 4.2 中等问题

| 问题 | 影响 | 解决方案 |
|------|------|----------|
| 无并发控制 | 目标站点过载 | Semaphore 限制 |
| 无连接池复用 | 性能低下 | 配置连接池 |
| 无超时控制 | 请求挂起 | 设置超时 |
| 无日志记录 | 难以调试 | tracing 日志 |

---

## 5. 遗漏点清单

### 5.1 功能遗漏

| 功能 | 优先级 | 说明 |
|------|--------|------|
| 资源去重 | 🔴 高 | URL 哈希去重 |
| 反爬策略 | 🔴 高 | UA轮换、延时、代理池 |
| 错误重试 | 🔴 高 | 指数退避重试 |
| URL 规范化 | 🟠 中 | 统一 URL 格式 |
| 并发控制 | 🟠 中 | Semaphore 限流 |
| M3U8 解析 | 🟠 中 | 提取 ts 片段列表 |
| Cookie 管理 | 🟠 中 | 持久化存储 |
| 请求签名 | 🟡 低 | 某些网站需要 |

### 5.2 安全遗漏

| 检查项 | 状态 | 说明 |
|--------|------|------|
| robots.txt | ❌ | 未检查爬取权限 |
| 速率限制 | ❌ | 无请求频率控制 |
| 输入验证 | ❌ | URL 未验证 |
| 敏感信息 | ⚠️ | 日志可能泄露 |

---

## 6. 风险评估矩阵

| 风险项 | 影响 | 概率 | 缓解措施 |
|--------|------|------|----------|
| 反爬被封 | 🔴 高 | 🔴 高 | 代理池 + 频率控制 + UA轮换 |
| 内存溢出 | 🔴 高 | 🟠 中 | 流式处理 + 大小限制 |
| 正则误匹配 | 🟠 中 | 🟠 中 | 改进正则 + 后验证 |
| 动态渲染失败 | 🔴 高 | 🟠 中 | 回退机制 + 多浏览器支持 |
| 法律风险 | 🔴 高 | 🟢 低 | robots.txt 检查 + 授权 |

---

## 7. 工作量重新评估

### 7.1 MVP 阶段

| 任务 | 工时 | 说明 |
|------|------|------|
| 基础提取器 | 2天 | videos + m3u8 |
| URL 处理 | 1天 | 相对路径 + 去重 + 规范化 |
| HTTP 客户端 | 1天 | 配置 + 重试 + 超时 |
| 错误处理 | 1天 | 统一错误类型 + 日志 |
| 单元测试 | 1天 | 核心功能测试 |
| **小计** | **6天** | |

### 7.2 完整版阶段

| 任务 | 工时 | 说明 |
|------|------|------|
| Headless 集成 | 2天 | chromiumoxide 配置 |
| 反爬策略 | 2天 | UA轮换 + 代理 + 延时 |
| 并发控制 | 2天 | Semaphore + 连接池 |
| M3U8 解析 | 2天 | ts 片段提取 + 合并 |
| 日志监控 | 1天 | tracing + metrics |
| 集成测试 | 2天 | E2E 测试 |
| 文档 | 1天 | API 文档 + 使用指南 |
| **小计** | **12天** | |

### 7.3 总结

| 阶段 | 原评估 | 建议评估 | 差异 |
|------|--------|----------|------|
| MVP | 5-7天 | **6-8天** | +1天 |
| 完整版 | 10-14天 | **18-20天** | +8天 |

**差异原因：**
- 错误处理和重试机制被低估
- 反爬策略复杂度高
- 测试和文档未纳入原评估

---

## 8. 推荐依赖

```toml
[dependencies]
# HTML 解析
scraper = "0.18"

# HTTP 客户端
reqwest = { version = "0.11", features = ["json", "cookies", "gzip", "brotli"] }

# Headless 浏览器 (推荐替代 headless_chrome)
chromiumoxide = { version = "0.5", features = ["tokio-runtime"] }

# 异步运行时
tokio = { version = "1", features = ["full"] }

# URL 处理
url = "2.4"

# 正则表达式
regex = "1.10"

# URL 编码
urlencoding = "2.1"

# 错误处理
thiserror = "1.0"
anyhow = "1.0"

# 日志
tracing = "0.1"
tracing-subscriber = "0.3"

# 并发控制
tokio-stream = "0.1"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 9. 下一步行动

### 立即行动

1. ✅ 替换 `headless_chrome` 为 `chromiumoxide`
2. ✅ 实现 URL 规范化和去重
3. ✅ 添加错误处理和重试机制
4. ✅ 配置 HTTP 客户端连接池

### 短期目标 (1周内)

1. 实现反爬策略框架
2. 添加并发控制
3. 编写单元测试

### 中期目标 (2周内)

1. Headless 浏览器集成
2. M3U8 解析器
3. 集成测试

---

*报告结束*