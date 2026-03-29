/**
 * AddDownload Component
 * 
 * Modal for adding a new download task
 */

import React, { useState } from 'react';
import { X, Link, FolderOpen, Settings, ChevronDown, ChevronUp } from 'lucide-react';
import { useDownloadStore } from '../../stores/downloadStore';
import { fileService } from '../../services/file';
import type { DownloadConfig } from '../../types';

interface AddDownloadProps {
  onClose: () => void;
}

const AddDownload: React.FC<AddDownloadProps> = ({ onClose }) => {
  const [url, setUrl] = useState('');
  const [filename, setFilename] = useState('');
  const [outputDir, setOutputDir] = useState('');
  const [connections, setConnections] = useState(4);
  const [maxSpeed, setMaxSpeed] = useState(0);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const { addTask, startTask } = useDownloadStore();

  // Load default directory on mount
  React.useEffect(() => {
    fileService.getDefaultDownloadDir().then(setOutputDir);
  }, []);

  // Extract filename from URL
  const handleUrlChange = (value: string) => {
    setUrl(value);
    if (!filename) {
      try {
        const urlObj = new URL(value);
        const pathParts = urlObj.pathname.split('/');
        const extractedFilename = pathParts[pathParts.length - 1] || 'download';
        setFilename(decodeURIComponent(extractedFilename));
      } catch {
        // Invalid URL, ignore
      }
    }
  };

  // Select output directory
  const handleSelectDir = async () => {
    const selected = await fileService.selectDirectory();
    if (selected) {
      setOutputDir(selected);
    }
  };

  // Handle form submission
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!url.trim()) {
      setError('Please enter a URL');
      return;
    }

    // Validate URL
    try {
      const urlObj = new URL(url);
      if (urlObj.protocol !== 'http:' && urlObj.protocol !== 'https:') {
        setError('Only HTTP and HTTPS URLs are supported');
        return;
      }
    } catch {
      setError('Please enter a valid URL');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const config: DownloadConfig = {
        filename: filename || undefined,
        output_dir: outputDir || undefined,
        connections,
        max_speed: maxSpeed * 1024 * 1024, // Convert MB/s to bytes/s
        headers: {},
      };

      const taskId = await addTask(url, config);
      
      // Start the download immediately
      await startTask(taskId);
      
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add download');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-slate-800 border border-slate-700 rounded-xl w-full max-w-xl mx-4 overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-700">
          <h2 className="text-lg font-semibold text-white">Add Download</h2>
          <button
            onClick={onClose}
            className="p-1 text-slate-400 hover:text-white transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-6 space-y-4">
          {/* URL Input */}
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
              URL
            </label>
            <div className="relative">
              <Link size={18} className="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" />
              <input
                type="text"
                value={url}
                onChange={(e) => handleUrlChange(e.target.value)}
                placeholder="https://example.com/file.zip"
                className="input-field pl-10"
                autoFocus
              />
            </div>
          </div>

          {/* Filename Input */}
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
              Filename (optional)
            </label>
            <input
              type="text"
              value={filename}
              onChange={(e) => setFilename(e.target.value)}
              placeholder="Auto-detected from URL"
              className="input-field"
            />
          </div>

          {/* Output Directory */}
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
              Save to
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={outputDir}
                onChange={(e) => setOutputDir(e.target.value)}
                placeholder="Default download folder"
                className="input-field flex-1"
              />
              <button
                type="button"
                onClick={handleSelectDir}
                className="btn-secondary flex items-center gap-2"
              >
                <FolderOpen size={18} />
                Browse
              </button>
            </div>
          </div>

          {/* Advanced Options */}
          <div>
            <button
              type="button"
              onClick={() => setShowAdvanced(!showAdvanced)}
              className="flex items-center gap-2 text-sm text-slate-400 hover:text-white transition-colors"
            >
              <Settings size={16} />
              Advanced Options
              {showAdvanced ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
            </button>

            {showAdvanced && (
              <div className="mt-4 p-4 bg-slate-700/50 rounded-lg space-y-4">
                {/* Connections */}
                <div>
                  <label className="block text-sm font-medium text-slate-300 mb-2">
                    Concurrent Connections
                  </label>
                  <input
                    type="number"
                    min={1}
                    max={16}
                    value={connections}
                    onChange={(e) => setConnections(parseInt(e.target.value) || 4)}
                    className="input-field"
                  />
                  <p className="mt-1 text-xs text-slate-500">
                    More connections can speed up downloads but may be blocked by some servers
                  </p>
                </div>

                {/* Max Speed */}
                <div>
                  <label className="block text-sm font-medium text-slate-300 mb-2">
                    Max Speed (MB/s, 0 = unlimited)
                  </label>
                  <input
                    type="number"
                    min={0}
                    max={1000}
                    step={0.1}
                    value={maxSpeed}
                    onChange={(e) => setMaxSpeed(parseFloat(e.target.value) || 0)}
                    className="input-field"
                  />
                </div>
              </div>
            )}
          </div>

          {/* Error */}
          {error && (
            <div className="p-3 bg-red-500/20 border border-red-500/50 rounded-lg text-red-400 text-sm">
              {error}
            </div>
          )}

          {/* Actions */}
          <div className="flex justify-end gap-3 pt-4">
            <button
              type="button"
              onClick={onClose}
              className="btn-secondary"
              disabled={isLoading}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="btn-primary flex items-center gap-2"
              disabled={isLoading}
            >
              {isLoading ? 'Adding...' : 'Add Download'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default AddDownload;