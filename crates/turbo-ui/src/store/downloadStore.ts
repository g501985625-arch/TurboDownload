/**
 * Download Store - Global state management for downloads
 * 
 * Uses Zustand for lightweight state management
 */

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { DownloadTask, DownloadConfig, DownloadProgress } from '../types/download';
import {
  startDownloadWithConfig,
  pauseDownload,
  resumeDownload,
  cancelDownload,
  getDownloadProgress,
  listDownloads,
} from '../api/tauri';

// ============================================================================
// Store State & Actions
// ============================================================================

interface DownloadState {
  // State
  tasks: DownloadTask[];
  activeTaskIds: string[];
  isLoading: boolean;
  error: string | null;
  
  // Settings
  defaultThreads: number;
  defaultDownloadPath: string;
  speedLimit: number;
  
  // Actions
  addTask: (config: DownloadConfig) => Promise<void>;
  pauseTask: (taskId: string) => Promise<void>;
  resumeTask: (taskId: string) => Promise<void>;
  cancelTask: (taskId: string) => Promise<void>;
  removeTask: (taskId: string) => void;
  updateTaskProgress: (taskId: string, progress: DownloadProgress) => void;
  updateTaskStatus: (taskId: string, status: DownloadTask['status']) => void;
  setTaskError: (taskId: string, error: string) => void;
  
  // Bulk actions
  loadTasks: () => Promise<void>;
  refreshProgress: () => Promise<void>;
  clearCompleted: () => void;
  clearAll: () => void;
  
  // Settings
  setDefaultThreads: (threads: number) => void;
  setDefaultDownloadPath: (path: string) => void;
  setSpeedLimit: (limit: number) => void;
  
  // Getters
  getActiveTasks: () => DownloadTask[];
  getCompletedTasks: () => DownloadTask[];
  getTaskById: (taskId: string) => DownloadTask | undefined;
}

// ============================================================================
// Store Implementation
// ============================================================================

