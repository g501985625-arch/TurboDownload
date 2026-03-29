# P5: turbo-integration 任务链设计

## 概述

本文档按乐高式任务链设计，每个子任务细化到1-2小时完成。

---

## 任务块概览

| 任务块 | 描述 | 预估工时 | 依赖 |
|--------|------|----------|------|
| T5.1 | 项目初始化 | 2h | P1, P2, P4 |
| T5.2 | 下载命令封装 | 5h | T5.1 |
| T5.3 | 爬虫命令封装 | 3h | T5.1 |
| T5.4 | 文件系统操作 | 3h | T5.1 |
| T5.5 | 配置管理 | 3h | T5.1 |
| T5.6 | 系统通知 | 2h | T5.1 |
| T5.7 | 事件系统 | 2h | T5.2, T5.3 |
| **总计** | | **20h** | |

---

## T5.1: 项目初始化

### T5.1.1: 创建项目结构 (30分钟)

**文件路径**: `crates/turbo-integration/`

**步骤**:
1. 运行 `cargo init --lib`
2. 创建目录结构:
   - `src/commands/`
   - `src/events/`
   - `src/config/`
   - `src/fs/`
   - `src/notification/`
3. 创建模块入口文件

**验收标准**:
- [ ] 目录结构符合规范
- [ ] `cargo check` 通过

---

### T5.1.2: 配置 Cargo.toml (30分钟)

**文件路径**: `crates/turbo-integration/Cargo.toml`

**步骤**:
1. 配置 package 信息
2. 添加内部依赖
3. 添加外部依赖
4. 配置 features

**输出**: Cargo.toml 配置完成

**验收标准**:
- [ ] 依赖版本正确
- [ ] `cargo build` 成功

---

### T5.1.3: 创建错误类型 (1小时)

**文件路径**: `src/error.rs`

**函数/类型**:
```rust
pub enum IntegrationError {
    CommandError(String),
    ConfigError(String),
    FileSystemError(String),
    NotificationError(String),
    InternalError(String),
}
```

**验收标准**:
- [ ] 实现 `std::error::Error`
- [ ] 实现 `From` 转换
- [ ] 单元测试通过

---

### T5.1.4: 创建 lib.rs 入口 (30分钟)

**文件路径**: `src/lib.rs`

**步骤**:
1. 声明所有模块
2. 导出公开接口
3. 添加文档注释

**验收标准**:
- [ ] 模块声明正确
- [ ] 文档注释完整
- [ ] `cargo doc` 通过

---

## T5.2: 下载命令封装

### T5.2.1: 创建命令模块入口 (30分钟)

**文件路径**: `src/commands/mod.rs`

**步骤**:
1. 声明子模块
2. 导出命令函数

**验收标准**:
- [ ] 模块结构正确

---

### T5.2.2: 实现 add_download 命令 (1小时)

**文件路径**: `src/commands/download.rs`

**函数签名**:
```rust
#[tauri::command]
pub async fn add_download(
    url: String,
    config: Option<DownloadConfigJson>,
    app: AppHandle,
) -> Result<String, IntegrationError>
```

**输入**: url (String), config (Option<DownloadConfigJson>)
**输出**: task_id (String)

**步骤**:
1. 解析配置参数
2. 调用 turbo-manager 的 add_task
3. 返回任务 ID

**验收标准**:
- [ ] 命令可被 Tauri 调用
- [ ] 错误处理完整
- [ ] 单元测试通过

---

### T5.2.3: 实现 start_download 命令 (45分钟)

**文件路径**: `src/commands/download.rs`

**函数签名**:
```rust
#[tauri::command]
pub async fn start_download(
    task_id: String,
    app: AppHandle,
) -> Result<(), IntegrationError>
```

**验收标准**:
- [ ] 任务正确启动
- [ ] 错误处理完整

---

### T5.2.4: 实现 pause_download 命令 (45分钟)

**文件路径**: `src/commands/download.rs`

**函数签名**:
```rust
#[tauri::command]
pub async fn pause_download(
    task_id: String,
    app: AppHandle,
) -> Result<(), IntegrationError>
```

**验收标准**:
- [ ] 任务正确暂停
- [ ] 错误处理完整

---

### T5.2.5: 实现 resume_download 命令 (45分钟)

**文件路径**: `src/commands/download.rs`

**验收标准**:
- [ ] 任务正确恢复

---

### T5.2.6: 实现 cancel_download 命令 (45分钟)

**文件路径**: `src/commands/download.rs`

**验收标准**:
- [ ] 任务正确取消

---

### T5.2.7: 实现 get_download_progress 命令 (1小时)

**文件路径**: `src/commands/download.rs`

**函数签名**:
```rust
#[tauri::command]
pub async fn get_download_progress(
    task_id: String,
    app: AppHandle,
) -> Result<Option<DownloadProgressJson>, IntegrationError>
```

**验收标准**:
- [ ] 正确返回进度信息
- [ ] JSON 序列化正确

---

### T5.2.8: 实现 get_all_downloads 命令 (45分钟)

**文件路径**: `src/commands/download.rs`

**验收标准**:
- [ ] 返回所有任务列表

---

## T5.3: 爬虫命令封装

### T5.3.1: 创建爬虫命令模块 (30分钟)

**文件路径**: `src/commands/crawler.rs`

**验收标准**:
- [ ] 模块创建完成

---

### T5.3.2: 实现 crawl_url 命令 (1小时)

