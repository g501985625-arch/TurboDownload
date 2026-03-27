# P3: turbo-ui 开发检查清单

## 任务检查清单

本文档为每个开发任务提供详细的输入、处理步骤、输出和验证方法。

---

## T3.1: 项目初始化

### T3.1.1: 创建 Vite React 项目

| 项目 | 内容 |
|------|------|
| **输入** | 无 |
| **处理步骤** | 1. 运行 `npm create vite@latest packages/turbo-ui -- --template react-ts`<br>2. 进入项目目录<br>3. 安装核心依赖<br>4. 安装开发依赖 |
| **输出** | 完整的项目目录结构 |
| **验证方法** | `npm run dev` 启动成功<br>TypeScript 编译无错误 |

**检查项**:
- [ ] `package.json` 存在且格式正确
- [ ] `tsconfig.json` 配置正确
- [ ] `vite.config.ts` 存在
- [ ] 所有依赖安装成功

---

### T3.1.2: 配置目录结构

| 项目 | 内容 |
|------|------|
| **输入** | Vite 项目模板 |
| **处理步骤** | 1. 创建 `src/components/` 子目录<br>2. 创建 `src/hooks/` 目录<br>3. 创建 `src/stores/` 目录<br>4. 创建 `src/types/` 目录<br>5. 创建 `src/utils/` 目录<br>6. 创建 `src/styles/` 目录 |
| **输出** | 完整的目录结构 |
| **验证方法** | 目录结构符合规范 |

**检查项**:
- [ ] `src/components/common/` 存在
- [ ] `src/components/tasks/` 存在
- [ ] `src/components/settings/` 存在
- [ ] `src/hooks/` 存在
- [ ] `src/stores/` 存在
- [ ] `src/types/` 存在

---

### T3.1.3: 配置路径别名

| 项目 | 内容 |
|------|------|
| **输入** | tsconfig.json<br>vite.config.ts |
| **处理步骤** | 1. 更新 `tsconfig.json` 添加 paths 配置<br>2. 更新 `vite.config.ts` 添加 resolve.alias<br>3. 重启开发服务器 |
| **输出** | 路径别名可用 |
| **验证方法** | `import { Button } from '@components/common/Button'` 无错误 |

**检查项**:
- [ ] `@/*` 别名可用
- [ ] `@components/*` 别名可用
- [ ] `@hooks/*` 别名可用
- [ ] `@stores/*` 别名可用
- [ ] `@utils/*` 别名可用

---

### T3.1.4: 配置测试环境

| 项目 | 内容 |
|------|------|
| **输入** | Vitest 配置 |
| **处理步骤** | 1. 安装 `vitest` 和测试库<br>2. 创建 `vitest.config.ts`<br>3. 创建 `src/__tests__/setup.ts`<br>4. 配置测试脚本 |
| **输出** | 测试环境可用 |
| **验证方法** | `npm run test` 运行成功 |

**检查项**:
- [ ] `vitest.config.ts` 配置正确
- [ ] `@testing-library/react` 可用
- [ ] `@testing-library/jest-dom` 可用
- [ ] setup 文件正确配置

---

## T3.2: 基础组件开发

### T3.2.1: Button 组件

| 项目 | 内容 |
|------|------|
| **输入** | 组件设计规范 |
| **处理步骤** | 1. 定义 ButtonProps 接口<br>2. 实现 variant 样式<br>3. 实现 size 样式<br>4. 实现 loading 状态<br>5. 添加单元测试 |
| **输出** | `src/components/common/Button/Button.tsx` |
| **验证方法** | Storybook 渲染正常<br>测试覆盖 > 80% |

**检查项**:
- [ ] primary variant 正确
- [ ] secondary variant 正确
- [ ] outline variant 正确
- [ ] ghost variant 正确
- [ ] danger variant 正确
- [ ] sm/md/lg sizes 正确
- [ ] isLoading 状态正确
- [ ] disabled 状态正确

**测试用例**:
```tsx
describe('Button', () => {
  it('renders with children', () => {
    render(<Button>Click me</Button>)
    expect(screen.getByRole('button')).toHaveTextContent('Click me')
  })
  
  it('shows loading state', () => {
    render(<Button isLoading>Loading</Button>)
    expect(screen.getByRole('button')).toBeDisabled()
  })
  
  it('handles click events', async () => {
    const handleClick = vi.fn()
    render(<Button onClick={handleClick}>Click</Button>)
    await userEvent.click(screen.getByRole('button'))
    expect(handleClick).toHaveBeenCalledTimes(1)
  })
})
```

---

### T3.2.2: Input 组件

| 项目 | 内容 |
|------|------|
| **输入** | 组件设计规范 |
| **处理步骤** | 1. 定义 InputProps 接口<br>2. 实现标签显示<br>3. 实现错误状态<br>4. 实现图标支持<br>5. 添加单元测试 |
| **输出** | `src/components/common/Input/Input.tsx` |
| **验证方法** | 测试覆盖 > 80% |

