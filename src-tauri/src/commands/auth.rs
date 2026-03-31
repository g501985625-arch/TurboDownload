use tauri::{State, command};
use std::sync::Arc;
use tokio::sync::RwLock; // 使用tokio的RwLock以支持异步
use crate::api::{
    auth::AuthConfig,
    token::{save_token, load_token}
};

use std::net::IpAddr;

#[command]
pub async fn generate_auth_token(auth_config: State<'_, Arc<RwLock<AuthConfig>>>) -> Result<String, String> {
    let new_token = AuthConfig::generate_token();
    
    // 保存到文件
    save_token(&new_token)?;
    
    // 更新配置
    {
        let mut config = auth_config.write().await;
        config.token = new_token.clone();
    }
    
    Ok(new_token)
}

#[command]
pub async fn get_auth_token() -> Result<String, String> {
    // 从文件加载token
    match load_token() {
        Ok(token) => Ok(token),
        Err(_) => {
            // 如果文件不存在，返回空字符串
            Ok(String::new())
        }
    }
}

#[command]
pub async fn set_allowed_ips(ips: Vec<String>, auth_config: State<'_, Arc<RwLock<AuthConfig>>>) -> Result<(), String> {
    let parsed_ips: Result<Vec<IpAddr>, _> = ips.iter()
        .map(|ip_str| ip_str.parse::<IpAddr>())
        .collect();
    
    let valid_ips = parsed_ips.map_err(|e| format!("Invalid IP address: {}", e))?;
    
    // 更新配置
    {
        let mut config = auth_config.write().await;
        config.allowed_ips = valid_ips;
    }
    
    Ok(())
}

#[command]
pub async fn toggle_auth(enabled: bool, auth_config: State<'_, Arc<RwLock<AuthConfig>>>) -> Result<(), String> {
    // 更新配置
    {
        let mut config = auth_config.write().await;
        config.enable_auth = enabled;
    }
    
    Ok(())
}

#[command]
pub async fn get_auth_status(auth_config: State<'_, Arc<RwLock<AuthConfig>>>) -> Result<AuthStatus, String> {
    let config = auth_config.read().await;
    Ok(AuthStatus {
        enabled: config.enable_auth,
        token_exists: !config.token.is_empty(),
        allowed_ips: config.allowed_ips.clone(),
    })
}

#[derive(serde::Serialize)]
struct AuthStatus {
    enabled: bool,
    token_exists: bool,
    allowed_ips: Vec<IpAddr>,
}