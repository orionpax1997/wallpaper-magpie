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

    #[test]
    fn test_get_api_key() {
        let mut config = AppConfig::default();
        assert_eq!(config.get_api_key("unsplash"), None);
        config.unsplash_api_key = Some("key123".to_string());
        assert_eq!(config.get_api_key("unsplash"), Some("key123".to_string()));
        assert_eq!(config.get_api_key("wallhaven"), None);
        assert_eq!(config.get_api_key("unknown"), None);
    }

    #[test]
    fn test_set_api_key() {
        let mut config = AppConfig::default();
        config.set_api_key("unsplash", "key1".to_string());
        assert_eq!(config.unsplash_api_key, Some("key1".to_string()));
        config.set_api_key("pexels", "key2".to_string());
        assert_eq!(config.pexels_api_key, Some("key2".to_string()));
        config.set_api_key("wallhaven", "key3".to_string());
        assert_eq!(config.wallhaven_api_key, Some("key3".to_string()));
        config.set_api_key("unknown", "key4".to_string()); // should not panic
        assert_eq!(config.get_api_key("unknown"), None);
    }

    #[test]
    fn test_has_api_key_empty_string() {
        let mut config = AppConfig::default();
        config.unsplash_api_key = Some("".to_string());
        assert!(
            !config.has_api_key("unsplash"),
            "Empty string should return false"
        );
    }
}
