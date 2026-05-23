use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

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