export const useDownloadStore = create<DownloadState>()(
  persist(
    (set, get) => ({
      // Initial state
      tasks: [],
      activeTaskIds: [],
      isLoading: false,
      error: null,
      
      // Default settings
      defaultThreads: 4,
      defaultDownloadPath: '',
      speedLimit: 0,
      
      // ----------------------------------------------------------------------
      // Actions
      // ----------------------------------------------------------------------
      
      /**
       * Add a new download task
       */
      addTask: async (config: DownloadConfig) => {
        set({ isLoading: true, error: null });
        
        try {
          const taskId = await startDownloadWithConfig({
            ...config,
            threads: config.threads || get().defaultThreads,
          });
          
          // Create initial task
          const newTask: DownloadTask = {
            id: taskId,
            url: config.url,
            filename: config.output_path.split('/').pop() || 'unknown',
            output_path: config.output_path,
            status: 'downloading',
            progress: {
              total: 0,
              downloaded: 0,
              speed: 0,
              avg_speed: 0,
              eta: null,
              percent: 0,
            },
            threads: config.threads || get().defaultThreads,
            resume_support: config.resume_support ?? true,
            created_at: Date.now(),
          };
          
          set((state) => ({
            tasks: [newTask, ...state.tasks],
            activeTaskIds: [...state.activeTaskIds, taskId],
            isLoading: false,
          }));
        } catch (error) {
          set({
            isLoading: false,
            error: error instanceof Error ? error.message : 'Failed to start download',
          });
        }
      },
      
      /**
       * Pause a download task
       */
      pauseTask: async (taskId: string) => {
        try {
          await pauseDownload(taskId);
          set((state) => ({
            tasks: state.tasks.map((t) =>
              t.id === taskId ? { ...t, status: 'paused' } : t
            ),
            activeTaskIds: state.activeTaskIds.filter((id) => id !== taskId),
          }));
        } catch (error) {
          console.error('Failed to pause task:', error);
        }
      },
      
      /**
       * Resume a download task
       */
      resumeTask: async (taskId: string) => {
        const task = get().tasks.find((t) => t.id === taskId);
        if (!task) return;
        
        try {
          await resumeDownload(taskId, task.output_path);
          set((state) => ({
            tasks: state.tasks.map((t) =>
              t.id === taskId ? { ...t, status: 'downloading' } : t
            ),
            activeTaskIds: [...state.activeTaskIds, taskId],
          }));
        } catch (error) {
          console.error('Failed to resume task:', error);
        }
      },
      
      /**
       * Cancel a download task
       */
      cancelTask: async (taskId: string) => {
        try {
          await cancelDownload(taskId);
          set((state) => ({
            tasks: state.tasks.map((t) =>
              t.id === taskId ? { ...t, status: 'error', error: 'Cancelled by user' } : t
            ),
            activeTaskIds: state.activeTaskIds.filter((id) => id !== taskId),
          }));
        } catch (error) {
          console.error('Failed to cancel task:', error);
        }
      },
      
      /**
       * Remove a task from the list
       */
      removeTask: (taskId: string) => {
        set((state) => ({
          tasks: state.tasks.filter((t) => t.id !== taskId),
          activeTaskIds: state.activeTaskIds.filter((id) => id !== taskId),
        }));
      },
      
      /**
       * Update task progress
       */
      updateTaskProgress: (taskId: string, progress: DownloadProgress) => {
        set((state) => ({
          tasks: state.tasks.map((t) =>
            t.id === taskId ? { ...t, progress } : t
          ),
        }));
      },
      
      /**
       * Update task status
       */
      updateTaskStatus: (taskId: string, status: DownloadTask['status']) => {
        set((state) => {
          const newState: Partial<DownloadState> = {
            tasks: state.tasks.map((t) =>
              t.id === taskId
                ? {
                    ...t,
                    status,
                    completed_at: status === 'completed' ? Date.now() : t.completed_at,
                  }
                : t
            ),
          };
          
          // Update active task IDs
          if (status === 'completed' || status === 'error') {
            newState.activeTaskIds = state.activeTaskIds.filter((id) => id !== taskId);
          }
          
          return newState;
        });
      },
      
      /**
       * Set task error
       */
      setTaskError: (taskId: string, error: string) => {
        set((state) => ({
          tasks: state.tasks.map((t) =>
            t.id === taskId ? { ...t, status: 'error', error } : t
          ),
          activeTaskIds: state.activeTaskIds.filter((id) => id !== taskId),
        }));
      },
      
      /**
       * Load all tasks from backend
       */
      loadTasks: async () => {
        try {
          const taskIds = await listDownloads();
          // TODO: Fetch full task details for each ID
          console.log('Loaded task IDs:', taskIds);
        } catch (error) {
          console.error('Failed to load tasks:', error);
        }
      },
      
      /**
       * Refresh progress for all active tasks
       */
      refreshProgress: async () => {
        const { activeTaskIds } = get();
        
        for (const taskId of activeTaskIds) {
          try {
            const progress = await getDownloadProgress(taskId);
            get().updateTaskProgress(taskId, progress);
            
            // Check if completed
            if (progress.percent >= 100) {
              get().updateTaskStatus(taskId, 'completed');
            }
          } catch (error) {
            console.error(`Failed to get progress for ${taskId}:`, error);
          }
        }
      },
      
      /**
       * Clear all completed tasks
       */
      clearCompleted: () => {
        set((state) => ({
          tasks: state.tasks.filter((t) => t.status !== 'completed'),
        }));
      },
      
      /**
       * Clear all tasks
       */
      clearAll: () => {
        set({
          tasks: [],
          activeTaskIds: [],
        });
      },
      
      // ----------------------------------------------------------------------
      // Settings
      // ----------------------------------------------------------------------
      
      setDefaultThreads: (threads: number) => {
        set({ defaultThreads: threads });
      },
      
      setDefaultDownloadPath: (path: string) => {
        set({ defaultDownloadPath: path });
      },
      
      setSpeedLimit: (limit: number) => {
        set({ speedLimit: limit });
      },
      
      // ----------------------------------------------------------------------
      // Getters
      // ----------------------------------------------------------------------
      
      getActiveTasks: () => {
        return get().tasks.filter((t) => t.status === 'downloading');
      },
      
      getCompletedTasks: () => {
        return get().tasks.filter((t) => t.status === 'completed');
      },
      
      getTaskById: (taskId: string) => {
        return get().tasks.find((t) => t.id === taskId);
      },
    }),
    {
      name: 'download-store',
      partialize: (state) => ({
        defaultThreads: state.defaultThreads,
        defaultDownloadPath: state.defaultDownloadPath,
        speedLimit: state.speedLimit,
      }),
    }
  )
);

// ============================================================================
// Hooks
// ============================================================================

/**
 * Hook to get filtered tasks
 */
export function useFilteredTasks(status?: DownloadTask['status']) {
  const tasks = useDownloadStore((state) => state.tasks);
  
  if (!status) return tasks;
  return tasks.filter((t) => t.status === status);
}

/**
 * Hook to get task statistics
 */
export function useTaskStats() {
  const tasks = useDownloadStore((state) => state.tasks);
  
  return {
    total: tasks.length,
    downloading: tasks.filter((t) => t.status === 'downloading').length,
    paused: tasks.filter((t) => t.status === 'paused').length,
    completed: tasks.filter((t) => t.status === 'completed').length,
    error: tasks.filter((t) => t.status === 'error').length,
  };
}

/**
 * Hook to get total download speed
 */
export function useTotalSpeed() {
  const activeTasks = useDownloadStore((state) =>
    state.tasks.filter((t) => t.status === 'downloading')
  );
  
  return activeTasks.reduce((sum, t) => sum + (t.progress.speed || 0), 0);
}