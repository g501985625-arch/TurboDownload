use thiserror::Error;

/// Crawler errors
#[derive(Debug, Error)]
pub enum CrawlerError {
    /// Network error
    #[error("Network error: {0}")]
    Network(String),
    
    /// HTTP error
    #[error("HTTP error {0}: {1}")]
    Http(u16, String),
    
    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),
    
    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    /// Timeout
    #[error("Timeout")]
    Timeout,
    
    /// Rate limited
    #[error("Rate limited")]
    RateLimited,
    
    /// Cancelled
    #[error("Cancelled")]
    Cancelled,
    
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, CrawlerError>;

impl From<url::ParseError> for CrawlerError {
    fn from(err: url::ParseError) -> Self {
        CrawlerError::InvalidUrl(err.to_string())
    }
}

impl From<reqwest::Error> for CrawlerError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            CrawlerError::Timeout
        } else {
            CrawlerError::Network(err.to_string())
        }
    }
}