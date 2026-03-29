# TurboDownload 详细任务分解

## 概述

本文档详细分解各项目的开发任务，精确到每个步骤，并包含工时估算和依赖关系。

---

## Project-1: turbo-downloader (核心下载引擎)

### 项目信息
| 属性 | 值 |
|------|------|
| **预估工时** | 40 人时 |
| **优先级** | P0 (最高) - 其他项目依赖 |
| **依赖项** | 无 |

---

### T1.1: 项目初始化

#### T1.1.1: 创建 Rust crate 结构
**预估**: 0.5h
**负责人**: 程序代理

**步骤**:
1. 创建 `crates/turbo-downloader/` 目录
2. 运行 `cargo init --lib`
3. 创建子模块文件:
   - `src/lib.rs` - 模块入口
   - `src/error.rs` - 错误定义
   - `src/http_client.rs` - HTTP 客户端
   - `src/chunk.rs` - 分片处理
   - `src/download.rs` - 下载核心
   - `src/progress.rs` - 进度管理
   - `src/resume.rs` - 断点续传

**验收标准**:
- [ ] `cargo check` 通过
- [ ] 目录结构符合规范

#### T1.1.2: 配置 Cargo.toml 依赖
**预估**: 0.5h
**负责人**: 程序代理

**步骤**:
1. 编辑 `Cargo.toml`，添加依赖:
```toml
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

[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.5"
tempfile = "3.8"
```

**验收标准**:
- [ ] 所有依赖版本正确
- [ ] `cargo build` 成功

#### T1.1.3: 配置开发环境
**预估**: 0.5h
**负责人**: 程序代理

**步骤**:
1. 创建 `.vscode/settings.json` (如使用 VS Code)
2. 配置 `rustfmt.toml`
3. 配置 `clippy.toml`
4. 创建测试目录结构 `tests/`

**验收标准**:
- [ ] `cargo fmt --check` 通过
- [ ] `cargo clippy` 无警告

---

### T1.2: HTTP 客户端封装

#### T1.2.1: 设计 HttpClient 结构体
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 定义 `HttpClient` 结构体
2. 定义 `HttpClientBuilder` 构建器
3. 实现超时配置
4. 实现代理支持
5. 实现 User-Agent 设置

**代码框架**:
```rust
pub struct HttpClient {
    client: reqwest::Client,
    config: HttpClientConfig,
}

pub struct HttpClientConfig {
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub user_agent: String,
    pub proxy: Option<ProxyConfig>,
}

impl HttpClient {
    pub fn builder() -> HttpClientBuilder { ... }
    pub async fn head(&self, url: &str) -> Result<HeadResponse, DownloadError>;
    pub async fn get_range(&self, url: &str, start: u64, end: u64) -> Result<Bytes, DownloadError>;
}
```

**验收标准**:
- [ ] 单元测试覆盖配置构建
- [ ] 支持自定义请求头

#### T1.2.2: 实现 GET 请求方法
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 实现 `get()` 方法
2. 添加重定向处理
3. 添加错误重试逻辑
4. 实现响应流式读取

**验收标准**:
- [ ] 测试 200/301/302/404/500 响应
- [ ] 测试重定向限制

#### T1.2.3: 实现 HEAD 请求获取文件大小
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 实现 `head()` 方法
2. 解析 `Content-Length` 头
3. 解析 `Accept-Ranges` 头
4. 解析 `ETag` 和 `Last-Modified`

**代码**:
```rust
pub struct HeadResponse {
    pub content_length: Option<u64>,
    pub accept_ranges: bool,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub content_type: Option<String>,
}

impl HttpClient {
    pub async fn head(&self, url: &str) -> Result<HeadResponse, DownloadError> {
        let resp = self.client.head(url).send().await?;
        // 解析响应头...
    }
}
```

**验收标准**:
- [ ] 正确解析各种响应头
- [ ] 处理缺失头部的情况

#### T1.2.4: 添加请求头配置
**预估**: 0.5h
**负责人**: 程序代理

**步骤**:
1. 实现自定义请求头支持
2. 实现默认请求头
3. 添加 `Range` 请求头支持

**验收标准**:
- [ ] 支持添加任意请求头
- [ ] 测试 Range 请求

---

### T1.3: 多线程下载核心

#### T1.3.1: 设计分片策略
**预估**: 2h
**负责人**: 程序代理

**步骤**:
1. 根据文件大小计算最优分片数
2. 根据线程数计算分片大小
3. 实现分片计算算法
4. 处理不支持 Range 的情况

