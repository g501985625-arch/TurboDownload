# P1: turbo-downloader 开发检查清单

## 任务检查清单

本文档为每个开发任务提供详细的输入、处理步骤、输出和验证方法。

---

## T1.1: 项目初始化

### T1.1.1: 创建 Rust crate 结构

| 项目 | 内容 |
|------|------|
| **输入** | 无 |
| **处理步骤** | 1. 创建 `crates/turbo-downloader` 目录<br>2. 运行 `cargo init --lib`<br>3. 创建子模块目录结构<br>4. 创建各模块入口文件 |
| **输出** | 完整的项目目录结构 |
| **验证方法** | `cargo check` 无错误<br>目录结构符合规范 |

**检查项**:
- [ ] `Cargo.toml` 存在且格式正确
- [ ] `src/lib.rs` 存在
- [ ] 所有子模块目录已创建
- [ ] 每个 `mod.rs` 文件已创建

---

### T1.1.2: 配置 Cargo.toml 依赖

| 项目 | 内容 |
|------|------|
| **输入** | `Cargo.toml` 模板 |
| **处理步骤** | 1. 编辑 `[package]` 部分<br>2. 添加 `[dependencies]`<br>3. 添加 `[dev-dependencies]`<br>4. 运行 `cargo fetch` |
| **输出** | 正确配置的 `Cargo.toml` |
| **验证方法** | `cargo fetch` 成功<br>`cargo build` 成功 |

**检查项**:
- [ ] 所有依赖版本已指定
- [ ] workspace 依赖正确引用
- [ ] dev-dependencies 完整
- [ ] 无版本冲突

---

### T1.1.3: 创建测试目录结构

| 项目 | 内容 |
|------|------|
| **输入** | 测试框架设计 |
| **处理步骤** | 1. 创建 `tests/` 目录<br>2. 创建测试模块文件<br>3. 创建测试工具函数 |
| **输出** | 完整的测试目录结构 |
| **验证方法** | `cargo test` 可运行 |

**检查项**:
- [ ] `tests/mod.rs` 存在
- [ ] 各测试模块文件存在
- [ ] `tests/common/mod.rs` 工具模块存在

---

### T1.1.4: 配置开发工具

| 项目 | 内容 |
|------|------|
| **输入** | 工具配置模板 |
| **处理步骤** | 1. 创建 `rustfmt.toml`<br>2. 创建 `.cargo/config.toml`<br>3. 创建 `.vscode/` 配置 |
| **输出** | 完整的开发工具配置 |
| **验证方法** | `cargo fmt -- --check` 通过 |

**检查项**:
- [ ] `rustfmt.toml` 配置正确
- [ ] `.cargo/config.toml` 配置正确
- [ ] VS Code 扩展推荐已配置

---

## T1.2: HTTP 客户端封装

### T1.2.1: 定义 HTTP 错误类型

| 项目 | 内容 |
|------|------|
| **输入** | 错误处理设计 |
| **处理步骤** | 1. 创建 `HttpError` 枚举<br>2. 实现 `From` trait<br>3. 实现 `Display` 和 `Error`<br>4. 添加单元测试 |
| **输出** | `src/http/error.rs` |
| **验证方法** | 编译通过<br>测试覆盖 > 80% |

**检查项**:
- [ ] 所有 HTTP 错误场景已覆盖
- [ ] 错误消息清晰易懂
- [ ] 包含错误链信息

**测试用例**:
```rust
#[test]
fn test_http_error_display() {
    let err = HttpError::ConnectionFailed("timeout".to_string());
    assert!(err.to_string().contains("timeout"));
}

#[test]
fn test_from_reqwest_error() {
    // 测试 From<reqwest::Error> 转换
}
```

---

### T1.2.2: 实现 HTTP 客户端

| 项目 | 内容 |
|------|------|
| **输入** | reqwest 库<br>HTTP 客户端设计 |
| **处理步骤** | 1. 创建 `HttpClient` 结构体<br>2. 实现配置 builder<br>3. 实现 GET/HEAD 请求<br>4. 添加重试逻辑 |
| **输出** | `src/http/client.rs` |
| **验证方法** | 集成测试通过<br>Mock 服务器测试通过 |

**检查项**:
- [ ] 支持自定义 headers
- [ ] 支持超时配置
- [ ] 支持代理设置
- [ ] 支持 TLS 配置

