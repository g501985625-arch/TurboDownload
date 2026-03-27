/**
 * Custom hook for Tauri integration
 */

import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import type { DownloadProgress } from '../types';
import { useDownloadStore } from '../stores/downloadStore';

/**
 * Hook to listen for download progress events
 */
export function useDownloadProgress() {
  const updateProgress = useDownloadStore(state => state.updateProgress);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen<DownloadProgress>('download-progress', (event) => {
        updateProgress(event.payload.id, event.payload);
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [updateProgress]);
}

/**
 * Hook to manage download lifecycle
 */
export function useDownloadLifecycle() {
  const refreshTasks = useDownloadStore(state => state.refreshTasks);

  useEffect(() => {
    // Refresh tasks on mount
    refreshTasks();
  }, [refreshTasks]);
}

export default {
  useDownloadProgress,
  useDownloadLifecycle
};