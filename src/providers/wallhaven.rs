use std::path::Path;
use serde::Deserialize;

use crate::error::{AppError, Result};
use crate::models::{ApiKeyRequirement, FilterType, Provider, SearchParams, SortOrder, Wallpaper};

pub struct WallhavenProvider {
    api_key: String,
    client: reqwest::Client,
}

impl WallhavenProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
    
    fn build_search_url(&self, params: &SearchParams) -> String {
        let mut url = "https://wallhaven.cc/api/v1/search".to_string();
        
        if !params.query.is_empty() {
            url.push_str(&format!("?q={}", urlencoding::encode(&params.query)));
        } else {
            url.push('?');
        }
        
        let per_page = 24u32;
        let pages_needed = ((params.limit + per_page - 1) / per_page).max(1);
        url.push_str(&format!("&page=1&pages={}", pages_needed));
        
        if let Some(ref resolution) = params.resolution {
            if resolution.contains('x') {
                url.push_str(&format!("&atleast={}", resolution));
            }
        }
        
        if let Some(ref color) = params.color {
            url.push_str(&format!("&colors={}", color.trim_start_matches('#')));
        }
        
        if let Some(ref sort) = params.sort {
            let sorting = match sort {
                SortOrder::Latest => "date_added",
                SortOrder::Popular => "relevance",
                SortOrder::Random => "random",
                SortOrder::Views => "views",
                SortOrder::Favorites => "favorites",
                _ => "date_added",
            };
            url.push_str(&format!("&sorting={}", sorting));
        }
        
        if let Some(ref top_range) = params.provider_specific.get("topRange") {
            url.push_str(&format!("&topRange={}", top_range));
        }
        
        if let Some(ref categories) = params.provider_specific.get("categories") {
            url.push_str(&format!("&categories={}", categories));
        } else {
            url.push_str("&categories=111");
        }
        
        if let Some(ref purity) = params.provider_specific.get("purity") {
            url.push_str(&format!("&purity={}", purity));
        } else {
            url.push_str("&purity=100");
        }
        
        if !self.api_key.is_empty() {
            url.push_str(&format!("&apikey={}", self.api_key));
        }
        
        url
    }
    
    fn parse_wallpapers(&self, response: WallhavenSearchResponse, limit: u32) -> Vec<Wallpaper> {
        response.data.into_iter()
            .take(limit as usize)
            .map(|wallpaper| Wallpaper {
                id: wallpaper.id.clone(),
                source: "wallhaven".to_string(),
                url: wallpaper.path,
                filename: format!("wallhaven-{}.jpg", wallpaper.id),
                resolution: Some(wallpaper.resolution),
                file_size: Some(wallpaper.file_size),
                photographer: None,
            })
            .collect()
    }
}

#[async_trait::async_trait]
impl Provider for WallhavenProvider {
    fn name(&self) -> &str {
        "wallhaven"
    }
    
    fn requires_api_key(&self) -> ApiKeyRequirement {
        if self.api_key.is_empty() {
            ApiKeyRequirement::Optional
        } else {
            ApiKeyRequirement::Required
        }
    }
    
    fn available_filters(&self) -> Vec<FilterType> {
        vec![
            FilterType::Query,
            FilterType::Resolution,
            FilterType::Color,
            FilterType::Sort,
            FilterType::Limit,
            FilterType::Purity,
            FilterType::Category,
            FilterType::TopRange,
        ]
    }
    
    async fn search(&self, params: &SearchParams) -> Result<Vec<Wallpaper>> {
        let url = self.build_search_url(params);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::ApiError {
                status_code: status.as_u16(),
                message: text,
            });
        }
        
        let data: WallhavenSearchResponse = response.json().await?;
        Ok(self.parse_wallpapers(data, params.limit))
    }
    
    async fn download(&self, wallpaper: &Wallpaper, path: &Path) -> Result<()> {
        let response = self.client
            .get(&wallpaper.url)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(AppError::DownloadError(
                format!("Failed to download: HTTP {}", response.status())
            ));
        }
        
        let bytes = response.bytes().await?;
        std::fs::write(path, bytes)?;
        
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct WallhavenSearchResponse {
    data: Vec<WallhavenWallpaper>,
    meta: WallhavenMeta,
}

#[derive(Debug, Deserialize)]
struct WallhavenWallpaper {
    id: String,
    url: String,
    resolution: String,
    file_size: u64,
    path: String,
}

#[derive(Debug, Deserialize)]
struct WallhavenMeta {
    current_page: u32,
    last_page: u32,
    per_page: u32,
    total: u32,
    query: Option<String>,
    seed: Option<String>,
}