**检查项**:
- [ ] label 正确显示
- [ ] error 状态正确
- [ ] helperText 正确显示
- [ ] leftIcon/rightIcon 正确
- [ ] disabled 状态正确

---

### T3.2.3: Modal 组件

| 项目 | 内容 |
|------|------|
| **输入** | 组件设计规范 |
| **处理步骤** | 1. 定义 ModalProps 接口<br>2. 实现 Portal 渲染<br>3. 实现背景点击关闭<br>4. 实现 ESC 键关闭<br>5. 添加动画效果 |
| **输出** | `src/components/common/Modal/Modal.tsx` |
| **验证方法** | 可访问性测试通过 |

**检查项**:
- [ ] Portal 正确渲染
- [ ] isOpen 控制显示隐藏
- [ ] onClose 正确调用
- [ ] ESC 键关闭
- [ ] 背景点击关闭
- [ ] 焦点陷阱正确

---

### T3.2.4: Progress 组件

| 项目 | 内容 |
|------|------|
| **输入** | 组件设计规范 |
| **处理步骤** | 1. 定义 ProgressProps 接口<br>2. 实现进度计算<br>3. 实现变体样式<br>4. 实现动画效果<br>5. 添加标签显示 |
| **输出** | `src/components/common/Progress/Progress.tsx` |
| **验证方法** | 测试覆盖 > 80% |

**检查项**:
- [ ] 进度百分比计算正确
- [ ] variant 样式正确
- [ ] size 样式正确
- [ ] animated 动画正确
- [ ] showLabel 显示正确

---

### T3.2.5: Toast 组件

| 项目 | 内容 |
|------|------|
| **输入** | 组件设计规范 |
| **处理步骤** | 1. 定义 Toast 类型<br>2. 实现 ToastContainer<br>3. 实现 useToast hook<br>4. 实现自动关闭<br>5. 实现手动关闭 |
| **输出** | `src/components/common/Toast/` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] success 类型显示正确
- [ ] error 类型显示正确
- [ ] warning 类型显示正确
- [ ] info 类型显示正确
- [ ] 自动关闭工作
- [ ] 手动关闭工作

---

## T3.3: 状态管理实现

### T3.3.1: Task Store

| 项目 | 内容 |
|------|------|
| **输入** | 状态设计文档 |
| **处理步骤** | 1. 定义 Task 类型<br>2. 定义 TaskState 接口<br>3. 实现 CRUD actions<br>4. 实现 selectors<br>5. 配置持久化 |
| **输出** | `src/stores/task/taskStore.ts` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] addTask 正确工作
- [ ] updateTask 正确工作
- [ ] removeTask 正确工作
- [ ] selectTask 正确工作
- [ ] setFilter 正确工作
- [ ] 持久化正常工作

**测试用例**:
```tsx
describe('useTaskStore', () => {
  it('adds a task', () => {
    const { addTask, tasks } = useTaskStore.getState()
    addTask(mockTask)
    expect(useTaskStore.getState().tasks.has(mockTask.id)).toBe(true)
  })
  
  it('updates a task', () => {
    const { addTask, updateTask } = useTaskStore.getState()
    addTask(mockTask)
    updateTask(mockTask.id, { status: 'downloading' })
    expect(useTaskStore.getState().tasks.get(mockTask.id)?.status).toBe('downloading')
  })
})
```

---

### T3.3.2: Settings Store

| 项目 | 内容 |
|------|------|
| **输入** | 设置设计文档 |
| **处理步骤** | 1. 定义 Settings 类型<br>2. 实现 updateGeneral<br>3. 实现 updateDownload<br>4. 实现 updateAppearance<br>5. 实现 resetToDefaults |
| **输出** | `src/stores/settings/settingsStore.ts` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] general 设置更新正确
- [ ] download 设置更新正确
- [ ] appearance 设置更新正确
- [ ] 重置功能正常
- [ ] 持久化正常

---

### T3.3.3: UI Store

| 项目 | 内容 |
|------|------|
| **输入** | UI 状态设计 |
| **处理步骤** | 1. 定义 UIState 接口<br>2. 实现 toggleSidebar<br>3. 实现 setActiveTab<br>4. 实现模态框控制 |
| **输出** | `src/stores/ui/uiStore.ts` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] sidebarCollapsed 切换正常
- [ ] activeTab 设置正常
- [ ] 模态框开关正常

---

## T3.4: 任务列表组件

### T3.4.1: TaskList 组件

