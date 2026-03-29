# P2: turbo-crawler 开发检查清单

## 任务检查清单

本文档为每个开发任务提供详细的输入、处理步骤、输出和验证方法。

---

## T2.1: 项目初始化

### T2.1.1: 创建 Rust crate 结构

| 项目 | 内容 |
|------|------|
| **输入** | 无 |
| **处理步骤** | 1. 创建 `crates/turbo-crawler` 目录<br>2. 运行 `cargo init --lib`<br>3. 创建子模块目录结构<br>4. 创建各模块入口文件 |
| **输出** | 完整的项目目录结构 |
| **验证方法** | `cargo check` 无错误<br>目录结构符合规范 |

**检查项**:
- [ ] `Cargo.toml` 存在且格式正确
- [ ] `src/lib.rs` 存在
- [ ] 所有子模块目录已创建
- [ ] 每个 `mod.rs` 文件已创建

---

### T2.1.2: 配置 Cargo.toml 依赖

| 项目 | 内容 |
|------|------|
| **输入** | `Cargo.toml` 模板 |
| **处理步骤** | 1. 编辑 `[package]` 部分<br>2. 添加 `[dependencies]`<br>3. 添加 `[dev-dependencies]`<br>4. 运行 `cargo fetch` |
| **输出** | 正确配置的 `Cargo.toml` |
| **验证方法** | `cargo fetch` 成功<br>`cargo build` 成功 |

**检查项**:
- [ ] 所有依赖版本已指定
- [ ] workspace 依赖正确引用
- [ ] dev-dependencies 完整
- [ ] 无版本冲突

---

### T2.1.3: 创建测试目录结构

| 项目 | 内容 |
|------|------|
| **输入** | 测试框架设计 |
| **处理步骤** | 1. 创建 `tests/` 目录<br>2. 创建测试模块文件<br>3. 创建测试工具函数 |
| **输出** | 完整的测试目录结构 |
| **验证方法** | `cargo test` 可运行 |

**检查项**:
- [ ] `tests/mod.rs` 存在
- [ ] 各测试模块文件存在
- [ ] `tests/common/mod.rs` 工具模块存在

---

### T2.1.4: 配置开发工具

| 项目 | 内容 |
|------|------|
| **输入** | 工具配置模板 |
| **处理步骤** | 1. 创建 `rustfmt.toml`<br>2. 创建 `.cargo/config.toml`<br>3. 创建 `.vscode/` 配置 |
| **输出** | 完整的开发工具配置 |
| **验证方法** | `cargo fmt -- --check` 通过 |

**检查项**:
- [ ] `rustfmt.toml` 配置正确
- [ ] `.cargo/config.toml` 配置正确
- [ ] VS Code 扩展推荐已配置

---

## T2.2: HTTP 抓取核心

### T2.2.1: 定义 FetchClient 结构体

| 项目 | 内容 |
|------|------|
| **输入** | reqwest 库<br>HTTP 客户端设计 |
| **处理步骤** | 1. 创建 `FetchClientConfig` 结构体<br>2. 创建 `FetchClient` 结构体<br>3. 实现 builder 模式 |
| **输出** | `src/fetch/client.rs` |
| **验证方法** | 编译通过<br>配置构建测试通过 |

**检查项**:
- [ ] 配置选项完整
- [ ] builder 模式正确实现
- [ ] 默认值合理

**测试用例**:
```rust
#[test]
fn test_client_builder() {
    let client = FetchClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .user_agent("TestBot/1.0")
        .build()
        .unwrap();
    
    assert_eq!(client.config().timeout, Duration::from_secs(30));
    assert_eq!(client.config().user_agent, "TestBot/1.0");
}
```

---

### T2.2.2: 实现 GET 请求与响应处理

| 项目 | 内容 |
|------|------|
| **输入** | HTTP 客户端<br>响应设计 |
| **处理步骤** | 1. 定义 `FetchResponse` 结构体<br>2. 实现 GET 方法<br>3. 实现 HEAD 方法<br>4. 实现响应解析 |
| **输出** | `src/fetch/response.rs` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] GET 请求正常
- [ ] HEAD 请求正常
- [ ] 重定向处理正确
- [ ] 响应解析正确

