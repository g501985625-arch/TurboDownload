//! TurboDownload CLI Module
//! 命令行工具模块，提供下载任务管理功能

use clap::{Parser, Subcommand};

pub mod api_client;

pub use api_client::ApiClient;

/// CLI 主结构
#[derive(Parser)]
#[command(name = "turbodl")]
#[command(about = "TurboDownload CLI - 控制下载任务", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// CLI 子命令
#[derive(Subcommand)]
pub enum Commands {
    /// 开始下载
    Download {
        /// 下载 URL
        url: String,
        /// 保存文件名
        #[arg(short, long)]
        output: Option<String>,
        /// 线程数
        #[arg(short, long, default_value = "4")]
        threads: u32,
    },
    /// 暂停下载
    Pause {
        /// 任务 ID
        task_id: String,
    },
    /// 恢复下载
    Resume {
        /// 任务 ID
        task_id: String,
    },
    /// 取消下载
    Cancel {
        /// 任务 ID
        task_id: String,
    },
    /// 列出所有任务
    List,
    /// 获取任务状态
    Status {
        /// 任务 ID
        task_id: String,
    },
}

/// 格式化字节数
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 执行 CLI 命令
pub async fn execute(cli: Cli, api_client: &ApiClient) -> Result<(), String> {
    match cli.command {
        Commands::Download { url, output, threads } => {
            let task_id = api_client.start_download(&url, output.as_deref(), threads).await?;
            println!("✅ 下载任务已创建: {}", task_id);
        }
        Commands::Pause { task_id } => {
            api_client.pause_download(&task_id).await?;
            println!("⏸️  任务已暂停: {}", task_id);
        }
        Commands::Resume { task_id } => {
            api_client.resume_download(&task_id).await?;
            println!("▶️  任务已恢复: {}", task_id);
        }
        Commands::Cancel { task_id } => {
            api_client.cancel_download(&task_id).await?;
            println!("🛑 任务已取消: {}", task_id);
        }
        Commands::List => {
            let tasks = api_client.list_downloads().await?;
            println!("\n{:<20} {:<30} {:<10}", "任务ID", "文件名", "状态");
            println!("{:-<20} {:-<30} {:-<10}", "", "", "");
            for task in tasks {
                println!(
                    "{:<20} {:<30} {:<10}",
                    task.id, task.filename, task.status
                );
            }
            println!();
        }
        Commands::Status { task_id } => {
            let status = api_client.get_download_status(&task_id).await?;
            println!("\n📋 任务: {}", task_id);
            println!("   进度: {:.1}%", status.progress * 100.0);
            println!("   速度: {}/s", format_bytes(status.speed));
            println!("   已下载: {}", format_bytes(status.downloaded));
            println!("   总大小: {}", format_bytes(status.total));
            println!();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }
}