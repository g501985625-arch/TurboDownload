// Commands module
use tokio::sync::Mutex;
use turbo_crawler::CrawlConfig;
use turbo_crawler::Crawler;
use turbo_downloader::Client;
use turbo_downloader::Manager;

pub mod download;
pub mod crawler;
pub mod privacy;
pub mod update;

/// Application state holding the downloader and crawler managers
pub struct AppState {
    pub download_manager: Mutex<Manager>,
    pub crawler: Mutex<Crawler>,
}

impl AppState {
    /// Create new application state
    pub fn new() -> Result<Self, String> {
        use turbo_downloader::http::PrivacyClientConfig;
        
        // Initialize HTTP client for downloader
        let client = Client::new(
            PrivacyClientConfig::default()
        ).map_err(|e| e.to_string())?;

        // Create download manager
        let download_manager = Manager::new(client, 3);

        // Create crawler
        let crawler = Crawler::new(CrawlConfig::default())
            .map_err(|e| e.to_string())?;

        Ok(Self {
            download_manager: Mutex::new(download_manager),
            crawler: Mutex::new(crawler),
        })
    }
}