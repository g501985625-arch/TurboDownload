# P3: turbo-ui 代码模板

本文档提供核心组件结构和实现模板。

---

## 核心类型定义

### 1. Task 类型 (src/types/task.ts)

```typescript
// src/types/task.ts

export type TaskStatus = 'pending' | 'downloading' | 'paused' | 'completed' | 'failed' | 'cancelled'

export interface Task {
  /** 任务唯一标识 */
  id: string
  /** 下载 URL */
  url: string
  /** 文件名 */
  filename: string
  /** 输出路径 */
  outputPath: string
  /** 文件总大小 (字节) */
  totalSize: number
  /** 已下载大小 (字节) */
  downloadedSize: number
  /** 当前下载速度 (字节/秒) */
  speed: number
  /** 任务状态 */
  status: TaskStatus
  /** 进度百分比 (0-100) */
  progress: number
  /** 预计剩余时间 (秒) */
  eta?: number
  /** 并发线程数 */
  threads: number
  /** 创建时间 */
  createdAt: number
  /** 开始时间 */
  startedAt?: number
  /** 完成时间 */
  completedAt?: number
  /** 错误信息 */
  error?: string
}

export interface TaskFilter {
  status: TaskStatus | 'all'
  sortBy: 'createdAt' | 'filename' | 'progress' | 'size'
  sortOrder: 'asc' | 'desc'
  searchQuery?: string
}

export interface TaskActions {
  start: (id: string) => Promise<void>
  pause: (id: string) => Promise<void>
  resume: (id: string) => Promise<void>
  cancel: (id: string) => Promise<void>
  retry: (id: string) => Promise<void>
  delete: (id: string) => Promise<void>
}
```

---

### 2. Settings 类型 (src/types/settings.ts)

```typescript
// src/types/settings.ts

export interface Settings {
  general: GeneralSettings
  download: DownloadSettings
  appearance: AppearanceSettings
  advanced: AdvancedSettings
}

export interface GeneralSettings {
  /** 界面语言 */
  language: 'en' | 'zh-CN'
  /** 开机自启动 */
  startAtLogin: boolean
  /** 最小化到托盘 */
  minimizeToTray: boolean
  /** 显示通知 */
  showNotifications: boolean
  /** 默认下载目录 */
  defaultDownloadDir: string
}

export interface DownloadSettings {
  /** 最大并发任务数 */
  maxConcurrentTasks: number
  /** 默认线程数 */
  defaultThreads: number
  /** 最大速度限制 (KB/s, 0=不限) */
  maxSpeed: number
  /** 重试次数 */
  retryAttempts: number
  /** 重试延迟 (毫秒) */
  retryDelay: number
  /** 启用断点续传 */
  enableResume: boolean
}

export interface AppearanceSettings {
  /** 主题 */
  theme: 'light' | 'dark' | 'system'
  /** 强调色 */
  accentColor: string
  /** 紧凑模式 */
  compactMode: boolean
}

export interface AdvancedSettings {
  /** 日志级别 */
  logLevel: 'debug' | 'info' | 'warn' | 'error'
  /** 代理设置 */
  proxy?: string
  /** 自定义 User-Agent */
  customUserAgent?: string
}

export const defaultSettings: Settings = {
  general: {
    language: 'en',
    startAtLogin: false,
    minimizeToTray: true,
    showNotifications: true,
    defaultDownloadDir: '~/Downloads',
  },
  download: {
    maxConcurrentTasks: 3,
    defaultThreads: 4,
    maxSpeed: 0,
    retryAttempts: 3,
    retryDelay: 1000,
    enableResume: true,
  },
  appearance: {
    theme: 'system',
    accentColor: '#0ea5e9',
    compactMode: false,
  },
  advanced: {
    logLevel: 'info',
  },
}
```

---

### 3. API 类型 (src/types/api.ts)

```typescript
// src/types/api.ts

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: ApiError
}

export interface ApiError {
  code: string
  message: string
  details?: Record<string, unknown>
}

export interface CreateTaskRequest {
  url: string
  outputPath?: string
  filename?: string
  threads?: number
  headers?: Record<string, string>
}

export interface UpdateTaskRequest {
  status?: TaskStatus
  threads?: number
}

export interface TaskProgressEvent {
  taskId: string
  downloadedSize: number
  speed: number
  progress: number
  eta?: number
  status: TaskStatus
}
```

---

## 核心 Store 实现

### 1. Task Store (src/stores/task/taskStore.ts)

