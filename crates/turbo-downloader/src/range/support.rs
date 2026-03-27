use serde::{Deserialize, Serialize};

/// Range 请求支持信息
///
/// 通过 HEAD 请求获取，用于检测服务器是否支持 Range 请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeSupport {
    /// 请求是否成功（HTTP 2xx）
    pub supported: bool,
    /// 文件总大小（Content-Length）
    pub content_length: Option<u64>,
    /// 服务器支持的 Range 单位（如 "bytes"）
    pub accept_ranges: Option<String>,
    /// 实体标签（ETag）
    pub etag: Option<String>,
    /// 内容类型（MIME 类型）
    pub content_type: Option<String>,
    /// 最后修改时间
    pub last_modified: Option<String>,
}

impl RangeSupport {
    /// 检查服务器是否支持字节 Range 请求
    ///
    /// 当服务器响应 Accept-Ranges: bytes 时，表示支持断点续传
    pub fn is_supported(&self) -> bool {
        self.supported
            && self
                .accept_ranges
                .as_deref()
                .map(|v| v.eq_ignore_ascii_case("bytes"))
                .unwrap_or(false)
    }

    /// 检查是否支持 Range 请求（不限定单位）
    ///
    /// 某些服务器返回 Accept-Ranges: none 表示不支持 Range
    pub fn accepts_ranges(&self) -> bool {
        if let Some(accept_ranges) = &self.accept_ranges {
            !accept_ranges.eq_ignore_ascii_case("none")
        } else {
            false
        }
    }

    /// 获取文件大小，如果未知返回 None
    pub fn file_size(&self) -> Option<u64> {
        self.content_length
    }

    /// 检查是否有 ETag
    pub fn has_etag(&self) -> bool {
        self.etag.is_some()
    }

    /// 检查是否有最后修改时间
    pub fn has_last_modified(&self) -> bool {
        self.last_modified.is_some()
    }

    /// 创建不支持 Range 的响应
    pub fn unsupported() -> Self {
        Self {
            supported: false,
            content_length: None,
            accept_ranges: None,
            etag: None,
            content_type: None,
            last_modified: None,
        }
    }

    /// 创建支持 Range 的响应
    pub fn supported_with_size(size: u64) -> Self {
        Self {
            supported: true,
            content_length: Some(size),
            accept_ranges: Some("bytes".to_string()),
            etag: None,
            content_type: None,
            last_modified: None,
        }
    }
}

impl Default for RangeSupport {
    fn default() -> Self {
        Self::unsupported()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_supported_with_bytes() {
        let support = RangeSupport {
            supported: true,
            content_length: Some(1000),
            accept_ranges: Some("bytes".to_string()),
            etag: None,
            content_type: None,
            last_modified: None,
        };
        assert!(support.is_supported());
    }

    #[test]
    fn test_is_supported_not_bytes() {
        let support = RangeSupport {
            supported: true,
            content_length: Some(1000),
            accept_ranges: Some("none".to_string()),
            etag: None,
            content_type: None,
            last_modified: None,
        };
        assert!(!support.is_supported());
    }

    #[test]
    fn test_is_supported_no_accept_ranges() {
        let support = RangeSupport {
            supported: true,
            content_length: Some(1000),
            accept_ranges: None,
            etag: None,
            content_type: None,
            last_modified: None,
        };
        assert!(!support.is_supported());
    }

    #[test]
    fn test_accepts_ranges_none() {
        let support = RangeSupport {
            supported: true,
            content_length: Some(1000),
            accept_ranges: Some("none".to_string()),
            etag: None,
            content_type: None,
            last_modified: None,
        };
        assert!(!support.accepts_ranges());
    }

    #[test]
    fn test_file_size() {
        let support = RangeSupport::supported_with_size(1024);
        assert_eq!(support.file_size(), Some(1024));
    }

    #[test]
    fn test_supported_with_size() {
        let support = RangeSupport::supported_with_size(5000);
        assert!(support.is_supported());
        assert_eq!(support.content_length, Some(5000));
    }
}