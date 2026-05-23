use std::path::Path;

use crate::error::{AppError, Result};
use crate::models::{ApiKeyRequirement, FilterType, Provider, SearchParams, Wallpaper};

pub struct PexelsProvider {
    api_key: String,
}

impl PexelsProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait::async_trait]
impl Provider for PexelsProvider {
    fn name(&self) -> &str {
        "pexels"
    }
    
    fn requires_api_key(&self) -> ApiKeyRequirement {
        ApiKeyRequirement::Required
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
