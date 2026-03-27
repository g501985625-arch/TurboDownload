//! TurboDownload - A fast download manager with web scraping capabilities
//!
//! Built with Tauri 2.x, React, TypeScript, and Rust

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use turbo_download_lib::commands::AppState;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            // Download commands
            turbo_download_lib::commands::add_download,
            turbo_download_lib::commands::start_download,
            turbo_download_lib::commands::pause_download,
            turbo_download_lib::commands::resume_download,
            turbo_download_lib::commands::cancel_download,
            turbo_download_lib::commands::remove_download,
            turbo_download_lib::commands::get_download,
            turbo_download_lib::commands::get_all_downloads,
            turbo_download_lib::commands::get_download_progress,
            // Crawler commands
            turbo_download_lib::commands::crawl_page,
            turbo_download_lib::commands::scan_site,
            turbo_download_lib::commands::extract_download_links,
            turbo_download_lib::commands::get_resource_info,
            // File commands
            turbo_download_lib::commands::get_default_download_dir,
            turbo_download_lib::commands::get_home_dir,
            turbo_download_lib::commands::file_exists,
            turbo_download_lib::commands::get_file_size,
            turbo_download_lib::commands::delete_file,
            turbo_download_lib::commands::create_dir,
            turbo_download_lib::commands::list_dir,
            turbo_download_lib::commands::open_in_file_manager,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}