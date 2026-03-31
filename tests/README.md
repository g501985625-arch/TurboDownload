# API 集成测试

此目录包含用于测试 TurboDownload API 的集成测试。

## 文件说明

- `api_integration_test.rs`: Rust 集成测试文件，包含以下测试用例：
  - `test_server_start`: 测试 HTTP 服务器启动
  - `test_health_endpoint`: 测试健康检查端点
  - `test_rest_api_endpoints`: 测试 REST API 端点
  - `test_websocket_connection`: 测试 WebSocket 连接
  - `test_cli_commands`: 测试 CLI 工具
  - `test_authentication`: 测试认证功能

- `test_api_integration.sh`: Bash 脚本，用于运行 API 集成测试

## 如何运行测试

### 方法 1: 使用 Cargo (推荐)

```bash
# 确保已安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 进入项目目录
cd /Users/macipad/Desktop/TurboDownload/code/TurboDownload

# 运行测试
cargo test --test api_integration_test
```

### 方法 2: 使用测试脚本

```bash
# 确保服务器正在运行
# (在另一个终端中启动 TurboDownload 服务器)

# 运行测试脚本
cd /Users/macipad/Desktop/TurboDownload/code/TurboDownload/scripts
./test_api_integration.sh
```

## 注意事项

- 运行测试前，请确保 TurboDownload 服务器正在本地 8080 端口运行
- 部分测试需要特定的依赖项，如 websocat 用于 WebSocket 测试
- CLI 测试需要 turbodl 可执行文件在 PATH 中或当前目录下