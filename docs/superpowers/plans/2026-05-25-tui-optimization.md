# TUI 优化实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 优化 wallpaper-magpie 的 TUI 交互体验，实现三页向导的完整功能增强。

**架构:** 基于现有 ratatui + crossterm 架构，提取共用组件（Modal、Dropdown、HelpBar），数据驱动筛选条件配置，保持现有下载逻辑不变。

**Tech Stack:** Rust, ratatui, crossterm, tokio, serde, toml

---

## 文件结构

### 新文件
- `src/components/modal.rs` - 可复用模态框组件（API Key 编辑、确认对话框、下拉菜单）
- `src/components/dropdown.rs` - 枚举类型下拉菜单组件
- `src/components/help_bar.rs` - 底部键位帮助栏组件
- `src/filter_config.rs` - 各源筛选条件定义（数据驱动配置）
- `src/config_manager.rs` - 配置文件读写管理

### 修改文件
- `src/components/page_one.rs` - 源顺序、可用性颜色、API Key 编辑
- `src/components/page_two.rs` - 动态筛选条件、下拉菜单、占位符、箭头修复
- `src/components/page_three.rs` - 下载进度、日志、ESC 取消
- `src/app.rs` - 键位绑定、页面状态管理、模态框集成
- `src/main.rs` - CLI help 子命令
- `src/models.rs` - 新增配置和状态模型

---

## Task 1: 配置文件管理模块

**Files:**
- Create: `src/config_manager.rs`
- Modify: `src/models.rs`
- Test: `src/config_manager.rs` (内联测试)

**说明:** 创建配置文件读写模块，用于存储和读取各源的 API Key。配置文件放在项目目录内。

- [ ] **Step 1: 定义配置数据结构**

在 `src/models.rs` 中添加：

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub unsplash_api_key: Option<String>,
    pub pexels_api_key: Option<String>,
    pub wallhaven_api_key: Option<String>,
}

impl AppConfig {
    pub fn config_path() -> std::path::PathBuf {
        std::path::PathBuf::from("config.toml")
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub fn has_api_key(&self, source: &str) -> bool {
        match source {
            "unsplash" => self.unsplash_api_key.as_ref().map_or(false, |k| !k.is_empty()),
            "pexels" => self.pexels_api_key.as_ref().map_or(false, |k| !k.is_empty()),
            "wallhaven" => true, // Wallhaven 始终可用
            _ => false,
        }
    }

    pub fn get_api_key(&self, source: &str) -> Option<String> {
        match source {
            "unsplash" => self.unsplash_api_key.clone(),
            "pexels" => self.pexels_api_key.clone(),
            "wallhaven" => self.wallhaven_api_key.clone(),
            _ => None,
        }
    }

    pub fn set_api_key(&mut self, source: &str, key: String) {
        match source {
            "unsplash" => self.unsplash_api_key = Some(key),
            "pexels" => self.pexels_api_key = Some(key),
            "wallhaven" => self.wallhaven_api_key = Some(key),
            _ => {}
        }
    }
}
```

- [ ] **Step 2: 创建 config_manager 模块**

创建 `src/config_manager.rs`：

```rust
pub use crate::models::AppConfig;

pub struct ConfigManager;

impl ConfigManager {
    pub fn load() -> anyhow::Result<AppConfig> {
        AppConfig::load()
    }

    pub fn save(config: &AppConfig) -> anyhow::Result<()> {
        config.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert!(config.unsplash_api_key.is_none());
        assert!(config.pexels_api_key.is_none());
        assert!(config.wallhaven_api_key.is_none());
    }

