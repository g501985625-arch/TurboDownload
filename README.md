# TurboDownload

A fast download manager with web scraping capabilities, built with Tauri 2.x + React + TypeScript + Rust.

## Features

- **HTTP/HTTPS Downloads**: Multi-threaded downloads with progress tracking
- **Resume Support**: Pause and resume downloads
- **Web Crawler**: Extract downloadable resources from any webpage
- **Resource Detection**: Automatic detection of images, videos, documents, and more
- **Modern UI**: Clean, dark-themed interface built with React and Tailwind CSS

## Tech Stack

### Frontend
- React 18
- TypeScript 5
- Tailwind CSS 3
- Zustand (State Management)
- Lucide React (Icons)

### Backend
- Rust
- Tauri 2.x
- Tokio (Async Runtime)
- Reqwest (HTTP Client)
- Scraper (HTML Parsing)

## Project Structure

```
TurboDownload/
├── src/                    # Frontend React application
│   ├── components/         # React components
│   │   ├── DownloadList/   # Download task list
│   │   ├── DownloadItem/   # Individual download item
│   │   ├── AddDownload/    # Add download modal
│   │   ├── CrawlerPanel/   # Web crawler UI
│   │   └── Settings/       # Settings panel
│   ├── stores/             # Zustand state stores
│   ├── services/           # Frontend services
│   ├── hooks/              # Custom React hooks
│   ├── types/              # TypeScript type definitions
│   └── App.tsx             # Main application component
│
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── models/         # Data models
│   │   ├── services/       # Business logic
│   │   │   ├── http_downloader.rs
│   │   │   ├── download_manager.rs
│   │   │   └── crawler/
│   │   └── main.rs         # Application entry point
│   └── Cargo.toml          # Rust dependencies
│
└── package.json            # Node.js dependencies
```

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.70+
- pnpm or npm

### Installation

1. Clone the repository:
```bash
cd ~/Projects/TurboDownload
```

2. Install frontend dependencies:
```bash
npm install
```

3. Run in development mode:
```bash
npm run tauri:dev
```

### Building for Production

```bash
npm run tauri:build
```

## Core Functionality

### Download Management

- Add downloads via URL
- Multi-threaded downloading (configurable connections)
- Pause, resume, and cancel downloads
- Progress tracking with speed and ETA
- Automatic filename detection

### Web Crawler

- Crawl any webpage to discover resources
- Filter by resource type (images, videos, documents, etc.)
- Batch download selected resources
- Resource type detection from URL patterns

### Settings

- Default download directory
- Concurrent download limits
- Speed limiting
- Desktop notifications

## API Commands

The following Tauri commands are available:

### Download Commands
- `add_download(url, config?)` - Add a new download
- `start_download(task_id)` - Start a download
- `pause_download(task_id)` - Pause a download
- `resume_download(task_id)` - Resume a paused download
- `cancel_download(task_id)` - Cancel a download
- `remove_download(task_id)` - Remove a download
- `get_download(task_id)` - Get download details
- `get_all_downloads()` - Get all downloads
- `get_download_progress(task_id)` - Get download progress

### Crawler Commands
- `crawl_url(url)` - Crawl a URL for resources
- `crawl_url_with_depth(url, depth)` - Crawl with specified depth

### File Commands
- `select_directory()` - Open directory picker
- `get_default_download_dir()` - Get default download directory
- `file_exists(path)` - Check if file exists

## Agent API

TurboDownload provides a REST API for programmatic access to download management. This enables AI agents and external systems to interact with TurboDownload programmatically.

### Base URLs

- **REST API**: `http://localhost:8080/api/v1`
- **WebSocket**: `ws://localhost:8080/ws`

### Authentication

All API requests (except `/health`) require a Bearer token:

```bash
curl -H "Authorization: Bearer <token>" http://localhost:8080/api/v1/downloads
```

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/api/v1/download` | Create download |
| GET | `/api/v1/download/:id` | Get download status |
| POST | `/api/v1/download/:id/pause` | Pause download |
| POST | `/api/v1/download/:id/resume` | Resume download |
| DELETE | `/api/v1/download/:id` | Cancel download |
| GET | `/api/v1/downloads` | List all downloads |

### Quick Start

1. **Check health**:
```bash
curl http://localhost:8080/health
```

2. **Create a download**:
```bash
curl -X POST http://localhost:8080/api/v1/download \
  -H "Authorization: Bearer your_token" \
  -H "Content-Type: application/json" \
  -d '{"url":"https://example.com/file.zip","filename":"file.zip","threads":4}'
```

3. **Get download status**:
```bash
curl http://localhost:8080/api/v1/download/<task_id> \
  -H "Authorization: Bearer your_token"
```

4. **List all downloads**:
```bash
curl http://localhost:8080/api/v1/downloads \
  -H "Authorization: Bearer your_token"
```

### WebSocket Events

Connect to `ws://localhost:8080/ws?token=<token>` to receive real-time updates:

- `Progress` - Download progress updates
- `Completed` - Download completed
- `Failed` - Download failed
- `StatusChanged` - Status changed

### Documentation

For complete API documentation including request/response schemas, error codes, and examples in JavaScript, see [docs/API.md](docs/API.md).

### Testing

Run the API test script:

```bash
./scripts/test_api.sh
```

For Rust integration tests:

```bash
cargo test --package turbo-download --test api_test
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.