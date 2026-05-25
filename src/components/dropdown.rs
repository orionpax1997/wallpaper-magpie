use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

#[derive(Debug, Clone)]
pub struct Dropdown {
    pub options: Vec<String>,
    pub selected_index: usize,
    pub title: String,
}

impl Dropdown {
    pub fn new(title: String, options: Vec<String>, current_value: Option<String>) -> Self {
        let selected_index = current_value
            .and_then(|v| options.iter().position(|o| o == &v))
            .unwrap_or(0);

        Self {
            options,
            selected_index,
            title,
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.options.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn get_selected(&self) -> String {
        self.options
            .get(self.selected_index)
            .cloned()
            .unwrap_or_default()
    }
}

pub fn render_dropdown(f: &mut Frame, dropdown: &Dropdown) {
    let area = f.size();
    let height = (dropdown.options.len() as u16 + 4).min(area.height - 4);
    let popup_area = centered_rect(50, height, area);

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(dropdown.title.clone())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let items: Vec<ListItem> = dropdown
        .options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            let style = if i == dropdown.selected_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(option.clone()).style(style)
        })
        .collect();

    let list = List::new(items).block(block);
    f.render_widget(list, popup_area);
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(height),
            Constraint::Min(1),
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
