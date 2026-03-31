/**
 * UpdateSection Component
 * 
 * Settings section for auto-update configuration
 */

import React from 'react';
import { RefreshCw, CheckCircle, AlertCircle, ChevronDown } from 'lucide-react';

export type UpdateFrequency = 'daily' | 'weekly' | 'manual';

export interface UpdateSettings {
  autoCheckEnabled: boolean;
  checkFrequency: UpdateFrequency;
  lastCheckTime?: string;
  skippedVersions: string[];
}

interface UpdateSectionProps {
  settings: UpdateSettings;
  currentVersion: string;
  onSettingsChange: (settings: UpdateSettings) => void;
  onCheckNow: () => void;
  isChecking: boolean;
  checkStatus?: 'idle' | 'success' | 'error';
  checkMessage?: string;
}

const frequencyOptions: { value: UpdateFrequency; label: string }[] = [
  { value: 'daily', label: '每天' },
  { value: 'weekly', label: '每周' },
  { value: 'manual', label: '手动' },
];

const formatLastCheckTime = (time?: string): string => {
  if (!time) return '从未检查';
  
  const date = new Date(time);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);
  
  if (diffMins < 1) return '刚刚';
  if (diffMins < 60) return `${diffMins} 分钟前`;
  if (diffHours < 24) return `${diffHours} 小时前`;
  if (diffDays < 7) return `${diffDays} 天前`;
  
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
};

const UpdateSection: React.FC<UpdateSectionProps> = ({
  settings,
  currentVersion,
  onSettingsChange,
  onCheckNow,
  isChecking,
  checkStatus,
  checkMessage,
}) => {
  const handleToggleAutoCheck = (enabled: boolean) => {
    onSettingsChange({
      ...settings,
      autoCheckEnabled: enabled,
    });
  };

  const handleFrequencyChange = (frequency: UpdateFrequency) => {
    onSettingsChange({
      ...settings,
      checkFrequency: frequency,
    });
  };

  return (
    <section className="card">
      <h3 className="text-lg font-medium text-white mb-4">自动更新</h3>
      
      <div className="space-y-4">
        {/* Auto Check Toggle */}
        <div className="flex items-center justify-between">
          <div>
            <label className="text-sm font-medium text-slate-300">
              自动检查更新
            </label>
            <p className="text-xs text-slate-500 mt-0.5">
              自动检测新版本并提示更新
            </p>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={settings.autoCheckEnabled}
              onChange={(e) => handleToggleAutoCheck(e.target.checked)}
              className="sr-only peer"
            />
            <div className="w-11 h-6 bg-slate-600 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-500 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
          </label>
        </div>

        {/* Check Frequency */}
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            检查频率
          </label>
          <div className="relative">
            <select
              value={settings.checkFrequency}
              onChange={(e) => handleFrequencyChange(e.target.value as UpdateFrequency)}
              disabled={!settings.autoCheckEnabled}
              className="input-field w-full disabled:bg-slate-700 disabled:text-slate-400 appearance-none cursor-pointer"
            >
              {frequencyOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
            <ChevronDown 
              size={16} 
              className="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none" 
            />
          </div>
        </div>

        {/* Current Version */}
        <div className="flex items-center justify-between p-3 bg-slate-800 rounded-lg">
          <div>
            <span className="text-sm text-slate-400">当前版本</span>
            <p className="text-sm font-medium text-white">v{currentVersion}</p>
          </div>
          <span className="px-2 py-1 text-xs font-medium text-green-400 bg-green-900/30 rounded-full">
            最新
          </span>
        </div>

        {/* Last Check Time */}
        <div className="flex items-center justify-between">
          <div>
            <span className="text-sm text-slate-400">上次检查</span>
            <p className="text-sm font-medium text-white">
              {formatLastCheckTime(settings.lastCheckTime)}
            </p>
          </div>
          <button
            onClick={onCheckNow}
            disabled={isChecking}
            className="btn-secondary flex items-center gap-2"
          >
            <RefreshCw size={14} className={isChecking ? 'animate-spin' : ''} />
            {isChecking ? '检查中...' : '立即检查'}
          </button>
        </div>

        {/* Check Status Message */}
        {checkStatus && checkStatus !== 'idle' && (
          <div className={`flex items-center gap-2 p-3 rounded-lg ${
            checkStatus === 'success' 
              ? 'bg-green-900/20 text-green-400' 
              : 'bg-red-900/20 text-red-400'
          }`}>
            {checkStatus === 'success' ? (
              <CheckCircle size={16} />
            ) : (
              <AlertCircle size={16} />
            )}
            <span className="text-sm">{checkMessage}</span>
          </div>
        )}
      </div>
    </section>
  );
};

export default UpdateSection;
