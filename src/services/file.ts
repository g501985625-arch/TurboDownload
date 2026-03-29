/**
 * File Service
 * 
 * Frontend service for file system operations
 */

import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import type { ApiResult } from '../types';

// Helper to unwrap API result
async function unwrapApiResult<T>(result: ApiResult<T>): Promise<T> {
  if ('Ok' in result) {
    return result.Ok;
  }
  throw new Error(result.Err.message);
}

/**
 * File service for file system operations
 */
export const fileService = {
  /**
   * Open a directory picker dialog
   */
  async selectDirectory(): Promise<string | null> {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Download Directory'
    });
    
    if (selected && typeof selected === 'string') {
      return selected;
    }
    return null;
  },

  /**
   * Get the default download directory
   */
  async getDefaultDownloadDir(): Promise<string> {
    const result = await invoke<ApiResult<string>>('get_default_download_dir');
    return unwrapApiResult(result);
  },

  /**
   * Check if a file exists
   */
  async fileExists(path: string): Promise<boolean> {
    return await invoke<boolean>('file_exists', { path });
  }
};

export default fileService;