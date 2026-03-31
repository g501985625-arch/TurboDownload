//! Bilibili platform adapter
//!
//! Provides media extraction from Bilibili using their public API

use super::{MediaResource, MediaQuality, VideoFormat, PlatformAdapter, Result};
use crate::{CrawlerError, CrawlerClient, Platform};
use regex::Regex;
use std::sync::OnceLock;
use serde::Deserialize;
use async_trait::async_trait;

/// BV ID extraction regex
static BV_REGEX: OnceLock<Regex> = OnceLock::new();
static AV_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_bv_regex() -> &'static Regex {
    BV_REGEX.get_or_init(|| Regex::new(r"(?i)BV[a-zA-Z0-9]{10}").unwrap())
}

fn get_av_regex() -> &'static Regex {
    AV_REGEX.get_or_init(|| Regex::new(r"(?i)av\d+").unwrap())
}

/// Bilibili API response structures
#[derive(Debug, Deserialize)]
struct BilibiliApiResponse<T: Default> {
    code: i32,
    message: String,
    #[serde(default)]
    data: Option<T>,
}

#[derive(Debug, Deserialize, Default)]
struct VideoInfoData {
    #[serde(default)]
    bvid: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    pic: Option<String>,
    #[serde(default)]
    duration: Option<String>,
    #[serde(default)]
    owner: Option<VideoOwner>,
    #[serde(default)]
    stat: Option<VideoStat>,
}

#[derive(Debug, Deserialize)]
struct VideoOwner {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    mid: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct VideoStat {
    #[serde(default)]
    view: Option<u64>,
    #[serde(default)]
    like: Option<u64>,
    #[serde(default)]
    coin: Option<u64>,
    #[serde(default)]
    favorite: Option<u64>,
    #[serde(default)]
    share: Option<u64>,
}

#[derive(Debug, Deserialize, Default)]
struct PlayUrlData {
    #[serde(default)]
    durl: Option<Vec<Durl>>,
    #[serde(default)]
    support_formats: Option<Vec<SupportFormat>>,
}

#[derive(Debug, Deserialize)]
struct Durl {
    #[serde(default)]
    order: Option<u32>,
    #[serde(default)]
    length: Option<u64>,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    md5: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SupportFormat {
    #[serde(default)]
    quality: Option<u32>,
    #[serde(default)]
    format: Option<String>,
    #[serde(default)]
    new_description: Option<String>,
}

/// Bilibili adapter
pub struct BilibiliAdapter {
    /// API base URL
    api_base: String,
    /// Use pgc (Bangumi/Series) API
    use_pgc: bool,
}

impl BilibiliAdapter {
    /// Create a new Bilibili adapter
    pub fn new() -> Self {
        Self {
            api_base: "https://api.bilibili.com".to_string(),
            use_pgc: false,
        }
    }

    /// Create with pgc mode
    pub fn with_pgc() -> Self {
        Self {
            api_base: "https://api.bilibili.com".to_string(),
            use_pgc: true,
        }
    }

    /// Extract BV ID from URL
    pub fn extract_bvid(url: &str) -> Option<String> {
        get_bv_regex()
            .find(url)
            .map(|m| m.as_str().to_uppercase())
    }

    /// Extract AV号 from URL
    pub fn extract_avid(url: &str) -> Option<u64> {
        get_av_regex()
            .find(url)
            .and_then(|m| {
                let s = m.as_str();
                s[2..].parse::<u64>().ok()
            })
    }

    /// Convert AV号 to BV号
    fn av_to_bv(avid: u64) -> String {
        // Bilibili uses a specific algorithm for BV conversion
        // This is a simplified version - in production, use a proper library
        format!("BV{:010}", avid)
    }

    /// Get video info from Bilibili API
    async fn get_video_info(&self, bvid: &str) -> Result<VideoInfoData> {
        let url = format!("{}/x/web-interface/view?bvid={}", self.api_base, bvid);
        
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| CrawlerError::Network(e.to_string()))?;
        
        let json: BilibiliApiResponse<VideoInfoData> = response.json()
            .await
            .map_err(|e| CrawlerError::Parse(e.to_string()))?;
        
        if json.code != 0 {
            return Err(CrawlerError::Parse(format!("API error: {}", json.message)));
        }
        
        json.data
            .ok_or_else(|| CrawlerError::Parse("No video data".to_string()))
    }

    /// Get play URL from Bilibili API
    async fn get_play_url(&self, bvid: &str, cid: u64) -> Result<PlayUrlData> {
        let url = format!(
            "{}/x/player/playurl?bvid={}&cid={}&qn=80&fnval=16",
            self.api_base, bvid, cid
        );
        
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| CrawlerError::Network(e.to_string()))?;
        
        let json: BilibiliApiResponse<PlayUrlData> = response.json()
            .await
            .map_err(|e| CrawlerError::Parse(e.to_string()))?;
        
        if json.code != 0 {
            return Err(CrawlerError::Parse(format!("API error: {}", json.message)));
        }
        
        json.data
            .ok_or_else(|| CrawlerError::Parse("No play URL data".to_string()))
    }

    /// Get CID for video (needed for play URL)
    async fn get_cid(&self, bvid: &str) -> Result<u64> {
        let url = format!("{}/x/web-interface/view?bvid={}", self.api_base, bvid);
        
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| CrawlerError::Network(e.to_string()))?;
        
        #[derive(Deserialize)]
        struct ViewResponse {
            code: i32,
            data: Option<ViewData>,
        }
        
        #[derive(Deserialize)]
        struct ViewData {
            cid: u64,
        }
        
        let json: ViewResponse = response.json()
            .await
            .map_err(|e| CrawlerError::Parse(e.to_string()))?;
        
