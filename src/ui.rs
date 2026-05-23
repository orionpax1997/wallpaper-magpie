use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppStep};

pub fn draw(f: &mut Frame<'_>, app: &App) {
    let size = f.size();
    
    let background = Block::default().style(Style::default().bg(Color::Black));
    f.render_widget(background, size);
    
    match app.current_step {
        AppStep::SelectSource => draw_select_source(f, app, size),
        AppStep::ConfigureFilters => draw_configure_filters(f, app, size),
        AppStep::ConfirmAndDownload => draw_confirm(f, app, size),
        AppStep::Downloading => draw_downloading(f, app, size),
        AppStep::Completed => draw_completed(f, app, size),
    }
    
    if let Some(ref error) = app.error_message {
        draw_error_popup(f, error, size);
    }
}

fn draw_select_source(f: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);
    
    let title = Paragraph::new("步骤 1/3: 选择壁纸来源")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);
    
    let sources_text = app.available_sources.iter()
        .enumerate()
        .map(|(i, source)| {
            let prefix = if app.selected_source.as_ref() == Some(source) {
                "▸ "
            } else {
                "  "
            };
            Line::from(format!("{}{}. {}", prefix, i + 1, source))
        })
        .collect::<Vec<_>>();
    
    let sources = Paragraph::new(sources_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("可用来源"));
    f.render_widget(sources, chunks[1]);
}

fn draw_configure_filters(_f: &mut Frame<'_>, _app: &App, _area: Rect) {
}

fn draw_confirm(_f: &mut Frame<'_>, _app: &App, _area: Rect) {
}

fn draw_downloading(_f: &mut Frame<'_>, _app: &App, _area: Rect) {
}

fn draw_completed(_f: &mut Frame<'_>, _app: &App, _area: Rect) {
}

fn draw_error_popup(f: &mut Frame<'_>, error: &str, area: Rect) {
    let popup_area = centered_rect(60, 30, area);
    
    let block = Block::default()
        .title("错误")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red));
    
    let text = Paragraph::new(error)
        .style(Style::default().fg(Color::Red))
        .wrap(Wrap { trim: true })
        .block(block);
    
    f.render_widget(Clear, popup_area);
    f.render_widget(text, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
