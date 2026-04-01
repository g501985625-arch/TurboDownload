import { useState } from 'react';
import { 
  Row, Col, 
  Input, Button, Typography, 
  Table, Space, Tag, Card, message 
} from 'antd';
import { 
  ScanOutlined, 
  VideoCameraOutlined, 
  PictureOutlined, 
  FileTextOutlined, 
  AudioOutlined,
  PlayCircleOutlined,
  DownloadOutlined
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { invoke } from '@tauri-apps/api/core';

const { Title, Text } = Typography;
const { Search } = Input;

interface ResourceItem {
  key: string;
  name: string;
  type: 'video' | 'image' | 'document' | 'audio';
  size: string;
  url: string;
  status: 'pending' | 'scanned';
}

// ========== Mock API Interfaces ==========
const mockApi = {
  // 扫描 URL 接口
  scanUrl: async (url: string): Promise<ResourceItem[]> => {
    // Simulate network delay
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    // Mock scan result based on URL
    const mockResources: ResourceItem[] = [
      { key: '1', name: 'video_sample.mp4', type: 'video', size: '256 MB', url: `${url}/video_sample.mp4`, status: 'scanned' },
      { key: '2', name: 'image_001.jpg', type: 'image', size: '2.4 MB', url: `${url}/image_001.jpg`, status: 'scanned' },
      { key: '3', name: 'document.pdf', type: 'document', size: '5.8 MB', url: `${url}/document.pdf`, status: 'scanned' },
      { key: '4', name: 'audio_track.mp3', type: 'audio', size: '8.2 MB', url: `${url}/audio_track.mp3`, status: 'scanned' },
      { key: '5', name: 'photo_gallery.png', type: 'image', size: '1.2 MB', url: `${url}/photo_gallery.png`, status: 'scanned' },
      { key: '6', name: 'presentation.pptx', type: 'document', size: '15.3 MB', url: `${url}/presentation.pptx`, status: 'scanned' },
    ];
    return mockResources;
  },

  // 获取资源列表接口
  getResourceList: async (): Promise<ResourceItem[]> => {
    await new Promise(resolve => setTimeout(resolve, 500));
    return [];
  },

  // 预览接口
  preview: async (resource: ResourceItem): Promise<string> => {
    await new Promise(resolve => setTimeout(resolve, 300));
    return resource.url;
  },

  // 下载接口
  download: async (resource: ResourceItem): Promise<void> => {
    await new Promise(resolve => setTimeout(resolve, 300));
    // Trigger download
    const link = document.createElement('a');
    link.href = resource.url;
    link.download = resource.name;
    link.click();
  },
};

// ========== Component ==========
const Radar = () => {
  const [loading, setLoading] = useState(false);
  const [resources, setResources] = useState<ResourceItem[]>([]);

  // 动态计算统计：根据扫描结果统计资源类型数量
  const stats = {
    video: resources.filter(r => r.type === 'video').length,
    image: resources.filter(r => r.type === 'image').length,
    document: resources.filter(r => r.type === 'document').length,
    audio: resources.filter(r => r.type === 'audio').length,
  };

  const handleScan = async (value: string) => {
    if (!value) {
      message.warning('请输入URL地址');
      return;
    }
    
    // Basic URL validation
    try {
      new URL(value);
    } catch {
      message.error('请输入有效的URL地址');
      return;
    }
    
    setLoading(true);
    setResources([]); // Clear previous results
    
    try {
      // 调用真实的 Tauri 命令：扫描 URL
      const scanResults = await invoke<any[]>('scan_url', { url: value });
      
      // 转换后端返回的数据格式为前端 ResourceItem 格式
      const convertedResources: ResourceItem[] = scanResults.map((item, index) => {
        // 将后端的 resource_type 转换为前端类型
        let resourceType: ResourceItem['type'] = 'document';
        const rt = item.resource_type?.toLowerCase() || '';
        if (rt.includes('image')) resourceType = 'image';
        else if (rt.includes('video')) resourceType = 'video';
        else if (rt.includes('audio')) resourceType = 'audio';
        else if (rt.includes('document')) resourceType = 'document';
        
        // 格式化文件大小
        let sizeStr = 'Unknown';
        if (item.size) {
          const bytes = item.size;
          if (bytes < 1024) sizeStr = `${bytes} B`;
          else if (bytes < 1024 * 1024) sizeStr = `${(bytes / 1024).toFixed(1)} KB`;
          else if (bytes < 1024 * 1024 * 1024) sizeStr = `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
          else sizeStr = `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
        }
        
        return {
          key: item.url || String(index),
          name: item.filename || item.url.split('/').pop() || 'Unknown',
          type: resourceType,
          size: sizeStr,
          url: item.url,
          status: 'scanned' as const,
        };
      });
      
      // 去重：根据 URL 去重
      const seen = new Set<string>();
      const uniqueResources = convertedResources.filter(item => {
        if (seen.has(item.url)) return false;
        seen.add(item.url);
        return true;
      });
      
      setResources(uniqueResources);
      message.success(`扫描完成，发现 ${uniqueResources.length} 个资源`);
    } catch (error) {
      console.error('Scan error:', error);
      message.error('扫描失败: ' + (error as Error).message);
    } finally {
      setLoading(false);
    }
  };

  // 处理预览
  const handlePreview = async (record: ResourceItem) => {
    try {
      // 调用 Mock API：预览接口
      const previewUrl = await mockApi.preview(record);
      window.open(previewUrl, '_blank');
    } catch (error) {
      message.error('预览失败');
    }
  };

  // 处理下载 - 使用真实的 Tauri 命令
  const handleDownload = async (record: ResourceItem) => {
    try {
      // 调用 Tauri 命令进行真实下载
      const taskInfo = await invoke('start_download', {
        url: record.url,
        filename: record.name,
      });
      console.log('Download started:', taskInfo);
      message.success(`开始下载: ${record.name}`);
    } catch (error) {
      console.error('Download error:', error);
      message.error('下载失败: ' + (error as Error).message);
    }
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'video': return <VideoCameraOutlined style={{ color: '#1890ff' }} />;
      case 'image': return <PictureOutlined style={{ color: '#52c41a' }} />;
      case 'document': return <FileTextOutlined style={{ color: '#fa8c16' }} />;
      case 'audio': return <AudioOutlined style={{ color: '#722ed1' }} />;
      default: return <FileTextOutlined />;
    }
  };

  const getTypeTag = (type: string) => {
    const colorMap: Record<string, string> = {
      video: 'blue',
      image: 'green',
      document: 'orange',
      audio: 'purple',
    };
    const labelMap: Record<string, string> = {
      video: '视频',
      image: '图片',
      document: '文档',
      audio: '音频',
    };
    return <Tag color={colorMap[type]}>{labelMap[type]}</Tag>;
  };

  const columns: ColumnsType<ResourceItem> = [
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
      render: (text, record) => (
        <Space>
          {getTypeIcon(record.type)}
          <a href={record.url} target="_blank" rel="noopener noreferrer">{text}</a>
        </Space>
      ),
    },
    {
      title: '类型',
      dataIndex: 'type',
      key: 'type',
      width: 100,
      render: (type) => getTypeTag(type),
    },
    {
      title: '大小',
      dataIndex: 'size',
      key: 'size',
      width: 100,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: (status) => (
        <Tag color={status === 'scanned' ? 'success' : 'default'}>
          {status === 'scanned' ? '已扫描' : '待扫描'}
        </Tag>
      ),
    },
    {
      title: '操作',
      key: 'action',
      width: 150,
      render: (_, record) => (
        <Space size="small">
          <Button 
            type="link" 
            icon={<PlayCircleOutlined />} 
            size="small"
            onClick={() => handlePreview(record)}
          >
            预览
          </Button>
          <Button 
            type="link" 
            icon={<DownloadOutlined />} 
            size="small"
            onClick={() => handleDownload(record)}
          >
            下载
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <Title level={2} style={{ marginBottom: 8 }}>雷达</Title>
      <Text type="secondary" style={{ marginBottom: 24, display: 'block' }}>
        发现网页中的可下载资源
      </Text>

      {/* URL 输入区域 */}
      <Card style={{ marginBottom: 24, borderRadius: 12 }}>
        <Search
          placeholder="输入URL地址进行扫描"
          enterButton={<><ScanOutlined /> 扫描</>}
          size="large"
          onSearch={handleScan}
          loading={loading}
          style={{ maxWidth: 600 }}
        />
      </Card>

      {/* 资源类型统计卡片 */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={12} sm={6}>
          <Card
            hoverable
            style={{ 
              borderRadius: 12,
              borderColor: '#1890ff',
              boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
            }}
            styles={{ body: { padding: '16px' } }}
          >
            <Space direction="vertical" size={4} style={{ width: '100%', textAlign: 'center' }}>
              <div style={{
                width: 40,
                height: 40,
                borderRadius: 10,
                background: '#e6f7ff',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                margin: '0 auto',
              }}>
                <VideoCameraOutlined style={{ fontSize: 20, color: '#1890ff' }} />
              </div>
              <Title level={3} style={{ margin: 0, color: '#1890ff' }}>{stats.video}</Title>
              <Text type="secondary">视频</Text>
            </Space>
          </Card>
        </Col>
        <Col xs={12} sm={6}>
          <Card
            hoverable
            style={{ 
              borderRadius: 12,
              borderColor: '#52c41a',
              boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
            }}
            styles={{ body: { padding: '16px' } }}
          >
            <Space direction="vertical" size={4} style={{ width: '100%', textAlign: 'center' }}>
              <div style={{
                width: 40,
                height: 40,
                borderRadius: 10,
                background: '#f6ffed',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                margin: '0 auto',
              }}>
                <PictureOutlined style={{ fontSize: 20, color: '#52c41a' }} />
              </div>
              <Title level={3} style={{ margin: 0, color: '#52c41a' }}>{stats.image}</Title>
              <Text type="secondary">图片</Text>
            </Space>
          </Card>
        </Col>
        <Col xs={12} sm={6}>
          <Card
            hoverable
            style={{ 
              borderRadius: 12,
              borderColor: '#fa8c16',
              boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
            }}
            styles={{ body: { padding: '16px' } }}
          >
            <Space direction="vertical" size={4} style={{ width: '100%', textAlign: 'center' }}>
              <div style={{
                width: 40,
                height: 40,
                borderRadius: 10,
                background: '#fff7e6',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                margin: '0 auto',
              }}>
                <FileTextOutlined style={{ fontSize: 20, color: '#fa8c16' }} />
              </div>
              <Title level={3} style={{ margin: 0, color: '#fa8c16' }}>{stats.document}</Title>
              <Text type="secondary">文档</Text>
            </Space>
          </Card>
        </Col>
        <Col xs={12} sm={6}>
          <Card
            hoverable
            style={{ 
              borderRadius: 12,
              borderColor: '#722ed1',
              boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
            }}
            styles={{ body: { padding: '16px' } }}
          >
            <Space direction="vertical" size={4} style={{ width: '100%', textAlign: 'center' }}>
              <div style={{
                width: 40,
                height: 40,
                borderRadius: 10,
                background: '#f9f0ff',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                margin: '0 auto',
              }}>
                <AudioOutlined style={{ fontSize: 20, color: '#722ed1' }} />
              </div>
              <Title level={3} style={{ margin: 0, color: '#722ed1' }}>{stats.audio}</Title>
              <Text type="secondary">音频</Text>
            </Space>
          </Card>
        </Col>
      </Row>

      {/* 资源列表 */}
      <Card style={{ borderRadius: 12 }}>
        <Title level={4} style={{ marginBottom: 16 }}>资源列表</Title>
        <Table
          columns={columns}
          dataSource={resources}
          loading={loading}
          pagination={{ pageSize: 10 }}
          locale={{ emptyText: '请输入URL进行扫描' }}
        />
      </Card>
    </div>
  );
};

export default Radar;