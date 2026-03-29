# P4: turbo-manager 项目启动文档

## 项目概述

**项目名称**: turbo-manager (后端管理服务)  
**项目类型**: Rust crate (lib + bin)  
**预估工时**: 55 人时  
**优先级**: P1 (高) - 依赖 P1, P2  
**依赖项**: turbo-downloader, turbo-crawler

### 核心功能

- 任务管理 API (REST/gRPC)
- 任务调度与并发控制
- 配置管理与持久化
- 日志收集与查询
- WebSocket 实时通信
- 与前端 IPC 桥接
- 系统托盘集成

---

## 技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| Rust | 1.75+ | 核心语言 |
| Tokio | 1.x | 异步运行时 |
| Axum | 0.7 | Web 框架 |
| Tower | 0.4 | 中间件 |
| Serde | 1.x | 序列化 |
| SQLx | 0.7 | 数据库 |
| SQLite | 3.x | 嵌入式数据库 |
| tracing | 0.1 | 日志 |
| tower-http | 0.5 | HTTP 中间件 |

---

## 项目初始化步骤

### 步骤 1: 创建项目目录

```bash
# 进入项目根目录
cd ~/Projects/TurboDownload

# 创建 crate 目录
mkdir -p crates/turbo-manager

# 初始化 Rust 项目
cd crates/turbo-manager
cargo init --lib
```

**预期输出**:
```
Created library package
```

### 步骤 2: 创建目录结构

```bash
# 创建源代码目录结构
mkdir -p src/{api,scheduler,store,config,logging,ipc,task,events}

# 创建测试目录
mkdir -p tests

# 创建示例目录
mkdir -p examples
```

**最终目录结构**:
```
crates/turbo-manager/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs              # 库入口
│   ├── main.rs             # 可执行入口
│   ├── api/
│   │   ├── mod.rs          # API 模块入口
│   │   ├── routes.rs       # 路由定义
│   │   ├── handlers.rs     # 请求处理
│   │   ├── middleware.rs   # 中间件
│   │   └── ws.rs           # WebSocket 处理
│   ├── scheduler/
│   │   ├── mod.rs          # 调度器模块入口
│   │   ├── queue.rs        # 任务队列
│   │   ├── worker.rs       # 工作线程
│   │   └── priority.rs     # 优先级管理
│   ├── store/
│   │   ├── mod.rs          # 存储模块入口
│   │   ├── database.rs     # 数据库操作
│   │   ├── task_store.rs   # 任务存储
│   │   └── config_store.rs # 配置存储
│   ├── config/
│   │   ├── mod.rs          # 配置模块入口
│   │   ├── settings.rs     # 设置定义
│   │   └── loader.rs       # 配置加载
│   ├── logging/
│   │   ├── mod.rs          # 日志模块入口
│   │   ├── setup.rs        # 日志初始化
│   │   └── query.rs        # 日志查询
│   ├── ipc/
│   │   ├── mod.rs          # IPC 模块入口
│   │   ├── commands.rs     # 命令定义
│   │   └── events.rs       # 事件定义
│   ├── task/
│   │   ├── mod.rs          # 任务模块入口
│   │   ├── manager.rs      # 任务管理器
│   │   └── monitor.rs      # 任务监控
│   └── events/
│       ├── mod.rs          # 事件模块入口
│       └── bus.rs          # 事件总线
├── tests/
│   ├── mod.rs
│   ├── api_test.rs
│   ├── scheduler_test.rs
│   └── store_test.rs
└── examples/
    ├── basic_server.rs
    └── with_frontend.rs
```

### 步骤 3: 配置 Cargo.toml

