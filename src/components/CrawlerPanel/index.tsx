/**
 * CrawlerPanel Component
 * 
 * Web resource crawler and extractor
 */

import React, { useState } from 'react';
import {
  Globe,
  Search,
  Download,
  Filter,
  CheckSquare,
  Square,
  FileText,
  Image,
  Video,
  Music,
  Archive,
  Package,
  File,
  Loader,
  AlertCircle
} from 'lucide-react';
import { crawlerService } from '../../services/crawler';
import { useDownloadStore } from '../../stores/downloadStore';
import type { CrawlResult, Resource, ResourceType } from '../../types';

// Resource type icons
const resourceIcons: Record<ResourceType, React.ReactNode> = {
  image: <Image size={16} />,
  video: <Video size={16} />,
  audio: <Music size={16} />,
  document: <FileText size={16} />,
  archive: <Archive size={16} />,
  executable: <Package size={16} />,
  other: <File size={16} />,
};

// Resource type colors
const resourceColors: Record<ResourceType, string> = {
  image: 'text-green-400 bg-green-400/10',
  video: 'text-purple-400 bg-purple-400/10',
  audio: 'text-pink-400 bg-pink-400/10',
  document: 'text-blue-400 bg-blue-400/10',
  archive: 'text-orange-400 bg-orange-400/10',
  executable: 'text-red-400 bg-red-400/10',
  other: 'text-slate-400 bg-slate-400/10',
};

// Format file size
function formatSize(bytes: number | null): string {
  if (bytes === null || bytes === 0) return 'Unknown';
  const units = ['B', 'KB', 'MB', 'GB'];
  let size = bytes;
  let unitIndex = 0;
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }
  return `${size.toFixed(1)} ${units[unitIndex]}`;
}

