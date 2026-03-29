/**
 * Type definitions for TurboDownload
 * 
 * These types mirror the Rust models for consistency
 */

// Download Status Enum
export type DownloadStatus = 
  | 'pending'
  | 'downloading'
  | 'paused'
  | 'completed'
  | 'failed'
  | 'cancelled';

// Resource Type Enum
export type ResourceType = 
  | 'image'
  | 'video'
  | 'audio'
  | 'document'
  | 'archive'
  | 'executable'
  | 'other';

// Download Configuration
export interface DownloadConfig {
  filename?: string;
  output_dir?: string;
  connections: number;
  max_speed: number;
  headers: Record<string, string>;
}

// Download Progress
export interface DownloadProgress {
  id: string;
  progress: number;
  speed: number;
  total_size: number;
  downloaded: number;
  eta: number | null;
  status: DownloadStatus;
}

// Download Task
export interface DownloadTask {
  id: string;
  url: string;
  filename: string;
  output_dir: string;
  status: DownloadStatus;
  progress: number;
  speed: number;
  total_size: number;
  downloaded: number;
  error: string | null;
  created_at: string;
  completed_at: string | null;
  config: DownloadConfig;
}

// Resource
export interface Resource {
  url: string;
  resource_type: ResourceType;
  title: string | null;
  size: number | null;
  mime_type: string | null;
}

// Crawl Result
export interface CrawlResult {
  source_url: string;
  resources: Resource[];
  title: string | null;
  depth: number;
  crawled_at: string;
}

// Application Error
export interface AppError {
  type: string;
  message: string;
}

// API Result type
export type ApiResult<T> = 
  | { Ok: T }
  | { Err: AppError };

// Helper to check if result is Ok
export function isOk<T>(result: ApiResult<T>): result is { Ok: T } {
  return 'Ok' in result;
}

// Helper to check if result is Err
export function isErr<T>(result: ApiResult<T>): result is { Err: AppError } {
  return 'Err' in result;
}

// Helper to unwrap result
export function unwrapResult<T>(result: ApiResult<T>): T {
  if (isOk(result)) {
    return result.Ok;
  }
  throw new Error(result.Err.message);
}