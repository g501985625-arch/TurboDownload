// 隐私配置结构
use super::tls::TlsConfig;
use super::logging::LoggingConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrivacyConfig {
    pub use_system_proxy: bool,      // 默认 false
    pub custom_dns_servers: Vec<String>,  // 可选
    pub bypass_proxy: bool,          // 默认 true
    pub disable_certificate_verification: bool, // 默认 false
    pub random_user_agent: bool,     // 默认 true
    pub no_logs: bool,               // 默认 true (已废弃，使用 logging 代替)
    pub tls: TlsConfig,
    pub logging: LoggingConfig,      // 日志模式配置
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            use_system_proxy: false,
            custom_dns_servers: vec![],
            bypass_proxy: true,
            disable_certificate_verification: false,
            random_user_agent: true,
            no_logs: true,
            tls: TlsConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}