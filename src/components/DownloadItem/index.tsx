/**
 * DownloadItem Component
 * 
 * Displays a single download task with controls
 */

import React from 'react';
import {
  Play,
  Pause,
  X,
  Trash2,
  CheckCircle,
  AlertCircle,
  Clock,
  Download,
  File,
  ChevronDown,
  ChevronUp,
  FolderOpen
} from 'lucide-react';
import type { DownloadTask } from '../../types';
import { useDownloadStore } from '../../stores/downloadStore';

interface DownloadItemProps {
  task: DownloadTask;
  isSelected: boolean;
  onSelect: () => void;
}

// Format file size
function formatSize(bytes: number): string {
  if (bytes === 0) return 'Unknown';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = bytes;
  let unitIndex = 0;
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }
  return `${size.toFixed(1)} ${units[unitIndex]}`;
}

// Format speed
function formatSpeed(bytesPerSec: number): string {
  return `${formatSize(bytesPerSec)}/s`;
}

// Format time
function formatETA(seconds: number | null): string {
  if (seconds === null || seconds <= 0) return '--:--';
  
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;
  
  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}`;
}

// Get status color and icon
function getStatusInfo(status: DownloadTask['status']): { color: string; icon: React.ReactNode; label: string } {
  switch (status) {
    case 'pending':
      return { color: 'text-yellow-500', icon: <Clock size={16} />, label: 'Pending' };
    case 'downloading':
      return { color: 'text-blue-500', icon: <Download size={16} className="animate-bounce" />, label: 'Downloading' };
    case 'paused':
      return { color: 'text-orange-500', icon: <Pause size={16} />, label: 'Paused' };
    case 'completed':
      return { color: 'text-green-500', icon: <CheckCircle size={16} />, label: 'Completed' };
    case 'failed':
      return { color: 'text-red-500', icon: <AlertCircle size={16} />, label: 'Failed' };
    case 'cancelled':
      return { color: 'text-slate-500', icon: <X size={16} />, label: 'Cancelled' };
    default:
      return { color: 'text-slate-500', icon: <Clock size={16} />, label: 'Unknown' };
  }
}

const DownloadItem: React.FC<DownloadItemProps> = ({ task, isSelected, onSelect: _onSelect }) => {
  const [showDetails, setShowDetails] = React.useState(false);
  
  const { startTask, pauseTask, resumeTask, cancelTask, removeTask } = useDownloadStore();
  
  const statusInfo = getStatusInfo(task.status);
  const progress = Math.min(100, Math.max(0, task.progress));
  const isDownloading = task.status === 'downloading';
  const isPaused = task.status === 'paused';
  const isPending = task.status === 'pending';
  const isCompleted = task.status === 'completed';
  const isFailed = task.status === 'failed';
  const isCancelled = task.status === 'cancelled';
  const isActive = isDownloading || isPaused || isPending;
  const isDone = isCompleted || isFailed || isCancelled;

  const handleStart = () => startTask(task.id);
  const handlePause = () => pauseTask(task.id);
  const handleResume = () => resumeTask(task.id);
  const handleCancel = () => cancelTask(task.id);
  const handleRemove = () => removeTask(task.id);

  return (
    <div
      className={`
        bg-slate-800 border rounded-lg overflow-hidden transition-all
        ${isSelected ? 'border-blue-500 ring-1 ring-blue-500/30' : 'border-slate-700 hover:border-slate-600'}
      `}
    >
      {/* Main Row */}
      <div className="p-4">
        <div className="flex items-start gap-4">
          {/* File Icon */}
          <div className="p-2 bg-slate-700 rounded-lg">
            <File size={24} className="text-slate-400" />
          </div>

          {/* Info */}
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2">
              <h3 className="font-medium text-white truncate" title={task.filename}>
                {task.filename}
              </h3>
              <span className={`flex items-center gap-1 text-xs ${statusInfo.color}`}>
                {statusInfo.icon}
                {statusInfo.label}
              </span>
            </div>
            
            <p className="text-sm text-slate-400 truncate" title={task.url}>
              {task.url}
            </p>

            {/* Progress Bar */}
            {(isActive || isCompleted) && (
              <div className="mt-2">
                <div className="flex items-center justify-between text-xs text-slate-400 mb-1">
                  <span>
                    {formatSize(task.downloaded)} / {task.total_size > 0 ? formatSize(task.total_size) : 'Unknown'}
                  </span>
                  <span>{progress.toFixed(1)}%</span>
                </div>
                <div className="h-1.5 bg-slate-700 rounded-full overflow-hidden">
                  <div
                    className={`h-full rounded-full transition-all duration-300 ${
                      isCompleted ? 'bg-green-500' :
                      isFailed ? 'bg-red-500' :
                      isPaused ? 'bg-orange-500' :
                      'bg-blue-500'
                    }`}
                    style={{ width: `${progress}%` }}
                  />
                </div>
              </div>
            )}

            {/* Speed & ETA */}
            {isDownloading && (
              <div className="flex items-center gap-4 mt-2 text-xs text-slate-400">
                <span>Speed: {formatSpeed(task.speed)}</span>
                <span>ETA: {formatETA(null)}</span>
              </div>
            )}

            {/* Error message */}
            {isFailed && task.error && (
              <p className="mt-2 text-xs text-red-400">{task.error}</p>
            )}
          </div>

          {/* Actions */}
          <div className="flex items-center gap-1">
            {isPending && (
              <button
                onClick={handleStart}
                className="p-2 text-green-400 hover:bg-green-500/20 rounded-lg transition-colors"
                title="Start"
              >
                <Play size={18} />
              </button>
            )}
            
            {isDownloading && (
              <button
                onClick={handlePause}
                className="p-2 text-orange-400 hover:bg-orange-500/20 rounded-lg transition-colors"
                title="Pause"
              >
                <Pause size={18} />
              </button>
            )}
            
            {isPaused && (
              <button
                onClick={handleResume}
                className="p-2 text-green-400 hover:bg-green-500/20 rounded-lg transition-colors"
                title="Resume"
              >
                <Play size={18} />
              </button>
            )}
            
            {isActive && (
              <button
                onClick={handleCancel}
                className="p-2 text-red-400 hover:bg-red-500/20 rounded-lg transition-colors"
                title="Cancel"
              >
                <X size={18} />
              </button>
            )}
            
            {isDone && (
              <button
                onClick={handleRemove}
                className="p-2 text-slate-400 hover:text-red-400 hover:bg-red-500/20 rounded-lg transition-colors"
                title="Remove"
              >
                <Trash2 size={18} />
              </button>
            )}

            {/* Expand button */}
            <button
              onClick={() => setShowDetails(!showDetails)}
              className="p-2 text-slate-400 hover:text-white rounded-lg transition-colors"
            >
              {showDetails ? <ChevronUp size={18} /> : <ChevronDown size={18} />}
            </button>
          </div>
        </div>
      </div>

      {/* Details Panel */}
      {showDetails && (
        <div className="px-4 py-3 border-t border-slate-700 bg-slate-800/50">
          <dl className="grid grid-cols-2 gap-x-6 gap-y-2 text-sm">
            <div>
              <dt className="text-slate-500">ID</dt>
              <dd className="text-slate-300 font-mono text-xs">{task.id}</dd>
            </div>
            <div>
              <dt className="text-slate-500">Location</dt>
              <dd className="text-slate-300 flex items-center gap-1">
                <FolderOpen size={14} />
                {task.output_dir}
              </dd>
            </div>
            <div>
              <dt className="text-slate-500">Created</dt>
              <dd className="text-slate-300">
                {new Date(task.created_at).toLocaleString()}
              </dd>
            </div>
            {task.completed_at && (
              <div>
                <dt className="text-slate-500">Completed</dt>
                <dd className="text-slate-300">
                  {new Date(task.completed_at).toLocaleString()}
                </dd>
              </div>
            )}
            <div>
              <dt className="text-slate-500">Connections</dt>
              <dd className="text-slate-300">{task.config.connections}</dd>
            </div>
            <div>
              <dt className="text-slate-500">Max Speed</dt>
              <dd className="text-slate-300">
                {task.config.max_speed > 0 ? formatSpeed(task.config.max_speed) : 'Unlimited'}
              </dd>
            </div>
          </dl>
        </div>
      )}
    </div>
  );
};

export default DownloadItem;