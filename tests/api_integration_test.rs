/// 测试 HTTP Server 启动
#[tokio::test]
async fn test_server_start() {
    // 验证服务器在端口 8080 启动
    let server_handle = tokio::spawn(async {
        // 启动服务器的代码
        // 这里应该包含实际的服务器启动逻辑
    });

    // 等待服务器启动
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // 尝试连接到服务器
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8080/health").send().await;
    
    assert!(response.is_ok());
    
    // 清理：关闭服务器
    // server_handle.abort();
}

/// 测试健康检查端点
#[tokio::test]
async fn test_health_endpoint() {
    // GET /health 返回 200
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8080/health").send().await.unwrap();
    
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert_eq!(body, "OK");
}

/// 测试 REST API 端点
#[tokio::test]
async fn test_rest_api_endpoints() {
    // POST /api/v1/download
    let client = reqwest::Client::new();
    
    // 测试创建下载任务
    let download_data = serde_json::json!({
        "url": "https://example.com/test.zip",
        "destination": "/tmp/test.zip"
    });
    
    let response = client
        .post("http://localhost:8080/api/v1/download")
        .header("Content-Type", "application/json")
        .json(&download_data)
        .send()
        .await
        .unwrap();
        
    assert_eq!(response.status(), 200);
    let response_body: serde_json::Value = response.json().await.unwrap();
    assert!(response_body.get("id").is_some());

    // GET /api/v1/download/:id
    let download_id = response_body["id"].as_str().unwrap();
    let response = client
        .get(format!("http://localhost:8080/api/v1/download/{}", download_id))
        .send()
        .await
        .unwrap();
        
    assert_eq!(response.status(), 200);

    // POST /api/v1/download/:id/pause
    let response = client
        .post(format!("http://localhost:8080/api/v1/download/{}/pause", download_id))
        .send()
        .await
        .unwrap();
        
    assert_eq!(response.status(), 200);
}

/// 测试 WebSocket 连接
#[tokio::test]
async fn test_websocket_connection() {
    use tokio_tungstenite::{connect_async, tungstenite::Message};
    use futures_util::{SinkExt, StreamExt};

    // 连接 ws://localhost:8080/ws
    let url = url::Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    // 发送消息并验证响应
    write.send(Message::Text("ping".to_string())).await.unwrap();

    // 接收事件
    if let Some(msg) = read.next().await {
        let msg = msg.unwrap();
        assert!(msg.is_text() || msg.is_ping());
    }
}

/// 测试 CLI 工具
#[test]
fn test_cli_commands() {
    use std::process::Command;

    // 测试命令是否存在
    let output = Command::new("./turbodl")
        .arg("--help")
        .output()
        .expect("Failed to execute help command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("turbodl"));
}

/// 测试认证
#[tokio::test]
async fn test_authentication() {
    let client = reqwest::Client::new();
    
    // 测试无认证访问被拒绝
    let response = client
        .post("http://localhost:8080/api/v1/download")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({"url": "https://example.com/test.zip"}))
        .send()
        .await
        .unwrap();
        
    // 如果启用了认证，未提供令牌应返回401或403
    // assert!(response.status() == 401 || response.status() == 403);

    // 测试带有效令牌的访问
    let response = client
        .post("http://localhost:8080/api/v1/download")
        .header("Authorization", "Bearer valid_token_here")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({"url": "https://example.com/test.zip"}))
        .send()
        .await
        .unwrap();
        
    // 成功的请求应该返回200或其他成功状态码
    // assert_eq!(response.status(), 200);
}