```toml
[package]
name = "turbo-manager"
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["TurboDownload Team"]
description = "Backend management service for TurboDownload"
repository = "https://github.com/turbodownload/turbo-manager"
keywords = ["download", "manager", "api", "scheduler"]
categories = ["web-programming", "asynchronous"]

[lib]
name = "turbo_manager"
path = "src/lib.rs"

[[bin]]
name = "turbo-manager"
path = "src/main.rs"

[dependencies]
# 异步运行时
tokio = { workspace = true, features = ["full"] }

# Web 框架
axum = { version = "0.7", features = ["ws", "macros"] }
axum-extra = { version = "0.9", features = ["typed-header"] }
tower = { version = "0.4", features = ["util", "timeout", "limit"] }
tower-http = { version = "0.5", features = ["fs", "cors", "trace"] }
hyper = { version = "1.0", features = ["full"] }

# 序列化
serde = { workspace = true }
serde_json = { workspace = true }

# 数据库
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }

# 错误处理
thiserror = { workspace = true }
anyhow = "1.0"

# 日志
tracing = { workspace = true }
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# 时间
chrono = { version = "0.4", features = ["serde"] }

# UUID
uuid = { version = "1.0", features = ["v4", "serde"] }

# 配置
config = "0.14"
directories = "5.0"

# 并发
parking_lot = "0.12"
async-channel = "2.1"

# 内部依赖
turbo-downloader = { path = "../turbo-downloader" }
turbo-crawler = { path = "../turbo-crawler" }

[dev-dependencies]
# 测试
tokio-test = "0.4"
tempfile = "3.8"
reqwest = { version = "0.11", features = ["json"] }
criterion = { version = "0.5", features = ["async_tokio"] }

[features]
default = ["sqlite"]
sqlite = ["sqlx/sqlite"]
```

### 步骤 4: 配置工作空间

确保根目录 `Cargo.toml` 包含此 crate:

```toml
[workspace]
members = [
    "crates/turbo-downloader",
    "crates/turbo-crawler",
    "crates/turbo-manager",
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

### 2. 安装 SQLite

```bash
# macOS
brew install sqlite

# Ubuntu/Debian
sudo apt-get install libsqlite3-dev

# Windows (使用 vcpkg 或下载预编译版本)
```

### 3. 配置数据库

```bash
# 创建数据库目录
mkdir -p data

# 创建数据库
sqlite3 data/turbo.db < schema/init.sql
```

### 4. 配置 IDE (VS Code)

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

---

## 依赖安装命令

### 开发模式

```bash
# 下载所有依赖
cd ~/Projects/TurboDownload
cargo fetch -p turbo-manager

# 或使用工作空间根目录
cargo fetch
```

### 验证安装

```bash
# 检查编译
cargo check -p turbo-manager

# 运行服务
cargo run -p turbo-manager

# 运行测试
cargo test -p turbo-manager
```

---

## 快速开始

### 创建第一个示例

创建 `examples/basic_server.rs`:

```rust
use turbo_manager::{Manager, ManagerConfig};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建管理器配置
    let config = ManagerConfig {
        bind_address: "127.0.0.1:3000".parse()?,
        database_url: "sqlite:data/turbo.db".to_string(),
        max_concurrent_tasks: 3,
        download_dir: "./downloads".into(),
    };

    // 创建管理器
    let manager = Manager::new(config).await?;

    // 启动服务
    let addr: SocketAddr = "127.0.0.1:3000".parse()?;
    manager.start(addr).await?;

    println!("Server running at http://{}", addr);

    // 等待关闭信号
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}
```

### 运行示例

```bash
cargo run --example basic_server
```

---

## 开发工作流

### 日常开发

```bash
# 开启自动编译
cargo watch -x check -x test

# 运行特定测试
cargo test -p turbo-manager --test api_test

# 运行基准测试
cargo bench -p turbo-manager
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
- [ ] SQLite 安装完成 (`sqlite3 --version`)
- [ ] 项目编译成功 (`cargo check`)
- [ ] 测试通过 (`cargo test`)
- [ ] Clippy 无警告 (`cargo clippy`)
- [ ] rustfmt 通过 (`cargo fmt -- --check`)
- [ ] IDE 配置正确 (rust-analyzer 工作)

---

## 故障排除

### 常见问题

#### 1. SQLite 链接错误

确保安装了 SQLite 开发库:
```bash
# macOS
brew install sqlite

# 设置链接器标志
export RUSTFLAGS="-L /usr/local/opt/sqlite/lib"
```

#### 2. 端口占用

检查端口是否被占用:
```bash
lsof -i :3000
kill -9 <PID>
```

#### 3. 数据库迁移失败

手动运行迁移:
```bash
sqlx migrate run
```

---

## 相关文档

- [TASK_CHAIN.md](./TASK_CHAIN.md) - 详细任务链
- [DEV_CHECKLIST.md](./DEV_CHECKLIST.md) - 开发步骤清单
- [CODE_TEMPLATES.md](./CODE_TEMPLATES.md) - 代码模板
- [../API_CONTRACTS.md](../API_CONTRACTS.md) - 接口契约