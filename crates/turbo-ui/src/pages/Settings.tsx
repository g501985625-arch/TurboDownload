import { useEffect, useState } from 'react';
import { Typography, Switch, Form, Input, Card, Button, Slider, message, Divider } from 'antd';
import { FolderOutlined, SaveOutlined, ReloadOutlined } from '@ant-design/icons';
import { useDownloadStore } from '../store/downloadStore';

const { Title, Text } = Typography;

const Settings = () => {
  const [form] = Form.useForm();
  const [isLoading, setIsLoading] = useState(false);
  
  // Store state
  const defaultThreads = useDownloadStore((state) => state.defaultThreads);
  const defaultDownloadPath = useDownloadStore((state) => state.defaultDownloadPath);
  const speedLimit = useDownloadStore((state) => state.speedLimit);
  const setDefaultThreads = useDownloadStore((state) => state.setDefaultThreads);
  const setDefaultDownloadPath = useDownloadStore((state) => state.setDefaultDownloadPath);
  const setSpeedLimit = useDownloadStore((state) => state.setSpeedLimit);
  
  // Local settings state
  const [autoStart, setAutoStart] = useState(false);
  const [minimizeToTray, setMinimizeToTray] = useState(true);
  const [showNotifications, setShowNotifications] = useState(true);
  const [autoResume, setAutoResume] = useState(true);
  
  // Initialize form with store values
  useEffect(() => {
    form.setFieldsValue({
      downloadPath: defaultDownloadPath || './downloads',
      threads: defaultThreads,
      speedLimit: speedLimit / 1024 / 1024, // Convert to MB/s
    });
  }, [defaultDownloadPath, defaultThreads, speedLimit, form]);
  
  const handleSave = async (values: {
    downloadPath: string;
    threads: number;
    speedLimit: number;
  }) => {
    setIsLoading(true);
    
    try {
      // Update store
      setDefaultDownloadPath(values.downloadPath);
      setDefaultThreads(values.threads);
      setSpeedLimit(values.speedLimit * 1024 * 1024); // Convert MB/s to bytes/s
      
      message.success('设置已保存');
    } catch (error) {
      message.error('保存设置失败');
    } finally {
      setIsLoading(false);
    }
  };
  
  const handleReset = () => {
    form.setFieldsValue({
      downloadPath: './downloads',
      threads: 4,
      speedLimit: 0,
    });
    setAutoStart(false);
    setMinimizeToTray(true);
    setShowNotifications(true);
    setAutoResume(true);
    message.info('设置已重置为默认值');
  };
  
  return (
    <div style={{ padding: 24, maxWidth: 800 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 24 }}>
        <div>
          <Title level={2} style={{ marginBottom: 4 }}>设置</Title>
          <Text type="secondary">配置您的下载管理器</Text>
        </div>
        <div style={{ display: 'flex', gap: 12 }}>
          <Button
            icon={<ReloadOutlined />}
            onClick={handleReset}
          >
            重置
          </Button>
          <Button
            type="primary"
            icon={<SaveOutlined />}
            onClick={() => form.submit()}
            loading={isLoading}
          >
            保存设置
          </Button>
        </div>
      </div>
      
      <Form
        form={form}
        layout="vertical"
        onFinish={handleSave}
        initialValues={{
          downloadPath: defaultDownloadPath || './downloads',
          threads: defaultThreads,
          speedLimit: speedLimit / 1024 / 1024,
        }}
      >
        {/* 下载设置 */}
        <Card title="下载设置" style={{ marginBottom: 16 }}>
          <Form.Item
            name="downloadPath"
            label="默认下载目录"
            rules={[{ required: true, message: '请输入下载目录' }]}
          >
            <Input
              prefix={<FolderOutlined />}
              placeholder="./downloads"
              size="large"
            />
          </Form.Item>
          
          <Form.Item
            name="threads"
            label="默认线程数"
            rules={[{ required: true, min: 1, max: 32 }]}
            extra="每个下载任务使用的线程数，建议 4-8 线程"
          >
            <Slider
              min={1}
              max={16}
              marks={{
                1: '1',
                4: '4',
                8: '8',
                12: '12',
                16: '16',
              }}
            />
          </Form.Item>
          
          <Form.Item
            name="speedLimit"
            label="速度限制 (MB/s)"
            extra="0 表示不限速"
          >
            <Slider
              min={0}
              max={100}
              marks={{
                0: '不限速',
                10: '10',
                50: '50',
                100: '100',
              }}
            />
          </Form.Item>
          
          <Form.Item label="自动恢复下载">
            <Switch
              checked={autoResume}
              onChange={setAutoResume}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              启动时自动恢复未完成的下载
            </Text>
          </Form.Item>
        </Card>
        
        {/* 应用设置 */}
        <Card title="应用设置" style={{ marginBottom: 16 }}>
          <Form.Item label="开机自启">
            <Switch
              checked={autoStart}
              onChange={setAutoStart}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              系统启动时自动运行 TurboDownload
            </Text>
          </Form.Item>
          
          <Form.Item label="最小化到系统托盘">
            <Switch
              checked={minimizeToTray}
              onChange={setMinimizeToTray}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              关闭窗口时最小化到系统托盘而不是退出
            </Text>
          </Form.Item>
          
          <Form.Item label="下载完成通知">
            <Switch
              checked={showNotifications}
              onChange={setShowNotifications}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              下载完成或失败时显示系统通知
            </Text>
          </Form.Item>
        </Card>
        
        {/* 网络设置 */}
        <Card title="网络设置">
          <Form.Item label="User-Agent">
            <Input
              defaultValue="TurboDownload/1.0"
              placeholder="自定义 User-Agent"
            />
          </Form.Item>
          
          <Form.Item label="连接超时 (秒)">
            <Slider
              min={5}
              max={120}
              defaultValue={30}
              marks={{
                5: '5s',
                30: '30s',
                60: '60s',
                120: '120s',
              }}
            />
          </Form.Item>
          
          <Form.Item label="重试次数">
            <Slider
              min={0}
              max={10}
              defaultValue={3}
              marks={{
                0: '0',
                3: '3',
                5: '5',
                10: '10',
              }}
            />
          </Form.Item>
        </Card>
      </Form>
      
      <Divider />
      
      {/* 关于 */}
      <Card title="关于 TurboDownload" size="small">
        <Text>版本: 1.0.0</Text>
        <br />
        <Text type="secondary">一个快速的多线程下载管理器</Text>
        <br />
        <Text type="secondary">
          <a href="https://github.com/g501985625-arch/TurboDownload" target="_blank" rel="noopener noreferrer">
            GitHub 仓库
          </a>
        </Text>
      </Card>
    </div>
  );
};

export default Settings;