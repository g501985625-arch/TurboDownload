// TurboDownload - Main entry point
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use std::sync::Arc;
use tauri::Manager;

mod commands;
mod privacy;
mod api;
use commands::AppState;
use privacy::{load_privacy_config_from_file, logging::LoggingConfig};

fn main() {
    // Load privacy configuration first to get logging settings
    let privacy_config = load_privacy_config_from_file()
        .expect("Failed to load privacy config");
    
    // Initialize logging based on privacy config
    privacy_config.logging.init();
    
    // Initialize logger with the configured level
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    info!("TurboDownload starting...");
    info!("Logging mode: {}", privacy_config.logging.mode_display_name());

    // Create application state with real downloader and crawler
    let app_state = match AppState::new() {
        Ok(state) => Arc::new(state),
        Err(e) => {
            log::error!("Failed to initialize app state: {}", e);
            eprintln!("Error: Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    };

    // 为 API 服务器创建独立的管理器（不使用共享状态）
    use turbo_downloader::Client;
    let client = Client::new(turbo_downloader::http::PrivacyClientConfig::default())
        .expect("Failed to create HTTP client for API server");
    let download_manager = turbo_downloader::Manager::new(client, 3);
    let api_download_manager = Arc::new(tokio::sync::Mutex::new(download_manager));

    // Start HTTP API server in background with shared download manager
    let api_port = 8080u16;
    let api_server = api::AgentServer::with_state(api_port, api_download_manager);
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime for API server");
        rt.block_on(async {
            if let Err(e) = api_server.start().await {
                log::error!("API server error: {}", e);
            }
        });
    });
    info!("HTTP API server started on port {} (sharing download manager)", api_port);

    let privacy_state = std::sync::Mutex::new(privacy_config);

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(app_state)
        .manage(privacy_state)
        .invoke_handler(tauri::generate_handler![
            // Download commands
            commands::download::start_download,
            commands::download::pause_download,
            commands::download::resume_download,
            commands::download::cancel_download,
            commands::download::get_download_status,
            commands::download::list_downloads,
            // Crawler commands
            commands::crawler::crawl_url,
            commands::crawler::crawl_batch,
            commands::crawler::scan_site,
            commands::crawler::scan_url,
            // Privacy commands
            commands::privacy::get_privacy_config,
            commands::privacy::set_privacy_config,
            // Update commands
            commands::update::check_update,
            commands::update::download_update,
            commands::update::install_update,
            commands::update::get_current_version,
        ])
        .setup(|app| {
            info!("TurboDownload setup complete");
            
            // Get main window
            if let Some(window) = app.get_webview_window("main") {
                info!("Main window created successfully");
                let _ = window.set_title(&format!("TurboDownload v{}", env!("CARGO_PKG_VERSION")));
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}