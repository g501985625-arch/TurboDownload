#!/bin/bash
# 自动保存项目状态到持久化记忆
# 建议每小时执行一次

echo "💾 自动保存项目状态..."

# 获取当前时间
TIME=$(date '+%Y-%m-%d %H:%M')

# 保存 P3 状态
python3 .agents/skills/persistent-memory/scripts/memory.py add \
    "[$TIME] TurboDownload P3 状态更新: 自动保存当前执行状态" \
    --tags "turbo-download,p3,auto-save,timestamp" \
    --source "assistant" 2>/dev/null

echo "✅ 项目状态已保存到持久化记忆"
