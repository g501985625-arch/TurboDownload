# TurboDownload 项目群架构

## 概述

TurboDownload 采用模块化架构，拆分为 6 个独立开发项目，每个项目可独立开发、测试和发布。

## 项目群架构图

```
TurboDownload 项目群
│
├── crates/                          # Rust crates 工作空间
│   ├── turbo-downloader/           # Project-1: 核心下载引擎
│   ├── turbo-crawler/              # Project-2: 资源抓取服务
│   ├── turbo-manager/              # Project-4: 下载管理器
│   └── turbo-integration/          # Project-5: 系统集成层
│
├── packages/                        # 前端包
│   └── turbo-ui/                   # Project-3: 前端 UI 框架
│
└── apps/                           # 应用程序
    └── turbo-app/                  # Project-6: 主应用集成
        ├── src/                    # React 前端入口
        └── src-tauri/              # Tauri 后端入口
```

## 项目清单

### Project-1: turbo-downloader (核心下载引擎)

| 属性 | 描述 |
|------|------|
| **类型** | Rust crate |
| **路径** | `crates/turbo-downloader/` |
| **职责** | HTTP/HTTPS 多线程下载核心 |
| **技术栈** | Rust + reqwest + tokio |
| **输出** | 独立的 Rust crate (lib) |
| **依赖** | tokio, reqwest, sha2, serde |

**核心功能**：
- HTTP/HTTPS 协议支持
- 多线程分片下载
- 断点续传
- 进度回调
- 速度计算
- 错误重试

---

### Project-2: turbo-crawler (资源抓取服务)

| 属性 | 描述 |
|------|------|
| **类型** | Rust crate |
| **路径** | `crates/turbo-crawler/` |
| **职责** | 网页资源抓取与解析 |
| **技术栈** | Rust + scraper + reqwest |
| **输出** | 独立的 Rust crate (lib) |
| **依赖** | turbo-downloader, scraper, url, regex |

**核心功能**：
- HTML 解析
- URL 提取与规范化
- 资源类型分类
- 整站扫描
- 深度爬取

---

### Project-3: turbo-ui (前端 UI 框架)

| 属性 | 描述 |
|------|------|
| **类型** | npm package |
| **路径** | `packages/turbo-ui/` |
| **职责** | React + TypeScript UI 组件 |
| **技术栈** | React + TypeScript + Tailwind + Zustand |
| **输出** | 独立的 npm package |
| **依赖** | React, lucide-react, zustand |

**核心功能**：
- 基础组件库 (Button, Modal, Input, Progress)
- 下载列表组件
- 抓取面板组件
- 设置面板
- 状态管理 (Zustand stores)

---

### Project-4: turbo-manager (下载管理器)

| 属性 | 描述 |
|------|------|
| **类型** | Rust crate |
| **路径** | `crates/turbo-manager/` |
| **职责** | 下载任务调度与管理 |
| **技术栈** | Rust + tokio |
| **输出** | 独立的 Rust crate (lib) |
| **依赖** | turbo-downloader, tokio, serde |

**核心功能**：
- 任务队列管理
- 状态机实现
- 并发控制
- 持久化存储
- 事件通知

---

### Project-5: turbo-integration (系统集成层)

| 属性 | 描述 |
|------|------|
| **类型** | Rust crate |
| **路径** | `crates/turbo-integration/` |
| **职责** | Tauri 集成、系统API、文件操作 |
| **技术栈** | Rust + Tauri 2.x |
| **输出** | 独立的 Rust crate (lib) |
| **依赖** | turbo-manager, turbo-crawler, tauri |

**核心功能**：
- Tauri 命令封装
- 文件系统操作
- 系统通知
- 配置管理
- IPC 通信

---

### Project-6: turbo-app (主应用集成)

| 属性 | 描述 |
|------|------|
| **类型** | Tauri 应用 |
| **路径** | `apps/turbo-app/` |
| **职责** | 整合所有模块，主应用入口 |
| **技术栈** | Tauri 2.x + React |
| **输出** | 完整的桌面应用 (dmg, exe, AppImage) |
| **依赖** | 所有其他项目 |

**核心功能**：
- 应用入口
- 模块集成
- 主界面
- 应用配置
- 打包发布

## 项目依赖关系

```
                    ┌─────────────────┐
                    │   turbo-app     │ (Project-6)
                    │   (Main App)    │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
    ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
    │  turbo-ui   │  │  turbo-     │  │  turbo-     │
    │  (UI)       │  │ integration │  │ manager     │
    │  (P3)       │  │ (P5)        │  │ (P4)        │
    └─────────────┘  └──────┬──────┘  └──────┬──────┘
                            │                │
                            ▼                ▼
                    ┌─────────────┐  ┌─────────────┐
                    │  turbo-     │  │  turbo-     │
                    │  crawler    │  │ downloader  │
                    │  (P2)       │  │ (P1)        │
                    └──────┬──────┘  └─────────────┘
                           │
                           ▼
                   ┌─────────────┐
                   │  turbo-     │
                   │ downloader  │
                   │ (P1)        │
                   └─────────────┘
```