**测试用例**:
```rust
#[tokio::test]
async fn test_get_request() {
    let client = FetchClient::new(Default::default()).unwrap();
    let url: Url = "https://httpbin.org/get".parse().unwrap();
    let response = client.get(&url).await.unwrap();
    
    assert_eq!(response.status, 200);
    assert!(response.body.len() > 0);
}

#[tokio::test]
async fn test_redirect_follow() {
    let client = FetchClient::new(FetchClientConfig {
        follow_redirects: true,
        ..Default::default()
    }).unwrap();
    
    let url: Url = "https://httpbin.org/redirect/2".parse().unwrap();
    let response = client.get(&url).await.unwrap();
    
    assert_ne!(response.final_url, url);
}
```

---

### T2.2.3: 实现并发请求控制

| 项目 | 内容 |
|------|------|
| **输入** | 并发控制设计 |
| **处理步骤** | 1. 创建 `ConcurrencyControl` 结构体<br>2. 使用 Semaphore 控制并发<br>3. 实现 `get_many` 批量方法 |
| **输出** | `src/fetch/concurrency.rs` |
| **验证方法** | 并发测试通过 |

**检查项**:
- [ ] 并发数限制有效
- [ ] 批量请求正常
- [ ] 无资源泄漏

**测试用例**:
```rust
#[tokio::test]
async fn test_concurrent_requests() {
    let client = FetchClient::new(Default::default()).unwrap();
    let urls: Vec<Url> = vec![
        "https://httpbin.org/delay/1".parse().unwrap(),
        "https://httpbin.org/delay/1".parse().unwrap(),
        "https://httpbin.org/delay/1".parse().unwrap(),
    ];
    
    let start = std::time::Instant::now();
    let results = client.get_many(&urls, 2).await;
    let elapsed = start.elapsed();
    
    // 3 个请求，并发 2，应该需要约 2 秒
    assert!(elapsed.as_secs() >= 2);
    assert!(results.len() >= 3);
}
```

---

## T2.3: HTML 解析器

### T2.3.1: 实现 HTML 解析核心

| 项目 | 内容 |
|------|------|
| **输入** | scraper 库<br>HTML 解析设计 |
| **处理步骤** | 1. 创建 `HtmlParser` 结构体<br>2. 实现 CSS 选择器<br>3. 实现 `Element` 结构体<br>4. 实现 URL 解析 |
| **输出** | `src/parse/html.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] CSS 选择器工作正常
- [ ] 元素属性提取正确
- [ ] URL 解析正确

**测试用例**:
```rust
#[test]
fn test_html_parse() {
    let html = r#"
        <html>
            <head><title>Test</title></head>
            <body>
                <a href="/link1">Link 1</a>
                <img src="/image.png" alt="Image">
            </body>
        </html>
    "#;
    
    let base_url: Url = "https://example.com".parse().unwrap();
    let parser = HtmlParser::from_html(html, Some(base_url));
    
    assert_eq!(parser.title(), Some("Test".to_string()));
    
    let links = parser.select("a[href]").unwrap();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].abs_url("href").unwrap().as_str(), "https://example.com/link1");
}
```

---

### T2.3.2: 实现 CSS 解析

| 项目 | 内容 |
|------|------|
| **输入** | CSS 内容<br>URL 提取规则 |
| **处理步骤** | 1. 创建 `CssParser` 结构体<br>2. 实现 url() 解析<br>3. 实现 @import 解析<br>4. 实现 @font-face 解析 |
| **输出** | `src/parse/css.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] url() 解析正确
- [ ] @import 解析正确
- [ ] 相对 URL 转换正确

**测试用例**:
```rust
#[test]
fn test_css_url_extraction() {
    let css = r#"
        .background {
            background-image: url('images/bg.png');
        }
        @import url('styles/main.css');
    "#;
    
    let base_url: Url = "https://example.com/css/style.css".parse().unwrap();
    let parser = CssParser::new(Some(base_url));
    
    let urls = parser.extract_urls(css);
    assert!(urls.iter().any(|u| u.url.as_str().ends_with("images/bg.png")));
    
    let imports = parser.parse_imports(css);
    assert!(imports.iter().any(|u| u.as_str().ends_with("styles/main.css")));
}
```

---

### T2.3.3: 实现 JS 资源解析

