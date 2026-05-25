use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone)]
pub enum ModalType {
    ApiKeyEdit {
        source: String,
        current_value: String,
    },
    Confirm {
        title: String,
        message: String,
    },
}

#[derive(Debug, Clone)]
pub struct Modal {
    pub modal_type: ModalType,
    pub input_buffer: String,
    pub cursor_position: usize,
}

impl Modal {
    pub fn new(modal_type: ModalType) -> Self {
        let input_buffer = match &modal_type {
            ModalType::ApiKeyEdit { current_value, .. } => current_value.clone(),
            _ => String::new(),
        };
        Self {
            modal_type,
            input_buffer,
            cursor_position: 0,
        }
    }

    pub fn handle_input(&mut self, c: char) {
        self.input_buffer.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input_buffer.remove(self.cursor_position);
        }
    }

    pub fn handle_delete(&mut self) {
        if self.cursor_position < self.input_buffer.len() {
            self.input_buffer.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input_buffer.len() {
            self.cursor_position += 1;
        }
    }

    pub fn get_value(&self) -> String {
        self.input_buffer.clone()
    }
}

pub fn render_modal(f: &mut Frame, modal: &Modal) {
    let area = f.size();
    let popup_area = centered_rect(60, 30, area);

    f.render_widget(Clear, popup_area);

    let (title, content) = match &modal.modal_type {
        ModalType::ApiKeyEdit { source, .. } => (
            format!("编辑 {} API Key", source),
            format!("当前值: {}", modal.input_buffer),
        ),
        ModalType::Confirm { title, message } => (title.clone(), message.clone()),
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let text = vec![
        Line::from(content),
        Line::from(""),
        Line::from(vec![Span::raw("[Enter] 确认  "), Span::raw("[Esc] 取消")]),
    ];

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, popup_area);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
