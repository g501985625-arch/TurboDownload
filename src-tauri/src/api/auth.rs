use axum::{
    extract::State,
    http::{
        header::AUTHORIZATION,
        Request, StatusCode,
    },
    middleware::Next,
    response::Response,
    body::Body,
};
use std::net::IpAddr;
use std::sync::Arc;
use subtle::ConstantTimeEq;
use tokio::sync::RwLock;
use std::collections::VecDeque;
use std::time::Instant;

/// 认证配置
pub struct AuthConfig {
    pub token: String,
    pub allowed_ips: Vec<IpAddr>,
    pub enable_auth: bool,
}

/// 简单的速率限制器
pub struct RateLimiter {
    requests: VecDeque<(IpAddr, Instant)>,
    max_requests: usize,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: VecDeque::new(),
            max_requests,
            window_secs,
        }
    }
    
    pub fn check(&mut self, ip: IpAddr) -> bool {
        let now = Instant::now();
        
        // 清理过期记录
        while let Some(&(req_ip, time)) = self.requests.front() {
            if now.duration_since(time).as_secs() > self.window_secs {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        
        // 检查该 IP 的请求数
        let count = self.requests.iter()
            .filter(|(req_ip, _)| *req_ip == ip)
            .count();
        
        if count >= self.max_requests {
            return false;
        }
        
        // 记录新请求
        self.requests.push_back((ip, now));
        true
    }
}

impl AuthConfig {
    pub fn generate_token() -> String {
        // 生成随机 token
        uuid::Uuid::new_v4().to_string()
    }
}

/// Token 验证中间件
pub async fn auth_middleware(
    config: State<Arc<RwLock<AuthConfig>>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let config_guard = config.read().await;
    if !config_guard.enable_auth {
        drop(config_guard); // 提前释放读锁
        return Ok(next.run(request).await);
    }
    
    // 验证 IP 白名单
    if let Some(ip) = request.extensions().get::<IpAddr>() {
        if !config_guard.allowed_ips.is_empty() && !config_guard.allowed_ips.contains(ip) {
            drop(config_guard); // 提前释放读锁
            return Err(StatusCode::FORBIDDEN);
        }
    }
    
    // 验证 Token (使用常量时间比较防止时序攻击)
    if let Some(auth_header) = request.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                // 使用 subtle::ConstantTimeEq 进行安全的字符串比较
                let config_token = config_guard.token.as_bytes();
                let auth_token = token.as_bytes();
                
                // 常量时间比较
                let is_equal = bool::from(auth_token.ct_eq(config_token));
                
                if is_equal {
                    drop(config_guard); // 提前释放读锁
                    return Ok(next.run(request).await);
                }
            }
        }
    }
    
    drop(config_guard); // 释放读锁
    Err(StatusCode::UNAUTHORIZED)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_time_eq() {
        let a = "same-token";
        let b = "same-token";
        let c = "different";
        
        assert!(bool::from(a.as_bytes().ct_eq(b.as_bytes())));
        assert!(!bool::from(a.as_bytes().ct_eq(c.as_bytes())));
    }
}