| 项目 | 内容 |
|------|------|
| **输入** | JS 代码<br>资源模式 |
| **处理步骤** | 1. 创建 `JsParser` 结构体<br>2. 实现 fetch 识别<br>3. 实现动态 import 识别<br>4. 实现 URL 模式匹配 |
| **输出** | `src/parse/js.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] fetch 识别正确
- [ ] import 识别正确
- [ ] 过滤非 URL 字符串

**测试用例**:
```rust
#[test]
fn test_js_resource_extraction() {
    let js = r#"
        fetch('/api/data.json');
        import('./module.js');
        const url = "https://example.com/resource";
    "#;
    
    let parser = JsParser::new(None);
    let resources = parser.extract_resource_urls(js);
    
    assert!(resources.iter().any(|r| r.url.path() == "/api/data.json"));
    assert!(resources.iter().any(|r| r.url.path() == "./module.js"));
}
```

---

## T2.4: 资源提取器

### T2.4.1: 实现链接提取器

| 项目 | 内容 |
|------|------|
| **输入** | HTML 解析器<br>链接规则 |
| **处理步骤** | 1. 创建 `LinkExtractor` 结构体<br>2. 实现 <a> 提取<br>3. 实现 <link> 提取<br>4. 实现相对 URL 转换 |
| **输出** | `src/extract/links.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] <a> 链接提取正确
- [ ] <link> 链接提取正确
- [ ] 相对 URL 转换正确

**测试用例**:
```rust
#[test]
fn test_link_extraction() {
    let html = r#"
        <a href="/page1">Page 1</a>
        <a href="https://other.com/page2">External</a>
        <link rel="stylesheet" href="/style.css">
    "#;
    
    let base_url: Url = "https://example.com".parse().unwrap();
    let parser = HtmlParser::from_html(html, Some(base_url));
    let extractor = LinkExtractor::new(parser);
    
    let links = extractor.extract_all();
    assert_eq!(links.len(), 3);
    
    let anchor_links: Vec<_> = links.iter()
        .filter(|l| l.link_type == LinkType::Anchor)
        .collect();
    assert_eq!(anchor_links.len(), 2);
}
```

---

### T2.4.2: 实现图片提取器

| 项目 | 内容 |
|------|------|
| **输入** | HTML 解析器<br>CSS 解析器 |
| **处理步骤** | 1. 创建 `ImageExtractor` 结构体<br>2. 实现 <img> 提取<br>3. 实现 srcset 解析<br>4. 实现 CSS 图片提取 |
| **输出** | `src/extract/images.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] <img> 提取正确
- [ ] srcset 解析正确
- [ ] CSS 图片提取正确

**测试用例**:
```rust
#[test]
fn test_image_extraction() {
    let html = r#"
        <img src="/img1.png" alt="Image 1">
        <img srcset="/img2-small.png 400w, /img2-large.png 800w">
        <picture>
            <source srcset="/img3.webp" type="image/webp">
        </picture>
    "#;
    
    let base_url: Url = "https://example.com".parse().unwrap();
    let parser = HtmlParser::from_html(html, Some(base_url));
    let extractor = ImageExtractor::new(parser, None);
    
    let images = extractor.extract_all();
    assert!(images.len() >= 3);
    
    let img_tags: Vec<_> = images.iter()
        .filter(|i| i.source == ImageSource::ImgTag)
        .collect();
    assert!(!img_tags.is_empty());
}
```

---

### T2.4.3: 实现媒体资源提取器

| 项目 | 内容 |
|------|------|
| **输入** | HTML 解析器 |
| **处理步骤** | 1. 创建 `MediaExtractor` 结构体<br>2. 实现 video 提取<br>3. 实现 audio 提取<br>4. 实现 iframe 识别 |
| **输出** | `src/extract/media.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] video 标签提取正确
- [ ] audio 标签提取正确
- [ ] source 标签处理正确

**测试用例**:
```rust
#[test]
fn test_video_extraction() {
    let html = r#"
        <video src="/video.mp4" poster="/poster.jpg">
            <source src="/video.webm" type="video/webm">
        </video>
    "#;
    
    let base_url: Url = "https://example.com".parse().unwrap();
    let parser = HtmlParser::from_html(html, Some(base_url));
    let extractor = MediaExtractor::new(parser);
    
    let videos = extractor.extract_videos();
    assert!(videos.len() >= 1);
}
```

---

### T2.4.4: 实现字体提取器

| 项目 | 内容 |
|------|------|
| **输入** | HTML 解析器<br>CSS 解析器 |
| **处理步骤** | 1. 创建 `FontExtractor` 结构体<br>2. 实现 @font-face 解析<br>3. 实现字体格式识别 |
| **输出** | `src/extract/fonts.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] @font-face 解析正确
- [ ] 字体格式识别正确

---

## T2.5: 过滤与限速

### T2.5.1: 实现 URL 过滤器

| 项目 | 内容 |
|------|------|
| **输入** | URL 规则 |
| **处理步骤** | 1. 创建 `UrlFilter` 结构体<br>2. 实现域名过滤<br>3. 实现正则过滤<br>4. 实现协议过滤 |
| **输出** | `src/filter/url_filter.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] 域名过滤正确
- [ ] 正则匹配正确
- [ ] 协议过滤正确

