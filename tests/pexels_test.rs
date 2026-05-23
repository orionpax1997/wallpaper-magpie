use wallpaper_magpie::models::{SearchParams, Provider, ApiKeyRequirement};
use wallpaper_magpie::providers::pexels::PexelsProvider;

#[tokio::test]
async fn test_pexels_provider_name() {
    let provider = PexelsProvider::new("test-key".to_string());
    assert_eq!(provider.name(), "pexels");
}

#[tokio::test]
async fn test_pexels_requires_api_key() {
    let provider = PexelsProvider::new("test-key".to_string());
    assert!(matches!(provider.requires_api_key(), ApiKeyRequirement::Required));
}

#[tokio::test]
async fn test_pexels_available_filters() {
    let provider = PexelsProvider::new("test-key".to_string());
    let filters = provider.available_filters();
    use wallpaper_magpie::models::FilterType;
    assert!(filters.contains(&FilterType::Size));
}
