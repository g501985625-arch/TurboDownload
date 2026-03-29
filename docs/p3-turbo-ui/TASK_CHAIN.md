# P3: turbo-ui 详细任务链

## 任务概览

| 任务编号 | 任务名称 | 预估时间 | 依赖任务 |
|----------|----------|----------|----------|
| T3.1 | 项目初始化 | 3h | 无 |
| T3.2 | 基础组件开发 | 8h | T3.1 |
| T3.3 | 状态管理实现 | 5h | T3.1 |
| T3.4 | 任务列表组件 | 6h | T3.2, T3.3 |
| T3.5 | 任务详情组件 | 5h | T3.4 |
| T3.6 | 进度可视化 | 4h | T3.2 |
| T3.7 | 设置面板 | 4h | T3.2, T3.3 |
| T3.8 | 主题系统 | 3h | T3.1 |
| T3.9 | 国际化 | 2h | T3.1 |
| T3.10 | 测试与文档 | 5h | T3.1-T3.9 |

**总工时**: 45h (约 6 个工作日)

---

## T3.1: 项目初始化

### T3.1.1: 创建 Vite React 项目

**时间**: 1h  
**依赖**: 无

#### 步骤

1. **创建项目**
   ```bash
   cd ~/Projects/TurboDownload
   npm create vite@latest packages/turbo-ui -- --template react-ts
   cd packages/turbo-ui
   ```

2. **安装依赖**
   ```bash
   npm install zustand @tanstack/react-query react-router-dom
   npm install lucide-react clsx tailwind-merge
   npm install -D tailwindcss postcss autoprefixer
   npm install -D @types/node
   npm install -D vitest @testing-library/react @testing-library/jest-dom jsdom
   ```

3. **初始化 Tailwind**
   ```bash
   npx tailwindcss init -p
   ```

#### 验收标准

- [ ] `npm run dev` 启动成功
- [ ] Tailwind CSS 工作正常
- [ ] TypeScript 编译无错误

---

### T3.1.2: 配置目录结构

**时间**: 0.5h  
**依赖**: T3.1.1

#### 步骤

1. **创建目录**
   ```bash
   mkdir -p src/{components,hooks,stores,types,utils,styles,locales}
   mkdir -p src/components/{common,tasks,settings,layout}
   mkdir -p src/stores/{task,settings,ui}
   ```

2. **创建索引文件**
   ```typescript
   // src/index.ts
   export * from './components'
   export * from './hooks'
   export * from './stores'
   export * from './types'
   ```

#### 验收标准

- [ ] 目录结构符合规范
- [ ] 所有索引文件已创建

---

### T3.1.3: 配置路径别名

**时间**: 0.5h  
**依赖**: T3.1.2

#### 步骤

1. **更新 tsconfig.json**
   ```json
   {
     "compilerOptions": {
       "baseUrl": ".",
       "paths": {
         "@/*": ["./src/*"],
         "@components/*": ["./src/components/*"],
         "@hooks/*": ["./src/hooks/*"],
         "@stores/*": ["./src/stores/*"],
         "@utils/*": ["./src/utils/*"]
       }
     }
   }
   ```

2. **更新 vite.config.ts**
   ```typescript
   import path from 'path'
   
   export default defineConfig({
     resolve: {
       alias: {
         '@': path.resolve(__dirname, './src'),
         '@components': path.resolve(__dirname, './src/components'),
         '@hooks': path.resolve(__dirname, './src/hooks'),
         '@stores': path.resolve(__dirname, './src/stores'),
         '@utils': path.resolve(__dirname, './src/utils'),
       },
     },
   })
   ```

#### 验收标准

- [ ] 路径别名工作正常
- [ ] 导入无错误

---

### T3.1.4: 配置测试环境

**时间**: 1h  
**依赖**: T3.1.2

#### 步骤

1. **创建测试配置**
   ```typescript
   // vitest.config.ts
   import { defineConfig } from 'vitest/config'
   import react from '@vitejs/plugin-react'
   
   export default defineConfig({
     plugins: [react()],
     test: {
       globals: true,
       environment: 'jsdom',
       setupFiles: './src/__tests__/setup.ts',
     },
   })
   ```

2. **创建 setup 文件**
   ```typescript
   // src/__tests__/setup.ts
   import '@testing-library/jest-dom'
   import { cleanup } from '@testing-library/react'
   import { afterEach } from 'vitest'
   
   afterEach(() => {
     cleanup()
   })
   ```

#### 验收标准

- [ ] `npm run test` 运行成功
- [ ] 测试工具可用

---

## T3.2: 基础组件开发

### T3.2.1: Button 组件

**时间**: 1.5h  
**依赖**: T3.1

#### 步骤

1. **定义 Props 接口**
   ```typescript
   // src/components/common/Button/Button.tsx
   export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
     variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger'
     size?: 'sm' | 'md' | 'lg'
     isLoading?: boolean
     leftIcon?: React.ReactNode
     rightIcon?: React.ReactNode
   }
   ```

2. **实现组件**
   ```tsx
   export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
     ({ className, variant = 'primary', size = 'md', isLoading, ...props }, ref) => {
       return (
         <button
           ref={ref}
           className={cn(
             'inline-flex items-center justify-center rounded-md font-medium',
             'transition-colors focus:outline-none focus:ring-2',
             variantStyles[variant],
             sizeStyles[size],
             className
           )}
           disabled={isLoading}
           {...props}
         />
       )
     }
   )
   ```

