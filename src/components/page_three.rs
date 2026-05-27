use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use std::fmt;

#[derive(Debug, Clone)]
pub enum LogEntry {
    Start { filename: String },
    Success { filename: String },
    Failure { filename: String, error: String },
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogEntry::Start { filename } => write!(f, "\u{25B6} 开始下载: {}", filename),
            LogEntry::Success { filename } => write!(f, "\u{2713} 完成: {}", filename),
            LogEntry::Failure { filename, error } => {
                write!(f, "\u{2717} 失败: {} ({})", filename, error)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PageThree {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub pending: usize,
    pub is_preparing: bool,
    pub is_downloading: bool,
    pub logs: Vec<LogEntry>,
    pub cancelled: bool,
    pub confirm_cancel: bool,
    pub list_state: ListState,
    pub scrollbar_state: ScrollbarState,
}

impl PageThree {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            failed: 0,
            in_progress: 0,
            pending: 0,
            is_preparing: false,
            is_downloading: false,
            logs: Vec::new(),
            cancelled: false,
            confirm_cancel: false,
            list_state: ListState::default(),
            scrollbar_state: ScrollbarState::default(),
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
                self.failed += 1;
            }
        }
        self.logs.push(entry);
        self.scroll_to_bottom();
    }

    pub fn scroll_to_bottom(&mut self) {
        if self.logs.is_empty() {
            return;
        }
        *self.list_state.offset_mut() = self.logs.len().saturating_sub(1);
    }

    pub fn scroll_up(&mut self) {
        let new_offset = self.list_state.offset().saturating_sub(1);
        *self.list_state.offset_mut() = new_offset;
    }

    pub fn scroll_down(&mut self) {
        *self.list_state.offset_mut() = self.list_state.offset() + 1;
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

pub fn render_page_three(f: &mut Frame, page: &mut PageThree, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(1),
        ])
        .split(area);

    let completed = page
        .logs
        .iter()
        .filter(|l| matches!(l, LogEntry::Success { .. }))
        .count();
    let failed = page
        .logs
        .iter()
        .filter(|l| matches!(l, LogEntry::Failure { .. }))
        .count();
    let in_progress = page
        .logs
        .iter()
        .filter(|l| matches!(l, LogEntry::Start { .. }))
        .count()
        .saturating_sub(completed + failed);

    let stats_text = if page.is_preparing {
        format!("总数: {}  准备中: {} ...", page.total, page.total)
    } else {
        format!(
            "{}总数: {}  已完成: {}  进行中: {}  待处理: {}",
            if page.in_progress > 0 { "正在下载... " } else { "" },
            page.total,
            completed,
            in_progress,
            page.pending
        )
    };
    let stats = Paragraph::new(stats_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("下载进度").borders(Borders::ALL));
    f.render_widget(stats, chunks[0]);

    let log_items: Vec<ListItem> = if page.is_preparing {
        (0..page.total)
            .map(|i| ListItem::new(format!("\u{25B6} 正在准备下载 {} ...", i + 1)))
            .collect()
    } else {
        page.logs
            .iter()
            .map(|log| ListItem::new(log.to_string()))
            .collect()
    };

    let log_height = chunks[1].height as usize;
    let content_len = if page.is_preparing { page.total } else { page.logs.len() };
    let max_offset = content_len.saturating_sub(log_height);

    if page.list_state.offset() > max_offset {
        *page.list_state.offset_mut() = max_offset;
    }

    let log_list = List::new(log_items)
        .block(Block::default().title("日志").borders(Borders::ALL));
    f.render_stateful_widget(log_list, chunks[1], &mut page.list_state);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .thumb_style(Style::default().fg(Color::White));
    page.scrollbar_state = page.scrollbar_state
        .content_length(content_len)
        .position(page.list_state.offset())
        .viewport_content_length(log_height);
    f.render_stateful_widget(scrollbar, chunks[1], &mut page.scrollbar_state);

    if !page.confirm_cancel {
        let help_text = if page.is_downloading {
            "正在下载中... | [Esc] 取消 | [↑↓] 滚动日志"
        } else if page.is_preparing {
            "准备中..."
        } else {
            "[Enter] 确认下载 | [Esc] 取消 | [↑↓] 滚动日志"
        };
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[2]);
    }

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
