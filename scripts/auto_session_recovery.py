#!/usr/bin/env python3
"""
自动会话恢复脚本
新会话启动时自动执行，恢复项目上下文
"""

import subprocess
import sys
from datetime import datetime

def run_cmd(cmd):
    """执行命令并返回输出"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        return result.stdout.strip()
    except Exception as e:
        return f"Error: {e}"

def main():
    print("🔄 自动会话恢复启动...")
    print(f"时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print()
    
    # 1. 读取持久化记忆
    print("📚 读取持久化记忆...")
    recent = run_cmd("python3 .agents/skills/persistent-memory/scripts/memory.py recent --limit 5")
    print(recent)
    print()
    
    # 2. 搜索项目状态
    print("🔍 搜索项目状态...")
    project_state = run_cmd('python3 .agents/skills/persistent-memory/scripts/memory.py search "TurboDownload P3" --limit 3')
    print(project_state)
    print()
    
    # 3. 搜索流程规则
    print("📋 搜索流程规则...")
    workflow = run_cmd('python3 .agents/skills/persistent-memory/scripts/memory.py search "v2.1 流程" --limit 3')
    print(workflow)
    print()
    
    # 4. 检查项目状态文件
    print("📊 检查项目状态文件...")
    try:
        with open('/Users/macipad/.openclaw/workspace/projects/TurboDownload/PROJECT_STATE.md', 'r') as f:
            content = f.read()
            # 提取关键信息
            lines = content.split('\n')
            for line in lines[:50]:
                if '最后更新' in line or '当前状态' in line or 'P3' in line:
                    print(line)
    except Exception as e:
        print(f"无法读取项目状态: {e}")
    print()
    
    # 5. 检查 Git 状态
    print("📁 检查 TurboDownload Git 状态...")
    git_status = run_cmd("cd /Users/macipad/.openclaw/workspace/projects/TurboDownload && git log --oneline -3")
    print(git_status)
    print()
    
    print("✅ 自动会话恢复完成")
    print()
    print("💡 提示: 如需完整项目状态，请查看:")
    print("   - projects/TurboDownload/PROJECT_STATE.md")
    print("   - memory/2026-03-27.md")

if __name__ == "__main__":
    main()
