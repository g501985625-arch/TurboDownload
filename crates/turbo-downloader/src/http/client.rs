use crate::Result;
use reqwest::Client as ReqwestClient;

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub timeout: std::time::Duration,
    pub user_agent: String,
    pub max_connections: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: std::time::Duration::from_secs(300),
            user_agent: "TurboDownload/1.0".to_string(),
            max_connections: 32,
        }
    }
}

/// HTTP client
#[derive(Clone)]
#[allow(dead_code)]
pub struct Client {
    inner: ReqwestClient,
    config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let inner = ReqwestClient::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .pool_max_idle_per_host(config.max_connections)
            .build()?;

        Ok(Self { inner, config })
    }

    pub fn with_defaults() -> Result<Self> {
        Self::new(ClientConfig::default())
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