**文件签名**:
```rust
#[tauri::command]
pub async fn crawl_url(
    url: String,
    app: AppHandle,
) -> Result<Vec<ResourceJson>, IntegrationError>
```

**验收标准**:
- [ ] 正确返回资源列表
- [ ] 错误处理完整

---

### T5.3.3: 实现 scan_site 命令 (1小时)

**文件路径**: `src/commands/crawler.rs`

**验收标准**:
- [ ] 整站扫描功能正常
- [ ] 进度回调正确

---

### T5.3.4: 实现 cancel_scan 命令 (30分钟)

**验收标准**:
- [ ] 扫描正确取消

---

## T5.4: 文件系统操作

### T5.4.1: 创建文件系统模块 (30分钟)

**文件路径**: `src/fs/mod.rs`

---

### T5.4.2: 实现 select_directory 命令 (1小时)

**文件路径**: `src/fs/dialog.rs`

**函数签名**:
```rust
#[tauri::command]
pub async fn select_directory(
    app: AppHandle,
) -> Result<Option<String>, IntegrationError>
```

**步骤**:
1. 使用 tauri::api::dialog::blocking::FileDialogBuilder
2. 返回选择的目录路径

**验收标准**:
- [ ] 对话框正确弹出
- [ ] 路径正确返回

---

### T5.4.3: 实现 get_default_download_dir 命令 (45分钟)

**文件路径**: `src/fs/operations.rs`

**验收标准**:
- [ ] 返回系统默认下载目录

---

### T5.4.4: 实现 file_exists 命令 (30分钟)

**验收标准**:
- [ ] 正确检查文件存在性

---

### T5.4.5: 实现 ensure_directory 命令 (30分钟)

**验收标准**:
- [ ] 目录正确创建

---

## T5.5: 配置管理

### T5.5.1: 创建配置模块 (30分钟)

**文件路径**: `src/config/mod.rs`

---

### T5.5.2: 定义配置类型 (1小时)

**文件路径**: `src/config/types.rs`

**类型定义**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub download_dir: PathBuf,
    pub max_concurrent_downloads: usize,
    pub default_threads: usize,
    pub default_chunk_size: u64,
    pub speed_limit: u64,
    pub notifications: NotificationConfig,
    pub crawler: CrawlerConfig,
}
```

**验收标准**:
- [ ] 序列化/反序列化测试通过

---

### T5.5.3: 实现配置管理器 (1.5小时)

**文件路径**: `src/config/manager.rs`

**函数**:
- `load() -> Result<AppConfig, IntegrationError>`
- `save(config: &AppConfig) -> Result<(), IntegrationError>`
- `get_config_path() -> PathBuf`

**验收标准**:
- [ ] 配置文件正确读写
- [ ] 默认配置生成

---

### T5.5.4: 实现 get_config/save_config 命令 (1小时)

**文件路径**: `src/commands/system.rs`

**验收标准**:
- [ ] Tauri 命令正确实现

---

## T5.6: 系统通知

### T5.6.1: 创建通知模块 (30分钟)

**文件路径**: `src/notification/mod.rs`

---

### T5.6.2: 实现 show_notification 命令 (1小时)

**文件路径**: `src/notification/notify.rs`

**函数签名**:
```rust
#[tauri::command]
pub async fn show_notification(
    title: String,
    body: String,
    app: AppHandle,
) -> Result<(), IntegrationError>
```

**验收标准**:
- [ ] 通知正确显示
- [ ] 权限处理正确

---

### T5.6.3: 实现通知权限检查 (30分钟)

**验收标准**:
- [ ] 权限状态正确返回

---

## T5.7: 事件系统

### T5.7.1: 创建事件模块 (30分钟)

**文件路径**: `src/events/mod.rs`

---

### T5.7.2: 定义事件常量 (30分钟)

**文件路径**: `src/events/emitter.rs`

**事件列表**:
- `download:progress`
- `download:completed`
- `download:failed`
- `task:state_changed`
- `crawl:progress`
- `crawl:completed`

**验收标准**:
- [ ] 事件名称正确定义

---

### T5.7.3: 实现 EventEmitter trait (1小时)

**函数**:
```rust
pub fn emit_progress(app: &AppHandle, progress: DownloadProgress)
pub fn emit_completed(app: &AppHandle, result: DownloadResult)
pub fn emit_failed(app: &AppHandle, task_id: &str, error: &str)
```

**验收标准**:
- [ ] 事件正确发送到前端
- [ ] JSON payload 正确

---

## 验收汇总

| 任务块 | 子任务数 | 预估工时 | 验收标准 |
|--------|----------|----------|----------|
| T5.1 | 4 | 2.5h | 项目可编译 |
| T5.2 | 8 | 5.5h | 下载命令全部可用 |
| T5.3 | 4 | 3h | 爬虫命令全部可用 |
| T5.4 | 5 | 3h | 文件操作全部可用 |
| T5.5 | 4 | 3.5h | 配置读写正常 |
| T5.6 | 3 | 2h | 通知功能正常 |
| T5.7 | 3 | 2h | 事件发送正常 |

---

## 依赖关系图

```
T5.1 (项目初始化)
├── T5.2 (下载命令) ──┐
├── T5.3 (爬虫命令) ──┼── T5.7 (事件系统)
├── T5.4 (文件系统)  │
├── T5.5 (配置管理)  │
└── T5.6 (系统通知) ─┘
```