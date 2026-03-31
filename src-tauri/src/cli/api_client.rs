//! API Client for TurboDownload CLI
//! 通过 HTTP API 与 TurboDownload 服务通信

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// API 客户端
pub struct ApiClient {
    client: Client,
    base_url: String,
}

/// 下载任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub id: String,
    pub filename: String,
    pub status: String,
    pub url: String,
    pub created_at: String,
}

/// 下载状态信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStatus {
    pub task_id: String,
    pub progress: f64,
    pub speed: u64,
    pub downloaded: u64,
    pub total: u64,
    pub status: String,
}

/// 开始下载请求
#[derive(Debug, Serialize)]
struct StartDownloadRequest {
    url: String,
    filename: Option<String>,
    threads: u32,
}

/// 开始下载响应
#[derive(Debug, Deserialize)]
struct StartDownloadResponse {
    task_id: String,
}

impl ApiClient {
    /// 创建新的 API 客户端
    pub fn new(port: u16) -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://127.0.0.1:{}/api/v1", port),
        }
    }

    /// 创建新的 API 客户端（自定义地址）
    pub fn with_url(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// 开始下载
    pub async fn start_download(
        &self,
        url: &str,
        filename: Option<&str>,
        threads: u32,
    ) -> Result<String, String> {
        let request = StartDownloadRequest {
            url: url.to_string(),
            filename: filename.map(|s| s.to_string()),
            threads,
        };

        let response = self
            .client
            .post(&format!("{}/download", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 错误: {}", response.status()));
        }

        let result: StartDownloadResponse = response
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        Ok(result.task_id)
    }

    /// 暂停下载
    pub async fn pause_download(&self, task_id: &str) -> Result<(), String> {
        let response = self
            .client
            .post(&format!("{}/download/{}/pause", self.base_url, task_id))
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 错误: {}", response.status()));
        }

        Ok(())
    }

    /// 恢复下载
    pub async fn resume_download(&self, task_id: &str) -> Result<(), String> {
        let response = self
            .client
            .post(&format!("{}/download/{}/resume", self.base_url, task_id))
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 错误: {}", response.status()));
        }

        Ok(())
    }

    /// 取消下载
    pub async fn cancel_download(&self, task_id: &str) -> Result<(), String> {
        let response = self
            .client
            .delete(&format!("{}/download/{}", self.base_url, task_id))
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 错误: {}", response.status()));
        }

        Ok(())
    }

    /// 列出所有下载任务
    pub async fn list_downloads(&self) -> Result<Vec<DownloadTask>, String> {
        let response = self
            .client
            .get(&format!("{}/downloads", self.base_url))
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 错误: {}", response.status()));
        }

        let tasks: Vec<DownloadTask> = response
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        Ok(tasks)
    }

    /// 获取下载状态
    pub async fn get_download_status(&self, task_id: &str) -> Result<DownloadStatus, String> {
        let response = self
            .client
            .get(&format!("{}/download/{}/status", self.base_url, task_id))
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 错误: {}", response.status()));
        }

        let status: DownloadStatus = response
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_client_creation() {
        let client = ApiClient::new(8080);
        assert_eq!(client.base_url, "http://127.0.0.1:8080/api/v1");

        let client2 = ApiClient::with_url("http://localhost:3000/api");
        assert_eq!(client2.base_url, "http://localhost:3000/api");
    }
}