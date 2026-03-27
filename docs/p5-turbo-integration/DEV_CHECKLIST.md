# P5: turbo-integration 开发检查清单

## 概述

本文档提供详细的开发步骤检查清单，每个步骤可勾选确认完成。

---

## T5.1: 项目初始化

### T5.1.1: 创建项目结构

- [ ] 创建 `crates/turbo-integration/` 目录
- [ ] 运行 `cargo init --lib`
- [ ] 创建 `src/commands/` 目录
- [ ] 创建 `src/events/` 目录
- [ ] 创建 `src/config/` 目录
- [ ] 创建 `src/fs/` 目录
- [ ] 创建 `src/notification/` 目录
- [ ] 创建 `tests/` 目录
- [ ] 验证: `cargo check` 通过

### T5.1.2: 配置 Cargo.toml

- [ ] 设置 package name: `turbo-integration`
- [ ] 设置 version: `0.1.0`
- [ ] 添加内部依赖:
  - [ ] `turbo-downloader`
  - [ ] `turbo-crawler`
  - [ ] `turbo-manager`
- [ ] 添加外部依赖:
  - [ ] `tauri` 2.0
  - [ ] `tokio`
  - [ ] `serde` / `serde_json`
  - [ ] `thiserror`
  - [ ] `tracing`
  - [ ] `directories`
  - [ ] `toml`
- [ ] 验证: `cargo build` 成功

### T5.1.3: 创建错误类型

- [ ] 创建 `src/error.rs`
- [ ] 定义 `IntegrationError` 枚举
- [ ] 实现 `std::fmt::Display`
- [ ] 实现 `std::error::Error`
- [ ] 实现 `From` 转换:
  - [ ] `From<DownloadError>`
  - [ ] `From<CrawlerError>`
  - [ ] `From<ManagerError>`
  - [ ] `From<std::io::Error>`
- [ ] 编写单元测试
- [ ] 验证: `cargo test` 通过

### T5.1.4: 创建 lib.rs 入口

- [ ] 声明 `commands` 模块
- [ ] 声明 `events` 模块
- [ ] 声明 `config` 模块
- [ ] 声明 `fs` 模块
- [ ] 声明 `notification` 模块
- [ ] 导出公开接口
- [ ] 添加 crate 文档注释
- [ ] 验证: `cargo doc` 通过

---

## T5.2: 下载命令封装

### T5.2.1: 创建命令模块入口

- [ ] 创建 `src/commands/mod.rs`
- [ ] 声明 `download` 子模块
- [ ] 声明 `crawler` 子模块
- [ ] 声明 `system` 子模块
- [ ] 导出所有命令函数

### T5.2.2: 实现 add_download 命令

- [ ] 创建 `src/commands/download.rs`
- [ ] 导入必要依赖
- [ ] 定义 `add_download` 函数
- [ ] 添加 `#[tauri::command]` 宏
- [ ] 实现参数解析
- [ ] 调用 `turbo-manager` 的 `add_task`
- [ ] 返回任务 ID
- [ ] 处理所有错误情况
- [ ] 编写单元测试

### T5.2.3: 实现 start_download 命令

- [ ] 定义函数签名
- [ ] 获取 manager 状态
- [ ] 调用 `start_task`
- [ ] 错误处理
- [ ] 单元测试

### T5.2.4: 实现 pause_download 命令

- [ ] 定义函数签名
- [ ] 调用 `pause_task`
- [ ] 错误处理
- [ ] 单元测试

### T5.2.5: 实现 resume_download 命令

- [ ] 定义函数签名
- [ ] 调用 `resume_task`
- [ ] 错误处理
- [ ] 单元测试

### T5.2.6: 实现 cancel_download 命令

- [ ] 定义函数签名
- [ ] 调用 `cancel_task`
- [ ] 错误处理
- [ ] 单元测试

### T5.2.7: 实现 get_download_progress 命令

- [ ] 定义函数签名
- [ ] 调用 `get_progress`
- [ ] JSON 序列化
- [ ] 错误处理
- [ ] 单元测试

### T5.2.8: 实现 get_all_downloads 命令

- [ ] 定义函数签名
- [ ] 调用 `list_tasks`
- [ ] JSON 序列化
- [ ] 单元测试

---

## T5.3: 爬虫命令封装

### T5.3.1: 创建爬虫命令模块

- [ ] 创建 `src/commands/crawler.rs`
- [ ] 导入必要依赖

