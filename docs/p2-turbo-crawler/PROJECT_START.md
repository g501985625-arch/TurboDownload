# P2: turbo-crawler 项目启动文档

## 项目概述

**项目名称**: turbo-crawler (资源抓取服务)  
**项目类型**: Rust crate (lib)  
**预估工时**: 35 人时  
**优先级**: P1 (高) - 依赖 P1  
**依赖项**: turbo-downloader

### 核心功能

- 网页内容抓取
- HTML 解析与资源提取
- CSS/JS 资源解析
- 链接爬取与过滤
- 代理支持
- 请求限速与并发控制
- Cookie 管理

---

## 项目初始化步骤

### 步骤 1: 创建项目目录

```bash
# 进入项目根目录
cd ~/Projects/TurboDownload

# 创建 crate 目录
mkdir -p crates/turbo-crawler

# 初始化 Rust library 项目
cd crates/turbo-crawler
cargo init --lib
```

**预期输出**:
```
Created library package
```

### 步骤 2: 创建目录结构

```bash
# 创建源代码目录结构
mkdir -p src/{fetch,parse,extract,filter,rate_limit,cookie}

# 创建测试目录
mkdir -p tests

# 创建示例目录
mkdir -p examples
```

**最终目录结构**:
```
crates/turbo-crawler/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs              # 模块入口
│   ├── fetch/
│   │   ├── mod.rs          # 抓取模块入口
│   │   ├── client.rs       # HTTP 客户端封装
│   │   ├── request.rs      # 请求构建器
│   │   └── response.rs     # 响应处理
│   ├── parse/
│   │   ├── mod.rs          # 解析模块入口
│   │   ├── html.rs         # HTML 解析器
│   │   ├── css.rs          # CSS 解析器
│   │   └── js.rs           # JS 资源解析
│   ├── extract/
│   │   ├── mod.rs          # 提取模块入口
│   │   ├── links.rs        # 链接提取
│   │   ├── images.rs       # 图片提取
│   │   ├── media.rs        # 媒体资源提取
│   │   └── fonts.rs        # 字体提取
│   ├── filter/
│   │   ├── mod.rs          # 过滤模块入口
│   │   ├── url_filter.rs   # URL 过滤器
│   │   ├── mime_filter.rs  # MIME 类型过滤
│   │   └── size_filter.rs  # 文件大小过滤
│   ├── rate_limit/
│   │   ├── mod.rs          # 限速模块入口
│   │   ├── token_bucket.rs # 令牌桶算法
│   │   └── throttle.rs     # 节流控制
│   ├── cookie/
│   │   ├── mod.rs          # Cookie 模块入口
│   │   ├── jar.rs          # Cookie 存储
│   │   └── policy.rs       # Cookie 策略
│   └── error/
│       ├── mod.rs          # 错误模块入口
│       └── types.rs        # 错误类型定义
├── tests/
│   ├── mod.rs
│   ├── fetch_test.rs
│   ├── parse_test.rs
│   ├── extract_test.rs
│   └── filter_test.rs
└── examples/
    ├── basic_crawl.rs
    ├── depth_crawl.rs
    └── resource_extract.rs
```

### 步骤 3: 配置 Cargo.toml

```toml
[package]
name = "turbo-crawler"
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["TurboDownload Team"]
description = "High-performance web resource crawler and extractor"
repository = "https://github.com/turbodownload/turbo-crawler"
keywords = ["crawler", "scraper", "html", "web", "download"]
categories = ["web-programming", "parsing"]

[lib]
name = "turbo_crawler"
path = "src/lib.rs"

[dependencies]
# 异步运行时
tokio = { workspace = true, features = ["full"] }

# HTTP 客户端
reqwest = { workspace = true }

# HTML 解析
scraper = "0.18"
select = "0.6"

# URL 处理
url = { workspace = true }

# 序列化
serde = { workspace = true }
serde_json = { workspace = true }

# 错误处理
thiserror = { workspace = true }
anyhow = "1.0"

# 日志
tracing = { workspace = true }

# 正则表达式
regex = "1.10"

# CSS 解析
cssparser = "0.33"

# Robots.txt 解析
robotstxt = "0.3"

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# 并发控制
parking_lot = "0.12"
async-channel = "2.1"

# 域名处理
addr = "0.15"

# MIME 类型
mime = "0.3"
mime_guess = "2.0"

[dev-dependencies]
# 测试框架
tokio-test = "0.4"
wiremock = "0.5"
tempfile = "3.8"
criterion = { version = "0.5", features = ["async_tokio"] }

[[bench]]
name = "crawl_benchmark"
harness = false

[features]
default = ["native-tls"]
native-tls = ["reqwest/native-tls"]
rustls-tls = ["reqwest/rustls-tls"]
```

