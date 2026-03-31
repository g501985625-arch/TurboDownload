use std::path::Path;
use tempfile::TempDir;

/// 启动测试服务器
pub async fn start_test_server() -> TestServer {
    // 启动 TurboDownload 应用
    todo!("启动测试服务器的实现")
}

/// 创建临时下载目录
pub fn create_temp_download_dir() -> TempDir {
    // 创建临时目录
    tempfile::tempdir().expect("无法创建临时目录")
}

/// 验证文件下载完成
pub fn verify_download_complete(path: &Path, expected_size: u64) {
    // 验证文件存在且大小正确
    assert!(path.exists(), "下载文件应该存在");
    let actual_size = std::fs::metadata(path)
        .expect("无法获取文件元数据")
        .len();
    assert_eq!(actual_size, expected_size, "文件大小应该匹配预期");
}

// 为上面的函数定义所需类型
pub struct TestServer;