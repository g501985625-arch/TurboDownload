/**
 * UpdateProgress Component
 * 
 * Displays download progress for updates
 */

import React from 'react';
import { Modal, Button, Progress, Typography, Space } from 'antd';
import { DownloadOutlined, CheckCircleOutlined, CloseCircleOutlined, RedoOutlined } from '@ant-design/icons';

interface UpdateProgressProps {
  isOpen: boolean;
  progress: number;
  status: 'downloading' | 'completed' | 'error';
  errorMessage?: string;
  version: string;
  onCancel: () => void;
  onClose: () => void;
  onRetry?: () => void;
}

const UpdateProgress: React.FC<UpdateProgressProps> = ({
  isOpen,
  progress,
  status,
  errorMessage,
  version,
  onCancel,
  onClose,
  onRetry = () => {}, // 默认空函数
}) => {
  const { Title, Text, Paragraph } = Typography;

  const getStatusIcon = () => {
    switch (status) {
      case 'completed':
        return <CheckCircleOutlined style={{ fontSize: '48px', color: '#52c41a' }} />;
      case 'error':
        return <CloseCircleOutlined style={{ fontSize: '48px', color: '#ff4d4f' }} />;
      default:
        return <DownloadOutlined spin style={{ fontSize: '48px', color: '#1890ff' }} />;
    }
  };

  const getStatusText = () => {
    switch (status) {
      case 'completed':
        return '下载完成';
      case 'error':
        return '下载失败';
      default:
        return `正在下载 v${version}...`;
    }
  };

  const getStatusDescription = () => {
    switch (status) {
      case 'completed':
        return '更新包已下载完成，请重启应用以完成更新。';
      case 'error':
        return errorMessage || '下载过程中出现错误，请稍后重试。';
      default:
        return `已下载 ${progress.toFixed(1)}%`;
    }
  };

  return (
    <Modal
      open={isOpen}
      onCancel={status !== 'downloading' ? onClose : undefined}
      footer={null}
      closable={status !== 'downloading'}
      centered
      width={400}
      zIndex={1000}
    >
      <div style={{ textAlign: 'center', padding: '24px' }}>
        {/* Status Icon */}
        <div style={{ marginBottom: '16px' }}>
          {getStatusIcon()}
        </div>

        {/* Status Title */}
        <Title level={4} style={{ marginBottom: '8px' }}>
          {getStatusText()}
        </Title>

        {/* Status Description */}
        <Paragraph type="secondary" style={{ marginBottom: '24px' }}>
          {getStatusDescription()}
        </Paragraph>

        {/* Progress Bar - Only show when downloading */}
        {status === 'downloading' && (
          <div style={{ marginBottom: '24px' }}>
            <Space direction="vertical" size="small" style={{ width: '100%' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                <Text type="secondary">下载进度</Text>
                <Text strong type="secondary">{progress.toFixed(0)}%</Text>
              </div>
              <Progress percent={Math.round(progress)} status="active" />
            </Space>
          </div>
        )}

        {/* Action Buttons */}
        <Space direction="vertical" size="middle" style={{ width: '100%' }}>
          {status === 'downloading' && (
            <Button
              size="large"
              block
              onClick={onCancel}
            >
              取消下载
            </Button>
          )}
          
          {status === 'completed' && (
            <Button
              type="primary"
              size="large"
              block
              onClick={onClose}
            >
              知道了
            </Button>
          )}
          
          {status === 'error' && onRetry && (
            <Space size="small" style={{ width: '100%', display: 'flex' }}>
              <Button
                type="primary"
                size="large"
                icon={<RedoOutlined />}
                onClick={onRetry}
                style={{ flex: 1 }}
              >
                重试
              </Button>
              <Button
                size="large"
                onClick={onClose}
                style={{ flex: 1 }}
              >
                关闭
              </Button>
            </Space>
          )}
          
          {status === 'error' && !onRetry && (
            <Button
              size="large"
              block
              onClick={onClose}
            >
              关闭
            </Button>
          )}
        </Space>
      </div>
    </Modal>
  );
};

export default UpdateProgress;
