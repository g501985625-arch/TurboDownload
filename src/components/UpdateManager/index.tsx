import React, { useEffect } from 'react';
import { useUpdater } from '../../hooks/useUpdater';
import UpdateNotification from '../UpdateNotification';
import UpdateProgress from '../UpdateProgress';

interface UpdateManagerProps {
  autoCheckOnStart?: boolean;
}

const UpdateManager: React.FC<UpdateManagerProps> = ({ autoCheckOnStart = true }) => {
  const {
    updateInfo,
    progress,
    showUpdateNotification,
    showUpdateProgress,
    setShowUpdateNotification,
    setShowUpdateProgress,
    checkForUpdates,
    downloadUpdate
  } = useUpdater();

  // Auto-check for updates when component mounts (if enabled)
  useEffect(() => {
    if (autoCheckOnStart) {
      const timer = setTimeout(() => {
        checkForUpdates();
      }, 3000); // Delay check to not interfere with initial loading
  
      return () => clearTimeout(timer);
    }
  }, [autoCheckOnStart, checkForUpdates]);

  const handleUpdate = async () => {
    setShowUpdateNotification(false);
    await downloadUpdate();
  };

  const handleLater = () => {
    setShowUpdateNotification(false);
  };

  const handleSkip = () => {
    setShowUpdateNotification(false);
  };

  const handleCloseNotification = () => {
    setShowUpdateNotification(false);
  };

  const handleRetry = async () => {
    await downloadUpdate();
  };

  const handleCloseProgress = () => {
    setShowUpdateProgress(false);
  };

  const handleCancelDownload = () => {
    // Currently, we can't cancel the download from the frontend
    // The Rust implementation would need to support cancellation
    setShowUpdateProgress(false);
  };

  if (!updateInfo) {
    return null;
  }

  return (
    <>
      <UpdateNotification
        updateInfo={updateInfo}
        isOpen={showUpdateNotification}
        onUpdate={handleUpdate}
        onLater={handleLater}
        onSkip={handleSkip}
        onClose={handleCloseNotification}
      />
      
      <UpdateProgress
        isOpen={showUpdateProgress}
        progress={progress.percent}
        status={progress.status}
        errorMessage={progress.status === 'error' ? '下载更新失败，请检查网络连接' : undefined}
        version={updateInfo.version}
        onCancel={handleCancelDownload}
        onClose={handleCloseProgress}
        onRetry={handleRetry}
      />
    </>
  );
};

export default UpdateManager;