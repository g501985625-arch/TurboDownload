# P6: turbo-app 任务链设计

## 概述

本文档按乐高式任务链设计，每个子任务细化到1-2小时完成。

---

## 任务块概览

| 任务块 | 描述 | 预估工时 | 依赖 |
|--------|------|----------|------|
| T6.1 | 项目初始化 | 2h | P1-P5 |
| T6.2 | Rust 后端集成 | 3h | T6.1 |
| T6.3 | 前端基础架构 | 3h | T6.1 |
| T6.4 | 主界面开发 | 3h | T6.2, T6.3 |
| T6.5 | 端到端测试 | 2h | T6.4 |
| T6.6 | 打包发布 | 2h | T6.5 |
| **总计** | | **15h** | |

---

## T6.1: 项目初始化

### T6.1.1: 创建 Tauri 项目结构 (1小时)

**文件路径**: `apps/turbo-app/`

**步骤**:
1. 创建 `apps/turbo-app/` 目录
2. 初始化 npm 项目
3. 初始化 Tauri (`npx tauri init`)
4. 配置项目结构

**验收标准**:
- [ ] Tauri 项目创建成功
- [ ] 目录结构符合规范

---

### T6.1.2: 配置 Cargo.toml (30分钟)

**文件路径**: `src-tauri/Cargo.toml`

**步骤**:
1. 配置 package 信息
2. 添加内部模块依赖
3. 配置 features

**验收标准**:
- [ ] `cargo check` 通过

---

### T6.1.3: 配置前端项目 (30分钟)

**文件路径**: `package.json`, `tsconfig.json`, `vite.config.ts`

**步骤**:
1. 安装 React + TypeScript + Vite
2. 配置 Tailwind CSS
3. 配置路径别名

**验收标准**:
- [ ] `npm run dev` 成功
- [ ] 前端页面正常显示

---

## T6.2: Rust 后端集成

### T6.2.1: 创建 main.rs 入口 (30分钟)

**文件路径**: `src-tauri/src/main.rs`

**函数**: `main()`

**步骤**:
1. 定义 main 函数
2. 调用 lib 模块

**验收标准**:
- [ ] 编译通过

---

### T6.2.2: 创建 lib.rs 模块 (1小时)

**文件路径**: `src-tauri/src/lib.rs`

**函数**: `run()`

**步骤**:
1. 创建 Tauri Builder
2. 注册插件
3. 配置 setup
4. 注册命令

**验收标准**:
- [ ] 应用可启动
- [ ] 命令可调用

---

### T6.2.3: 创建 setup.rs 初始化 (1.5小时)

**文件路径**: `src-tauri/src/setup.rs`

**函数**: `init(app: &AppHandle)`

**步骤**:
1. 创建下载管理器实例
2. 初始化状态管理
3. 加载应用配置
4. 设置日志

**输入**: AppHandle
**输出**: Result<(), Box<dyn Error>>

**验收标准**:
- [ ] 管理器正确初始化
- [ ] 状态正确注入

---

### T6.2.4: 集成 turbo-integration 命令 (30分钟)

**文件路径**: `src-tauri/src/lib.rs`

**步骤**:
1. 导入 `turbo_integration::register_commands`
2. 添加到 invoke_handler

**验收标准**:
- [ ] 所有命令可调用

---

## T6.3: 前端基础架构

### T6.3.1: 创建状态管理 (1小时)

**文件路径**: `src/stores/`

**函数/类型**:
- `useDownloadStore`
- `useSettingsStore`

**步骤**:
1. 安装 zustand
2. 定义状态类型
3. 实现 actions
4. 实现 selectors

**验收标准**:
- [ ] Store 可正常使用
- [ ] 状态持久化正常

---

### T6.3.2: 创建 Tauri 命令 Hooks (1小时)

**文件路径**: `src/hooks/useTauriCommands.ts`

**函数**:
- `useAddDownload()`
- `useStartDownload()`
- `useGetProgress()`

**步骤**:
1. 封装 `invoke` 调用
2. 添加类型定义
3. 实现错误处理

