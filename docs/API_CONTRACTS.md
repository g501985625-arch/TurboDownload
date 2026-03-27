# TurboDownload 模块接口契约

## 概述

本文档定义了 TurboDownload 项目群各模块间的接口契约（API Contracts）。所有模块必须严格遵守这些契约，确保模块间的解耦和互操作性。

## 模块依赖关系

```
turbo-app (P6)
    ├── turbo-ui (P3)
    ├── turbo-integration (P5)
    │   ├── turbo-manager (P4)
    │   │   └── turbo-downloader (P1)
    │   └── turbo-crawler (P2)
    │       └── turbo-downloader (P1)
    └── tauri
```

---

## P1: turbo-downloader (核心下载引擎)

### 模块信息
- **类型**: Rust crate (lib)
- **依赖**: tokio, reqwest, sha2, serde

### 公开接口

#### 1.1 核心数据结构

```rust
/// 下载任务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    /// 任务唯一标识
    pub id: String,
    /// 下载 URL
    pub url: String,
    /// 输出文件路径
    pub output_path: PathBuf,
    /// 并发线程数
    pub threads: usize,
    /// 分片大小 (字节)，0 表示自动
    pub chunk_size: u64,
    /// 是否支持断点续传
    pub resume_support: bool,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 请求头
    pub headers: HashMap<String, String>,
    /// 速度限制 (字节/秒)，0 表示不限制
    pub speed_limit: u64,
}

/// 下载状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DownloadState {
    /// 等待中
    Pending,
    /// 准备中 (获取文件信息)
    Preparing,
    /// 下载中
    Downloading,
    /// 已暂停
    Paused,
    /// 已完成
    Completed,
    /// 失败
    Failed(String),
    /// 已取消
    Cancelled,
}

/// 下载进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    /// 任务 ID
    pub task_id: String,
    /// 已下载字节数
    pub downloaded: u64,
    /// 总字节数 (可能为 0，如果服务器不支持)
    pub total: u64,
    /// 下载速度 (字节/秒)
    pub speed: u64,
    /// 预估剩余时间 (秒)
    pub eta: Option<u64>,
    /// 当前状态
    pub state: DownloadState,
    /// 分片进度
    pub chunks: Vec<ChunkProgress>,
}

/// 分片进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkProgress {
    /// 分片索引
    pub index: usize,
    /// 起始字节
    pub start: u64,
    /// 结束字节
    pub end: u64,
    /// 已下载字节数
    pub downloaded: u64,
    /// 是否完成
    pub completed: bool,
}

/// 下载结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    /// 任务 ID
    pub task_id: String,
    /// 输出文件路径
    pub output_path: PathBuf,
    /// 文件大小
    pub file_size: u64,
    /// 下载耗时 (毫秒)
    pub duration_ms: u64,
    /// 平均速度 (字节/秒)
    pub avg_speed: u64,
}

/// 进度回调类型
pub type ProgressCallback = Box<dyn Fn(DownloadProgress) + Send + Sync>;
```

#### 1.2 Downloader Trait

```rust
/// 下载器接口
#[async_trait]
pub trait Downloader: Send + Sync {
    /// 创建新的下载任务
    async fn create_task(&self, config: DownloadConfig) -> Result<String, DownloadError>;
    
    /// 开始下载
    async fn start(&self, task_id: &str, callback: Option<ProgressCallback>) -> Result<(), DownloadError>;
    
    /// 暂停下载
    async fn pause(&self, task_id: &str) -> Result<(), DownloadError>;
    
    /// 恢复下载
    async fn resume(&self, task_id: &str, callback: Option<ProgressCallback>) -> Result<(), DownloadError>;
    
    /// 取消下载
    async fn cancel(&self, task_id: &str) -> Result<(), DownloadError>;
    
    /// 获取下载进度
    async fn get_progress(&self, task_id: &str) -> Result<Option<DownloadProgress>, DownloadError>;
    
    /// 获取所有任务 ID
    async fn list_tasks(&self) -> Result<Vec<String>, DownloadError>;
    
    /// 移除任务记录
    async fn remove_task(&self, task_id: &str) -> Result<(), DownloadError>;
}
```

#### 1.3 错误类型

```rust
/// 下载错误类型
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("File already exists: {0}")]
    FileExists(PathBuf),
    
    #[error("Server error: {0}")]
    ServerError(u16, String),
    
    #[error("Resume not supported")]
    ResumeNotSupported,
    
    #[error("Task already running: {0}")]
    TaskAlreadyRunning(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}
```

#### 1.4 工厂方法