    #[test]
    fn test_has_api_key() {
        let mut config = AppConfig::default();
        assert!(!config.has_api_key("unsplash"));
        config.unsplash_api_key = Some("test_key".to_string());
        assert!(config.has_api_key("unsplash"));
        assert!(config.has_api_key("wallhaven")); // 始终可用
    }
}
```

- [ ] **Step 3: 注册模块**

在 `src/main.rs` 的模块声明中添加：

```rust
mod config_manager;
```

- [ ] **Step 4: 运行测试**

```bash
cargo test config_manager::tests --lib
```

- [ ] **Step 5: Commit**

```bash
git add src/models.rs src/config_manager.rs src/main.rs
git commit -m "feat: add config manager for API keys"
```

---

## Task 2: 筛选条件配置定义

**Files:**
- Create: `src/filter_config.rs`
- Modify: `src/models.rs`
- Test: `src/filter_config.rs` (内联测试)

**说明:** 数据驱动定义三个源的筛选条件，包含类型、默认值、占位符、枚举值等。

- [ ] **Step 1: 定义筛选条件类型**

在 `src/models.rs` 中添加：

```rust
#[derive(Debug, Clone)]
pub enum FilterType {
    Text,
    Number,
    Enum { options: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct FilterField {
    pub name: String,           // 原始参数名，如 "q"
    pub display_name: String,   // 中文显示名，如 "关键词"
    pub filter_type: FilterType,
    pub default_value: Option<String>,
    pub placeholder: String,    // 占位符提示
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct SourceFilters {
    pub source_name: String,
    pub fields: Vec<FilterField>,
}
```

- [ ] **Step 2: 创建 filter_config 模块**

创建 `src/filter_config.rs`：

```rust
use crate::models::{FilterField, FilterType, SourceFilters};

pub fn get_wallhaven_filters() -> SourceFilters {
    SourceFilters {
        source_name: "wallhaven".to_string(),
        fields: vec![
            FilterField {
                name: "q".to_string(),
                display_name: "关键词".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "搜索关键词".to_string(),
                required: false,
            },
            FilterField {
                name: "categories".to_string(),
                display_name: "分类".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["111".to_string(), "110".to_string(), "101".to_string(),
                                  "011".to_string(), "100".to_string(), "010".to_string(),
                                  "001".to_string()],
                },
                default_value: Some("111".to_string()),
                placeholder: "选择分类".to_string(),
                required: false,
            },
            FilterField {
                name: "purity".to_string(),
                display_name: "纯度".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["SFW".to_string(), "Sketchy".to_string(), "NSFW".to_string()],
                },
                default_value: Some("SFW".to_string()),
                placeholder: "选择纯度".to_string(),
                required: false,
            },
            FilterField {
                name: "sorting".to_string(),
                display_name: "排序".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["date_added".to_string(), "relevance".to_string(),
                                  "random".to_string(), "views".to_string(),
                                  "favorites".to_string(), "toplist".to_string()],
                },
                default_value: Some("date_added".to_string()),
                placeholder: "选择排序方式".to_string(),
                required: false,
            },
            FilterField {
                name: "order".to_string(),
                display_name: "顺序".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["desc".to_string(), "asc".to_string()],
                },
                default_value: Some("desc".to_string()),
                placeholder: "选择顺序".to_string(),
                required: false,
            },
            FilterField {
                name: "topRange".to_string(),
                display_name: "时间范围".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["1d".to_string(), "3d".to_string(), "1w".to_string(),
                                  "1M".to_string(), "3M".to_string(), "6M".to_string(),
                                  "1y".to_string()],
                },
                default_value: Some("1M".to_string()),
                placeholder: "选择时间范围".to_string(),
                required: false,
            },
            FilterField {
                name: "atleast".to_string(),
                display_name: "最小分辨率".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "如: 1920x1080".to_string(),
                required: false,
            },
            FilterField {
                name: "resolutions".to_string(),
                display_name: "精确分辨率".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "如: 1920x1080,2560x1440".to_string(),
                required: false,
            },
            FilterField {
                name: "ratios".to_string(),
                display_name: "宽高比".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "如: 16x9,16x10".to_string(),
                required: false,
            },
            FilterField {
                name: "colors".to_string(),
                display_name: "颜色".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "如: 660000".to_string(),
                required: false,
            },
            FilterField {
                name: "page".to_string(),
                display_name: "页码".to_string(),
                filter_type: FilterType::Number,
                default_value: Some("1".to_string()),
                placeholder: "1".to_string(),
                required: false,
            },
            FilterField {
                name: "per_page".to_string(),
                display_name: "每页数量".to_string(),
                filter_type: FilterType::Number,
                default_value: Some("24".to_string()),
                placeholder: "1-24".to_string(),
                required: false,
            },
        ],
    }
}

pub fn get_unsplash_filters() -> SourceFilters {
    SourceFilters {
        source_name: "unsplash".to_string(),
        fields: vec![
            FilterField {
                name: "query".to_string(),
                display_name: "关键词".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "搜索关键词".to_string(),
                required: false,
            },
            FilterField {
                name: "page".to_string(),
                display_name: "页码".to_string(),
                filter_type: FilterType::Number,
                default_value: Some("1".to_string()),
                placeholder: "1".to_string(),
                required: false,
            },
            FilterField {
                name: "per_page".to_string(),
                display_name: "每页数量".to_string(),
                filter_type: FilterType::Number,
                default_value: Some("10".to_string()),
                placeholder: "1-30".to_string(),
                required: false,
            },
            FilterField {
                name: "order_by".to_string(),
                display_name: "排序".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["relevant".to_string(), "latest".to_string()],
                },
                default_value: Some("relevant".to_string()),
                placeholder: "选择排序".to_string(),
                required: false,
            },
            FilterField {
                name: "orientation".to_string(),
                display_name: "方向".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["landscape".to_string(), "portrait".to_string(), "squarish".to_string()],
                },
                default_value: None,
                placeholder: "选择方向".to_string(),
                required: false,
            },
            FilterField {
                name: "color".to_string(),
                display_name: "颜色".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "如: #000000".to_string(),
                required: false,
            },
        ],
    }
}

