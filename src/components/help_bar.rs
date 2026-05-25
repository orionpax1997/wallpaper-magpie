use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

#[derive(Debug, Clone)]
pub struct HelpBar {
    pub items: Vec<String>,
}

impl HelpBar {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn for_page_one() -> Self {
        Self::new(vec![
            "[↑/↓] 切换".to_string(),
            "[Enter] 下一步".to_string(),
            "[Esc] 退出".to_string(),
            "[e] 编辑 API Key".to_string(),
        ])
    }

    pub fn for_page_two() -> Self {
        Self::new(vec![
            "[↑/↓] 切换".to_string(),
            "[Enter] 确认/下一步".to_string(),
            "[Esc] 上一步".to_string(),
            "[e] 编辑值".to_string(),
        ])
    }

    pub fn for_page_three() -> Self {
        Self::new(vec![
            "[Enter] 完成".to_string(),
            "[Esc] 取消".to_string(),
        ])
    }

    pub fn for_modal() -> Self {
        Self::new(vec![
            "[Enter] 确认".to_string(),
            "[Esc] 取消".to_string(),
        ])
    }

    pub fn for_dropdown() -> Self {
        Self::new(vec![
            "[↑/↓] 选择".to_string(),
            "[Enter] 确认".to_string(),
            "[Esc] 取消".to_string(),
        ])
    }
}

pub fn render_help_bar(f: &mut Frame, help_bar: &HelpBar, area: Rect) {
    let spans: Vec<Span> = help_bar.items.iter().map(|item| {
        Span::styled(item.clone(), Style::default().fg(Color::Gray))
    }).collect();

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    f.render_widget(paragraph, area);
}
