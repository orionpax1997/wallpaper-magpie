pub use crate::models::AppConfig;

pub struct ConfigManager;

impl ConfigManager {
    pub fn load() -> anyhow::Result<AppConfig> {
        AppConfig::load()
    }

    pub fn save(config: &AppConfig) -> anyhow::Result<()> {
        config.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert!(config.unsplash_api_key.is_none());
        assert!(config.pexels_api_key.is_none());
        assert!(config.wallhaven_api_key.is_none());
    }

    #[test]
    fn test_has_api_key() {
        let mut config = AppConfig::default();
        assert!(!config.has_api_key("unsplash"));
        config.unsplash_api_key = Some("test_key".to_string());
        assert!(config.has_api_key("unsplash"));
        assert!(config.has_api_key("wallhaven")); // always available
    }
}