**验收标准**:
- [ ] Hooks 可正常调用
- [ ] TypeScript 类型正确

---

### T6.3.3: 创建主布局组件 (1小时)

**文件路径**: `src/components/MainLayout.tsx`

**功能**:
- 侧边栏导航
- 主内容区域
- 顶部标题栏

**验收标准**:
- [ ] 布局响应式
- [ ] 导航正常工作

---

## T6.4: 主界面开发

### T6.4.1: 创建下载列表页面 (1.5小时)

**文件路径**: `src/components/DownloadPage.tsx`

**功能**:
- 任务列表显示
- 添加任务按钮
- 任务操作（暂停/恢复/取消）

**验收标准**:
- [ ] 任务列表正确显示
- [ ] 操作按钮正常工作

---

### T6.4.2: 创建爬虫面板页面 (1小时)

**文件路径**: `src/components/CrawlerPage.tsx`

**功能**:
- URL 输入
- 扫描配置
- 资源列表显示

**验收标准**:
- [ ] 扫描功能正常
- [ ] 资源列表正确显示

---

### T6.4.3: 创建设置页面 (30分钟)

**文件路径**: `src/components/SettingsPage.tsx`

**功能**:
- 下载目录设置
- 并发数设置
- 通知设置

**验收标准**:
- [ ] 设置可保存
- [ ] 设置可加载

---

### T6.4.4: 实现事件监听 (1小时)

**文件路径**: `src/hooks/useEventListeners.ts`

**功能**:
- 监听 `download:progress`
- 监听 `download:completed`
- 监听 `download:failed`

**验收标准**:
- [ ] 事件正确接收
- [ ] UI 正确更新

---

## T6.5: 端到端测试

### T6.5.1: 编写 E2E 测试脚本 (1小时)

**文件路径**: `tests/e2e/`

**测试用例**:
- 添加下载任务
- 暂停/恢复任务
- 取消任务
- 扫描网页

**验收标准**:
- [ ] 测试脚本可运行
- [ ] 核心流程覆盖

---

### T6.5.2: 执行集成测试 (1小时)

**步骤**:
1. 启动应用
2. 执行测试用例
3. 记录测试结果

**验收标准**:
- [ ] 所有测试通过
- [ ] 无明显 bug

---

## T6.6: 打包发布

### T6.6.1: 配置打包参数 (1小时)

**文件路径**: `tauri.conf.json`

**步骤**:
1. 配置应用图标
2. 配置应用签名
3. 配置自动更新

**验收标准**:
- [ ] 打包配置正确

---

### T6.6.2: 构建 macOS 应用 (30分钟)

**命令**: `npm run tauri:build -- --target universal-apple-darwin`

**输出**:
- `turbo-app_x64.app`
- `turbo-app_arm64.app`
- `turbo-app_universal.app`

**验收标准**:
- [ ] DMG 文件生成成功
- [ ] 应用可正常启动

---

### T6.6.3: 构建 Windows/Linux 应用 (30分钟)

**命令**: 
- Windows: `npm run tauri:build -- --target x86_64-pc-windows-msvc`
- Linux: `npm run tauri:build -- --target x86_64-unknown-linux-gnu`

**验收标准**:
- [ ] 安装包生成成功

---

## 验收汇总

| 任务块 | 子任务数 | 预估工时 | 验收标准 |
|--------|----------|----------|----------|
| T6.1 | 3 | 2h | 项目可启动 |
| T6.2 | 4 | 3h | 后端集成完成 |
| T6.3 | 3 | 3h | 前端基础完成 |
| T6.4 | 4 | 4h | 主界面可用 |
| T6.5 | 2 | 2h | 测试通过 |
| T6.6 | 3 | 1h | 打包完成 |

---

## 依赖关系图

```
T6.1 (项目初始化)
├── T6.2 (Rust 后端) ──┐
└── T6.3 (前端基础) ──┼── T6.4 (主界面) ── T6.5 (测试) ── T6.6 (打包)
                       └──────────────────────────────────────────────┘
```