3. **编写测试**
   ```typescript
   // Button.test.tsx
   describe('Button', () => {
     it('renders correctly', () => {
       render(<Button>Click me</Button>)
       expect(screen.getByRole('button')).toBeInTheDocument()
     })
     
     it('shows loading state', () => {
       render(<Button isLoading>Click me</Button>)
       expect(screen.getByRole('button')).toBeDisabled()
     })
   })
   ```

#### 验收标准

- [ ] 所有 variant 正确渲染
- [ ] 所有 size 正确渲染
- [ ] loading 状态正常
- [ ] 测试覆盖 > 80%

---

### T3.2.2: Input 组件

**时间**: 1.5h  
**依赖**: T3.1

#### 步骤

1. **定义 Props**
   ```typescript
   export interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
     label?: string
     error?: string
     helperText?: string
     leftIcon?: React.ReactNode
     rightIcon?: React.ReactNode
   }
   ```

2. **实现组件**
   ```tsx
   export const Input = forwardRef<HTMLInputElement, InputProps>(
     ({ className, label, error, helperText, leftIcon, rightIcon, ...props }, ref) => {
       return (
         <div className="space-y-1">
           {label && <label className="text-sm font-medium">{label}</label>}
           <div className="relative">
             {leftIcon && <span className="absolute left-3 top-1/2 -translate-y-1/2">{leftIcon}</span>}
             <input
               ref={ref}
               className={cn(
                 'w-full rounded-md border border-input bg-background px-3 py-2',
                 'focus:outline-none focus:ring-2 focus:ring-primary',
                 error && 'border-danger-500',
                 className
               )}
               {...props}
             />
             {rightIcon && <span className="absolute right-3 top-1/2 -translate-y-1/2">{rightIcon}</span>}
           </div>
           {error && <p className="text-sm text-danger-500">{error}</p>}
           {helperText && !error && <p className="text-sm text-muted-foreground">{helperText}</p>}
         </div>
       )
     }
   )
   ```

#### 验收标准

- [ ] 标签正确显示
- [ ] 错误状态正确
- [ ] 图标正确渲染
- [ ] 测试覆盖 > 80%

---

### T3.2.3: Modal 组件

**时间**: 2h  
**依赖**: T3.2.1

#### 步骤

1. **使用 React Portal**
   ```tsx
   export const Modal: FC<ModalProps> = ({ isOpen, onClose, title, children, footer }) => {
     if (!isOpen) return null
     
     return createPortal(
       <div className="fixed inset-0 z-50 flex items-center justify-center">
         <div className="fixed inset-0 bg-black/50" onClick={onClose} />
         <div className="relative z-10 w-full max-w-lg rounded-lg bg-background p-6 shadow-lg">
           <div className="mb-4 flex items-center justify-between">
             <h2 className="text-lg font-semibold">{title}</h2>
             <Button variant="ghost" size="sm" onClick={onClose}>
               <X className="h-4 w-4" />
             </Button>
           </div>
           <div className="mb-4">{children}</div>
           {footer && <div className="flex justify-end gap-2">{footer}</div>}
         </div>
       </div>,
       document.body
     )
   }
   ```

2. **添加动画**
   ```tsx
   // 使用 Tailwind 动画或 framer-motion
   ```

#### 验收标准

- [ ] 正确使用 Portal
- [ ] 点击背景关闭
- [ ] ESC 键关闭
- [ ] 动画流畅

---

### T3.2.4: Progress 组件

**时间**: 1.5h  
**依赖**: T3.1

#### 步骤

1. **定义 Props**
   ```typescript
   export interface ProgressProps {
     value: number
     max?: number
     variant?: 'default' | 'success' | 'warning' | 'danger'
     size?: 'sm' | 'md' | 'lg'
     showLabel?: boolean
     animated?: boolean
   }
   ```

2. **实现组件**
   ```tsx
   export const Progress: FC<ProgressProps> = ({
     value,
     max = 100,
     variant = 'default',
     size = 'md',
     showLabel = false,
     animated = false,
   }) => {
     const percentage = Math.min(100, Math.max(0, (value / max) * 100))
     
     return (
       <div className="w-full">
         {showLabel && (
           <div className="mb-1 flex justify-between text-sm">
             <span>{percentage.toFixed(1)}%</span>
           </div>
         )}
         <div className={cn('w-full rounded-full bg-secondary', sizeStyles[size])}>
           <div
             className={cn(
               'rounded-full transition-all duration-300',
               variantStyles[variant],
               animated && 'animate-pulse'
             )}
             style={{ width: `${percentage}%` }}
           />
         </div>
       </div>
     )
   }
   ```

#### 验收标准

- [ ] 进度计算正确
- [ ] variant 正确应用
- [ ] 动画正常
- [ ] 标签正确显示

---

### T3.2.5: Toast 组件

**时间**: 1.5h  
**依赖**: T3.1

#### 步骤

1. **定义 Toast 类型**
   ```typescript
   export type ToastType = 'success' | 'error' | 'warning' | 'info'
   
   export interface Toast {
     id: string
     type: ToastType
     title: string
     description?: string
     duration?: number
   }
   ```

2. **实现 Toast Container**
   ```tsx
   export const ToastContainer: FC = () => {
     const { toasts, removeToast } = useToast()
     
     return createPortal(
       <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
         {toasts.map(toast => (
           <ToastItem key={toast.id} toast={toast} onClose={() => removeToast(toast.id)} />
         ))}
       </div>,
       document.body
     )
   }
   ```

