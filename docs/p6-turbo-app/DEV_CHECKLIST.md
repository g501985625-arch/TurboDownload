# P6: turbo-app 开发检查清单

## 概述

本文档提供详细的开发步骤检查清单，每个步骤可勾选确认完成。

---

## T6.1: 项目初始化

### T6.1.1: 创建 Tauri 项目结构

- [ ] 创建 `apps/turbo-app/` 目录
- [ ] 初始化 npm 项目 (`npm init -y`)
- [ ] 安装 Tauri CLI (`npm install @tauri-apps/cli@^2.0`)
- [ ] 初始化 Tauri (`npx tauri init`)
- [ ] 创建 `src-tauri/` 目录
- [ ] 创建 `src/` 目录
- [ ] 验证: 项目结构正确

### T6.1.2: 配置 Cargo.toml

- [ ] 设置 package name: `turbo-app`
- [ ] 设置 version: `0.1.0`
- [ ] 添加内部依赖:
  - [ ] `turbo-downloader`
  - [ ] `turbo-crawler`
  - [ ] `turbo-manager`
  - [ ] `turbo-integration`
- [ ] 添加 Tauri 依赖
- [ ] 验证: `cargo check` 通过

### T6.1.3: 配置前端项目

- [ ] 安装 React: `npm install react react-dom`
- [ ] 安装 TypeScript: `npm install -D typescript @types/react @types/react-dom`
- [ ] 安装 Vite: `npm install -D vite @vitejs/plugin-react`
- [ ] 配置 `tsconfig.json`
- [ ] 配置 `vite.config.ts`
- [ ] 安装 Tailwind CSS
- [ ] 配置 `tailwind.config.js`
- [ ] 验证: `npm run dev` 成功

---

## T6.2: Rust 后端集成

### T6.2.1: 创建 main.rs 入口

- [ ] 创建 `src-tauri/src/main.rs`
- [ ] 定义 `main()` 函数
- [ ] 调用 `turbo_app::run()`
- [ ] 添加 Windows 子系统属性
- [ ] 验证: 编译通过

### T6.2.2: 创建 lib.rs 模块

- [ ] 创建 `src-tauri/src/lib.rs`
- [ ] 定义 `run()` 函数
- [ ] 创建 Tauri Builder
- [ ] 注册 Shell 插件
- [ ] 配置 setup 回调
- [ ] 注册命令处理器
- [ ] 添加移动端入口点
- [ ] 验证: 应用可启动

### T6.2.3: 创建 setup.rs 初始化

- [ ] 创建 `src-tauri/src/setup.rs`
- [ ] 定义 `init()` 函数
- [ ] 创建下载管理器:
  - [ ] 配置最大并发数
  - [ ] 配置数据库路径
  - [ ] 配置临时目录
- [ ] 注入状态到 Tauri
- [ ] 加载应用配置
- [ ] 初始化日志
- [ ] 验证: 管理器正确初始化

### T6.2.4: 集成 turbo-integration 命令

- [ ] 导入 `turbo_integration`
- [ ] 使用 `register_commands()`
- [ ] 验证: 所有命令可调用

---

## T6.3: 前端基础架构

### T6.3.1: 创建状态管理

- [ ] 创建 `src/stores/` 目录
- [ ] 安装 zustand: `npm install zustand`
- [ ] 创建 `downloadStore.ts`:
  - [ ] 定义 `DownloadTask` 类型
  - [ ] 定义 `tasks` 状态
  - [ ] 实现 `addTask` action
  - [ ] 实现 `updateTask` action
  - [ ] 实现 `removeTask` action
- [ ] 创建 `settingsStore.ts`:
  - [ ] 定义配置类型
  - [ ] 实现配置读写
- [ ] 验证: Store 可正常使用

### T6.3.2: 创建 Tauri 命令 Hooks

- [ ] 创建 `src/hooks/` 目录
- [ ] 创建 `useTauriCommands.ts`
- [ ] 安装 `@tauri-apps/api`
- [ ] 实现命令封装:
  - [ ] `addDownload(url, config?)`
  - [ ] `startDownload(taskId)`
  - [ ] `pauseDownload(taskId)`
  - [ ] `resumeDownload(taskId)`
  - [ ] `cancelDownload(taskId)`
  - [ ] `getDownloadProgress(taskId)`
  - [ ] `getAllDownloads()`
- [ ] 添加 TypeScript 类型
- [ ] 验证: Hooks 可正常调用

### T6.3.3: 创建主布局组件

