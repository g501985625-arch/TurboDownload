use crate::Result;
use reqwest::Client as ReqwestClient;
use std::time::Duration;

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub timeout: Duration,
    pub user_agent: String,
    pub max_connections: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300),
            user_agent: "TurboDownload/1.0".to_string(),
            max_connections: 32,
        }
    }
}

/// Privacy configuration for HTTP client
#[derive(Debug, Clone)]
pub struct PrivacyClientConfig {
    pub base_config: ClientConfig,
    pub use_system_proxy: bool,
    pub custom_dns_servers: Vec<String>,
    pub bypass_proxy: bool,
    pub disable_certificate_verification: bool,
    pub random_user_agent: bool,
    pub no_logs: bool,
}

impl Default for PrivacyClientConfig {
    fn default() -> Self {
        Self {
            base_config: ClientConfig::default(),
            use_system_proxy: false,
            custom_dns_servers: vec![],
            bypass_proxy: true,
            disable_certificate_verification: false,
            random_user_agent: true,
            no_logs: true,
        }
    }
}

impl From<crate::privacy::config::PrivacyConfig> for PrivacyClientConfig {
    fn from(privacy_config: crate::privacy::config::PrivacyConfig) -> Self {
        Self {
            base_config: ClientConfig::default(),
            use_system_proxy: privacy_config.use_system_proxy,
            custom_dns_servers: privacy_config.custom_dns_servers,
            bypass_proxy: privacy_config.bypass_proxy,
            disable_certificate_verification: privacy_config.disable_certificate_verification,
            random_user_agent: privacy_config.random_user_agent,
            no_logs: privacy_config.no_logs,
        }
    }
}

/// HTTP client
#[derive(Clone)]
#[allow(dead_code)]
pub struct Client {
    inner: ReqwestClient,
    config: PrivacyClientConfig,
}

impl Client {
    pub fn new(config: PrivacyClientConfig) -> Result<Self> {
        let mut builder = ReqwestClient::builder()
            .timeout(config.base_config.timeout)
            .user_agent(&config.base_config.user_agent)
            .pool_max_idle_per_host(config.base_config.max_connections);

        // Apply privacy settings
        if config.disable_certificate_verification {
            builder = builder.danger_accept_invalid_certs(true);
        }

        // Handle custom DNS servers if provided
        if !config.custom_dns_servers.is_empty() {
            // Note: reqwest doesn't directly support custom DNS servers
            // This would require using lower-level hyper APIs or a resolver configuration
            // For now, we'll log this requirement
            log::info!("Custom DNS servers configured: {:?}", config.custom_dns_servers);
        }

        // Handle proxy settings
        if config.bypass_proxy {
            // Explicitly disable proxy usage
            builder = builder.no_proxy();
        } else if !config.use_system_proxy {
            // Don't set any proxy (use default behavior)
        }
        // If use_system_proxy is true, we let reqwest use system proxy by default

        let inner = builder.build()?;

        Ok(Self { inner, config })
    }

    pub fn with_defaults() -> Result<Self> {
        Self::new(PrivacyClientConfig::default())
    }

    /// Perform HEAD request
    pub async fn head(&self, url: &str) -> Result<super::response::HeadResponse> {
        let response = self.inner.head(url).send().await?;
        let status = response.status().as_u16();
        let headers = response.headers();

        Ok(super::response::HeadResponse::from_headers(status, headers))
    }

    /// Perform range request
    pub async fn get_range(&self, url: &str, range: std::ops::Range<u64>) -> Result<bytes::Bytes> {
        let range_header = format!("bytes={}-{}", range.start, range.end.saturating_sub(1));

        let response = self
            .inner
            .get(url)
            .header("Range", range_header)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() && status.as_u16() != 206 {
            return Err(crate::DownloadError::Http(
                status.as_u16(),
                status.to_string(),
            ));
        }

        let bytes = response.bytes().await?;
        Ok(bytes)
    }
}