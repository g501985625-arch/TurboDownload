# P6: turbo-app 代码模板

## 概述

本文档提供核心代码模板，可直接复制使用。

---

## 1. Rust 后端入口模板

### 文件: `src-tauri/src/main.rs`

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    turbo_app::run()
}
```

---

## 2. lib.rs 模块模板

### 文件: `src-tauri/src/lib.rs`

```rust
mod setup;

use tauri::Manager;

/// 应用主入口
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        // 注册插件
        .plugin(tauri_plugin_shell::init())
        // 设置初始化回调
        .setup(|app| {
            setup::init(app)?;
            Ok(())
        })
        // 注册所有命令
        .invoke_handler(turbo_integration::register_commands())
        // 运行应用
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 3. setup.rs 初始化模板

### 文件: `src-tauri/src/setup.rs`

```rust
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use turbo_manager::{DownloadManager, DownloadManagerBuilder};
use turbo_integration::config::AppConfig;

/// 初始化应用
pub fn init(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 获取应用数据目录
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;

    // 加载或创建配置
    let config = AppConfig::load().unwrap_or_default();

    // 创建下载管理器
    let manager = DownloadManagerBuilder::new()
        .max_concurrent(config.max_concurrent_downloads)
        .db_path(app_data_dir.join("tasks.db"))
        .temp_dir(std::env::temp_dir().join("turbo-download"))
        .auto_retry(3)
        .retry_interval(1000)
        .build()?;

    // 启动管理器
    tokio::runtime::Handle::current().block_on(async {
        manager.start().await
    }).map_err(|e| format!("Failed to start manager: {}", e))?;

    // 注入状态
    app.manage(Arc::new(manager) as Arc<dyn DownloadManager>);

    tracing::info!("Application initialized successfully");
    Ok(())
}
```

---

## 4. 前端入口模板

### 文件: `src/main.tsx`

```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/globals.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

---

## 5. App.tsx 根组件模板

### 文件: `src/App.tsx`

```tsx
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import MainLayout from './components/MainLayout';
import DownloadPage from './components/DownloadPage';
import CrawlerPage from './components/CrawlerPage';
import SettingsPage from './components/SettingsPage';

function App() {
  return (
    <BrowserRouter>
      <MainLayout>
        <Routes>
          <Route path="/" element={<DownloadPage />} />
          <Route path="/downloads" element={<DownloadPage />} />
          <Route path="/crawler" element={<CrawlerPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </MainLayout>
    </BrowserRouter>
  );
}

export default App;
```

---

## 6. 主布局组件模板

### 文件: `src/components/MainLayout.tsx`

```tsx
import { ReactNode, useState } from 'react';
import { NavLink } from 'react-router-dom';
import {
  Download,
  Search,
  Settings,
  Menu,
  X
} from 'lucide-react';
import { clsx } from 'clsx';

interface MainLayoutProps {
  children: ReactNode;
}

const navItems = [
  { to: '/downloads', icon: Download, label: '下载' },
  { to: '/crawler', icon: Search, label: '爬虫' },
  { to: '/settings', icon: Settings, label: '设置' },
];

export default function MainLayout({ children }: MainLayoutProps) {
  const [sidebarOpen, setSidebarOpen] = useState(true);

  return (
    <div className="flex h-screen bg-gray-900 text-gray-100">
      {/* 侧边栏 */}
      <aside className={clsx(
        "flex flex-col bg-gray-800 transition-all duration-300",
        sidebarOpen ? "w-64" : "w-16"
      )}>
        {/* Logo */}
        <div className="flex items-center justify-between p-4 border-b border-gray-700">
          {sidebarOpen && (
            <h1 className="text-xl font-bold text-blue-400">TurboDownload</h1>
          )}
          <button
            onClick={() => setSidebarOpen(!sidebarOpen)}
            className="p-1 rounded hover:bg-gray-700"
          >
            {sidebarOpen ? <X size={20} /> : <Menu size={20} />}
          </button>
        </div>

        {/* 导航菜单 */}
        <nav className="flex-1 p-2">
          {navItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              className={({ isActive }) =>
                clsx(
                  "flex items-center gap-3 px-3 py-2 rounded-lg mb-1 transition-colors",
                  isActive
                    ? "bg-blue-600 text-white"
                    : "text-gray-400 hover:bg-gray-700 hover:text-white"
                )
              }
            >
              <item.icon size={20} />
              {sidebarOpen && <span>{item.label}</span>}
            </NavLink>
          ))}
        </nav>
      </aside>

      {/* 主内容区 */}
      <main className="flex-1 overflow-auto">
        {children}
      </main>
    </div>
  );
}
```

---

## 7. 下载页面模板

### 文件: `src/components/DownloadPage.tsx`

```tsx
import { useState, useEffect } from 'react';
import {
  Plus,
  Pause,
  Play,
  X,
  FolderOpen,
  ExternalLink
} from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useDownloadStore } from '../stores/downloadStore';

