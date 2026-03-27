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

// Re-export main types
pub use error::{CrawlerError, Result};
pub use http::CrawlerClient;
pub use parser::HtmlParser;
pub use extractor::{Resource, ResourceExtractor, ResourceType};
pub use classifier::ResourceClassifier;
pub use scheduler::{UrlScheduler, QueuePolicy};
pub use crawler::{Crawler, CrawlConfig, CrawlResult};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");