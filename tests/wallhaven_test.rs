use wallpaper_magpie::models::{SearchParams, Provider, ApiKeyRequirement};
use wallpaper_magpie::providers::wallhaven::WallhavenProvider;

#[tokio::test]
async fn test_wallhaven_provider_name() {
    let provider = WallhavenProvider::new("".to_string());
    assert_eq!(provider.name(), "wallhaven");
}

#[tokio::test]
async fn test_wallhaven_optional_api_key() {
    let provider = WallhavenProvider::new("".to_string());
    assert!(matches!(provider.requires_api_key(), ApiKeyRequirement::Optional));
    
    let provider_with_key = WallhavenProvider::new("test-key".to_string());
    assert!(matches!(provider_with_key.requires_api_key(), ApiKeyRequirement::Required));
}

#[tokio::test]
async fn test_wallhaven_available_filters() {
    let provider = WallhavenProvider::new("".to_string());
    let filters = provider.available_filters();
    use wallpaper_magpie::models::FilterType;
    assert!(filters.contains(&FilterType::Purity));
    assert!(filters.contains(&FilterType::Category));
    assert!(filters.contains(&FilterType::TopRange));
}
