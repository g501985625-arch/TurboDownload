/**
 * Settings Panel Component
 * 
 * Application settings and preferences
 */

import React, { useState, useEffect } from 'react';
import {
  Settings,
  FolderOpen,
  Save,
  RefreshCw,
  Info,
  CheckCircle
} from 'lucide-react';
import { fileService } from '../../services/file';

interface AppSettings {
  defaultDownloadDir: string;
  maxConcurrentDownloads: number;
  defaultConnections: number;
  maxSpeedLimit: number;
  autoStartDownloads: boolean;
  showNotifications: boolean;
  confirmBeforeDelete: boolean;
}

const defaultSettings: AppSettings = {
  defaultDownloadDir: '',
  maxConcurrentDownloads: 3,
  defaultConnections: 4,
  maxSpeedLimit: 0,
  autoStartDownloads: true,
  showNotifications: true,
  confirmBeforeDelete: true,
};

const SettingsPanel: React.FC = () => {
  const [settings, setSettings] = useState<AppSettings>(defaultSettings);
  const [isSaving, setIsSaving] = useState(false);
  const [saveMessage, setSaveMessage] = useState<string | null>(null);

  // Load settings on mount
  useEffect(() => {
    loadSettings();
  }, []);

  // Load settings from localStorage
  const loadSettings = async () => {
    try {
      const saved = localStorage.getItem('turbodownload_settings');
      if (saved) {
        const parsed = JSON.parse(saved);
        setSettings({ ...defaultSettings, ...parsed });
      } else {
        // Load default download directory
        const defaultDir = await fileService.getDefaultDownloadDir();
        setSettings(prev => ({ ...prev, defaultDownloadDir: defaultDir }));
      }
    } catch (err) {
      console.error('Failed to load settings:', err);
    }
  };

  // Save settings
  const saveSettings = async () => {
    setIsSaving(true);
    try {
      localStorage.setItem('turbodownload_settings', JSON.stringify(settings));
      setSaveMessage('Settings saved successfully!');
      setTimeout(() => setSaveMessage(null), 3000);
    } catch (err) {
      console.error('Failed to save settings:', err);
      setSaveMessage('Failed to save settings');
    } finally {
      setIsSaving(false);
    }
  };

  // Select download directory
  const handleSelectDir = async () => {
    const selected = await fileService.selectDirectory();
    if (selected) {
      setSettings(prev => ({ ...prev, defaultDownloadDir: selected }));
    }
  };

  // Update setting
  const updateSetting = <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => {
    setSettings(prev => ({ ...prev, [key]: value }));
  };

  return (
    <div className="h-full overflow-auto p-6">
      <div className="max-w-2xl mx-auto">
        {/* Header */}
        <div className="flex items-center gap-3 mb-6">
          <Settings size={24} className="text-blue-500" />
          <h2 className="text-xl font-semibold text-white">Settings</h2>
        </div>

        {/* Settings Sections */}
        <div className="space-y-6">
          {/* Download Settings */}
          <section className="card">
            <h3 className="text-lg font-medium text-white mb-4">Download Settings</h3>
            
            <div className="space-y-4">
              {/* Default Download Directory */}
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Default Download Directory
                </label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={settings.defaultDownloadDir}
                    onChange={(e) => updateSetting('defaultDownloadDir', e.target.value)}
                    className="input-field flex-1"
                  />
                  <button
                    onClick={handleSelectDir}
                    className="btn-secondary flex items-center gap-2"
                  >
                    <FolderOpen size={18} />
                    Browse
                  </button>
                </div>
              </div>

              {/* Max Concurrent Downloads */}
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Max Concurrent Downloads
                </label>
                <input
                  type="number"
                  min={1}
                  max={10}
                  value={settings.maxConcurrentDownloads}
                  onChange={(e) => updateSetting('maxConcurrentDownloads', parseInt(e.target.value) || 3)}
                  className="input-field w-32"
                />
                <p className="mt-1 text-xs text-slate-500">
                  Maximum number of downloads running at the same time
                </p>
              </div>

              {/* Default Connections */}
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Default Connections per Download
                </label>
                <input
                  type="number"
                  min={1}
                  max={16}
                  value={settings.defaultConnections}
                  onChange={(e) => updateSetting('defaultConnections', parseInt(e.target.value) || 4)}
                  className="input-field w-32"
                />
                <p className="mt-1 text-xs text-slate-500">
                  Number of concurrent connections per download (multi-threaded)
                </p>
              </div>

              {/* Max Speed Limit */}
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Global Speed Limit (MB/s, 0 = unlimited)
                </label>
                <input
                  type="number"
                  min={0}
                  step={0.1}
                  value={settings.maxSpeedLimit}
                  onChange={(e) => updateSetting('maxSpeedLimit', parseFloat(e.target.value) || 0)}
                  className="input-field w-32"
                />
              </div>

              {/* Auto Start Downloads */}
              <div className="flex items-center justify-between">
                <div>
                  <label className="text-sm font-medium text-slate-300">
                    Auto-start Downloads
                  </label>
                  <p className="text-xs text-slate-500">
                    Automatically start downloads when added
                  </p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={settings.autoStartDownloads}
                    onChange={(e) => updateSetting('autoStartDownloads', e.target.checked)}
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-slate-600 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-500 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
                </label>
              </div>
            </div>
          </section>

          {/* Notification Settings */}
          <section className="card">
            <h3 className="text-lg font-medium text-white mb-4">Notifications</h3>
            
            <div className="space-y-4">
              {/* Show Notifications */}
              <div className="flex items-center justify-between">
                <div>
                  <label className="text-sm font-medium text-slate-300">
                    Show Desktop Notifications
                  </label>
                  <p className="text-xs text-slate-500">
                    Get notified when downloads complete
                  </p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={settings.showNotifications}
                    onChange={(e) => updateSetting('showNotifications', e.target.checked)}
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-slate-600 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-500 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
                </label>
              </div>
            </div>
          </section>

          {/* Behavior Settings */}
          <section className="card">
            <h3 className="text-lg font-medium text-white mb-4">Behavior</h3>
            
            <div className="space-y-4">
              {/* Confirm Before Delete */}
              <div className="flex items-center justify-between">
                <div>
                  <label className="text-sm font-medium text-slate-300">
                    Confirm Before Delete
                  </label>
                  <p className="text-xs text-slate-500">
                    Show confirmation dialog before removing downloads
                  </p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={settings.confirmBeforeDelete}
                    onChange={(e) => updateSetting('confirmBeforeDelete', e.target.checked)}
                    className="sr-only peer"
                  />
                  <div className="w-11 h-6 bg-slate-600 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-500 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
                </label>
              </div>
            </div>
          </section>

          {/* About Section */}
          <section className="card">
            <h3 className="text-lg font-medium text-white mb-4 flex items-center gap-2">
              <Info size={18} />
              About
            </h3>
            
            <div className="space-y-2 text-sm text-slate-400">
              <p><strong className="text-white">TurboDownload</strong> v0.1.0</p>
              <p>A fast download manager with web scraping capabilities.</p>
              <p className="text-xs mt-2">
                Built with Tauri 2.x + React + TypeScript + Rust
              </p>
            </div>
          </section>

          {/* Save Button */}
          <div className="flex items-center justify-end gap-4">
            {saveMessage && (
              <span className="flex items-center gap-2 text-sm text-green-400">
                <CheckCircle size={16} />
                {saveMessage}
              </span>
            )}
            <button
              onClick={saveSettings}
              disabled={isSaving}
              className="btn-primary flex items-center gap-2"
            >
              {isSaving ? (
                <>
                  <RefreshCw size={18} className="animate-spin" />
                  Saving...
                </>
              ) : (
                <>
                  <Save size={18} />
                  Save Settings
                </>
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SettingsPanel;