**测试用例**:
```rust
#[test]
fn test_url_filter() {
    let filter = UrlFilterBuilder::new()
        .allow_domains(vec!["example.com".to_string()])
        .block_patterns(vec![r"\.pdf$"])
        .build();
    
    let allowed: Url = "https://example.com/page".parse().unwrap();
    let blocked_domain: Url = "https://other.com/page".parse().unwrap();
    let blocked_pattern: Url = "https://example.com/doc.pdf".parse().unwrap();
    
    assert_eq!(filter.should_allow(&allowed), FilterResult::Allowed);
    assert!(matches!(filter.should_allow(&blocked_domain), FilterResult::Denied(_)));
    assert!(matches!(filter.should_allow(&blocked_pattern), FilterResult::Denied(_)));
}
```

---

### T2.5.2: 实现 MIME 类型过滤器

| 项目 | 内容 |
|------|------|
| **输入** | MIME 类型规则 |
| **处理步骤** | 1. 创建 `MimeFilter` 结构体<br>2. 实现 MIME 类型识别<br>3. 实现扩展名识别<br>4. 实现过滤逻辑 |
| **输出** | `src/filter/mime_filter.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] MIME 类型识别正确
- [ ] 扩展名识别正确
- [ ] 过滤逻辑正确

**测试用例**:
```rust
#[test]
fn test_mime_filter() {
    let filter = MimeFilter {
        allowed_types: vec![MimeType::Html, MimeType::Css, MimeType::Image],
        blocked_types: vec![],
    };
    
    assert_eq!(filter.should_allow(Some("text/html")), FilterResult::Allowed);
    assert_eq!(filter.should_allow(Some("image/png")), FilterResult::Allowed);
    assert!(matches!(filter.should_allow(Some("application/pdf")), FilterResult::Denied(_)));
}
```

---

### T2.5.3: 实现请求限速

| 项目 | 内容 |
|------|------|
| **输入** | 限速配置 |
| **处理步骤** | 1. 创建 `TokenBucket` 结构体<br>2. 实现令牌获取<br>3. 创建 `RequestThrottle` 结构体<br>4. 实现域名级限速 |
| **输出** | `src/rate_limit/throttle.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] 令牌桶工作正常
- [ ] 域名级限速正常
- [ ] 最小间隔控制正确

**测试用例**:
```rust
#[tokio::test]
async fn test_token_bucket() {
    let bucket = TokenBucket::new(2, 1); // 容量 2，每秒补充 1
    
    assert!(bucket.try_acquire());
    assert!(bucket.try_acquire());
    assert!(!bucket.try_acquire()); // 桶空
    
    tokio::time::sleep(Duration::from_millis(1100)).await;
    assert!(bucket.try_acquire()); // 补充了一个令牌
}

#[tokio::test]
async fn test_request_throttle() {
    let throttle = RequestThrottle::new(10, 2, Duration::from_millis(100));
    let url: Url = "https://example.com/page".parse().unwrap();
    
    let start = std::time::Instant::now();
    throttle.wait_for_permission(&url).await;
    throttle.wait_for_permission(&url).await;
    let elapsed = start.elapsed();
    
    // 至少应该等待 min_interval
    assert!(elapsed >= Duration::from_millis(100));
}
```

---

## T2.6: Cookie 管理

### T2.6.1: 实现 Cookie 存储

| 项目 | 内容 |
|------|------|
| **输入** | Cookie 规范 |
| **处理步骤** | 1. 创建 `Cookie` 结构体<br>2. 创建 `CookieJar` 结构体<br>3. 实现 Cookie 解析<br>4. 实现过期处理 |
| **输出** | `src/cookie/jar.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] Cookie 解析正确
- [ ] Cookie 存储正确
- [ ] 过期处理正确

**测试用例**:
```rust
#[test]
fn test_cookie_jar() {
    let mut jar = CookieJar::new();
    
    let cookie = Cookie::parse(
        "session=abc123; Domain=example.com; Path=/",
        "example.com"
    ).unwrap();
    
    jar.add(cookie);
    
    let url: Url = "https://example.com/page".parse().unwrap();
    let cookie_header = jar.get_for_url(&url);
    
    assert!(cookie_header.contains("session=abc123"));
}

