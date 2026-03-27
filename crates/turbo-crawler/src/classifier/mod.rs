//! Resource classifier module

use crate::extractor::Resource;

pub struct ResourceClassifier {
    allowed_extensions: Vec<String>,
    blocked_extensions: Vec<String>,
    allowed_domains: Vec<String>,
    blocked_domains: Vec<String>,
}

impl ResourceClassifier {
    pub fn new() -> Self {
        Self {
            allowed_extensions: vec![],
            blocked_extensions: vec![],
            allowed_domains: vec![],
            blocked_domains: vec![],
        }
    }

    pub fn with_allowed_extensions(mut self, extensions: Vec<String>) -> Self {
        self.allowed_extensions = extensions;
        self
    }

    pub fn with_blocked_extensions(mut self, extensions: Vec<String>) -> Self {
        self.blocked_extensions = extensions;
        self
    }

    pub fn with_allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.allowed_domains = domains;
        self
    }

    pub fn with_blocked_domains(mut self, domains: Vec<String>) -> Self {
        self.blocked_domains = domains;
        self
    }

    pub fn should_include(&self, resource: &Resource) -> bool {
        if let Ok(url) = url::Url::parse(&resource.url) {
            if let Some(host) = url.host_str() {
                for blocked in &self.blocked_domains {
                    if host.contains(blocked) {
                        return false;
                    }
                }

                if !self.allowed_domains.is_empty() {
                    let mut is_allowed = false;
                    for domain in &self.allowed_domains {
                        if host.contains(domain) {
                            is_allowed = true;
                            break;
                        }
                    }
                    if !is_allowed {
                        return false;
                    }
                }
            }
        }

        for blocked in &self.blocked_extensions {
            if resource.url.to_lowercase().ends_with(&format!(".{}", blocked)) {
                return false;
            }
        }

        if !self.allowed_extensions.is_empty() {
            let url_lower = resource.url.to_lowercase();
            let mut is_allowed = false;
            for ext in &self.allowed_extensions {
                if url_lower.ends_with(&format!(".{}", ext)) {
                    is_allowed = true;
                    break;
                }
            }
            if !is_allowed {
                return false;
            }
        }

        true
    }

    pub fn filter(&self, resources: Vec<Resource>) -> Vec<Resource> {
        resources.into_iter()
            .filter(|r| self.should_include(r))
            .collect()
    }
}

impl Default for ResourceClassifier {
    fn default() -> Self {
        Self::new()
    }
}