const CrawlerPanel: React.FC = () => {
  const [url, setUrl] = useState('');
  const [isCrawling, setIsCrawling] = useState(false);
  const [result, setResult] = useState<CrawlResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selectedUrls, setSelectedUrls] = useState<Set<string>>(new Set());
  const [filterTypes, setFilterTypes] = useState<Set<ResourceType>>(new Set());

  const { addTask, startTask } = useDownloadStore();

  // Crawl the URL
  const handleCrawl = async () => {
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

    setIsCrawling(true);
    setError(null);
    setResult(null);
    setSelectedUrls(new Set());

    try {
      const crawlResult = await crawlerService.crawlUrl(url);
      setResult(crawlResult);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to crawl URL');
    } finally {
      setIsCrawling(false);
    }
  };

  // Toggle URL selection
  const toggleUrl = (url: string) => {
    const newSelected = new Set(selectedUrls);
    if (newSelected.has(url)) {
      newSelected.delete(url);
    } else {
      newSelected.add(url);
    }
    setSelectedUrls(newSelected);
  };

  // Select all filtered resources
  const selectAll = () => {
    const filtered = getFilteredResources();
    setSelectedUrls(new Set(filtered.map(r => r.url)));
  };

  // Deselect all
  const deselectAll = () => {
    setSelectedUrls(new Set());
  };

  // Get filtered resources
  const getFilteredResources = (): Resource[] => {
    if (!result) return [];
    if (filterTypes.size === 0) return result.resources;
    return result.resources.filter(r => filterTypes.has(r.resource_type));
  };

  // Toggle filter type
  const toggleFilter = (type: ResourceType) => {
    const newFilter = new Set(filterTypes);
    if (newFilter.has(type)) {
      newFilter.delete(type);
    } else {
      newFilter.add(type);
    }
    setFilterTypes(newFilter);
  };

  // Download selected resources
  const handleDownloadSelected = async () => {
    const urls = Array.from(selectedUrls);
    for (const downloadUrl of urls) {
      try {
        const taskId = await addTask(downloadUrl);
        await startTask(taskId);
      } catch (err) {
        console.error('Failed to add download:', err);
      }
    }
    // Clear selection after adding
    setSelectedUrls(new Set());
  };

  // Get unique resource types
  const getResourceTypes = (): { type: ResourceType; count: number }[] => {
    if (!result) return [];
    const counts = new Map<ResourceType, number>();
    result.resources.forEach(r => {
      counts.set(r.resource_type, (counts.get(r.resource_type) || 0) + 1);
    });
    return Array.from(counts.entries()).map(([type, count]) => ({ type, count }));
  };

  const filteredResources = getFilteredResources();
  const resourceTypes = getResourceTypes();

  return (
    <div className="h-full flex flex-col">
      {/* Input Section */}
      <div className="p-4 border-b border-slate-700 bg-slate-800/50">
        <div className="flex gap-3">
          <div className="relative flex-1">
            <Globe size={18} className="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" />
            <input
              type="text"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleCrawl()}
              placeholder="Enter URL to crawl (e.g., https://example.com/gallery)"
              className="input-field pl-10"
            />
          </div>
          <button
            onClick={handleCrawl}
            disabled={isCrawling}
            className="btn-primary flex items-center gap-2"
          >
            {isCrawling ? (
              <>
                <Loader size={18} className="animate-spin" />
                Crawling...
              </>
            ) : (
              <>
                <Search size={18} />
                Crawl
              </>
            )}
          </button>
        </div>

        {/* Error */}
        {error && (
          <div className="mt-3 p-3 bg-red-500/20 border border-red-500/50 rounded-lg text-red-400 text-sm flex items-center gap-2">
            <AlertCircle size={16} />
            {error}
          </div>
        )}
      </div>

      {/* Results Section */}
      {result && (
        <div className="flex-1 overflow-auto p-4">
          {/* Result Header */}
          <div className="mb-4">
            <h3 className="text-lg font-semibold text-white">
              {result.title || 'Crawl Results'}
            </h3>
            <p className="text-sm text-slate-400">
              Found {result.resources.length} resources from {result.source_url}
            </p>
          </div>

          {/* Filters */}
          <div className="flex items-center gap-4 mb-4 p-3 bg-slate-700/50 rounded-lg">
            <span className="text-sm text-slate-400 flex items-center gap-2">
              <Filter size={16} />
              Filter:
            </span>
            {resourceTypes.map(({ type, count }) => (
              <button
                key={type}
                onClick={() => toggleFilter(type)}
                className={`
                  flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-colors
                  ${filterTypes.has(type) 
                    ? `${resourceColors[type]} border border-current` 
                    : 'text-slate-400 hover:text-white bg-slate-600/50'
                  }
                `}
              >
                {resourceIcons[type]}
                {type} ({count})
              </button>
            ))}
            {filterTypes.size > 0 && (
              <button
                onClick={() => setFilterTypes(new Set())}
                className="text-sm text-slate-400 hover:text-white"
              >
                Clear
              </button>
            )}
          </div>

          {/* Selection Actions */}
          <div className="flex items-center gap-3 mb-4">
            <button
              onClick={selectAll}
              className="text-sm text-slate-400 hover:text-white flex items-center gap-1"
            >
              <CheckSquare size={16} />
              Select All
            </button>
            <button
              onClick={deselectAll}
              className="text-sm text-slate-400 hover:text-white flex items-center gap-1"
            >
              <Square size={16} />
              Deselect All
            </button>
            {selectedUrls.size > 0 && (
              <button
                onClick={handleDownloadSelected}
                className="btn-success flex items-center gap-2 ml-auto"
              >
                <Download size={16} />
                Download Selected ({selectedUrls.size})
              </button>
            )}
          </div>

          {/* Resource List */}
          <div className="space-y-2">
            {filteredResources.map((resource, index) => (
              <div
                key={`${resource.url}-${index}`}
                className={`
                  flex items-center gap-3 p-3 rounded-lg border transition-colors cursor-pointer
                  ${selectedUrls.has(resource.url)
                    ? 'bg-blue-500/10 border-blue-500/50'
                    : 'bg-slate-800 border-slate-700 hover:border-slate-600'
                  }
                `}
                onClick={() => toggleUrl(resource.url)}
              >
                {/* Checkbox */}
                <div className="flex-shrink-0">
                  {selectedUrls.has(resource.url) ? (
                    <CheckSquare size={18} className="text-blue-400" />
                  ) : (
                    <Square size={18} className="text-slate-500" />
                  )}
                </div>

                {/* Type Icon */}
                <div className={`p-2 rounded-lg ${resourceColors[resource.resource_type]}`}>
                  {resourceIcons[resource.resource_type]}
                </div>

                {/* URL */}
                <div className="flex-1 min-w-0">
                  <p className="text-sm text-white truncate" title={resource.url}>
                    {resource.url}
                  </p>
                  {resource.title && (
                    <p className="text-xs text-slate-400 truncate">{resource.title}</p>
                  )}
                </div>

                {/* Size */}
                <div className="text-xs text-slate-400">
                  {formatSize(resource.size)}
                </div>
              </div>
            ))}
          </div>

          {filteredResources.length === 0 && (
            <div className="text-center text-slate-400 py-8">
              No resources match the current filter
            </div>
          )}
        </div>
      )}

      {/* Empty State */}
      {!result && !isCrawling && (
        <div className="flex-1 flex items-center justify-center text-slate-400">
          <div className="text-center">
            <Globe size={64} className="mx-auto mb-4 opacity-50" />
            <p className="text-lg">Enter a URL to discover downloadable resources</p>
            <p className="text-sm mt-2">Images, videos, documents, and more</p>
          </div>
        </div>
      )}
    </div>
  );
};

export default CrawlerPanel;