```rust
/// 下载器构建器
pub struct DownloaderBuilder {
    max_concurrent_tasks: usize,
    default_threads: usize,
    default_chunk_size: u64,
    temp_dir: PathBuf,
}

impl DownloaderBuilder {
    pub fn new() -> Self;
    pub fn max_concurrent_tasks(mut self, max: usize) -> Self;
    pub fn default_threads(mut self, threads: usize) -> Self;
    pub fn default_chunk_size(mut self, size: u64) -> Self;
    pub fn temp_dir(mut self, dir: PathBuf) -> Self;
    pub fn build(self) -> Result<Arc<dyn Downloader>, DownloadError>;
}
```

---

## P2: turbo-crawler (资源抓取服务)

### 模块信息
- **类型**: Rust crate (lib)
- **依赖**: turbo-downloader, scraper, url, regex

### 公开接口

#### 2.1 核心数据结构

```rust
/// 资源类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Script,
    Stylesheet,
    Font,
    Other(String),
}

/// 抓取到的资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// 资源 URL
    pub url: String,
    /// 资源类型
    pub resource_type: ResourceType,
    /// 文件名 (从 URL 解析)
    pub filename: Option<String>,
    /// 文件大小 (如果可获取)
    pub size: Option<u64>,
    /// MIME 类型
    pub mime_type: Option<String>,
    /// 来源页面
    pub source_url: String,
    /// 是否可下载
    pub downloadable: bool,
}

/// 扫描配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// 最大深度
    pub max_depth: usize,
    /// 最大页面数
    pub max_pages: usize,
    /// 资源类型过滤
    pub resource_types: Option<Vec<ResourceType>>,
    /// URL 模式过滤 (正则)
    pub url_patterns: Vec<String>,
    /// 是否跨域
    pub cross_domain: bool,
    /// 请求超时 (秒)
    pub timeout: u64,
    /// 用户代理
    pub user_agent: Option<String>,
}

/// 扫描结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// 扫描的 URL
    pub url: String,
    /// 发现的资源
    pub resources: Vec<Resource>,
    /// 扫描的页面数
    pub pages_scanned: usize,
    /// 扫描耗时 (毫秒)
    pub duration_ms: u64,
    /// 错误信息
    pub errors: Vec<String>,
}

/// 爬取进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlProgress {
    /// 当前 URL
    pub current_url: String,
    /// 已扫描页面数
    pub pages_scanned: usize,
    /// 已发现资源数
    pub resources_found: usize,
    /// 当前深度
    pub current_depth: usize,
    /// 状态
    pub status: CrawlStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrawlStatus {
    Running,
    Completed,
    Cancelled,
    Failed(String),
}
```

#### 2.2 Crawler Trait

```rust
/// 爬虫接口
#[async_trait]
pub trait Crawler: Send + Sync {
    /// 单页面爬取
    async fn crawl(&self, url: &str) -> Result<Vec<Resource>, CrawlerError>;
    
    /// 整站扫描
    async fn scan_site(
        &self,
        url: &str,
        config: ScanConfig,
        callback: Option<Box<dyn Fn(CrawlProgress) + Send + Sync>>,
    ) -> Result<ScanResult, CrawlerError>;
    
    /// 取消扫描
    async fn cancel_scan(&self) -> Result<(), CrawlerError>;
    
    /// 获取资源信息
    async fn get_resource_info(&self, url: &str) -> Result<Resource, CrawlerError>;
}

/// 资源分类器接口
pub trait ResourceClassifier: Send + Sync {
    /// 分类资源类型
    fn classify(&self, url: &str, mime_type: Option<&str>) -> ResourceType;
    
    /// 判断是否可下载
    fn is_downloadable(&self, resource_type: &ResourceType) -> bool;
}
```

#### 2.3 错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum CrawlerError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Cancelled")]
    Cancelled,
    
    #[error("Rate limited")]
    RateLimited,
    
    #[error("Internal error: {0}")]
    Internal(String),
}
```

#### 2.4 工厂方法

```rust
pub struct CrawlerBuilder {
    user_agent: String,
    timeout: Duration,
    max_redirects: usize,
}

impl CrawlerBuilder {
    pub fn new() -> Self;
    pub fn user_agent(mut self, agent: String) -> Self;
    pub fn timeout(mut self, timeout: Duration) -> Self;
    pub fn max_redirects(mut self, max: usize) -> Self;
    pub fn build(self) -> Result<Arc<dyn Crawler>, CrawlerError>;
}
```

---

## P4: turbo-manager (下载管理器)

### 模块信息
- **类型**: Rust crate (lib)
- **依赖**: turbo-downloader, tokio, serde

### 公开接口

#### 4.1 核心数据结构

```rust
/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    /// 任务 ID
    pub id: String,
    /// 任务配置
    pub config: DownloadConfig,
    /// 当前状态
    pub state: TaskState,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 进度信息
    pub progress: Option<DownloadProgress>,
}