3. **实现 useToast hook**
   ```typescript
   export const useToast = () => {
     const { toasts, addToast, removeToast } = useToastStore()
     
     const toast = useCallback((options: Omit<Toast, 'id'>) => {
       const id = uuid()
       addToast({ ...options, id })
       
       if (options.duration !== 0) {
         setTimeout(() => removeToast(id), options.duration || 5000)
       }
     }, [addToast, removeToast])
     
     return { toast, toasts, removeToast }
   }
   ```

#### 验收标准

- [ ] 所有类型正确显示
- [ ] 自动关闭
- [ ] 可手动关闭
- [ ] 堆叠显示

---

## T3.3: 状态管理实现

### T3.3.1: Task Store

**时间**: 2h  
**依赖**: T3.1

#### 步骤

1. **定义类型**
   ```typescript
   // src/types/task.ts
   export type TaskStatus = 'pending' | 'downloading' | 'paused' | 'completed' | 'failed' | 'cancelled'
   
   export interface Task {
     id: string
     url: string
     filename: string
     outputPath: string
     totalSize: number
     downloadedSize: number
     speed: number
     status: TaskStatus
     progress: number
     eta?: number
     threads: number
     createdAt: number
     startedAt?: number
     completedAt?: number
     error?: string
   }
   ```

2. **实现 Store**
   ```typescript
   // src/stores/task/taskStore.ts
   interface TaskState {
     tasks: Map<string, Task>
     selectedTaskId: string | null
     filter: TaskFilter
     
     // Actions
     addTask: (task: Task) => void
     updateTask: (id: string, updates: Partial<Task>) => void
     removeTask: (id: string) => void
     selectTask: (id: string | null) => void
     setFilter: (filter: TaskFilter) => void
     
     // Selectors
     getFilteredTasks: () => Task[]
     getTaskById: (id: string) => Task | undefined
   }
   
   export const useTaskStore = create<TaskState>()(
     devtools(
       persist(
         (set, get) => ({
           tasks: new Map(),
           selectedTaskId: null,
           filter: { status: 'all', sortBy: 'createdAt', sortOrder: 'desc' },
           
           addTask: (task) => set((state) => {
             const newTasks = new Map(state.tasks)
             newTasks.set(task.id, task)
             return { tasks: newTasks }
           }),
           
           updateTask: (id, updates) => set((state) => {
             const newTasks = new Map(state.tasks)
             const task = newTasks.get(id)
             if (task) {
               newTasks.set(id, { ...task, ...updates })
             }
             return { tasks: newTasks }
           }),
           
           // ... 其他 actions
         }),
         { name: 'task-store' }
       )
     )
   )
   ```

#### 验收标准

- [ ] CRUD 操作正常
- [ ] 持久化工作
- [ ] 选择器正确
- [ ] DevTools 可用

---

### T3.3.2: Settings Store

**时间**: 1.5h  
**依赖**: T3.1

#### 步骤

1. **定义类型**
   ```typescript
   export interface Settings {
     general: GeneralSettings
     download: DownloadSettings
     appearance: AppearanceSettings
     advanced: AdvancedSettings
   }
   
   export interface GeneralSettings {
     language: 'en' | 'zh-CN'
     startAtLogin: boolean
     minimizeToTray: boolean
     showNotifications: boolean
     defaultDownloadDir: string
   }
   
   export interface DownloadSettings {
     maxConcurrentTasks: number
     defaultThreads: number
     maxSpeed: number
     retryAttempts: number
     retryDelay: number
     enableResume: boolean
   }
   
   export interface AppearanceSettings {
     theme: 'light' | 'dark' | 'system'
     accentColor: string
     compactMode: boolean
   }
   ```

2. **实现 Store**
   ```typescript
   interface SettingsState extends Settings {
     updateGeneral: (settings: Partial<GeneralSettings>) => void
     updateDownload: (settings: Partial<DownloadSettings>) => void
     updateAppearance: (settings: Partial<AppearanceSettings>) => void
     resetToDefaults: () => void
   }
   
   export const useSettingsStore = create<SettingsState>()(
     persist(
       (set) => ({
         ...defaultSettings,
         
         updateGeneral: (settings) => set((state) => ({
           general: { ...state.general, ...settings }
         })),
         
         // ... 其他 actions
       }),
       { name: 'settings-store' }
     )
   )
   ```

#### 验收标准

- [ ] 设置分组正确
- [ ] 持久化工作
- [ ] 重置功能正常

---

### T3.3.3: UI Store

**时间**: 1.5h  
**依赖**: T3.1

#### 步骤

1. **实现 Store**
   ```typescript
   interface UIState {
     sidebarCollapsed: boolean
     activeTab: string
     modals: {
       addTask: boolean
       settings: boolean
       about: boolean
     }
     
     toggleSidebar: () => void
     setActiveTab: (tab: string) => void
     openModal: (modal: keyof UIState['modals']) => void
     closeModal: (modal: keyof UIState['modals']) => void
   }
   
   export const useUIStore = create<UIState>((set) => ({
     sidebarCollapsed: false,
     activeTab: 'downloads',
     modals: { addTask: false, settings: false, about: false },
     
     toggleSidebar: () => set((state) => ({
       sidebarCollapsed: !state.sidebarCollapsed
     })),
     
     // ... 其他 actions
   }))
   ```

#### 验收标准

- [ ] UI 状态管理正常
- [ ] 模态框控制正常
- [ ] 侧边栏切换正常

---

## T3.4: 任务列表组件

### T3.4.1: TaskList 组件

**时间**: 2h  
**依赖**: T3.2, T3.3

#### 步骤

