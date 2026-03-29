/**
 * Download Store
 * 
 * Global state management for downloads using Zustand
 */

import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { DownloadTask, DownloadConfig, DownloadProgress, ApiResult } from '../types';

// Helper to unwrap API result
async function unwrapApiResult<T>(result: ApiResult<T>): Promise<T> {
  if ('Ok' in result) {
    return result.Ok;
  }
  throw new Error(result.Err.message);
}

interface DownloadState {
  // State
  tasks: DownloadTask[];
  isLoading: boolean;
  error: string | null;
  selectedTaskId: string | null;

  // Actions
  addTask: (url: string, config?: DownloadConfig) => Promise<string>;
  startTask: (taskId: string) => Promise<void>;
  pauseTask: (taskId: string) => Promise<void>;
  resumeTask: (taskId: string) => Promise<void>;
  cancelTask: (taskId: string) => Promise<void>;
  removeTask: (taskId: string) => Promise<void>;
  refreshTasks: () => Promise<void>;
  selectTask: (taskId: string | null) => void;
  clearError: () => void;

  // Progress tracking
  updateProgress: (taskId: string, progress: DownloadProgress) => void;
}

export const useDownloadStore = create<DownloadState>((set, get) => ({
  // Initial state
  tasks: [],
  isLoading: false,
  error: null,
  selectedTaskId: null,

  // Add a new download task
  addTask: async (url: string, config?: DownloadConfig) => {
    set({ isLoading: true, error: null });
    try {
      const result = await invoke<ApiResult<string>>('add_download', { 
        url, 
        config: config || null 
      });
      const taskId = await unwrapApiResult(result);
      
      // Refresh tasks to get the new task
      await get().refreshTasks();
      
      return taskId;
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  // Start a download task
  startTask: async (taskId: string) => {
    set({ error: null });
    try {
      const result = await invoke<ApiResult<void>>('start_download', { taskId });
      await unwrapApiResult(result);
      
      // Update task status locally
      set(state => ({
        tasks: state.tasks.map(t => 
          t.id === taskId ? { ...t, status: 'downloading' as const } : t
        )
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ error: message });
      throw error;
    }
  },

  // Pause a download task
  pauseTask: async (taskId: string) => {
    set({ error: null });
    try {
      const result = await invoke<ApiResult<void>>('pause_download', { taskId });
      await unwrapApiResult(result);
      
      set(state => ({
        tasks: state.tasks.map(t => 
          t.id === taskId ? { ...t, status: 'paused' as const } : t
        )
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ error: message });
      throw error;
    }
  },

  // Resume a paused task
  resumeTask: async (taskId: string) => {
    set({ error: null });
    try {
      const result = await invoke<ApiResult<void>>('resume_download', { taskId });
      await unwrapApiResult(result);
      
      set(state => ({
        tasks: state.tasks.map(t => 
          t.id === taskId ? { ...t, status: 'downloading' as const } : t
        )
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ error: message });
      throw error;
    }
  },

  // Cancel a download task
  cancelTask: async (taskId: string) => {
    set({ error: null });
    try {
      const result = await invoke<ApiResult<void>>('cancel_download', { taskId });
      await unwrapApiResult(result);
      
      set(state => ({
        tasks: state.tasks.map(t => 
          t.id === taskId ? { ...t, status: 'cancelled' as const } : t
        )
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ error: message });
      throw error;
    }
  },

  // Remove a download task
  removeTask: async (taskId: string) => {
    set({ error: null });
    try {
      const result = await invoke<ApiResult<void>>('remove_download', { taskId });
      await unwrapApiResult(result);
      
      set(state => ({
        tasks: state.tasks.filter(t => t.id !== taskId),
        selectedTaskId: state.selectedTaskId === taskId ? null : state.selectedTaskId
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ error: message });
      throw error;
    }
  },

  // Refresh all tasks from backend
  refreshTasks: async () => {
    set({ isLoading: true, error: null });
    try {
      const result = await invoke<ApiResult<DownloadTask[]>>('get_all_downloads');
      const tasks = await unwrapApiResult(result);
      set({ tasks, isLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ error: message, isLoading: false });
    }
  },

  // Select a task
  selectTask: (taskId: string | null) => {
    set({ selectedTaskId: taskId });
  },

  // Clear error
  clearError: () => {
    set({ error: null });
  },

  // Update progress for a task
  updateProgress: (taskId: string, progress: DownloadProgress) => {
    set(state => ({
      tasks: state.tasks.map(t => 
        t.id === taskId 
          ? { 
              ...t, 
              progress: progress.progress,
              speed: progress.speed,
              total_size: progress.total_size,
              downloaded: progress.downloaded,
              status: progress.status
            } 
          : t
      )
    }));
  }
}));

// Export for convenience
export type { DownloadState };