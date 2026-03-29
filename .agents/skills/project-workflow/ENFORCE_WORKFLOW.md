# 开发流程强制执行机制

> 确保每次项目开发都严格按照 v2.1 流程执行

---

## 强制执行检查清单

### 阶段开始前必须检查

#### Phase 1: 架构设计阶段
- [ ] 检查 `planning/P{阶段}/ARCHITECTURE.md` 是否存在
- [ ] 检查 `planning/P{阶段}/DETAILED_TASK_CHAIN.md` 是否存在
- [ ] 检查任务链是否细化到 1-2 小时子任务
- [ ] 检查每个子任务是否有明确验收标准

**如不满足，禁止进入 Phase 2！**

#### Phase 2: 并行开发阶段
- [ ] 确认 Phase 1 已完成
- [ ] 确认任务链已分配给主程序和开发员
- [ ] 确认双方了解 v2.1 流程规则

---

## 强制执行脚本

### 1. 阶段启动前检查脚本

```bash
#!/bin/bash
# check_phase_prerequisites.sh

PHASE=$1
PROJECT_DIR=$2

echo "🔍 检查 Phase $PHASE 前置条件..."

if [ "$PHASE" = "2" ]; then
    # Phase 2 需要 Phase 1 完成
    if [ ! -f "$PROJECT_DIR/planning/p${PHASE}/DETAILED_TASK_CHAIN.md" ]; then
        echo "❌ 错误: 详细任务链不存在！"
        echo "   文件: $PROJECT_DIR/planning/p${PHASE}/DETAILED_TASK_CHAIN.md"
        echo "   必须先完成 Phase 1 架构设计！"
        exit 1
    fi
    
    if [ ! -f "$PROJECT_DIR/planning/p${PHASE}/ARCHITECTURE.md" ]; then
        echo "❌ 错误: 架构设计文档不存在！"
        exit 1
    fi
    
    echo "✅ Phase $PHASE 前置条件检查通过"
fi
```

### 2. 任务执行前检查脚本

```bash
#!/bin/bash
# check_task_prerequisites.sh

TASK_ID=$1
echo "🔍 检查任务 $TASK_ID 前置条件..."

# 检查是否有详细任务链
if [ ! -f "planning/p3/DETAILED_TASK_CHAIN.md" ]; then
    echo "❌ 错误: 未找到详细任务链！"
    echo "   必须由架构师先完成 Phase 1 设计！"
    exit 1
fi

# 检查任务是否在任务链中
if ! grep -q "$TASK_ID" "planning/p3/DETAILED_TASK_CHAIN.md"; then
    echo "❌ 错误: 任务 $TASK_ID 不在任务链中！"
    exit 1
fi

echo "✅ 任务 $TASK_ID 前置条件检查通过"
```

---

## 自动化强制执行

### 方案 1: Git Hook（推荐）

在 `.git/hooks/pre-commit` 中添加：

```bash
#!/bin/bash
# 提交前检查是否按流程执行

# 检查是否有任务链设计
if [ ! -f "planning/p3/DETAILED_TASK_CHAIN.md" ]; then
    echo "❌ 错误: 未找到详细任务链设计！"
    echo "   按照 v2.1 流程，必须先完成 Phase 1 架构设计！"
    exit 1
fi

# 检查提交信息是否包含任务ID
COMMIT_MSG=$(cat "$1")
if ! echo "$COMMIT_MSG" | grep -qE "T[0-9]+\.[0-9]+"; then
    echo "⚠️ 警告: 提交信息未包含任务ID (如 T3.1)"
    echo "   按照 v2.1 流程，每个提交必须关联任务ID！"
fi
```

### 方案 2: Makefile 强制检查

