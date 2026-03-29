# TurboDownload 项目依赖图

## 模块依赖关系总览

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              turbo-app (P6)                                  │
│                           [主应用集成入口]                                    │
└─────────────────────────────────────────────────────────────────────────────┘
                    │                    │                    │
                    ▼                    ▼                    ▼
        ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐
        │    turbo-ui       │  │  turbo-integration │  │                   │
        │      (P3)         │  │       (P5)         │  │                   │
        │   [前端UI框架]     │  │   [系统集成层]      │  │                   │
        └───────────────────┘  └─────────┬─────────┘  └───────────────────┘
                                           │
                    ┌──────────────────────┼──────────────────────┐
                    │                      │                      │
                    ▼                      ▼                      ▼
        ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐
        │   turbo-crawler   │  │   turbo-manager   │  │                   │
        │       (P2)        │  │       (P4)        │  │                   │
        │   [资源抓取服务]   │  │   [下载管理器]     │  │                   │
        └─────────┬─────────┘  └─────────┬─────────┘  └───────────────────┘
                  │                      │
                  └──────────┬───────────┘
                             │
                             ▼
                ┌─────────────────────────┐
                │   turbo-downloader      │
                │         (P1)            │
                │   [核心下载引擎]         │
                └─────────────────────────┘
                             │
                             ▼
                ┌─────────────────────────┐
                │   外部依赖              │
                │  tokio, reqwest, tauri │
                └─────────────────────────┘
```

---

## Rust Crate 依赖关系

### 依赖矩阵

| Crate | turbo-downloader | turbo-crawler | turbo-manager | turbo-integration | turbo-app |
|-------|:----------------:|:-------------:|:-------------:|:-----------------:|:---------:|
| turbo-downloader | - | ✅ | ✅ | - | - |
| turbo-crawler | - | - | - | ✅ | - |
| turbo-manager | - | - | - | ✅ | - |
| turbo-integration | - | - | - | - | ✅ |
| turbo-ui | - | - | - | - | ✅ |

### Cargo.toml 依赖声明

#### turbo-downloader (P1)
```toml
[package]
name = "turbo-downloader"
version = "0.1.0"

[dependencies]
tokio = { workspace = true }
reqwest = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
sha2 = "0.10"
futures = "0.3"
bytes = "1.5"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

#### turbo-crawler (P2)
```toml
[package]
name = "turbo-crawler"
version = "0.1.0"

[dependencies]
turbo-downloader = { path = "../turbo-downloader" }
tokio = { workspace = true }
reqwest = { workspace = true }
scraper = "0.18"
url = "2.5"
regex = "1.10"
serde = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
```

#### turbo-manager (P4)
```toml
[package]
name = "turbo-manager"
version = "0.1.0"

[dependencies]
turbo-downloader = { path = "../turbo-downloader" }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
rusqlite = { version = "0.30", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
```

#### turbo-integration (P5)
```toml
[package]
name = "turbo-integration"
version = "0.1.0"

[dependencies]
turbo-downloader = { path = "../turbo-downloader" }
turbo-crawler = { path = "../turbo-crawler" }
turbo-manager = { path = "../turbo-manager" }
tauri = { version = "2.0", features = ["notification-all", "dialog-all", "fs-all"] }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
directories = "5.0"
```

#### turbo-app/src-tauri (P6)
```toml
[package]
name = "turbo-app"
version = "0.1.0"

[dependencies]
turbo-integration = { path = "../../../crates/turbo-integration" }
tauri = { version = "2.0", features = ["tray-icon", "macos-private-api"] }
tauri-plugin-shell = "2.0"
tokio = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
```

---

## 前端依赖关系

### npm 依赖矩阵

| Package | turbo-ui | turbo-app |
|---------|:--------:|:---------:|
| turbo-ui | - | ✅ |

### package.json 依赖声明

#### turbo-ui (P3)
```json
{
  "name": "@turbodownload/ui",
  "version": "0.1.0",
  "type": "module",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "import": "./dist/index.js",
      "types": "./dist/index.d.ts"
    },
    "./styles.css": "./dist/styles.css"
  },
  "peerDependencies": {
    "react": ">=18.0.0",
    "react-dom": ">=18.0.0"
  },
  "dependencies": {
    "zustand": "^4.4.0",
    "lucide-react": "^0.300.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0"
  },
  "devDependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "typescript": "^5.3.0",
    "tailwindcss": "^3.4.0",
    "vite": "^5.0.0",
    "vitest": "^1.0.0",
    "@testing-library/react": "^14.0.0"
  }
}
```

