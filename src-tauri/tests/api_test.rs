//! TurboDownload API Tests
//!
//! These tests require the TurboDownload server to be running on http://localhost:8080
//!
//! Run with: cargo test --package turbo-download --test api_test

use reqwest::Client;
use serde_json::json;

const BASE_URL: &str = "http://localhost:8080";
const API_URL: &str = "http://localhost:8080/api/v1";
const TEST_TOKEN: &str = "test_token_12345";

fn auth_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", TEST_TOKEN).parse().unwrap(),
    );
    headers
}

#[tokio::test]
async fn test_health_check() {
    let client = Client::new();
    let response = client
        .get(format!("{}/health", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200, "Health check should return 200");

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert_eq!(body.get("status").unwrap(), "ok", "Status should be ok");
}

#[tokio::test]
async fn test_health_check_response_format() {
    let client = Client::new();
    let response = client
        .get(format!("{}/health", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    // Verify required fields exist
    assert!(body.get("status").is_some(), "Should have status field");
    assert!(body.get("version").is_some(), "Should have version field");
}

#[tokio::test]
async fn test_create_download() {
    let client = Client::new();
    let response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "url": "https://example.com/test.zip",
            "filename": "test.zip",
            "threads": 4
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200, "Create download should return 200");

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert!(body.get("task_id").is_some(), "Should have task_id");
    assert_eq!(body.get("status").unwrap(), "pending", "Initial status should be pending");
}

#[tokio::test]
async fn test_create_download_with_options() {
    let client = Client::new();
    let response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "url": "https://example.com/file.zip",
            "filename": "custom.zip",
            "threads": 8,
            "options": {
                "timeout": 600,
                "retry": 5,
                "proxy": ""
            }
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert_eq!(body.get("filename").unwrap(), "custom.zip");
    assert_eq!(body.get("threads").unwrap(), 8);
}

#[tokio::test]
async fn test_create_download_invalid_url() {
    let client = Client::new();
    let response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "url": "not-a-valid-url",
            "filename": "test.zip",
            "threads": 4
        }))
        .send()
        .await
        .expect("Failed to send request");

    // Should return 400 Bad Request
    assert_eq!(response.status(), 400, "Invalid URL should return 400");
}

#[tokio::test]
async fn test_create_download_missing_url() {
    let client = Client::new();
    let response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "filename": "test.zip",
            "threads": 4
        }))
        .send()
        .await
        .expect("Failed to send request");

    // Should return 400 Bad Request
    assert_eq!(response.status(), 400, "Missing URL should return 400");
}

#[tokio::test]
async fn test_get_download_status() {
    // First create a download
    let client = Client::new();
    
    let create_response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "url": "https://example.com/status-test.zip",
            "filename": "status-test.zip",
            "threads": 4
        }))
        .send()
        .await
        .expect("Failed to create download");

    let create_body: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse create response");

    let task_id = create_body.get("task_id").unwrap().as_str().unwrap();

    // Now get the status
    let status_response = client
        .get(format!("{}/download/{}", API_URL, task_id))
        .headers(auth_headers())
        .send()
        .await
        .expect("Failed to send status request");

    assert_eq!(status_response.status(), 200);

    let status_body: serde_json::Value = status_response
        .json()
        .await
        .expect("Failed to parse status response");

    assert_eq!(status_body.get("task_id").unwrap(), task_id);
    assert!(status_body.get("progress").is_some() || status_body.get("status").is_some());
}

#[tokio::test]
async fn test_get_download_status_not_found() {
    let client = Client::new();
    let fake_id = "550e8400-e29b-41d4-a716-446655440000";
    
    let response = client
        .get(format!("{}/download/{}", API_URL, fake_id))
        .headers(auth_headers())
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404, "Non-existent task should return 404");
}

#[tokio::test]
async fn test_list_downloads() {
    let client = Client::new();
    let response = client
        .get(format!("{}/downloads", API_URL))
        .headers(auth_headers())
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    assert!(body.get("tasks").is_some(), "Should have tasks array");
    assert!(body.get("total").is_some(), "Should have total count");
}

#[tokio::test]
async fn test_list_downloads_with_filter() {
    let client = Client::new();
    let response = client
        .get(format!("{}/downloads?status=downloading&limit=10", API_URL))
        .headers(auth_headers())
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    // Verify filtered results
    if let Some(tasks) = body.get("tasks").as_array() {
        for task in tasks {
            assert_eq!(task.get("status").unwrap(), "downloading");
        }
    }
}

