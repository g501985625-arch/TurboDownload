use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
    Router,
    routing::get,
};
use tokio::net::TcpListener;
use log::info;
use tokio::sync::Mutex;
use turbo_downloader::{Manager, Client};

// 引入认证模块
use crate::api::auth::{AuthConfig, auth_middleware};
use tokio::sync::RwLock;
use std::net::IpAddr;

// 导入WebSocket相关模块
pub use crate::api::websocket::{WsState, DownloadEvent, broadcast_event, WsStateWithManager, ws_handler_with_manager};

// 导入下载监控器
pub use crate::api::download_monitor::DownloadMonitor;

pub use crate::api::routes::download::{
    ApiState,
    start_download, 
    get_download_status, 
    pause_download, 
    resume_download, 
    cancel_download, 
    list_downloads
};

/// Agent API Server
pub struct AgentServer {
    addr: SocketAddr,
    state: Option<Arc<ApiState>>,
    ws_state: Option<Arc<WsState>>,
    auth_config: Arc<RwLock<AuthConfig>>,
}

impl AgentServer {
    pub fn new(port: u16) -> Self {
        Self {
            addr: SocketAddr::from(([127, 0, 0, 1], port)),
            state: None,
            ws_state: None,
            auth_config: Arc::new(RwLock::new(AuthConfig {
                token: String::new(),
                allowed_ips: vec![IpAddr::from([127, 0, 0, 1])], // 默认允许本地访问
                enable_auth: false, // 默认禁用认证
            })),
        }
    }

    /// Create server with shared download manager state
    pub fn with_state(port: u16, download_manager: Arc<Mutex<Manager>>) -> Self {
        let ws_state = Arc::new(WsState::new());
        let api_state = Arc::new(ApiState { download_manager, ws_state: ws_state.clone() });
        Self {
            addr: SocketAddr::from(([127, 0, 0, 1], port)),
            state: Some(api_state),
            ws_state: Some(ws_state),
            auth_config: Arc::new(RwLock::new(AuthConfig {
                token: String::new(),
                allowed_ips: vec![IpAddr::from([127, 0, 0, 1])], // 默认允许本地访问
                enable_auth: false, // 默认禁用认证
            })),
        }
    }

    /// Create server with both API and WebSocket states
    pub fn with_states(port: u16, download_manager: Arc<Mutex<Manager>>, ws_state: Arc<WsState>) -> Self {
        let api_state = Arc::new(ApiState { download_manager, ws_state: ws_state.clone() });
        Self {
            addr: SocketAddr::from(([127, 0, 0, 1], port)),
            state: Some(api_state),
            ws_state: Some(ws_state),
            auth_config: Arc::new(RwLock::new(AuthConfig {
                token: String::new(),
                allowed_ips: vec![IpAddr::from([127, 0, 0, 1])], // 默认允许本地访问
                enable_auth: false, // 默认禁用认证
            })),
        }
    }

    /// Create server with authentication config
    pub fn with_auth(port: u16, download_manager: Arc<Mutex<Manager>>, ws_state: Arc<WsState>, auth_config: AuthConfig) -> Self {
        let api_state = Arc::new(ApiState { download_manager, ws_state: ws_state.clone() });
        Self {
            addr: SocketAddr::from(([127, 0, 0, 1], port)),
            state: Some(api_state),
            ws_state: Some(ws_state),
            auth_config: Arc::new(RwLock::new(auth_config)),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ws_state = self.ws_state.clone();
        let app = create_app(self.state.clone(), ws_state.clone(), self.auth_config.clone());
        
        // 如果两个状态都存在，启动下载监控器
        if let (Some(api_state), Some(ws_state)) = (&self.state, &ws_state) {
            let download_manager = api_state.download_manager.clone();
            let monitor = DownloadMonitor::new(download_manager, ws_state.clone());
            monitor.start_monitoring().await;
        }
        
        info!("Starting Agent API server on http://{}", self.addr);
        
        let listener = TcpListener::bind(self.addr).await?;
        axum::serve(listener, app.into_make_service()).await?;
        Ok(())
    }
}

/// 创建 Axum Router（带可选状态）
pub fn create_app(state: Option<Arc<ApiState>>, ws_state: Option<Arc<WsState>>, auth_config: Arc<RwLock<AuthConfig>>) -> Router {
    // 克隆 ws_state 供后续使用
    let ws_state_clone = ws_state.clone();
    
    // 如果有状态，使用它；否则创建一个默认状态
    let api_state = state.unwrap_or_else(|| {
        let empty_ws = ws_state_clone.clone().unwrap_or_else(|| Arc::new(WsState::new()));
        // 注意：这里需要处理可能创建失败的情况
        let client = turbo_downloader::Client::new(turbo_downloader::http::PrivacyClientConfig::default())
            .unwrap_or_else(|_| {
                // 创建一个默认客户端
                turbo_downloader::Client::new(turbo_downloader::http::PrivacyClientConfig::default())
                    .expect("Failed to create HTTP client")
            });
        let manager = Manager::new(client, 3);
        Arc::new(ApiState {
            download_manager: Arc::new(Mutex::new(manager)),
            ws_state: empty_ws,
        })
    });
    
    let mut router = Router::new()
        // 健康检查
        .route("/health", get(health_check));

    // 如果有WebSocket状态，添加WebSocket路由（带初始状态同步）
    if let Some(ws_state) = ws_state_clone.clone() {
        let ws_state_clone = ws_state.clone();
        let dm_clone = api_state.download_manager.clone();
        router = router.route("/ws", get(move |ws| {
            let ws_state = ws_state_clone.clone();
            let dm = dm_clone.clone();
            crate::api::websocket::ws_handler_with_manager(ws, ws_state, dm)
        }));
    }

    // 添加 API 路由（不应用状态，让调用者应用）
    router = router
        // POST /api/v1/download - 创建下载
        .route("/api/v1/download", axum::routing::post(start_download))
        // GET /api/v1/download/:id - 获取状态
        .route("/api/v1/download/:id", axum::routing::get(get_download_status))
        // POST /api/v1/download/:id/pause - 暂停
        .route("/api/v1/download/:id/pause", axum::routing::post(pause_download))
        // POST /api/v1/download/:id/resume - 恢复
        .route("/api/v1/download/:id/resume", axum::routing::post(resume_download))
        // DELETE /api/v1/download/:id - 取消
        .route("/api/v1/download/:id", axum::routing::delete(cancel_download))
        // GET /api/v1/downloads - 列出所有
        .route("/api/v1/downloads", axum::routing::get(list_downloads))
        // 应用认证中间件（带状态）
        .layer(axum::middleware::from_fn_with_state(
            auth_config.clone(),
            auth_middleware,
        ));
    
    // 应用状态并返回
    router.with_state(api_state)
}

/// 健康检查处理器
async fn health_check() -> &'static str {
    "TurboDownload Agent API is running"
}

/// 创建 API 状态（供外部调用）
pub fn create_api_state() -> Result<(Arc<ApiState>, Arc<WsState>), String> {
    use turbo_downloader::http::PrivacyClientConfig;
    
    // 创建 HTTP 客户端
    let client = Client::new(PrivacyClientConfig::default())
        .map_err(|e| e.to_string())?;

    // 创建下载管理器
    let download_manager = Manager::new(client, 3);

    let ws_state = Arc::new(WsState::new());
    let api_state = Arc::new(ApiState {
        download_manager: Arc::new(Mutex::new(download_manager)),
        ws_state: ws_state.clone(),
    });

    Ok((api_state, ws_state))
}