- [ ] 创建 `src/components/` 目录
- [ ] 安装 `lucide-react` 图标库
- [ ] 安装 `react-router-dom`
- [ ] 创建 `MainLayout.tsx`:
  - [ ] 侧边栏导航
  - [ ] 主内容区域
  - [ ] 顶部标题栏
- [ ] 配置路由
- [ ] 验证: 布局正常显示

---

## T6.4: 主界面开发

### T6.4.1: 创建下载列表页面

- [ ] 创建 `src/components/DownloadPage.tsx`
- [ ] 实现任务列表组件:
  - [ ] 任务卡片
  - [ ] 进度条
  - [ ] 速度显示
  - [ ] ETA 显示
- [ ] 实现操作按钮:
  - [ ] 暂停/恢复
  - [ ] 取消
  - [ ] 打开文件
  - [ ] 打开目录
- [ ] 实现"添加下载"功能:
  - [ ] URL 输入框
  - [ ] 配置选项
  - [ ] 提交按钮
- [ ] 验证: 下载流程正常

### T6.4.2: 创建爬虫面板页面

- [ ] 创建 `src/components/CrawlerPage.tsx`
- [ ] 实现 URL 输入区域
- [ ] 实现扫描配置:
  - [ ] 最大深度
  - [ ] 最大页面数
  - [ ] 资源类型过滤
- [ ] 实现资源列表:
  - [ ] 类型图标
  - [ ] 文件名
  - [ ] 大小
  - [ ] 选择框
- [ ] 实现批量下载功能
- [ ] 验证: 扫描流程正常

### T6.4.3: 创建设置页面

- [ ] 创建 `src/components/SettingsPage.tsx`
- [ ] 实现设置项:
  - [ ] 下载目录选择
  - [ ] 最大并发数
  - [ ] 默认线程数
  - [ ] 速度限制
  - [ ] 通知设置
- [ ] 实现保存/重置按钮
- [ ] 验证: 设置可保存和加载

### T6.4.4: 实现事件监听

- [ ] 创建 `src/hooks/useEventListeners.ts`
- [ ] 安装 `@tauri-apps/api/event`
- [ ] 监听下载事件:
  - [ ] `download:progress`
  - [ ] `download:completed`
  - [ ] `download:failed`
  - [ ] `task:state_changed`
- [ ] 监听爬虫事件:
  - [ ] `crawl:progress`
  - [ ] `crawl:completed`
- [ ] 更新 Store 状态
- [ ] 验证: 事件正确处理

---

## T6.5: 端到端测试

### T6.5.1: 编写 E2E 测试脚本

- [ ] 创建 `tests/e2e/` 目录
- [ ] 编写下载测试:
  - [ ] 添加任务
  - [ ] 开始下载
  - [ ] 暂停/恢复
  - [ ] 取消任务
- [ ] 编写爬虫测试:
  - [ ] 单页爬取
  - [ ] 整站扫描
- [ ] 验证: 测试脚本可运行

### T6.5.2: 执行集成测试

- [ ] 启动应用
- [ ] 执行所有测试用例
- [ ] 记录测试结果
- [ ] 修复发现的 bug
- [ ] 验证: 所有测试通过

---

## T6.6: 打包发布

### T6.6.1: 配置打包参数

- [ ] 准备应用图标:
  - [ ] 32x32.png
  - [ ] 128x128.png
  - [ ] icon.icns (macOS)
  - [ ] icon.ico (Windows)
- [ ] 配置 `tauri.conf.json`:
  - [ ] productName
  - [ ] identifier
  - [ ] version
  - [ ] bundle 配置
- [ ] 配置签名证书
- [ ] 验证: 配置正确

### T6.6.2: 构建 macOS 应用

- [ ] 执行构建命令
- [ ] 检查输出文件:
  - [ ] .app 文件
  - [ ] .dmg 文件
- [ ] 测试安装和运行
- [ ] 验证: 应用正常工作

### T6.6.3: 构建 Windows/Linux 应用

- [ ] 构建 Windows 版本
- [ ] 构建 Linux 版本
- [ ] 测试安装和运行
- [ ] 验证: 所有平台打包成功

---

## 最终验收清单

### 编译检查

- [ ] `cargo check` 无错误
- [ ] `cargo clippy` 无警告
- [ ] `npm run build` 成功

### 功能检查

- [ ] 添加下载任务正常
- [ ] 下载进度显示正常
- [ ] 暂停/恢复/取消正常
- [ ] 网页扫描正常
- [ ] 设置保存/加载正常
- [ ] 通知显示正常

### 打包检查

- [ ] macOS 打包成功
- [ ] 应用签名正确
- [ ] 应用可正常启动
- [ ] 自动更新可用（如配置）