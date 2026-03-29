use crate::error::DownloadError;
use crate::Result;
use bytes::Bytes;
use reqwest::Client;
use std::time::Duration;
use crate::range::RangeSupport;

/// RangeClient 配置
#[derive(Debug, Clone)]
pub struct RangeClientConfig {
    /// 请求超时时间
    pub timeout: Duration,
    /// 重试次数
    pub retry_count: u32,
    /// 用户代理
    pub user_agent: String,
}

impl Default for RangeClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300),
            retry_count: 3,
            user_agent: "TurboDownload/1.0".to_string(),
        }
    }
}

/// Range HTTP 客户端
///
/// 用于发送 Range 请求，支持多线程下载和断点续传
#[derive(Clone)]
pub struct RangeClient {
    inner: Client,
    config: RangeClientConfig,
}

impl RangeClient {
    /// 创建新的 RangeClient
    pub fn new(config: RangeClientConfig) -> Result<Self> {
        let inner = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()?;

        Ok(Self { inner, config })
    }

    /// 使用默认配置创建 RangeClient
    pub fn with_defaults() -> Result<Self> {
        Self::new(RangeClientConfig::default())
    }

    /// 检测服务器是否支持 Range 请求
    ///
    /// 发送 HEAD 请求，检查响应头中的 Accept-Ranges 字段
    pub async fn check_range_support(&self, url: &str) -> Result<RangeSupport> {
        let response = self.inner.head(url).send().await?;

        let headers = response.headers();
        let status = response.status();

        Ok(RangeSupport {
            supported: status.is_success(),
            content_length: headers
                .get("content-length")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
            accept_ranges: headers
                .get("accept-ranges")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            etag: headers
                .get("etag")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            content_type: headers
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            last_modified: headers
                .get("last-modified")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
        })
    }

    /// 获取文件总大小
    ///
    /// 通过 HEAD 请求获取 Content-Length
    pub async fn get_content_length(&self, url: &str) -> Result<u64> {
        let support = self.check_range_support(url).await?;

        support
            .content_length
            .ok_or(DownloadError::ValidationFailed(
                "Content-Length not found in response".to_string(),
            ))
    }

    /// 下载指定范围的数据
    ///
    /// # 参数
    /// - `url`: 下载地址
    /// - `start`: 起始字节位置（包含）
    /// - `end`: 结束字节位置（不包含）
    ///
    /// # 返回
    /// 返回指定范围的字节数据
    pub async fn fetch_range(&self, url: &str, start: u64, end: u64) -> Result<Bytes> {
        let range_header = format!("bytes={}-{}", start, end.saturating_sub(1));

        let response = self
            .inner
            .get(url)
            .header("Range", range_header)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() && status.as_u16() != 206 {
            return Err(DownloadError::Http(
                status.as_u16(),
                status.to_string(),
            ));
        }

        let bytes = response.bytes().await?;
        Ok(bytes)
    }

    /// 从指定位置开始下载到文件末尾（断点续传用）
    ///
    /// # 参数
    /// - `url`: 下载地址
    /// - `start`: 起始字节位置（包含）
    ///
    /// # 返回
    /// 从 start 位置到文件末尾的所有字节数据
    pub async fn fetch_from(&self, url: &str, start: u64) -> Result<Bytes> {
        let response = self
            .inner
            .get(url)
            .header("Range", format!("bytes={}-", start))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() && status.as_u16() != 206 {
            return Err(DownloadError::Http(
                status.as_u16(),
                status.to_string(),
            ));
        }

        let bytes = response.bytes().await?;
        Ok(bytes)
    }

    /// 获取配置引用
    pub fn config(&self) -> &RangeClientConfig {
        &self.config
    }

    /// 获取内部 reqwest Client 引用
    pub fn inner(&self) -> &Client {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = RangeClientConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(300));
        assert_eq!(config.retry_count, 3);
        assert_eq!(config.user_agent, "TurboDownload/1.0");
    }

    #[test]
    fn test_range_client_creation() {
        let client = RangeClient::with_defaults();
        assert!(client.is_ok());
    }
}