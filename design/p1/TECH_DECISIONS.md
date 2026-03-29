# P1: turbo-downloader 关键技术决策

## 1. 异步运行时选择

### 决策: 使用 Tokio

| 选项 | 优点 | 缺点 | 决策 |
|------|------|------|------|
| **Tokio** | 生态成熟、功能完整 | 编译稍慢 | ✅ 采用 |
| async-std | 轻量、与 std API 接近 | 生态较小 | ❌ 不选 |
| smol | 极轻量 | 功能有限 | ❌ 不选 |

**理由**:
- Tokio 是 Rust 异步生态事实标准
- 与 reqwest、tower 等主流库天然兼容
- 调试工具完善 (tokio-console)

**文档参考**: https://tokio.rs/

---

## 2. HTTP 客户端选择

### 决策: 使用 Reqwest

| 选项 | 优点 | 缺点 | 决策 |
|------|------|------|------|
| **Reqwest** | API 友好、功能完整 | 编译稍慢 | ✅ 采用 |
| surf | 异步接口优雅 | 文档较少 | ❌ 不选 |
| ureq | 同步/轻量 | 不支持异步 | ❌ 不选 |
| hyper | 底层灵活 | 需要自行封装 | ❌ 不选 |

**理由**:
- 支持 async/await
- 连接池内置
- 支持 HTTP2、TLS
- 社区活跃

**配置**:
```toml
reqwest = { version = "0.11", default-features = false, features = ["json", "rustls-tls", "stream"] }
```

---

## 3. 错误处理策略

### 决策: 使用 thiserror + anyhow

| 库 | 用途 | 决策 |
|---|------|------|
| thiserror | 定义错误类型 | ✅ 采用 |
| anyhow | 错误链传播 | ✅ 采用 |
| failure | 已废弃 | ❌ 不选 |

**理由**:
- `thiserror`: 简洁定义枚举错误类型
- `anyhow`: 方便应用层错误处理
- 保持 API 错误类型精确

**示例**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("HTTP {0}: {1}")]
    Http(u16, String),
    
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
}
```

---

## 4. 并发模型

### 决策: Tokio Task + Channel

**方案**:
- 每个分片 = 1 个 Tokio Task
- 进度更新 = async Channel
- 任务调度 = Arc<Mutex<...>>

**理由**:
- Tokio Task 轻量 (2KB 栈)
- Channel 天然支持背压
- 无锁设计减少争用

**性能目标**:
- 支持 32 分片并发
- 进度更新延迟 < 100ms

---

## 5. 进度计算策略

### 决策: 滑动窗口 + 指数移动平均

**算法**:
```rust
// 窗口大小: 10 个样本
// 速度 = 总字节 / 总时间
// ETA = 剩余字节 / 当前速度
```

**理由**:
- 滑动窗口平滑瞬时波动
- 实现简单、性能好
- 足够准确

**备选方案**: 固定时间窗口 - 不采用原因: 需要额外定时器

---

## 6. 断点续传存储

### 决策: JSON 文件存储

| 方案 | 优点 | 缺点 | 决策 |
|------|------|------|------|
| **JSON 文件** | 简单、可读、跨语言 | 性能一般 | ✅ 采用 |
| SQLite | 性能好、查询方便 | 依赖额外库 | ❌ 不选 |
| 自定义二进制 | 性能最佳 | 不可读、难调试 | ❌ 不选 |

**理由**:
- 实现简单、快速开发
- 便于调试和迁移
- 数据量小 (< 1KB/任务)

**文件位置**: `~/.cache/turbo-downloader/resume/`

---

## 7. 分片策略

### 决策: 自适应分片大小

**算法**:
```
if file_size < 10MB:
    分片数 = 2
elif file_size < 100MB:
    分片数 = 4
elif file_size < 1GB:
    分片数 = 8
else:
    分片数 = 16 (最大)

分片大小 = file_size / 分片数 (最小 1MB)
```

**理由**:
- 小文件不需要太多分片
- 大文件需要更多并发
- 避免过度分片导致开销

**约束**:
- 最小分片: 1MB
- 最大分片数: 32
- 默认分片数: 4

---

## 8. 速度限制

### 决策: Token Bucket 算法

**算法**:
```rust
struct RateLimiter {
    capacity: u64,      // 桶容量
    tokens: u64,        // 当前 token 数
    refill_rate: u64,  // 每秒填充速度
    last_refill: Instant,
}

