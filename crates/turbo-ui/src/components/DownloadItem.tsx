import { Card, Typography, Space, Progress, Button } from 'antd';
import { 
  FileOutlined, 
  PauseCircleOutlined, 
  PlayCircleOutlined, 
  CloseCircleOutlined,
  CheckCircleFilled
} from '@ant-design/icons';

const { Text } = Typography;

export type DownloadStatus = 'downloading' | 'paused' | 'completed';

interface DownloadItemProps {
  filename: string;
  size: number; // bytes
  progress: number; // 0-100
  speed?: number; // bytes per second
  status: DownloadStatus;
  onPause?: () => void;
  onResume?: () => void;
  onCancel?: () => void;
}

// Format bytes to human readable
const formatSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

// Format speed to human readable
const formatSpeed = (bytesPerSecond: number): string => {
  if (!bytesPerSecond || bytesPerSecond === 0) return '';
  return formatSize(bytesPerSecond) + '/s';
};

const statusConfig = {
  downloading: {
    color: '#1890ff',
    background: '#e6f7ff',
    borderColor: '#91d5ff',
    text: '下载中',
  },
  paused: {
    color: '#fa8c16',
    background: '#fff7e6',
    borderColor: '#ffd591',
    text: '已暂停',
  },
  completed: {
    color: '#52c41a',
    background: '#f6ffed',
    borderColor: '#b7eb8f',
    text: '已完成',
  },
};

const DownloadItem: React.FC<DownloadItemProps> = ({
  filename,
  size,
  progress,
  speed,
  status,
  onPause,
  onResume,
  onCancel,
}) => {
  const config = statusConfig[status];
  const downloadedSize = Math.floor((size * progress) / 100);

  return (
    <Card
      style={{
        borderColor: config.borderColor,
        borderRadius: 12,
        marginBottom: 12,
        boxShadow: '0 2px 8px rgba(0,0,0,0.06)',
      }}
      styles={{ body: { padding: '16px 20px' } }}
    >
      <Space direction="vertical" size={12} style={{ width: '100%' }}>
        {/* File info row */}
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <Space size={12}>
            <div
              style={{
                width: 40,
                height: 40,
                borderRadius: 8,
                background: config.background,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                color: config.color,
                fontSize: 18,
              }}
            >
              {status === 'completed' ? <CheckCircleFilled /> : <FileOutlined />}
            </div>
            <div>
              <Text strong style={{ fontSize: 14, display: 'block', maxWidth: 300, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                {filename}
              </Text>
              <Text type="secondary" style={{ fontSize: 12 }}>
                {formatSize(downloadedSize)} / {formatSize(size)}
              </Text>
            </div>
          </Space>
          
          {/* Status badge */}
          <div
            style={{
              padding: '4px 12px',
              borderRadius: 12,
              background: config.background,
              color: config.color,
              fontSize: 12,
              fontWeight: 500,
              border: `1px solid ${config.borderColor}`,
            }}
          >
            {config.text}
          </div>
        </div>

        {/* Progress bar */}
        <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
          <Progress
            percent={Math.round(progress)}
            strokeColor={config.color}
            trailColor="#f0f0f0"
            showInfo={false}
            style={{ flex: 1, margin: 0 }}
          />
          {status === 'completed' && (
            <Text strong style={{ color: config.color, minWidth: 60, textAlign: 'right' }}>
              100%
            </Text>
          )}
          {status !== 'completed' && (
            <Text type="secondary" style={{ minWidth: 80, textAlign: 'right' }}>
              {Math.round(progress)}%
            </Text>
          )}
        </div>

        {/* Speed and actions row */}
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <Text type="secondary" style={{ fontSize: 12 }}>
            {status === 'downloading' && speed && (
              <span style={{ color: '#1890ff', fontWeight: 500 }}>
                {formatSpeed(speed)}
              </span>
            )}
            {status === 'paused' && '等待中...'}
            {status === 'completed' && '下载完成'}
          </Text>

          {/* Action buttons */}
          <Space size={8}>
            {status === 'downloading' && onPause && (
              <Button
                type="text"
                icon={<PauseCircleOutlined />}
                onClick={onPause}
                style={{ color: '#fa8c16' }}
              >
                暂停
              </Button>
            )}
            {status === 'paused' && onResume && (
              <Button
                type="text"
                icon={<PlayCircleOutlined />}
                onClick={onResume}
                style={{ color: '#1890ff' }}
              >
                继续
              </Button>
            )}
            {status !== 'completed' && onCancel && (
              <Button
                type="text"
                danger
                icon={<CloseCircleOutlined />}
                onClick={onCancel}
              >
                取消
              </Button>
            )}
          </Space>
        </div>
      </Space>
    </Card>
  );
};

export default DownloadItem;