**测试用例**:
```rust
#[tokio::test]
async fn test_http_client_get() {
    let client = HttpClient::new(Default::default()).unwrap();
    let response = client.get("https://httpbin.org/get").await.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_http_client_timeout() {
    let config = HttpClientConfig {
        timeout: Duration::from_millis(1),
        ..Default::default()
    };
    let client = HttpClient::new(config).unwrap();
    let result = client.get("https://httpbin.org/delay/10").await;
    assert!(matches!(result, Err(HttpError::Timeout(_))));
}
```

---

### T1.2.3: 实现响应处理

| 项目 | 内容 |
|------|------|
| **输入** | HTTP 响应设计 |
| **处理步骤** | 1. 定义 `HttpResponse` 结构体<br>2. 实现响应解析<br>3. 实现内容长度获取<br>4. 实现范围请求支持 |
| **输出** | `src/http/response.rs` |
| **验证方法** | 单元测试通过<br>集成测试通过 |

**检查项**:
- [ ] 正确解析 Content-Length
- [ ] 正确处理 Accept-Ranges
- [ ] 正确解析 ETag
- [ ] 正确处理重定向

**测试用例**:
```rust
#[test]
fn test_parse_content_length() {
    let headers = vec![("content-length", "1024")];
    let len = parse_content_length(&headers);
    assert_eq!(len, Some(1024));
}

#[test]
fn test_supports_range_requests() {
    let headers = vec![("accept-ranges", "bytes")];
    assert!(supports_range_requests(&headers));
}
```

---

## T1.3: 多线程下载核心

### T1.3.1: 实现分片策略

| 项目 | 内容 |
|------|------|
| **输入** | 文件大小<br>线程数配置 |
| **处理步骤** | 1. 创建 `ChunkStrategy` trait<br>2. 实现固定大小策略<br>3. 实现固定数量策略<br>4. 实现自适应策略 |
| **输出** | `src/chunk/strategy.rs` |
| **验证方法** | 单元测试通过<br>边界条件测试通过 |

**检查项**:
- [ ] 分片大小均匀
- [ ] 最后一个分片处理正确
- [ ] 小文件不分片
- [ ] 线程数限制有效

**测试用例**:
```rust
#[test]
fn test_fixed_chunk_strategy() {
    let strategy = FixedChunkStrategy::new(1024, 4);
    let chunks = strategy.plan(4096);
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[0].size, 1024);
}

#[test]
fn test_last_chunk_partial() {
    let strategy = FixedChunkStrategy::new(1000, 4);
    let chunks = strategy.plan(3500);
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[3].size, 500); // 最后一个分片
}
```

---

### T1.3.2: 实现分片下载器

| 项目 | 内容 |
|------|------|
| **输入** | HTTP 客户端<br>分片计划 |
| **处理步骤** | 1. 创建 `ChunkWorker` 结构体<br>2. 实现范围请求下载<br>3. 实现进度报告<br>4. 实现错误重试 |
| **输出** | `src/chunk/worker.rs` |
| **验证方法** | 集成测试通过<br>并发测试通过 |

**检查项**:
- [ ] 正确设置 Range header
- [ ] 并发下载稳定
- [ ] 进度回调正确
- [ ] 错误重试有效

**测试用例**:
```rust
#[tokio::test]
async fn test_chunk_worker_download() {
    let mut server = MockServer::start().await;
    server.mock(|when, then| {
        when.method(GET).path("/file").header("range", "bytes=0-999");
        then.status(206).body(vec![0u8; 1000]);
    });
    
    let worker = ChunkWorker::new(/* ... */);
    let result = worker.download().await.unwrap();
    assert_eq!(result.bytes.len(), 1000);
}

#[tokio::test]
async fn test_concurrent_chunks() {
    // 测试多个分片并发下载
    let chunks = create_test_chunks(4);
    let results = futures::future::join_all(
        chunks.into_iter().map(|c| worker.download_chunk(c))
    ).await;
    
    assert!(results.iter().all(|r| r.is_ok()));
}
```

---

### T1.3.3: 实现下载任务管理

| 项目 | 内容 |
|------|------|
| **输入** | 下载配置 |
| **处理步骤** | 1. 创建 `DownloadTask` 结构体<br>2. 实现任务状态管理<br>3. 实现并发控制<br>4. 实现文件写入 |
| **输出** | `src/download/task.rs` |
| **验证方法** | 集成测试通过<br>文件完整性验证 |