## 工作空间配置

### Cargo.toml (根目录)

```toml
[workspace]
members = [
    "crates/turbo-downloader",
    "crates/turbo-crawler",
    "crates/turbo-manager",
    "crates/turbo-integration",
    "apps/turbo-app/src-tauri",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["TurboDownload Team"]

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
sha2 = "0.10"
```

### package.json (根目录)

```json
{
  "name": "turbo-download-monorepo",
  "version": "0.1.0",
  "private": true,
  "workspaces": [
    "packages/*",
    "apps/*"
  ],
  "scripts": {
    "dev": "npm run dev -w turbo-app",
    "build": "npm run build -w turbo-ui && npm run build -w turbo-app",
    "test": "npm run test --workspaces",
    "lint": "eslint packages apps --ext .ts,.tsx"
  }
}
```

## 目录结构详览

```
TurboDownload/
├── Cargo.toml                      # Rust 工作空间配置
├── package.json                    # npm 工作空间配置
├── Cargo.lock
├── package-lock.json
│
├── crates/                         # Rust crates
│   ├── turbo-downloader/          # P1: 核心下载引擎
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── http_client.rs
│   │   │   ├── chunk.rs
│   │   │   ├── download.rs
│   │   │   ├── progress.rs
│   │   │   ├── resume.rs
│   │   │   └── error.rs
│   │   └── tests/
│   │       ├── mod.rs
│   │       ├── download_test.rs
│   │       └── chunk_test.rs
│   │
│   ├── turbo-crawler/              # P2: 资源抓取服务
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── parser.rs
│   │   │   ├── extractor.rs
│   │   │   ├── classifier.rs
│   │   │   ├── scanner.rs
│   │   │   └── error.rs
│   │   └── tests/
│   │
│   ├── turbo-manager/              # P4: 下载管理器
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── queue.rs
│   │   │   ├── state.rs
│   │   │   ├── scheduler.rs
│   │   │   ├── storage.rs
│   │   │   └── error.rs
│   │   └── tests/
│   │
│   └── turbo-integration/          # P5: 系统集成层
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── commands/
│       │   │   ├── mod.rs
│       │   │   ├── download.rs
│       │   │   ├── crawler.rs
│       │   │   └── system.rs
│       │   ├── fs.rs
│       │   ├── notify.rs
│       │   ├── config.rs
│       │   └── error.rs
│       └── tests/
│
├── packages/                       # 前端包
│   └── turbo-ui/                  # P3: 前端 UI 框架
│       ├── package.json
│       ├── tsconfig.json
│       ├── src/
│       │   ├── index.ts
│       │   ├── components/
│       │   │   ├── Button/
│       │   │   ├── Modal/
│       │   │   ├── Input/
│       │   │   ├── Progress/
│       │   │   ├── DownloadList/
│       │   │   ├── CrawlerPanel/
│       │   │   └── Settings/
│       │   ├── stores/
│       │   │   ├── downloadStore.ts
│       │   │   ├── crawlerStore.ts
│       │   │   └── settingsStore.ts
│       │   ├── hooks/
│       │   └── types/
│       └── tests/
│
├── apps/                           # 应用程序
│   └── turbo-app/                  # P6: 主应用集成
│       ├── package.json
│       ├── src/                    # React 前端
│       │   ├── main.tsx
│       │   ├── App.tsx
│       │   └── index.css
│       ├── src-tauri/              # Tauri 后端
│       │   ├── Cargo.toml
│       │   ├── tauri.conf.json
│       │   ├── src/
│       │   │   ├── main.rs
│       │   │   └── lib.rs
│       │   └── icons/
│       └── dist/
│
└── docs/                           # 文档
    ├── PROJECT_STRUCTURE.md
    ├── WORKFLOW.md
    ├── API_CONTRACTS.md
    ├── TASK_BREAKDOWN.md
    └── DEPENDENCY_GRAPH.md
```

## 开发环境要求

### Rust 环境
- Rust 1.70+ (推荐使用 rustup)
- cargo-edit (可选，用于依赖管理)
- cargo-nextest (可选，用于测试)

### Node.js 环境
- Node.js 18+
- npm 9+ 或 pnpm 8+

### 系统依赖
- macOS: Xcode Command Line Tools
- Windows: Microsoft Visual Studio C++ Build Tools
- Linux: build-essential, libgtk-3-dev, libwebkit2gtk-4.0-dev

## 快速开始

### 1. 克隆项目
```bash
cd ~/Projects/TurboDownload
```

### 2. 安装依赖
```bash
# Rust 依赖
cargo fetch

# Node.js 依赖
npm install
```

### 3. 开发模式
```bash
# 启动开发服务器
npm run dev

# 或只构建特定 crate
cargo build -p turbo-downloader
```

### 4. 运行测试
```bash
# Rust 测试
cargo test --workspace

# 前端测试
npm run test
```

### 5. 构建发布
```bash
# 构建所有
npm run build

# 或构建特定平台
cargo tauri build
```