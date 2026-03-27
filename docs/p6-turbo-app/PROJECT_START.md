# P6: turbo-app 项目启动文档

## 项目概述

**项目名称**: turbo-app (主应用集成)  
**项目类型**: Tauri 桌面应用  
**预估工时**: 15 人时  
**优先级**: P2 - 依赖所有模块  
**依赖项**: 
- P1: turbo-downloader (核心下载引擎)
- P2: turbo-crawler (资源抓取服务)
- P3: turbo-ui (前端 UI 框架)
- P4: turbo-manager (下载管理器)
- P5: turbo-integration (系统集成层)

### 核心功能

- 应用入口
- 模块集成
- 主界面
- 打包发布

---

## 项目初始化步骤

### 步骤 1: 创建 Tauri 项目

```bash
# 进入项目根目录
cd ~/Projects/TurboDownload

# 创建应用目录
mkdir -p apps/turbo-app

# 初始化 Tauri 项目
cd apps/turbo-app
npm init -y
npm install @tauri-apps/cli@^2.0
npx tauri init
```

**预期输出**:
```
Tauri project initialized!
```

### 步骤 2: 创建目录结构

**最终目录结构**:
```
apps/turbo-app/
├── src-tauri/                  # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json         # Tauri 配置
│   ├── build.rs
│   ├── icons/                  # 应用图标
│   └── src/
│       ├── main.rs             # 主入口
│       ├── lib.rs
│       └── setup.rs            # 初始化逻辑
├── src/                        # 前端源码
│   ├── main.tsx                # 前端入口
│   ├── App.tsx                 # 根组件
│   ├── components/             # 页面组件
│   │   ├── MainLayout.tsx
│   │   ├── DownloadPage.tsx
│   │   ├── CrawlerPage.tsx
│   │   └── SettingsPage.tsx
│   ├── stores/                 # 状态管理
│   │   ├── downloadStore.ts
│   │   └── settingsStore.ts
│   ├── hooks/                  # 自定义 hooks
│   │   └── useTauriCommands.ts
│   └── styles/
│       └── globals.css
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
└── index.html
```

### 步骤 3: 配置 Cargo.toml

```toml
[package]
name = "turbo-app"
version = "0.1.0"
description = "TurboDownload Desktop Application"
authors = ["TurboDownload Team"]
license = "MIT"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[dependencies]
# 内部模块
turbo-downloader = { path = "../../crates/turbo-downloader" }
turbo-crawler = { path = "../../crates/turbo-crawler" }
turbo-manager = { path = "../../crates/turbo-manager" }
turbo-integration = { path = "../../crates/turbo-integration" }

# Tauri
tauri = { version = "2.0", features = ["notification", "shell-open"] }
tauri-plugin-shell = "2.0"

# 异步运行时
tokio = { workspace = true, features = ["full"] }

# 序列化
serde = { workspace = true }
serde_json = { workspace = true }

# 日志
tracing = { workspace = true }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 错误处理
thiserror = { workspace = true }
anyhow = "1.0"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

### 步骤 4: 配置 package.json

```json
{
  "name": "turbo-app",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.20.0",
    "zustand": "^4.4.0",
    "lucide-react": "^0.300.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0",
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "autoprefixer": "^10.4.16",
    "postcss": "^8.4.32",
    "tailwindcss": "^3.4.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0"
  }
}
```

### 步骤 5: 配置 tauri.conf.json

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "TurboDownload",
  "version": "0.1.0",
  "identifier": "com.turbodownload.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "TurboDownload",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "category": "Utility",
    "shortDescription": "High-performance download manager",
    "longDescription": "TurboDownload is a modern download manager with multi-threaded downloads, resume support, and web resource crawling.",
    "macOS": {
      "minimumSystemVersion": "10.13"
    }
  },
  "plugins": {
    "shell": {
      "open": true
    }
  }
}
```

---

## 开发环境配置

### 1. 安装前端依赖

```bash
cd ~/Projects/TurboDownload/apps/turbo-app
npm install
```

### 2. 安装 Rust 依赖

```bash
cd src-tauri
cargo fetch
```

### 3. 验证安装

```bash
# 开发模式启动
npm run tauri:dev
```

---

## 快速开始

### 主入口文件

创建 `src-tauri/src/main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    turbo_app::run()
}
```

创建 `src-tauri/src/lib.rs`:

```rust
mod setup;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            setup::init(app)?;
            Ok(())
        })
        .invoke_handler(turbo_integration::register_commands())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

创建 `src-tauri/src/setup.rs`:

```rust
use tauri::{AppHandle, Manager};
use std::sync::Arc;
use turbo_manager::{DownloadManager, DownloadManagerBuilder};

pub fn init(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 创建下载管理器
    let manager = DownloadManagerBuilder::new()
        .max_concurrent(3)
        .db_path(app.path().app_data_dir()?.join("tasks.db"))
        .temp_dir(std::env::temp_dir())
        .build()?;
    
    // 管理器状态
    app.manage(Arc::new(manager) as Arc<dyn DownloadManager>);
    
    Ok(())
}
```

---

## 开发工作流

### 日常开发

```bash
# 启动开发服务器（前端+后端热重载）
npm run tauri:dev

# 仅运行前端
npm run dev

# 仅检查 Rust
cargo check --manifest-path src-tauri/Cargo.toml
```

### 代码质量检查

```bash
# 前端检查
npm run build

# Rust 检查
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

### 构建发布

```bash
# 构建所有平台
npm run tauri:build
```

---

## 环境验证清单

- [ ] Node.js 18+ 安装完成
- [ ] Rust 工具链安装完成
- [ ] Tauri CLI 安装完成
- [ ] 前端依赖安装完成
- [ ] 项目可启动 (`npm run tauri:dev`)
- [ ] 窗口正确显示

---

## 相关文档

- [TASK_CHAIN.md](./TASK_CHAIN.md) - 详细任务链
- [DEV_CHECKLIST.md](./DEV_CHECKLIST.md) - 开发步骤清单
- [CODE_TEMPLATES.md](./CODE_TEMPLATES.md) - 代码模板
- [../API_CONTRACTS.md](../API_CONTRACTS.md) - 接口契约