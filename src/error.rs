use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("API request failed: {status_code} - {message}")]
    ApiError { status_code: u16, message: String },

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Download failed: {0}")]
    DownloadError(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Rate limit exceeded. Retry after: {retry_after} seconds")]
    RateLimitError { retry_after: u64 },

    #[error("API Key required for {provider}. Please configure it.")]
    ApiKeyRequired { provider: String },

    #[error("Invalid filter value: {filter} = {value}")]
    InvalidFilter { filter: String, value: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP client error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Configuration parse error: {0}")]
    ConfigParseError(#[from] config::ConfigError),
}

#[allow(unused)]
pub type Result<T> = std::result::Result<T, AppError>;
