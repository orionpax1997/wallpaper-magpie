use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

#[derive(Debug, Clone)]
pub enum LogEntry {
    Start { filename: String },
    Success { filename: String },
    Failure { filename: String, error: String },
}

impl LogEntry {
    pub fn to_string(&self) -> String {
        match self {
            LogEntry::Start { filename } => format!("\u{25B6} 开始下载: {}", filename),
            LogEntry::Success { filename } => format!("\u{2713} 完成: {}", filename),
            LogEntry::Failure { filename, error } => {
                format!("\u{2717} 失败: {} ({})", filename, error)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PageThree {
    pub total: usize,
    pub completed: usize,
    pub in_progress: usize,
    pub logs: Vec<LogEntry>,
    pub cancelled: bool,
    pub confirm_cancel: bool,
}

impl PageThree {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            in_progress: 0,
            logs: Vec::new(),
            cancelled: false,
            confirm_cancel: false,
        }
    }

    pub fn add_log(&mut self, entry: LogEntry) {
        match &entry {
            LogEntry::Start { .. } => self.in_progress += 1,
            LogEntry::Success { .. } => {
                self.in_progress = self.in_progress.saturating_sub(1);
                self.completed += 1;
            }
            LogEntry::Failure { .. } => {
                self.in_progress = self.in_progress.saturating_sub(1);
                self.completed += 1;
            }
        }
        self.logs.push(entry);
    }

    pub fn handle_esc(&mut self) {
        if self.confirm_cancel {
            self.cancelled = true;
        } else {
            self.confirm_cancel = true;
        }
    }

    pub fn dismiss_confirm(&mut self) {
        self.confirm_cancel = false;
    }
}

pub fn render_page_three(f: &mut Frame, page: &PageThree, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(area);

    let stats_text = format!(
        "总数: {}  已完成: {}  进行中: {}",
        page.total, page.completed, page.in_progress
    );
    let stats = Paragraph::new(stats_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("下载进度").borders(Borders::ALL));
    f.render_widget(stats, chunks[0]);

    let log_items: Vec<ListItem> = page
        .logs
        .iter()
        .map(|log| ListItem::new(log.to_string()))
        .collect();

    let log_list = List::new(log_items).block(Block::default().title("日志").borders(Borders::ALL));
    f.render_widget(log_list, chunks[1]);

    if page.confirm_cancel {
        let popup_area = super::modal::centered_rect(60, 20, area);
        let text = vec![
            Line::from("确定要取消下载吗？"),
            Line::from("已下载的文件将保留。"),
            Line::from(""),
            Line::from("[Esc] 确认取消  [其他键] 继续"),
        ];
        let paragraph = Paragraph::new(text).block(
            Block::default()
                .title("确认")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );
        f.render_widget(Clear, popup_area);
        f.render_widget(paragraph, popup_area);
    }
}
