# P4: turbo-manager 开发检查清单

## 任务检查清单

本文档为每个开发任务提供详细的输入、处理步骤、输出和验证方法。

---

## T4.1: 项目初始化

### T4.1.1: 创建 Rust crate 结构

| 项目 | 内容 |
|------|------|
| **输入** | 无 |
| **处理步骤** | 1. 创建 `crates/turbo-manager` 目录<br>2. 运行 `cargo init --lib`<br>3. 创建子模块目录结构<br>4. 创建各模块入口文件 |
| **输出** | 完整的项目目录结构 |
| **验证方法** | `cargo check` 无错误<br>目录结构符合规范 |

**检查项**:
- [ ] `Cargo.toml` 存在且格式正确
- [ ] `src/lib.rs` 存在
- [ ] `src/main.rs` 存在
- [ ] 所有子模块目录已创建

---

### T4.1.2: 配置 Cargo.toml 依赖

| 项目 | 内容 |
|------|------|
| **输入** | `Cargo.toml` 模板 |
| **处理步骤** | 1. 编辑 `[package]` 部分<br>2. 添加 `[dependencies]`<br>3. 添加内部依赖<br>4. 运行 `cargo fetch` |
| **输出** | 正确配置的 `Cargo.toml` |
| **验证方法** | `cargo fetch` 成功<br>`cargo build` 成功 |

**检查项**:
- [ ] 所有依赖版本已指定
- [ ] workspace 依赖正确引用
- [ ] 内部 crate 依赖正确
- [ ] 无版本冲突

---

### T4.1.3: 创建测试目录结构

| 项目 | 内容 |
|------|------|
| **输入** | 测试框架设计 |
| **处理步骤** | 1. 创建 `tests/` 目录<br>2. 创建测试模块文件<br>3. 创建测试工具函数 |
| **输出** | 完整的测试目录结构 |
| **验证方法** | `cargo test` 可运行 |

**检查项**:
- [ ] `tests/mod.rs` 存在
- [ ] 各测试模块文件存在
- [ ] 测试框架可用

---

### T4.1.4: 配置开发工具

| 项目 | 内容 |
|------|------|
| **输入** | 工具配置模板 |
| **处理步骤** | 1. 创建 `rustfmt.toml`<br>2. 创建 `.cargo/config.toml`<br>3. 创建数据库 schema |
| **输出** | 完整的开发工具配置 |
| **验证方法** | `cargo fmt -- --check` 通过 |

**检查项**:
- [ ] `rustfmt.toml` 配置正确
- [ ] `.cargo/config.toml` 配置正确
- [ ] 数据库 schema 文件存在

---

## T4.2: 数据存储层

### T4.2.1: 设计数据库模式

| 项目 | 内容 |
|------|------|
| **输入** | 数据模型设计 |
| **处理步骤** | 1. 设计任务表<br>2. 设计配置表<br>3. 设计日志表<br>4. 创建索引 |
| **输出** | `schema/*.sql` 文件 |
| **验证方法** | 表结构合理<br>索引正确 |

**检查项**:
- [ ] tasks 表定义完整
- [ ] settings 表定义完整
- [ ] logs 表定义完整
- [ ] 索引已创建

---

### T4.2.2: 实现数据库连接池

| 项目 | 内容 |
|------|------|
| **输入** | 数据库配置 |
| **处理步骤** | 1. 定义 DatabaseConfig<br>2. 实现 Database 结构体<br>3. 实现连接池创建<br>4. 实现迁移运行 |
| **输出** | `src/store/database.rs` |
| **验证方法** | 单元测试通过<br>连接正常 |

**检查项**:
- [ ] 连接池配置正确
- [ ] 连接超时设置合理
- [ ] 迁移运行正常
- [ ] 错误处理完整

**测试用例**:
```rust
#[tokio::test]
async fn test_database_connection() {
    let db = Database::new(DatabaseConfig::default()).await.unwrap();
    assert!(db.pool().size() > 0);
}

#[tokio::test]
async fn test_run_migrations() {
    let db = Database::new(DatabaseConfig::default()).await.unwrap();
    db.run_migrations().await.unwrap();
}
```

---

### T4.2.3: 实现任务存储

| 项目 | 内容 |
|------|------|
| **输入** | Task 类型定义 |
| **处理步骤** | 1. 实现 create 方法<br>2. 实现 get 方法<br>3. 实现 list 方法<br>4. 实现 update 方法<br>5. 实现 delete 方法 |
| **输出** | `src/store/task_store.rs` |
| **验证方法** | 单元测试通过<br>CRUD 操作正常 |

