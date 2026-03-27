# TurboDownload 项目群概览

## 项目简介

TurboDownload 是一个高性能下载管理器，支持多线程下载、断点续传和网页资源抓取。本项目采用模块化架构，拆分为 6 个独立开发项目。

## 项目群架构

```
TurboDownload 项目群
├── Project-1: 核心下载引擎 (turbo-downloader)
├── Project-2: 资源抓取服务 (turbo-crawler)
├── Project-3: 前端 UI 框架 (turbo-ui)
├── Project-4: 下载管理器 (turbo-manager)
├── Project-5: 系统集成层 (turbo-integration)
└── Project-6: 主应用集成 (turbo-app)
```

## 技术栈

| 层级 | 技术 |
|------|------|
| **前端** | React 18 + TypeScript 5 + Tailwind CSS 3 + Zustand |
| **后端** | Rust + Tauri 2.x + Tokio + Reqwest |
| **桌面** | Tauri 2.x (跨平台) |
| **存储** | SQLite (任务持久化) |
| **测试** | Vitest (前端) + cargo-nextest (后端) |

## 项目明细

### P1: turbo-downloader (核心下载引擎)
- **职责**: HTTP/HTTPS 多线程下载核心
- **类型**: Rust crate (lib)
- **工时**: 40 人时
- **优先级**: P0 (最高)
- **核心功能**:
  - 多线程分片下载
  - 断点续传
  - 进度回调
  - 速度计算

### P2: turbo-crawler (资源抓取服务)
- **职责**: 网页资源抓取与解析
- **类型**: Rust crate (lib)
- **工时**: 30 人时
- **优先级**: P1
- **核心功能**:
  - HTML 解析
  - URL 提取与规范化
  - 资源类型分类
  - 整站扫描

### P3: turbo-ui (前端 UI 框架)
- **职责**: React + TypeScript UI 组件
- **类型**: npm package
- **工时**: 25 人时
- **优先级**: P1 (可并行开发)
- **核心功能**:
  - 基础组件库
  - 下载列表组件
  - 抓取面板组件
  - 状态管理

### P4: turbo-manager (下载管理器)
- **职责**: 下载任务调度与管理
- **类型**: Rust crate (lib)
- **工时**: 25 人时
- **优先级**: P1
- **核心功能**:
  - 任务队列管理
  - 状态机实现
  - 并发控制
  - 持久化存储

### P5: turbo-integration (系统集成层)
- **职责**: Tauri 集成、系统API、文件操作
- **类型**: Rust crate (lib)
- **工时**: 20 人时
- **优先级**: P2
- **核心功能**:
  - Tauri 命令封装
  - 文件系统操作
  - 系统通知
  - 配置管理

### P6: turbo-app (主应用集成)
- **职责**: 整合所有模块，主应用入口
- **类型**: Tauri 应用
- **工时**: 15 人时
- **优先级**: P2
- **核心功能**:
  - 应用入口
  - 模块集成
  - 主界面
  - 打包发布

## 开发里程碑

```
Week 1-2: M1 - 核心下载引擎就绪
├── P1 完成
└── 输出: 可独立使用的下载库

Week 2-3: M2 - 功能模块就绪
├── P2 完成
├── P3 完成
├── P4 完成
└── 输出: 所有功能模块可用

Week 3-4: M3 - 系统集成完成
├── P5 完成
├── P6 开发中
└── 输出: 可运行的应用原型

Week 4: M4 - 发布准备就绪
├── P6 完成
├── 测试通过
├── 打包完成
└── 输出: 可发布的桌面应用
```

## 文档索引

| 文档 | 描述 |
|------|------|
| [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md) | 项目群架构详细说明 |
| [WORKFLOW.md](./WORKFLOW.md) | 开发流程与规范 |
| [API_CONTRACTS.md](./API_CONTRACTS.md) | 模块间接口契约 |
| [TASK_BREAKDOWN.md](./TASK_BREAKDOWN.md) | 详细任务分解 |
| [DEPENDENCY_GRAPH.md](./DEPENDENCY_GRAPH.md) | 项目依赖关系图 |

## 快速开始

### 环境准备
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js (推荐使用 nvm)
nvm install 20
nvm use 20

# 安装依赖
cd ~/Projects/TurboDownload
cargo fetch
npm install
```

### 开发运行
```bash
# 启动开发服务器
npm run tauri:dev

# 运行测试
cargo test --workspace
npm run test
```

### 构建发布
```bash
# 构建所有平台
npm run tauri:build
```

## 职责分工

| 角色 | 职责 |
|------|------|
| **架构师** | 设计模块接口、技术选型、架构决策、代码审核 |
| **程序代理** | 执行具体开发任务、技术验证、编写测试 |
| **总管** | 协调各模块、进度把控、资源分配、风险管控 |

## 联系方式

- **项目地址**: ~/Projects/TurboDownload/
- **文档地址**: ~/Projects/TurboDownload/docs/