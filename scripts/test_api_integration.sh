#!/bin/bash
# 集成测试脚本

echo "测试 HTTP Server..."

# 检查服务器是否运行
if curl -s http://localhost:8080/health; then
    echo "✓ HTTP Server 响应正常"
else
    echo "✗ HTTP Server 无法访问"
fi

echo -e "\n\n测试 WebSocket..."
# 使用 websocat 或 wscat 测试 WebSocket 连接
if command -v websocat &> /dev/null; then
    echo "使用 websocat 测试 WebSocket 连接..."
    # 由于 websocat 是交互式工具，我们只做简单的连接测试
    timeout 5 bash -c 'echo "ping" | websocat ws://localhost:8080/ws' &> /dev/null
    if [ $? -eq 0 ]; then
        echo "✓ WebSocket 连接测试完成"
    else
        echo "✗ WebSocket 连接失败"
    fi
else
    echo "⚠ 未安装 websocat，跳过 WebSocket 测试"
    echo "  可以通过以下命令安装:"
    echo "  npm install -g websocat"
    echo "  或者使用 cargo install websocat"
fi

echo -e "\n\n测试 CLI..."
# 检查 turbodl 命令是否存在
if command -v ./turbodl &> /dev/null; then
    ./turbodl --help
    echo "✓ CLI 命令存在"
else
    if [ -f "./turbodl" ]; then
        chmod +x ./turbodl
        ./turbodl --help
        echo "✓ CLI 命令存在"
    else
        echo "✗ CLI 命令不存在，请确保 turbodl 在当前目录"
    fi
fi

echo -e "\n\n测试 REST API..."
# 测试 API 端点
echo "测试 /health 端点..."
if curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/health | grep -q "200"; then
    echo "✓ /health 端点响应正常"
else
    echo "✗ /health 端点响应异常"
fi

# 测试 API 版本端点
echo "测试 /api/v1/ 端点..."
if curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/api/v1/ | grep -q "200\|404\|405"; then
    echo "✓ /api/v1/ 端点可访问"
else
    echo "✗ /api/v1/ 端点无法访问"
fi

echo -e "\n\nAPI 集成测试脚本执行完成"