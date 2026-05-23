use crate::models::SearchParams;

#[derive(Debug, Clone, PartialEq)]
pub enum AppStep {
    SelectSource,
    ConfigureFilters,
    ConfirmAndDownload,
    Downloading,
    Completed,
}

#[derive(Debug, Clone)]
pub struct App {
    pub current_step: AppStep,
    pub selected_source: Option<String>,
    pub search_params: SearchParams,
    pub available_sources: Vec<String>,
    pub current_filter_index: usize,
    pub message: Option<String>,
    pub error_message: Option<String>,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_step: AppStep::SelectSource,
            selected_source: None,
            search_params: SearchParams::default(),
            available_sources: vec![
                "unsplash".to_string(),
                "pexels".to_string(),
                "wallhaven".to_string(),
            ],
            current_filter_index: 0,
            message: None,
            error_message: None,
            should_quit: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
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
}
