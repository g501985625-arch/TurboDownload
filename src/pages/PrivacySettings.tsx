import { useState, useEffect } from 'react';
import { Typography, Switch, Form, Input, Card, Button, message, Divider, Radio, Alert } from 'antd';
import { SecurityScanOutlined, BlockOutlined, GlobalOutlined, LockOutlined, FileTextOutlined } from '@ant-design/icons';
import { useDownloadStore } from '../store/downloadStore';

const { Title, Text } = Typography;

const PrivacySettings = () => {
  const [form] = Form.useForm();
  
  // 代理设置状态
  const [useSystemProxy, setUseSystemProxy] = useState(false);
  const [disableAllProxy, setDisableAllProxy] = useState(true);
  
  // DNS 设置状态
  const [dnsServer, setDnsServer] = useState('');
  
  // User-Agent 设置状态
  const [enableUARandomization, setEnableUARandomization] = useState(true);
  const [customUA, setCustomUA] = useState('');
  
  // TLS 设置状态
  const [verifyCert, setVerifyCert] = useState(true);
  
  // 日志设置状态
  const [logMode, setLogMode] = useState<'none' | 'error' | 'full'>('error');
  
  // 隐私模式状态
  const [privacyMode, setPrivacyMode] = useState(false);

  // 加载现有设置
  useEffect(() => {
    // 尝试从本地存储加载现有设置
    const savedUseSystemProxy = localStorage.getItem('useSystemProxy');
    const savedDisableAllProxy = localStorage.getItem('disableAllProxy');
    const savedDnsServer = localStorage.getItem('dnsServer');
    const savedEnableUARandomization = localStorage.getItem('enableUARandomization');
    const savedCustomUA = localStorage.getItem('customUA');
    const savedVerifyCert = localStorage.getItem('verifyCert');
    const savedLogMode = localStorage.getItem('logMode');
    const savedPrivacyMode = localStorage.getItem('privacyMode');

    if (savedUseSystemProxy !== null) setUseSystemProxy(savedUseSystemProxy === 'true');
    if (savedDisableAllProxy !== null) setDisableAllProxy(savedDisableAllProxy === 'true');
    if (savedDnsServer) setDnsServer(savedDnsServer);
    if (savedEnableUARandomization !== null) setEnableUARandomization(savedEnableUARandomization === 'true');
    if (savedCustomUA) setCustomUA(savedCustomUA);
    if (savedVerifyCert !== null) setVerifyCert(savedVerifyCert === 'true');
    if (savedLogMode) setLogMode(savedLogMode as 'none' | 'error' | 'full');
    if (savedPrivacyMode !== null) setPrivacyMode(savedPrivacyMode === 'true');
  }, []);

  // 当隐私模式切换时，同步相关设置
  useEffect(() => {
    if (privacyMode) {
      // 启用隐私模式时，自动调整相关设置
      setDisableAllProxy(true);
      setEnableUARandomization(true);
      setLogMode('none');
    }
  }, [privacyMode]);

  const handleSave = async () => {
    try {
      // 保存到本地存储
      localStorage.setItem('useSystemProxy', String(useSystemProxy));
      localStorage.setItem('disableAllProxy', String(disableAllProxy));
      localStorage.setItem('dnsServer', dnsServer);
      localStorage.setItem('enableUARandomization', String(enableUARandomization));
      localStorage.setItem('customUA', customUA);
      localStorage.setItem('verifyCert', String(verifyCert));
      localStorage.setItem('logMode', logMode);
      localStorage.setItem('privacyMode', String(privacyMode));

      // 尝试调用后端API保存配置
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const privacyConfig = {
          use_system_proxy: useSystemProxy,
          custom_dns_servers: dnsServer ? [dnsServer] : [],
          bypass_proxy: disableAllProxy,
          disable_certificate_verification: !verifyCert,
          random_user_agent: enableUARandomization,
          no_logs: logMode === 'none', // 保持兼容性
          tls: {
            verify_certificates: verifyCert,
            custom_ca_cert: null,
          },
          logging: {
            mode: logMode === 'full' ? 'Full' : logMode === 'error' ? 'ErrorOnly' : 'None',
            log_file_path: null,
          }
        };
        await invoke('set_privacy_config', { config: privacyConfig });
        message.success('隐私设置已保存');
      } catch (apiError) {
        console.warn('无法调用后端API，使用本地存储作为备选方案:', apiError);
        message.success('隐私设置已保存到本地');
      }
    } catch (error) {
      message.error('保存设置失败');
      console.error('保存隐私设置时出错:', error);
    }
  };

  const handleReset = () => {
    setUseSystemProxy(false);
    setDisableAllProxy(true);
    setDnsServer('');
    setEnableUARandomization(true);
    setCustomUA('');
    setVerifyCert(true);
    setLogMode('error');
    setPrivacyMode(false);
    
    localStorage.removeItem('useSystemProxy');
    localStorage.removeItem('disableAllProxy');
    localStorage.removeItem('dnsServer');
    localStorage.removeItem('enableUARandomization');
    localStorage.removeItem('customUA');
    localStorage.removeItem('verifyCert');
    localStorage.removeItem('logMode');
    localStorage.removeItem('privacyMode');
    
    message.info('隐私设置已重置为默认值');
  };

  // 尝试获取后端隐私配置
  useEffect(() => {
    const loadPrivacyConfig = async () => {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const config = await invoke('get_privacy_config');
        
        if (config) {
          setUseSystemProxy(config.use_system_proxy || false);
          setDisableAllProxy(config.bypass_proxy || true);
          setDnsServer(config.custom_dns_servers && config.custom_dns_servers.length > 0 ? config.custom_dns_servers[0] : '');
          setEnableUARandomization(config.random_user_agent || true);
          setVerifyCert(!config.disable_certificate_verification || true);
          setLogMode(config.no_logs ? 'none' : config.logging?.mode?.toLowerCase() || 'error');
          
          // 计算隐私模式状态（如果禁用了代理、启用了UA随机化且日志为none）
          const calculatedPrivacyMode = config.bypass_proxy && config.random_user_agent && config.no_logs;
          setPrivacyMode(calculatedPrivacyMode);
        }
      } catch (error) {
        console.warn('无法从后端获取隐私配置，使用本地存储或默认值:', error);
      }
    };

    loadPrivacyConfig();
  }, []);

  return (
    <div style={{ padding: 24, maxWidth: 800 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 24 }}>
        <div>
          <Title level={2} style={{ marginBottom: 4 }}>隐私设置</Title>
          <Text type="secondary">配置您的隐私和安全选项</Text>
        </div>
        <div style={{ display: 'flex', gap: 12 }}>
          <Button
            icon={<GlobalOutlined />}
            onClick={handleReset}
          >
            重置
          </Button>
          <Button
            type="primary"
            icon={<SecurityScanOutlined />}
            onClick={handleSave}
          >
            保存设置
          </Button>
        </div>
      </div>

      <Form
        layout="vertical"
        initialValues={{
          dnsServer,
          customUA,
          logMode
        }}
      >
        {/* 代理设置 */}
        <Card 
          title={
            <span>
              <GlobalOutlined /> 代理设置
            </span>
          } 
          style={{ marginBottom: 16 }}
        >
          <Form.Item label="使用系统代理">
            <Switch
              checked={useSystemProxy}
              onChange={setUseSystemProxy}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              使用系统的代理设置
            </Text>
          </Form.Item>
          
          <Form.Item label="禁用所有代理">
            <Switch
              checked={disableAllProxy}
              onChange={setDisableAllProxy}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              完全禁用所有代理功能
            </Text>
          </Form.Item>
        </Card>

        {/* DNS 设置 */}
        <Card 
          title={
            <span>
              <GlobalOutlined /> DNS 设置
            </span>
          } 
          style={{ marginBottom: 16 }}
        >
          <Form.Item label="自定义 DNS 服务器">
            <Input
              placeholder="自定义 DNS 服务器 (如: 8.8.8.8:53)"
              value={dnsServer}
              onChange={(e) => setDnsServer(e.target.value)}
              addonBefore="DNS:"
            />
          </Form.Item>
        </Card>

        {/* User-Agent 设置 */}
        <Card 
          title={
            <span>
              <GlobalOutlined /> User-Agent
            </span>
          } 
          style={{ marginBottom: 16 }}
        >
          <Form.Item label="启用 UA 随机化">
            <Switch
              checked={enableUARandomization}
              onChange={setEnableUARandomization}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              随机化 User-Agent 以避免被识别
            </Text>
          </Form.Item>
          
          {!enableUARandomization && (
            <Form.Item label="自定义 UA (可选)">
              <Input
                placeholder="自定义 UA (可选)"
                value={customUA}
                onChange={(e) => setCustomUA(e.target.value)}
              />
            </Form.Item>
          )}
        </Card>

        {/* TLS 设置 */}
        <Card 
          title={
            <span>
              <LockOutlined /> TLS/SSL
            </span>
          } 
          style={{ marginBottom: 16 }}
        >
          <Form.Item label="验证服务器证书">
            <Switch
              checked={verifyCert}
              onChange={setVerifyCert}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              验证服务器 SSL 证书的有效性
            </Text>
          </Form.Item>
          
          {!verifyCert && (
            <Alert 
              type="warning" 
              message="关闭证书验证存在安全风险" 
              showIcon
            />
          )}
        </Card>

        {/* 日志设置 */}
        <Card 
          title={
            <span>
              <FileTextOutlined /> 日志设置
            </span>
          } 
          style={{ marginBottom: 16 }}
        >
          <Form.Item label="日志级别">
            <Radio.Group 
              value={logMode} 
              onChange={(e) => setLogMode(e.target.value)}
              optionType="button"
              buttonStyle="solid"
              style={{ width: '100%' }}
            >
              <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                <Radio.Button 
                  value="none"
                  style={{ 
                    height: 'auto', 
                    padding: '8px 12px',
                    textAlign: 'left',
                    whiteSpace: 'normal'
                  }}
                >
                  <div>
                    <div><strong>无日志（隐私模式）</strong></div>
                    <div style={{ fontSize: 12, opacity: 0.7 }}>完全不记录任何日志</div>
                  </div>
                </Radio.Button>
                
                <Radio.Button 
                  value="error"
                  style={{ 
                    height: 'auto', 
                    padding: '8px 12px',
                    textAlign: 'left',
                    whiteSpace: 'normal'
                  }}
                >
                  <div>
                    <div><strong>仅错误日志（默认）</strong></div>
                    <div style={{ fontSize: 12, opacity: 0.7 }}>只记录错误信息</div>
                  </div>
                </Radio.Button>
                
                <Radio.Button 
                  value="full"
                  style={{ 
                    height: 'auto', 
                    padding: '8px 12px',
                    textAlign: 'left',
                    whiteSpace: 'normal'
                  }}
                >
                  <div>
                    <div><strong>完整日志</strong></div>
                    <div style={{ fontSize: 12, opacity: 0.7 }}>记录所有操作和调试信息</div>
                  </div>
                </Radio.Button>
              </div>
            </Radio.Group>
          </Form.Item>
        </Card>

        {/* 隐私模式一键开关 */}
        <Card 
          title={
            <span>
              <BlockOutlined /> 隐私模式
            </span>
          } 
          style={{ marginBottom: 16 }}
        >
          <Form.Item label="启用最大隐私保护">
            <Switch
              checked={privacyMode}
              onChange={setPrivacyMode}
              checkedChildren="开启"
              unCheckedChildren="关闭"
            />
            <Text type="secondary" style={{ marginLeft: 8 }}>
              一键禁用代理、启用 UA 随机化、关闭日志
            </Text>
          </Form.Item>
          
          <Text type="secondary" style={{ display: 'block', marginTop: 8 }}>
            启用后将自动应用以下设置：
          </Text>
          <ul style={{ marginTop: 4, paddingLeft: 20 }}>
            <li>禁用所有代理</li>
            <li>启用 UA 随机化</li>
            <li>日志级别设为"无"</li>
          </ul>
        </Card>
      </Form>
    </div>
  );
};

export default PrivacySettings;