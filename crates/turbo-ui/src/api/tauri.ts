/**
 * Tauri API bindings for TurboDownload
 * 
 * This module provides TypeScript wrappers around Rust Tauri commands
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  DownloadConfig,
  DownloadProgress,
  DownloadResult,
  ScanResult,
  ResourceItem,
} from '../types/download';

// ============================================================================
// Download Commands
// ============================================================================

/**
 * Start a new download
 * 
 * @param url - Download URL
 * @param outputPath - Output file path
 * @param threads - Number of download threads (default: 4)
 * @returns Task ID for tracking
 */
export async function startDownload(
  url: string,
  outputPath: string,
  threads: number = 4
): Promise<string> {
  return await invoke('start_download', {
    url,
    outputPath,
    threads,
  });
}

/**
 * Start a download with full configuration
 * 
 * @param config - Download configuration
 * @returns Task ID for tracking
 */
export async function startDownloadWithConfig(
  config: DownloadConfig
): Promise<string> {
  return await invoke('start_download_with_config', { config });
}

/**
 * Pause a download
 * 
 * @param taskId - Task ID to pause
 */
export async function pauseDownload(taskId: string): Promise<void> {
  return await invoke('pause_download', { taskId });
}

/**
 * Resume a download
 * 
 * @param taskId - Task ID to resume
 * @param outputPath - Output file path
 * @returns Task ID
 */
export async function resumeDownload(
  taskId: string,
  outputPath: string
): Promise<string> {
  return await invoke('resume_download', { taskId, outputPath });
}

/**
 * Cancel a download
 * 
 * @param taskId - Task ID to cancel
 */
export async function cancelDownload(taskId: string): Promise<void> {
  return await invoke('cancel_download', { taskId });
}

/**
 * Get download progress
 * 
 * @param taskId - Task ID
 * @returns Current progress information
 */
export async function getDownloadProgress(
  taskId: string
): Promise<DownloadProgress> {
  return await invoke('get_progress', { taskId });
}

/**
 * List all active downloads
 * 
 * @returns Array of task IDs
 */
export async function listDownloads(): Promise<string[]> {
  return await invoke('list_downloads');
}

/**
 * Get download result
 * 
 * @param taskId - Task ID
 * @returns Download result
 */
export async function getDownloadResult(
  taskId: string
): Promise<DownloadResult> {
  return await invoke('get_download_result', { taskId });
}

// ============================================================================
// Radar/Crawler Commands
// ============================================================================

/**
 * Scan a URL for downloadable resources
 * 
 * @param url - URL to scan
 * @returns Scan result with resources
 */
export async function scanUrl(url: string): Promise<ScanResult> {
  return await invoke('scan_url', { url });
}

/**
 * Get resource list from a scanned URL
 * 
 * @param url - Scanned URL
 * @returns Array of resources
 */
export async function getResourceList(url: string): Promise<ResourceItem[]> {
  return await invoke('get_resource_list', { url });
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Format bytes to human-readable string
 * 
 * @param bytes - Number of bytes
 * @param decimals - Number of decimal places (default: 2)
 * @returns Formatted string (e.g., "1.5 MB")
 */
export function formatBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

/**
 * Format speed to human-readable string
 * 
 * @param bytesPerSecond - Speed in bytes/second
 * @returns Formatted string (e.g., "1.5 MB/s")
 */
export function formatSpeed(bytesPerSecond: number): string {
  return formatBytes(bytesPerSecond) + '/s';
}

/**
 * Format duration to human-readable string
 * 
 * @param seconds - Duration in seconds
 * @returns Formatted string (e.g., "2h 30m")
 */
export function formatDuration(seconds: number): string {
  if (seconds < 60) return `${Math.round(seconds)}s`;
  if (seconds < 3600) return `${Math.round(seconds / 60)}m`;
  
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.round((seconds % 3600) / 60);
  return `${hours}h ${minutes}m`;
}

/**
 * Calculate ETA
 * 
 * @param remainingBytes - Remaining bytes to download
 * @param speedBytesPerSecond - Current download speed
 * @returns ETA in seconds or null if speed is 0
 */
export function calculateETA(
  remainingBytes: number,
  speedBytesPerSecond: number
): number | null {
  if (speedBytesPerSecond <= 0) return null;
  return Math.round(remainingBytes / speedBytesPerSecond);
}

// ============================================================================
// Error Handling
// ============================================================================

/**
 * Tauri command error
 */
export class TauriError extends Error {
  constructor(
    message: string,
    public code?: string,
    public cause?: unknown
  ) {
    super(message);
    this.name = 'TauriError';
  }
}

/**
 * Wrap invoke call with error handling
 * 
 * @param command - Command name
 * @param args - Command arguments
 * @returns Result
 */
export async function invokeWithError<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await invoke(command, args);
  } catch (error) {
    console.error(`Command '${command}' failed:`, error);
    throw new TauriError(
      `Failed to execute '${command}'`,
      'INVOKE_ERROR',
      error
    );
  }
}