**算法**:
```rust
pub fn calculate_chunks(file_size: u64, threads: usize, min_chunk: u64) -> Vec<ChunkRange> {
    if !supports_range || file_size < min_chunk * threads as u64 {
        // 单线程下载
        return vec![ChunkRange { start: 0, end: file_size }];
    }
    
    let chunk_size = file_size / threads as u64;
    (0..threads)
        .map(|i| ChunkRange {
            start: i as u64 * chunk_size,
            end: if i == threads - 1 { file_size } else { (i + 1) as u64 * chunk_size },
        })
        .collect()
}
```

**验收标准**:
- [ ] 边界情况测试
- [ ] 大文件测试

#### T1.3.2: 实现单分片下载
**预估**: 2h
**负责人**: 程序代理

**步骤**:
1. 实现 Range 请求下载
2. 实现流式写入
3. 实现进度回调
4. 实现错误处理

**代码框架**:
```rust
pub async fn download_chunk(
    client: &HttpClient,
    url: &str,
    range: ChunkRange,
    output: &mut File,
    callback: Option<&ProgressCallback>,
) -> Result<(), DownloadError> {
    let mut stream = client.get_range(url, range.start, range.end).await?;
    let mut downloaded = 0u64;
    
    while let Some(chunk) = stream.chunk().await? {
        output.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        
        if let Some(cb) = callback {
            cb(ChunkProgress { downloaded, ... });
        }
    }
    
    Ok(())
}
```

**验收标准**:
- [ ] 单元测试
- [ ] 支持 Range 请求
- [ ] 进度回调正常

#### T1.3.3: 实现多分片并行下载
**预估**: 3h
**负责人**: 程序代理

**步骤**:
1. 创建异步任务池
2. 实现分片并行执行
3. 实现进度汇总
4. 实现任务取消

**代码框架**:
```rust
pub async fn download_parallel(
    client: Arc<HttpClient>,
    url: String,
    chunks: Vec<ChunkRange>,
    output_path: PathBuf,
    callback: Option<ProgressCallback>,
) -> Result<(), DownloadError> {
    let file = Arc::new(Mutex::new(File::create(&output_path)?));
    let cancel_flag = Arc::new(AtomicBool::new(false));
    
    let handles: Vec<_> = chunks.into_iter().map(|chunk| {
        let client = client.clone();
        let file = file.clone();
        let cancel = cancel_flag.clone();
        
        tokio::spawn(async move {
            if cancel.load(Ordering::Relaxed) {
                return Err(DownloadError::Cancelled);
            }
            download_chunk(&client, &url, chunk, file).await
        })
    }).collect();
    
    // 等待所有任务完成
    let results = futures::future::join_all(handles).await;
    // 处理结果...
}
```

**验收标准**:
- [ ] 多线程测试
- [ ] 并发安全
- [ ] 取消功能正常

#### T1.3.4: 实现分片合并
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 设计临时文件命名
2. 实现分片文件管理
3. 实现合并逻辑
4. 实现清理逻辑

**验收标准**:
- [ ] 合并正确性测试
- [ ] 清理完整性测试

---

### T1.4: 断点续传

#### T1.4.1: 设计状态持久化格式
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 设计 JSON 状态格式
2. 设计文件存储位置
3. 定义状态版本

**状态格式**:
```json
{
  "version": 1,
  "task_id": "uuid",
  "url": "https://...",
  "output_path": "/path/to/file",
  "file_size": 104857600,
  "chunks": [
    { "index": 0, "start": 0, "end": 52428800, "downloaded": 10485760 },
    { "index": 1, "start": 52428800, "end": 104857600, "downloaded": 0 }
  ],
  "etag": "\"abc123\"",
  "last_modified": "Wed, 25 Mar 2026 10:00:00 GMT",
  "created_at": "2026-03-25T10:00:00Z",
  "updated_at": "2026-03-25T11:30:00Z"
}
```

**验收标准**:
- [ ] 格式可扩展
- [ ] 序列化/反序列化测试

#### T1.4.2: 实现状态保存
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 实现状态序列化
2. 实现原子写入
3. 实现定期保存

**验收标准**:
- [ ] 状态保存测试
- [ ] 原子性测试

#### T1.4.3: 实现状态恢复
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 检测未完成任务
2. 加载状态文件
3. 验证文件一致性
4. 恢复下载进度

**验收标准**:
- [ ] 状态恢复测试
- [ ] 异常状态处理

#### T1.4.4: 实现断点下载
**预估**: 2h
**负责人**: 程序代理

