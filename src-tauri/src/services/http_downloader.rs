//! HTTP Downloader Service
//! 
//! Handles HTTP/HTTPS downloads with progress tracking and resumable support

use anyhow::Context;
use futures::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

use crate::models::{DownloadConfig, DownloadProgress, DownloadStatus, Result, AppError};
use crate::privacy::{random_user_agent, UserAgentPool, PrivacyConfig};
use crate::privacy::tls::{TlsConfig, create_http_client};

/// Progress callback type
pub type ProgressCallback = Arc<dyn Fn(DownloadProgress) + Send + Sync>;

/// HTTP Downloader
pub struct HttpDownloader {
    /// HTTP client
    client: reqwest::Client,
    /// Progress callback
    progress_callback: Option<ProgressCallback>,
    /// User-Agent pool
    ua_pool: UserAgentPool,
    /// Privacy config
    privacy_config: PrivacyConfig,
}

impl HttpDownloader {
    /// Create a new HTTP downloader with default settings
    pub fn new() -> Self {
        Self::with_privacy_configs(PrivacyConfig::default())
    }

    /// Create a new HTTP downloader with custom privacy config
    pub fn with_privacy_config(privacy_config: PrivacyConfig) -> Self {
        let user_agent = if privacy_config.random_user_agent {
            random_user_agent()
        } else if let Some(ref custom) = privacy_config.custom_user_agent {
            custom.clone()
        } else {
            "TurboDownload/1.0".to_string()
        };

        // 使用TLS配置创建HTTP客户端
        let base_client = create_http_client(&privacy_config.tls)
            .expect("Failed to create HTTP client");
        
        let client = base_client
            .user_agent(&user_agent)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            progress_callback: None,
            ua_pool: UserAgentPool::new(),
            privacy_config,
        }
    }

    /// Create a new HTTP downloader with custom privacy config (alternative name for compatibility)
    pub fn with_privacy_configs(privacy_config: PrivacyConfig) -> Self {
        Self::with_privacy_config(privacy_config)
    }

    /// Set progress callback
    pub fn with_progress_callback(mut self, callback: ProgressCallback) -> Self {
        self.progress_callback = Some(callback);
        self
    }

    /// Refresh User-Agent (for new download session)
    fn get_user_agent(&self) -> String {
        if self.privacy_config.random_user_agent {
            self.ua_pool.random()
        } else if let Some(ref custom) = self.privacy_config.custom_user_agent {
            custom.clone()
        } else {
            "TurboDownload/1.0".to_string()
        }
    }

    /// Get file info from URL (size, filename)
    pub async fn get_file_info(&self, url: &str) -> Result<(Option<u64>, Option<String>)> {
        let user_agent = self.get_user_agent();
        
        let response = self
            .client
            .head(url)
            .header(reqwest::header::USER_AGENT, &user_agent)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Failed to get file info: {}", e)))?;

        let size = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        let filename = response
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                // Parse filename from Content-Disposition
                v.split("filename=")
                    .nth(1)
                    .map(|s| s.trim().trim_matches('"').to_string())
            });

        Ok((size, filename))
    }

    /// Download a file with progress tracking
    pub async fn download(
        &self,
        url: &str,
        output_path: PathBuf,
        config: &DownloadConfig,
        cancel_flag: Arc<RwLock<bool>>,
        task_id: String,
    ) -> Result<()> {
        // Get User-Agent for this download
        let user_agent = self.get_user_agent();
        
        // Create the request
        let mut request = self.client.get(url);
        
        // Set User-Agent header
        request = request.header(reqwest::header::USER_AGENT, &user_agent);
        
        // Add custom headers
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }

        // Send request
        let response = request
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Failed to start download: {}", e)))?;

        // Check response status
        if !response.status().is_success() {
            return Err(AppError::DownloadError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        // Get total size
        let total_size = response
            .content_length()
            .unwrap_or(0);

        // Create parent directory if needed
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .context("Failed to create output directory")?;
        }

        // Create output file
        let mut file = File::create(&output_path)
            .await
            .context("Failed to create output file")?;

        // Download with progress
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();
        let start_time = std::time::Instant::now();
        let mut last_progress_update = start_time;

        while let Some(chunk) = stream.next().await {
            // Check cancel flag
            if *cancel_flag.read().await {
                return Err(AppError::DownloadError("Download cancelled".to_string()));
            }

            let chunk = chunk.context("Failed to read chunk")?;
            file.write_all(&chunk)
                .await
                .context("Failed to write chunk")?;

            downloaded += chunk.len() as u64;

            // Update progress (throttle to 10 updates/sec)
            let now = std::time::Instant::now();
            if now.duration_since(last_progress_update).as_millis() > 100 {
                last_progress_update = now;
                
                if let Some(callback) = &self.progress_callback {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = if elapsed > 0.0 {
                        (downloaded as f64 / elapsed) as u64
                    } else {
                        0
                    };

                    let progress = if total_size > 0 {
                        (downloaded as f64 / total_size as f64) * 100.0
                    } else {
                        0.0
                    };

                    let eta = if speed > 0 && total_size > downloaded {
                        Some((total_size - downloaded) / speed)
                    } else {
                        None
                    };

                    callback(DownloadProgress {
                        id: task_id.clone(),
                        progress,
                        speed,
                        total_size,
                        downloaded,
                        eta,
                        status: DownloadStatus::Downloading,
                    });
                }
            }
        }

        // Ensure file is synced
        file.sync_all()
            .await
            .context("Failed to sync file")?;

        // Final progress update
        if let Some(callback) = &self.progress_callback {
            callback(DownloadProgress {
                id: task_id.clone(),
                progress: 100.0,
                speed: 0,
                total_size,
                downloaded,
                eta: None,
                status: DownloadStatus::Completed,
            });
        }

        Ok(())
    }
}

impl Default for HttpDownloader {
    fn default() -> Self {
        Self::new()
    }
}