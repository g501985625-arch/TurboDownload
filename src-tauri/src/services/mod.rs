//! Services Module
//! 
//! Core business logic services

pub mod http_downloader;
pub mod download_manager;
pub mod crawler;

pub use http_downloader::HttpDownloader;
pub use download_manager::DownloadManager;
pub use crawler::CrawlerService;
pub use crawler::HtmlParser;
pub use crawler::UrlExtractor;