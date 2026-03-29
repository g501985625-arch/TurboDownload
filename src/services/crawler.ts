/**
 * Crawler Service
 * 
 * Frontend service for web crawling operations
 */

import { invoke } from '@tauri-apps/api/core';
import type { CrawlResult, ApiResult } from '../types';

// Helper to unwrap API result
async function unwrapApiResult<T>(result: ApiResult<T>): Promise<T> {
  if ('Ok' in result) {
    return result.Ok;
  }
  throw new Error(result.Err.message);
}

/**
 * Crawler service for web resource extraction
 */
export const crawlerService = {
  /**
   * Crawl a URL and extract resources
   */
  async crawlUrl(url: string): Promise<CrawlResult> {
    const result = await invoke<ApiResult<CrawlResult>>('crawl_url', { url });
    return unwrapApiResult(result);
  },

  /**
   * Crawl a URL with specified depth
   */
  async crawlUrlWithDepth(url: string, depth: number): Promise<CrawlResult> {
    const result = await invoke<ApiResult<CrawlResult>>('crawl_url_with_depth', { 
      url, 
      depth 
    });
    return unwrapApiResult(result);
  },

  /**
   * Get resource type icon name
   */
  getResourceIcon(type: string): string {
    switch (type) {
      case 'image':
        return 'image';
      case 'video':
        return 'film';
      case 'audio':
        return 'music';
      case 'document':
        return 'file-text';
      case 'archive':
        return 'archive';
      case 'executable':
        return 'package';
      default:
        return 'file';
    }
  },

  /**
   * Format file size for display
   */
  formatFileSize(bytes: number | null): string {
    if (bytes === null || bytes === 0) return 'Unknown';
    
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  },

  /**
   * Filter resources by type
   */
  filterByType(resources: CrawlResult['resources'], types: string[]): CrawlResult['resources'] {
    if (types.length === 0) return resources;
    return resources.filter(r => types.includes(r.resource_type));
  },

  /**
   * Sort resources by URL
   */
  sortByUrl(resources: CrawlResult['resources']): CrawlResult['resources'] {
    return [...resources].sort((a, b) => a.url.localeCompare(b.url));
  },

  /**
   * Sort resources by type
   */
  sortByType(resources: CrawlResult['resources']): CrawlResult['resources'] {
    const typeOrder = ['video', 'audio', 'image', 'document', 'archive', 'executable', 'other'];
    return [...resources].sort((a, b) => {
      const aIndex = typeOrder.indexOf(a.resource_type);
      const bIndex = typeOrder.indexOf(b.resource_type);
      return aIndex - bIndex;
    });
  },

  /**
   * Extract unique resource types from results
   */
  getUniqueTypes(resources: CrawlResult['resources']): string[] {
    const types = new Set(resources.map(r => r.resource_type));
    return Array.from(types);
  },

  /**
   * Generate batch download URLs
   */
  generateBatchUrls(resources: CrawlResult['resources'], types?: string[]): string[] {
    const filtered = types ? this.filterByType(resources, types) : resources;
    return filtered.map(r => r.url);
  }
};

export default crawlerService;