/// 任务状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskState {
    Queued,
    Preparing,
    Downloading,
    Paused,
    Completed,
    Failed(String),
    Cancelled,
}

/// 管理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerConfig {
    /// 最大并发任务数
    pub max_concurrent: usize,
    /// 数据库路径
    pub db_path: PathBuf,
    /// 临时文件目录
    pub temp_dir: PathBuf,
    /// 自动重试次数
    pub auto_retry: usize,
    /// 重试间隔 (毫秒)
    pub retry_interval: u64,
}

/// 管理器事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManagerEvent {
    TaskAdded(String),
    TaskStarted(String),
    TaskProgress(String, DownloadProgress),
    TaskPaused(String),
    TaskResumed(String),
    TaskCompleted(String, DownloadResult),
    TaskFailed(String, String),
    TaskCancelled(String),
    TaskRemoved(String),
}
```

#### 4.2 DownloadManager Trait

```rust
/// 下载管理器接口
#[async_trait]
pub trait DownloadManager: Send + Sync {
    /// 添加任务
    async fn add_task(&self, config: DownloadConfig) -> Result<String, ManagerError>;
    
    /// 开始任务
    async fn start_task(&self, task_id: &str) -> Result<(), ManagerError>;
    
    /// 暂停任务
    async fn pause_task(&self, task_id: &str) -> Result<(), ManagerError>;
    
    /// 恢复任务
    async fn resume_task(&self, task_id: &str) -> Result<(), ManagerError>;
    
    /// 取消任务
    async fn cancel_task(&self, task_id: &str) -> Result<(), ManagerError>;
    
    /// 移除任务
    async fn remove_task(&self, task_id: &str) -> Result<(), ManagerError>;
    
    /// 获取任务信息
    async fn get_task(&self, task_id: &str) -> Result<Option<TaskInfo>, ManagerError>;
    
    /// 获取所有任务
    async fn list_tasks(&self) -> Result<Vec<TaskInfo>, ManagerError>;
    
    /// 获取任务进度
    async fn get_progress(&self, task_id: &str) -> Result<Option<DownloadProgress>, ManagerError>;
    
    /// 订阅事件
    async fn subscribe(&self) -> Receiver<ManagerEvent>;
    
    /// 启动管理器
    async fn start(&self) -> Result<(), ManagerError>;
    
    /// 停止管理器
    async fn stop(&self) -> Result<(), ManagerError>;
}
```

#### 4.3 持久化接口

```rust
/// 任务存储接口
#[async_trait]
pub trait TaskStorage: Send + Sync {
    /// 保存任务
    async fn save_task(&self, task: &TaskInfo) -> Result<(), StorageError>;
    
    /// 加载任务
    async fn load_task(&self, id: &str) -> Result<Option<TaskInfo>, StorageError>;
    
    /// 加载所有任务
    async fn load_all_tasks(&self) -> Result<Vec<TaskInfo>, StorageError>;
    
    /// 删除任务
    async fn delete_task(&self, id: &str) -> Result<(), StorageError>;
    
    /// 更新任务状态
    async fn update_state(&self, id: &str, state: TaskState) -> Result<(), StorageError>;
}
```

---

## P5: turbo-integration (系统集成层)

### 模块信息
- **类型**: Rust crate (lib)
- **依赖**: turbo-manager, turbo-crawler, tauri

### 公开接口

#### 5.1 Tauri 命令

```rust
// === 下载命令 ===

/// 添加下载任务
#[tauri::command]
pub async fn add_download(
    url: String,
    config: Option<DownloadConfigJson>,
    app: AppHandle,
) -> Result<String, IntegrationError>;

/// 开始下载
#[tauri::command]
pub async fn start_download(
    task_id: String,
    app: AppHandle,
) -> Result<(), IntegrationError>;

/// 暂停下载
#[tauri::command]
pub async fn pause_download(
    task_id: String,
    app: AppHandle,
) -> Result<(), IntegrationError>;

/// 恢复下载
#[tauri::command]
pub async fn resume_download(
    task_id: String,
    app: AppHandle,
) -> Result<(), IntegrationError>;

/// 取消下载
#[tauri::command]
pub async fn cancel_download(
    task_id: String,
    app: AppHandle,
) -> Result<(), IntegrationError>;