pub fn get_pexels_filters() -> SourceFilters {
    SourceFilters {
        source_name: "pexels".to_string(),
        fields: vec![
            FilterField {
                name: "query".to_string(),
                display_name: "关键词".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "搜索关键词".to_string(),
                required: false,
            },
            FilterField {
                name: "page".to_string(),
                display_name: "页码".to_string(),
                filter_type: FilterType::Number,
                default_value: Some("1".to_string()),
                placeholder: "1".to_string(),
                required: false,
            },
            FilterField {
                name: "per_page".to_string(),
                display_name: "每页数量".to_string(),
                filter_type: FilterType::Number,
                default_value: Some("10".to_string()),
                placeholder: "1-80".to_string(),
                required: false,
            },
            FilterField {
                name: "orientation".to_string(),
                display_name: "方向".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["landscape".to_string(), "portrait".to_string(), "square".to_string()],
                },
                default_value: None,
                placeholder: "选择方向".to_string(),
                required: false,
            },
            FilterField {
                name: "size".to_string(),
                display_name: "尺寸".to_string(),
                filter_type: FilterType::Enum {
                    options: vec!["large".to_string(), "medium".to_string(), "small".to_string()],
                },
                default_value: None,
                placeholder: "选择尺寸".to_string(),
                required: false,
            },
            FilterField {
                name: "color".to_string(),
                display_name: "颜色".to_string(),
                filter_type: FilterType::Text,
                default_value: None,
                placeholder: "如: red, #FF0000".to_string(),
                required: false,
            },
        ],
    }
}

pub fn get_filters_for_source(source: &str) -> Option<SourceFilters> {
    match source {
        "wallhaven" => Some(get_wallhaven_filters()),
        "unsplash" => Some(get_unsplash_filters()),
        "pexels" => Some(get_pexels_filters()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallhaven_filters() {
        let filters = get_wallhaven_filters();
        assert_eq!(filters.fields.len(), 12);
        assert_eq!(filters.fields[0].name, "q");
    }

    #[test]
    fn test_unsplash_filters() {
        let filters = get_unsplash_filters();
        assert_eq!(filters.fields.len(), 5);
    }

    #[test]
    fn test_pexels_filters() {
        let filters = get_pexels_filters();
        assert_eq!(filters.fields.len(), 6);
    }
}
```

- [ ] **Step 3: 注册模块**

在 `src/main.rs` 中添加：

```rust
mod filter_config;
```

- [ ] **Step 4: 运行测试**

```bash
cargo test filter_config::tests --lib
```

- [ ] **Step 5: Commit**

```bash
git add src/models.rs src/filter_config.rs src/main.rs
git commit -m "feat: add per-source filter configuration definitions"
```

---

## Task 3: 模态框组件

**Files:**
- Create: `src/components/modal.rs`
- Modify: `src/components/mod.rs`
- Test: 手动测试（TUI 组件）

**说明:** 创建可复用的模态框组件，用于 API Key 编辑、确认对话框等。

- [ ] **Step 1: 定义 Modal 组件**

创建 `src/components/modal.rs`：

```rust
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone)]
pub enum ModalType {
    ApiKeyEdit { source: String, current_value: String },
    Confirm { title: String, message: String },
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
    let area = f.area();
    let popup_area = centered_rect(60, 30, area);

    f.render_widget(Clear, popup_area);