**检查项**:
- [ ] create 正确实现
- [ ] get 正确实现
- [ ] list 支持过滤和分页
- [ ] update 正确实现
- [ ] delete 正确实现

**测试用例**:
```rust
#[tokio::test]
async fn test_task_store_crud() {
    let store = create_test_store().await;
    
    // Create
    let task = create_test_task();
    store.create(&task).await.unwrap();
    
    // Read
    let found = store.get(&task.id).await.unwrap();
    assert!(found.is_some());
    
    // Update
    store.update(&task.id, &TaskUpdates { status: Some(TaskStatus::Downloading), .. }).await.unwrap();
    
    // Delete
    store.delete(&task.id).await.unwrap();
    let found = store.get(&task.id).await.unwrap();
    assert!(found.is_none());
}
```

---

## T4.3: REST API 实现

### T4.3.1: 定义 API 路由

| 项目 | 内容 |
|------|------|
| **输入** | API 设计文档 |
| **处理步骤** | 1. 定义任务路由<br>2. 定义配置路由<br>3. 定义系统路由<br>4. 定义 WebSocket 路由 |
| **输出** | `src/api/routes.rs` |
| **验证方法** | 路由定义完整<br>可访问 |

**检查项**:
- [ ] GET /api/tasks 定义
- [ ] POST /api/tasks 定义
- [ ] GET /api/tasks/:id 定义
- [ ] PUT /api/tasks/:id 定义
- [ ] DELETE /api/tasks/:id 定义
- [ ] POST /api/tasks/:id/start 定义
- [ ] POST /api/tasks/:id/pause 定义
- [ ] POST /api/tasks/:id/cancel 定义
- [ ] GET /api/settings 定义
- [ ] PUT /api/settings 定义
- [ ] GET /ws 定义

---

### T4.3.2: 实现任务处理函数

| 项目 | 内容 |
|------|------|
| **输入** | 路由定义<br>TaskManager |
| **处理步骤** | 1. 实现请求/响应类型<br>2. 实现 create_task<br>3. 实现 list_tasks<br>4. 实现 get_task<br>5. 实现 update_task<br>6. 实现 delete_task |
| **输出** | `src/api/handlers.rs` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] CreateTaskRequest 定义
- [ ] TaskResponse 定义
- [ ] ApiResponse 定义
- [ ] 所有处理函数实现
- [ ] 错误处理完整

**测试用例**:
```rust
#[tokio::test]
async fn test_create_task_api() {
    let app = create_test_app().await;
    
    let response = app
        .post("/api/tasks")
        .json(&CreateTaskRequest {
            url: "https://example.com/file.zip".to_string(),
            output_path: None,
            filename: None,
            threads: None,
        })
        .send()
        .await;
    
    assert_eq!(response.status(), 200);
    let body: ApiResponse<TaskResponse> = response.json().await;
    assert!(body.success);
}

#[tokio::test]
async fn test_list_tasks_api() {
    let app = create_test_app().await;
    
    let response = app.get("/api/tasks").send().await;
    assert_eq!(response.status(), 200);
}
```

---

### T4.3.3: 实现中间件

| 项目 | 内容 |
|------|------|
| **输入** | 中间件需求 |
| **处理步骤** | 1. 实现认证中间件<br>2. 实现请求限流<br>3. 实现日志中间件<br>4. 实现 CORS 中间件 |
| **输出** | `src/api/middleware.rs` |
| **验证方法** | 中间件测试通过 |

**检查项**:
- [ ] 认证中间件工作正常
- [ ] 限流中间件工作正常
- [ ] 日志中间件工作正常
- [ ] CORS 配置正确

---

## T4.4: WebSocket 实现

### T4.4.1: 实现 WebSocket 处理

| 项目 | 内容 |
|------|------|
| **输入** | WebSocket 升级 |
| **处理步骤** | 1. 实现 WebSocket 升级处理<br>2. 实现消息处理<br>3. 实现连接管理 |
| **输出** | `src/api/ws.rs` |
| **验证方法** | WebSocket 连接正常 |

**检查项**:
- [ ] WebSocket 升级正确
- [ ] 消息收发正常
- [ ] 连接断开处理正确

**测试用例**:
```rust
#[tokio::test]
async fn test_websocket_connection() {
    let app = create_test_app().await;
    
    let mut ws = app.get("/ws").await.unwrap();
    
    ws.send(Message::Text(r#"{"type":"ping"}"#.to_string())).await.unwrap();
    let msg = ws.recv().await.unwrap();
    assert!(msg.is_text());
}
```

