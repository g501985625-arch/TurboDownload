//! TurboDownload CLI 入口
//! 命令行工具，用于控制下载任务

use clap::Parser;
use turbo_download_lib::cli::{Cli, ApiClient, execute};

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::init();

    // 解析命令行参数
    let cli = Cli::parse();

    // 创建 API 客户端（默认连接本地 8080 端口）
    let api_client = ApiClient::new(8080);

    // 执行命令
    match execute(cli, &api_client).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("❌ 错误: {}", e);
            std::process::exit(1);
        }
    }
}