/// 移除下载任务
#[tauri::command]
pub async fn remove_download(
    task_id: String,
    app: AppHandle,
) -> Result<(), IntegrationError>;

/// 获取下载进度
#[tauri::command]
pub async fn get_download_progress(
    task_id: String,
    app: AppHandle,
) -> Result<Option<DownloadProgressJson>, IntegrationError>;

/// 获取所有下载任务
#[tauri::command]
pub async fn get_all_downloads(
    app: AppHandle,
) -> Result<Vec<TaskInfoJson>, IntegrationError>;

// === 爬虫命令 ===

/// 爬取 URL
#[tauri::command]
pub async fn crawl_url(
    url: String,
    app: AppHandle,
) -> Result<Vec<ResourceJson>, IntegrationError>;

/// 扫描站点
#[tauri::command]
pub async fn scan_site(
    url: String,
    config: ScanConfigJson,
    app: AppHandle,
) -> Result<ScanResultJson, IntegrationError>;

/// 取消扫描
#[tauri::command]
pub async fn cancel_scan(
    app: AppHandle,
) -> Result<(), IntegrationError>;

// === 系统命令 ===

/// 选择目录
#[tauri::command]
pub async fn select_directory(
    app: AppHandle,
) -> Result<Option<String>, IntegrationError>;

/// 获取默认下载目录
#[tauri::command]
pub async fn get_default_download_dir() -> Result<String, IntegrationError>;

/// 检查文件是否存在
#[tauri::command]
pub async fn file_exists(
    path: String,
) -> Result<bool, IntegrationError>;

/// 显示通知
#[tauri::command]
pub async fn show_notification(
    title: String,
    body: String,
    app: AppHandle,
) -> Result<(), IntegrationError>;

/// 获取配置
#[tauri::command]
pub async fn get_config(
    app: AppHandle,
) -> Result<AppConfigJson, IntegrationError>;

/// 保存配置
#[tauri::command]
pub async fn save_config(
    config: AppConfigJson,
    app: AppHandle,
) -> Result<(), IntegrationError>;
```

#### 5.2 事件系统

```rust
/// 前端事件
pub mod events {
    /// 下载进度更新
    pub const DOWNLOAD_PROGRESS: &str = "download:progress";
    /// 下载完成
    pub const DOWNLOAD_COMPLETED: &str = "download:completed";
    /// 下载失败
    pub const DOWNLOAD_FAILED: &str = "download:failed";
    /// 任务状态变更
    pub const TASK_STATE_CHANGED: &str = "task:state_changed";
    /// 爬取进度
    pub const CRAWL_PROGRESS: &str = "crawl:progress";
    /// 爬取完成
    pub const CRAWL_COMPLETED: &str = "crawl:completed";
}

/// 事件发送器
pub trait EventEmitter: Send + Sync {
    fn emit<T: Serialize + Clone>(&self, event: &str, payload: T) -> Result<(), IntegrationError>;
}
```

#### 5.3 配置管理

```rust
/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 默认下载目录
    pub download_dir: PathBuf,
    /// 最大并发任务数
    pub max_concurrent_downloads: usize,
    /// 默认线程数
    pub default_threads: usize,
    /// 默认分片大小
    pub default_chunk_size: u64,
    /// 速度限制
    pub speed_limit: u64,
    /// 通知设置
    pub notifications: NotificationConfig,
    /// 爬虫设置
    pub crawler: CrawlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub on_complete: bool,
    pub on_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerConfig {
    pub timeout: u64,
    pub max_depth: usize,
    pub max_pages: usize,
    pub user_agent: String,
}
```

---

## P3: turbo-ui (前端 UI 框架)

### 模块信息
- **类型**: npm package
- **依赖**: React, lucide-react, zustand

### 公开接口

#### 3.1 组件导出

```typescript
// === 基础组件 ===
export { Button } from './components/Button';
export { Modal } from './components/Modal';
export { Input } from './components/Input';
export { Progress } from './components/Progress';
export { Select } from './components/Select';
export { Tabs } from './components/Tabs';
export { Toast } from './components/Toast';

// === 业务组件 ===
export { DownloadList } from './components/DownloadList';
export { DownloadItem } from './components/DownloadItem';
export { AddDownloadModal } from './components/AddDownloadModal';
export { CrawlerPanel } from './components/CrawlerPanel';
export { SettingsPanel } from './components/SettingsPanel';
export { TaskQueue } from './components/TaskQueue';

