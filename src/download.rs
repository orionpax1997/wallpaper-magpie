use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task;

use crate::error::{AppError, Result};
use crate::models::{Provider, Wallpaper};

impl From<tokio::sync::AcquireError> for AppError {
    fn from(e: tokio::sync::AcquireError) -> Self {
        AppError::DownloadError(e.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub current_file: Option<String>,
}

pub struct DownloadManager {
    concurrent_limit: usize,
}

impl DownloadManager {
    pub fn new(concurrent_limit: usize) -> Self {
        Self { concurrent_limit }
    }

    pub async fn download_wallpapers(
        &self,
        provider: Arc<dyn Provider>,
        wallpapers: Vec<Wallpaper>,
        base_path: PathBuf,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<Vec<(Wallpaper, Result<PathBuf>)>> {
        let semaphore = Arc::new(Semaphore::new(self.concurrent_limit));
        let total = wallpapers.len();
        let mut handles = Vec::new();

        for (idx, wallpaper) in wallpapers.into_iter().enumerate() {
            let permit = semaphore.clone().acquire_owned().await?;
            let provider = provider.clone();
            let tx = progress_tx.clone();
            let base = base_path.clone();

            let handle = task::spawn(async move {
                let result = Self::download_single(provider, &wallpaper, &base).await;

                let progress = DownloadProgress {
                    total,
                    completed: idx + 1,
                    failed: if result.is_err() { 1 } else { 0 },
                    current_file: Some(wallpaper.filename.clone()),
                };
                let _ = tx.send(progress).await;

                drop(permit);
                (wallpaper, result)
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    return Err(AppError::DownloadError(format!("Task join error: {}", e)));
                }
            }
        }

        Ok(results)
    }

    async fn download_single(
        provider: Arc<dyn Provider>,
        wallpaper: &Wallpaper,
        base_path: &Path,
    ) -> Result<PathBuf> {
        let file_path = base_path.join(&wallpaper.filename);

        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        provider.download(wallpaper, &file_path).await?;

        Ok(file_path)
    }
}
