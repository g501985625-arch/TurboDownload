import { Typography, Switch, Form, Input, Card } from 'antd';

const { Title, Text } = Typography;

const Settings = () => {
  return (
    <div style={{ padding: 24, maxWidth: 600 }}>
      <Title level={2}>设置</Title>
      <Text type="secondary">配置您的下载管理器</Text>
      
      <div style={{ marginTop: 24 }}>
        <Card title="常规设置" style={{ marginBottom: 16 }}>
          <Form layout="vertical">
            <Form.Item label="下载目录">
              <Input placeholder="/Users/macipad/Downloads" />
            </Form.Item>
            <Form.Item label="最大并发下载数">
              <Input type="number" defaultValue={3} />
            </Form.Item>
          </Form>
        </Card>
        
        <Card title="启动设置">
          <Form layout="vertical">
            <Form.Item label="开机自启">
              <Switch defaultChecked={false} />
            </Form.Item>
            <Form.Item label="最小化到系统托盘">
              <Switch defaultChecked={true} />
            </Form.Item>
          </Form>
        </Card>
      </div>
    </div>
  );
};

export default Settings;