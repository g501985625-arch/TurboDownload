# P5 + P6 详细任务链

> **创建日期**: 2026-03-30
> **目标**: 完成 TurboDownload App 集成与构建
> **流程版本**: v2.1

---

## P5: 模块集成测试

### 阶段目标
将 P1-P4 的所有模块集成到 Tauri App，确保各模块协同工作

---

### T5.1: 前端集成 (P4 → Tauri)

**任务ID**: T5.1
**负责人**: 开发员 (developer)
**前置依赖**: 无
**预计耗时**: 2h
**截止时间**: 2026-03-30 12:00

#### 工作内容

**步骤 1: 复制 P4 UI 源码**
```bash
# 源路径: ~/Projects/TurboDownload/crates/turbo-ui/src/
# 目标路径: ~/.openclaw/workspace/projects/TurboDownload/src/
```
- 复制 pages/ 目录 (Dashboard.tsx, Download.tsx, Radar.tsx, Settings.tsx)
- 复制 components/ 目录
- 复制 types/ 目录 (download.ts)
- 复制 store/ 目录 (downloadStore.ts)
- 复制 hooks/ 目录 (useProgressUpdater.ts)
- 复制 api/ 目录 (tauri.ts)

**步骤 2: 配置 package.json**
- 添加 React + Vite + Ant Design 依赖
- 配置 Tauri 集成脚本

**步骤 3: 配置 Vite**
- 适配 Tauri 的 dev server 端口
- 配置 build 输出目录为 `../src-tauri/dist`

**步骤 4: 修复路径问题**
- 检查并修复所有导入路径
- 确保 TypeScript 配置正确

#### 验收标准
- [ ] `npm install` 成功
- [ ] `npm run dev` 启动开发服务器 (端口 1420)
- [ ] `npm run build` 生成生产包到 `src-tauri/dist/`
- [ ] 无 TypeScript 编译错误
- [ ] 无 ESLint 错误

#### 产出文件
```
src/
├── api/
│   └── tauri.ts
├── components/
├── hooks/
│   └── useProgressUpdater.ts
├── pages/
│   ├── Dashboard.tsx
│   ├── Download.tsx
│   ├── Radar.tsx
│   └── Settings.tsx
├── store/
│   └── downloadStore.ts
├── types/
│   └── download.ts
├── main.tsx          # React 入口
└── style.css
package.json          # 更新依赖
vite.config.ts        # Vite + Tauri 配置
tsconfig.json         # TypeScript 配置
```

---

### T5.2: Rust 命令实现

**任务ID**: T5.2
**负责人**: 主程序 (lead-developer)
**前置依赖**: T5.1 完成
**预计耗时**: 3h
**截止时间**: 2026-03-30 15:00

#### 工作内容

**步骤 1: 添加依赖**
```toml
# src-tauri/Cargo.toml
[dependencies]
turbo-downloader = { path = "../../TurboDownload/crates/turbo-downloader" }
turbo-crawler = { path = "../../TurboDownload/crates/turbo-crawler" }
```

**步骤 2: 实现下载命令**
```rust
// src-tauri/src/commands/download.rs
pub async fn start_download(url: String, filename: String) -> Result<DownloadTask, String> {
    // 调用 turbo-downloader 库
    // 返回真实任务状态
}
```
- 使用 `turbo_downloader::Downloader` 创建下载任务
- 实现 `pause_download` - 调用 `manager.pause()`
- 实现 `resume_download` - 调用 `manager.resume()`
- 实现 `cancel_download` - 调用 `manager.cancel()`

**步骤 3: 实现爬虫命令**
```rust
// src-tauri/src/commands/crawler.rs
pub async fn crawl_url(url: String) -> Result<CrawlResult, String> {
    // 调用 turbo-crawler 库
}
```
- 使用 `turbo_crawler::Crawler` 扫描 URL
- 返回资源列表

**步骤 4: 添加事件系统**
- 实现进度事件推送
- 实现状态变更通知

#### 验收标准
- [ ] `cargo check` 无错误
- [ ] `cargo test` 通过
- [ ] 命令能调用底层库
- [ ] 返回真实数据
- [ ] 错误处理完善

#### 产出文件
```
src-tauri/
├── Cargo.toml          # 添加依赖
└── src/
    └── commands/
        ├── download.rs   # 完整实现
        └── crawler.rs    # 完整实现
```