        json.data
            .map(|d| d.cid)
            .ok_or_else(|| CrawlerError::Parse("No CID found".to_string()))
    }

    /// Map quality level to MediaQuality
    fn map_quality(qn: u32) -> MediaQuality {
        match qn {
            127 | 126 | 125 => MediaQuality::FourK,
            120 | 116 => MediaQuality::FullHD,
            80 | 112 | 74 => MediaQuality::High,
            64 | 32 | 16 => MediaQuality::Medium,
            6 | 3 => MediaQuality::Low,
            _ => MediaQuality::Medium,
        }
    }

    /// Extract media from Bilibili URL
    pub async fn extract_media(&self, url: &str, _client: &CrawlerClient) -> Result<Vec<MediaResource>> {
        // Try to extract video ID
        let bvid = Self::extract_bvid(url);
        let avid = Self::extract_avid(url);
        
        let bvid = match (bvid, avid) {
            (Some(bv), _) => bv,
            (None, Some(av)) => Self::av_to_bv(av),
            _ => return Err(CrawlerError::InvalidUrl("Invalid Bilibili URL".to_string())),
        };

        // Get video info
        let video_info = self.get_video_info(&bvid).await?;
        
        // Get CID for play URL
        let cid = self.get_cid(&bvid).await?;
        
        // Get play URLs
        let play_data = self.get_play_url(&bvid, cid).await?;
        
        let mut resources = Vec::new();
        
        // Extract download URLs
        if let Some(durls) = play_data.durl {
            for durl in durls {
                let url = durl.url.unwrap_or_default();
                if url.is_empty() {
                    continue;
                }
                
                let order = durl.order.unwrap_or(0);
                let length = durl.length.unwrap_or(0);
                let size = durl.size.unwrap_or(0);
                
                // Determine quality from order
                let quality = match order {
                    1 => MediaQuality::FourK,
                    2 => MediaQuality::FullHD,
                    3 => MediaQuality::High,
                    4 => MediaQuality::Medium,
                    5 => MediaQuality::Low,
                    _ => MediaQuality::Medium,
                };
                
                let mut resource = MediaResource::new(
                    url.clone(),
                    url,
                    quality,
                    VideoFormat::MP4,
                )
                .with_file_size(size)
                .with_duration(length as f64)
                .with_metadata("bvid".to_string(), bvid.clone());
                
                if let Some(title) = &video_info.title {
                    resource = resource.with_metadata("title".to_string(), title.clone());
                }
                
                if let Some(owner) = &video_info.owner {
                    if let Some(name) = &owner.name {
                        resource = resource.with_metadata("author".to_string(), name.clone());
                    }
                }
                
                if let Some(stat) = &video_info.stat {
                    if let Some(views) = stat.view {
                        resource = resource.with_metadata("views".to_string(), views.to_string());
                    }
                }
                
                resources.push(resource);
            }
        }
        
        // If no URLs found, try to get a basic resource with the video page
        if resources.is_empty() {
            resources.push(MediaResource::new(
                format!("bili://{}", bvid),
                url.to_string(),
                MediaQuality::Medium,
                VideoFormat::MP4,
            ).with_metadata(
                "platform".to_string(),
                "bilibili".to_string(),
            ).with_metadata(
                "bvid".to_string(),
                bvid,
            ).with_metadata(
                "note".to_string(),
                "Use Bilibili client or yt-dlp for full extraction".to_string(),
            ));
        }
        
        Ok(resources)
    }
}

#[async_trait]
impl PlatformAdapter for BilibiliAdapter {
    fn can_handle(&self, url: &str) -> bool {
        let url_lower = url.to_lowercase();
        
        // Check for various Bilibili URL patterns
        url_lower.contains("bilibili.com") ||
        url_lower.contains("b23.tv") ||
        url_lower.contains("bilibili") // ForBV号 in text
    }

    async fn extract_media(&self, url: &str, client: &CrawlerClient) -> Result<Vec<MediaResource>> {
        if !self.can_handle(url) {
            return Err(CrawlerError::InvalidUrl("Not a Bilibili URL".to_string()));
        }

        self.extract_media(url, client).await
    }

    fn platform_name(&self) -> &str {
        "Bilibili"
    }

    fn platform(&self) -> Platform {
        Platform::Bilibili
    }
}

impl Default for BilibiliAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bvid() {
        assert_eq!(
            BilibiliAdapter::extract_bvid("https://www.bilibili.com/video/BV1xx411c7XD"),
            Some("BV1XX411C7XD".to_string())
        );
        
        assert_eq!(
            BilibiliAdapter::extract_bvid("BV1aW411t7fP"),
            Some("BV1AW411T7FP".to_string())
        );
    }

    #[test]
    fn test_extract_avid() {
        assert_eq!(
            BilibiliAdapter::extract_avid("https://www.bilibili.com/video/av170001"),
            Some(170001)
        );
    }

    #[test]
    fn test_can_handle() {
        let adapter = BilibiliAdapter::new();
        
        assert!(adapter.can_handle("https://www.bilibili.com/video/BV1xx411c7XD"));
        assert!(adapter.can_handle("https://b23.tv/abc123"));
        assert!(adapter.can_handle("https://www.bilibili.com/video/av170001"));
        assert!(!adapter.can_handle("https://youtube.com/watch?v=abc"));
    }

    #[test]
    fn test_quality_mapping() {
        assert_eq!(BilibiliAdapter::map_quality(127), MediaQuality::FourK);
        assert_eq!(BilibiliAdapter::map_quality(80), MediaQuality::High);
        assert_eq!(BilibiliAdapter::map_quality(32), MediaQuality::Medium);
    }
}