**检查项**:
- [ ] 任务状态正确转换
- [ ] 文件写入位置正确
- [ ] 支持暂停/恢复
- [ ] 支持取消

**测试用例**:
```rust
#[tokio::test]
async fn test_download_task_lifecycle() {
    let task = DownloadTask::new(config);
    assert_eq!(task.status(), TaskStatus::Pending);
    
    task.start().await.unwrap();
    assert_eq!(task.status(), TaskStatus::Downloading);
    
    task.pause().await.unwrap();
    assert_eq!(task.status(), TaskStatus::Paused);
}

#[tokio::test]
async fn test_file_integrity() {
    let task = create_test_task().await;
    task.download().await.unwrap();
    
    let expected_hash = compute_hash(&test_data);
    let actual_hash = compute_file_hash(&task.output_path);
    assert_eq!(expected_hash, actual_hash);
}
```

---

## T1.4: 断点续传

### T1.4.1: 实现状态序列化

| 项目 | 内容 |
|------|------|
| **输入** | 下载状态数据 |
| **处理步骤** | 1. 定义 `ResumeState` 结构体<br>2. 实现 Serialize/Deserialize<br>3. 实现状态文件读写<br>4. 添加版本兼容 |
| **输出** | `src/resume/state.rs` |
| **验证方法** | 序列化测试通过<br>版本迁移测试通过 |

**检查项**:
- [ ] JSON 序列化正确
- [ ] 文件读写正确
- [ ] 版本号包含在状态中
- [ ] 向后兼容旧版本状态

**测试用例**:
```rust
#[test]
fn test_state_serialization() {
    let state = ResumeState {
        version: 1,
        url: "https://example.com/file".to_string(),
        total_size: 1024,
        downloaded: 512,
        chunks: vec![/* ... */],
    };
    
    let json = serde_json::to_string(&state).unwrap();
    let decoded: ResumeState = serde_json::from_str(&json).unwrap();
    assert_eq!(state.url, decoded.url);
}

#[test]
fn test_backward_compatibility() {
    // 测试旧版本状态文件仍可解析
    let old_json = r#"{"version": 1, "url": "...", "total_size": 100}"#;
    let state: ResumeState = serde_json::from_str(old_json).unwrap();
    assert_eq!(state.total_size, 100);
}
```

---

### T1.4.2: 实现恢复逻辑

| 项目 | 内容 |
|------|------|
| **输入** | 已保存的状态 |
| **处理步骤** | 1. 验证状态有效性<br>2. 检查服务器文件变化<br>3. 计算剩余分片<br>4. 恢复下载 |
| **输出** | `src/resume/recovery.rs` |
| **验证方法** | 恢复测试通过<br>边界条件测试通过 |

**检查项**:
- [ ] 正确验证文件未变化
- [ ] 正确计算剩余字节
- [ ] 正确设置 Range header
- [ ] 处理部分数据损坏

**测试用例**:
```rust
#[tokio::test]
async fn test_resume_from_saved_state() {
    // 模拟已下载一半的状态
    let state = create_partial_state(50);
    save_state(&state).await.unwrap();
    
    let recovery = Recovery::new();
    let task = recovery.resume_from_state(&state).await.unwrap();
    
    assert_eq!(task.downloaded(), 50);
    assert!(task.is_resuming());
}

#[tokio::test]
async fn test_detect_file_changed() {
    let state = create_state_with_etag("old-etag");
    let server = create_server_with_etag("new-etag");
    
    let result = Recovery::new().validate(&state, &server).await;
    assert!(matches!(result, Err(ResumeError::FileChanged)));
}
```

---

## T1.5: 进度回调

### T1.5.1: 实现进度追踪器

| 项目 | 内容 |
|------|------|
| **输入** | 下载事件 |
| **处理步骤** | 1. 创建 `ProgressTracker` 结构体<br>2. 实现进度计算<br>3. 实现速度计算<br>4. 实现回调触发 |
| **输出** | `src/progress/tracker.rs` |
| **验证方法** | 单元测试通过<br>性能测试通过 |

**检查项**:
- [ ] 进度百分比正确
- [ ] 下载速度准确
- [ ] 剩余时间估算合理
- [ ] 回调频率可控

