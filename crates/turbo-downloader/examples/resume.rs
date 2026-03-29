//! Resume download example

use std::path::PathBuf;
use turbo_downloader::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize recovery module
    let client = Client::with_defaults()?;
    let recovery = Recovery::new(client);

    // Load previous state (if exists)
    let state_path = PathBuf::from("./downloads/state.json");

    if state_path.exists() {
        let state = ResumeState::load(&state_path).await?;

        if let Some(state) = state {
            println!("Found saved state for task: {}", state.task_id);
            println!("  URL: {}", state.url);
            println!(
                "  Downloaded: {} / {} bytes",
                state.downloaded, state.file_size
            );

            // Try to recover
            match recovery.try_recover(state).await? {
                Some(chunks) => {
                    println!("Resuming download with {} chunks remaining", chunks.len());
                    // Continue download...
                }
                None => {
                    println!("Download already completed!");
                }
            }
        }
    } else {
        println!("No previous download state found");
    }

    // Save current state during download (example)
    let state = ResumeState {
        task_id: "example-task".to_string(),
        url: "https://example.com/file.zip".to_string(),
        file_size: 1_000_000,
        etag: Some("\"abc123\"".to_string()),
        downloaded: 500_000,
        chunks: vec![],
        output_path: PathBuf::from("./downloads/file.zip"),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Save state periodically
    state.save(&state_path).await?;
    println!("Download state saved to {:?}", state_path);

    Ok(())
}
