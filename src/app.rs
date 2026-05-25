use crossterm::event::{KeyCode, KeyEvent};

use crate::components::modal::{Modal, ModalType};
use crate::components::page_one::PageOne;
use crate::config::AppConfig as FullConfig;
use crate::config_manager::ConfigManager;
use crate::models::{AppConfig, SearchParams, SortOrder};

#[derive(Debug, Clone, PartialEq)]
pub enum AppStep {
    SelectSource,
    ConfigureFilters,
    ConfirmAndDownload,
    Downloading,
    Completed,
}

#[derive(Debug)]
pub struct App {
    pub current_step: AppStep,
    pub page_one: PageOne,
    pub current_page: u8,
    pub modal: Option<Modal>,
    pub selected_source: Option<String>,
    pub search_params: SearchParams,
    pub available_sources: Vec<String>,
    pub current_filter_index: usize,
    pub filter_values: std::collections::HashMap<String, String>,
    pub editing_filter: Option<String>,
    pub edit_buffer: String,
    pub message: Option<String>,
    pub error_message: Option<String>,
    pub should_quit: bool,
    pub download_progress: Option<crate::download::DownloadProgress>,
    pub download_results: Vec<(String, bool)>,
    pub config: AppConfig,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_step: AppStep::SelectSource,
            page_one: PageOne::new(&AppConfig::default()),
            current_page: 1,
            modal: None,
            selected_source: None,
            search_params: SearchParams::default(),
            available_sources: vec![
                "unsplash".to_string(),
                "pexels".to_string(),
                "wallhaven".to_string(),
            ],
            current_filter_index: 0,
            filter_values: std::collections::HashMap::new(),
            editing_filter: None,
            edit_buffer: String::new(),
            message: None,
            error_message: None,
            should_quit: false,
            download_progress: None,
            download_results: Vec::new(),
            config: AppConfig::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let config = ConfigManager::load().unwrap_or_default();
        let page_one = PageOne::new(&config);
        Self {
            config,
            page_one,
            ..Self::default()
        }
    }

    pub fn handle_page_one_input(&mut self, key: KeyEvent) {
        if self.modal.is_some() {
            return;
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.page_one.previous();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.page_one.next();
            }
            KeyCode::Enter => {
                if let Some(source) = self.page_one.get_selected_source() {
                    if self.page_one.is_selected_available(&self.config) {
                        self.selected_source = Some(source.name.clone());
                        self.current_step = AppStep::ConfigureFilters;
                        self.current_page = 2;
                    }
                }
            }
            KeyCode::Char('e') => {
                if let Some(source) = self.page_one.get_selected_source() {
                    let current_value = self.config.get_api_key(&source.name).unwrap_or_default();
                    let modal_type = ModalType::ApiKeyEdit {
                        source: source.name.clone(),
                        current_value,
                    };
                    self.modal = Some(Modal::new(modal_type));
                }
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn select_source(&mut self, source: String) {
        self.selected_source = Some(source);
        self.current_step = AppStep::ConfigureFilters;
        self.current_filter_index = 0;
    }

    pub fn next_step(&mut self) {
        self.current_step = match self.current_step {
            AppStep::SelectSource => AppStep::ConfigureFilters,
            AppStep::ConfigureFilters => AppStep::ConfirmAndDownload,
            AppStep::ConfirmAndDownload => AppStep::Downloading,
            AppStep::Downloading => AppStep::Completed,
            AppStep::Completed => AppStep::SelectSource,
        };
    }

    pub fn previous_step(&mut self) {
        self.current_step = match self.current_step {
            AppStep::ConfigureFilters => AppStep::SelectSource,
            AppStep::ConfirmAndDownload => AppStep::ConfigureFilters,
            _ => self.current_step.clone(),
        };
    }

    pub fn set_error(&mut self, msg: String) {
        self.error_message = Some(msg);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn start_editing_filter(&mut self, filter_name: &str) {
        self.editing_filter = Some(filter_name.to_string());
        self.edit_buffer = self
            .filter_values
            .get(filter_name)
            .cloned()
            .unwrap_or_default();
    }

    pub fn commit_filter_edit(&mut self) {
        if let Some(ref filter) = self.editing_filter {
            self.filter_values
                .insert(filter.clone(), self.edit_buffer.clone());

            match filter.as_str() {
                "query" => self.search_params.query = self.edit_buffer.clone(),
                "resolution" => self.search_params.resolution = Some(self.edit_buffer.clone()),
                "color" => self.search_params.color = Some(self.edit_buffer.clone()),
                "orientation" => self.search_params.orientation = Some(self.edit_buffer.clone()),
                "limit" => {
                    if let Ok(val) = self.edit_buffer.parse::<u32>() {
                        self.search_params.limit = val;
                    }
                }
                "sort" => {
                    self.search_params.sort = match self.edit_buffer.as_str() {
                        "latest" => Some(SortOrder::Latest),
                        "popular" => Some(SortOrder::Popular),
                        "relevant" => Some(SortOrder::Relevant),
                        "random" => Some(SortOrder::Random),
                        _ => None,
                    };
                }
                _ => {}
            }
        }
        self.editing_filter = None;
    }

    pub fn cancel_filter_edit(&mut self) {
        self.editing_filter = None;
        self.edit_buffer.clear();
    }

    pub async fn execute_download(&mut self) -> crate::error::Result<()> {
        use crate::download::DownloadManager;
        use crate::providers;
        use tokio::sync::mpsc;

        let full_config = FullConfig::load()?;

        let source_name = self.selected_source.as_ref().unwrap();
        let source_config = full_config.get_source_config(source_name).ok_or_else(|| {
            crate::error::AppError::ConfigError(format!("Source {} not configured", source_name))
        })?;

        let provider = providers::create_provider(source_name, source_config).ok_or_else(|| {
            crate::error::AppError::ApiKeyRequired {
                provider: source_name.clone(),
            }
        })?;

        let wallpapers = provider.search(&self.search_params).await?;

        let download_path = full_config.expand_download_path().join(source_name);
        let manager = DownloadManager::new(full_config.concurrent_downloads);

        let (progress_tx, mut progress_rx) = mpsc::channel(100);

        let download_handle = tokio::spawn(async move {
            manager
                .download_wallpapers(provider, wallpapers, download_path, progress_tx)
                .await
        });

        while let Some(progress) = progress_rx.recv().await {
            self.download_progress = Some(progress);
        }

        let results = download_handle.await.map_err(|e| {
            crate::error::AppError::DownloadError(format!("Download task failed: {}", e))
        })??;

        self.download_results = results
            .into_iter()
            .map(|(wallpaper, result)| (wallpaper.filename, result.is_ok()))
            .collect();

        self.current_step = AppStep::Completed;

        Ok(())
    }
}
