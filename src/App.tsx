/**
 * TurboDownload App
 * 
 * Main application component
 */

import React, { useState } from 'react';
import { 
  Download, 
  Globe, 
  Settings, 
  RefreshCw
} from 'lucide-react';
import { useDownloadStore } from './stores/downloadStore';
import { useDownloadLifecycle } from './hooks/useTauri';
import DownloadList from './components/DownloadList';
import AddDownload from './components/AddDownload';
import CrawlerPanel from './components/CrawlerPanel';
import SettingsPanel from './components/Settings';

type TabId = 'downloads' | 'crawler' | 'settings';

interface Tab {
  id: TabId;
  label: string;
  icon: React.ReactNode;
}

const tabs: Tab[] = [
  { id: 'downloads', label: 'Downloads', icon: <Download size={20} /> },
  { id: 'crawler', label: 'Crawler', icon: <Globe size={20} /> },
  { id: 'settings', label: 'Settings', icon: <Settings size={20} /> },
];

function App() {
  const [activeTab, setActiveTab] = useState<TabId>('downloads');
  const [showAddDownload, setShowAddDownload] = useState(false);
  
  const { tasks, isLoading, refreshTasks } = useDownloadStore();
  
  // Initialize download lifecycle
  useDownloadLifecycle();

  // Calculate statistics
  const downloadingCount = tasks.filter(t => t.status === 'downloading').length;
  const completedCount = tasks.filter(t => t.status === 'completed').length;

  const handleRefresh = async () => {
    await refreshTasks();
  };

  return (
    <div className="flex flex-col h-screen bg-slate-900">
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 border-b border-slate-700 bg-slate-800">
        <div className="flex items-center gap-3">
          <Download className="w-8 h-8 text-blue-500" />
          <h1 className="text-xl font-bold text-white">TurboDownload</h1>
        </div>
        
        <div className="flex items-center gap-4">
          {/* Stats */}
          <div className="flex items-center gap-6 text-sm">
            <div className="flex items-center gap-2">
              <span className="w-2 h-2 rounded-full bg-blue-500 animate-pulse"></span>
              <span className="text-slate-300">
                {downloadingCount} downloading
              </span>
            </div>
            <div className="flex items-center gap-2">
              <span className="w-2 h-2 rounded-full bg-green-500"></span>
              <span className="text-slate-300">
                {completedCount} completed
              </span>
            </div>
          </div>

          {/* Refresh button */}
          <button
            onClick={handleRefresh}
            disabled={isLoading}
            className="p-2 text-slate-400 hover:text-white transition-colors disabled:opacity-50"
            title="Refresh"
          >
            <RefreshCw size={20} className={isLoading ? 'animate-spin' : ''} />
          </button>
        </div>
      </header>

      {/* Navigation Tabs */}
      <nav className="flex border-b border-slate-700 bg-slate-800">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`
              flex items-center gap-2 px-6 py-3 text-sm font-medium transition-colors border-b-2
              ${activeTab === tab.id
                ? 'text-blue-400 border-blue-400 bg-slate-800'
                : 'text-slate-400 border-transparent hover:text-white hover:bg-slate-700/50'
              }
            `}
          >
            {tab.icon}
            {tab.label}
          </button>
        ))}
      </nav>

      {/* Main Content */}
      <main className="flex-1 overflow-hidden">
        {activeTab === 'downloads' && (
          <div className="h-full flex flex-col">
            {/* Add Download Button */}
            <div className="p-4 border-b border-slate-700 bg-slate-800/50">
              <button
                onClick={() => setShowAddDownload(true)}
                className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
              >
                <Download size={18} />
                Add Download
              </button>
            </div>
            
            {/* Download List */}
            <div className="flex-1 overflow-auto">
              <DownloadList />
            </div>
          </div>
        )}

        {activeTab === 'crawler' && (
          <CrawlerPanel />
        )}

        {activeTab === 'settings' && (
          <SettingsPanel />
        )}
      </main>

      {/* Add Download Modal */}
      {showAddDownload && (
        <AddDownload onClose={() => setShowAddDownload(false)} />
      )}
    </div>
  );
}

export default App;