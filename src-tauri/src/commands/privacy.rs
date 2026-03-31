use turbo_downloader::PrivacyConfig;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_privacy_config(state: State<'_, Mutex<PrivacyConfig>>) -> Result<PrivacyConfig, String> {
    let config = state.lock().unwrap();
    Ok(config.clone())
}

#[tauri::command]
pub fn set_privacy_config(
    new_config: PrivacyConfig,
    state: State<'_, Mutex<PrivacyConfig>>,
) -> Result<(), String> {
    let mut config = state.lock().unwrap();
    *config = new_config.clone();
    
    // 尝试保存配置到文件
    save_privacy_config_to_file(&new_config)?;
    
    Ok(())
}

fn save_privacy_config_to_file(config: &PrivacyConfig) -> Result<(), String> {
    // 确保配置目录存在
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("turbodownload");
    
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    let config_path = config_dir.join("privacy.json");

    let json_content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize privacy config: {}", e))?;

    std::fs::write(config_path, json_content)
        .map_err(|e| format!("Failed to write privacy config to file: {}", e))?;

    Ok(())
}

pub fn load_privacy_config_from_file() -> Result<PrivacyConfig, String> {
    // 尝试从文件加载配置
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("turbodownload");
    
    let config_path = config_dir.join("privacy.json");

    // 如果文件不存在，则返回默认配置
    if !config_path.exists() {
        log::info!("Privacy config file not found, using default config");
        return Ok(PrivacyConfig::default());
    }

    // 尝试读取文件
    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read privacy config file: {}", e))?;

    // 尝试解析配置
    let config = serde_json::from_str::<PrivacyConfig>(&content)
        .map_err(|e| format!("Failed to parse privacy config: {}", e))?;

    log::info!("Successfully loaded privacy config from file");
    Ok(config)
}