```typescript
// src/stores/task/taskStore.ts

import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'
import type { Task, TaskFilter, TaskStatus } from '@/types/task'

interface TaskState {
  tasks: Map<string, Task>
  selectedTaskId: string | null
  filter: TaskFilter

  // Actions
  addTask: (task: Task) => void
  updateTask: (id: string, updates: Partial<Task>) => void
  removeTask: (id: string) => void
  selectTask: (id: string | null) => void
  setFilter: (filter: Partial<TaskFilter>) => void
  clearCompleted: () => void

  // Selectors
  getFilteredTasks: () => Task[]
  getTaskById: (id: string) => Task | undefined
  getTasksByStatus: (status: TaskStatus) => Task[]
}

export const useTaskStore = create<TaskState>()(
  devtools(
    persist(
      (set, get) => ({
        tasks: new Map(),
        selectedTaskId: null,
        filter: {
          status: 'all',
          sortBy: 'createdAt',
          sortOrder: 'desc',
        },

        addTask: (task) =>
          set((state) => {
            const newTasks = new Map(state.tasks)
            newTasks.set(task.id, task)
            return { tasks: newTasks }
          }),

        updateTask: (id, updates) =>
          set((state) => {
            const newTasks = new Map(state.tasks)
            const task = newTasks.get(id)
            if (task) {
              newTasks.set(id, { ...task, ...updates })
            }
            return { tasks: newTasks }
          }),

        removeTask: (id) =>
          set((state) => {
            const newTasks = new Map(state.tasks)
            newTasks.delete(id)
            const selectedTaskId = state.selectedTaskId === id ? null : state.selectedTaskId
            return { tasks: newTasks, selectedTaskId }
          }),

        selectTask: (id) => set({ selectedTaskId: id }),

        setFilter: (filter) =>
          set((state) => ({
            filter: { ...state.filter, ...filter },
          })),

        clearCompleted: () =>
          set((state) => {
            const newTasks = new Map(state.tasks)
            for (const [id, task] of newTasks) {
              if (task.status === 'completed') {
                newTasks.delete(id)
              }
            }
            return { tasks: newTasks }
          }),

        getFilteredTasks: () => {
          const { tasks, filter } = get()
          let result = Array.from(tasks.values())

          // Filter by status
          if (filter.status !== 'all') {
            result = result.filter((t) => t.status === filter.status)
          }

          // Filter by search query
          if (filter.searchQuery) {
            const query = filter.searchQuery.toLowerCase()
            result = result.filter(
              (t) =>
                t.filename.toLowerCase().includes(query) ||
                t.url.toLowerCase().includes(query)
            )
          }

          // Sort
          result.sort((a, b) => {
            let comparison = 0
            switch (filter.sortBy) {
              case 'createdAt':
                comparison = a.createdAt - b.createdAt
                break
              case 'filename':
                comparison = a.filename.localeCompare(b.filename)
                break
              case 'progress':
                comparison = a.progress - b.progress
                break
              case 'size':
                comparison = a.totalSize - b.totalSize
                break
            }
            return filter.sortOrder === 'asc' ? comparison : -comparison
          })

          return result
        },

        getTaskById: (id) => get().tasks.get(id),

        getTasksByStatus: (status) =>
          Array.from(get().tasks.values()).filter((t) => t.status === status),
      }),
      {
        name: 'task-store',
        serialize: true,
        deserialize: (str) => {
          const data = JSON.parse(str)
          return {
            ...data.state,
            tasks: new Map(Object.entries(data.state.tasks || {})),
          }
        },
      }
    )
  )
)
```

---

### 2. Settings Store (src/stores/settings/settingsStore.ts)

```typescript
// src/stores/settings/settingsStore.ts

import { create } from 'zustand'
import { persist, devtools } from 'zustand/middleware'
import type { Settings, GeneralSettings, DownloadSettings, AppearanceSettings } from '@/types/settings'
import { defaultSettings } from '@/types/settings'

interface SettingsState extends Settings {
  updateGeneral: (settings: Partial<GeneralSettings>) => void
  updateDownload: (settings: Partial<DownloadSettings>) => void
  updateAppearance: (settings: Partial<AppearanceSettings>) => void
  updateAdvanced: (settings: Partial<AdvancedSettings>) => void
  resetToDefaults: () => void
}

export const useSettingsStore = create<SettingsState>()(
  devtools(
    persist(
      (set) => ({
        ...defaultSettings,

        updateGeneral: (settings) =>
          set(
            (state) => ({
              general: { ...state.general, ...settings },
            }),
            false,
            'updateGeneral'
          ),

        updateDownload: (settings) =>
          set(
            (state) => ({
              download: { ...state.download, ...settings },
            }),
            false,
            'updateDownload'
          ),

        updateAppearance: (settings) =>
          set(
            (state) => ({
              appearance: { ...state.appearance, ...settings },
            }),
            false,
            'updateAppearance'
          ),

        updateAdvanced: (settings) =>
          set(
            (state) => ({
              advanced: { ...state.advanced, ...settings },
            }),
            false,
            'updateAdvanced'
          ),

        resetToDefaults: () => set(defaultSettings, false, 'resetToDefaults'),
      }),
      {
        name: 'settings-store',
      }
    )
  )
)
```

