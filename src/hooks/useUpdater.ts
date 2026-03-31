import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { isNewerVersion, shouldSkipVersion } from '../utils/version';

export interface UpdateInfo {
  version: string;
  currentVersion: string;
  notes: string;
  date: string;
  downloadUrl?: string;
}

export interface UpdateProgressEvent {
  percent: number;
  downloaded: number;
  total?: number;
  status: 'downloading' | 'completed' | 'error';
}

export function useUpdater() {
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [isChecking, setIsChecking] = useState(false);
  const [isDownloading, setIsDownloading] = useState(false);
  const [progress, setProgress] = useState<UpdateProgressEvent>({
    percent: 0,
    downloaded: 0,
    status: 'downloading'
  });
  const [showUpdateNotification, setShowUpdateNotification] = useState(false);
  const [showUpdateProgress, setShowUpdateProgress] = useState(false);

  // Listen for update download progress events
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen<UpdateProgressEvent>('update-download-progress', (event) => {
        setProgress(event.payload);
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const checkForUpdates = async (skippedVersions: string[] = []): Promise<boolean> => {
    setIsChecking(true);
    try {
      const result: UpdateInfo | null = await invoke('check_update');
      if (result) {
        // Check if the available version is newer than current and not skipped
        if (isNewerVersion(result.version, result.currentVersion) && !shouldSkipVersion(result.version, skippedVersions)) {
          setUpdateInfo(result);
          setShowUpdateNotification(true);
          return true;
        } else {
          // Version is not newer or was skipped
          return false;
        }
      } else {
        return false;
      }
    } catch (error) {
      console.error('Error checking for updates:', error);
      return false;
    } finally {
      setIsChecking(false);
    }
  };

  const downloadUpdate = async () => {
    if (!updateInfo) return;

    setIsDownloading(true);
    setProgress({
      percent: 0,
      downloaded: 0,
      status: 'downloading'
    });
    setShowUpdateProgress(true);

    try {
      await invoke('download_update');
    } catch (error) {
      console.error('Error downloading update:', error);
      setProgress(prev => ({
        ...prev,
        status: 'error'
      }));
    } finally {
      setIsDownloading(false);
    }
  };

  const installUpdate = async () => {
    try {
      await invoke('install_update');
    } catch (error) {
      console.error('Error installing update:', error);
    }
  };

  const getCurrentVersion = async (): Promise<string> => {
    try {
      return await invoke('get_current_version');
    } catch (error) {
      console.error('Error getting current version:', error);
      return 'unknown';
    }
  };

  return {
    updateInfo,
    isChecking,
    isDownloading,
    progress,
    showUpdateNotification,
    showUpdateProgress,
    setShowUpdateNotification,
    setShowUpdateProgress,
    checkForUpdates,
    downloadUpdate,
    installUpdate,
    getCurrentVersion
  };
}