#[tokio::test]
async fn test_pause_download() {
    // First create a download
    let client = Client::new();
    
    let create_response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "url": "https://example.com/pause-test.zip",
            "filename": "pause-test.zip",
            "threads": 4
        }))
        .send()
        .await
        .expect("Failed to create download");

    let create_body: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse create response");

    let task_id = create_body.get("task_id").unwrap().as_str().unwrap();

    // Try to pause
    let pause_response = client
        .post(format!("{}/download/{}/pause", API_URL, task_id))
        .headers(auth_headers())
        .send()
        .await
        .expect("Failed to send pause request");

    // Should return 200 or 400 depending on implementation
    assert!(pause_response.status() == 200 || pause_response.status() == 400);
}

#[tokio::test]
async fn test_resume_download() {
    // First create a download
    let client = Client::new();
    
    let create_response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "url": "https://example.com/resume-test.zip",
            "filename": "resume-test.zip",
            "threads": 4
        }))
        .send()
        .await
        .expect("Failed to create download");

    let create_body: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse create response");

    let task_id = create_body.get("task_id").unwrap().as_str().unwrap();

    // Try to resume
    let resume_response = client
        .post(format!("{}/download/{}/resume", API_URL, task_id))
        .headers(auth_headers())
        .send()
        .await
        .expect("Failed to send resume request");

    // Should return 200 or 400 depending on implementation
    assert!(resume_response.status() == 200 || resume_response.status() == 400);
}

#[tokio::test]
async fn test_cancel_download() {
    // First create a download
    let client = Client::new();
    
    let create_response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "url": "https://example.com/cancel-test.zip",
            "filename": "cancel-test.zip",
            "threads": 4
        }))
        .send()
        .await
        .expect("Failed to create download");

    let create_body: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse create response");

    let task_id = create_body.get("task_id").unwrap().as_str().unwrap();

    // Cancel the download
    let cancel_response = client
        .delete(format!("{}/download/{}", API_URL, task_id))
        .headers(auth_headers())
        .send()
        .await
        .expect("Failed to send cancel request");

    assert_eq!(cancel_response.status(), 200);

    let cancel_body: serde_json::Value = cancel_response
        .json()
        .await
        .expect("Failed to parse cancel response");

    assert_eq!(cancel_body.get("status").unwrap(), "cancelled");
}

#[tokio::test]
async fn test_cancel_download_not_found() {
    let client = Client::new();
    let fake_id = "550e8400-e29b-41d4-a716-446655440000";
    
    let response = client
        .delete(format!("{}/download/{}", API_URL, fake_id))
        .headers(auth_headers())
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404, "Non-existent task should return 404");
}

#[tokio::test]
async fn test_auth_required() {
    let client = Client::new();
    
    // Try to create download without auth
    let response = client
        .post(format!("{}/download", API_URL))
        .json(&json!({
            "url": "https://example.com/test.zip",
            "filename": "test.zip"
        }))
        .send()
        .await
        .expect("Failed to send request");

    // Should return 401 Unauthorized
    assert_eq!(response.status(), 401, "Should require authentication");
}

#[tokio::test]
async fn test_invalid_auth_token() {
    let client = Client::new();
    
    let mut invalid_headers = reqwest::header::HeaderMap::new();
    invalid_headers.insert(
        reqwest::header::AUTHORIZATION,
        "Bearer invalid_token".parse().unwrap(),
    );
    
    let response = client
        .get(format!("{}/downloads", API_URL))
        .headers(invalid_headers)
        .send()
        .await
        .expect("Failed to send request");

    // Should return 401 Unauthorized
    assert_eq!(response.status(), 401, "Invalid token should return 401");
}

#[tokio::test]
async fn test_concurrent_downloads() {
    let client = Client::new();
    
    // Create multiple downloads concurrently
    let futures = (0..5).map(|i| {
        client
            .post(format!("{}/download", API_URL))
            .headers(auth_headers())
            .json(&json!({
                "url": format!("https://example.com/concurrent{}.zip", i),
                "filename": format!("concurrent{}.zip", i),
                "threads": 2
            }))
            .send()
    });

    let results = futures::future::join_all(futures).await;
    
    for result in results {
        let response = result.expect("Failed to send request");
        assert_eq!(response.status(), 200);
    }
}

#[tokio::test]
async fn test_threads_validation() {
    let client = Client::new();
    
    // Test with invalid thread count (too high)
    let response = client
        .post(format!("{}/download", API_URL))
        .headers(auth_headers())
        .json(&json!({
            "url": "https://example.com/test.zip",
            "filename": "test.zip",
            "threads": 100  // Too high
        }))
        .send()
        .await
        .expect("Failed to send request");

    // Should either accept with capped threads or return 400
    assert!(response.status() == 200 || response.status() == 400);
}