---

### 3. UI Store (src/stores/ui/uiStore.ts)

```typescript
// src/stores/ui/uiStore.ts

import { create } from 'zustand'

interface ModalState {
  addTask: boolean
  settings: boolean
  about: boolean
  confirmDelete: boolean
}

interface UIState {
  sidebarCollapsed: boolean
  activeTab: 'downloads' | 'completed' | 'settings'
  modals: ModalState
  deletingTaskId: string | null

  toggleSidebar: () => void
  setActiveTab: (tab: UIState['activeTab']) => void
  openModal: (modal: keyof ModalState) => void
  closeModal: (modal: keyof ModalState) => void
  setDeletingTaskId: (id: string | null) => void
}

export const useUIStore = create<UIState>((set) => ({
  sidebarCollapsed: false,
  activeTab: 'downloads',
  modals: {
    addTask: false,
    settings: false,
    about: false,
    confirmDelete: false,
  },
  deletingTaskId: null,

  toggleSidebar: () =>
    set((state) => ({
      sidebarCollapsed: !state.sidebarCollapsed,
    })),

  setActiveTab: (tab) => set({ activeTab: tab }),

  openModal: (modal) =>
    set((state) => ({
      modals: { ...state.modals, [modal]: true },
    })),

  closeModal: (modal) =>
    set((state) => ({
      modals: { ...state.modals, [modal]: false },
    })),

  setDeletingTaskId: (id) => set({ deletingTaskId: id }),
}))
```

---

## 核心组件实现

### 1. Button 组件 (src/components/common/Button/Button.tsx)

```tsx
// src/components/common/Button/Button.tsx

import { forwardRef, type ButtonHTMLAttributes } from 'react'
import { cn } from '@/utils/cn'

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  isLoading?: boolean
  leftIcon?: React.ReactNode
  rightIcon?: React.ReactNode
}

const variantStyles: Record<string, string> = {
  primary: 'bg-primary-500 text-white hover:bg-primary-600 focus:ring-primary-500',
  secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
  outline: 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
  ghost: 'hover:bg-accent hover:text-accent-foreground',
  danger: 'bg-danger-500 text-white hover:bg-danger-600 focus:ring-danger-500',
}

const sizeStyles: Record<string, string> = {
  sm: 'h-8 px-3 text-xs rounded-md',
  md: 'h-10 px-4 text-sm rounded-md',
  lg: 'h-12 px-6 text-base rounded-lg',
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      className,
      variant = 'primary',
      size = 'md',
      isLoading = false,
      leftIcon,
      rightIcon,
      disabled,
      children,
      ...props
    },
    ref
  ) => {
    return (
      <button
        ref={ref}
        className={cn(
          'inline-flex items-center justify-center gap-2 font-medium',
          'transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2',
          'disabled:pointer-events-none disabled:opacity-50',
          variantStyles[variant],
          sizeStyles[size],
          className
        )}
        disabled={disabled || isLoading}
        aria-busy={isLoading}
        {...props}
      >
        {isLoading ? (
          <svg
            className="h-4 w-4 animate-spin"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
        ) : (
          leftIcon
        )}
        {children}
        {!isLoading && rightIcon}
      </button>
    )
  }
)

Button.displayName = 'Button'
```

---

### 2. Progress 组件 (src/components/common/Progress/Progress.tsx)

```tsx
// src/components/common/Progress/Progress.tsx

import { type FC, useMemo } from 'react'
import { cn } from '@/utils/cn'

export interface ProgressProps {
  value: number
  max?: number
  variant?: 'default' | 'success' | 'warning' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  showLabel?: boolean
  animated?: boolean
  striped?: boolean
  className?: string
}

const variantStyles: Record<string, string> = {
  default: 'bg-primary-500',
  success: 'bg-success-500',
  warning: 'bg-warning-500',
  danger: 'bg-danger-500',
}

const sizeStyles: Record<string, string> = {
  sm: 'h-1',
  md: 'h-2',
  lg: 'h-3',
}

export const Progress: FC<ProgressProps> = ({
  value,
  max = 100,
  variant = 'default',
  size = 'md',
  showLabel = false,
  animated = false,
  striped = false,
  className,
}) => {
  const percentage = useMemo(() => {
    return Math.min(100, Math.max(0, (value / max) * 100))
  }, [value, max])

  return (
    <div className={cn('w-full', className)}>
      {showLabel && (
        <div className="mb-1 flex justify-between text-xs font-medium">
          <span>{percentage.toFixed(1)}%</span>
        </div>
      )}
      <div
        className={cn(
          'w-full overflow-hidden rounded-full bg-secondary',
          sizeStyles[size]
        )}
        role="progressbar"
        aria-valuenow={percentage}
        aria-valuemin={0}
        aria-valuemax={100}
      >
        <div
          className={cn(
            'h-full rounded-full transition-all duration-300',
            variantStyles[variant],
            animated && 'animate-pulse',
            striped && 'bg-stripes'
          )}
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  )
}
```

