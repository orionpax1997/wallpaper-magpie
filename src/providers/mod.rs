use std::sync::Arc;

use crate::models::Provider;

pub mod pexels;
pub mod unsplash;
pub mod wallhaven;

use pexels::PexelsProvider;
use unsplash::UnsplashProvider;
use wallhaven::WallhavenProvider;

pub fn create_provider(name: &str, api_key: &str) -> Option<Arc<dyn Provider>> {
    match name {
        "unsplash" => {
            if api_key.is_empty() {
                None
            } else {
                Some(Arc::new(UnsplashProvider::new(api_key.to_string())))
            }
        }
        "pexels" => {
            if api_key.is_empty() {
                None
            } else {
                Some(Arc::new(PexelsProvider::new(api_key.to_string())))
            }
        }
        "wallhaven" => Some(Arc::new(WallhavenProvider::new(api_key.to_string()))),
        _ => None,
    }
}
