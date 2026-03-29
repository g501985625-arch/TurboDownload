/**
 * Progress Updater Hook
 * 
 * Automatically polls for download progress updates
 */

import { useEffect, useRef, useCallback } from 'react';
import { useDownloadStore } from '../store/downloadStore';

interface UseProgressUpdaterOptions {
  /** Polling interval in milliseconds (default: 1000) */
  interval?: number;
  /** Whether to enable automatic updates (default: true) */
  enabled?: boolean;
}

/**
 * Hook to automatically update download progress
 * 
 * Usage:
 * ```tsx
 * function App() {
 *   useProgressUpdater({ interval: 1000 });
 *   return <div>...</div>;
 * }
 * ```
 */
export function useProgressUpdater(options: UseProgressUpdaterOptions = {}) {
  const { interval = 1000, enabled = true } = options;
  
  const refreshProgress = useDownloadStore((state) => state.refreshProgress);
  const activeTaskIds = useDownloadStore((state) => state.activeTaskIds);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  
  const updateProgress = useCallback(async () => {
    if (activeTaskIds.length === 0) return;
    await refreshProgress();
  }, [activeTaskIds.length, refreshProgress]);
  
  useEffect(() => {
    if (!enabled) return;
    
    // Initial update
    updateProgress();
    
    // Set up interval
    intervalRef.current = setInterval(updateProgress, interval);
    
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [enabled, interval, updateProgress]);
  
  return {
    isActive: activeTaskIds.length > 0,
    activeCount: activeTaskIds.length,
  };
}

/**
 * Hook to subscribe to a single task's progress
 * 
 * Usage:
 * ```tsx
 * function TaskItem({ taskId }: { taskId: string }) {
 *   const progress = useTaskProgress(taskId);
 *   return <Progress percent={progress?.percent || 0} />;
 * }
 * ```
 */
export function useTaskProgress(taskId: string) {
  return useDownloadStore((state) =>
    state.tasks.find((t) => t.id === taskId)?.progress
  );
}

/**
 * Hook to subscribe to a single task's status
 */
export function useTaskStatus(taskId: string) {
  return useDownloadStore((state) =>
    state.tasks.find((t) => t.id === taskId)?.status
  );
}

/**
 * Hook to get notified when a task completes
 * 
 * Usage:
 * ```tsx
 * function DownloadComponent() {
 *   useTaskCompletion((task) => {
 *     notification.success({ message: `${task.filename} completed!` });
 *   });
 *   return ...;
 * }
 * ```
 */
export function useTaskCompletion(
  onComplete: (task: { id: string; filename: string }) => void
) {
  const completedRef = useRef<Set<string>>(new Set());
  const tasks = useDownloadStore((state) => state.tasks);
  
  useEffect(() => {
    tasks.forEach((task) => {
      if (task.status === 'completed' && !completedRef.current.has(task.id)) {
        completedRef.current.add(task.id);
        onComplete({ id: task.id, filename: task.filename });
      }
    });
  }, [tasks, onComplete]);
}

/**
 * Hook to get notified when a task errors
 */
export function useTaskError(
  onError: (task: { id: string; filename: string; error: string }) => void
) {
  const erroredRef = useRef<Set<string>>(new Set());
  const tasks = useDownloadStore((state) => state.tasks);
  
  useEffect(() => {
    tasks.forEach((task) => {
      if (task.status === 'error' && task.error && !erroredRef.current.has(task.id)) {
        erroredRef.current.add(task.id);
        onError({ id: task.id, filename: task.filename, error: task.error });
      }
    });
  }, [tasks, onError]);
}