### 步骤 4: 配置工作空间

确保根目录 `Cargo.toml` 包含此 crate:

```toml
[workspace]
members = [
    "crates/turbo-downloader",
    "crates/turbo-crawler",
    # ... 其他 crates
]
resolver = "2"
```

---

## 开发环境配置

### 1. 安装 Rust 工具链

```bash
# 确保使用最新稳定版
rustup update stable

# 安装必要组件
rustup component add clippy rustfmt rust-src
```

### 2. 安装开发工具

```bash
# 安装 cargo-nextest (更好的测试运行器)
cargo install cargo-nextest

# 安装 cargo-watch (自动重新编译)
cargo install cargo-watch
```

### 3. 配置 IDE (VS Code)

创建 `.vscode/settings.json`:

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.inlayHints.chainingHints.enable": true,
  "rust-analyzer.inlayHints.parameterHints.enable": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

### 4. 配置 rustfmt

创建 `rustfmt.toml`:

```toml
edition = "2021"
max_width = 100
use_small_heuristics = "Default"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_imports = true
```

---

## 依赖安装命令

### 开发模式

```bash
# 下载所有依赖
cd ~/Projects/TurboDownload
cargo fetch -p turbo-crawler

# 或使用工作空间根目录
cargo fetch
```

### 验证安装

```bash
# 检查编译
cargo check -p turbo-crawler

# 运行测试
cargo test -p turbo-crawler

# 或使用 nextest
cargo nextest run -p turbo-crawler
```

---

## 快速开始

### 创建第一个示例

创建 `examples/basic_crawl.rs`:

```rust
use turbo_crawler::{Crawler, CrawlConfig, ResourceFilter};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建爬虫配置
    let config = CrawlConfig {
        start_url: "https://example.com".parse()?,
        max_depth: 2,
        max_pages: 100,
        concurrent_requests: 5,
        request_delay_ms: 500,
        user_agent: Some("TurboCrawler/0.1.0".to_string()),
        respect_robots_txt: true,
        allowed_domains: None,
        resource_filter: ResourceFilter::default(),
    };

    // 创建爬虫
    let crawler = Crawler::new(config)?;

    // 开始爬取并收集资源
    let resources = crawler.crawl().await?;

    println!("Found {} resources:", resources.len());
    for resource in resources {
        println!("  - {} ({:?})", resource.url, resource.resource_type);
    }

    Ok(())
}
```

### 运行示例

```bash
cargo run --example basic_crawl
```

---

## 开发工作流

### 日常开发

```bash
# 开启自动编译
cargo watch -x check -x test

# 运行特定测试
cargo test -p turbo-crawler --test fetch_test

# 运行基准测试
cargo bench -p turbo-crawler
```

### 代码质量检查

```bash
# 格式化代码
cargo fmt

# 检查格式
cargo fmt -- --check

# 运行 clippy
cargo clippy -- -D warnings

# 运行所有检查
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test
```

### 发布准备

```bash
# 运行所有测试
cargo test --release

# 生成文档
cargo doc --no-deps --open

# 检查未使用的依赖
cargo +nightly udeps
```

---

## 环境验证清单

- [ ] Rust 工具链安装完成 (`rustc --version`)
- [ ] Cargo 可用 (`cargo --version`)
- [ ] 项目编译成功 (`cargo check`)
- [ ] 测试通过 (`cargo test`)
- [ ] Clippy 无警告 (`cargo clippy`)
- [ ] rustfmt 通过 (`cargo fmt -- --check`)
- [ ] IDE 配置正确 (rust-analyzer 工作)

---

## 故障排除

### 常见问题

#### 1. HTML 解析错误

确保正确处理编码:
```rust
// 使用 scraper 库处理编码
let document = Html::parse_document(&html_content);
```

#### 2. Robots.txt 访问失败

正确处理网络错误:
```rust
match robotstxt::parse_from_url(&robots_url).await {
    Ok(rules) => { /* 使用规则 */ },
    Err(_) => { /* 默认允许 */ }
}
```

#### 3. 内存占用过高

对于大型网站，使用流式处理:
```rust
// 限制并发请求数
let semaphore = Arc::new(Semaphore::new(5));
```

---

## 相关文档

- [TASK_CHAIN.md](./TASK_CHAIN.md) - 详细任务链
- [DEV_CHECKLIST.md](./DEV_CHECKLIST.md) - 开发步骤清单
- [CODE_TEMPLATES.md](./CODE_TEMPLATES.md) - 代码模板
- [../API_CONTRACTS.md](../API_CONTRACTS.md) - 接口契约