/// 日志级别配置
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LogMode {
    Full,       // 完整日志
    ErrorOnly,  // 仅错误日志
    None,       // 无日志（隐私模式）
}

impl Default for LogMode {
    fn default() -> Self {
        LogMode::ErrorOnly  // 默认仅错误日志
    }
}

/// 日志配置结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggingConfig {
    pub mode: LogMode,
    pub log_file_path: Option<std::path::PathBuf>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            mode: LogMode::default(),
            log_file_path: None,
        }
    }
}

impl LoggingConfig {
    /// 根据配置初始化日志系统
    pub fn init(&self) {
        match self.mode {
            LogMode::None => {
                // 完全禁用日志
                std::env::set_var("RUST_LOG", "off");
            }
            LogMode::ErrorOnly => {
                std::env::set_var("RUST_LOG", "error");
            }
            LogMode::Full => {
                std::env::set_var("RUST_LOG", "info");
            }
        }
    }
    
    /// 获取日志级别的显示名称
    pub fn mode_display_name(&self) -> &'static str {
        match self.mode {
            LogMode::Full => "完整日志",
            LogMode::ErrorOnly => "仅错误日志",
            LogMode::None => "无日志（隐私模式）",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_mode_default() {
        assert_eq!(LogMode::default(), LogMode::ErrorOnly);
    }
    
    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.mode, LogMode::ErrorOnly);
        assert_eq!(config.log_file_path, None);
    }
    
    #[test]
    fn test_mode_display_name() {
        assert_eq!(LoggingConfig { mode: LogMode::Full, log_file_path: None }.mode_display_name(), "完整日志");
        assert_eq!(LoggingConfig { mode: LogMode::ErrorOnly, log_file_path: None }.mode_display_name(), "仅错误日志");
        assert_eq!(LoggingConfig { mode: LogMode::None, log_file_path: None }.mode_display_name(), "无日志（隐私模式）");
    }
}