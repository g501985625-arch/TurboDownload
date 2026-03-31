use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};
use serde_json::json;

use crate::api::server::{WsState, DownloadEvent, broadcast_event};
use turbo_downloader::Manager;

pub struct DownloadMonitor {
    download_manager: Arc<Mutex<Manager>>,
    ws_state: Arc<WsState>,
}

impl DownloadMonitor {
    pub fn new(download_manager: Arc<Mutex<Manager>>, ws_state: Arc<WsState>) -> Self {
        Self {
            download_manager,
            ws_state,
        }
    }

    pub async fn start_monitoring(&self) {
        let download_manager = self.download_manager.clone();
        let ws_state = self.ws_state.clone();

        tokio::spawn(async move {
            let mut previous_states = std::collections::HashMap::new();

            loop {
                sleep(Duration::from_millis(500)).await; // 每500毫秒检查一次

                let manager = download_manager.lock().await;
                let task_ids = manager.list_tasks();
                
                for task_id in task_ids {
                    if let Some(task) = manager.get_task(&task_id) {
                        let current_progress = if task.file_size > 0 {
                            (task.downloaded as f64 / task.file_size as f64) * 100.0
                        } else {
                            0.0
                        };
                        
                        let current_speed = task.speed();
                        let current_status = format!("{:?}", task.state());
                        
                        let previous_data = previous_states.get(&task_id);
                        
                        // 检查进度是否发生变化
                        if let Some(&(prev_progress, prev_speed, ref prev_status)) = previous_data {
                            if prev_progress != current_progress || prev_speed != current_speed {
                                // 发送进度更新事件
                                let event = DownloadEvent::Progress {
                                    task_id: task_id.clone(),
                                    downloaded: task.downloaded,
                                    total: task.file_size,
                                    speed: current_speed,
                                };
                                
                                broadcast_event(&ws_state, event);
                            }
                            
                            // 检查状态变化
                            if prev_status != &current_status {
                                match current_status.as_str() {
                                    "Completed" => {
                                        let filename = task.config.output_path
                                            .file_name()
                                            .map(|n| n.to_string_lossy().to_string())
                                            .unwrap_or_else(|| "unknown".to_string());
                                            
                                        let event = DownloadEvent::Completed {
                                            task_id: task_id.clone(),
                                            filename,
                                        };
                                        
                                        broadcast_event(&ws_state, event);
                                        
                                        // 修复: 任务完成后清理 HashMap，防止内存泄漏
                                        previous_states.remove(&task_id);
                                        continue;
                                    },
                                    "Error" => {
                                        let event = DownloadEvent::Error {
                                            task_id: task_id.clone(),
                                            message: "Download failed".to_string(),
                                        };
                                        
                                        broadcast_event(&ws_state, event);
                                        
                                        // 修复: 任务错误时清理 HashMap，防止内存泄漏
                                        previous_states.remove(&task_id);
                                        continue;
                                    },
                                    "Paused" => {
                                        let event = DownloadEvent::Paused {
                                            task_id: task_id.clone(),
                                        };
                                        
                                        broadcast_event(&ws_state, event);
                                    },
                                    "Downloading" => {
                                        let event = DownloadEvent::Resumed {
                                            task_id: task_id.clone(),
                                        };
                                        
                                        broadcast_event(&ws_state, event);
                                    },
                                    _ => {}
                                }
                            }
                        }
                        
                        // 更新之前的状态
                        previous_states.insert(task_id, (current_progress, current_speed, current_status));
                    }
                }
                
                drop(manager); // 释放锁
            }
        });
    }
}