use crate::privacy::tls::{TlsConfig, create_http_client};
use crate::state::AppState;
use tauri::State;
use std::sync::Mutex;

#[tauri::command]
pub async fn get_tls_config(state: State<'_, Mutex<AppState>>) -> Result<TlsConfig, String> {
    let app_state = state.lock().unwrap();
    Ok(app_state.config.privacy.tls.clone())
}

#[tauri::command]
pub async fn set_tls_config(config: TlsConfig, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut app_state = state.lock().unwrap();
    app_state.config.privacy.tls = config;
    
    // 如果证书验证被禁用，记录警告
    if !config.verify_certificates {
        log::warn!("TLS certificate verification disabled - security risk!");
    }
    
    // 更新全局HTTP客户端
    let client = create_http_client(&config)
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    app_state.http_client = client;
    
    Ok(())
}