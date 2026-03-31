use std::path::PathBuf;

/// TLS 配置
pub struct TlsConfig {
    pub verify_certificates: bool,  // 默认 true
    pub custom_ca_cert: Option<PathBuf>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            verify_certificates: true,
            custom_ca_cert: None,
        }
    }
}

/// 创建 HTTP 客户端（带 TLS 配置）
pub fn create_http_client(config: &TlsConfig) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let mut client_builder = reqwest::Client::builder();
    
    if !config.verify_certificates {
        // 禁用证书验证（用户选择）
        client_builder = client_builder.danger_accept_invalid_certs(true);
        log::warn!("TLS certificate verification disabled - security risk!");
    }
    
    if let Some(ca_path) = &config.custom_ca_cert {
        // 加载自定义 CA 证书
        let cert = std::fs::read(ca_path)?;
        let cert = reqwest::Certificate::from_pem(&cert)?;
        client_builder = client_builder.add_root_certificate(cert);
    }
    
    Ok(client_builder.build()?)
}