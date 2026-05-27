use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    backend::CrosstermBackend,
    Frame, Terminal,
};
use std::io::Stdout;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

use crate::components::modal::{render_modal, Modal, ModalType};
use crate::components::page_one::render_page_one;
use crate::components::page_three::render_page_three;
use crate::components::page_two::render_page_two;
use crate::components::{page_one::PageOne, page_three::PageThree, page_two::PageTwo};
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
    pub message: Option<String>,
    pub error_message: Option<String>,
    pub should_quit: bool,
    pub download_progress: Option<crate::download::DownloadProgress>,
    pub download_results: Vec<(String, bool)>,
    pub config: AppConfig,
    pub page_two: Option<PageTwo>,
    pub page_three: Option<PageThree>,
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
            message: None,
            error_message: None,
            should_quit: false,
            download_progress: None,
            download_results: Vec::new(),
            config: AppConfig::default(),
            page_two: None,
            page_three: None,
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
                        self.page_two = PageTwo::new(&source.name);
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

    pub fn handle_page_two_input(&mut self, key: KeyEvent) {
        let page_two = match &mut self.page_two {
            Some(p) => p,
            None => return,
        };

        if page_two.dropdown.is_some() {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    page_two.previous();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    page_two.next();
                }
                KeyCode::Enter => {
                    page_two.confirm_edit();
                }
                KeyCode::Esc => {
                    page_two.cancel_edit();
                }
                _ => {}
            }
            return;
        }

        if page_two.editing_index.is_some() {
            match key.code {
                KeyCode::Enter => {
                    page_two.confirm_edit();
                }
                KeyCode::Esc => {
                    page_two.cancel_edit();
                }
                KeyCode::Char(c) => {
                    page_two.editing_buffer.push(c);
                }
                KeyCode::Backspace => {
                    page_two.editing_buffer.pop();
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                page_two.previous();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                page_two.next();
            }
            KeyCode::Char('e') => {
                page_two.start_editing();
            }
            KeyCode::Esc => {
                self.current_step = AppStep::SelectSource;
                self.current_page = 1;
            }
            KeyCode::Enter => {
                if let Some(ref page_two) = self.page_two {
                    let filter_params = page_two.get_filter_params();
                    self.search_params.provider_specific.clear();
                    for (key, value) in filter_params {
                        if key == "sorting" {
                            self.search_params.sort = match value.as_str() {
                                "date_added" => Some(SortOrder::Latest),
                                "relevance" => Some(SortOrder::Popular),
                                "random" => Some(SortOrder::Random),
                                "views" => Some(SortOrder::Views),
                                "favorites" => Some(SortOrder::Favorites),
                                "toplist" => Some(SortOrder::Favorites),
                                _ => Some(SortOrder::Latest),
                            };
                        } else if key == "query" {
                            self.search_params.query = value;
                        } else if key == "atleast" && value.contains('x') {
                            self.search_params.resolution = Some(value);
                        } else if key == "colors" && !value.is_empty() {
                            self.search_params.color = Some(value);
                        } else if key == "page" {
                            let pages: u32 = value.parse().unwrap_or(1);
                            self.search_params.limit = pages * 10;
                            self.search_params.provider_specific.insert(key, value);
                        } else {
                            self.search_params.provider_specific.insert(key, value);
                        }
                    }
                }
                self.current_step = AppStep::ConfirmAndDownload;
                self.current_page = 3;
                if self.page_three.is_none() {
                    self.page_three = Some(PageThree::new(self.search_params.limit as usize));
                }
                if let Some(ref mut page) = self.page_three {
                    page.total = self.search_params.limit as usize;
                    page.confirm_cancel = false;
                    page.cancelled = false;
                }
            }
            _ => {}
        }
    }

    pub fn select_source(&mut self, source: String) {
        self.selected_source = Some(source);
        self.current_step = AppStep::ConfigureFilters;
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

    pub fn handle_page_three_input(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        let page = match &mut self.page_three {
            Some(p) => p,
            None => return,
        };

        match key.code {
            KeyCode::Enter | KeyCode::Char('\r') => {
                if !page.confirm_cancel {
                    self.next_step();
                }
            }
            KeyCode::Esc => {
                if page.confirm_cancel {
                    page.cancelled = true;
                    self.current_step = AppStep::ConfigureFilters;
                    self.current_page = 2;
                } else {
                    page.handle_esc();
                }
            }
            KeyCode::Up => {
                if !page.confirm_cancel {
                    page.scroll_up();
                }
            }
            KeyCode::Down => {
                if !page.confirm_cancel {
                    page.scroll_down();
                }
            }
            _ => {
                if page.confirm_cancel {
                    page.dismiss_confirm();
                }
            }
        }
    }

    pub async fn handle_input(&mut self, key: KeyEvent) {
        if let Some(ref mut modal) = self.modal {
            match key.code {
                KeyCode::Enter => {
                    let value = modal.get_value();
                    if let ModalType::ApiKeyEdit { source, .. } = &modal.modal_type {
                        self.config.set_api_key(source, value);
                        if let Err(e) = ConfigManager::save(&self.config) {
                            let _ = e;
                        }
                    }
                    self.modal = None;
                }
                KeyCode::Esc => {
                    self.modal = None;
                }
                KeyCode::Char(c) => modal.handle_input(c),
                KeyCode::Backspace => modal.handle_backspace(),
                KeyCode::Delete => modal.handle_delete(),
                KeyCode::Left => modal.move_cursor_left(),
                KeyCode::Right => modal.move_cursor_right(),
                _ => {}
            }
            return;
        }

        match self.current_page {
            1 => self.handle_page_one_input(key),
            2 => self.handle_page_two_input(key),
            3 => self.handle_page_three_input(key),
            _ => {}
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let area = f.size();

        match self.current_page {
            1 => {
                render_page_one(f, &self.page_one, &self.config, area);
            }
            2 => {
                if let Some(ref page_two) = self.page_two {
                    render_page_two(f, page_two, area);
                }
            }
            3 => {
                if let Some(ref mut page_three) = self.page_three {
                    render_page_three(f, page_three, area);
                }
            }
            _ => {}
        }

        if let Some(ref modal) = self.modal {
            render_modal(f, modal);
        }
    }

    pub async fn execute_download(
        &mut self,
        terminal: Arc<TokioMutex<Terminal<CrosstermBackend<Stdout>>>>,
    ) -> crate::error::Result<()> {
        use crate::download::{DownloadManager, DownloadStatus};
        use crate::providers;
        use tokio::sync::mpsc;

        let source_name = self.selected_source.as_ref().unwrap();
        let api_key = self.config.get_api_key(source_name).unwrap_or_default();
        let provider = providers::create_provider(source_name, &api_key).ok_or_else(|| {
            crate::error::AppError::ApiKeyRequired {
                provider: source_name.clone(),
            }
        })?;

        let wallpapers = provider.search(&self.search_params).await?;

        let download_path = self.config.expand_download_path().join(source_name);
        let manager = DownloadManager::new(self.config.concurrent_downloads);

        let (progress_tx, mut progress_rx) = mpsc::channel(100);

        if let Some(ref mut page) = self.page_three {
            page.total = wallpapers.len();
            page.completed = 0;
            page.in_progress = 0;
            page.failed = 0;
            page.pending = wallpapers.len();
            page.is_preparing = true;
        }

        let download_handle = tokio::spawn(async move {
            manager
                .download_wallpapers(provider, wallpapers, download_path, progress_tx)
                .await
        });

        while let Some(progress) = progress_rx.recv().await {
            self.download_progress = Some(progress.clone());
            if let Some(ref mut page) = self.page_three {
                page.total = progress.total;
                page.pending = progress.pending;

                if let Some(ref filename) = progress.current_file {
                    match progress.status {
                        DownloadStatus::Pending => {
                            page.is_preparing = false;
                            page.add_log(crate::components::page_three::LogEntry::Start {
                                filename: filename.clone(),
                            });
                            page.in_progress += 1;
                        }
                        DownloadStatus::Completed => {
                            page.add_log(crate::components::page_three::LogEntry::Success {
                                filename: filename.clone(),
                            });
                            page.in_progress = page.in_progress.saturating_sub(1);
                            page.completed += 1;
                        }
                        DownloadStatus::Failed => {
                            page.add_log(crate::components::page_three::LogEntry::Failure {
                                filename: filename.clone(),
                                error: "download failed".to_string(),
                            });
                            page.in_progress = page.in_progress.saturating_sub(1);
                            page.failed += 1;
                        }
                        DownloadStatus::Downloading => {}
                    }
                }
            }

            let term = terminal.clone();
            let app = self as *mut App;
            tokio::task::block_in_place(move || {
                let mut term = term.blocking_lock();
                let app = unsafe { &mut *app };
                term.draw(|f| {
                    app.draw(f);
                })
                .ok();
            });
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
