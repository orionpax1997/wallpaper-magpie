use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::models::AppConfig;

#[derive(Debug, Clone)]
pub struct SourceItem {
    pub name: String,
    pub display_name: String,
    pub requires_api_key: bool,
}

#[derive(Debug, Clone)]
pub struct PageOne {
    pub sources: Vec<SourceItem>,
    pub selected_index: usize,
}

impl PageOne {
    pub fn new(_config: &AppConfig) -> Self {
        let sources = vec![
            SourceItem {
                name: "wallhaven".to_string(),
                display_name: "Wallhaven".to_string(),
                requires_api_key: false,
            },
            SourceItem {
                name: "unsplash".to_string(),
                display_name: "Unsplash".to_string(),
                requires_api_key: true,
            },
            SourceItem {
                name: "pexels".to_string(),
                display_name: "Pexels".to_string(),
                requires_api_key: true,
            },
        ];

        Self {
            sources,
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.sources.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn get_selected_source(&self) -> Option<&SourceItem> {
        self.sources.get(self.selected_index)
    }

    pub fn is_selected_available(&self, config: &AppConfig) -> bool {
        if let Some(source) = self.get_selected_source() {
            if source.requires_api_key {
                config.has_api_key(&source.name)
            } else {
                true
            }
        } else {
            false
        }
    }
}

pub fn render_page_one(f: &mut Frame, page: &PageOne, config: &AppConfig, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new("步骤 1/3: 选择壁纸来源")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);

    let sources_text: Vec<Line> = page
        .sources
        .iter()
        .enumerate()
        .map(|(i, source)| {
            let is_selected = i == page.selected_index;
            let is_available = if source.requires_api_key {
                config.has_api_key(&source.name)
            } else {
                true
            };

            let marker = if is_selected { "> " } else { "  " };
            let bullet = if is_available { "● " } else { "○ " };

            let color = if is_available {
                Color::Green
            } else {
                Color::Gray
            };

            let prefix = format!("{}{}", marker, bullet);
            let line = format!(
                "{}{}. {} {}",
                prefix,
                i + 1,
                source.display_name,
                if source.requires_api_key {
                    if is_available {
                        "(已配置)"
                    } else {
                        "(需要API Key)"
                    }
                } else {
                    ""
                }
            );

            Line::from(Span::styled(line, Style::default().fg(color)))
        })
        .collect();

    let sources = Paragraph::new(sources_text)
        .block(Block::default().borders(Borders::ALL).title("可用来源"));
    f.render_widget(sources, chunks[1]);

    let help = Paragraph::new("↑↓/j/k: 切换 | Enter: 选择 | e: 编辑API Key | Esc: 退出")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}
