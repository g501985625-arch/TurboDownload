import { Typography } from 'antd';

const { Title, Text } = Typography;

const Radar = () => {
  return (
    <div style={{ padding: 24 }}>
      <Title level={2}>雷达</Title>
      <Text type="secondary">发现可下载的资源</Text>
    </div>
  );
};

export default Radar;