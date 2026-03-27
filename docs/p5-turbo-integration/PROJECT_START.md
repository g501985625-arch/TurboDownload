# P5: turbo-integration 项目启动文档

## 项目概述

**项目名称**: turbo-integration (系统集成层)  
**项目类型**: Rust crate (lib)  
**预估工时**: 20 人时  
**优先级**: P2 - 依赖 P1, P2, P4  
**依赖项**: 
- P1: turbo-downloader (核心下载引擎)
- P2: turbo-crawler (资源抓取服务)
- P4: turbo-manager (下载管理器)

### 核心功能

- Tauri 命令封装
- 文件系统操作
- 系统通知
- 配置管理
- 事件系统

---

## 项目初始化步骤

### 步骤 1: 创建项目目录

```bash
# 进入项目根目录
cd ~/Projects/TurboDownload

# 创建 crate 目录
mkdir -p crates/turbo-integration

# 初始化 Rust library 项目
cd crates/turbo-integration
cargo init --lib
```

**预期输出**:
```
Created library package
```

### 步骤 2: 创建目录结构

```bash
# 创建源代码目录结构
mkdir -p src/{commands,events,config,fs,notification}

# 创建测试目录
mkdir -p tests
```

**最终目录结构**:
```
crates/turbo-integration/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs              # 模块入口
│   ├── commands/
│   │   ├── mod.rs          # 命令模块入口
│   │   ├── download.rs     # 下载相关命令
│   │   ├── crawler.rs      # 爬虫相关命令
│   │   └── system.rs       # 系统相关命令
│   ├── events/
│   │   ├── mod.rs          # 事件模块入口
│   │   └── emitter.rs      # 事件发送器
│   ├── config/
│   │   ├── mod.rs          # 配置模块入口
│   │   ├── manager.rs      # 配置管理器
│   │   └── types.rs        # 配置类型
│   ├── fs/
│   │   ├── mod.rs          # 文件系统模块入口
│   │   ├── dialog.rs       # 文件对话框
│   │   └── operations.rs   # 文件操作
│   ├── notification/
│   │   ├── mod.rs          # 通知模块入口
│   │   └── notify.rs       # 系统通知
│   └── error.rs            # 错误类型定义
├── tests/
│   ├── mod.rs
│   ├── commands_test.rs
│   ├── config_test.rs
│   └── events_test.rs
└── README.md
```

### 步骤 3: 配置 Cargo.toml

```toml
[package]
name = "turbo-integration"
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["TurboDownload Team"]
description = "System integration layer for TurboDownload"
repository = "https://github.com/turbodownload/turbo-integration"
keywords = ["tauri", "integration", "desktop"]
categories = ["gui", "os"]

[lib]
name = "turbo_integration"
path = "src/lib.rs"

[dependencies]
# 内部依赖
turbo-downloader = { path = "../turbo-downloader" }
turbo-crawler = { path = "../turbo-crawler" }
turbo-manager = { path = "../turbo-manager" }

# Tauri
tauri = { version = "2.0", features = ["notification"] }

# 异步运行时
tokio = { workspace = true, features = ["full"] }

# 序列化
serde = { workspace = true }
serde_json = { workspace = true }

# 错误处理
thiserror = { workspace = true }
anyhow = "1.0"

# 日志
tracing = { workspace = true }

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# 配置文件
directories = "5.0"
toml = "0.8"

[dev-dependencies]
tokio-test = "0.4"
tempfile = "3.8"

[features]
default = []
```

### 步骤 4: 配置工作空间

确保根目录 `Cargo.toml` 包含此 crate:

```toml
[workspace]
members = [
    "crates/turbo-downloader",
    "crates/turbo-crawler",
    "crates/turbo-manager",
    "crates/turbo-integration",
    # ... 其他 crates
]
resolver = "2"
```

---

## 开发环境配置

### 1. 安装 Tauri CLI

```bash
# 安装 Tauri CLI
cargo install tauri-cli --version "^2.0"

# 或使用 npm
npm install -g @tauri-apps/cli@^2.0
```

### 2. 检查系统依赖

```bash
# macOS - 已安装 Xcode Command Line Tools 即可
xcode-select --install

# 验证 Tauri 依赖
cargo tauri info
```

### 3. 配置 IDE (VS Code)

添加 Tauri 扩展推荐：

```json
{
  "recommendations": [
    "tauri-apps.tauri-vscode"
  ]
}
```

---

## 依赖安装命令

### 开发模式

```bash
# 下载所有依赖
cd ~/Projects/TurboDownload
cargo fetch -p turbo-integration
```

### 验证安装

```bash
# 检查编译
cargo check -p turbo-integration

# 运行测试
cargo test -p turbo-integration
```

---

## 快速开始

### 创建第一个 Tauri 命令

创建 `src/commands/download.rs`:

```rust
use tauri::AppHandle;
use turbo_manager::{DownloadManager, DownloadConfig};
use crate::error::IntegrationError;

/// 添加下载任务
#[tauri::command]
pub async fn add_download(
    url: String,
    config: Option<DownloadConfig>,
    app: AppHandle,
) -> Result<String, IntegrationError> {
    let manager = app.state::<std::sync::Arc<dyn DownloadManager>>();
    let task_id = manager.add_task(config.unwrap_or_default()).await?;
    Ok(task_id)
}
```

### 注册命令到 Tauri

在应用的 `main.rs` 中：

```rust
fn main() {
    tauri::Builder::default()
        .manage(create_download_manager())
        .invoke_handler(tauri::generate_handler![
            turbo_integration::commands::download::add_download,
            turbo_integration::commands::download::start_download,
            // ... 其他命令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 开发工作流

### 日常开发

```bash
# 开启自动编译
cargo watch -x "check -p turbo-integration"

# 运行测试
cargo test -p turbo-integration
```

### 代码质量检查

```bash
# 格式化代码
cargo fmt

# 检查格式
cargo fmt -- --check

# 运行 clippy
cargo clippy -- -D warnings
```

---

## 环境验证清单

- [ ] Rust 工具链安装完成
- [ ] Tauri CLI 安装完成 (`cargo tauri --version`)
- [ ] 项目编译成功 (`cargo check -p turbo-integration`)
- [ ] 测试通过 (`cargo test -p turbo-integration`)
- [ ] 内部依赖可用 (turbo-downloader, turbo-crawler, turbo-manager)

---

## 相关文档

- [TASK_CHAIN.md](./TASK_CHAIN.md) - 详细任务链
- [DEV_CHECKLIST.md](./DEV_CHECKLIST.md) - 开发步骤清单
- [CODE_TEMPLATES.md](./CODE_TEMPLATES.md) - 代码模板
- [../API_CONTRACTS.md](../API_CONTRACTS.md) - 接口契约