### T5.3.2: 实现 crawl_url 命令

- [ ] 定义函数签名
- [ ] 获取 crawler 状态
- [ ] 调用 `crawl`
- [ ] JSON 序列化
- [ ] 错误处理
- [ ] 单元测试

### T5.3.3: 实现 scan_site 命令

- [ ] 定义函数签名
- [ ] 调用 `scan_site`
- [ ] 进度回调实现
- [ ] 错误处理
- [ ] 单元测试

### T5.3.4: 实现 cancel_scan 命令

- [ ] 定义函数签名
- [ ] 调用 `cancel_scan`
- [ ] 单元测试

---

## T5.4: 文件系统操作

### T5.4.1: 创建文件系统模块

- [ ] 创建 `src/fs/mod.rs`
- [ ] 创建 `src/fs/dialog.rs`
- [ ] 创建 `src/fs/operations.rs`

### T5.4.2: 实现 select_directory 命令

- [ ] 导入 `tauri::api::dialog`
- [ ] 定义函数签名
- [ ] 使用 `FileDialogBuilder`
- [ ] 返回选择结果
- [ ] 错误处理

### T5.4.3: 实现 get_default_download_dir 命令

- [ ] 使用 `directories` crate
- [ ] 返回系统下载目录
- [ ] 处理不存在情况

### T5.4.4: 实现 file_exists 命令

- [ ] 使用 `std::path::Path`
- [ ] 返回布尔值

### T5.4.5: 实现 ensure_directory 命令

- [ ] 使用 `std::fs::create_dir_all`
- [ ] 处理权限错误

---

## T5.5: 配置管理

### T5.5.1: 创建配置模块

- [ ] 创建 `src/config/mod.rs`
- [ ] 创建 `src/config/types.rs`
- [ ] 创建 `src/config/manager.rs`

### T5.5.2: 定义配置类型

- [ ] 定义 `AppConfig` 结构体
- [ ] 定义 `NotificationConfig`
- [ ] 定义 `CrawlerConfig`
- [ ] 添加 `Serialize/Deserialize`
- [ ] 实现 `Default` trait
- [ ] 单元测试序列化

### T5.5.3: 实现配置管理器

- [ ] 实现 `get_config_path()`
- [ ] 实现 `load()`
- [ ] 实现 `save()`
- [ ] 处理配置不存在情况
- [ ] 单元测试

### T5.5.4: 实现配置命令

- [ ] 实现 `get_config` 命令
- [ ] 实现 `save_config` 命令
- [ ] 在 `commands/system.rs` 中注册

---

## T5.6: 系统通知

### T5.6.1: 创建通知模块

- [ ] 创建 `src/notification/mod.rs`
- [ ] 创建 `src/notification/notify.rs`

### T5.6.2: 实现 show_notification 命令

- [ ] 导入 `tauri::notification`
- [ ] 定义函数签名
- [ ] 发送通知
- [ ] 错误处理

### T5.6.3: 实现通知权限检查

- [ ] 检查通知权限
- [ ] 请求权限（如需要）

---

## T5.7: 事件系统

### T5.7.1: 创建事件模块

- [ ] 创建 `src/events/mod.rs`
- [ ] 创建 `src/events/emitter.rs`

### T5.7.2: 定义事件常量

- [ ] 定义 `DOWNLOAD_PROGRESS`
- [ ] 定义 `DOWNLOAD_COMPLETED`
- [ ] 定义 `DOWNLOAD_FAILED`
- [ ] 定义 `TASK_STATE_CHANGED`
- [ ] 定义 `CRAWL_PROGRESS`
- [ ] 定义 `CRAWL_COMPLETED`

### T5.7.3: 实现 EventEmitter

- [ ] 实现 `emit_progress()`
- [ ] 实现 `emit_completed()`
- [ ] 实现 `emit_failed()`
- [ ] 实现 `emit_state_changed()`
- [ ] 单元测试事件发送

---

## 最终验收清单

### 编译检查

- [ ] `cargo check` 无错误
- [ ] `cargo clippy` 无警告
- [ ] `cargo fmt -- --check` 通过

### 测试检查

- [ ] `cargo test` 全部通过
- [ ] 测试覆盖率 > 70%

### 文档检查

- [ ] `cargo doc` 无警告
- [ ] 公开接口有文档注释

### 集成检查

- [ ] 可被 `turbo-app` 正确引用
- [ ] 所有命令可在 Tauri 中注册