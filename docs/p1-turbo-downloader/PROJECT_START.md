# P1: turbo-downloader 项目启动文档

## 项目概述

**项目名称**: turbo-downloader (核心下载引擎)  
**项目类型**: Rust crate (lib)  
**预估工时**: 40 人时  
**优先级**: P0 (最高) - 其他项目依赖  
**依赖项**: 无

### 核心功能

- HTTP/HTTPS 协议支持
- 多线程分片下载
- 断点续传
- 进度回调
- 速度计算
- 错误重试

---

## 项目初始化步骤

### 步骤 1: 创建项目目录

```bash
# 进入项目根目录
cd ~/Projects/TurboDownload

# 创建 crate 目录
mkdir -p crates/turbo-downloader

# 初始化 Rust library 项目
cd crates/turbo-downloader
cargo init --lib
```

**预期输出**:
```
Created library package
```

### 步骤 2: 创建目录结构

```bash
# 创建源代码目录结构
mkdir -p src/{http,chunk,download,progress,resume,error}

# 创建测试目录
mkdir -p tests

# 创建示例目录
mkdir -p examples
```

**最终目录结构**:
```
crates/turbo-downloader/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs              # 模块入口
│   ├── http/
│   │   ├── mod.rs          # HTTP 模块入口
│   │   ├── client.rs       # HTTP 客户端
│   │   └── response.rs     # 响应处理
│   ├── chunk/
│   │   ├── mod.rs          # 分片模块入口
│   │   ├── strategy.rs     # 分片策略
│   │   └── worker.rs       # 分片下载器
│   ├── download/
│   │   ├── mod.rs          # 下载模块入口
│   │   ├── task.rs         # 下载任务
│   │   └── manager.rs      # 下载管理
│   ├── progress/
│   │   ├── mod.rs          # 进度模块入口
│   │   ├── tracker.rs      # 进度追踪
│   │   └── speed.rs        # 速度计算
│   ├── resume/
│   │   ├── mod.rs          # 断点续传模块入口
│   │   ├── state.rs        # 状态持久化
│   │   └── recovery.rs     # 恢复逻辑
│   └── error/
│       ├── mod.rs          # 错误模块入口
│       └── types.rs        # 错误类型定义
├── tests/
│   ├── mod.rs
│   ├── http_test.rs
│   ├── chunk_test.rs
│   ├── download_test.rs
│   └── resume_test.rs
└── examples/
    └── basic_download.rs
```

### 步骤 3: 配置 Cargo.toml

```toml
[package]
name = "turbo-downloader"
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["TurboDownload Team"]
description = "High-performance multi-threaded download engine"
repository = "https://github.com/turbodownload/turbo-downloader"
keywords = ["download", "http", "async", "multi-thread"]
categories = ["network-programming", "asynchronous"]

[lib]
name = "turbo_downloader"
path = "src/lib.rs"

[dependencies]
# 异步运行时
tokio = { workspace = true, features = ["full"] }

# HTTP 客户端
reqwest = { workspace = true }

# 序列化
serde = { workspace = true }
serde_json = { workspace = true }

# 错误处理
thiserror = { workspace = true }
anyhow = "1.0"

# 日志
tracing = { workspace = true }

# 加密 (用于 ETag 验证)
sha2 = "0.10"

# 异步工具
futures = "0.3"
bytes = "1.5"

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# UUID 生成
uuid = { version = "1.6", features = ["v4", "serde"] }

# 并发工具
parking_lot = "0.12"

[dev-dependencies]
# 测试框架
tokio-test = "0.4"
wiremock = "0.5"
tempfile = "3.8"
criterion = { version = "0.5", features = ["async_tokio"] }

[[bench]]
name = "download_benchmark"
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
    # ... 其他 crates
]
resolver = "2"
```

---

## 开发环境配置

### 1. 安装 Rust 工具链

```bash
# 安装 rustup (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 确保使用最新稳定版
rustup update stable

# 安装必要组件
rustup component add clippy rustfmt rust-src
```

### 2. 安装开发工具

```bash
# 安装 cargo-edit (可选，用于依赖管理)
cargo install cargo-edit

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

创建 `.vscode/extensions.json`:

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "vadimcn.vscode-lldb",
    "tamasfe.even-better-toml",
    "serayuzgur.crates"
  ]
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

### 5. 配置 clippy

创建 `clippy.toml`:

```toml
msrv = "1.70"
avoid-breaking-exported-api = false
```

---

## 依赖安装命令

### 开发模式

```bash
# 下载所有依赖
cd ~/Projects/TurboDownload
cargo fetch -p turbo-downloader

# 或使用工作空间根目录
cargo fetch
```

### 验证安装

```bash
# 检查编译
cargo check -p turbo-downloader

# 运行测试
cargo test -p turbo-downloader

# 或使用 nextest
cargo nextest run -p turbo-downloader
```

---

## 快速开始

### 创建第一个示例

创建 `examples/basic_download.rs`:

```rust
use turbo_downloader::{DownloaderBuilder, DownloadConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建下载器
    let downloader = DownloaderBuilder::new()
        .max_concurrent_tasks(3)
        .default_threads(4)
        .build()?;

    // 创建下载配置
    let config = DownloadConfig {
        id: uuid::Uuid::new_v4().to_string(),
        url: "https://example.com/largefile.zip".to_string(),
        output_path: PathBuf::from("./downloads/largefile.zip"),
        threads: 4,
        chunk_size: 0, // 自动
        resume_support: true,
        user_agent: None,
        headers: Default::default(),
        speed_limit: 0,
    };

    // 创建任务
    let task_id = downloader.create_task(config.clone()).await?;

    // 开始下载
    downloader.start(&task_id, Some(Box::new(|progress| {
        println!("Progress: {:.2}%", 
            progress.downloaded as f64 / progress.total as f64 * 100.0);
    }))).await?;

    Ok(())
}
```

### 运行示例

```bash
cargo run --example basic_download
```

---

## 开发工作流

### 日常开发

```bash
# 开启自动编译
cargo watch -x check -x test

# 运行特定测试
cargo test -p turbo-downloader --test http_test

# 运行基准测试
cargo bench -p turbo-downloader
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

#### 1. 依赖下载失败

```bash
# 配置镜像源 (中国大陆)
# 编辑 ~/.cargo/config.toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

#### 2. OpenSSL 编译错误

```bash
# macOS
brew install openssl
export OPENSSL_DIR=/usr/local/opt/openssl

# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config
```

#### 3. Tokio 运行时错误

确保所有异步代码在 tokio 运行时中执行：
```rust
#[tokio::main]
async fn main() {
    // 异步代码
}
```

---

## 相关文档

- [TASK_CHAIN.md](./TASK_CHAIN.md) - 详细任务链
- [DEV_CHECKLIST.md](./DEV_CHECKLIST.md) - 开发步骤清单
- [CODE_TEMPLATES.md](./CODE_TEMPLATES.md) - 代码模板
- [../API_CONTRACTS.md](../API_CONTRACTS.md) - 接口契约