**步骤**:
1. 检查服务器支持
2. 验证文件未变更
3. 计算未下载分片
4. 从断点继续下载

**验收标准**:
- [ ] 断点续传测试
- [ ] 文件变更检测

---

### T1.5: 进度回调

#### T1.5.1: 设计进度回调接口
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 定义回调 trait
2. 定义事件类型
3. 定义频率控制

**验收标准**:
- [ ] 接口设计评审
- [ ] 文档完整

#### T1.5.2: 实现实时进度上报
**预估**: 2h
**负责人**: 程序代理

**步骤**:
1. 实现进度收集
2. 实现进度汇总
3. 实现回调触发

**验收标准**:
- [ ] 实时性测试
- [ ] 频率限制测试

#### T1.5.3: 实现速度计算
**预估**: 1h
**负责人**: 程序代理

**步骤**:
1. 实现滑动窗口计算
2. 实现平均速度计算
3. 实现 ETA 估算

**验收标准**:
- [ ] 速度计算准确
- [ ] ETA 合理

---

### T1.6: 测试与优化

#### T1.6.1: 编写单元测试
**预估**: 3h
**负责人**: 程序代理

**步骤**:
1. 编写 HTTP 客户端测试
2. 编写分片计算测试
3. 编写下载测试 (使用 mock)
4. 编写断点续传测试

**测试清单**:
- [ ] `tests/http_client_test.rs`
- [ ] `tests/chunk_test.rs`
- [ ] `tests/download_test.rs`
- [ ] `tests/resume_test.rs`

**覆盖率要求**: > 80%

#### T1.6.2: 性能测试
**预估**: 2h
**负责人**: 程序代理

**步骤**:
1. 编写基准测试
2. 测试大文件下载
3. 测试高并发场景
4. 分析性能瓶颈

**验收标准**:
- [ ] 基准测试结果
- [ ] 性能报告

#### T1.6.3: 优化内存使用
**预估**: 2h
**负责人**: 程序代理

**步骤**:
1. 分析内存使用
2. 优化缓冲区大小
3. 实现流式处理
4. 内存泄漏检查

**验收标准**:
- [ ] 内存使用稳定
- [ ] 无内存泄漏

---

## Project-2: turbo-crawler (资源抓取服务)

### 项目信息
| 属性 | 值 |
|------|------|
| **预估工时** | 30 人时 |
| **优先级** | P1 - 依赖 P1 |
| **依赖项** | P1: turbo-downloader |

---

### T2.1: 项目初始化 (同 P1，略)

**预估**: 1h

### T2.2: HTML 解析器

#### T2.2.1: 设计解析器结构
**预估**: 1h

**步骤**:
1. 引入 `scraper` crate
2. 定义 `HtmlParser` 结构体
3. 定义解析配置

#### T2.2.2: 实现标签提取
**预估**: 2h

**步骤**:
1. 提取 `<a>` 标签
2. 提取 `<img>` 标签
3. 提取 `<video>` / `<source>` 标签
4. 提取 `<link>` 标签
5. 提取 `<script>` 标签

#### T2.2.3: 实现属性解析
**预估**: 1h

**步骤**:
1. 解析 `href` 属性
2. 解析 `src` 属性
3. 解析 `srcset` 属性
4. 处理相对路径

### T2.3: URL 提取器

#### T2.3.1: 设计 URL 规范化
**预估**: 1h

**步骤**:
1. 解析相对 URL
2. 解析绝对 URL
3. 处理 URL 编码
4. 处理锚点

#### T2.3.2: 实现 URL 过滤
**预估**: 1h

**步骤**:
1. 同域名过滤
2. 正则匹配过滤
3. 黑名单过滤

#### T2.3.3: 实现 URL 去重
**预估**: 1h

**步骤**:
1. 规范化 URL
2. 使用 HashSet 去重
3. 实现持久化去重

### T2.4: 资源分类器

#### T2.4.1: 实现类型识别
**预估**: 2h

**步骤**:
1. 根据扩展名识别
2. 根据 MIME 类型识别
3. 根据 URL 模式识别

#### T2.4.2: 实现可下载判断
**预估**: 1h

**步骤**:
1. 判断文件大小
2. 判断访问权限
3. 判断内容类型

### T2.5: 整站扫描器

#### T2.5.1: 实现广度优先爬取
**预估**: 3h

**步骤**:
1. 实现队列管理
2. 实现并发控制
3. 实现深度限制

#### T2.5.2: 实现进度回调
**预估**: 1h

**步骤**:
1. 定义进度事件
2. 实现回调触发
3. 实现取消功能

