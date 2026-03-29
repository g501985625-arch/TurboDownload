use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;

/// HEAD request response
#[derive(Debug, Clone)]
pub struct HeadResponse {
    pub status: u16,
    pub content_length: Option<u64>,
    pub accept_ranges: Option<String>,
    pub etag: Option<String>,
    pub content_type: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
}

impl HeadResponse {
    pub fn from_headers(status: u16, headers: &HeaderMap) -> Self {
        Self {
            status,
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
                .and_then(|v| DateTime::parse_from_rfc2822(v).ok())
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }

    /// Check if server supports range requests
    pub fn supports_range(&self) -> bool {
        self.accept_ranges.as_deref() == Some("bytes")
    }
}
