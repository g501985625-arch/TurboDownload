# P2: turbo-crawler 技术架构设计

## 1. 设计目标

高性能网页资源抓取服务，支持 HTML 解析、URL 提取、资源分类和整站扫描。

### 非功能性需求

| 需求 | 目标值 |
|------|--------|
| 并发抓取 | 支持 1-50 并发 |
| URL 提取速度 | > 1000 URL/s |
| 内存效率 | 单域名 < 100MB |
| 支持协议 | HTTP/HTTPS |
| 解析格式 | HTML/CSS/JS |

---

## 2. 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                    TurboCrawler                          │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
│  │  URL 调度器  │  │  资源分类器  │  │  URL 规范化 │      │
│  │  Scheduler  │  │ Classifier  │  │  Normalizer │      │
│  └─────────────┘  └─────────────┘  └─────────────┘      │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐   │
│  │              抓取引擎 (Crawl Engine)              │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌──────────┐  │   │
│  │  │ HTTP Client │  │ HTML Parser │  │ 资源提取 │  │   │
│  │  │  (reqwest)  │  │  (scraper)  │  │ (Extract)│  │   │
│  │  └─────────────┘  └─────────────┘  └──────────┘  │   │
│  └─────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐   │
│  │                  存储层                           │   │
│  │  ┌─────────────┐  ┌─────────────┐               │   │
│  │  │ URL 队列    │  │ 资源索引    │               │   │
│  │  │ URLQueue   │  │ ResourceIdx │               │   │
│  │  └─────────────┘  └─────────────┘               │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

## 3. 模块划分

| 模块 | 职责 | 关键组件 |
|------|------|----------|
| `http` | HTTP 请求 | Client, Response |
| `parser` | HTML 解析 | HtmlParser, CssParser |
| `extractor` | 资源提取 | UrlExtractor, ResourceExtractor |
| `scheduler` | URL 调度 | UrlQueue, Scheduler |
| `classifier` | 资源分类 | TypeClassifier |
| `normalizer` | URL 规范化 | UrlNormalizer |

---

## 4. 核心流程

### 4.1 单页抓取流程

```
输入 URL → 规范化 → 检查已访问 → HTTP 请求 → 解析 HTML → 提取资源 → 分类存储
                                              ↓
                                         提取新 URL → 加入队列
```

### 4.2 整站扫描流程

```
种子 URL → URL 队列 → 调度器分发 → 并发抓取 → 结果处理
              ↑                              ↓
              └──────── 新 URL 回收 ─────────┘
```

---

## 5. 数据结构

### 5.1 URL 条目

```rust
pub struct UrlEntry {
    pub url: String,
    pub depth: u32,
    pub source: Option<String>,
    pub resource_type: ResourceType,
    pub status: UrlStatus,
}
```

### 5.2 资源信息

```rust
pub struct Resource {
    pub url: String,
    pub resource_type: ResourceType,
    pub size: Option<u64>,
    pub content_type: Option<String>,
    pub source_page: String,
}
```

### 5.3 资源类型

```rust
pub enum ResourceType {
    Html,       // HTML 页面
    Css,        // 样式表
    Js,         // JavaScript
    Image,      // 图片
    Font,       // 字体
    Video,      // 视频
    Audio,      // 音频
    Document,   // 文档
    Other,      // 其他
}
```

---

## 6. 技术选型

| 组件 | 选择 | 理由 |
|------|------|------|
| HTTP 客户端 | reqwest | 复用 P1 |
| HTML 解析 | scraper | 纯 Rust，性能好 |
| URL 处理 | url crate | 标准库 |
| CSS 解析 | cssparser | 可选 |
| 异步运行时 | tokio | 复用 P1 |

---

## 7. 并发模型

- 使用 Tokio Task 实现并发抓取
- 使用 `Arc<Mutex<...>>` 或 `DashMap` 管理 URL 队列
- 使用 Channel 进行任务分发

---

## 8. 目录结构

```
crates/turbo-crawler/
├── src/
│   ├── lib.rs
│   ├── http/
│   │   ├── mod.rs
│   │   └── client.rs
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── html.rs
│   │   └── css.rs
│   ├── extractor/
│   │   ├── mod.rs
│   │   ├── url.rs
│   │   └── resource.rs
│   ├── scheduler/
│   │   ├── mod.rs
│   │   ├── queue.rs
│   │   └── policy.rs
│   ├── classifier/
│   │   ├── mod.rs
│   │   └── types.rs
│   └── normalizer/
│       ├── mod.rs
│       └── url.rs
├── tests/
└── examples/
```

---

*设计版本: v0.1.0*
*设计日期: 2026-03-26*