impl RateLimiter {
    fn try_acquire(&mut self, bytes: usize) -> bool {
        self.refill();
        if self.tokens >= bytes as u64 {
            self.tokens -= bytes as u64;
            true
        } else {
            false
        }
    }
}
```

**理由**:
- 精确控制
- 无突发限制
- 实现相对简单

---

## 9. 重试策略

### 决策: 指数退避

**参数**:
- 最大重试: 3 次
- 初始间隔: 1 秒
- 退避系数: 2
- 最大间隔: 30 秒

**代码**:
```rust
async fn retry_with_backoff<F, T, E>(mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut<Result<T, E>>,
{
    let mut attempt = 0;
    let max_attempts = 3;
    let base_delay = Duration::from_secs(1);
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt >= max_attempts => return Err(e),
            Err(_) => {
                attempt += 1;
                let delay = base_delay * 2u32.pow(attempt - 1);
                tokio::time::sleep(delay).await;
            }
        }
    }
}
```

---

## 10. 日志与追踪

### 决策: 使用 tracing

| 库 | 用途 | 决策 |
|---|------|------|
| **tracing** | 结构化日志 | ✅ 采用 |
| log | 兼容旧代码 | 保留兼容 |
| println! | 调试输出 | ❌ 不用 |

**理由**:
- 结构化日志便于分析
- 异步安全
- 与 tokio 生态集成

**级别**:
- ERROR: 下载失败
- WARN: 可恢复错误
- INFO: 关键事件 (开始/完成)
- DEBUG: 详细流程

---

## 11. 依赖版本策略

### 决策: Workspace 管理

**根 Cargo.toml**:
```toml
[workspace]
members = ["crates/turbo-downloader"]
resolver = "2"

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
tracing = "0.1"
```

**理由**:
- 统一版本管理
- 避免依赖冲突
- 简化 crate 配置

---

## 12. 测试策略

### 决策: 单元测试 + 集成测试 + Mock

**测试分层**:

| 层级 | 工具 | 目标 |
|------|------|------|
| 单元测试 | tokio-test | 核心逻辑 |
| 集成测试 | wiremock | HTTP 交互 |
| 性能测试 | criterion | 基准测试 |

**理由**:
- Mock 避免网络依赖
- 单元测试保证逻辑正确
- 性能测试监控变化

---

## 13. 文档生成

### 决策: Cargo Doc

**命令**:
```bash
cargo doc --no-deps --open
```

**理由**:
- 内置支持
- 生成 API 文档
- 支持代码链接

**代码注释规范**:
```rust
/// 下载配置
///
/// # Example
/// ```
/// let config = DownloadConfig::default();
/// ```
pub struct DownloadConfig { ... }
```

---

## 14. 发布策略

### 决策: 语义化版本 (SemVer)

**版本号**: `MAJOR.MINOR.PATCH`

| 级别 | 变更 |
|------|------|
| MAJOR | 不兼容 API 变化 |
| MINOR | 新增功能 (向后兼容) |
| PATCH | Bug 修复 |

**初期版本**: 0.1.0 (不稳定)

---

## 15. 性能目标

| 指标 | 目标 |
|------|------|
| 并发分片 | 1-32 |
| 内存占用 | < 10MB (单文件) |
| 进度延迟 | < 100ms |
| 启动时间 | < 100ms |
| 错误恢复 | 自动重试 3 次 |

---

## 16. 安全性

### 决策: 最小权限原则

**考虑**:
- TLS 支持 (native-tls / rustls)
- User-Agent 设置
- 请求头验证
- 本地文件路径验证 (防止目录遍历)

**依赖审计**:
- 定期运行 `cargo audit`
- 关注 CVEs

---

## 17. 兼容性

### 目标平台

| 平台 | 状态 |
|------|------|
| Linux | ✅ 支持 |
| macOS | ✅ 支持 |
| Windows | ✅ 支持 |

### Rust 版本

- 最低版本: 1.70
- 推荐版本: latest stable

---

## 18. 总结

| 决策点 | 最终选择 |
|--------|----------|
| 运行时 | Tokio |
| HTTP 客户端 | Reqwest |
| 错误处理 | thiserror + anyhow |
| 日志 | tracing |
| 存储 | JSON 文件 |
| 测试 | tokio-test + wiremock |

---

*技术决策版本: v0.1.0*
*决策日期: 2026-03-26*