---

### 3. TaskItem 组件 (src/components/tasks/list/TaskItem.tsx)

```tsx
// src/components/tasks/list/TaskItem.tsx

import { type FC, useCallback } from 'react'
import {
  Play,
  Pause,
  X,
  RefreshCw,
  FolderOpen,
  CheckCircle,
  AlertCircle,
  Clock,
  Download,
} from 'lucide-react'
import { cn } from '@/utils/cn'
import { formatSize, formatSpeed, formatETA } from '@/utils/format'
import type { Task, TaskStatus } from '@/types/task'
import { Progress } from '@/components/common/Progress'
import { Button } from '@/components/common/Button'

interface TaskItemProps {
  task: Task
  isSelected: boolean
  onClick: () => void
  onStart: (id: string) => void
  onPause: (id: string) => void
  onCancel: (id: string) => void
  onRetry: (id: string) => void
}

const statusIcons: Record<TaskStatus, FC<{ className?: string }>> = {
  pending: Clock,
  downloading: Download,
  paused: Pause,
  completed: CheckCircle,
  failed: AlertCircle,
  cancelled: X,
}

const statusColors: Record<TaskStatus, string> = {
  pending: 'text-muted-foreground',
  downloading: 'text-primary-500',
  paused: 'text-warning-500',
  completed: 'text-success-500',
  failed: 'text-danger-500',
  cancelled: 'text-muted-foreground',
}

export const TaskItem: FC<TaskItemProps> = ({
  task,
  isSelected,
  onClick,
  onStart,
  onPause,
  onCancel,
  onRetry,
}) => {
  const StatusIcon = statusIcons[task.status]

  const handleAction = useCallback(
    (e: React.MouseEvent, action: () => void) => {
      e.stopPropagation()
      action()
    },
    []
  )

  return (
    <div
      className={cn(
        'flex items-center gap-4 rounded-lg border p-4 transition-colors cursor-pointer',
        isSelected
          ? 'border-primary-500 bg-primary-50 dark:bg-primary-950'
          : 'border-border hover:bg-accent'
      )}
      onClick={onClick}
      role="button"
      tabIndex={0}
      aria-selected={isSelected}
    >
      <StatusIcon className={cn('h-5 w-5 flex-shrink-0', statusColors[task.status])} />

      <div className="flex-1 min-w-0">
        <div className="font-medium truncate" title={task.filename}>
          {task.filename}
        </div>

        <div className="flex items-center gap-4 text-sm text-muted-foreground mt-1">
          <span>
            {formatSize(task.downloadedSize)} / {formatSize(task.totalSize)}
          </span>
          {task.status === 'downloading' && (
            <>
              <span>{formatSpeed(task.speed)}</span>
              {task.eta && <span>ETA: {formatETA(task.eta)}</span>}
            </>
          )}
          {task.status === 'failed' && task.error && (
            <span className="text-danger-500 truncate">{task.error}</span>
          )}
        </div>

        {task.status === 'downloading' && (
          <div className="mt-2">
            <Progress value={task.progress} size="sm" />
          </div>
        )}
      </div>

      <div className="flex items-center gap-1" onClick={(e) => e.stopPropagation()}>
        {task.status === 'pending' && (
          <Button
            variant="ghost"
            size="sm"
            onClick={(e) => handleAction(e, () => onStart(task.id))}
            aria-label="Start"
          >
            <Play className="h-4 w-4" />
          </Button>
        )}
        {task.status === 'downloading' && (
          <Button
            variant="ghost"
            size="sm"
            onClick={(e) => handleAction(e, () => onPause(task.id))}
            aria-label="Pause"
          >
            <Pause className="h-4 w-4" />
          </Button>
        )}
        {task.status === 'paused' && (
          <Button
            variant="ghost"
            size="sm"
            onClick={(e) => handleAction(e, () => onStart(task.id))}
            aria-label="Resume"
          >
            <Play className="h-4 w-4" />
          </Button>
        )}
        {task.status === 'failed' && (
          <Button
            variant="ghost"
            size="sm"
            onClick={(e) => handleAction(e, () => onRetry(task.id))}
            aria-label="Retry"
          >
            <RefreshCw className="h-4 w-4" />
          </Button>
        )}
        {task.status === 'completed' && (
          <Button
            variant="ghost"
            size="sm"
            aria-label="Open folder"
          >
            <FolderOpen className="h-4 w-4" />
          </Button>
        )}
        <Button
          variant="ghost"
          size="sm"
          onClick={(e) => handleAction(e, () => onCancel(task.id))}
          aria-label="Cancel"
        >
          <X className="h-4 w-4" />
        </Button>
      </div>
    </div>
  )
}
```

