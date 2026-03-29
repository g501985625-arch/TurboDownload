#!/bin/bash
# 会话恢复脚本 - 新会话启动时执行

echo "🔄 会话恢复中..."

# 1. 读取项目状态
echo "📊 读取项目状态..."
cat /Users/macipad/.openclaw/workspace/projects/TurboDownload/PROJECT_STATE.md

# 2. 检查 Git 状态
echo ""
echo "📁 检查项目 Git 状态..."
cd /Users/macipad/.openclaw/workspace/projects/TurboDownload && git status --short

# 3. 检查编译状态
echo ""
echo "🔨 检查编译状态..."
cargo check 2>&1 | tail -5

echo ""
echo "✅ 会话恢复完成"
