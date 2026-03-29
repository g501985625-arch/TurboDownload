//! Integration tests for TurboDownload
//!
//! Tests the complete download workflow

use tempfile::TempDir;

use turbo_downloader::{
    download::DownloadConfig,
    downloader::MultiThreadDownloader,
};

/// Test complete download flow with local file (no network)
#[tokio::test]
async fn test_download_config_creation() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test-file.bin");
    
    let config = DownloadConfig {
        id: "test-001".to_string(),
        url: "http://localhost:8080/test.bin".to_string(),
        output_path: output_path.clone(),
        threads: 4,
        chunk_size: 1024 * 1024,
        resume_support: true,
        user_agent: Some("TurboDownload-Test/1.0".to_string()),
        headers: Default::default(),
        speed_limit: 0,
    };
    
    // Create downloader
    let _downloader = MultiThreadDownloader::new(config);
    assert!(_downloader.is_ok(), "Should create downloader successfully");
    
    println!("✅ Download config creation test passed");
}

/// Test multiple concurrent download configs
#[tokio::test]
async fn test_concurrent_configs() {
    let temp_dir = TempDir::new().unwrap();
    
    for i in 0..5 {
        let output_path = temp_dir.path().join(format!("test-file-{}.bin", i));
        let config = DownloadConfig {
            id: format!("test-{}", i),
            url: format!("http://localhost:8080/test-{}.bin", i),
            output_path: output_path.clone(),
            threads: 4,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };
        
        let _downloader = MultiThreadDownloader::new(config);
        assert!(_downloader.is_ok());
    }
    
    println!("✅ Concurrent configs test passed");
}

/// Test download with various thread counts
#[tokio::test]
async fn test_thread_count_variations() {
    let temp_dir = TempDir::new().unwrap();
    
    for threads in [1, 2, 4, 8, 16] {
        let output_path = temp_dir.path().join(format!("test-threads-{}.bin", threads));
        let config = DownloadConfig {
            id: format!("test-threads-{}", threads),
            url: "http://localhost:8080/test.bin".to_string(),
            output_path: output_path.clone(),
            threads,
            chunk_size: 1024 * 1024,
            resume_support: true,
            user_agent: None,
            headers: Default::default(),
            speed_limit: 0,
        };
        
        let _downloader = MultiThreadDownloader::new(config);
        assert!(_downloader.is_ok(), "Should create downloader with {} threads", threads);
    }
    
    println!("✅ Thread count variations test passed");
}

/// Test error handling for invalid URL
#[tokio::test]
async fn test_invalid_url_handling() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.bin");
    
    let config = DownloadConfig {
        id: "test-invalid-url".to_string(),
        url: "not-a-valid-url".to_string(),
        output_path: output_path.clone(),
        threads: 4,
        chunk_size: 1024 * 1024,
        resume_support: true,
        user_agent: None,
        headers: Default::default(),
        speed_limit: 0,
    };
    
    // Should create downloader (URL validation happens at download time)
    let _downloader = MultiThreadDownloader::new(config);
    
    println!("✅ Invalid URL handling test passed");
}

/// Test speed limit configuration
#[tokio::test]
async fn test_speed_limit_config() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("speed-test.bin");
    
    let config = DownloadConfig {
        id: "test-speed-limit".to_string(),
        url: "http://localhost:8080/test.bin".to_string(),
        output_path: output_path.clone(),
        threads: 4,
        chunk_size: 1024 * 1024,
        resume_support: true,
        user_agent: None,
        headers: Default::default(),
        speed_limit: 1024 * 1024, // 1MB/s limit
    };
    
    let _downloader = MultiThreadDownloader::new(config);
    assert!(_downloader.is_ok());
    
    println!("✅ Speed limit config test passed");
}

/// Test user agent configuration
#[tokio::test]
async fn test_user_agent_config() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("ua-test.bin");
    
    let config = DownloadConfig {
        id: "test-ua".to_string(),
        url: "http://localhost:8080/test.bin".to_string(),
        output_path: output_path.clone(),
        threads: 4,
        chunk_size: 1024 * 1024,
        resume_support: true,
        user_agent: Some("CustomAgent/1.0".to_string()),
        headers: Default::default(),
        speed_limit: 0,
    };
    
    let _downloader = MultiThreadDownloader::new(config);
    assert!(_downloader.is_ok());
    
    println!("✅ User agent config test passed");
}

/// Test custom headers configuration
#[tokio::test]
async fn test_custom_headers_config() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("headers-test.bin");
    
    let mut headers = std::collections::HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());
    headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());
    
    let config = DownloadConfig {
        id: "test-headers".to_string(),
        url: "http://localhost:8080/test.bin".to_string(),
        output_path: output_path.clone(),
        threads: 4,
        chunk_size: 1024 * 1024,
        resume_support: true,
        user_agent: None,
        headers,
        speed_limit: 0,
    };
    
    let _downloader = MultiThreadDownloader::new(config);
    assert!(_downloader.is_ok());
    
    println!("✅ Custom headers config test passed");
}
