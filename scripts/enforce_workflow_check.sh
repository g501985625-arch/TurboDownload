#!/bin/bash
# 强制执行 v2.1 流程检查脚本
# 每次项目开发前必须执行

echo "🔒 强制执行 v2.1 流程检查..."
echo "时间: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# 检查是否在项目目录
if [ ! -f "Cargo.toml" ] && [ ! -f "package.json" ]; then
    echo "⚠️ 警告: 不在项目根目录，跳过流程检查"
    exit 0
fi

# 检查项目类型
if [ -d "planning" ]; then
    echo "📁 检测到规划目录，执行流程检查..."
    echo ""
    
    # 查找当前阶段
    for phase in p1 p2 p3 p4 p5 p6; do
        if [ -d "planning/$phase" ]; then
            echo "📊 检查阶段: $phase"
            
            # 检查详细任务链
            if [ ! -f "planning/$phase/DETAILED_TASK_CHAIN.md" ]; then
                echo ""
                echo "❌❌❌ 流程违规！❌❌❌"
                echo ""
                echo "错误: Phase $phase 缺少详细任务链！"
                echo ""
                echo "按照 v2.1 流程，必须先完成 Phase 1 架构设计："
                echo "  1. 架构师设计技术架构"
                echo "  2. 架构师规划详细任务链"
                echo "  3. 任务细化到 1-2 小时子任务"
                echo "  4. 明确验收标准"
                echo ""
                echo "当前负责人: 架构师 (chief-architect)"
                echo ""
                echo "禁止进入 Phase 2 开发！"
                echo ""
                exit 1
            fi
            
            # 检查架构设计文档
            if [ ! -f "planning/$phase/ARCHITECTURE.md" ]; then
                echo "⚠️ 警告: Phase $phase 缺少架构设计文档"
            fi
            
            echo "✅ Phase $phase 详细任务链已存在"
            echo ""
        fi
    done
fi

echo "✅ 流程检查通过 - 符合 v2.1 规范"
echo ""
