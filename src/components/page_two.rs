use crate::components::dropdown::{render_dropdown, Dropdown};
use crate::filter_config::get_filters_for_source;
use crate::models::{FilterFieldType, SourceFilters};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug, Clone)]
pub struct PageTwo {
    pub source_name: String,
    pub filters: SourceFilters,
    pub selected_index: usize,
    pub filter_values: Vec<String>,
    pub editing_index: Option<usize>,
    pub editing_buffer: String,
    pub dropdown: Option<Dropdown>,
}

impl PageTwo {
    pub fn new(source_name: &str) -> Option<Self> {
        let filters = get_filters_for_source(source_name)?;
        let filter_values: Vec<String> = filters
            .fields
            .iter()
            .map(|f| f.default_value.clone().unwrap_or_default())
            .collect();

        Some(Self {
            source_name: source_name.to_string(),
            filters,
            selected_index: 0,
            filter_values,
            editing_index: None,
            editing_buffer: String::new(),
            dropdown: None,
        })
    }

    pub fn next(&mut self) {
        if self.selected_index < self.filters.fields.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn start_editing(&mut self) {
        let field = match self.filters.fields.get(self.selected_index) {
            Some(f) => f,
            None => return,
        };
        self.editing_index = Some(self.selected_index);
        self.editing_buffer = self
            .filter_values
            .get(self.selected_index)
            .cloned()
            .unwrap_or_default();

        match &field.filter_type {
            FilterFieldType::Enum { options } => {
                let current_value = self.filter_values.get(self.selected_index).cloned();
                self.dropdown = Some(Dropdown::new(
                    field.display_name.clone(),
                    options.clone(),
                    current_value,
                ));
            }
            _ => {}
        }
    }

    pub fn confirm_edit(&mut self) {
        if let Some(ref mut dropdown) = self.dropdown {
            if let Some(idx) = self.editing_index {
                self.filter_values[idx] = dropdown.get_selected();
            }
        } else if let Some(idx) = self.editing_index {
            self.filter_values[idx] = self.editing_buffer.clone();
        }
        self.editing_index = None;
        self.editing_buffer.clear();
        self.dropdown = None;
    }

    pub fn cancel_edit(&mut self) {
        self.editing_index = None;
        self.editing_buffer.clear();
        self.dropdown = None;
    }

    pub fn get_filter_params(&self) -> Vec<(String, String)> {
        self.filters
            .fields
            .iter()
            .zip(self.filter_values.iter())
            .map(|(field, value)| (field.name.clone(), value.clone()))
            .collect()
    }
}

pub fn render_page_two(f: &mut Frame, page: &PageTwo, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new(format!("步骤 2/3: 配置筛选条件 ({})", page.source_name))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);

    let filter_lines: Vec<Line> = page
        .filters
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let is_selected = i == page.selected_index;
            let is_editing = page.editing_index == Some(i);
            let value = page.filter_values.get(i).cloned().unwrap_or_default();
            let display_value = if is_editing && page.dropdown.is_none() {
                page.editing_buffer.clone()
            } else if value.is_empty() {
                "(未设置)".to_string()
            } else {
                value
            };

            let prefix = if is_selected { "▸ " } else { "  " };
            let value_color = if is_selected {
                Color::Yellow
            } else {
                Color::Gray
            };

            Line::from(vec![
                Span::styled(
                    format!("{}{}. {}: ", prefix, i + 1, field.display_name),
                    Style::default().fg(Color::White),
                ),
                Span::styled(display_value, Style::default().fg(value_color)),
            ])
        })
        .collect();

    let filters_widget = Paragraph::new(filter_lines)
        .block(Block::default().borders(Borders::ALL).title("筛选条件"));
    f.render_widget(filters_widget, chunks[1]);

    let help = Paragraph::new("↑↓: 切换 | Enter: 确认/下一步 | Esc: 上一步 | e: 编辑")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);

    if let Some(ref dropdown) = page.dropdown {
        render_dropdown(f, dropdown);
    }
}
