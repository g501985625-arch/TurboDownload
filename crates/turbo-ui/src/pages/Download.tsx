import { useState } from 'react';
import { Typography, Tabs, Button } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import DownloadList, { DownloadTask } from '../components/DownloadList';
import type { DownloadStatus } from '../components/DownloadItem';

const { Title, Text } = Typography;

type FilterType = 'all' | 'downloading' | 'paused' | 'completed';

// Mock data for demonstration
const initialTasks: DownloadTask[] = [
  {
    id: '1',
    filename: 'ubuntu-22.04.3-desktop-amd64.iso',
    size: 4.5 * 1024 * 1024 * 1024,
    progress: 78,
    speed: 12.5 * 1024 * 1024,
    status: 'downloading',
  },
  {
    id: '2',
    filename: 'node-v20.10.0-x64.msi',
    size: 32 * 1024 * 1024,
    progress: 45,
    speed: 2.1 * 1024 * 1024,
    status: 'downloading',
  },
  {
    id: '3',
    filename: 'react-framework-bundle.tar.gz',
    size: 156 * 1024 * 1024,
    progress: 100,
    status: 'completed',
  },
  {
    id: '4',
    filename: 'design-assets.zip',
    size: 890 * 1024 * 1024,
    progress: 23,
    status: 'paused',
  },
  {
    id: '5',
    filename: 'video-tutorial-part1.mp4',
    size: 2.1 * 1024 * 1024 * 1024,
    progress: 12,
    speed: 800 * 1024,
    status: 'downloading',
  },
];

const Download = () => {
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