#### turbo-app (P6)
```json
{
  "name": "@turbodownload/app",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build"
  },
  "dependencies": {
    "@turbodownload/ui": "workspace:*",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tauri-apps/api": "^2.0.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0"
  }
}
```

---

## 外部依赖清单

### Rust 外部依赖

| 依赖 | 版本 | 用途 | 使用模块 |
|------|------|------|----------|
| tokio | 1.35 | 异步运行时 | 所有 Rust crate |
| reqwest | 0.11 | HTTP 客户端 | P1, P2 |
| serde | 1.0 | 序列化 | 所有 Rust crate |
| serde_json | 1.0 | JSON 处理 | 所有 Rust crate |
| thiserror | 1.0 | 错误处理 | 所有 Rust crate |
| tracing | 0.1 | 日志追踪 | 所有 Rust crate |
| sha2 | 0.10 | 哈希计算 | P1 |
| futures | 0.3 | Future 扩展 | P1 |
| bytes | 1.5 | 字节处理 | P1 |
| scraper | 0.18 | HTML 解析 | P2 |
| url | 2.5 | URL 处理 | P2 |
| regex | 1.10 | 正则匹配 | P2 |
| rusqlite | 0.30 | SQLite 数据库 | P4 |
| chrono | 0.4 | 时间处理 | P1, P4 |
| uuid | 1.6 | UUID 生成 | P1, P4 |
| tauri | 2.0 | 桌面应用框架 | P5, P6 |
| directories | 5.0 | 系统目录 | P5 |

### Node.js 外部依赖

| 依赖 | 版本 | 用途 | 使用模块 |
|------|------|------|----------|
| react | 18.2 | UI 框架 | P3, P6 |
| react-dom | 18.2 | React DOM | P3, P6 |
| typescript | 5.3 | 类型系统 | P3, P6 |
| vite | 5.0 | 构建工具 | P3, P6 |
| tailwindcss | 3.4 | CSS 框架 | P3, P6 |
| zustand | 4.4 | 状态管理 | P3 |
| lucide-react | 0.300 | 图标库 | P3 |
| vitest | 1.0 | 测试框架 | P3 |

---

## 构建顺序

### 开发构建顺序

```
1. P1: turbo-downloader    (无依赖，首先构建)
   │
   ├─→ 2a. P2: turbo-crawler (依赖 P1)
   │
   ├─→ 2b. P4: turbo-manager (依赖 P1)
   │
   └─→ 2c. P3: turbo-ui      (无依赖，可并行)
          │
          ├─→ 3. P5: turbo-integration (依赖 P1, P2, P4)
          │
          └─→ 4. P6: turbo-app (依赖所有)
```

### 发布构建顺序

```bash
# 1. 构建所有 Rust crates
cargo build --workspace --release

# 2. 构建前端包
npm run build -w @turbodownload/ui

# 3. 构建主应用
npm run tauri:build -w @turbodownload/app
```

---

## 版本兼容性

### Rust 版本要求
- **最低版本**: Rust 1.70+
- **推荐版本**: Rust 1.75+

### Node.js 版本要求
- **最低版本**: Node.js 18.0+
- **推荐版本**: Node.js 20.0+

### 操作系统支持

| 操作系统 | 版本要求 | 架构 |
|----------|----------|------|
| macOS | 10.15+ (Catalina) | x86_64, aarch64 |
| Windows | Windows 10+ | x86_64 |
| Linux | glibc 2.31+ | x86_64 |

---

## 依赖更新策略

### 更新检查频率
- **安全更新**: 立即应用
- **小版本更新**: 每月检查
- **大版本更新**: 评估后决定

### 更新流程
1. 检查变更日志 (CHANGELOG)
2. 检查破坏性变更
3. 在开发分支测试
4. 更新锁定文件
5. 通过所有测试后合并

### 依赖审计
```bash
# Rust 依赖审计
cargo audit

# Node.js 依赖审计
npm audit
npm audit fix
```

---

## 工作空间配置

### 根目录 Cargo.toml
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
repository = "https://github.com/turbodownload/turbodownload"

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

### 根目录 package.json
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
    "dev": "npm run tauri:dev -w @turbodownload/app",
    "build": "npm run build -w @turbodownload/ui && npm run build -w @turbodownload/app",
    "test": "cargo test --workspace && npm run test --workspaces",
    "lint": "cargo clippy --workspace && npm run lint --workspaces",
    "audit": "cargo audit && npm audit"
  },
  "devDependencies": {
    "typescript": "^5.3.0"
  }
}
```