1. **定义组件**
   ```tsx
   export const TaskList: FC = () => {
     const { getFilteredTasks, selectTask, selectedTaskId } = useTaskStore()
     const tasks = getFilteredTasks()
     
     return (
       <div className="flex flex-col gap-2">
         <TaskFilters />
         <div className="flex-1 overflow-auto">
           {tasks.length === 0 ? (
             <EmptyState />
           ) : (
             tasks.map(task => (
               <TaskItem
                 key={task.id}
                 task={task}
                 isSelected={task.id === selectedTaskId}
                 onClick={() => selectTask(task.id)}
               />
             ))
           )}
         </div>
       </div>
     )
   }
   ```

2. **实现虚拟滚动** (可选，用于大量任务)
   ```tsx
   import { useVirtualizer } from '@tanstack/react-virtual'
   
   const virtualizer = useVirtualizer({
     count: tasks.length,
     getScrollElement: () => parentRef.current,
     estimateSize: () => 60,
   })
   ```

#### 验收标准

- [ ] 任务正确渲染
- [ ] 选择功能正常
- [ ] 空状态显示正常
- [ ] 过滤功能正常

---

### T3.4.2: TaskItem 组件

**时间**: 2h  
**依赖**: T3.4.1

#### 步骤

1. **实现组件**
   ```tsx
   interface TaskItemProps {
     task: Task
     isSelected: boolean
     onClick: () => void
   }
   
   export const TaskItem: FC<TaskItemProps> = ({ task, isSelected, onClick }) => {
     const { startTask, pauseTask, cancelTask } = useTaskActions()
     
     return (
       <div
         className={cn(
           'flex items-center gap-4 rounded-lg border p-4 transition-colors cursor-pointer',
           isSelected ? 'border-primary-500 bg-primary-50' : 'border-border hover:bg-accent'
         )}
         onClick={onClick}
       >
         <TaskIcon status={task.status} />
         <div className="flex-1 min-w-0">
           <div className="font-medium truncate">{task.filename}</div>
           <div className="flex items-center gap-4 text-sm text-muted-foreground">
             <span>{formatSize(task.downloadedSize)} / {formatSize(task.totalSize)}</span>
             <span>{formatSpeed(task.speed)}</span>
             {task.eta && <span>ETA: {formatETA(task.eta)}</span>}
           </div>
           <Progress value={task.progress} size="sm" />
         </div>
         <TaskActions task={task} onStart={startTask} onPause={pauseTask} onCancel={cancelTask} />
       </div>
     )
   }
   ```

#### 验收标准

- [ ] 任务信息正确显示
- [ ] 进度条正确显示
- [ ] 操作按钮可用
- [ ] 选中状态正确

---

### T3.4.3: TaskFilters 组件

**时间**: 2h  
**依赖**: T3.4.1

#### 步骤

1. **实现过滤组件**
   ```tsx
   export const TaskFilters: FC = () => {
     const { filter, setFilter } = useTaskStore()
     const [searchQuery, setSearchQuery] = useState('')
     
     return (
       <div className="flex items-center gap-4">
         <Input
           placeholder="Search tasks..."
           value={searchQuery}
           onChange={(e) => setSearchQuery(e.target.value)}
           leftIcon={<Search className="h-4 w-4" />}
         />
         <Select
           value={filter.status}
           onChange={(value) => setFilter({ ...filter, status: value })}
           options={[
             { value: 'all', label: 'All Tasks' },
             { value: 'downloading', label: 'Downloading' },
             { value: 'completed', label: 'Completed' },
             { value: 'paused', label: 'Paused' },
             { value: 'failed', label: 'Failed' },
           ]}
         />
         <SortButton
           sortBy={filter.sortBy}
           sortOrder={filter.sortOrder}
           onChange={(sortBy, sortOrder) => setFilter({ ...filter, sortBy, sortOrder })}
         />
       </div>
     )
   }
   ```

#### 验收标准

- [ ] 搜索功能正常
- [ ] 状态过滤正常
- [ ] 排序功能正常

---

## T3.5: 任务详情组件

### T3.5.1: TaskDetail 组件

**时间**: 2h  
**依赖**: T3.4

#### 步骤

1. **实现详情面板**
   ```tsx
   export const TaskDetail: FC = () => {
     const { selectedTaskId, getTaskById } = useTaskStore()
     const task = selectedTaskId ? getTaskById(selectedTaskId) : null
     
     if (!task) {
       return <EmptyDetailState />
     }
     
     return (
       <div className="flex flex-col gap-6 p-6">
         <TaskHeader task={task} />
         <TaskProgressSection task={task} />
         <TaskInfoSection task={task} />
         <TaskActionSection task={task} />
       </div>
     )
   }
   ```

2. **实现子组件**
   ```tsx
   const TaskHeader: FC<{ task: Task }> = ({ task }) => (
     <div className="flex items-start justify-between">
       <div>
         <h2 className="text-xl font-semibold">{task.filename}</h2>
         <p className="text-sm text-muted-foreground">{task.url}</p>
       </div>
       <TaskStatusBadge status={task.status} />
     </div>
   )
   
   const TaskProgressSection: FC<{ task: Task }> = ({ task }) => (
     <div className="space-y-4">
       <Progress value={task.progress} showLabel />
       <div className="grid grid-cols-3 gap-4">
         <StatCard label="Downloaded" value={formatSize(task.downloadedSize)} />
         <StatCard label="Speed" value={formatSpeed(task.speed)} />
         <StatCard label="ETA" value={task.eta ? formatETA(task.eta) : '--'} />
       </div>
     </div>
   )
   ```

#### 验收标准

