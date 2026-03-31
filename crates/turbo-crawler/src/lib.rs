//! Turbo Crawler - Web resource crawler and extractor
//!
//! A Rust library for web crawling, resource extraction, and site scanning.

pub mod error;
pub mod http;
pub mod parser;
pub mod extractor;
pub mod classifier;
pub mod scheduler;
pub mod crawler;
pub mod adapters;

// Re-export main types
pub use error::{CrawlerError, Result};
pub use http::CrawlerClient;
pub use parser::HtmlParser;
pub use extractor::{Resource, ResourceExtractor, ResourceType, Platform, StreamFormat};
pub use classifier::ResourceClassifier;
pub use scheduler::{UrlScheduler, QueuePolicy};
pub use crawler::{Crawler, CrawlConfig, CrawlResult};

// Re-export adapter types
pub use adapters::{
    PlatformAdapter, AdapterRegistry, MediaResource, MediaQuality, VideoFormat,
    YouTubeAdapter, BilibiliAdapter, GenericAdapter,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");