| 项目 | 内容 |
|------|------|
| **输入** | Task Store<br>设计规范 |
| **处理步骤** | 1. 连接 Task Store<br>2. 实现任务渲染<br>3. 实现空状态<br>4. 实现选择功能<br>5. 可选：实现虚拟滚动 |
| **输出** | `src/components/tasks/list/TaskList.tsx` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] 任务列表正确渲染
- [ ] 选择功能正常
- [ ] 空状态显示正常
- [ ] 过滤功能正常

---

### T3.4.2: TaskItem 组件

| 项目 | 内容 |
|------|------|
| **输入** | Task 类型 |
| **处理步骤** | 1. 渲染任务信息<br>2. 渲染进度条<br>3. 渲染操作按钮<br>4. 实现选中样式 |
| **输出** | `src/components/tasks/list/TaskItem.tsx` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] 文件名正确显示
- [ ] 进度信息正确
- [ ] 操作按钮正确
- [ ] 选中状态样式正确

---

### T3.4.3: TaskFilters 组件

| 项目 | 内容 |
|------|------|
| **输入** | 过滤设计 |
| **处理步骤** | 1. 实现搜索输入<br>2. 实现状态过滤<br>3. 实现排序控制 |
| **输出** | `src/components/tasks/list/TaskFilters.tsx` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] 搜索功能正常
- [ ] 状态过滤正常
- [ ] 排序功能正常

---

## T3.5: 任务详情组件

### T3.5.1: TaskDetail 组件

| 项目 | 内容 |
|------|------|
| **输入** | Task Store<br>设计规范 |
| **处理步骤** | 1. 获取选中任务<br>2. 实现详情布局<br>3. 实现进度区域<br>4. 实现信息区域<br>5. 实现操作区域 |
| **输出** | `src/components/tasks/detail/TaskDetail.tsx` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] 未选中时显示空状态
- [ ] 选中时显示详情
- [ ] 进度实时更新
- [ ] 操作按钮正确

---

### T3.5.2: TaskInfo 组件

| 项目 | 内容 |
|------|------|
| **输入** | Task 类型 |
| **处理步骤** | 1. 渲染信息列表<br>2. 实现复制功能<br>3. 格式化显示 |
| **输出** | `src/components/tasks/detail/TaskInfo.tsx` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] URL 显示正确
- [ ] 路径显示正确
- [ ] 时间格式化正确
- [ ] 复制功能正常

---

### T3.5.3: TaskActions 组件

| 项目 | 内容 |
|------|------|
| **输入** | Task 状态 |
| **处理步骤** | 1. 根据状态显示按钮<br>2. 绑定操作函数<br>3. 实现确认对话框 |
| **输出** | `src/components/tasks/detail/TaskActions.tsx` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] pending 状态按钮正确
- [ ] downloading 状态按钮正确
- [ ] paused 状态按钮正确
- [ ] failed 状态按钮正确
- [ ] completed 状态按钮正确

---

## T3.6: 进度可视化

### T3.6.1: ProgressBar 组件

| 项目 | 内容 |
|------|------|
| **输入** | 进度数据 |
| **处理步骤** | 1. 计算百分比<br>2. 实现变体样式<br>3. 实现动画<br>4. 实现条纹效果 |
| **输出** | `src/components/tasks/progress/ProgressBar.tsx` |
| **验证方法** | 视觉测试通过 |

**检查项**:
- [ ] 百分比计算正确
- [ ] 变体样式正确
- [ ] 动画流畅
- [ ] 条纹效果正确

---

### T3.6.2: SpeedIndicator 组件

| 项目 | 内容 |
|------|------|
| **输入** | 速度值 (bytes/sec) |
| **处理步骤** | 1. 格式化速度<br>2. 实现动画图标<br>3. 单位转换 |
| **输出** | `src/components/tasks/progress/SpeedIndicator.tsx` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] B/s 格式化正确
- [ ] KB/s 格式化正确
- [ ] MB/s 格式化正确
- [ ] 动画显示正确

---

### T3.6.3: ETAIndicator 组件

| 项目 | 内容 |
|------|------|
| **输入** | 剩余时间 (秒) |
| **处理步骤** | 1. 格式化时间<br>2. 处理边界情况<br>3. 显示图标 |
| **输出** | `src/components/tasks/progress/ETAIndicator.tsx` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] 秒格式化正确
- [ ] 分钟格式化正确
- [ ] 小时格式化正确
- [ ] 零值处理正确

---

## T3.7: 设置面板

### T3.7.1: SettingsPanel 组件

| 项目 | 内容 |
|------|------|
| **输入** | Settings Store |
| **处理步骤** | 1. 实现侧边导航<br>2. 实现内容区域<br>3. 实现切换逻辑 |
| **输出** | `src/components/settings/SettingsPanel.tsx` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] 导航切换正常
- [ ] 内容区域更新正常
- [ ] 布局正确

---

### T3.7.2: GeneralSettings 组件

