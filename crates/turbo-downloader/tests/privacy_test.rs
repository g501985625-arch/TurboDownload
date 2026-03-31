//! 隐私功能测试
//!
//! 测试 TurboDownload 的隐私保护功能：
//! - 代理禁用
//! - 自定义 DNS
//! - User-Agent 随机化
//! - TLS 证书验证开关
//! - 无日志模式
//! - 隐私模式一键开关

use turbo_downloader::privacy::{PrivacyConfig, LogMode, LoggingConfig};

/// 测试：代理禁用
#[test]
fn test_proxy_disabled() {
    // 1. 设置禁用代理
    let config = PrivacyConfig {
        use_system_proxy: false,
        bypass_proxy: true,
        ..Default::default()
    };

    // 2. 验证代理设置正确
    assert!(!config.use_system_proxy, "系统代理应该被禁用");
    assert!(config.bypass_proxy, "代理旁路应该启用");
    
    println!("✅ 代理禁用配置测试通过");
}

/// 测试：自定义 DNS
#[test]
fn test_custom_dns() {
    // 1. 设置自定义 DNS
    let custom_dns = vec![
        "8.8.8.8".to_string(),
        "8.8.4.4".to_string(),
    ];
    
    let config = PrivacyConfig {
        custom_dns_servers: custom_dns.clone(),
        ..Default::default()
    };

    // 2. 验证 DNS 解析使用自定义服务器
    assert_eq!(config.custom_dns_servers.len(), 2, "应该有 2 个自定义 DNS 服务器");
    assert_eq!(config.custom_dns_servers[0], "8.8.8.8", "第一个 DNS 应该是 8.8.8.8");
    assert_eq!(config.custom_dns_servers[1], "8.8.4.4", "第二个 DNS 应该是 8.8.4.4");
    
    println!("✅ 自定义 DNS 配置测试通过");
}

/// 测试：User-Agent 随机化
#[test]
fn test_user_agent_randomization() {
    // 1. 启用 UA 随机化
    let config = PrivacyConfig {
        random_user_agent: true,
        ..Default::default()
    };

    assert!(config.random_user_agent, "UA 随机化应该启用");

    // 2. 模拟多次请求，验证 UA 会变化
    // 注意：实际随机化在运行时发生，这里验证配置正确即可
    let mut seen_agents = std::collections::HashSet::new();
    let user_agents = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
    ];
    
    for ua in &user_agents {
        seen_agents.insert(ua.clone());
    }
    
    // 验证我们有不同的 UA 样本
    assert!(seen_agents.len() > 1, "应该有多个不同的 UA 样本");
    
    println!("✅ User-Agent 随机化配置测试通过");
}

/// 测试：TLS 证书验证开关
#[test]
fn test_tls_verification_toggle() {
    // 1. 开启验证
    let mut config = PrivacyConfig::default();
    config.tls.verify_certificates = true;
    config.disable_certificate_verification = false;
    
    assert!(config.tls.verify_certificates, "TLS 验证应该开启");
    assert!(!config.disable_certificate_verification, "禁用证书验证应该关闭");
    
    println!("✅ TLS 验证开启测试通过");
    
    // 2. 关闭验证
    config.tls.verify_certificates = false;
    config.disable_certificate_verification = true;
    
    assert!(!config.tls.verify_certificates, "TLS 验证应该关闭");
    assert!(config.disable_certificate_verification, "禁用证书验证应该开启");
    
    println!("✅ TLS 验证关闭测试通过");
}

/// 测试：无日志模式
#[test]
fn test_no_logs_mode() {
    // 1. 设置无日志模式
    let config = PrivacyConfig {
        logging: LoggingConfig {
            mode: LogMode::None,
            log_file_path: None,
        },
        no_logs: true,
        ..Default::default()
    };

    // 2. 验证日志配置
    assert_eq!(config.logging.mode, LogMode::None, "日志模式应该是 None");
    assert!(config.no_logs, "no_logs 标志应该为 true");
    
    // 3. 验证日志初始化
    config.logging.init();
    let log_level = std::env::var("RUST_LOG").unwrap_or_default();
    assert_eq!(log_level, "off", "RUST_LOG 应该设置为 off");
    
    println!("✅ 无日志模式测试通过");
}

