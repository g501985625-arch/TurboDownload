use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use turbo_downloader::Manager;

/// WebSocket 事件类型
#[derive(Clone, Debug, serde::Serialize)]
pub enum DownloadEvent {
    Progress {
        task_id: String,
        downloaded: u64,
        total: u64,
        speed: u64,
    },
    Completed {
        task_id: String,
        filename: String,
    },
    Error {
        task_id: String,
        message: String,
    },
    Paused {
        task_id: String,
    },
    Resumed {
        task_id: String,
    },
}

/// WebSocket 状态
pub struct WsState {
    pub tx: broadcast::Sender<DownloadEvent>,
}

/// 组合状态：包含 WebSocket 状态和下载管理器（用于初始状态同步）
pub struct WsStateWithManager {
    pub ws_state: Arc<WsState>,
    pub download_manager: Arc<Mutex<Manager>>,
}

impl WsState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }
}

/// WebSocket 处理器（仅 WebSocket 状态）
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    state: Arc<WsState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state, None))
}

/// WebSocket 处理器（带下载管理器，用于初始状态同步）
pub async fn ws_handler_with_manager(
    ws: WebSocketUpgrade,
    state: Arc<WsState>,
    download_manager: Arc<Mutex<Manager>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state, Some(download_manager)))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<WsState>, download_manager: Option<Arc<Mutex<Manager>>>) {
    // 修复: 添加初始状态同步
    if let Some(ref manager) = download_manager {
        let mgr = manager.lock().await;
        let task_ids = mgr.list_tasks();
        
        // 提前释放锁，避免跨越 await
        let tasks: Vec<_> = task_ids.into_iter()
            .filter_map(|task_id| mgr.get_task(&task_id).map(|t| (task_id, t)))
            .collect();
        drop(mgr);
        
        for (task_id, task) in tasks {
            let _progress = if task.file_size > 0 {
                (task.downloaded as f64 / task.file_size as f64) * 100.0
            } else {
                0.0
            };
            
            let event = DownloadEvent::Progress {
                task_id: task_id.clone(),
                downloaded: task.downloaded,
                total: task.file_size,
                speed: task.speed(),
            };
            
            let msg = serde_json::to_string(&event).unwrap();
            let _ = socket.send(axum::extract::ws::Message::Text(msg)).await;
        }
    }
    
    let mut rx = state.tx.subscribe();
    
    while let Ok(event) = rx.recv().await {
        let msg = serde_json::to_string(&event).unwrap();
        if socket.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
            // 修复: 添加连接断开日志
            log::info!("WebSocket client disconnected");
            break;
        }
    }
    
    log::info!("WebSocket handler finished for client");
}

/// 发送事件到所有客户端
pub fn broadcast_event(state: &WsState, event: DownloadEvent) {
    let _ = state.tx.send(event);
}