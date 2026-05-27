use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
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

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub pending: usize,
    pub status: DownloadStatus,
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
        cancelled: Arc<AtomicBool>,
    ) -> Result<Vec<(Wallpaper, Result<PathBuf>)>> {
        let semaphore = Arc::new(Semaphore::new(self.concurrent_limit));
        let total = wallpapers.len();
        let mut handles = Vec::new();

        for (idx, wallpaper) in wallpapers.into_iter().enumerate() {
            if cancelled.load(Ordering::Relaxed) {
                break;
            }

            let permit = semaphore.clone().acquire_owned().await?;
            let provider = provider.clone();
            let tx = progress_tx.clone();
            let base = base_path.clone();
            let cancelled = cancelled.clone();

            let handle = task::spawn(async move {
                let start_progress = DownloadProgress {
                    total,
                    completed: idx,
                    failed: 0,
                    pending: total - idx,
                    status: DownloadStatus::Pending,
                    current_file: Some(wallpaper.filename.clone()),
                };
                let _ = tx.send(start_progress).await;

                if cancelled.load(Ordering::Relaxed) {
                    drop(permit);
                    return (wallpaper, Err(AppError::DownloadError("已取消".to_string())));
                }

                let result = Self::download_single(provider, &wallpaper, &base).await;

                let progress = DownloadProgress {
                    total,
                    completed: idx + 1,
                    failed: if result.is_err() { 1 } else { 0 },
                    pending: total - idx - 1,
                    status: if result.is_ok() {
                        DownloadStatus::Completed
                    } else {
                        DownloadStatus::Failed
                    },
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
            if cancelled.load(Ordering::Relaxed) {
                handle.abort();
                continue;
            }
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
