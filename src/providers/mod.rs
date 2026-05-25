use std::sync::Arc;

use crate::config::SourceConfig;
use crate::models::Provider;

pub mod pexels;
pub mod unsplash;
pub mod wallhaven;

use pexels::PexelsProvider;
use unsplash::UnsplashProvider;
use wallhaven::WallhavenProvider;

pub fn create_provider(name: &str, config: &SourceConfig) -> Option<Arc<dyn Provider>> {
    match name {
        "unsplash" => {
            if config.api_key.is_empty() {
                None
            } else {
                Some(Arc::new(UnsplashProvider::new(config.api_key.clone())))
            }
        }
        "pexels" => {
            if config.api_key.is_empty() {
                None
            } else {
                Some(Arc::new(PexelsProvider::new(config.api_key.clone())))
            }
        }
        "wallhaven" => Some(Arc::new(WallhavenProvider::new(config.api_key.clone()))),
        _ => None,
    }
}
