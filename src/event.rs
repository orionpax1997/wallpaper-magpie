use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

use crate::app::{App, AppStep};

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    pub fn next_event(&self) -> std::io::Result<AppEvent> {
        if event::poll(self.tick_rate)? {
            if let Event::Key(key) = event::read()? {
                return Ok(AppEvent::Key(key));
            }
        }
        Ok(AppEvent::Tick)
    }
}

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            if app.modal.is_some() {
                app.modal = None;
                return;
            }
            if app.editing_filter.is_some() {
                app.cancel_filter_edit();
                return;
            }
            match app.current_step {
                AppStep::ConfigureFilters => app.previous_step(),
                AppStep::ConfirmAndDownload => app.previous_step(),
                _ => app.quit(),
            }
            return;
        }
        _ => {}
    }

    match app.current_step {
        AppStep::SelectSource => app.handle_page_one_input(key),
        AppStep::ConfigureFilters => handle_configure_filters(app, key),
        AppStep::ConfirmAndDownload => handle_confirm(app, key),
        _ => {}
    }
}

fn handle_configure_filters(app: &mut App, key: KeyEvent) {
    if let Some(ref _filter) = app.editing_filter {
        match key.code {
            KeyCode::Enter | KeyCode::Char('\r') => app.commit_filter_edit(),
            KeyCode::Esc => app.cancel_filter_edit(),
            KeyCode::Backspace => {
                app.edit_buffer.pop();
            }
            KeyCode::Char(c) => {
                app.edit_buffer.push(c);
            }
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Tab | KeyCode::Down => {
            app.current_filter_index = (app.current_filter_index + 1) % 7;
        }
        KeyCode::Up => {
            app.current_filter_index = if app.current_filter_index == 0 {
                6
            } else {
                app.current_filter_index - 1
            };
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            let filter_names = vec![
                "query",
                "resolution",
                "color",
                "orientation",
                "sort",
                "limit",
            ];
            if let Some(name) = filter_names.get(app.current_filter_index) {
                app.start_editing_filter(name);
            }
        }
        KeyCode::Enter | KeyCode::Char('\r') | KeyCode::Char('n') | KeyCode::Char('N') => {
            let filter_names = vec![
                "query",
                "resolution",
                "color",
                "orientation",
                "sort",
                "limit",
                "下一步 →",
            ];
            if app.current_filter_index < 6 {
                let name = filter_names[app.current_filter_index];
                app.start_editing_filter(name);
            } else {
                app.next_step();
            }
        }
        _ => {}
    }
}

fn handle_confirm(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char('\r') | KeyCode::Char('d') | KeyCode::Char('D') => {
            app.next_step();
        }
        _ => {}
    }
}
