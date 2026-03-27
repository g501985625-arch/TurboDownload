//! Test utilities for turbo-downloader

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header};

/// Start a mock HTTP server
pub async fn start_mock_server() -> MockServer {
    MockServer::start().await
}

/// Create a mock file response
pub fn mock_file_response(server: &MockServer, path: &str, size: u64) {
    let body = vec![0u8; size as usize];
    
    Mock::given(method("GET"))
        .and(path(path))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(body))
        .mount(server);
}

/// Create a mock response with range support
pub fn mock_range_response(server: &MockServer, path: &str, size: u64) {
    let body = vec![0u8; size as usize];
    
    // First response for HEAD request
    Mock::given(method("HEAD"))
        .and(path(path))
        .respond_with(ResponseTemplate::new(200)
            .insert_header("content-length", size.to_string())
            .insert_header("accept-ranges", "bytes")
            .insert_header("etag", "\"test-etag\""))
        .mount(server);
    
    // Response for range requests
    Mock::given(method("GET"))
        .and(path(path))
        .and(header("range", wiremock::matchers::any()))
        .respond_with(ResponseTemplate::new(206)
            .set_body_bytes(body)
            .insert_header("content-length", size.to_string())
            .insert_header("accept-ranges", "bytes"))
        .mount(server);
}

/// Mock error response
pub fn mock_error_response(server: &MockServer, path: &str, status: u16) {
    Mock::given(method("GET"))
        .and(path(path))
        .respond_with(ResponseTemplate::new(status))
        .mount(server);
}

/// Mock redirect response
pub fn mock_redirect_response(server: &MockServer, from: &str, to: &str) {
    Mock::given(method("GET"))
        .and(path(from))
        .respond_with(ResponseTemplate::new(302)
            .insert_header("location", to))
        .mount(server);
}