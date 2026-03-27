# Turbo Downloader

High-performance multi-threaded download engine with resume support.

## Features

- ✅ **Multi-threaded Downloads** - Parallel chunk downloads for maximum speed
- ✅ **Resume Support** - Pause and resume downloads automatically
- ✅ **Progress Tracking** - Real-time progress and speed monitoring
- ✅ **Configurable Concurrency** - Control thread count and chunk size
- ✅ **Event System** - Progress events for UI integration
- ✅ **Clean API** - Simple and intuitive Rust API

## Quick Start

```rust
use turbo_downloader::{
    download::DownloadConfig,
    downloader::MultiThreadDownloader,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure download
    let config = DownloadConfig {
        id: "my-download".to_string(),
        url: "https://example.com/file.zip".to_string(),
        output_path: PathBuf::from("./file.zip"),
        threads: 8,
        chunk_size: 1024 * 1024, // 1MB chunks
        resume_support: true,
        ..Default::default()
    };
    
    // Create downloader
    let downloader = MultiThreadDownloader::new(config)?;
    
    // Start download
    let result = downloader.download().await?;
    
    println!("Downloaded {} bytes in {} ms", result.file_size, result.duration_ms);
    
    Ok(())
}
```

## Architecture

```
turbo-downloader/
├── src/
│   ├── chunk/          # Chunk management
│   ├── commands/       # CLI commands
│   ├── download/       # Download configuration
│   ├── downloader/     # Main downloader
│   ├── error/          # Error types
│   ├── event/          # Event system
│   ├── http/           # HTTP client
│   ├── pool/           # Worker pool
│   ├── progress/       # Progress tracking
│   ├── range/          # HTTP Range requests
│   ├── resume/         # Resume support
│   └── storage/        # File storage
```

## Modules

### T3.1 Range Request Module
- `src/range/` - HTTP Range request support
- `RangeClient` - Client for range requests
- `RangeSupport` - Server capability detection

### T3.2 Chunk Management Module
- `src/chunk/` - Chunk management
- `Chunk` - Chunk data structure
- `ChunkManager` - Chunk allocation and tracking

### T3.3 Thread Pool Module
- `src/pool/` - Worker pool
- `WorkerPool` - Concurrent task execution

### T3.4 Chunk Download Worker
- `src/chunk/worker.rs` - Chunk download worker
- `ChunkWorker` - Downloads individual chunks
- Retry mechanism with exponential backoff

### T3.5 Storage Module
- `src/storage/` - File storage
- `ChunkWriter` - Writes chunk data
- `FileMerger` - Merges chunks into final file

### T3.6 Resume Module
- `src/storage/state.rs` - State persistence
- `StateManager` - Saves/loads download state
- `DownloadState` - Serializable state

### T3.7 Event Module
- `src/event/` - Event system
- `DownloadEvent` - Progress events
- `EventEmitter` - Event generation

### T3.8 Progress Module
- `src/progress/` - Progress tracking
- `Tracker` - Progress calculation
- Speed and ETA calculation

### T3.9 Downloader Module
- `src/downloader.rs` - Main downloader
- `MultiThreadDownloader` - Orchestrates downloads

### T3.10 Commands Module
- `src/commands.rs` - CLI commands
- `start_download` - Start new download
- `pause_download` - Pause download
- `resume_download` - Resume download

## Testing

```bash
cargo test -p turbo-downloader
```

## License

MIT