---

### T5.3: 端到端测试

**任务ID**: T5.3
**负责人**: 开发员 (developer)
**前置依赖**: T5.2 完成
**预计耗时**: 2h
**截止时间**: 2026-03-30 17:00

#### 工作内容

**步骤 1: 测试下载流程**
- 输入 URL 开始下载
- 验证进度更新
- 测试暂停/恢复
- 测试取消

**步骤 2: 测试断点续传**
- 开始下载后关闭应用
- 重新打开应用
- 验证能恢复下载

**步骤 3: 测试雷达功能**
- 输入 URL 扫描
- 验证资源列表
- 选择资源下载

**步骤 4: 测试错误场景**
- 无效 URL 处理
- 网络断开恢复
- 磁盘空间不足

#### 验收标准
- [ ] 所有核心功能通过测试
- [ ] 测试报告完整
- [ ] 无明显 Bug

#### 产出文件
```
tests/
└── e2e_test_report.md
```

---

## P6: 应用打包发布

### 阶段目标
构建各平台安装包，准备发布

---

### T6.2.1: macOS 构建

**任务ID**: T6.2.1
**负责人**: 主程序 (lead-developer)
**前置依赖**: P5 完成
**预计耗时**: 2h

#### 工作内容
1. 执行 `cargo tauri build --target x86_64-apple-darwin`
2. 验证 .app bundle 生成
3. 测试在 macOS 运行
4. 检查签名（如需要）

#### 验收标准
- [ ] `TurboDownload.app` 生成成功
- [ ] 能在 macOS 正常打开
- [ ] 功能正常

---

### T6.2.2: Windows 构建

**任务ID**: T6.2.2
**负责人**: 开发员 (developer)
**前置依赖**: P5 完成
**预计耗时**: 2h

#### 工作内容
1. 配置 Windows 交叉编译或使用 CI
2. 执行 `cargo tauri build --target x86_64-pc-windows-msvc`
3. 验证 .msi 安装包生成
4. 测试安装流程

#### 验收标准
- [ ] `TurboDownload_1.0.0_x64.msi` 生成成功
- [ ] 能在 Windows 安装
- [ ] 功能正常

---

### T6.2.3: Linux 构建

**任务ID**: T6.2.3
**负责人**: 开发员 (developer)
**前置依赖**: P5 完成
**预计耗时**: 1.5h

#### 工作内容
1. 使用 Docker 交叉编译
2. 执行 `cargo tauri build --target x86_64-unknown-linux-gnu`
3. 验证 .deb 包生成

#### 验收标准
- [ ] `turbo-download_1.0.0_amd64.deb` 生成成功
- [ ] 能在 Ubuntu 安装运行

---

## 执行计划

```
Day 1 (2026-03-30):
  T5.1 (开发员) ───────┐
                       ├──-> T5.3 (开发员)
  T5.2 (主程序) ───────┘

Day 2 (2026-03-31):
  T6.2.1 (主程序) ─────┐
  T6.2.2 (开发员) ─────┼──-> 发布
  T6.2.3 (开发员) ─────┘
```

---

## 验证流程

每个子任务必须执行：

```
子任务开始 -> 执行 -> 自我验证 -> 提交报告 -> 互相验证 -> 通过 -> 下一任务
                              ↓未通过
                           打回改进
```

### 验证分工

| 验证类型 | 执行者 | 内容 |
|----------|--------|------|
| 自我验证 | 执行者 | 代码检查、构建测试、功能自测 |
| 互相验证 | 主程序 ↔ 开发员 | 代码审查、功能验证 |
| 技术验证 | 架构师 | 架构检查、性能评估 |
| 最终验证 | 总管 | 整体验收 |

---

## 风险预案

| 风险 | 概率 | 影响 | 应对 |
|------|------|------|------|
| P4 UI 复制后路径问题 | 中 | 中 | 提前检查所有 import 路径 |
| Rust 库依赖冲突 | 低 | 高 | 使用 workspace 统一管理 |
| 交叉编译失败 | 中 | 中 | 使用 GitHub Actions CI |
| 前端构建失败 | 中 | 中 | 检查 Node 版本和依赖 |

---

**任务链设计完成！等待总管确认后启动开发。**
