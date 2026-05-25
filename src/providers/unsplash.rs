use serde::Deserialize;
use std::path::Path;

use crate::error::{AppError, Result};
use crate::models::{ApiKeyRequirement, FilterType, Provider, SearchParams, SortOrder, Wallpaper};

pub struct UnsplashProvider {
    api_key: String,
    client: reqwest::Client,
}

impl UnsplashProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    fn build_search_url(&self, params: &SearchParams) -> String {
        let mut url = "https://api.unsplash.com/search/photos".to_string();

        url.push_str(&format!("?query={}", urlencoding::encode(&params.query)));
        url.push_str(&format!("&per_page={}", params.limit.min(30)));

        if let Some(ref order) = params.sort {
            let order_str = match order {
                SortOrder::Latest => "latest",
                SortOrder::Relevant => "relevant",
                _ => "relevant",
            };
            url.push_str(&format!("&order_by={}", order_str));
        }

        if let Some(ref orientation) = params.orientation {
            url.push_str(&format!("&orientation={}", orientation));
        }

        if let Some(ref color) = params.color {
            url.push_str(&format!("&color={}", color));
        }

        url
    }

    fn parse_photos(&self, response: UnsplashSearchResponse) -> Vec<Wallpaper> {
        response
            .results
            .into_iter()
            .map(|photo| Wallpaper {
                id: photo.id.clone(),
                source: "unsplash".to_string(),
                url: photo.urls.raw,
                filename: format!("unsplash-{}.jpg", photo.id),
                resolution: Some(format!("{}x{}", photo.width, photo.height)),
                file_size: None,
                photographer: Some(photo.user.name),
            })
            .collect()
    }
}

#[async_trait::async_trait]
impl Provider for UnsplashProvider {
    fn name(&self) -> &str {
        "unsplash"
    }

    fn requires_api_key(&self) -> ApiKeyRequirement {
        ApiKeyRequirement::Required
    }

    fn available_filters(&self) -> Vec<FilterType> {
        vec![
            FilterType::Query,
            FilterType::Resolution,
            FilterType::Color,
            FilterType::Orientation,
            FilterType::Sort,
            FilterType::Limit,
        ]
    }

    async fn search(&self, params: &SearchParams) -> Result<Vec<Wallpaper>> {
        let url = self.build_search_url(params);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Client-ID {}", self.api_key))
            .header("Accept-Version", "v1")
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

        let data: UnsplashSearchResponse = response.json().await?;
        Ok(self.parse_photos(data))
    }

    async fn download(&self, wallpaper: &Wallpaper, path: &Path) -> Result<()> {
        let response = self.client.get(&wallpaper.url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::DownloadError(format!(
                "Failed to download: HTTP {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await?;
        std::fs::write(path, bytes)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct UnsplashSearchResponse {
    total: u32,
    total_pages: u32,
    results: Vec<UnsplashPhoto>,
}

#[derive(Debug, Deserialize)]
struct UnsplashPhoto {
    id: String,
    width: u32,
    height: u32,
    urls: UnsplashUrls,
    user: UnsplashUser,
}

#[derive(Debug, Deserialize)]
struct UnsplashUrls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String,
}

#[derive(Debug, Deserialize)]
struct UnsplashUser {
    name: String,
}
