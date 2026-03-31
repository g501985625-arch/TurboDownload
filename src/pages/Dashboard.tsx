import { Row, Col, Typography } from 'antd';
import StatCard from '../components/StatCard';

const { Title } = Typography;

const Dashboard = () => {
  // Mock data - 后续可以从 store 或 API 获取
  const stats = {
    speed: '125 MB/s',
    tasks: 8,
    completed: 156,
  };

  return (
    <div style={{ padding: '24px' }}>
      <Title level={2} style={{ marginBottom: 24 }}>Dashboard</Title>
      <Row gutter={[24, 24]}>
        <Col xs={24} sm={12} lg={8}>
          <StatCard 
            type="speed" 
            value={stats.speed} 
            label="Current Speed" 
          />
        </Col>
        <Col xs={24} sm={12} lg={8}>
          <StatCard 
            type="tasks" 
            value={stats.tasks} 
            label="Active Tasks" 
          />
        </Col>
        <Col xs={24} sm={12} lg={8}>
          <StatCard 
            type="completed" 
            value={stats.completed} 
            label="Completed" 
          />
        </Col>
      </Row>
    </div>
  );
};

export default Dashboard;