---

### 4. TaskDetail 组件 (src/components/tasks/detail/TaskDetail.tsx)

```tsx
// src/components/tasks/detail/TaskDetail.tsx

import { type FC } from 'react'
import {
  Play,
  Pause,
  X,
  RefreshCw,
  FolderOpen,
  Trash2,
  ExternalLink,
} from 'lucide-react'
import { cn } from '@/utils/cn'
import { formatSize, formatSpeed, formatETA, formatDate } from '@/utils/format'
import { useTaskStore } from '@/stores/task/taskStore'
import { useUIStore } from '@/stores/ui/uiStore'
import { Progress } from '@/components/common/Progress'
import { Button } from '@/components/common/Button'

export const TaskDetail: FC = () => {
  const { selectedTaskId, getTaskById, updateTask } = useTaskStore()
  const { openModal, setDeletingTaskId } = useUIStore()

  const task = selectedTaskId ? getTaskById(selectedTaskId) : null

  if (!task) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        <p>Select a task to view details</p>
      </div>
    )
  }

  const handleStart = async () => {
    // Call API to start task
    updateTask(task.id, { status: 'downloading' })
  }

  const handlePause = async () => {
    // Call API to pause task
    updateTask(task.id, { status: 'paused' })
  }

  const handleCancel = async () => {
    // Call API to cancel task
    updateTask(task.id, { status: 'cancelled' })
  }

  const handleRetry = async () => {
    // Call API to retry task
    updateTask(task.id, { status: 'downloading' })
  }

  const handleDelete = () => {
    setDeletingTaskId(task.id)
    openModal('confirmDelete')
  }

  return (
    <div className="flex flex-col gap-6 p-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div className="min-w-0 flex-1">
          <h2 className="text-xl font-semibold truncate" title={task.filename}>
            {task.filename}
          </h2>
          <a
            href={task.url}
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1 text-sm text-muted-foreground hover:text-primary-500 mt-1"
          >
            <span className="truncate">{task.url}</span>
            <ExternalLink className="h-3 w-3 flex-shrink-0" />
          </a>
        </div>
        <span
          className={cn(
            'px-2 py-1 text-xs font-medium rounded-full',
            task.status === 'completed' && 'bg-success-100 text-success-700',
            task.status === 'downloading' && 'bg-primary-100 text-primary-700',
            task.status === 'paused' && 'bg-warning-100 text-warning-700',
            task.status === 'failed' && 'bg-danger-100 text-danger-700'
          )}
        >
          {task.status}
        </span>
      </div>

      {/* Progress Section */}
      <div className="space-y-4">
        <Progress
          value={task.progress}
          showLabel
          variant={
            task.status === 'completed'
              ? 'success'
              : task.status === 'failed'
              ? 'danger'
              : 'default'
          }
        />

        <div className="grid grid-cols-3 gap-4">
          <div className="rounded-lg border p-3">
            <p className="text-xs text-muted-foreground">Downloaded</p>
            <p className="text-lg font-semibold">
              {formatSize(task.downloadedSize)}
            </p>
            <p className="text-xs text-muted-foreground">
              of {formatSize(task.totalSize)}
            </p>
          </div>

          <div className="rounded-lg border p-3">
            <p className="text-xs text-muted-foreground">Speed</p>
            <p className="text-lg font-semibold">
              {task.status === 'downloading' ? formatSpeed(task.speed) : '--'}
            </p>
          </div>

          <div className="rounded-lg border p-3">
            <p className="text-xs text-muted-foreground">ETA</p>
            <p className="text-lg font-semibold">
              {task.eta && task.status === 'downloading' ? formatETA(task.eta) : '--'}
            </p>
          </div>
        </div>
      </div>

      {/* Info Section */}
      <div className="rounded-lg border p-4">
        <h3 className="text-sm font-medium mb-3">Task Information</h3>
        <dl className="space-y-2 text-sm">
          <div className="flex justify-between">
            <dt className="text-muted-foreground">URL</dt>
            <dd className="truncate max-w-xs">{task.url}</dd>
          </div>
          <div className="flex justify-between">
            <dt className="text-muted-foreground">Output Path</dt>
            <dd className="truncate max-w-xs">{task.outputPath}</dd>
          </div>
          <div className="flex justify-between">
            <dt className="text-muted-foreground">Total Size</dt>
            <dd>{formatSize(task.totalSize)}</dd>
          </div>
          <div className="flex justify-between">
            <dt className="text-muted-foreground">Threads</dt>
            <dd>{task.threads}</dd>
          </div>
          <div className="flex justify-between">
            <dt className="text-muted-foreground">Created</dt>
            <dd>{formatDate(task.createdAt)}</dd>
          </div>
          {task.startedAt && (
            <div className="flex justify-between">
              <dt className="text-muted-foreground">Started</dt>
              <dd>{formatDate(task.startedAt)}</dd>
            </div>
          )}
          {task.completedAt && (
            <div className="flex justify-between">
              <dt className="text-muted-foreground">Completed</dt>
              <dd>{formatDate(task.completedAt)}</dd>
            </div>
          )}
          {task.error && (
            <div className="flex justify-between">
              <dt className="text-muted-foreground text-danger-500">Error</dt>
              <dd className="text-danger-500">{task.error}</dd>
            </div>
          )}
        </dl>
      </div>

      {/* Actions Section */}
      <div className="flex items-center gap-2">
        {task.status === 'pending' && (
          <Button onClick={handleStart} leftIcon={<Play className="h-4 w-4" />}>
            Start
          </Button>
        )}
        {task.status === 'downloading' && (
          <Button
            variant="outline"
            onClick={handlePause}
            leftIcon={<Pause className="h-4 w-4" />}
          >
            Pause
          </Button>
        )}
        {task.status === 'paused' && (
          <Button onClick={handleStart} leftIcon={<Play className="h-4 w-4" />}>
            Resume
          </Button>
        )}
        {task.status === 'failed' && (
          <Button onClick={handleRetry} leftIcon={<RefreshCw className="h-4 w-4" />}>
            Retry
          </Button>
        )}
        {task.status === 'completed' && (
          <Button variant="outline" leftIcon={<FolderOpen className="h-4 w-4" />}>
            Open Folder
          </Button>
        )}
        <Button variant="ghost" onClick={handleCancel}>
          Cancel
        </Button>
        <Button variant="danger" size="sm" onClick={handleDelete}>
          <Trash2 className="h-4 w-4" />
        </Button>
      </div>
    </div>
  )
}
```

