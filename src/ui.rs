use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
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
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);

    let sources_text = app
        .available_sources
        .iter()
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

fn draw_configure_filters(f: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new(format!(
        "步骤 2/3: 配置筛选条件 ({})",
        app.selected_source.as_deref().unwrap_or("unknown")
    ))
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);

    let filters = [
        ("query", "关键词", app.search_params.query.clone()),
        (
            "resolution",
            "分辨率",
            app.search_params.resolution.clone().unwrap_or_default(),
        ),
        (
            "color",
            "颜色",
            app.search_params.color.clone().unwrap_or_default(),
        ),
        (
            "orientation",
            "方向",
            app.search_params.orientation.clone().unwrap_or_default(),
        ),
        (
            "sort",
            "排序",
            app.search_params
                .sort
                .map(|s| s.as_str().to_string())
                .unwrap_or_default(),
        ),
        ("limit", "数量", app.search_params.limit.to_string()),
    ];

    let mut filter_text: Vec<Line> = filters
        .iter()
        .enumerate()
        .map(|(i, (key, label, value))| {
            let editing = app.editing_filter.as_ref() == Some(&key.to_string());
            let prefix = if editing { "▸ " } else { "  " };
            let display_value = if editing {
                app.edit_buffer.clone()
            } else if value.is_empty() {
                "(未设置)".to_string()
            } else {
                value.clone()
            };

            Line::from(vec![
                Span::styled(
                    format!("{}{}. {}: ", prefix, i + 1, label),
                    Style::default().fg(Color::White),
                ),
                Span::styled(display_value, Style::default().fg(Color::Yellow)),
            ])
        })
        .collect();

    let is_next_step = app.current_filter_index == 6;
    let next_prefix = if is_next_step { "▸ " } else { "  " };
    filter_text.push(Line::from(vec![Span::styled(
        format!("{}7. 下一步 →", next_prefix),
        if is_next_step {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Green)
        },
    )]));

    let filters_widget =
        Paragraph::new(filter_text).block(Block::default().borders(Borders::ALL).title("筛选条件"));
    f.render_widget(filters_widget, chunks[1]);

    let help = Paragraph::new("↑↓/Tab: 切换 | Enter/e: 编辑/下一步 | Esc: 取消 | q: 退出")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_confirm(f: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new("步骤 3/3: 确认并下载")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);

    let confirm_text = vec![
        Line::from(vec![
            Span::styled("来源: ", Style::default().fg(Color::White)),
            Span::styled(
                app.selected_source.as_deref().unwrap_or("unknown"),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("关键词: ", Style::default().fg(Color::White)),
            Span::styled(&app.search_params.query, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("分辨率: ", Style::default().fg(Color::White)),
            Span::styled(
                app.search_params.resolution.as_deref().unwrap_or("不限"),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("颜色: ", Style::default().fg(Color::White)),
            Span::styled(
                app.search_params.color.as_deref().unwrap_or("不限"),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("方向: ", Style::default().fg(Color::White)),
            Span::styled(
                app.search_params.orientation.as_deref().unwrap_or("不限"),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("排序: ", Style::default().fg(Color::White)),
            Span::styled(
                app.search_params.sort.map(|s| s.as_str()).unwrap_or("默认"),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("数量: ", Style::default().fg(Color::White)),
            Span::styled(
                app.search_params.limit.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let confirm_widget = Paragraph::new(confirm_text)
        .block(Block::default().borders(Borders::ALL).title("下载确认"));
    f.render_widget(confirm_widget, chunks[1]);

    let help = Paragraph::new("Enter: 确认下载 | Esc: 返回 | q: 退出")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_downloading(f: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new("正在下载...")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);

    let mut text = vec![];

    if let Some(ref progress) = app.download_progress {
        let percent = if progress.total > 0 {
            (progress.completed as f64 / progress.total as f64) * 100.0
        } else {
            0.0
        };

        text.push(Line::from(format!(
            "进度: {}/{} ({:.1}%)",
            progress.completed, progress.total, percent
        )));

        if let Some(ref file) = progress.current_file {
            text.push(Line::from(format!("当前: {}", file)));
        }

        let filled = (percent / 100.0 * 50.0) as usize;
        let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(50 - filled));
        text.push(Line::from(bar));
    } else {
        text.push(Line::from("准备下载..."));
    }

    let progress_widget =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("下载进度"));
    f.render_widget(progress_widget, chunks[1]);

    let help = Paragraph::new("请稍候...")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_completed(f: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new("下载完成！")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, chunks[0]);

    let mut text = vec![];
    text.push(Line::from("下载结果:"));
    text.push(Line::from(""));

    for (filename, success) in &app.download_results {
        let icon = if *success { "✓" } else { "✗" };
        let color = if *success { Color::Green } else { Color::Red };
        text.push(Line::from(vec![
            Span::styled(format!("{} ", icon), Style::default().fg(color)),
            Span::styled(filename, Style::default().fg(Color::White)),
        ]));
    }

    let results_widget =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("结果"));
    f.render_widget(results_widget, chunks[1]);

    let help = Paragraph::new("Enter: 返回主菜单 | q: 退出")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
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
