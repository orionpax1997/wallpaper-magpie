use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub download_path: String,
    pub concurrent_downloads: usize,
    pub unsplash_api_key: Option<String>,
    pub pexels_api_key: Option<String>,
    pub wallhaven_api_key: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            download_path: "./wallpapers".to_string(),
            concurrent_downloads: 3,
            unsplash_api_key: None,
            pexels_api_key: None,
            wallhaven_api_key: None,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wallpaper-magpie")
            .join("config.toml")
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub fn has_api_key(&self, source: &str) -> bool {
        match source {
            "unsplash" => self
                .unsplash_api_key
                .as_ref()
                .is_some_and(|k| !k.is_empty()),
            "pexels" => self.pexels_api_key.as_ref().is_some_and(|k| !k.is_empty()),
            "wallhaven" => true,
            _ => false,
        }
    }

    pub fn get_api_key(&self, source: &str) -> Option<String> {
        match source {
            "unsplash" => self.unsplash_api_key.clone(),
            "pexels" => self.pexels_api_key.clone(),
            "wallhaven" => self.wallhaven_api_key.clone(),
            _ => None,
        }
    }

    pub fn set_api_key(&mut self, source: &str, key: String) {
        match source {
            "unsplash" => self.unsplash_api_key = Some(key),
            "pexels" => self.pexels_api_key = Some(key),
            "wallhaven" => self.wallhaven_api_key = Some(key),
            _ => {}
        }
    }

    pub fn expand_download_path(&self) -> PathBuf {
        PathBuf::from(&self.download_path)
    }
}
