/**
 * DownloadList Component
 * 
 * Displays a list of download tasks
 */

import React from 'react';
import { useDownloadStore } from '../../stores/downloadStore';
import DownloadItem from '../DownloadItem';
import { Inbox } from 'lucide-react';

const DownloadList: React.FC = () => {
  const { tasks, isLoading, selectedTaskId, selectTask } = useDownloadStore();

  if (isLoading && tasks.length === 0) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-slate-400">Loading...</div>
      </div>
    );
  }

  if (tasks.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-full text-slate-400">
        <Inbox size={64} className="mb-4 opacity-50" />
        <p className="text-lg">No downloads yet</p>
        <p className="text-sm mt-2">Click "Add Download" to start</p>
      </div>
    );
  }

  // Sort tasks by creation date (newest first)
  const sortedTasks = [...tasks].sort((a, b) => 
    new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
  );

  return (
    <div className="p-4 space-y-3">
      {sortedTasks.map((task) => (
        <DownloadItem
          key={task.id}
          task={task}
          isSelected={task.id === selectedTaskId}
          onSelect={() => selectTask(task.id === selectedTaskId ? null : task.id)}
        />
      ))}
    </div>
  );
};

export default DownloadList;