#### T2.5.3: 实现错误处理
**预估**: 1h

**步骤**:
1. 网络错误处理
2. 解析错误处理
3. 超时处理

### T2.6: 测试与优化

**预估**: 3h

**测试清单**:
- [ ] HTML 解析测试
- [ ] URL 提取测试
- [ ] 资源分类测试
- [ ] 整站扫描测试

---

## Project-3: turbo-ui (前端 UI 框架)

### 项目信息
| 属性 | 值 |
|------|------|
| **预估工时** | 25 人时 |
| **优先级** | P1 - 可并行开发 |
| **依赖项** | 无 (使用 mock 数据) |

---

### T3.1: 项目初始化

#### T3.1.1: 创建 npm package
**预估**: 0.5h

**步骤**:
1. 创建 `packages/turbo-ui/` 目录
2. 初始化 `package.json`
3. 配置 `tsconfig.json`
4. 配置 `vite.config.ts`

#### T3.1.2: 配置依赖
**预估**: 0.5h

```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "zustand": "^4.4.0",
    "lucide-react": "^0.300.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0"
  },
  "devDependencies": {
    "typescript": "^5.3.0",
    "tailwindcss": "^3.4.0",
    "vitest": "^1.0.0",
    "@testing-library/react": "^14.0.0"
  }
}
```

### T3.2: 基础组件库

#### T3.2.1: Button 组件
**预估**: 1h

**步骤**:
1. 定义 props 接口
2. 实现样式变体
3. 实现加载状态
4. 编写测试

#### T3.2.2: Modal 组件
**预估**: 1.5h

**步骤**:
1. 实现弹窗逻辑
2. 实现遮罩层
3. 实现动画效果
4. 实现键盘关闭

#### T3.2.3: Input 组件
**预估**: 1h

**步骤**:
1. 实现输入框
2. 实现前缀/后缀
3. 实现验证状态
4. 实现清除按钮

#### T3.2.4: Progress 组件
**预估**: 1h

**步骤**:
1. 实现进度条
2. 实现动画效果
3. 实现多色显示
4. 实现分段显示

### T3.3: 下载列表组件

#### T3.3.1: DownloadList 组件
**预估**: 2h

**步骤**:
1. 实现列表布局
2. 实现虚拟滚动
3. 实现排序功能
4. 实现过滤功能

#### T3.3.2: DownloadItem 组件
**预估**: 2h

**步骤**:
1. 实现任务显示
2. 实现进度显示
3. 实现操作按钮
4. 实现右键菜单

#### T3.3.3: AddDownloadModal 组件
**预估**: 1.5h

**步骤**:
1. 实现 URL 输入
2. 实现配置选项
3. 实现文件名预览
4. 实现目录选择

### T3.4: 抓取面板组件

#### T3.4.1: CrawlerPanel 组件
**预估**: 2h

**步骤**:
1. 实现 URL 输入
2. 实现扫描配置
3. 实现进度显示
4. 实现结果列表

#### T3.4.2: ResourceList 组件
**预估**: 1.5h

**步骤**:
1. 实现资源列表
2. 实现类型过滤
3. 实现批量选择
4. 实现批量下载

### T3.5: 状态管理

#### T3.5.1: downloadStore
**预估**: 2h

**步骤**:
1. 定义状态结构
2. 实现 actions
3. 实现 selectors
4. 实现持久化

#### T3.5.2: crawlerStore
**预估**: 1h

**步骤**:
1. 定义状态结构
2. 实现 actions
3. 实现 selectors

#### T3.5.3: settingsStore
**预估**: 1h

**步骤**:
1. 定义配置结构
2. 实现配置读写
3. 实现默认值

### T3.6: 测试与优化

**预估**: 3h

---

## Project-4: turbo-manager (下载管理器)

### 项目信息
| 属性 | 值 |
|------|------|
| **预估工时** | 25 人时 |
| **优先级** | P1 - 依赖 P1 |
| **依赖项** | P1: turbo-downloader |

---

### T4.1: 项目初始化
**预估**: 1h

### T4.2: 任务队列设计

#### T4.2.1: 设计队列结构
**预估**: 2h

**步骤**:
1. 定义优先级队列
2. 定义任务状态
3. 实现队列操作

#### T4.2.2: 实现并发控制
**预估**: 2h

**步骤**:
1. 实现信号量
2. 实现任务调度
3. 实现等待队列

### T4.3: 状态机实现

#### T4.3.1: 设计状态转换
**预估**: 1h

