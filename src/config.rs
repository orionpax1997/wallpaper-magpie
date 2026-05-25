use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub download_path: String,
    pub concurrent_downloads: usize,
    pub sources: HashMap<String, SourceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceConfig {
    pub enabled: bool,
    pub api_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut sources = HashMap::new();
        sources.insert(
            "unsplash".to_string(),
            SourceConfig {
                enabled: true,
                api_key: String::new(),
            },
        );
        sources.insert(
            "pexels".to_string(),
            SourceConfig {
                enabled: false,
                api_key: String::new(),
            },
        );
        sources.insert(
            "wallhaven".to_string(),
            SourceConfig {
                enabled: true,
                api_key: String::new(),
            },
        );

        Self {
            download_path: "./wallpapers".to_string(),
            concurrent_downloads: 3,
            sources,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        PathBuf::from("./config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| AppError::ConfigError(format!("Failed to read config: {}", e)))?;

        let config: AppConfig = toml::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse config: {}", e)))?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();

        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(&path, content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    pub fn get_source_config(&self, name: &str) -> Option<&SourceConfig> {
        self.sources.get(name)
    }

    pub fn set_source_config(&mut self, name: &str, source_config: SourceConfig) {
        self.sources.insert(name.to_string(), source_config);
    }

    pub fn expand_download_path(&self) -> PathBuf {
        PathBuf::from(&self.download_path)
    }
}
