import { Typography } from 'antd';

const { Title, Text } = Typography;

const Download = () => {
  return (
    <div style={{ padding: 24 }}>
      <Title level={2}>下载管理</Title>
      <Text type="secondary">管理您的下载任务</Text>
    </div>
  );
};

export default Download;