**测试用例**:
```rust
#[test]
fn test_progress_calculation() {
    let mut tracker = ProgressTracker::new(1000);
    tracker.update(500);
    
    assert_eq!(tracker.percentage(), 50.0);
    assert_eq!(tracker.remaining_bytes(), 500);
}

#[test]
fn test_speed_calculation() {
    let mut tracker = ProgressTracker::new(10000);
    tracker.set_time_origin(Instant::now());
    
    // 模拟 1 秒内下载 1000 字节
    tracker.update_with_time(1000, Duration::from_secs(1));
    
    assert_eq!(tracker.speed(), 1000.0); // bytes/sec
}

#[test]
fn test_eta_calculation() {
    let mut tracker = ProgressTracker::new(10000);
    tracker.update_with_time(2000, Duration::from_secs(2));
    
    // 速度 1000 bytes/sec，剩余 8000 字节
    assert_eq!(tracker.eta(), Duration::from_secs(8));
}
```

---

### T1.5.2: 实现速度计算

| 项目 | 内容 |
|------|------|
| **输入** | 下载事件时间戳 |
| **处理步骤** | 1. 实现滑动窗口算法<br>2. 实现指数移动平均<br>3. 实现速度平滑<br>4. 添加配置选项 |
| **输出** | `src/progress/speed.rs` |
| **验证方法** | 单元测试通过<br>模拟测试通过 |

**检查项**:
- [ ] 速度波动平滑
- [ ] 突发流量处理
- [ ] 历史数据清理
- [ ] 内存使用可控

**测试用例**:
```rust
#[test]
fn test_sliding_window_speed() {
    let mut calculator = SpeedCalculator::new()
        .window_size(Duration::from_secs(5));
    
    calculator.record(100, Instant::now());
    calculator.record(200, Instant::now());
    calculator.record(300, Instant::now());
    
    let speed = calculator.calculate();
    assert!(speed > 0.0);
}

#[test]
fn test_ema_speed() {
    let mut calculator = SpeedCalculator::new()
        .method(SmoothingMethod::Exponential { alpha: 0.3 });
    
    // 测试指数移动平均的平滑效果
    calculator.record(1000, Instant::now());
    let speed1 = calculator.calculate();
    
    calculator.record(100, Instant::now());
    let speed2 = calculator.calculate();
    
    // 速度应该平滑变化，不会突然降低太多
    assert!(speed2 > speed1 * 0.5);
}
```

---

## T1.6: 错误处理与重试

### T1.6.1: 定义错误类型

| 项目 | 内容 |
|------|------|
| **输入** | 错误场景分析 |
| **处理步骤** | 1. 定义 `DownloadError` 枚举<br>2. 实现错误分类<br>3. 实现可重试判断<br>4. 实现错误链 |
| **输出** | `src/error/types.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] 所有错误类型已定义
- [ ] 错误分类正确
- [ ] 可重试判断准确
- [ ] 错误消息清晰

**测试用例**:
```rust
#[test]
fn test_error_retryable() {
    let err = DownloadError::NetworkTimeout;
    assert!(err.is_retryable());
    
    let err = DownloadError::FileNotFound;
    assert!(!err.is_retryable());
}

#[test]
fn test_error_classification() {
    let err = DownloadError::ConnectionFailed("timeout".into());
    assert_eq!(err.category(), ErrorCategory::Network);
    
    let err = DownloadError::DiskFull;
    assert_eq!(err.category(), ErrorCategory::Storage);
}
```

---

### T1.6.2: 实现重试策略

| 项目 | 内容 |
|------|------|
| **输入** | 错误类型<br>重试配置 |
| **处理步骤** | 1. 创建 `RetryPolicy` trait<br>2. 实现固定间隔策略<br>3. 实现指数退避策略<br>4. 实现最大重试限制 |
| **输出** | `src/retry/policy.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] 重试间隔正确
- [ ] 最大次数限制有效
- [ ] 可配置策略

**测试用例**:
```rust
#[test]
fn test_exponential_backoff() {
    let policy = ExponentialBackoff::new()
        .initial_delay(Duration::from_millis(100))
        .max_delay(Duration::from_secs(10))
        .multiplier(2.0);
    
    assert_eq!(policy.delay(0), Duration::from_millis(100));
    assert_eq!(policy.delay(1), Duration::from_millis(200));
    assert_eq!(policy.delay(2), Duration::from_millis(400));
    assert_eq!(policy.delay(10), Duration::from_secs(10)); // 达到最大值
}

#[test]
fn test_max_retries() {
    let policy = FixedRetry::new()
        .max_attempts(3);
    
    assert!(policy.should_retry(0));
    assert!(policy.should_retry(1));
    assert!(policy.should_retry(2));
    assert!(!policy.should_retry(3));
}
```