- [ ] 详情正确显示
- [ ] 进度实时更新
- [ ] 操作按钮可用

---

### T3.5.2: TaskInfo 组件

**时间**: 1.5h  
**依赖**: T3.5.1

#### 步骤

```tsx
export const TaskInfo: FC<{ task: Task }> = ({ task }) => {
  return (
    <div className="rounded-lg border p-4">
      <h3 className="mb-4 font-medium">Task Information</h3>
      <dl className="space-y-2">
        <InfoRow label="URL" value={task.url} copyable />
        <InfoRow label="Output Path" value={task.outputPath} copyable />
        <InfoRow label="Total Size" value={formatSize(task.totalSize)} />
        <InfoRow label="Threads" value={task.threads.toString()} />
        <InfoRow label="Created" value={formatDate(task.createdAt)} />
        {task.startedAt && <InfoRow label="Started" value={formatDate(task.startedAt)} />}
        {task.completedAt && <InfoRow label="Completed" value={formatDate(task.completedAt)} />}
        {task.error && <InfoRow label="Error" value={task.error} className="text-danger-500" />}
      </dl>
    </div>
  )
}

const InfoRow: FC<{
  label: string
  value: string
  copyable?: boolean
  className?: string
}> = ({ label, value, copyable, className }) => (
  <div className="flex items-center justify-between">
    <dt className="text-sm text-muted-foreground">{label}</dt>
    <dd className={cn('flex items-center gap-2 text-sm', className)}>
      <span className="truncate max-w-xs">{value}</span>
      {copyable && <CopyButton value={value} />}
    </dd>
  </div>
)
```

#### 验收标准

- [ ] 信息正确显示
- [ ] 复制功能正常
- [ ] 样式正确

---

### T3.5.3: TaskActions 组件

**时间**: 1.5h  
**依赖**: T3.5.1

#### 步骤

```tsx
export const TaskActions: FC<{ task: Task }> = ({ task }) => {
  const { startTask, pauseTask, cancelTask, retryTask, openFolder, deleteTask } = useTaskActions()
  
  return (
    <div className="flex items-center gap-2">
      {task.status === 'pending' && (
        <Button onClick={() => startTask(task.id)} leftIcon={<Play className="h-4 w-4" />}>
          Start
        </Button>
      )}
      {task.status === 'downloading' && (
        <Button variant="outline" onClick={() => pauseTask(task.id)} leftIcon={<Pause className="h-4 w-4" />}>
          Pause
        </Button>
      )}
      {task.status === 'paused' && (
        <Button onClick={() => startTask(task.id)} leftIcon={<Play className="h-4 w-4" />}>
          Resume
        </Button>
      )}
      {task.status === 'failed' && (
        <Button onClick={() => retryTask(task.id)} leftIcon={<RefreshCw className="h-4 w-4" />}>
          Retry
        </Button>
      )}
      {!['downloading'].includes(task.status) && (
        <Button variant="ghost" size="sm" onClick={() => openFolder(task.outputPath)}>
          <FolderOpen className="h-4 w-4" />
        </Button>
      )}
      <Button variant="ghost" size="sm" onClick={() => cancelTask(task.id)}>
        <X className="h-4 w-4" />
      </Button>
      <Button variant="danger" size="sm" onClick={() => deleteTask(task.id)}>
        <Trash2 className="h-4 w-4" />
      </Button>
    </div>
  )
}
```

#### 验收标准

- [ ] 按钮根据状态显示
- [ ] 所有操作正常

---

## T3.6: 进度可视化

### T3.6.1: ProgressBar 组件

**时间**: 2h  
**依赖**: T3.2

#### 步骤

1. **实现高级进度条**
   ```tsx
   export interface ProgressBarProps {
     value: number
     max?: number
     variant?: 'default' | 'success' | 'warning' | 'danger' | 'striped'
     size?: 'sm' | 'md' | 'lg'
     showLabel?: boolean
     animated?: boolean
     striped?: boolean
     children?: React.ReactNode
   }
   
   export const ProgressBar: FC<ProgressBarProps> = ({
     value,
     max = 100,
     variant = 'default',
     size = 'md',
     showLabel = false,
     animated = false,
     striped = false,
     children,
   }) => {
     const percentage = Math.min(100, Math.max(0, (value / max) * 100))
     
     return (
       <div className="w-full">
         {showLabel && (
           <div className="mb-1 flex justify-between text-sm">
             <span>{children || `${percentage.toFixed(1)}%`}</span>
           </div>
         )}
         <div
           className={cn(
             'w-full overflow-hidden rounded-full bg-secondary',
             sizeStyles[size]
           )}
         >
           <div
             className={cn(
               'h-full rounded-full transition-all duration-300',
               variantStyles[variant],
               animated && 'animate-progress',
               striped && 'bg-stripes'
             )}
             style={{ width: `${percentage}%` }}
           />
         </div>
       </div>
     )
   }
   ```

2. **添加 CSS 条纹动画**
   ```css
   .bg-stripes {
     background-image: linear-gradient(
       45deg,
       rgba(255, 255, 255, 0.15) 25%,
       transparent 25%,
       transparent 50%,
       rgba(255, 255, 255, 0.15) 50%,
       rgba(255, 255, 255, 0.15) 75%,
       transparent 75%,
       transparent
     );
     background-size: 1rem 1rem;
   }
   ```

#### 验收标准

- [ ] 所有变体正确
- [ ] 动画流畅
- [ ] 条纹效果正确

---

### T3.6.2: SpeedIndicator 组件

**时间**: 1h  
**依赖**: T3.6.1

#### 步骤

