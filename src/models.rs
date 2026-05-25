use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::Result;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchParams {
    pub query: String,
    pub limit: u32,
    pub resolution: Option<String>,
    pub color: Option<String>,
    pub orientation: Option<String>,
    pub sort: Option<SortOrder>,
    pub provider_specific: HashMap<String, String>,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: 10,
            resolution: None,
            color: None,
            orientation: None,
            sort: None,
            provider_specific: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub source: String,
    pub url: String,
    pub filename: String,
    pub resolution: Option<String>,
    pub file_size: Option<u64>,
    pub photographer: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SortOrder {
    Latest,
    Popular,
    Relevant,
    Random,
    Views,
    Favorites,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Latest => "latest",
            SortOrder::Popular => "popular",
            SortOrder::Relevant => "relevant",
            SortOrder::Random => "random",
            SortOrder::Views => "views",
            SortOrder::Favorites => "favorites",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FilterType {
    Query,
    Resolution,
    Color,
    Orientation,
    Sort,
    Limit,
    Purity,
    Category,
    Size,
    TopRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ApiKeyRequirement {
    Required,
    Optional,
    NotRequired,
}

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn requires_api_key(&self) -> ApiKeyRequirement;
    fn available_filters(&self) -> Vec<FilterType>;
    async fn search(&self, params: &SearchParams) -> Result<Vec<Wallpaper>>;
    async fn download(&self, wallpaper: &Wallpaper, path: &Path) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub unsplash_api_key: Option<String>,
    pub pexels_api_key: Option<String>,
    pub wallhaven_api_key: Option<String>,
}

impl AppConfig {
    pub fn config_path() -> std::path::PathBuf {
        std::path::PathBuf::from("config.toml")
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
            "wallhaven" => true, // Wallhaven always available
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
}