---

## 工具函数实现

### 1. 格式化工具 (src/utils/format.ts)

```typescript
// src/utils/format.ts

/**
 * 格式化字节数为可读字符串
 */
export function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B'

  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const k = 1024
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  const size = bytes / Math.pow(k, i)

  return `${size.toFixed(i > 0 ? 2 : 0)} ${units[i]}`
}

/**
 * 格式化速度 (字节/秒)
 */
export function formatSpeed(bytesPerSec: number): string {
  if (bytesPerSec === 0) return '0 B/s'
  return `${formatSize(bytesPerSec)}/s`
}

/**
 * 格式化剩余时间 (秒)
 */
export function formatETA(seconds: number): string {
  if (seconds <= 0) return '--:--:--'

  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = Math.floor(seconds % 60)

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}`
}

/**
 * 格式化时间戳为日期字符串
 */
export function formatDate(timestamp: number): string {
  return new Date(timestamp).toLocaleString()
}

/**
 * 格式化持续时间 (毫秒)
 */
export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`

  const seconds = Math.floor(ms / 1000)
  if (seconds < 60) return `${seconds}s`

  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  if (minutes < 60) return `${minutes}m ${remainingSeconds}s`

  const hours = Math.floor(minutes / 60)
  const remainingMinutes = minutes % 60
  return `${hours}h ${remainingMinutes}m`
}
```

---

### 2. 类名工具 (src/utils/cn.ts)

```typescript
// src/utils/cn.ts

import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

/**
 * 合并 Tailwind CSS 类名
 */
export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs))
}
```

---

### 3. 存储工具 (src/utils/storage.ts)

```typescript
// src/utils/storage.ts

/**
 * 安全的 localStorage 操作
 */
export const storage = {
  get<T>(key: string, defaultValue: T): T {
    try {
      const item = localStorage.getItem(key)
      return item ? JSON.parse(item) : defaultValue
    } catch {
      return defaultValue
    }
  },

  set<T>(key: string, value: T): void {
    try {
      localStorage.setItem(key, JSON.stringify(value))
    } catch (error) {
      console.error('Failed to save to localStorage:', error)
    }
  },

  remove(key: string): void {
    try {
      localStorage.removeItem(key)
    } catch (error) {
      console.error('Failed to remove from localStorage:', error)
    }
  },
}
```