```tsx
export interface SpeedIndicatorProps {
  speed: number // bytes per second
  showIcon?: boolean
  animated?: boolean
}

export const SpeedIndicator: FC<SpeedIndicatorProps> = ({
  speed,
  showIcon = true,
  animated = true,
}) => {
  const formattedSpeed = useMemo(() => formatSpeed(speed), [speed])
  
  return (
    <div className="flex items-center gap-1 text-sm text-muted-foreground">
      {showIcon && (
        <ArrowDown
          className={cn(
            'h-4 w-4',
            animated && speed > 0 && 'animate-bounce'
          )}
        />
      )}
      <span className="font-mono">{formattedSpeed}</span>
    </div>
  )
}
```

#### 验收标准

- [ ] 速度格式正确
- [ ] 动画正常
- [ ] 图标显示

---

### T3.6.3: ETAIndicator 组件

**时间**: 1h  
**依赖**: T3.6.1

#### 步骤

```tsx
export interface ETAIndicatorProps {
  eta: number // seconds
  showIcon?: boolean
}

export const ETAIndicator: FC<ETAIndicatorProps> = ({ eta, showIcon = true }) => {
  const formattedETA = useMemo(() => formatETA(eta), [eta])
  
  if (eta <= 0) {
    return <span className="text-sm text-muted-foreground">--:--:--</span>
  }
  
  return (
    <div className="flex items-center gap-1 text-sm text-muted-foreground">
      {showIcon && <Clock className="h-4 w-4" />}
      <span>{formattedETA}</span>
    </div>
  )
}
```

#### 验收标准

- [ ] 时间格式正确
- [ ] 边界情况处理

---

## T3.7: 设置面板

### T3.7.1: SettingsPanel 组件

**时间**: 2h  
**依赖**: T3.2, T3.3

#### 步骤

```tsx
export const SettingsPanel: FC = () => {
  const [activeSection, setActiveSection] = useState<'general' | 'download' | 'appearance'>('general')
  
  return (
    <div className="flex h-full">
      <nav className="w-48 border-r p-4">
        <ul className="space-y-1">
          <li>
            <button
              className={cn(
                'w-full rounded-md px-3 py-2 text-left text-sm transition-colors',
                activeSection === 'general' ? 'bg-primary text-white' : 'hover:bg-accent'
              )}
              onClick={() => setActiveSection('general')}
            >
              General
            </button>
          </li>
          <li>
            <button
              className={cn(
                'w-full rounded-md px-3 py-2 text-left text-sm transition-colors',
                activeSection === 'download' ? 'bg-primary text-white' : 'hover:bg-accent'
              )}
              onClick={() => setActiveSection('download')}
            >
              Download
            </button>
          </li>
          <li>
            <button
              className={cn(
                'w-full rounded-md px-3 py-2 text-left text-sm transition-colors',
                activeSection === 'appearance' ? 'bg-primary text-white' : 'hover:bg-accent'
              )}
              onClick={() => setActiveSection('appearance')}
            >
              Appearance
            </button>
          </li>
        </ul>
      </nav>
      <div className="flex-1 overflow-auto p-6">
        {activeSection === 'general' && <GeneralSettings />}
        {activeSection === 'download' && <DownloadSettings />}
        {activeSection === 'appearance' && <AppearanceSettings />}
      </div>
    </div>
  )
}
```

#### 验收标准

- [ ] 导航正常
- [ ] 各设置页正确加载

---

### T3.7.2: GeneralSettings 组件

**时间**: 1h  
**依赖**: T3.7.1

#### 步骤

```tsx
export const GeneralSettings: FC = () => {
  const { general, updateGeneral } = useSettingsStore()
  
  return (
    <div className="space-y-6">
      <h2 className="text-lg font-semibold">General Settings</h2>
      
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <Label>Language</Label>
            <p className="text-sm text-muted-foreground">Select your preferred language</p>
          </div>
          <Select
            value={general.language}
            onChange={(value) => updateGeneral({ language: value as 'en' | 'zh-CN' })}
            options={[
              { value: 'en', label: 'English' },
              { value: 'zh-CN', label: '简体中文' },
            ]}
          />
        </div>
        
        <div className="flex items-center justify-between">
          <div>
            <Label>Start at Login</Label>
            <p className="text-sm text-muted-foreground">Automatically start the app when you log in</p>
          </div>
          <Switch
            checked={general.startAtLogin}
            onChange={(checked) => updateGeneral({ startAtLogin: checked })}
          />
        </div>
        
        <div className="flex items-center justify-between">
          <div>
            <Label>Default Download Directory</Label>
            <p className="text-sm text-muted-foreground">Where to save downloaded files</p>
          </div>
          <DirectoryPicker
            value={general.defaultDownloadDir}
            onChange={(path) => updateGeneral({ defaultDownloadDir: path })}
          />
        </div>
      </div>
    </div>
  )
}
```

#### 验收标准

- [ ] 所有设置可修改
- [ ] 持久化正常

---

### T3.7.3: DownloadSettings 组件

**时间**: 1h  
**依赖**: T3.7.1

#### 步骤