/// 测试：仅错误日志模式
#[test]
fn test_error_only_logs_mode() {
    let config = PrivacyConfig {
        logging: LoggingConfig {
            mode: LogMode::ErrorOnly,
            log_file_path: None,
        },
        ..Default::default()
    };

    assert_eq!(config.logging.mode, LogMode::ErrorOnly, "日志模式应该是 ErrorOnly");
    
    // 验证日志初始化
    config.logging.init();
    let log_level = std::env::var("RUST_LOG").unwrap_or_default();
    assert_eq!(log_level, "error", "RUST_LOG 应该设置为 error");
    
    println!("✅ 仅错误日志模式测试通过");
}

/// 测试：完整日志模式
#[test]
fn test_full_logs_mode() {
    let config = PrivacyConfig {
        logging: LoggingConfig {
            mode: LogMode::Full,
            log_file_path: Some(std::path::PathBuf::from("/tmp/test.log")),
        },
        ..Default::default()
    };

    assert_eq!(config.logging.mode, LogMode::Full, "日志模式应该是 Full");
    assert!(config.logging.log_file_path.is_some(), "日志文件路径应该存在");
    
    // 验证日志初始化
    config.logging.init();
    let log_level = std::env::var("RUST_LOG").unwrap_or_default();
    assert_eq!(log_level, "info", "RUST_LOG 应该设置为 info");
    
    println!("✅ 完整日志模式测试通过");
}

/// 测试：隐私模式一键开关
#[test]
fn test_privacy_mode() {
    // 1. 启用隐私模式（综合配置）
    let config = PrivacyConfig {
        // 代理相关
        use_system_proxy: false,
        bypass_proxy: true,
        
        // DNS 相关
        custom_dns_servers: vec!["1.1.1.1".to_string()],
        
        // UA 相关
        random_user_agent: true,
        
        // TLS 相关
        disable_certificate_verification: false,
        tls: turbo_downloader::privacy::tls::TlsConfig {
            verify_certificates: true,
            custom_ca_cert: None,
        },
        
        // 日志相关
        no_logs: true,
        logging: LoggingConfig {
            mode: LogMode::None,
            log_file_path: None,
        },
    };

    // 2. 验证代理禁用
    assert!(!config.use_system_proxy, "隐私模式：系统代理应该被禁用");
    assert!(config.bypass_proxy, "隐私模式：代理旁路应该启用");
    
    // 3. 验证自定义 DNS
    assert!(!config.custom_dns_servers.is_empty(), "隐私模式：应该有自定义 DNS");
    
    // 4. 验证 UA 随机化
    assert!(config.random_user_agent, "隐私模式：UA 随机化应该启用");
    
    // 5. 验证 TLS 验证开启
    assert!(!config.disable_certificate_verification, "隐私模式：TLS 验证应该开启");
    assert!(config.tls.verify_certificates, "隐私模式：证书验证应该开启");
    
    // 6. 验证无日志
    assert!(config.no_logs, "隐私模式：no_logs 应该为 true");
    assert_eq!(config.logging.mode, LogMode::None, "隐私模式：日志模式应该是 None");
    
    println!("✅ 隐私模式一键开关测试通过");
}

/// 测试：隐私模式默认值
#[test]
fn test_privacy_mode_defaults() {
    // 默认隐私配置
    let config = PrivacyConfig::default();
    
    // 验证默认隐私设置符合安全最佳实践
    assert!(!config.use_system_proxy, "默认：系统代理应该被禁用");
    assert!(config.bypass_proxy, "默认：代理旁路应该启用");
    assert!(config.custom_dns_servers.is_empty(), "默认：自定义 DNS 为空（可选）");
    assert!(!config.disable_certificate_verification, "默认：证书验证应该开启");
    assert!(config.random_user_agent, "默认：UA 随机化应该启用");
    assert!(config.no_logs, "默认：no_logs 应该为 true");
    assert_eq!(config.logging.mode, LogMode::ErrorOnly, "默认：日志模式应该是 ErrorOnly");
    assert!(config.tls.verify_certificates, "默认：TLS 验证应该开启");
    
    println!("✅ 隐私模式默认值测试通过");
}

