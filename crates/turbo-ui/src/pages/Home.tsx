import { Button, Card, Typography, Space } from 'antd';
import { DownloadOutlined, CloudDownloadOutlined } from '@ant-design/icons';

const { Title, Text } = Typography;

const Home = () => {
  return (
    <div style={{ 
      minHeight: '100vh', 
      display: 'flex', 
      flexDirection: 'column',
      alignItems: 'center', 
      justifyContent: 'center',
      background: 'linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)'
    }}>
      <Card 
        style={{ width: 400, textAlign: 'center', borderRadius: 12, boxShadow: '0 4px 12px rgba(0,0,0,0.1)' }}
      >
        <Space direction="vertical" size="large" style={{ width: '100%' }}>
          <CloudDownloadOutlined style={{ fontSize: 48, color: '#1890ff' }} />
          <Title level={2} style={{ margin: 0 }}>Turbo Download</Title>
          <Text type="secondary">Fast and reliable download manager</Text>
          <Button 
            type="primary" 
            icon={<DownloadOutlined />} 
            size="large"
            block
          >
            Start Downloading
          </Button>
        </Space>
      </Card>
    </div>
  );
};

export default Home;