```tsx
export const DownloadSettings: FC = () => {
  const { download, updateDownload } = useSettingsStore()
  
  return (
    <div className="space-y-6">
      <h2 className="text-lg font-semibold">Download Settings</h2>
      
      <div className="space-y-4">
        <div>
          <Label>Max Concurrent Tasks</Label>
          <Input
            type="number"
            min={1}
            max={10}
            value={download.maxConcurrentTasks}
            onChange={(e) => updateDownload({ maxConcurrentTasks: parseInt(e.target.value) })}
          />
        </div>
        
        <div>
          <Label>Default Threads per Task</Label>
          <Input
            type="number"
            min={1}
            max={32}
            value={download.defaultThreads}
            onChange={(e) => updateDownload({ defaultThreads: parseInt(e.target.value) })}
          />
        </div>
        
        <div>
          <Label>Max Speed (KB/s, 0 = unlimited)</Label>
          <Input
            type="number"
            min={0}
            value={download.maxSpeed}
            onChange={(e) => updateDownload({ maxSpeed: parseInt(e.target.value) })}
          />
        </div>
        
        <div className="flex items-center justify-between">
          <div>
            <Label>Enable Resume</Label>
            <p className="text-sm text-muted-foreground">Support resuming interrupted downloads</p>
          </div>
          <Switch
            checked={download.enableResume}
            onChange={(checked) => updateDownload({ enableResume: checked })}
          />
        </div>
      </div>
    </div>
  )
}
```

#### 验收标准

- [ ] 所有设置可修改
- [ ] 数值范围验证

---

## T3.8: 主题系统

### T3.8.1: ThemeProvider 组件

**时间**: 1.5h  
**依赖**: T3.1

#### 步骤

```tsx
type Theme = 'light' | 'dark' | 'system'

interface ThemeContextValue {
  theme: Theme
  setTheme: (theme: Theme) => void
  resolvedTheme: 'light' | 'dark'
}

const ThemeContext = createContext<ThemeContextValue | undefined>(undefined)

export const ThemeProvider: FC<{ children: React.ReactNode }> = ({ children }) => {
  const [theme, setThemeState] = useState<Theme>(() => {
    const stored = localStorage.getItem('theme') as Theme
    return stored || 'system'
  })
  
  const resolvedTheme = useMemo(() => {
    if (theme === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    }
    return theme
  }, [theme])
  
  useEffect(() => {
    const root = document.documentElement
    root.classList.remove('light', 'dark')
    root.classList.add(resolvedTheme)
    localStorage.setItem('theme', theme)
  }, [theme, resolvedTheme])
  
  const setTheme = useCallback((newTheme: Theme) => {
    setThemeState(newTheme)
  }, [])
  
  return (
    <ThemeContext.Provider value={{ theme, setTheme, resolvedTheme }}>
      {children}
    </ThemeContext.Provider>
  )
}

export const useTheme = () => {
  const context = useContext(ThemeContext)
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider')
  }
  return context
}
```

#### 验收标准

- [ ] 主题切换正常
- [ ] 系统主题跟随
- [ ] 持久化正常

---

### T3.8.2: AppearanceSettings 组件

**时间**: 1.5h  
**依赖**: T3.8.1

#### 步骤

```tsx
export const AppearanceSettings: FC = () => {
  const { theme, setTheme, resolvedTheme } = useTheme()
  const { appearance, updateAppearance } = useSettingsStore()
  
  return (
    <div className="space-y-6">
      <h2 className="text-lg font-semibold">Appearance</h2>
      
      <div className="space-y-4">
        <div>
          <Label>Theme</Label>
          <div className="mt-2 flex gap-4">
            <ThemeOption
              value="light"
              current={theme}
              icon={<Sun className="h-5 w-5" />}
              label="Light"
              onClick={() => setTheme('light')}
            />
            <ThemeOption
              value="dark"
              current={theme}
              icon={<Moon className="h-5 w-5" />}
              label="Dark"
              onClick={() => setTheme('dark')}
            />
            <ThemeOption
              value="system"
              current={theme}
              icon={<Monitor className="h-5 w-5" />}
              label="System"
              onClick={() => setTheme('system')}
            />
          </div>
        </div>
        
        <div>
          <Label>Accent Color</Label>
          <div className="mt-2 flex gap-2">
            {accentColors.map((color) => (
              <button
                key={color}
                className={cn(
                  'h-8 w-8 rounded-full border-2',
                  appearance.accentColor === color ? 'border-primary' : 'border-transparent'
                )}
                style={{ backgroundColor: color }}
                onClick={() => updateAppearance({ accentColor: color })}
              />
            ))}
          </div>
        </div>
        
        <div className="flex items-center justify-between">
          <div>
            <Label>Compact Mode</Label>
            <p className="text-sm text-muted-foreground">Use smaller UI elements</p>
          </div>
          <Switch
            checked={appearance.compactMode}
            onChange={(checked) => updateAppearance({ compactMode: checked })}
          />
        </div>
      </div>
    </div>
  )
}
```

#### 验收标准

- [ ] 主题切换正常
- [ ] 强调色选择正常
- [ ] 紧凑模式切换

---

## T3.9: 国际化

### T3.9.1: i18n 配置

**时间**: 1h  
**依赖**: T3.1

#### 步骤

```typescript
// src/locales/index.ts
import en from './en.json'
import zhCN from './zh-CN.json'

export const locales = {
  en,
  'zh-CN': zhCN,
}

export type Locale = keyof typeof locales

// src/hooks/useTranslation.ts
import { locales, Locale } from '@/locales'

export const useTranslation = () => {
  const { language } = useSettingsStore()
  const t = useCallback(
    (key: string, params?: Record<string, string>) => {
      let text = get(locales[language], key, key) as string
      if (params) {
        Object.entries(params).forEach(([k, v]) => {
          text = text.replace(new RegExp(`{{${k}}}`, 'g'), v)
        })
      }
      return text
    },
    [language]
  )
  return { t }
}
```

#### 验收标准