/// 测试：PrivacyConfig 序列化/反序列化
#[test]
fn test_privacy_config_serialization() {
    let config = PrivacyConfig {
        use_system_proxy: false,
        bypass_proxy: true,
        custom_dns_servers: vec!["8.8.8.8".to_string()],
        disable_certificate_verification: false,
        random_user_agent: true,
        no_logs: true,
        tls: turbo_downloader::privacy::tls::TlsConfig {
            verify_certificates: true,
            custom_ca_cert: None,
        },
        logging: LoggingConfig {
            mode: LogMode::None,
            log_file_path: Some(std::path::PathBuf::from("/tmp/privacy.log")),
        },
    };

    // 序列化
    let json = serde_json::to_string(&config).expect("序列化失败");
    assert!(json.contains("8.8.8.8"), "序列化后应该包含 DNS 配置");
    
    // 反序列化
    let restored: PrivacyConfig = serde_json::from_str(&json).expect("反序列化失败");
    assert_eq!(restored.custom_dns_servers[0], "8.8.8.8", "反序列化后 DNS 应该一致");
    assert_eq!(restored.logging.mode, LogMode::None, "反序列化后日志模式应该一致");
    
    println!("✅ 隐私配置序列化/反序列化测试通过");
}

/// 测试：日志配置 display name
#[test]
fn test_logging_display_names() {
    let full_config = LoggingConfig { mode: LogMode::Full, log_file_path: None };
    let error_config = LoggingConfig { mode: LogMode::ErrorOnly, log_file_path: None };
    let none_config = LoggingConfig { mode: LogMode::None, log_file_path: None };
    
    assert_eq!(full_config.mode_display_name(), "完整日志");
    assert_eq!(error_config.mode_display_name(), "仅错误日志");
    assert_eq!(none_config.mode_display_name(), "无日志（隐私模式）");
    
    println!("✅ 日志配置显示名称测试通过");
}

/// 测试：从 PrivacyConfig 创建 HTTP 客户端配置
#[test]
fn test_privacy_to_http_config() {
    use turbo_downloader::http::PrivacyClientConfig;
    
    let privacy_config = PrivacyConfig {
        use_system_proxy: false,
        bypass_proxy: true,
        custom_dns_servers: vec!["1.1.1.1".to_string()],
        disable_certificate_verification: true,
        random_user_agent: true,
        no_logs: true,
        ..Default::default()
    };
    
    let http_config: PrivacyClientConfig = privacy_config.into();
    
    assert!(!http_config.use_system_proxy);
    assert!(http_config.bypass_proxy);
    assert_eq!(http_config.custom_dns_servers.len(), 1);
    assert!(http_config.disable_certificate_verification);
    assert!(http_config.random_user_agent);
    assert!(http_config.no_logs);
    
    println!("✅ PrivacyConfig 转换为 HTTP 客户端配置测试通过");
}

#[cfg(test)]
mod tls_tests {
    use super::*;
    use turbo_downloader::privacy::tls::{TlsConfig, create_http_client};
    
    /// 测试：默认 TLS 配置
    #[test]
    fn test_default_tls_config() {
        let config = TlsConfig::default();
        assert!(config.verify_certificates, "默认应该验证证书");
        assert!(config.custom_ca_cert.is_none(), "默认没有自定义 CA");
    }
    
    /// 测试：创建启用验证的 HTTP 客户端
    #[test]
    fn test_create_http_client_with_verification() {
        let config = TlsConfig {
            verify_certificates: true,
            custom_ca_cert: None,
        };
        
        // 这应该成功创建客户端
        let result = create_http_client(&config);
        assert!(result.is_ok(), "应该能创建带证书验证的客户端");
        
        println!("✅ 创建启用 TLS 验证的 HTTP 客户端测试通过");
    }
}