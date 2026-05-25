use serde::Deserialize;
use std::path::Path;

use crate::error::{AppError, Result};
use crate::models::{ApiKeyRequirement, FilterType, Provider, SearchParams, Wallpaper};

pub struct PexelsProvider {
    api_key: String,
    client: reqwest::Client,
}

impl PexelsProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    fn build_search_url(&self, params: &SearchParams) -> String {
        let mut url = "https://api.pexels.com/v1/search".to_string();

        url.push_str(&format!("?query={}", urlencoding::encode(&params.query)));
        url.push_str(&format!("&per_page={}", params.limit.min(80)));

        if let Some(ref orientation) = params.orientation {
            let pexels_orientation = match orientation.as_str() {
                "landscape" => "landscape",
                "portrait" => "portrait",
                "squarish" | "square" => "square",
                _ => orientation.as_str(),
            };
            url.push_str(&format!("&orientation={}", pexels_orientation));
        }

        if let Some(ref color) = params.color {
            url.push_str(&format!("&color={}", color));
        }

        if let Some(ref size) = params.provider_specific.get("size") {
            url.push_str(&format!("&size={}", size));
        }

        url
    }

    fn parse_photos(&self, response: PexelsSearchResponse) -> Vec<Wallpaper> {
        response
            .photos
            .into_iter()
            .map(|photo| Wallpaper {
                id: photo.id.to_string(),
                source: "pexels".to_string(),
                url: photo.src.original,
                filename: format!("pexels-{}.jpg", photo.id),
                resolution: Some(format!("{}x{}", photo.width, photo.height)),
                file_size: None,
                photographer: Some(photo.photographer),
            })
            .collect()
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
        vec![
            FilterType::Query,
            FilterType::Resolution,
            FilterType::Color,
            FilterType::Orientation,
            FilterType::Size,
            FilterType::Limit,
        ]
    }

    async fn search(&self, params: &SearchParams) -> Result<Vec<Wallpaper>> {
        let url = self.build_search_url(params);

        let response = self
            .client
            .get(&url)
            .header("Authorization", &self.api_key)
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

        let data: PexelsSearchResponse = response.json().await?;
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
struct PexelsSearchResponse {
    total_results: u32,
    page: u32,
    per_page: u32,
    photos: Vec<PexelsPhoto>,
    next_page: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PexelsPhoto {
    id: u64,
    width: u32,
    height: u32,
    url: String,
    photographer: String,
    src: PexelsSrc,
}

#[derive(Debug, Deserialize)]
struct PexelsSrc {
    original: String,
    large2x: String,
    large: String,
    medium: String,
    small: String,
    portrait: String,
    landscape: String,
    tiny: String,
}