interface DownloadTask {
  id: string;
  url: string;
  filename: string;
  total_size: number;
  downloaded_size: number;
  state: string;
  speed: number;
  eta: number | null;
}

export default function DownloadPage() {
  const [showAddModal, setShowAddModal] = useState(false);
  const [newUrl, setNewUrl] = useState('');
  const { tasks, setTasks, updateTask } = useDownloadStore();

  // 加载所有任务
  useEffect(() => {
    loadTasks();
  }, []);

  // 监听下载事件
  useEffect(() => {
    const unlistenProgress = listen('download:progress', (event) => {
      const payload = event.payload as any;
      updateTask(payload.task_id, {
        downloaded_size: payload.downloaded,
        total_size: payload.total,
        speed: payload.speed,
      });
    });

    const unlistenCompleted = listen('download:completed', (event) => {
      const payload = event.payload as any;
      updateTask(payload.task_id, { state: 'completed' });
    });

    return () => {
      unlistenProgress.then(fn => fn());
      unlistenCompleted.then(fn => fn());
    };
  }, []);

  async function loadTasks() {
    try {
      const allTasks = await invoke<DownloadTask[]>('get_all_downloads');
      setTasks(allTasks);
    } catch (error) {
      console.error('Failed to load tasks:', error);
    }
  }

  async function addDownload() {
    try {
      const taskId = await invoke<string>('add_download', {
        url: newUrl,
        config: null
      });
      await invoke('start_download', { taskId });
      setNewUrl('');
      setShowAddModal(false);
      loadTasks();
    } catch (error) {
      console.error('Failed to add download:', error);
    }
  }

  async function pauseTask(taskId: string) {
    await invoke('pause_download', { taskId });
    loadTasks();
  }

  async function resumeTask(taskId: string) {
    await invoke('resume_download', { taskId });
    loadTasks();
  }

  async function cancelTask(taskId: string) {
    await invoke('cancel_download', { taskId });
    loadTasks();
  }

  return (
    <div className="p-6">
      {/* 标题栏 */}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold">下载任务</h2>
        <button
          onClick={() => setShowAddModal(true)}
          className="flex items-center gap-2 px-4 py-2 bg-blue-600 rounded-lg hover:bg-blue-700"
        >
          <Plus size={20} />
          添加下载
        </button>
      </div>

      {/* 任务列表 */}
      <div className="space-y-3">
        {tasks.map((task) => (
          <TaskCard
            key={task.id}
            task={task}
            onPause={() => pauseTask(task.id)}
            onResume={() => resumeTask(task.id)}
            onCancel={() => cancelTask(task.id)}
          />
        ))}

        {tasks.length === 0 && (
          <div className="text-center text-gray-500 py-12">
            暂无下载任务，点击"添加下载"开始
          </div>
        )}
      </div>

      {/* 添加下载模态框 */}
      {showAddModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center">
          <div className="bg-gray-800 rounded-xl p-6 w-full max-w-lg">
            <h3 className="text-xl font-bold mb-4">添加下载</h3>
            <input
              type="text"
              value={newUrl}
              onChange={(e) => setNewUrl(e.target.value)}
              placeholder="输入下载链接"
              className="w-full px-4 py-2 bg-gray-700 rounded-lg border border-gray-600 focus:border-blue-500 outline-none"
            />
            <div className="flex justify-end gap-3 mt-4">
              <button
                onClick={() => setShowAddModal(false)}
                className="px-4 py-2 text-gray-400 hover:text-white"
              >
                取消
              </button>
              <button
                onClick={addDownload}
                className="px-4 py-2 bg-blue-600 rounded-lg hover:bg-blue-700"
              >
                开始下载
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

// 任务卡片组件
function TaskCard({ task, onPause, onResume, onCancel }: {
  task: DownloadTask;
  onPause: () => void;
  onResume: () => void;
  onCancel: () => void;
}) {
  const progress = task.total_size > 0
    ? (task.downloaded_size / task.total_size) * 100
    : 0;

  return (
    <div className="bg-gray-800 rounded-lg p-4">
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1 min-w-0">
          <h4 className="font-medium truncate">{task.filename}</h4>
          <p className="text-sm text-gray-400 truncate">{task.url}</p>
        </div>
        <div className="flex items-center gap-2 ml-4">
          {task.state === 'downloading' && (
            <button onClick={onPause} className="p-1 hover:bg-gray-700 rounded">
              <Pause size={18} />
            </button>
          )}
          {task.state === 'paused' && (
            <button onClick={onResume} className="p-1 hover:bg-gray-700 rounded">
              <Play size={18} />
            </button>
          )}
          <button onClick={onCancel} className="p-1 hover:bg-gray-700 rounded text-red-400">
            <X size={18} />
          </button>
        </div>
      </div>

      {/* 进度条 */}
      <div className="relative h-2 bg-gray-700 rounded-full overflow-hidden mb-2">
        <div
          className="absolute h-full bg-blue-500 transition-all"
          style={{ width: `${progress}%` }}
        />
      </div>

      {/* 状态信息 */}
      <div className="flex items-center justify-between text-sm text-gray-400">
        <span>{formatSize(task.downloaded_size)} / {formatSize(task.total_size)}</span>
        <span>{formatSpeed(task.speed)}</span>
        {task.eta && <span>剩余 {formatTime(task.eta)}</span>}
      </div>
    </div>
  );
}

// 格式化大小
function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// 格式化速度
function formatSpeed(bytesPerSec: number): string {
  return formatSize(bytesPerSec) + '/s';
}

// 格式化时间
function formatTime(seconds: number): string {
  if (seconds < 60) return `${seconds}秒`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}分`;
  return `${Math.floor(seconds / 3600)}小时`;
}
```

---

## 8. 状态管理模板

### 文件: `src/stores/downloadStore.ts`

```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface DownloadTask {
  id: string;
  url: string;
  filename: string;
  total_size: number;
  downloaded_size: number;
  state: string;
  speed: number;
  eta: number | null;
}

interface DownloadStore {
  tasks: DownloadTask[];
  
  // Actions
  setTasks: (tasks: DownloadTask[]) => void;
  addTask: (task: DownloadTask) => void;
  updateTask: (id: string, updates: Partial<DownloadTask>) => void;
  removeTask: (id: string) => void;
  
  // Selectors
  getActiveTasks: () => DownloadTask[];
  getCompletedTasks: () => DownloadTask[];
}

export const useDownloadStore = create<DownloadStore>()(
  persist(
    (set, get) => ({
      tasks: [],
      
      setTasks: (tasks) => set({ tasks }),
      
      addTask: (task) => set((state) => ({
        tasks: [...state.tasks, task]
      })),
      
      updateTask: (id, updates) => set((state) => ({
        tasks: state.tasks.map((t) =>
          t.id === id ? { ...t, ...updates } : t
        )
      })),
      
      removeTask: (id) => set((state) => ({
        tasks: state.tasks.filter((t) => t.id !== id)
      })),
      
      getActiveTasks: () => get().tasks.filter(
        (t) => t.state === 'downloading' || t.state === 'queued'
      ),
      
      getCompletedTasks: () => get().tasks.filter(
        (t) => t.state === 'completed'
      ),
    }),
    {
      name: 'download-store',
    }
  )
);
```

---

## 9. Tauri 命令 Hooks 模板

### 文件: `src/hooks/useTauriCommands.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';

// 类型定义
interface DownloadConfig {
  output_path?: string;
  threads?: number;
  chunk_size?: number;
  resume_support?: boolean;
  user_agent?: string;
  speed_limit?: number;
}

interface DownloadProgress {
  task_id: string;
  downloaded: number;
  total: number;
  speed: number;
  eta: number | null;
  state: string;
}

// 下载命令
export async function addDownload(url: string, config?: DownloadConfig): Promise<string> {
  return invoke<string>('add_download', { url, config });
}

export async function startDownload(taskId: string): Promise<void> {
  return invoke('start_download', { taskId });
}

export async function pauseDownload(taskId: string): Promise<void> {
  return invoke('pause_download', { taskId });
}

export async function resumeDownload(taskId: string): Promise<void> {
  return invoke('resume_download', { taskId });
}

export async function cancelDownload(taskId: string): Promise<void> {
  return invoke('cancel_download', { taskId });
}

export async function getDownloadProgress(taskId: string): Promise<DownloadProgress | null> {
  return invoke<DownloadProgress | null>('get_download_progress', { taskId });
}

export async function getAllDownloads(): Promise<any[]> {
  return invoke('get_all_downloads');
}

// 系统命令
export async function selectDirectory(): Promise<string | null> {
  return invoke<string | null>('select_directory');
}

export async function getDefaultDownloadDir(): Promise<string> {
  return invoke<string>('get_default_download_dir');
}

export async function showNotification(title: string, body: string): Promise<void> {
  return invoke('show_notification', { title, body });
}

// 配置命令
export async function getConfig(): Promise<any> {
  return invoke('get_config');
}

export async function saveConfig(config: any): Promise<void> {
  return invoke('save_config', { config });
}
```

---

## 10. Vite 配置模板

### 文件: `vite.config.ts`

```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  // Tauri 开发服务器配置
  server: {
    port: 5173,
    strictPort: true,
  },
  // 构建配置
  build: {
    target: 'esnext',
    minify: 'esbuild',
    sourcemap: false,
  },
});
```

---

## 11. Tailwind 配置模板

### 文件: `tailwind.config.js`

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eff6ff',
          100: '#dbeafe',
          200: '#bfdbfe',
          300: '#93c5fd',
          400: '#60a5fa',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
          800: '#1e40af',
          900: '#1e3a8a',
        },
      },
    },
  },
  plugins: [],
}
```

---

## 12. tauri.conf.json 完整配置

### 文件: `src-tauri/tauri.conf.json`

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "TurboDownload",
  "version": "0.1.0",
  "identifier": "com.turbodownload.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "TurboDownload",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false,
        "center": true
      }
    ],
    "security": {
      "csp": null
    },
    "trayIcon": {
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "category": "Utility",
    "shortDescription": "High-performance download manager",
    "longDescription": "TurboDownload is a modern download manager with multi-threaded downloads, resume support, and web resource crawling.",
    "macOS": {
      "minimumSystemVersion": "10.13",
      "entitlements": null,
      "exceptionDomain": "",
      "frameworks": [],
      "providerShortName": null,
      "signingIdentity": null
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  },
  "plugins": {
    "shell": {
      "open": true
    }
  }
}
```