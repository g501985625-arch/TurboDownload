// TurboDownload - Library crate
pub mod commands;
pub mod api;
pub mod cli;  // CLI 模块

use log::info;

pub fn run() {
    info!("TurboDownload library loaded");
}