import { useState, useCallback } from 'react';
import { Typography, Tabs, Button, Modal, Input, Form, message } from 'antd';
import { PlusOutlined, LinkOutlined, ClearOutlined } from '@ant-design/icons';
import DownloadList from '../components/DownloadList';
import { useDownloadStore, useTaskStats } from '../store/downloadStore';
import { useProgressUpdater, useTaskCompletion, useTaskError } from '../hooks/useProgressUpdater';


const { Title, Text } = Typography;

type FilterType = 'all' | 'downloading' | 'paused' | 'completed' | 'error';

const Download = () => {
  // Enable automatic progress updates
  useProgressUpdater({ interval: 1000 });
  
  // Store state and actions
  const tasks = useDownloadStore((state) => state.tasks);
  const isLoading = useDownloadStore((state) => state.isLoading);
  const addTask = useDownloadStore((state) => state.addTask);
  const pauseTask = useDownloadStore((state) => state.pauseTask);
  const resumeTask = useDownloadStore((state) => state.resumeTask);
  const cancelTask = useDownloadStore((state) => state.cancelTask);
  const clearCompleted = useDownloadStore((state) => state.clearCompleted);
  
  // Stats
  const stats = useTaskStats();
  
  // Local state
  const [filter, setFilter] = useState<FilterType>('all');
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [form] = Form.useForm();
  
  // Notifications
  useTaskCompletion((task) => {
    message.success(`${task.filename} 下载完成！`);
  });
  
  useTaskError((task) => {
    message.error(`${task.filename} 下载失败: ${task.error}`);
  });
  
  // Filter tasks
  const filteredTasks = filter === 'all'
    ? tasks
    : tasks.filter((task) => task.status === filter);
  
  // Handlers
  const handlePause = useCallback(async (id: string) => {
    await pauseTask(id);
    message.info('下载已暂停');
  }, [pauseTask]);
  
  const handleResume = useCallback(async (id: string) => {
    await resumeTask(id);
    message.info('下载已恢复');
  }, [resumeTask]);
  
  const handleCancel = useCallback(async (id: string) => {
    await cancelTask(id);
    message.info('下载已取消');
  }, [cancelTask]);
  

  
  const handleAddDownload = useCallback(async (values: { url: string; filename?: string }) => {
    try {
      // Get default download path
      const defaultPath = useDownloadStore.getState().defaultDownloadPath || './downloads';
      const filename = values.filename || values.url.split('/').pop() || 'download';
      const outputPath = `${defaultPath}/${filename}`;
      
      await addTask({
        url: values.url,
        output_path: outputPath,
      });
      
      message.success('下载任务已添加');
      setIsModalOpen(false);
      form.resetFields();
    } catch (error) {
      message.error('添加下载失败: ' + (error instanceof Error ? error.message : '未知错误'));
    }
  }, [addTask, form]);
  
  const handleClearCompleted = useCallback(() => {
    clearCompleted();
    message.success('已清理完成的任务');
  }, [clearCompleted]);
  
  // Tab items
  const tabItems = [
    { label: `全部 (${stats.total})`, key: 'all' },
    { label: `下载中 (${stats.downloading})`, key: 'downloading' },
    { label: `已暂停 (${stats.paused})`, key: 'paused' },
    { label: `已完成 (${stats.completed})`, key: 'completed' },
    { label: `失败 (${stats.error})`, key: 'error' },
  ];
  
  // Convert store tasks to DownloadList format
  const listTasks: Array<{
    id: string;
    filename: string;
    size: number;
    progress: number;
    speed?: number;
    status: 'downloading' | 'paused' | 'completed';
  }> = filteredTasks.map((task) => ({
    id: task.id,
    filename: task.filename,
    size: task.progress.total || 0,
    progress: task.progress.percent,
    speed: task.progress.speed,
    status: task.status === 'error' || task.status === 'pending'
      ? 'paused'
      : task.status,
  }));
  
  return (
    <div style={{ padding: '24px' }}>
      {/* Header */}
      <div style={{
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
        marginBottom: 24,
      }}>
        <div>
          <Title level={2} style={{ marginBottom: 4 }}>下载管理</Title>
          <Text type="secondary">管理您的下载任务</Text>
        </div>
        <div style={{ display: 'flex', gap: 12 }}>
          <Button
            icon={<ClearOutlined />}
            onClick={handleClearCompleted}
            disabled={stats.completed === 0}
          >
            清理已完成
          </Button>
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={() => setIsModalOpen(true)}
            loading={isLoading}
          >
            新建下载
          </Button>
        </div>
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
        tasks={listTasks}
        onPause={handlePause}
        onResume={handleResume}
        onCancel={handleCancel}
      />
      
      {/* Add Download Modal */}
      <Modal
        title="新建下载"
        open={isModalOpen}
        onOk={() => form.submit()}
        onCancel={() => {
          setIsModalOpen(false);
          form.resetFields();
        }}
        confirmLoading={isLoading}
      >
        <Form
          form={form}
          layout="vertical"
          onFinish={handleAddDownload}
          style={{ marginTop: 16 }}
        >
          <Form.Item
            name="url"
            label="下载链接"
            rules={[
              { required: true, message: '请输入下载链接' },
              { type: 'url', message: '请输入有效的URL' },
            ]}
          >
            <Input
              prefix={<LinkOutlined />}
              placeholder="https://example.com/file.zip"
              size="large"
            />
          </Form.Item>
          
          <Form.Item
            name="filename"
            label="文件名（可选）"
          >
            <Input
              placeholder="留空将自动从URL提取"
              size="large"
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default Download;