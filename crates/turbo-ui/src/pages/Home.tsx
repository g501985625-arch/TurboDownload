import { Typography, Card, Row, Col, Statistic } from 'antd';
import { DownloadOutlined, CheckCircleOutlined, ClockCircleOutlined, RocketOutlined } from '@ant-design/icons';

const { Title, Text } = Typography;

const Home = () => {
  return (
    <div>
      <Title level={2}>欢迎使用 Turbo Download</Title>
      <Text type="secondary" style={{ display: 'block', marginBottom: 24 }}>
        快速、可靠的下载管理器
      </Text>
      
      <Row gutter={[16, 16]}>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic 
              title="正在下载" 
              value={0} 
              prefix={<DownloadOutlined style={{ color: '#1890ff' }} />} 
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic 
              title="已完成" 
              value={12} 
              prefix={<CheckCircleOutlined style={{ color: '#52c41a' }} />} 
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic 
              title="等待中" 
              value={0} 
              prefix={<ClockCircleOutlined style={{ color: '#faad14' }} />} 
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic 
              title="总速度" 
              value={0} 
              suffix="KB/s"
              prefix={<RocketOutlined style={{ color: '#722ed1' }} />} 
            />
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default Home;