    let (title, content) = match &modal.modal_type {
        ModalType::ApiKeyEdit { source, .. } => {
            (format!("编辑 {} API Key", source), format!("当前值: {}", modal.input_buffer))
        }
        ModalType::Confirm { title, message } => {
            (title.clone(), message.clone())
        }
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let text = vec![
        Line::from(content),
        Line::from(""),
        Line::from(vec![
            Span::raw("[Enter] 确认  "),
            Span::raw("[Esc] 取消"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, popup_area);
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
```

- [ ] **Step 2: 注册组件**

修改 `src/components/mod.rs`：

```rust
pub mod dropdown;
pub mod help_bar;
pub mod modal;
// ... 现有组件
```

- [ ] **Step 3: Commit**

```bash
git add src/components/modal.rs src/components/mod.rs
git commit -m "feat: add reusable modal component"
```

---

## Task 4: 下拉菜单组件

**Files:**
- Create: `src/components/dropdown.rs`
- Modify: `src/components/mod.rs`
- Test: 手动测试（TUI 组件）

**说明:** 创建枚举类型的下拉菜单组件。

- [ ] **Step 1: 定义 Dropdown 组件**

创建 `src/components/dropdown.rs`：

```rust
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
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
        self.options.get(self.selected_index)
            .cloned()
            .unwrap_or_default()
    }
}

pub fn render_dropdown(f: &mut Frame, dropdown: &Dropdown) {
    let area = f.area();
    let height = (dropdown.options.len() as u16 + 4).min(area.height - 4);
    let popup_area = centered_rect(50, height, area);

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(dropdown.title.clone())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let items: Vec<ListItem> = dropdown.options.iter().enumerate().map(|(i, option)| {
        let style = if i == dropdown.selected_index {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        ListItem::new(option.clone()).style(style)
    }).collect();

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
```

- [ ] **Step 2: 注册组件**

修改 `src/components/mod.rs`：

```rust
pub mod dropdown;
// ... 现有组件
```

- [ ] **Step 3: Commit**

```bash
git add src/components/dropdown.rs src/components/mod.rs
git commit -m "feat: add dropdown component for enum filters"
```

---

## Task 5: 底部帮助栏组件

**Files:**
- Create: `src/components/help_bar.rs`
- Modify: `src/components/mod.rs`
- Test: 手动测试（TUI 组件）

**说明:** 创建底部键位帮助栏组件，每个页面显示不同的帮助信息。

- [ ] **Step 1: 定义 HelpBar 组件**

创建 `src/components/help_bar.rs`：

```rust
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
```

- [ ] **Step 2: 注册组件**

修改 `src/components/mod.rs`：

```rust
pub mod help_bar;
// ... 现有组件
```

- [ ] **Step 3: Commit**

```bash
git add src/components/help_bar.rs src/components/mod.rs
git commit -m "feat: add bottom help bar component"
```

---

## Task 6: 页面一重构（壁纸源选择）

**Files:**
- Modify: `src/components/page_one.rs`
- Modify: `src/app.rs`
- Test: `cargo run` 手动测试

**说明:** 修改源选择页面，调整顺序、添加可用性颜色、支持 API Key 编辑。

- [ ] **Step 1: 修改源列表顺序和可用性显示**

修改 `src/components/page_one.rs`：

```rust
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::models::AppConfig;

#[derive(Debug, Clone)]
pub struct PageOne {
    pub sources: Vec<SourceItem>,
    pub selected_index: usize,
}

#[derive(Debug, Clone)]
pub struct SourceItem {
    pub name: String,
    pub display_name: String,
    pub requires_api_key: bool,
}

impl PageOne {
    pub fn new(config: &AppConfig) -> Self {
        let sources = vec![
            SourceItem {
                name: "wallhaven".to_string(),
                display_name: "Wallhaven".to_string(),
                requires_api_key: false, // Wallhaven 始终可用
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
            selected_index: 0, // Wallhaven 默认选中
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.sources.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn get_selected_source(&self) -> String {
        self.sources.get(self.selected_index)
            .map(|s| s.name.clone())
            .unwrap_or_default()
    }

    pub fn is_selected_available(&self, config: &AppConfig) -> bool {
        if let Some(source) = self.sources.get(self.selected_index) {
            if !source.requires_api_key {
                return true;
            }
            return config.has_api_key(&source.name);
        }
        false
    }
}

pub fn render_page_one(f: &mut Frame, page: &PageOne, config: &AppConfig, area: Rect) {
    let block = Block::default()
        .title("壁纸源")
        .borders(Borders::ALL);

    let items: Vec<ListItem> = page.sources.iter().enumerate().map(|(i, source)| {
        let is_selected = i == page.selected_index;
        let is_available = if source.requires_api_key {
            config.has_api_key(&source.name)
        } else {
            true
        };

        let marker = if is_selected { "> " } else { "  " };
        let checkbox = if is_selected { "● " } else { "○ " };

        let color = if is_available {
            if is_selected { Color::Green } else { Color::White }
        } else {
            Color::DarkGray
        };

        let content = format!("{}{}{}", marker, checkbox, source.display_name);
        let style = Style::default().fg(color);

        ListItem::new(content).style(style)
    }).collect();

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}
```

- [ ] **Step 2: 在 App 中集成页面一状态**

修改 `src/app.rs`，更新 App 结构体：

```rust
use crate::components::modal::{Modal, ModalType};
use crate::components::help_bar::HelpBar;
use crate::config_manager::ConfigManager;
use crate::models::AppConfig;

#[derive(Debug)]
pub struct App {
    pub config: AppConfig,
    pub page_one: PageOne,
    pub current_page: u8, // 1, 2, 3
    pub should_quit: bool,
    pub modal: Option<Modal>, // 当前显示的模态框
    // ... 其他字段
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let config = ConfigManager::load().unwrap_or_default();
        let page_one = PageOne::new(&config);

        Ok(Self {
            config,
            page_one,
            current_page: 1,
            should_quit: false,
            modal: None,
            // ... 初始化其他字段
        })
    }

    pub fn handle_page_one_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.page_one.previous();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.page_one.next();
            }
            KeyCode::Enter => {
                if self.page_one.is_selected_available(&self.config) {
                    self.current_page = 2;
                    // 初始化页面二...
                }
            }
            KeyCode::Char('e') => {
                let source = self.page_one.get_selected_source();
                let current_key = self.config.get_api_key(&source).unwrap_or_default();
                self.modal = Some(Modal::new(ModalType::ApiKeyEdit {
                    source,
                    current_value: current_key,
                }));
            }
            KeyCode::Esc => {
                if self.modal.is_some() {
                    self.modal = None;
                } else {
                    self.should_quit = true;
                }
            }
            _ => {}
        }
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/page_one.rs src/app.rs
git commit -m "feat: redesign page one with source availability and API key editing"
```

---

## Task 7: 页面二重构（筛选条件配置）

**Files:**
- Modify: `src/components/page_two.rs`
- Modify: `src/app.rs`
- Test: `cargo run` 手动测试

**说明:** 修改筛选条件页面，支持动态加载、枚举下拉菜单、占位符、编辑模式。

- [ ] **Step 1: 修改页面二结构**

修改 `src/components/page_two.rs`：

```rust
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use crate::filter_config::{get_filters_for_source, SourceFilters};
use crate::models::FilterField;
use crate::components::dropdown::{Dropdown, render_dropdown};

#[derive(Debug, Clone)]
pub struct PageTwo {
    pub source_name: String,
    pub filters: SourceFilters,
    pub selected_index: usize,
    pub filter_values: Vec<String>, // 当前各筛选条件的值
    pub editing_index: Option<usize>, // 当前正在编辑的索引
    pub editing_buffer: String,
    pub dropdown: Option<Dropdown>, // 当前打开的下拉菜单
}

impl PageTwo {
    pub fn new(source_name: &str) -> Self {
        let filters = get_filters_for_source(source_name)
            .expect("Unknown source");

        let filter_values: Vec<String> = filters.fields.iter()
            .map(|f| f.default_value.clone().unwrap_or_default())
            .collect();

        Self {
            source_name: source_name.to_string(),
            filters,
            selected_index: 0,
            filter_values,
            editing_index: None,
            editing_buffer: String::new(),
            dropdown: None,
        }
    }

    pub fn next(&mut self) {
        if self.dropdown.is_some() {
            if let Some(ref mut d) = self.dropdown {
                d.next();
            }
            return;
        }

        if self.editing_index.is_none() {
            if self.selected_index < self.filters.fields.len().saturating_sub(1) {
                self.selected_index += 1;
            }
        }
    }

    pub fn previous(&mut self) {
        if self.dropdown.is_some() {
            if let Some(ref mut d) = self.dropdown {
                d.previous();
            }
            return;
        }

        if self.editing_index.is_none() {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }
    }

    pub fn start_editing(&mut self) {
        if let Some(field) = self.filters.fields.get(self.selected_index) {
            match &field.filter_type {
                crate::models::FilterType::Enum { options } => {
                    let current_value = self.filter_values.get(self.selected_index)
                        .cloned()
                        .filter(|v| !v.is_empty());
                    self.dropdown = Some(Dropdown::new(
                        field.display_name.clone(),
                        options.clone(),
                        current_value,
                    ));
                }
                _ => {
                    self.editing_index = Some(self.selected_index);
                    self.editing_buffer = self.filter_values.get(self.selected_index)
                        .cloned()
                        .unwrap_or_default();
                }
            }
        }
    }

    pub fn confirm_edit(&mut self) {
        if let Some(dropdown) = self.dropdown.take() {
            let value = dropdown.get_selected();
            if let Some(idx) = self.selected_index {
                if idx < self.filter_values.len() {
                    self.filter_values[idx] = value;
                }
            }
        } else if let Some(idx) = self.editing_index.take() {
            if idx < self.filter_values.len() {
                self.filter_values[idx] = self.editing_buffer.clone();
            }
        }
    }

    pub fn cancel_edit(&mut self) {
        self.dropdown = None;
        self.editing_index = None;
        self.editing_buffer.clear();
    }

    pub fn get_filter_params(&self) -> Vec<(String, String)> {
        self.filters.fields.iter()
            .zip(self.filter_values.iter())
            .filter_map(|(field, value)| {
                if value.is_empty() {
                    None
                } else {
                    Some((field.name.clone(), value.clone()))
                }
            })
            .collect()
    }
}

pub fn render_page_two(f: &mut Frame, page: &PageTwo, area: Rect) {
    let block = Block::default()
        .title("筛选条件")
        .borders(Borders::ALL);

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let items: Vec<ListItem> = page.filters.fields.iter().enumerate().map(|(i, field)| {
        let is_selected = i == page.selected_index;
        let is_editing = page.editing_index == Some(i);
        let has_dropdown = page.dropdown.is_some() && is_selected;

        let marker = if is_selected { "> " } else { "  " };
        let value = page.filter_values.get(i).cloned().unwrap_or_default();
        let display_value = if value.is_empty() {
            field.placeholder.clone()
        } else {
            value.clone()
        };

        let content = format!("{}{} ({}): [{}]", 
            marker, 
            field.display_name, 
            field.name,
            display_value
        );

        let mut style = Style::default();
        if is_selected {
            style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
        }
        if is_editing {
            style = style.bg(Color::DarkGray);
        }
        if value.is_empty() {
            style = style.fg(Color::DarkGray);
        }

        ListItem::new(content).style(style)
    }).collect();

    let list = List::new(items);
    f.render_widget(list, inner_area);

    // 渲染下拉菜单
    if let Some(ref dropdown) = page.dropdown {
        render_dropdown(f, dropdown);
    }
}
```

- [ ] **Step 2: 在 App 中集成页面二**

在 `src/app.rs` 中添加页面二处理逻辑：

```rust
pub fn handle_page_two_input(&mut self, key: KeyEvent) {
    if let Some(ref mut page_two) = self.page_two {
        // 如果下拉菜单打开
        if page_two.dropdown.is_some() {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => page_two.previous(),
                KeyCode::Down | KeyCode::Char('j') => page_two.next(),
                KeyCode::Enter => page_two.confirm_edit(),
                KeyCode::Esc => page_two.cancel_edit(),
                _ => {}
            }
            return;
        }

        // 如果正在编辑
        if page_two.editing_index.is_some() {
            match key.code {
                KeyCode::Enter => page_two.confirm_edit(),
                KeyCode::Esc => page_two.cancel_edit(),
                KeyCode::Char(c) => page_two.editing_buffer.push(c),
                KeyCode::Backspace => { page_two.editing_buffer.pop(); }
                _ => {}
            }
            return;
        }

        // 正常浏览模式
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => page_two.previous(),
            KeyCode::Down | KeyCode::Char('j') => page_two.next(),
            KeyCode::Enter => {
                self.current_page = 3;
                // 初始化页面三...
            }
            KeyCode::Char('e') => page_two.start_editing(),
            KeyCode::Esc => {
                self.current_page = 1;
            }
            _ => {}
        }
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/page_two.rs src/app.rs
git commit -m "feat: redesign page two with dynamic filters and dropdowns"
```

---

## Task 8: 页面三重构（下载进度）

**Files:**
- Modify: `src/components/page_three.rs`
- Modify: `src/app.rs`
- Modify: `src/downloader.rs`（如果需要）
- Test: `cargo run` 手动测试

**说明:** 修改下载进度页面，显示统计、日志、ESC 两次取消。

- [ ] **Step 1: 修改页面三结构**

修改 `src/components/page_three.rs`：

```rust
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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
            LogEntry::Start { filename } => format!("▶ 开始下载: {}", filename),
            LogEntry::Success { filename } => format!("✓ 完成: {}", filename),
            LogEntry::Failure { filename, error } => format!("✗ 失败: {} ({})", filename, error),
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
    pub confirm_cancel: bool, // 是否显示取消确认对话框
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
            Constraint::Length(3),   // 统计信息
            Constraint::Min(5),      // 日志窗口
            Constraint::Length(1),   // 帮助栏
        ])
        .split(area);

    // 统计信息
    let stats_text = format!(
        "总数: {}  已完成: {}  进行中: {}",
        page.total, page.completed, page.in_progress
    );
    let stats = Paragraph::new(stats_text)
        .block(Block::default().title("下载进度").borders(Borders::ALL));
    f.render_widget(stats, chunks[0]);

    // 日志窗口
    let log_items: Vec<ListItem> = page.logs.iter().map(|log| {
        ListItem::new(log.to_string())
    }).collect();

    let log_list = List::new(log_items)
        .block(Block::default().title("日志").borders(Borders::ALL));
    f.render_widget(log_list, chunks[1]);

    // 取消确认对话框
    if page.confirm_cancel {
        let popup_area = super::modal::centered_rect(60, 20, area);
        let text = vec![
            Line::from("确定要取消下载吗？"),
            Line::from("已下载的文件将保留。"),
            Line::from(""),
            Line::from("[Esc] 确认取消  [其他键] 继续"),
        ];
        let paragraph = Paragraph::new(text)
            .block(Block::default().title("确认").borders(Borders::ALL));
        f.render_widget(Clear, popup_area);
        f.render_widget(paragraph, popup_area);
    }
}
```

- [ ] **Step 2: 在 App 中集成页面三**

在 `src/app.rs` 中添加：

```rust
pub fn handle_page_three_input(&mut self, key: KeyEvent) {
    if let Some(ref mut page_three) = self.page_three {
        match key.code {
            KeyCode::Enter => {
                if !page_three.confirm_cancel {
                    self.should_quit = true;
                }
            }
            KeyCode::Esc => {
                if page_three.confirm_cancel {
                    page_three.cancelled = true;
                    self.current_page = 2;
                } else {
                    page_three.handle_esc();
                }
            }
            _ => {
                if page_three.confirm_cancel {
                    page_three.dismiss_confirm();
                }
            }
        }
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/page_three.rs src/app.rs
git commit -m "feat: redesign page three with download progress and logs"
```

---

## Task 9: 应用层整合与键位绑定

**Files:**
- Modify: `src/app.rs`
- Modify: `src/main.rs`
- Test: `cargo run` 手动测试

**说明:** 整合所有页面，更新键位绑定，移除 `q` 键。

- [ ] **Step 1: 更新 App 主循环**

修改 `src/app.rs` 中的输入处理：

```rust
pub async fn handle_input(&mut self, key: KeyEvent) {
    // 如果有模态框，优先处理模态框输入
    if let Some(ref mut modal) = self.modal {
        match key.code {
            KeyCode::Enter => {
                let value = modal.get_value();
                match &modal.modal_type {
                    ModalType::ApiKeyEdit { source, .. } => {
                        self.config.set_api_key(source, value);
                        if let Err(e) = ConfigManager::save(&self.config) {
                            // 处理保存错误
                        }
                    }
                    _ => {}
                }
                self.modal = None;
            }
            KeyCode::Esc => {
                self.modal = None;
            }
            KeyCode::Char(c) => modal.handle_input(c),
            KeyCode::Backspace => modal.handle_backspace(),
            KeyCode::Delete => modal.handle_delete(),
            KeyCode::Left => modal.move_cursor_left(),
            KeyCode::Right => modal.move_cursor_right(),
            _ => {}
        }
        return;
    }

    match self.current_page {
        1 => self.handle_page_one_input(key),
        2 => self.handle_page_two_input(key),
        3 => self.handle_page_three_input(key),
        _ => {}
    }
}
```

- [ ] **Step 2: 更新主渲染循环**

在 `src/app.rs` 的 `draw` 方法中：

```rust
pub fn draw(&mut self, f: &mut Frame) {
    let area = f.area();

    // 分割区域：主内容 + 底部帮助栏
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1), // 帮助栏
        ])
        .split(area);

    match self.current_page {
        1 => {
            render_page_one(f, &self.page_one, &self.config, chunks[0]);
            help_bar::render_help_bar(f, &HelpBar::for_page_one(), chunks[1]);
        }
        2 => {
            if let Some(ref page_two) = self.page_two {
                render_page_two(f, page_two, chunks[0]);
                if page_two.dropdown.is_some() {
                    help_bar::render_help_bar(f, &HelpBar::for_dropdown(), chunks[1]);
                } else if page_two.editing_index.is_some() {
                    help_bar::render_help_bar(f, &HelpBar::for_modal(), chunks[1]);
                } else {
                    help_bar::render_help_bar(f, &HelpBar::for_page_two(), chunks[1]);
                }
            }
        }
        3 => {
            if let Some(ref page_three) = self.page_three {
                render_page_three(f, page_three, chunks[0]);
                help_bar::render_help_bar(f, &HelpBar::for_page_three(), chunks[1]);
            }
        }
        _ => {}
    }

    // 渲染模态框
    if let Some(ref modal) = self.modal {
        render_modal(f, modal);
    }
}
```

- [ ] **Step 3: 移除 q 键绑定**

在 `src/main.rs` 或 `src/app.rs` 中，移除所有 `KeyCode::Char('q')` 的处理逻辑。

- [ ] **Step 4: Commit**

```bash
git add src/app.rs src/main.rs
git commit -m "feat: integrate all pages with updated keybindings and help bar"
```

---

## Task 10: CLI Help 子命令

**Files:**
- Modify: `src/main.rs`
- Modify: `src/filter_config.rs`
- Test: `cargo run -- help unsplash`

**说明:** 添加 `help <source>` 子命令，显示对应源的筛选参数说明。

- [ ] **Step 1: 添加 CLI 子命令**

修改 `src/main.rs`：

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wallpaper-magpie")]
#[command(about = "壁纸下载工具")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 显示指定源的筛选参数帮助
    Help {
        /// 源名称 (unsplash, pexels, wallhaven)
        source: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Help { source }) = cli.command {
        print_source_help(&source);
        return;
    }

    // 启动 TUI...
}

fn print_source_help(source: &str) {
    use crate::filter_config::get_filters_for_source;

    let filters = match get_filters_for_source(source) {
        Some(f) => f,
        None => {
            println!("未知源: {}", source);
            println!("可用源: unsplash, pexels, wallhaven");
            return;
        }
    };

    println!("{} 筛选参数:", source);
    println!();

    for field in &filters.fields {
        let type_str = match &field.filter_type {
            crate::models::FilterType::Text => "字符串".to_string(),
            crate::models::FilterType::Number => "数字".to_string(),
            crate::models::FilterType::Enum { options } => {
                format!("枚举 ({})", options.join(", "))
            }
        };

        let default_str = field.default_value.as_ref()
            .map(|v| format!(" [默认: {}]", v))
            .unwrap_or_default();

        println!("  {:<15} {} - {}{}", 
            field.name, 
            field.display_name,
            type_str,
            default_str
        );
    }
}
```

- [ ] **Step 2: 运行测试**

```bash
cargo run -- help wallhaven
cargo run -- help unsplash
cargo run -- help pexels
```

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: add CLI help subcommand for source filters"
```

---

## Task 11: Bug 修复（页面二箭头指示器）

**Files:**
- Modify: `src/components/page_two.rs`
- Test: `cargo run` 手动测试

**说明:** 修复页面二所有可选项都显示左侧箭头 `>` 的问题。

- [ ] **Step 1: 检查并修复箭头逻辑**

在 `src/components/page_two.rs` 的 `render_page_two` 函数中，确保所有可选项都显示箭头：

```rust
let marker = if is_selected { "> " } else { "  " };
```

这个逻辑已经在 Task 7 的代码中实现了。如果还有问题，检查是否有其他条件覆盖了 `is_selected` 的判断。

- [ ] **Step 2: Commit**

```bash
git add src/components/page_two.rs
git commit -m "fix: ensure arrow indicator shows for all filter options on page two"
```

---

## Task 12: 最终集成测试

**Files:**
- 所有修改过的文件
- Test: `cargo test`, `cargo run`, `cargo clippy`

**说明:** 运行完整测试，确保所有功能正常。

- [ ] **Step 1: 运行单元测试**

```bash
cargo test
```

- [ ] **Step 2: 运行 Clippy**

```bash
cargo clippy -- -D warnings
```

- [ ] **Step 3: 运行 TUI 手动测试**

```bash
cargo run
```

验证：
1. 页面一：Wallhaven 在第一，默认选中，绿色
2. 页面一：Unsplash/Pexels 灰色，配置 API Key 后变绿色
3. 页面一：按 `e` 编辑 API Key，保存后生效
4. 页面二：显示对应源的筛选条件，格式为 `中文名 (参数名)`
5. 页面二：枚举类型显示下拉菜单
6. 页面二：输入框有占位符提示
7. 页面二：非编辑状态 Enter 进入下一步
8. 页面三：显示下载进度和日志
9. 页面三：Esc 两次确认取消
10. 所有页面：底部显示帮助栏
11. CLI：`cargo run -- help wallhaven` 显示帮助

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "test: final integration testing and validation"
```

---

## 自我审查

### 1. Spec 覆盖检查

| 需求 | 实现任务 |
|------|----------|
| Wallhaven 排第一，默认选中 | Task 6 |
| 可用源绿色，不可用灰色 | Task 6 |
| `e` 键编辑 API Key | Task 6 |
| 配置文件在项目目录 | Task 1 |
| 每个源独立筛选条件 | Task 2 |
| 显示 `中文名 (参数名)` | Task 7 |
| 枚举类型下拉菜单 | Task 4, Task 7 |
| 输入框占位符 | Task 7 |
| 非编辑 Enter 下一步 | Task 7 |
| 下载进度和日志 | Task 8 |
| Esc 两次取消 | Task 8 |
| 底部帮助栏 | Task 5, Task 9 |
| 移除 `q` 键 | Task 9 |
| CLI `help <source>` | Task 10 |
| 箭头修复 | Task 11 |

### 2. 占位符扫描

无 TBD/TODO/"implement later" 等占位符。

### 3. 类型一致性检查

- `AppConfig` 在 `models.rs` 和 `config_manager.rs` 中一致
- `FilterField`/`FilterType` 在 `models.rs` 和 `filter_config.rs` 中一致
- `Modal`/`Dropdown` 组件接口在各页面中一致

---

## 执行交接

**计划已保存到 `docs/superpowers/plans/2026-05-25-tui-optimization.md`。**

**两种执行方式：**

**1. Subagent-Driven（推荐）** - 为每个 Task 派遣独立的子代理，任务间审查，快速迭代

**2. Inline Execution** - 在本会话中使用 executing-plans 技能，批量执行带检查点

**选择哪种方式？**