---

## Hooks 实现

### 1. useTheme Hook (src/hooks/useTheme.ts)

```typescript
// src/hooks/useTheme.ts

import { useEffect, useCallback } from 'react'
import { useSettingsStore } from '@/stores/settings/settingsStore'

type Theme = 'light' | 'dark' | 'system'

export function useTheme() {
  const { appearance, updateAppearance } = useSettingsStore()

  const resolvedTheme = (() => {
    if (appearance.theme === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    }
    return appearance.theme
  })()

  const setTheme = useCallback(
    (theme: Theme) => {
      updateAppearance({ theme })
    },
    [updateAppearance]
  )

  useEffect(() => {
    const root = document.documentElement
    root.classList.remove('light', 'dark')
    root.classList.add(resolvedTheme)
  }, [resolvedTheme])

  useEffect(() => {
    if (appearance.theme !== 'system') return

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    const handler = () => {
      const newTheme = mediaQuery.matches ? 'dark' : 'light'
      document.documentElement.classList.remove('light', 'dark')
      document.documentElement.classList.add(newTheme)
    }

    mediaQuery.addEventListener('change', handler)
    return () => mediaQuery.removeEventListener('change', handler)
  }, [appearance.theme])

  return {
    theme: appearance.theme,
    setTheme,
    resolvedTheme,
  }
}
```

---

### 2. useToast Hook (src/hooks/useToast.ts)

```typescript
// src/hooks/useToast.ts

import { useCallback } from 'react'
import { create } from 'zustand'

type ToastType = 'success' | 'error' | 'warning' | 'info'

interface Toast {
  id: string
  type: ToastType
  title: string
  description?: string
  duration?: number
}

interface ToastState {
  toasts: Toast[]
  addToast: (toast: Omit<Toast, 'id'>) => void
  removeToast: (id: string) => void
}

const useToastStore = create<ToastState>((set) => ({
  toasts: [],

  addToast: (toast) => {
    const id = crypto.randomUUID()
    set((state) => ({
      toasts: [...state.toasts, { ...toast, id }],
    }))

    // Auto remove
    const duration = toast.duration ?? 5000
    if (duration > 0) {
      setTimeout(() => {
        set((state) => ({
          toasts: state.toasts.filter((t) => t.id !== id),
        }))
      }, duration)
    }
  },

  removeToast: (id) =>
    set((state) => ({
      toasts: state.toasts.filter((t) => t.id !== id),
    })),
}))

export function useToast() {
  const { toasts, addToast, removeToast } = useToastStore()

  const toast = useCallback(
    (options: Omit<Toast, 'id'>) => {
      addToast(options)
    },
    [addToast]
  )

  return {
    toast,
    toasts,
    removeToast,
    success: (title: string, description?: string) =>
      toast({ type: 'success', title, description }),
    error: (title: string, description?: string) =>
      toast({ type: 'error', title, description }),
    warning: (title: string, description?: string) =>
      toast({ type: 'warning', title, description }),
    info: (title: string, description?: string) =>
      toast({ type: 'info', title, description }),
  }
}
```

---

## 测试用例

### Button 测试 (src/components/common/Button/Button.test.tsx)

```tsx
// src/components/common/Button/Button.test.tsx

import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { Button } from './Button'

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
    render(<Button isLoading>Loading</Button>)
    expect(screen.getByRole('button')).toBeDisabled()
    expect(screen.getByRole('button')).toHaveAttribute('aria-busy', 'true')
  })

  it('is disabled when disabled prop is true', () => {
    render(<Button disabled>Disabled</Button>)
    expect(screen.getByRole('button')).toBeDisabled()
  })

  it.each(['primary', 'secondary', 'outline', 'ghost', 'danger'] as const)(
    'renders %s variant',
    (variant) => {
      render(<Button variant={variant}>Button</Button>)
      expect(screen.getByRole('button')).toBeInTheDocument()
    }
  )

  it.each(['sm', 'md', 'lg'] as const)('renders %s size', (size) => {
    render(<Button size={size}>Button</Button>)
    expect(screen.getByRole('button')).toBeInTheDocument()
  })

  it('renders with left icon', () => {
    render(<Button leftIcon={<span data-testid="left-icon">→</span>}>Button</Button>)
    expect(screen.getByTestId('left-icon')).toBeInTheDocument()
  })

  it('renders with right icon', () => {
    render(<Button rightIcon={<span data-testid="right-icon">→</span>}>Button</Button>)
    expect(screen.getByTestId('right-icon')).toBeInTheDocument()
  })
})
```

---