- [ ] 多语言支持
- [ ] 参数替换正常

---

### T3.9.2: 语言文件

**时间**: 1h  
**依赖**: T3.9.1

#### 步骤

```json
// src/locales/en.json
{
  "app": {
    "name": "TurboDownload",
    "version": "Version {{version}}"
  },
  "task": {
    "status": {
      "pending": "Pending",
      "downloading": "Downloading",
      "paused": "Paused",
      "completed": "Completed",
      "failed": "Failed",
      "cancelled": "Cancelled"
    },
    "actions": {
      "start": "Start",
      "pause": "Pause",
      "resume": "Resume",
      "cancel": "Cancel",
      "retry": "Retry",
      "delete": "Delete"
    }
  },
  "settings": {
    "general": "General",
    "download": "Download",
    "appearance": "Appearance"
  }
}

// src/locales/zh-CN.json
{
  "app": {
    "name": "TurboDownload",
    "version": "版本 {{version}}"
  },
  "task": {
    "status": {
      "pending": "等待中",
      "downloading": "下载中",
      "paused": "已暂停",
      "completed": "已完成",
      "failed": "失败",
      "cancelled": "已取消"
    },
    "actions": {
      "start": "开始",
      "pause": "暂停",
      "resume": "继续",
      "cancel": "取消",
      "retry": "重试",
      "delete": "删除"
    }
  },
  "settings": {
    "general": "常规",
    "download": "下载",
    "appearance": "外观"
  }
}
```

#### 验收标准

- [ ] 翻译完整
- [ ] 格式正确

---

## T3.10: 测试与文档

### T3.10.1: 单元测试

**时间**: 3h  
**依赖**: T3.1-T3.9

#### 步骤

```typescript
// src/__tests__/components/Button.test.tsx
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Button } from '@components/common/Button'

describe('Button', () => {
  it('renders with children', () => {
    render(<Button>Click me</Button>)
    expect(screen.getByRole('button', { name: /click me/i })).toBeInTheDocument()
  })
  
  it('handles click events', async () => {
    const handleClick = vi.fn()
    render(<Button onClick={handleClick}>Click me</Button>)
    
    await userEvent.click(screen.getByRole('button'))
    expect(handleClick).toHaveBeenCalledTimes(1)
  })
  
  it('shows loading state', () => {
    render(<Button isLoading>Click me</Button>)
    expect(screen.getByRole('button')).toBeDisabled()
    expect(screen.getByRole('button')).toHaveAttribute('aria-busy', 'true')
  })
  
  it.each(['primary', 'secondary', 'outline', 'ghost', 'danger'] as const)(
    'renders %s variant',
    (variant) => {
      render(<Button variant={variant}>Button</Button>)
      expect(screen.getByRole('button')).toBeInTheDocument()
    }
  )
})
```

#### 验收标准

- [ ] 组件测试覆盖 > 80%
- [ ] hooks 测试覆盖
- [ ] store 测试覆盖

---

### T3.10.2: 集成测试

**时间**: 1h  
**依赖**: T3.10.1

#### 步骤

```typescript
// src/__tests__/integration/TaskFlow.test.tsx
describe('Task Management Flow', () => {
  it('creates, starts, and completes a task', async () => {
    render(
      <Provider>
        <App />
      </Provider>
    )
    
    // Open add task modal
    await userEvent.click(screen.getByRole('button', { name: /add task/i }))
    
    // Fill form
    await userEvent.type(screen.getByLabelText(/url/i), 'https://example.com/file.zip')
    await userEvent.click(screen.getByRole('button', { name: /download/i }))
    
    // Verify task appears in list
    expect(await screen.findByText('file.zip')).toBeInTheDocument()
  })
})
```

#### 验收标准

- [ ] 主要流程测试
- [ ] 边界情况测试

---

### T3.10.3: 文档编写

**时间**: 1h  
**依赖**: T3.10.1

#### 步骤

```bash
# 生成组件文档
npm run docs

# 创建 Storybook stories
```

```typescript
// src/components/common/Button/Button.stories.tsx
import type { Meta, StoryObj } from '@storybook/react'
import { Button } from './Button'

const meta: Meta<typeof Button> = {
  title: 'Components/Common/Button',
  component: Button,
  argTypes: {
    variant: {
      control: 'select',
      options: ['primary', 'secondary', 'outline', 'ghost', 'danger'],
    },
    size: {
      control: 'select',
      options: ['sm', 'md', 'lg'],
    },
  },
}

export default meta
type Story = StoryObj<typeof Button>

export const Primary: Story = {
  args: {
    children: 'Primary Button',
    variant: 'primary',
  },
}
```

#### 验收标准

- [ ] 组件文档完整
- [ ] Storybook 可用
- [ ] README 更新

---

## 任务依赖图

```
T3.1 项目初始化
  ├── T3.2 基础组件
  │     ├── T3.4 任务列表
  │     │     └── T3.5 任务详情
  │     ├── T3.6 进度可视化
  │     └── T3.7 设置面板
  ├── T3.3 状态管理
  │     ├── T3.4 任务列表
  │     └── T3.7 设置面板
  ├── T3.8 主题系统
  └── T3.9 国际化
        └── T3.10 测试文档
```

---

## 里程碑

| 里程碑 | 完成任务 | 预计时间 |
|--------|----------|----------|
| M1 | T3.1, T3.2, T3.3 | Day 1-2 |
| M2 | T3.4, T3.5 | Day 3-4 |
| M3 | T3.6, T3.7 | Day 4-5 |
| M4 | T3.8, T3.9, T3.10 | Day 5-6 |