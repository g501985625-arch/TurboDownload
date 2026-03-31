//! User-Agent Pool Module
//! 
//! Provides random User-Agent rotation to prevent client fingerprinting

use rand::Rng;

/// Default User-Agent pool - common browsers on different platforms
const USER_AGENTS: &[&str] = &[
    // Windows - Chrome
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36",
    // Windows - Firefox
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0",
    // Windows - Edge
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
    // macOS - Chrome
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    // macOS - Firefox
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0",
    // macOS - Safari
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
    // Linux - Chrome
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    // Linux - Firefox
    "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
    // Android - Chrome
    "Mozilla/5.0 (Linux; Android 14; SM-S918B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 13; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    // iOS - Safari
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Mobile/15E148 Safari/604.1",
    // iPad - Safari
    "Mozilla/5.0 (iPad; CPU OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
];

/// Default fallback User-Agent
const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// User-Agent Pool for random rotation
pub struct UserAgentPool {
    agents: Vec<String>,
}

impl Default for UserAgentPool {
    fn default() -> Self {
        Self::new()
    }
}

impl UserAgentPool {
    /// Create a new User-Agent pool with default agents
    pub fn new() -> Self {
        Self {
            agents: USER_AGENTS.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Create a pool with custom agents
    pub fn with_agents(agents: Vec<String>) -> Self {
        if agents.is_empty() {
            Self::new()
        } else {
            Self { agents }
        }
    }

    /// Get a random User-Agent from the pool
    pub fn random(&self) -> String {
        if self.agents.is_empty() {
            return DEFAULT_USER_AGENT.to_string();
        }
        
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.agents.len());
        self.agents[index].clone()
    }

    /// Get a random User-Agent for a specific platform
    pub fn random_for_platform(&self, platform: Platform) -> String {
        let filtered: Vec<&String> = self.agents
            .iter()
            .filter(|ua| platform.matches(ua))
            .collect();
        
        if filtered.is_empty() {
            return DEFAULT_USER_AGENT.to_string();
        }
        
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..filtered.len());
        filtered[index].clone()
    }

    /// Get the number of agents in the pool
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    /// Check if the pool is empty
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }
}

/// Platform types for User-Agent filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Android,
    iOS,
    Any,
}

impl Platform {
    /// Check if a User-Agent matches this platform
    fn matches(&self, ua: &str) -> bool {
        match self {
            Platform::Windows => ua.contains("Windows NT"),
            Platform::MacOS => ua.contains("Mac OS X"),
            Platform::Linux => ua.contains("Linux") && !ua.contains("Android"),
            Platform::Android => ua.contains("Android"),
            Platform::iOS => ua.contains("iPhone") || ua.contains("iPad"),
            Platform::Any => true,
        }
    }
}

/// Get a random User-Agent (convenience function)
pub fn random_user_agent() -> String {
    UserAgentPool::new().random()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_ua() {
        let pool = UserAgentPool::new();
        let ua = pool.random();
        assert!(!ua.is_empty());
        assert!(ua.starts_with("Mozilla/5.0"));
    }

    #[test]
    fn test_platform_filter() {
        let pool = UserAgentPool::new();
        let windows_ua = pool.random_for_platform(Platform::Windows);
        assert!(windows_ua.contains("Windows NT"));
    }

    #[test]
    fn test_convenience_function() {
        let ua = random_user_agent();
        assert!(!ua.is_empty());
    }
}