---

### T4.4.2: 实现实时进度推送

| 项目 | 内容 |
|------|------|
| **输入** | Broadcaster |
| **处理步骤** | 1. 定义进度事件<br>2. 实现广播逻辑<br>3. 集成到 TaskManager |
| **输出** | 进度推送功能 |
| **验证方法** | 推送正常 |

**检查项**:
- [ ] ProgressEvent 定义
- [ ] broadcast_progress 实现
- [ ] 多客户端同步

---

## T4.5: 任务调度器

### T4.5.1: 实现任务队列

| 项目 | 内容 |
|------|------|
| **输入** | 任务调度需求 |
| **处理步骤** | 1. 定义 TaskQueue<br>2. 实现优先级队列<br>3. 实现并发控制 |
| **输出** | `src/scheduler/queue.rs` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] enqueue 正确
- [ ] dequeue 正确
- [ ] 优先级排序正确
- [ ] 并发限制有效

**测试用例**:
```rust
#[test]
fn test_queue_priority() {
    let queue = TaskQueue::new(3);
    
    queue.enqueue(task_low.clone(), 1);
    queue.enqueue(task_high.clone(), 10);
    
    let task = queue.dequeue().unwrap();
    assert_eq!(task.id, task_high.id);
}

#[test]
fn test_concurrent_limit() {
    let queue = TaskQueue::new(2);
    
    assert!(queue.can_start());
    queue.mark_active(&task1.id);
    assert!(queue.can_start());
    queue.mark_active(&task2.id);
    assert!(!queue.can_start());
}
```

---

### T4.5.2: 实现工作线程池

| 项目 | 内容 |
|------|------|
| **输入** | TaskQueue<br>Downloader |
| **处理步骤** | 1. 定义 WorkerPool<br>2. 创建工作线程<br>3. 实现任务执行循环 |
| **输出** | `src/scheduler/worker.rs` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] 线程池创建正确
- [ ] 任务执行正常
- [ ] 错误处理完整

---

### T4.5.3: 实现优先级调度

| 项目 | 内容 |
|------|------|
| **输入** | 优先级需求 |
| **处理步骤** | 1. 定义 Priority 枚举<br>2. 实现优先级计算<br>3. 集成到队列 |
| **输出** | `src/scheduler/priority.rs` |
| **验证方法** | 调度顺序正确 |

**检查项**:
- [ ] Priority 定义完整
- [ ] 优先级计算合理
- [ ] 调度顺序正确

---

## T4.6: 配置管理

### T4.6.1: 实现配置加载

| 项目 | 内容 |
|------|------|
| **输入** | 配置文件 |
| **处理步骤** | 1. 定义 Settings 结构<br>2. 实现配置加载<br>3. 实现配置验证 |
| **输出** | `src/config/settings.rs` |
| **验证方法** | 配置加载正常 |

**检查项**:
- [ ] Settings 定义完整
- [ ] 配置文件加载正常
- [ ] 默认值合理
- [ ] 验证逻辑正确

---

### T4.6.2: 实现配置持久化

| 项目 | 内容 |
|------|------|
| **输入** | ConfigStore |
| **处理步骤** | 1. 实现 get 方法<br>2. 实现 set 方法<br>3. 实现批量更新 |
| **输出** | `src/store/config_store.rs` |
| **验证方法** | 配置读写正常 |

**检查项**:
- [ ] get 正确实现
- [ ] set 正确实现
- [ ] 原子更新正确

---

## T4.7: 日志系统

### T4.7.1: 配置日志输出

| 项目 | 内容 |
|------|------|
| **输入** | 日志配置 |
| **处理步骤** | 1. 配置 tracing<br>2. 配置输出格式<br>3. 配置日志级别 |
| **输出** | `src/logging/setup.rs` |
| **验证方法** | 日志输出正常 |

**检查项**:
- [ ] 日志初始化正确
- [ ] 格式正确
- [ ] 级别过滤正确

---

### T4.7.2: 实现日志查询

| 项目 | 内容 |
|------|------|
| **输入** | LogStore |
| **处理步骤** | 1. 实现日志存储<br>2. 实现日志查询<br>3. 实现日志清理 |
| **输出** | `src/logging/query.rs` |
| **验证方法** | 查询正常 |

**检查项**:
- [ ] 日志存储正确
- [ ] 查询过滤正确
- [ ] 清理逻辑正确

