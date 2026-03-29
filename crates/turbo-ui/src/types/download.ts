/**
 * Download task status
 */
export type DownloadStatus = 
  | 'pending'      // Waiting to start
  | 'downloading'  // Currently downloading
  | 'paused'       // Paused by user
  | 'completed'    // Download finished
  | 'error';       // Error occurred

/**
 * Download progress information
 */
export interface DownloadProgress {
  /** Total file size in bytes */
  total: number;
  /** Downloaded bytes */
  downloaded: number;
  /** Current download speed (bytes/second) */
  speed: number;
  /** Average speed (bytes/second) */
  avg_speed: number;
  /** Estimated time of arrival (seconds) */
  eta: number | null;
  /** Download percentage (0-100) */
  percent: number;
}

/**
 * Download task information
 */
export interface DownloadTask {
  /** Unique task ID */
  id: string;
  /** Download URL */
  url: string;
  /** Output filename */
  filename: string;
  /** Output path */
  output_path: string;
  /** Download status */
  status: DownloadStatus;
  /** Current progress */
  progress: DownloadProgress;
  /** Number of download threads */
  threads: number;
  /** Whether resume is supported */
  resume_support: boolean;
  /** Error message if status is 'error' */
  error?: string;
  /** Creation timestamp */
  created_at: number;
  /** Completion timestamp */
  completed_at?: number;
}

/**
 * Download configuration
 */
export interface DownloadConfig {
  /** Unique task ID (auto-generated if not provided) */
  id?: string;
  /** Download URL */
  url: string;
  /** Output file path */
  output_path: string;
  /** Number of download threads (default: 4) */
  threads?: number;
  /** Chunk size in bytes (default: 1MB) */
  chunk_size?: number;
  /** Whether to enable resume support */
  resume_support?: boolean;
  /** User agent string */
  user_agent?: string;
  /** Additional HTTP headers */
  headers?: Record<string, string>;
  /** Speed limit in bytes/second (0 = unlimited) */
  speed_limit?: number;
}

/**
 * Download result
 */
export interface DownloadResult {
  /** Task ID */
  task_id: string;
  /** Whether download was successful */
  success: boolean;
  /** Output file path */
  output_path: string;
  /** File size in bytes */
  file_size: number;
  /** SHA256 checksum */
  checksum?: string;
  /** Error message if failed */
  error?: string;
  /** Download duration in milliseconds */
  duration_ms: number;
}

/**
 * Resource type for radar scanning
 */
export type ResourceType = 
  | 'image' 
  | 'video' 
  | 'audio' 
  | 'document' 
  | 'archive' 
  | 'script' 
  | 'stylesheet' 
  | 'font' 
  | 'html' 
  | 'other';

/**
 * Resource item from radar scan
 */
export interface ResourceItem {
  /** Unique identifier */
  key: string;
  /** Resource name/filename */
  name: string;
  /** Resource type */
  type: ResourceType;
  /** File size (formatted string) */
  size: string;
  /** Resource URL */
  url: string;
  /** Scan status */
  status: 'pending' | 'scanned';
  /** MIME type */
  mime_type?: string;
}

/**
 * Scan result from radar
 */
export interface ScanResult {
  /** Scanned URL */
  url: string;
  /** Found resources */
  resources: ResourceItem[];
  /** Total resources count */
  total_count: number;
  /** Scan duration in milliseconds */
  duration_ms: number;
}

/**
 * Download event types
 */
export type DownloadEventType = 
  | 'progress'     // Progress update
  | 'completed'    // Download completed
  | 'error'        // Error occurred
  | 'paused'       // Download paused
  | 'resumed';     // Download resumed

/**
 * Download event payload
 */
export interface DownloadEvent {
  /** Event type */
  type: DownloadEventType;
  /** Task ID */
  task_id: string;
  /** Event payload (varies by type) */
  payload: DownloadProgress | DownloadResult | { error: string };
  /** Event timestamp */
  timestamp: number;
}
