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
    if key.code == KeyCode::Esc {
        if app.modal.is_some() {
            app.modal = None;
            return;
        }
        match app.current_step {
            AppStep::ConfigureFilters => app.previous_step(),
            AppStep::ConfirmAndDownload => app.previous_step(),
            _ => app.quit(),
        }
        return;
    }

    match app.current_step {
        AppStep::SelectSource => app.handle_page_one_input(key),
        AppStep::ConfigureFilters => handle_configure_filters(app, key),
        AppStep::ConfirmAndDownload => handle_confirm(app, key),
        _ => {}
    }
}

fn handle_configure_filters(app: &mut App, key: KeyEvent) {
    app.handle_page_two_input(key);
}

fn handle_confirm(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.previous_step();
        }
        KeyCode::Enter | KeyCode::Char('\r') | KeyCode::Char('d') | KeyCode::Char('D') => {
            app.next_step();
        }
        _ => {}
    }
}
