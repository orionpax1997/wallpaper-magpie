use std::path::Path;

use crate::error::{AppError, Result};
use crate::models::{ApiKeyRequirement, FilterType, Provider, SearchParams, Wallpaper};

pub struct WallhavenProvider {
    api_key: Option<String>,
}

impl WallhavenProvider {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }
}

#[async_trait::async_trait]
impl Provider for WallhavenProvider {
    fn name(&self) -> &str {
        "wallhaven"
    }
    
    fn requires_api_key(&self) -> ApiKeyRequirement {
        ApiKeyRequirement::Optional
    }
    
    fn available_filters(&self) -> Vec<FilterType> {
        vec![]
    }
    
    async fn search(&self, _params: &SearchParams) -> Result<Vec<Wallpaper>> {
        Err(AppError::ProviderError("Not implemented".to_string()))
    }
    
    async fn download(&self, _wallpaper: &Wallpaper, _path: &Path) -> Result<()> {
        Err(AppError::ProviderError("Not implemented".to_string()))
    }
}
