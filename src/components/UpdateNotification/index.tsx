/**
 * UpdateNotification Component
 * 
 * Displays an update notification modal when a new version is available
 */

import React from 'react';
import { Modal, Button, Typography, Tag } from 'antd';
import { DownloadOutlined, ClockCircleOutlined, StepForwardOutlined, BulbOutlined, CloudDownloadOutlined } from '@ant-design/icons';
import { X } from 'lucide-react';

export interface UpdateInfo {
  version: string;
  currentVersion: string;
  notes: string;
  date: string;
  downloadUrl?: string; // Optional download URL for the update
}

interface UpdateNotificationProps {
  updateInfo: UpdateInfo;
  isOpen: boolean;
  onUpdate: () => void;
  onLater: () => void;
  onSkip: () => void;
  onClose: () => void;
}

const UpdateNotification: React.FC<UpdateNotificationProps> = ({
  updateInfo,
  isOpen,
  onUpdate,
  onLater,
  onSkip,
  onClose,
}) => {
  const { Title, Paragraph, Text } = Typography;

  return (
    <Modal
      open={isOpen}
      onCancel={onClose}
      footer={null}
      closable={true}
      centered
      width={480}
      zIndex={1000}
      keyboard={true}
      maskClosable={true}
      destroyOnClose={true}
    >
      <div style={{ padding: '24px' }}>
        {/* Header */}
        <div style={{ position: 'relative', marginBottom: '24px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <div style={{ fontSize: '24px', color: '#1890ff' }}>
              <BulbOutlined />
            </div>
            <div>
              <Title level={4} style={{ margin: 0, color: '#fff' }}>新版本可用</Title>
              <Text style={{ color: 'rgba(255, 255, 255, 0.8)' }}>
                v{updateInfo.currentVersion} → v{updateInfo.version}
              </Text>
            </div>
          </div>
          <button
            onClick={onClose}
            style={{
              position: 'absolute',
              top: 0,
              right: 0,
              border: 'none',
              background: 'transparent',
              fontSize: '16px',
              color: 'rgba(255, 255, 255, 0.6)',
              cursor: 'pointer'
            }}
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div>
          {/* Version Info */}
          <div style={{ 
            display: 'flex', 
            justifyContent: 'space-between', 
            marginBottom: '16px',
            padding: '12px',
            backgroundColor: '#f6ffed',
            borderRadius: '6px'
          }}>
            <div style={{ fontSize: '14px' }}>
              <Text type="secondary">发布日期：</Text>
              <Text strong>{updateInfo.date}</Text>
            </div>
            <Tag color="blue">新版本</Tag>
          </div>

          {/* Update Notes */}
          <div style={{ marginBottom: '24px' }}>
            <Text strong style={{ marginBottom: '8px', display: 'block' }}>更新内容</Text>
            <div 
              style={{ 
                maxHeight: '128px', 
                overflowY: 'auto', 
                padding: '12px', 
                backgroundColor: '#f5f5f5', 
                borderRadius: '6px',
                fontSize: '14px'
              }}
            >
              <Paragraph style={{ margin: 0, lineHeight: '1.6' }} type="secondary">
                {updateInfo.notes}
              </Paragraph>
            </div>
          </div>

          {/* Action Buttons */}
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            {updateInfo.downloadUrl ? (
              <Button
                type="primary"
                icon={<CloudDownloadOutlined />}
                size="large"
                block
                href={updateInfo.downloadUrl}
                target="_blank"
                rel="noopener noreferrer"
                style={{ marginBottom: '8px' }}
              >
                下载更新包
              </Button>
            ) : (
              <Button
                type="primary"
                icon={<DownloadOutlined />}
                size="large"
                block
                onClick={onUpdate}
                style={{ marginBottom: '8px' }}
              >
                立即更新
              </Button>
            )}
            
            <div style={{ display: 'flex', gap: '8px' }}>
              <Button
                icon={<ClockCircleOutlined />}
                size="large"
                style={{ flex: 1 }}
                onClick={onLater}
              >
                稍后提醒
              </Button>
              <Button
                icon={<StepForwardOutlined />}
                size="large"
                style={{ flex: 1 }}
                onClick={onSkip}
              >
                跳过此版本
              </Button>
            </div>
          </div>
        </div>
      </div>
    </Modal>
  );
};

export default UpdateNotification;
