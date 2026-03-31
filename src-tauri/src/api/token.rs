use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

/// Token 加密存储密钥 (32 字节)
fn get_encryption_key() -> Result<[u8; 32], String> {
    // 使用机器特定信息生成密钥
    let home = dirs::home_dir().ok_or("无法获取 home 目录")?;
    let key_path = home.join(".config/turbodownload/.key");

    if key_path.exists() {
        let key_data = fs::read(&key_path).map_err(|e| e.to_string())?;
        let decoded = BASE64.decode(&key_data).map_err(|e| e.to_string())?;
        let mut key = [0u8; 32];
        if decoded.len() != 32 {
            return Err("密钥长度无效".to_string());
        }
        key.copy_from_slice(&decoded);
        Ok(key)
    } else {
        // 生成新密钥
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        // 保存密钥 (base64 编码)
        let encoded = BASE64.encode(key);
        if let Some(parent) = key_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&key_path, encoded).map_err(|e| e.to_string())?;

        // 设置密钥文件安全权限 (仅在 Unix 系统上)
        set_secure_permissions(&key_path).map_err(|e| format!("设置密钥文件权限失败: {}", e))?;

        Ok(key)
    }
}

/// 生成随机 nonce (12 字节)
fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// 加密 Token
pub fn encrypt_token(token: &str, key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, token.as_bytes())
        .map_err(|e| e.to_string())?;

    // 将 nonce + ciphertext 组合
    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);
    Ok(result)
}

/// 解密 Token
pub fn decrypt_token(encrypted: &[u8], key: &[u8; 32]) -> Result<String, String> {
    if encrypted.len() < 12 {
        return Err("加密数据太短".to_string());
    }

    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(&encrypted[..12]);
    let ciphertext = &encrypted[12..];

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| e.to_string())?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

/// 保存 token 到文件 (加密存储)
pub fn save_token(token: &str) -> Result<(), String> {
    let path = token_file_path()?;
    let key = get_encryption_key()?;
    let encrypted = encrypt_token(token, &key)?;

    // 写入加密后的数据 (base64 编码)
    let encoded = BASE64.encode(encrypted);
    fs::write(&path, encoded).map_err(|e| e.to_string())?;

    // 设置文件安全权限 (仅在 Unix 系统上)
    set_secure_permissions(&path).map_err(|e| format!("设置token文件权限失败: {}", e))?;

    Ok(())
}

/// 从文件读取 token (解密)
pub fn load_token() -> Result<String, String> {
    let path = token_file_path()?;
    let encoded = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let encrypted = BASE64.decode(&encoded).map_err(|e| e.to_string())?;
    let key = get_encryption_key()?;
    decrypt_token(&encrypted, &key)
}

fn token_file_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("无法获取 home 目录")?;
    let path = home.join(".config/turbodownload/token");

    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    Ok(path)
}

// 跨平台设置文件安全权限的辅助函数
fn set_secure_permissions(path: &std::path::Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)
            .map_err(|e| format!("获取文件元数据失败: {}", e))?
            .permissions();
        perms.set_mode(0o600); // 只有所有者可读写
        fs::set_permissions(path, perms).map_err(|e| format!("设置Unix文件权限失败: {}", e))?;
    }
    
    #[cfg(windows)]
    {
        let mut perms = fs::metadata(path)
            .map_err(|e| format!("获取文件元数据失败: {}", e))?
            .permissions();
        perms.set_readonly(true); // 在 Windows 上设置为只读
        fs::set_permissions(path, perms).map_err(|e| format!("设置Windows文件权限失败: {}", e))?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32]; // 测试用密钥
        let token = "test-token-12345";

        let encrypted = encrypt_token(token, &key).unwrap();
        let decrypted = decrypt_token(&encrypted, &key).unwrap();

        assert_eq!(token, decrypted);
    }
}