---

## T4.8: IPC 桥接

### T4.8.1: 定义 IPC 命令

| 项目 | 内容 |
|------|------|
| **输入** | IPC 需求 |
| **处理步骤** | 1. 定义 IpcCommand 枚举<br>2. 定义 IpcResponse<br>3. 实现序列化 |
| **输出** | `src/ipc/commands.rs` |
| **验证方法** | 序列化正确 |

**检查项**:
- [ ] 所有命令定义
- [ ] 响应格式正确
- [ ] 序列化正常

---

### T4.8.2: 实现 IPC 处理器

| 项目 | 内容 |
|------|------|
| **输入** | IpcCommand<br>TaskManager |
| **处理步骤** | 1. 实现 IpcHandler<br>2. 实现命令分发<br>3. 实现响应生成 |
| **输出** | `src/ipc/handler.rs` |
| **验证方法** | IPC 正常工作 |

**检查项**:
- [ ] 所有命令处理
- [ ] 错误处理完整
- [ ] 响应格式正确

---

## T4.9: 测试与优化

### T4.9.1: 单元测试

| 项目 | 内容 |
|------|------|
| **输入** | 所有模块代码 |
| **处理步骤** | 1. 编写 store 测试<br>2. 编写 api 测试<br>3. 编写 scheduler 测试<br>4. 测量覆盖率 |
| **输出** | 完整的单元测试套件 |
| **验证方法** | `cargo test` 通过<br>覆盖率 > 80% |

**检查项**:
- [ ] store 测试完整
- [ ] api 测试完整
- [ ] scheduler 测试完整
- [ ] 测试覆盖率报告

**命令**:
```bash
cargo test -p turbo-manager
cargo tarpaulin -p turbo-manager
```

---

### T4.9.2: 集成测试

| 项目 | 内容 |
|------|------|
| **输入** | 完整应用 |
| **处理步骤** | 1. 创建测试服务器<br>2. 测试完整流程<br>3. 测试并发场景 |
| **输出** | 集成测试套件 |
| **验证方法** | 所有集成测试通过 |

**检查项**:
- [ ] 任务生命周期测试
- [ ] WebSocket 连接测试
- [ ] 并发下载测试

---

### T4.9.3: 性能测试

| 项目 | 内容 |
|------|------|
| **输入** | 基准测试设计 |
| **处理步骤** | 1. 创建 API 基准测试<br>2. 测量响应时间<br>3. 分析瓶颈 |
| **输出** | 基准测试报告 |
| **验证方法** | 基准测试可运行 |

**检查项**:
- [ ] API 基准测试完整
- [ ] 性能指标达标
- [ ] 瓶颈分析完成

---

## T4.10: 文档与示例

### T4.10.1: API 文档

| 项目 | 内容 |
|------|------|
| **输入** | 所有公开 API |
| **处理步骤** | 1. 添加文档注释<br>2. 生成 rustdoc<br>3. 创建 OpenAPI 文档 |
| **输出** | 完整的 API 文档 |
| **验证方法** | `cargo doc` 无警告 |

**检查项**:
- [ ] 所有公开项有文档
- [ ] 示例代码可运行
- [ ] OpenAPI 文档完整

---

### T4.10.2: 使用示例

| 项目 | 内容 |
|------|------|
| **输入** | 常见使用场景 |
| **处理步骤** | 1. 创建基础服务器示例<br>2. 创建前端集成示例 |
| **输出** | examples/ 目录 |
| **验证方法** | 所有示例可运行 |

**示例清单**:
- [ ] `basic_server.rs` - 基础服务器
- [ ] `with_frontend.rs` - 前端集成

---

## 发布前检查清单

### 代码质量

- [ ] 所有测试通过 (`cargo test`)
- [ ] 无 clippy 警告 (`cargo clippy`)
- [ ] 格式化正确 (`cargo fmt -- --check`)
- [ ] 无未使用的依赖 (`cargo udeps`)

### 功能验证

- [ ] REST API 正常
- [ ] WebSocket 正常
- [ ] 任务调度正常
- [ ] 配置管理正常
- [ ] 日志系统正常

### 性能验证

- [ ] API 响应时间达标
- [ ] 并发处理正常
- [ ] 内存使用合理

### 文档验证

- [ ] README 完整
- [ ] API 文档完整
- [ ] 示例代码可运行

### 发布准备

- [ ] 版本号更新
- [ ] 发布说明准备
- [ ] Git 标签创建