// === 类型导出 ===
export type {
  DownloadTask,
  DownloadProgress,
  TaskState,
  Resource,
  ResourceType,
  AppConfig,
} from './types';
```

#### 3.2 类型定义

```typescript
// types/index.ts

export interface DownloadTask {
  id: string;
  url: string;
  filename: string;
  outputPath: string;
  totalSize: number;
  downloadedSize: number;
  state: TaskState;
  threads: number;
  speed: number;
  eta: number | null;
  error: string | null;
  createdAt: string;
  updatedAt: string;
}

export type TaskState = 
  | 'queued'
  | 'preparing'
  | 'downloading'
  | 'paused'
  | 'completed'
  | 'failed'
  | 'cancelled';

export interface DownloadProgress {
  taskId: string;
  downloaded: number;
  total: number;
  speed: number;
  eta: number | null;
  state: TaskState;
  chunks: ChunkProgress[];
}

export interface ChunkProgress {
  index: number;
  start: number;
  end: number;
  downloaded: number;
  completed: boolean;
}

export interface Resource {
  url: string;
  resourceType: ResourceType;
  filename: string | null;
  size: number | null;
  mimeType: string | null;
  sourceUrl: string;
  downloadable: boolean;
}

export type ResourceType =
  | 'image'
  | 'video'
  | 'audio'
  | 'document'
  | 'archive'
  | 'script'
  | 'stylesheet'
  | 'font'
  | { other: string };

export interface AppConfig {
  downloadDir: string;
  maxConcurrentDownloads: number;
  defaultThreads: number;
  defaultChunkSize: number;
  speedLimit: number;
  notifications: NotificationConfig;
  crawler: CrawlerConfig;
}
```

#### 3.3 状态管理

```typescript
// stores/downloadStore.ts

import { create } from 'zustand';

interface DownloadStore {
  tasks: Map<string, DownloadTask>;
  
  // Actions
  addTask: (task: DownloadTask) => void;
  updateTask: (id: string, updates: Partial<DownloadTask>) => void;
  removeTask: (id: string) => void;
  setTasks: (tasks: DownloadTask[]) => void;
  
  // Selectors
  getTask: (id: string) => DownloadTask | undefined;
  getActiveTasks: () => DownloadTask[];
  getCompletedTasks: () => DownloadTask[];
}

export const useDownloadStore = create<DownloadStore>((set, get) => ({
  tasks: new Map(),
  
  addTask: (task) => set((state) => {
    const newTasks = new Map(state.tasks);
    newTasks.set(task.id, task);
    return { tasks: newTasks };
  }),
  
  updateTask: (id, updates) => set((state) => {
    const task = state.tasks.get(id);
    if (!task) return state;
    const newTasks = new Map(state.tasks);
    newTasks.set(id, { ...task, ...updates });
    return { tasks: newTasks };
  }),
  
  removeTask: (id) => set((state) => {
    const newTasks = new Map(state.tasks);
    newTasks.delete(id);
    return { tasks: newTasks };
  }),
  
  setTasks: (tasks) => set(() => ({
    tasks: new Map(tasks.map(t => [t.id, t])),
  })),
  
  getTask: (id) => get().tasks.get(id),
  getActiveTasks: () => Array.from(get().tasks.values())
    .filter(t => t.state === 'downloading' || t.state === 'queued'),
  getCompletedTasks: () => Array.from(get().tasks.values())
    .filter(t => t.state === 'completed'),
}));
```

---

## 接口版本控制

### 版本策略
- 使用语义化版本 (SemVer): MAJOR.MINOR.PATCH
- MAJOR: 破坏性变更
- MINOR: 新功能，向后兼容
- PATCH: Bug 修复

### 变更流程
1. 在 API_CONTRACTS.md 中记录变更
2. 更新版本号
3. 通知所有依赖方
4. 留出迁移时间

### 废弃策略
- 标记为 `#[deprecated]` (Rust) 或 `@deprecated` (TypeScript)
- 保留至少 2 个 MINOR 版本
- 在 CHANGELOG 中说明迁移方法

---

## 测试契约

### 契约测试要求
每个模块必须提供：
1. **接口测试**: 验证 trait/接口实现
2. **Mock 实现**: 供依赖方测试使用
3. **示例代码**: 展示正确用法

### 测试数据
- 使用 fixtures 目录存放测试数据
- 包含正常、边界、异常场景

---

## 文档要求

### 内联文档
- Rust: `///` 文档注释
- TypeScript: JSDoc 注释

### API 文档
- Rust: `cargo doc`
- TypeScript: TypeDoc

### 示例代码
- 每个公开接口至少一个使用示例
- 示例可编译/可运行