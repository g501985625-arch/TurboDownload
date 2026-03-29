import { useState } from 'react';
import { Typography, Tabs, Button } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import DownloadList, { DownloadTask } from './DownloadList';
import type { DownloadStatus } from './DownloadItem';

const { Title, Text } = Typography;

type FilterType = 'all' | 'downloading' | 'paused' | 'completed';

export interface DownloadProps {
  initialTasks?: DownloadTask[];
}

const Download: React.FC<DownloadProps> = ({ initialTasks = [] }) => {
  const [tasks, setTasks] = useState<DownloadTask[]>(initialTasks);
  const [filter, setFilter] = useState<FilterType>('all');

  // Filter tasks based on current tab
  const filteredTasks = filter === 'all' 
    ? tasks 
    : tasks.filter(task => task.status === filter);

  // Get counts for tabs
  const counts = {
    all: tasks.length,
    downloading: tasks.filter(t => t.status === 'downloading').length,
    paused: tasks.filter(t => t.status === 'paused').length,
    completed: tasks.filter(t => t.status === 'completed').length,
  };

  // Handle task actions
  const handlePause = (id: string) => {
    setTasks(prev => prev.map(task => 
      task.id === id ? { ...task, status: 'paused' as DownloadStatus, speed: undefined } : task
    ));
  };

  const handleResume = (id: string) => {
    setTasks(prev => prev.map(task => 
      task.id === id ? { ...task, status: 'downloading' as DownloadStatus, speed: 1024 * 1024 } : task
    ));
  };

  const handleCancel = (id: string) => {
    setTasks(prev => prev.filter(task => task.id !== id));
  };

  const tabItems = [
    { label: `全部 (${counts.all})`, key: 'all' },
    { label: `下载中 (${counts.downloading})`, key: 'downloading' },
    { label: `已暂停 (${counts.paused})`, key: 'paused' },
    { label: `已完成 (${counts.completed})`, key: 'completed' },
  ];

  return (
    <div style={{ padding: '24px' }}>
      <div style={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'center', 
        marginBottom: 24 
      }}>
        <div>
          <Title level={2} style={{ marginBottom: 4 }}>下载管理</Title>
          <Text type="secondary">管理您的下载任务</Text>
        </div>
        <Button type="primary" icon={<PlusOutlined />}>
          新建下载
        </Button>
      </div>

      {/* Filter tabs */}
      <Tabs
        activeKey={filter}
        onChange={(key) => setFilter(key as FilterType)}
        items={tabItems}
        style={{ marginBottom: 24 }}
      />

      {/* Download list */}
      <DownloadList
        tasks={filteredTasks}
        onPause={handlePause}
        onResume={handleResume}
        onCancel={handleCancel}
      />
    </div>
  );
};

export default Download;
export type { DownloadTask, DownloadStatus };