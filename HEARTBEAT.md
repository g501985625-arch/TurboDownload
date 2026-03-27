# HEARTBEAT.md

## 启动时必须执行（会话恢复）

### 1. 读取核心记忆文件
- Read `SOUL.md` - 身份定义
- Read `USER.md` - 用户信息  
- Read `IDENTITY.md` - 团队配置
- Read `MEMORY.md` - 长期记忆

### 2. 读取今日和昨日工作记录
- Read `memory/2026-03-27.md` (today)
- Read `memory/2026-03-26.md` (yesterday)

### 3. 读取持久化记忆
- 执行: `python3 .agents/skills/persistent-memory/scripts/memory.py recent --limit 10`

### 4. 恢复项目上下文
- 检查 TurboDownload 项目状态
- 确认 P3 Phase 2 执行进度
- 恢复主程序/开发员任务分配状态

---

## 定期任务（每小时）

- 读取最近10条记忆: pmem recent --limit 10

---

## 🕐 每日定时任务（23:55）

**任务名称**: 每日工作总结
**执行时间**: 每天 23:55

**任务内容**:
1. 读取当天 memory/YYYY-MM-DD.md 工作记录
2. 汇总当天完成的任务和进度
3. 读取当天主要会话内容摘要
4. 生成简洁的日报报告
5. **更新 MEMORY.md 长期记忆**

---

## 🕐 每日定时任务（23:55）

**任务名称**: 每日工作总结
**执行时间**: 每天 23:55
**任务内容**:
1. 读取当天 memory/YYYY-MM-DD.md 工作记录
2. 汇总当天完成的任务和进度
3. 读取当天主要会话内容摘要
4. 生成简洁的日报报告

**输出格式**:
```
📊 202X-XX-XX 工作日报

✅ 完成任务:
- 任务1
- 任务2

📝 会话摘要:
- 关键讨论点1
- 关键讨论点2

⏳ 待办事项:
- 待办1
- 待办2
```
