import { Empty } from 'antd';
import DownloadItem, { type DownloadStatus } from './DownloadItem';

export type { DownloadStatus };

export interface DownloadTask {
  id: string;
  filename: string;
  size: number; // bytes
  progress: number; // 0-100
  speed?: number; // bytes per second
  status: DownloadStatus;
}

interface DownloadListProps {
  tasks: DownloadTask[];
  onPause?: (id: string) => void;
  onResume?: (id: string) => void;
  onCancel?: (id: string) => void;
}

const DownloadList: React.FC<DownloadListProps> = ({
  tasks,
  onPause,
  onResume,
  onCancel,
}) => {
  if (!tasks || tasks.length === 0) {
    return (
      <Empty
        image={Empty.PRESENTED_IMAGE_SIMPLE}
        description="暂无下载任务"
        style={{ padding: '48px 0' }}
      />
    );
  }

  return (
    <div style={{ width: '100%' }}>
      {tasks.map((task) => (
        <DownloadItem
          key={task.id}
          filename={task.filename}
          size={task.size}
          progress={task.progress}
          speed={task.speed}
          status={task.status}
          onPause={onPause ? () => onPause(task.id) : undefined}
          onResume={onResume ? () => onResume(task.id) : undefined}
          onCancel={onCancel ? () => onCancel(task.id) : undefined}
        />
      ))}
    </div>
  );
};

export default DownloadList;