#[test]
fn test_cookie_expiry() {
    let mut cookie = Cookie {
        name: "test".to_string(),
        value: "value".to_string(),
        domain: "example.com".to_string(),
        path: "/".to_string(),
        expires: Some(chrono::Utc::now().timestamp() - 3600), // 1 小时前过期
        max_age: None,
        secure: false,
        http_only: false,
        creation_time: 0,
    };
    
    assert!(cookie.is_expired());
}
```

---

## T2.7: 测试与优化

### T2.7.1: 单元测试

| 项目 | 内容 |
|------|------|
| **输入** | 所有模块代码 |
| **处理步骤** | 1. 为每个模块编写测试<br>2. 覆盖边界条件<br>3. 测试错误路径<br>4. 测量覆盖率 |
| **输出** | 完整的单元测试套件 |
| **验证方法** | `cargo test` 通过<br>覆盖率 > 80% |

**检查项**:
- [ ] 每个公开函数有测试
- [ ] 边界条件测试完整
- [ ] 错误处理测试完整
- [ ] 测试覆盖率报告

**命令**:
```bash
cargo test -p turbo-crawler
cargo tarpaulin -p turbo-crawler
```

---

### T2.7.2: 集成测试

| 项目 | 内容 |
|------|------|
| **输入** | Mock 服务器<br>测试数据 |
| **处理步骤** | 1. 创建 Mock 服务器<br>2. 测试完整爬取流程<br>3. 测试资源提取<br>4. 测试并发场景 |
| **输出** | 集成测试套件 |
| **验证方法** | 所有集成测试通过 |

**检查项**:
- [ ] Mock 服务器正确配置
- [ ] 完整爬取流程测试
- [ ] 资源提取测试
- [ ] 并发安全测试

**测试场景**:
1. 简单页面爬取
2. 深度爬取
3. 资源提取
4. URL 过滤
5. 限速控制
6. Cookie 处理

---

### T2.7.3: 性能测试

| 项目 | 内容 |
|------|------|
| **输入** | 基准测试设计 |
| **处理步骤** | 1. 创建基准测试<br>2. 测量解析性能<br>3. 测量内存使用<br>4. 测量并发性能 |
| **输出** | 基准测试报告 |
| **验证方法** | 基准测试可重复运行 |

**基准测试**:
```bash
cargo bench -p turbo-crawler
```

---

## T2.8: 文档与示例

### T2.8.1: API 文档

| 项目 | 内容 |
|------|------|
| **输入** | 所有公开 API |
| **处理步骤** | 1. 添加文档注释<br>2. 生成 rustdoc<br>3. 添加示例代码<br>4. 审核文档完整性 |
| **输出** | 完整的 API 文档 |
| **验证方法** | `cargo doc` 无警告 |

**检查项**:
- [ ] 所有公开项有文档
- [ ] 示例代码可运行
- [ ] 链接正确
- [ ] 无文档警告

---

### T2.8.2: 使用示例

| 项目 | 内容 |
|------|------|
| **输入** | 常见使用场景 |
| **处理步骤** | 1. 创建基础示例<br>2. 创建高级示例<br>3. 创建完整应用示例<br>4. 测试所有示例 |
| **输出** | examples/ 目录 |
| **验证方法** | 所有示例可运行 |

**示例清单**:
- [ ] `basic_crawl.rs` - 基础爬取
- [ ] `depth_crawl.rs` - 深度爬取
- [ ] `resource_extract.rs` - 资源提取
- [ ] `filtered_crawl.rs` - 过滤爬取

---

## 发布前检查清单

### 代码质量

- [ ] 所有测试通过 (`cargo test`)
- [ ] 无 clippy 警告 (`cargo clippy`)
- [ ] 格式化正确 (`cargo fmt --check`)
- [ ] 无未使用的依赖 (`cargo udeps`)
- [ ] 文档完整 (`cargo doc`)

### 功能验证

- [ ] HTTP 抓取正常
- [ ] HTML 解析正常
- [ ] 资源提取正常
- [ ] URL 过滤正常
- [ ] 限速控制正常
- [ ] Cookie 管理正常

### 性能验证

- [ ] 解析速度达标
- [ ] 内存使用合理
- [ ] 并发安全

### 文档验证

- [ ] README 完整
- [ ] API 文档完整
- [ ] 示例代码可运行

### 发布准备

- [ ] 版本号更新
- [ ] 发布说明准备
- [ ] Git 标签创建