| 项目 | 内容 |
|------|------|
| **输入** | Settings Store |
| **处理步骤** | 1. 语言选择<br>2. 启动设置<br>3. 通知设置<br>4. 默认目录 |
| **输出** | `src/components/settings/GeneralSettings.tsx` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] 语言选择正常
- [ ] 开关切换正常
- [ ] 目录选择正常

---

### T3.7.3: DownloadSettings 组件

| 项目 | 内容 |
|------|------|
| **输入** | Settings Store |
| **处理步骤** | 1. 并发数设置<br>2. 线程数设置<br>3. 速度限制<br>4. 重试设置 |
| **输出** | `src/components/settings/DownloadSettings.tsx` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] 数值输入验证
- [ ] 范围限制正确
- [ ] 设置保存正常

---

## T3.8: 主题系统

### T3.8.1: ThemeProvider 组件

| 项目 | 内容 |
|------|------|
| **输入** | 主题设计 |
| **处理步骤** | 1. 定义 ThemeContext<br>2. 实现主题切换<br>3. 实现系统主题跟随<br>4. 实现持久化 |
| **输出** | `src/providers/ThemeProvider.tsx` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] light 主题正确
- [ ] dark 主题正确
- [ ] system 主题跟随正确
- [ ] 持久化正常

---

### T3.8.2: AppearanceSettings 组件

| 项目 | 内容 |
|------|------|
| **输入** | ThemeProvider<br>Settings Store |
| **处理步骤** | 1. 主题选择<br>2. 强调色选择<br>3. 紧凑模式 |
| **输出** | `src/components/settings/AppearanceSettings.tsx` |
| **验证方法** | 集成测试通过 |

**检查项**:
- [ ] 主题选择正常
- [ ] 强调色选择正常
- [ ] 紧凑模式切换正常

---

## T3.9: 国际化

### T3.9.1: i18n 配置

| 项目 | 内容 |
|------|------|
| **输入** | 语言文件 |
| **处理步骤** | 1. 创建语言文件<br>2. 实现 useTranslation hook<br>3. 实现参数替换 |
| **输出** | `src/locales/`<br>`src/hooks/useTranslation.ts` |
| **验证方法** | 单元测试通过 |

**检查项**:
- [ ] 英文翻译完整
- [ ] 中文翻译完整
- [ ] 参数替换正常

---

## T3.10: 测试与文档

### T3.10.1: 单元测试

| 项目 | 内容 |
|------|------|
| **输入** | 所有组件 |
| **处理步骤** | 1. 组件测试<br>2. Hooks 测试<br>3. Store 测试<br>4. 工具函数测试 |
| **输出** | 完整测试套件 |
| **验证方法** | `npm run test` 通过<br>覆盖率 > 80% |

**检查项**:
- [ ] 组件测试完整
- [ ] Hooks 测试完整
- [ ] Store 测试完整
- [ ] 覆盖率达标

**命令**:
```bash
npm run test
npm run test:coverage
```

---

### T3.10.2: 集成测试

| 项目 | 内容 |
|------|------|
| **输入** | 组件集成 |
| **处理步骤** | 1. 任务流程测试<br>2. 设置流程测试<br>3. 主题切换测试 |
| **输出** | 集成测试套件 |
| **验证方法** | 所有测试通过 |

**检查项**:
- [ ] 任务创建流程测试
- [ ] 任务操作流程测试
- [ ] 设置保存流程测试

---

### T3.10.3: 文档编写

| 项目 | 内容 |
|------|------|
| **输入** | 所有 API |
| **处理步骤** | 1. 组件文档<br>2. Hooks 文档<br>3. Store 文档<br>4. 使用示例 |
| **输出** | 完整文档 |
| **验证方法** | 文档可访问 |

**检查项**:
- [ ] 组件 Props 文档完整
- [ ] 使用示例完整
- [ ] README 更新

---

## 发布前检查清单

### 代码质量

- [ ] 所有测试通过 (`npm run test`)
- [ ] 无 ESLint 警告 (`npm run lint`)
- [ ] 类型检查通过 (`npm run typecheck`)
- [ ] 格式化正确 (`npm run format`)

### 功能验证

- [ ] 任务列表显示正常
- [ ] 任务详情显示正常
- [ ] 进度更新正常
- [ ] 设置保存正常
- [ ] 主题切换正常
- [ ] 国际化正常

### 构建验证

- [ ] 库构建成功 (`npm run build`)
- [ ] 无构建警告
- [ ] 输出文件正确

### 文档验证

- [ ] README 完整
- [ ] API 文档完整
- [ ] 示例代码可运行
- [ ] CHANGELOG 更新

### 发布准备

- [ ] 版本号更新
- [ ] 发布说明准备
- [ ] Git 标签创建
- [ ] 发布到 npm