**状态图**:
```
Pending → Preparing → Downloading → Completed
              ↓            ↓
           Failed ←    ← Paused
              ↓
          Cancelled
```

#### T4.3.2: 实现状态管理
**预估**: 2h

**步骤**:
1. 实现状态存储
2. 实现状态转换
3. 实现状态查询

### T4.4: 并发控制

#### T4.4.1: 实现任务调度器
**预估**: 3h

**步骤**:
1. 实现调度算法
2. 实现优先级处理
3. 实现资源分配

### T4.5: 持久化存储

#### T4.5.1: 设计存储格式
**预估**: 1h

#### T4.5.2: 实现 SQLite 存储
**预估**: 3h

**步骤**:
1. 设计数据库 schema
2. 实现 CRUD 操作
3. 实现事务处理

### T4.6: 测试与优化

**预估**: 3h

---

## Project-5: turbo-integration (系统集成层)

### 项目信息
| 属性 | 值 |
|------|------|
| **预估工时** | 20 人时 |
| **优先级** | P2 - 依赖 P1, P2, P4 |
| **依赖项** | P1, P2, P4 |

---

### T5.1: 项目初始化
**预估**: 1h

### T5.2: Tauri 命令封装

#### T5.2.1: 实现下载命令
**预估**: 3h

**步骤**:
1. 实现 `add_download`
2. 实现 `start_download`
3. 实现 `pause_download`
4. 实现 `resume_download`
5. 实现 `cancel_download`

#### T5.2.2: 实现爬虫命令
**预估**: 2h

**步骤**:
1. 实现 `crawl_url`
2. 实现 `scan_site`
3. 实现 `cancel_scan`

### T5.3: 文件系统操作

#### T5.3.1: 实现目录选择
**预估**: 1h

#### T5.3.2: 实现文件操作
**预估**: 1h

### T5.4: 系统通知

#### T5.4.1: 实现通知集成
**预估**: 1h

### T5.5: 配置管理

#### T5.5.1: 实现配置读写
**预估**: 2h

### T5.6: 测试与优化

**预估**: 3h

---

## Project-6: turbo-app (主应用集成)

### 项目信息
| 属性 | 值 |
|------|------|
| **预估工时** | 15 人时 |
| **优先级** | P2 - 依赖所有模块 |
| **依赖项** | P1, P2, P3, P4, P5 |

---

### T6.1: 项目初始化

#### T6.1.1: 创建 Tauri 应用
**预估**: 1h

**步骤**:
1. 创建 `apps/turbo-app/` 目录
2. 初始化 Tauri 项目
3. 配置依赖

#### T6.1.2: 配置前端
**预估**: 1h

### T6.2: 模块集成

#### T6.2.1: 集成 Rust crates
**预估**: 2h

**步骤**:
1. 添加 workspace 依赖
2. 配置 feature flags
3. 实现模块初始化

#### T6.2.2: 集成 UI 组件
**预估**: 1h

### T6.3: 主界面开发

#### T6.3.1: 实现主布局
**预估**: 2h

#### T6.3.2: 实现路由
**预估**: 1h

### T6.4: 端到端测试

**预估**: 3h

**测试清单**:
- [ ] 下载流程 E2E
- [ ] 爬取流程 E2E
- [ ] 设置流程 E2E

### T6.5: 打包发布

#### T6.5.1: 配置打包
**预估**: 2h

**步骤**:
1. 配置 tauri.conf.json
2. 配置签名
3. 配置更新

#### T6.5.2: 构建发布
**预估**: 2h

**步骤**:
1. macOS 构建
2. Windows 构建
3. Linux 构建

---

## 任务汇总

| 项目 | 预估工时 | 优先级 | 依赖 |
|------|----------|--------|------|
| P1: turbo-downloader | 40h | P0 | 无 |
| P2: turbo-crawler | 30h | P1 | P1 |
| P3: turbo-ui | 25h | P1 | 无 (并行) |
| P4: turbo-manager | 25h | P1 | P1 |
| P5: turbo-integration | 20h | P2 | P1, P2, P4 |
| P6: turbo-app | 15h | P2 | 所有模块 |
| **总计** | **155h** | | |

## 里程碑规划

```
M1: 核心下载引擎就绪 (Week 1-2)
└── P1 完成

M2: 功能模块就绪 (Week 2-3)
├── P2 完成
├── P3 完成
└── P4 完成

M3: 系统集成完成 (Week 3-4)
├── P5 完成
└── P6 开发中

M4: 发布准备就绪 (Week 4)
└── P6 完成，测试通过，打包发布
```