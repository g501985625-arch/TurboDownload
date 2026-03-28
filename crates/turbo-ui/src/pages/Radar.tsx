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

const Radar = () => {
  const [loading, setLoading] = useState(false);
  const [url, setUrl] = useState('');
  const [resources, setResources] = useState<ResourceItem[]>([]);

  // Mock statistics data
  const stats = {
    video: 12,
    image: 34,
    document: 8,
    audio: 5,
  };

  const handleScan = (value: string) => {
    if (!value) {
      message.warning('请输入URL地址');
      return;
    }
    setUrl(value);
    setLoading(true);
    
    // Mock scan result - 模拟扫描结果
    setTimeout(() => {
      const mockResources: ResourceItem[] = [
        { key: '1', name: 'video_sample.mp4', type: 'video', size: '256 MB', url: `${value}/video_sample.mp4`, status: 'scanned' },
        { key: '2', name: 'image_001.jpg', type: 'image', size: '2.4 MB', url: `${value}/image_001.jpg`, status: 'scanned' },
        { key: '3', name: 'document.pdf', type: 'document', size: '5.8 MB', url: `${value}/document.pdf`, status: 'scanned' },
        { key: '4', name: 'audio_track.mp3', type: 'audio', size: '8.2 MB', url: `${value}/audio_track.mp3`, status: 'scanned' },
        { key: '5', name: 'photo_gallery.png', type: 'image', size: '1.2 MB', url: `${value}/photo_gallery.png`, status: 'scanned' },
        { key: '6', name: 'presentation.pptx', type: 'document', size: '15.3 MB', url: `${value}/presentation.pptx`, status: 'scanned' },
      ];
      setResources(mockResources);
      setLoading(false);
      message.success('扫描完成，发现 6 个资源');
    }, 1500);
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
            onClick={() => message.info('预览功能开发中')}
          >
            预览
          </Button>
          <Button 
            type="link" 
            icon={<DownloadOutlined />} 
            size="small"
            onClick={() => message.info('下载功能开发中')}
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