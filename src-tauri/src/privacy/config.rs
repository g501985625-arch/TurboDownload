// 隐私配置结构
// 这个文件现在只是一个重新导出，实际定义在 turbo-downloader 库中
pub use turbo_downloader::PrivacyConfig;

/// 验证 DNS 服务器地址格式 (IP:Port)
/// 支持 IPv4 和 IPv6 地址
/// 例如: 8.8.8.8:53, [::1]:53, 1.1.1.1:853
pub fn validate_dns_server(address: &str) -> Result<(), String> {
    // 尝试解析 IPv6 格式 [::1]:53
    if address.starts_with('[') {
        if let Some(bracket_end) = address.find(']') {
            let ip_part = &address[1..bracket_end];
            let port_part = &address[bracket_end + 1..];
            
            // 验证 IP 部分是有效 IPv6
            if ip_part.parse::<std::net::Ipv6Addr>().is_err() {
                return Err(format!("Invalid IPv6 address: {}", ip_part));
            }
            
            // 验证端口部分
            if !port_part.starts_with(':') || port_part.len() < 2 {
                return Err("Invalid port format for IPv6 DNS server".to_string());
            }
            
            let port_str = &port_part[1..];
            let port: u16 = port_str.parse()
                .map_err(|_| format!("Invalid port number: {}", port_str))?;
            
            if port == 0 {
                return Err("Port number cannot be 0".to_string());
            }
            
            return Ok(());
        }
        return Err("Invalid IPv6 format: missing closing bracket".to_string());
    }
    
    // 尝试解析 IPv4 格式 8.8.8.8:53
    if let Some(colon_pos) = address.rfind(':') {
        let ip_part = &address[..colon_pos];
        let port_part = &address[colon_pos + 1..];
        
        // 验证 IP 部分是有效 IPv4
        if ip_part.parse::<std::net::Ipv4Addr>().is_err() {
            return Err(format!("Invalid IPv4 address: {}", ip_part));
        }
        
        // 验证端口部分
        let port: u16 = port_part.parse()
            .map_err(|_| format!("Invalid port number: {}", port_part))?;
        
        if port == 0 {
            return Err("Port number cannot be 0".to_string());
        }
        
        return Ok(());
    }
    
    Err("DNS server address must be in format IP:Port (e.g., 8.8.8.8:53)".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dns_server_ipv4() {
        assert!(validate_dns_server("8.8.8.8:53").is_ok());
        assert!(validate_dns_server("1.1.1.1:853").is_ok());
        assert!(validate_dns_server("0.0.0.0:0").is_err()); // 端口为0无效
        assert!(validate_dns_server("256.1.1.1:53").is_err()); // 无效IP
        assert!(validate_dns_server("8.8.8.8:").is_err()); // 无效端口
    }

    #[test]
    fn test_validate_dns_server_ipv6() {
        assert!(validate_dns_server("[::1]:53").is_ok());
        assert!(validate_dns_server("[::ffff:8.8.8.8]:53").is_ok());
        assert!(validate_dns_server("[2001:4860:4860::8888]:53").is_ok());
    }

    #[test]
    fn test_validate_dns_server_invalid() {
        assert!(validate_dns_server("8.8.8.8").is_err()); // 缺少端口
        assert!(validate_dns_server(":53").is_err()); // 缺少IP
        assert!(validate_dns_server("").is_err()); // 空字符串
    }
}