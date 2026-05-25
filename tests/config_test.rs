use wallpaper_magpie::config::AppConfig;

#[test]
fn test_default_config() {
    let config = AppConfig::default();
    assert_eq!(config.download_path, "./wallpapers");
    assert_eq!(config.concurrent_downloads, 3);
    assert!(config.sources.contains_key("unsplash"));
    assert!(config.sources.contains_key("pexels"));
    assert!(config.sources.contains_key("wallhaven"));
}

#[test]
fn test_save_and_load_config() {
    let mut config = AppConfig::default();
    config.download_path = "./test-wallpapers".to_string();

    config.save().unwrap();

    let loaded = AppConfig::load().unwrap();
    assert_eq!(loaded.download_path, "./test-wallpapers");

    let default = AppConfig::default();
    default.save().unwrap();
}

#[test]
fn test_expand_download_path() {
    let config = AppConfig::default();
    let path = config.expand_download_path();
    assert!(path.ends_with("wallpapers"));
}