```makefile
# Makefile

.PHONY: check-workflow check-phase1 check-task

check-workflow:
	@echo "🔍 检查 v2.1 流程 compliance..."
	@bash scripts/check_workflow.sh

check-phase1:
	@echo "🔍 检查 Phase 1 是否完成..."
	@test -f planning/p3/DETAILED_TASK_CHAIN.md || (echo "❌ Phase 1 未完成！"; exit 1)
	@echo "✅ Phase 1 已完成"

check-task: check-phase1
	@echo "🔍 检查任务 $(TASK) 是否可以执行..."
	@grep -q "$(TASK)" planning/p3/DETAILED_TASK_CHAIN.md || (echo "❌ 任务 $(TASK) 不在任务链中！"; exit 1)
	@echo "✅ 任务 $(TASK) 可以执行"

# 开发任务必须通过这个检查
dev-task: check-task
	@echo "🚀 启动任务 $(TASK)..."
```

### 方案 3: 会话启动强制检查

修改 `AGENTS.md`：

```markdown
## Every Session - 强制执行检查

### 1. 读取核心文件（必须）
1. Read `SOUL.md`
2. Read `USER.md`
3. Read `IDENTITY.md`
4. Read `MEMORY.md`

### 2. 流程强制执行检查（新增）
**执行**: `bash scripts/enforce_workflow_check.sh`

检查内容：
- [ ] 当前项目阶段
- [ ] Phase 1 是否完成（如需要）
- [ ] 详细任务链是否存在
- [ ] 当前任务是否在任务链中

**如检查失败，立即停止并报告！**

### 3. 自动恢复
执行: `python3 scripts/auto_session_recovery.py`
```

---

## 强制执行脚本实现

```bash
#!/bin/bash
# scripts/enforce_workflow_check.sh

echo "🔒 强制执行 v2.1 流程检查..."

# 检查是否在项目目录
if [ ! -f "Cargo.toml" ] && [ ! -f "package.json" ]; then
    echo "⚠️ 警告: 不在项目根目录，跳过流程检查"
    exit 0
fi

# 检查项目类型
if [ -d "planning" ]; then
    echo "📁 检测到规划目录，执行流程检查..."
    
    # 查找当前阶段
    for phase in p1 p2 p3 p4 p5 p6; do
        if [ -d "planning/$phase" ]; then
            echo "📊 发现阶段: $phase"
            
            # 检查详细任务链
            if [ ! -f "planning/$phase/DETAILED_TASK_CHAIN.md" ]; then
                echo "❌ 错误: Phase $phase 缺少详细任务链！"
                echo "   按照 v2.1 流程，必须先完成架构设计！"
                echo "   负责人: 架构师 (chief-architect)"
                exit 1
            fi
            
            echo "✅ Phase $phase 详细任务链已存在"
        fi
    done
fi

echo "✅ 流程检查通过"
```

---

## 记忆强制提醒

在 `MEMORY.md` 中添加：

```markdown
## ⚠️ 流程强制执行提醒

**每次项目开发必须遵守**：

1. **Phase 1 必须先完成**
   - 架构师设计详细任务链
   - 文件: `planning/P{阶段}/DETAILED_TASK_CHAIN.md`

2. **禁止跳过 Phase 1**
   - 没有任务链 = 禁止开发
   - 这是硬性规定，无例外

3. **检查方式**
   - 自动: `bash scripts/enforce_workflow_check.sh`
   - 手动: 检查 `planning/` 目录

**违反后果**：
- 开发任务无效
- 需要重新按流程执行
- 浪费时间

**记住**: 先设计，后开发！
```

---

## 实施方案

### 立即实施（今天）

1. ✅ 创建 `ENFORCE_WORKFLOW.md` 文档
2. ✅ 创建 `enforce_workflow_check.sh` 脚本
3. ✅ 更新 `AGENTS.md` 添加强制检查
4. ✅ 更新 `MEMORY.md` 添加强制提醒

### 短期实施（本周）

1. 配置 Git Hook
2. 配置 Makefile
3. 测试强制执行机制

### 长期实施（自动化工作流项目）

1. 自动化阶段检查
2. 自动化任务分配验证
3. 自动化流程合规监控

---

**目标**: 100% 流程合规，0% 违规开发
