use wallpaper_magpie::models::Provider;
use wallpaper_magpie::providers::unsplash::UnsplashProvider;

#[tokio::test]
async fn test_unsplash_provider_name() {
    let provider = UnsplashProvider::new("test-key".to_string());
    assert_eq!(provider.name(), "unsplash");
}

#[tokio::test]
async fn test_unsplash_requires_api_key() {
    let provider = UnsplashProvider::new("test-key".to_string());
    use wallpaper_magpie::models::ApiKeyRequirement;
    assert!(matches!(provider.requires_api_key(), ApiKeyRequirement::Required));
}

#[tokio::test]
async fn test_unsplash_available_filters() {
    let provider = UnsplashProvider::new("test-key".to_string());
    let filters = provider.available_filters();
    assert!(!filters.is_empty());
}