### Progress 测试 (src/components/common/Progress/Progress.test.tsx)

```tsx
// src/components/common/Progress/Progress.test.tsx

import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { Progress } from './Progress'

describe('Progress', () => {
  it('renders with default props', () => {
    render(<Progress value={50} />)
    expect(screen.getByRole('progressbar')).toBeInTheDocument()
  })

  it('shows label when showLabel is true', () => {
    render(<Progress value={50} showLabel />)
    expect(screen.getByText('50.0%')).toBeInTheDocument()
  })

  it('calculates percentage correctly', () => {
    render(<Progress value={25} max={100} showLabel />)
    expect(screen.getByText('25.0%')).toBeInTheDocument()
  })

  it('clamps value to 0-100', () => {
    const { rerender } = render(<Progress value={150} showLabel />)
    expect(screen.getByText('100.0%')).toBeInTheDocument()

    rerender(<Progress value={-10} showLabel />)
    expect(screen.getByText('0.0%')).toBeInTheDocument()
  })

  it.each(['default', 'success', 'warning', 'danger'] as const)(
    'renders %s variant',
    (variant) => {
      render(<Progress value={50} variant={variant} />)
      expect(screen.getByRole('progressbar')).toBeInTheDocument()
    }
  )
})
```

---

### formatSize 测试 (src/utils/format.test.ts)

```typescript
// src/utils/format.test.ts

import { describe, it, expect } from 'vitest'
import { formatSize, formatSpeed, formatETA, formatDate } from './format'

describe('formatSize', () => {
  it('formats bytes correctly', () => {
    expect(formatSize(0)).toBe('0 B')
    expect(formatSize(100)).toBe('100 B')
    expect(formatSize(1024)).toBe('1.00 KB')
    expect(formatSize(1024 * 1024)).toBe('1.00 MB')
    expect(formatSize(1024 * 1024 * 1024)).toBe('1.00 GB')
  })

  it('handles large values', () => {
    expect(formatSize(1536)).toBe('1.50 KB')
    expect(formatSize(2560000)).toBe('2.44 MB')
  })
})

describe('formatSpeed', () => {
  it('formats speed correctly', () => {
    expect(formatSpeed(0)).toBe('0 B/s')
    expect(formatSpeed(1024)).toBe('1.00 KB/s')
    expect(formatSpeed(1024 * 1024)).toBe('1.00 MB/s')
  })
})

describe('formatETA', () => {
  it('formats ETA correctly', () => {
    expect(formatETA(0)).toBe('--:--:--')
    expect(formatETA(30)).toBe('0:30')
    expect(formatETA(90)).toBe('1:30')
    expect(formatETA(3661)).toBe('1:01:01')
  })
})

describe('formatDate', () => {
  it('formats timestamp correctly', () => {
    const timestamp = Date.UTC(2024, 0, 1, 12, 0, 0)
    const result = formatDate(timestamp)
    expect(result).toBeTruthy()
  })
})
```

---

## 示例代码

### 基础使用示例

```tsx
// App.tsx

import { ThemeProvider } from '@/providers/ThemeProvider'
import { MainLayout } from '@/components/layout/MainLayout/MainLayout'

function App() {
  return (
    <ThemeProvider>
      <MainLayout />
    </ThemeProvider>
  )
}

export default App
```

### 创建任务示例

```tsx
// components/AddTaskModal.tsx

import { useState } from 'react'
import { Button } from '@/components/common/Button'
import { Input } from '@/components/common/Input'
import { Modal } from '@/components/common/Modal'
import { useTaskStore } from '@/stores/task/taskStore'
import { useToast } from '@/hooks/useToast'

interface AddTaskModalProps {
  isOpen: boolean
  onClose: () => void
}

export function AddTaskModal({ isOpen, onClose }: AddTaskModalProps) {
  const [url, setUrl] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const { addTask } = useTaskStore()
  const toast = useToast()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!url.trim()) {
      toast.error('Please enter a URL')
      return
    }

    setIsLoading(true)
    try {
      // Call API to create task
      const response = await fetch('/api/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ url }),
      })

      if (!response.ok) throw new Error('Failed to create task')

      const task = await response.json()
      addTask(task)
      toast.success('Task created successfully')
      onClose()
      setUrl('')
    } catch (error) {
      toast.error('Failed to create task', error.message)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Add Download Task">
      <form onSubmit={handleSubmit} className="space-y-4">
        <Input
          label="Download URL"
          type="url"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="https://example.com/file.zip"
          required
        />

        <div className="flex justify-end gap-2">
          <Button type="button" variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button type="submit" isLoading={isLoading}>
            Add Task
          </Button>
        </div>
      </form>
    </Modal>
  )
}
```