---

## T1.7: 测试与优化

### T1.7.1: 单元测试

| 项目 | 内容 |
|------|------|
| **输入** | 所有模块代码 |
| **处理步骤** | 1. 为每个模块编写测试<br>2. 覆盖边界条件<br>3. 测试错误路径<br>4. 测量覆盖率 |
| **输出** | 完整的单元测试套件 |
| **验证方法** | `cargo test` 通过<br>覆盖率 > 80% |

**检查项**:
- [ ] 每个公开函数有测试
- [ ] 边界条件测试完整
- [ ] 错误处理测试完整
- [ ] 测试覆盖率报告

**命令**:
```bash
cargo test -p turbo-downloader
cargo tarpaulin -p turbo-downloader
```

---

### T1.7.2: 集成测试

| 项目 | 内容 |
|------|------|
| **输入** | Mock 服务器<br>测试数据 |
| **处理步骤** | 1. 创建 Mock 服务器<br>2. 测试完整下载流程<br>3. 测试断点续传<br>4. 测试并发场景 |
| **输出** | 集成测试套件 |
| **验证方法** | 所有集成测试通过 |

**检查项**:
- [ ] Mock 服务器正确配置
- [ ] 完整下载流程测试
- [ ] 断点续传测试
- [ ] 并发安全测试

**测试场景**:
1. 正常下载
2. 分片下载
3. 断点续传
4. 网络错误恢复
5. 并发下载
6. 大文件下载

---

### T1.7.3: 性能测试

| 项目 | 内容 |
|------|------|
| **输入** | 基准测试设计 |
| **处理步骤** | 1. 创建基准测试<br>2. 测量下载速度<br>3. 测量内存使用<br>4. 测量并发性能 |
| **输出** | 基准测试报告 |
| **验证方法** | 基准测试可重复运行 |

**检查项**:
- [ ] 基准测试完整
- [ ] 性能指标达标
- [ ] 内存使用合理
- [ ] 无性能回归

**基准测试**:
```bash
cargo bench -p turbo-downloader
```

---

## T1.8: 文档与示例

### T1.8.1: API 文档

| 项目 | 内容 |
|------|------|
| **输入** | 所有公开 API |
| **处理步骤** | 1. 添加文档注释<br>2. 生成 rustdoc<br>3. 添加示例代码<br>4. 审核文档完整性 |
| **输出** | 完整的 API 文档 |
| **验证方法** | `cargo doc` 无警告 |

**检查项**:
- [ ] 所有公开项有文档
- [ ] 示例代码可运行
- [ ] 链接正确
- [ ] 无文档警告

---

### T1.8.2: 使用示例

| 项目 | 内容 |
|------|------|
| **输入** | 常见使用场景 |
| **处理步骤** | 1. 创建基础示例<br>2. 创建高级示例<br>3. 创建完整应用示例<br>4. 测试所有示例 |
| **输出** | examples/ 目录 |
| **验证方法** | 所有示例可运行 |

**示例清单**:
- [ ] `basic_download.rs` - 基础下载
- [ ] `multi_thread.rs` - 多线程下载
- [ ] `resume_download.rs` - 断点续传
- [ ] `progress_callback.rs` - 进度回调
- [ ] `custom_config.rs` - 自定义配置

---

## 发布前检查清单

### 代码质量

- [ ] 所有测试通过 (`cargo test`)
- [ ] 无 clippy 警告 (`cargo clippy`)
- [ ] 格式化正确 (`cargo fmt --check`)
- [ ] 无未使用的依赖 (`cargo udeps`)
- [ ] 文档完整 (`cargo doc`)

### 功能验证

- [ ] HTTP/HTTPS 下载正常
- [ ] 多线程下载正常
- [ ] 断点续传正常
- [ ] 进度回调正常
- [ ] 错误处理正常
- [ ] 重试机制正常

### 性能验证

- [ ] 下载速度达标
- [ ] 内存使用合理
- [ ] CPU 使用合理
- [ ] 并发安全

### 文档验证

- [ ] README 完整
- [ ] API 文档完整
- [ ] 示例代码可运行
- [ ] CHANGELOG 更新

### 发布准备

- [ ] 版本号更新
- [ ] 发布说明准备
- [ ] Git 标签创建
- [ ] 发布到 crates.io