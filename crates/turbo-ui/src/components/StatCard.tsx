import { Card, Typography, Space } from 'antd';
import { 
  ThunderboltOutlined, 
  UnorderedListOutlined, 
  CheckCircleOutlined 
} from '@ant-design/icons';

const { Text, Title } = Typography;

type StatCardType = 'speed' | 'tasks' | 'completed';

interface StatCardProps {
  type: StatCardType;
  value: string | number;
  label: string;
}

const colorMap = {
  speed: {
    borderColor: '#1890ff',
    iconColor: '#1890ff',
    background: '#e6f7ff',
  },
  tasks: {
    borderColor: '#fa8c16',
    iconColor: '#fa8c16',
    background: '#fff7e6',
  },
  completed: {
    borderColor: '#52c41a',
    iconColor: '#52c41a',
    background: '#f6ffed',
  },
};

const iconMap = {
  speed: <ThunderboltOutlined />,
  tasks: <UnorderedListOutlined />,
  completed: <CheckCircleOutlined />,
};

const StatCard: React.FC<StatCardProps> = ({ type, value, label }) => {
  const colors = colorMap[type];
  const icon = iconMap[type];

  return (
    <Card
      style={{ 
        borderColor: colors.borderColor,
        borderRadius: 12,
        boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
      }}
      styles={{
        body: { padding: '20px' }
      }}
    >
      <Space direction="vertical" size={8} style={{ width: '100%' }}>
        <div style={{ 
          display: 'flex', 
          alignItems: 'center', 
          justifyContent: 'space-between' 
        }}>
          <div style={{
            width: 48,
            height: 48,
            borderRadius: 12,
            background: colors.background,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: 24,
            color: colors.iconColor,
          }}>
            {icon}
          </div>
        </div>
        <Title level={2} style={{ margin: 0, color: colors.iconColor }}>
          {value}
        </Title>
        <Text type="secondary">{label}</Text>
      </Space>
    </Card>
  );
};

export default StatCard;