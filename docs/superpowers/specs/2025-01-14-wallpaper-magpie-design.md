# wallpaper-magpie 设计文档

## 1. 项目概述

**项目名称**: wallpaper-magpie（壁纸喜鹊）

**用途**: CLI + TUI 壁纸采集工具，支持从多个壁纸网站批量搜索、筛选和下载壁纸。

**技术栈**: Rust
- CLI: clap (v4, derive macros)
- TUI: ratatui
- HTTP: reqwest (async, tokio-based)
- Config: config + serde + toml
- Async runtime: tokio
- Error handling: anyhow + thiserror
- Progress bar: indicatif
- JSON processing: serde_json
- Terminal colors: owo-colors

## 2. 核心功能

### 2.1 多源壁纸支持
- **Unsplash**: 通过 API 搜索和下载，需要 Access Key
- **Pexels**: 通过 API 搜索和下载，需要 API Key
- **Wallhaven**: 通过 API 搜索和下载，API Key 可选（用于 NSFW 和个性化设置）
- 可扩展的 Provider trait，方便添加新来源

### 2.2 搜索与筛选

**注意**: 不同来源的 API 支持不同的筛选参数，TUI 界面需要根据所选来源动态显示可用的筛选选项。

#### Unsplash 搜索参数
- **Endpoint**: `GET /search/photos`
- **query**: 搜索关键词（必需）
- **page/per_page**: 分页，per_page 默认 10，最大 30
- **order_by**: `latest` | `relevant`（默认）
- **orientation**: `landscape` | `portrait` | `squarish`
- **color**: `black_and_white`, `black`, `white`, `yellow`, `orange`, `red`, `purple`, `magenta`, `green`, `teal`, `blue`
- **content_filter**: `low`（默认）| `high`
- **速率限制**: Demo 50/小时，生产 1000/小时
- **图片下载**: 使用 `urls.raw` + 尺寸参数构造 URL

#### Pexels 搜索参数
- **Endpoint**: `GET /v1/search`
- **query**: 搜索关键词（必需）
- **page/per_page**: 分页，per_page 最大 80
- **orientation**: `landscape` | `portrait` | `square`
- **size**: `large` | `medium` | `small`
- **color**: 颜色名称或 hex 值（如 `red`, `#FF0000`）
- **速率限制**: 200/小时，20,000/月
- **图片下载**: 使用 `src.original`

#### Wallhaven 搜索参数
- **Endpoint**: `GET /api/v1/search`
- **q**: 搜索关键词
- **categories**: `100`/`101`/`111`（General/Anime/People 的组合）
- **purity**: `100`/`110`/`111`（SFW/Sketchy/NSFW 的组合，NSFW 需要 API Key）
- **sorting**: `date_added` | `relevance` | `random` | `views` | `favorites` | `toplist`
- **order**: `desc`（默认）| `asc`
- **topRange**: `1d` | `3d` | `1w` | `1M` | `3M` | `6M` | `1y`（仅 sorting=toplist 时）
- **atleast**: 最小分辨率（如 `1920x1080`）
- **resolutions**: 逗号分隔的分辨率列表
- **ratios**: 逗号分隔的宽高比（如 `16x9,16x10`）
- **colors**: 逗号分隔的 hex 颜色值
- **每页结果**: 固定 24 条
- **速率限制**: 45/分钟
- **图片下载**: 使用 `path` 字段（原始图片 URL）

### 2.3 下载管理
- 异步并发下载（默认 3 个并发）
- 按来源分类存放: `{download_path}/{source}/`
- 下载进度实时显示
- 断点续传（可选）

### 2.4 配置管理
- 首次运行时 TUI 配置向导
- 配置内容包括：
  - 启用/禁用壁纸来源
  - API Key 配置（需要时）
  - 下载路径设置
- 配置文件路径: `~/.config/wallpaper-magpie/config.toml`

## 3. 架构设计

### 3.1 项目结构

```
wallpaper-magpie/
├── src/
│   ├── main.rs              # 程序入口，解析命令行参数
│   ├── cli.rs               # CLI 参数定义（clap derive）
│   ├── config.rs            # 配置管理（加载、保存、验证）
│   ├── app.rs               # TUI 应用主循环、状态管理
│   ├── ui.rs                # UI 渲染（不同步骤的界面）
│   ├── event.rs             # 事件处理（键盘输入等）
│   ├── download.rs          # 下载管理（并发控制、进度跟踪）
│   ├── providers/           # 壁纸来源适配器
│   │   ├── mod.rs           # Provider trait 定义
│   │   ├── unsplash.rs      # Unsplash 适配器
│   │   ├── pexels.rs        # Pexels 适配器
│   │   └── wallhaven.rs     # Wallhaven 适配器
│   └── models.rs            # 数据模型
├── Cargo.toml
└── README.md
```

### 3.2 数据模型

```rust
// Provider trait - 统一接口
#[async_trait]
pub trait Provider {
    fn name(&self) -> &str;
    fn requires_api_key(&self) -> ApiKeyRequirement;
    fn available_filters(&self) -> Vec<FilterType>;
    async fn search(&self, params: &SearchParams) -> Result<Vec<Wallpaper>>;
    async fn download(&self, wallpaper: &Wallpaper, path: &Path) -> Result<()>;
}

// API Key 需求级别
pub enum ApiKeyRequirement {
    Required,       // 必需（如 Unsplash, Pexels）
    Optional,       // 可选（如 Wallhaven，用于 NSFW 和个性化）
    NotRequired,    // 不需要
}

// 搜索参数 - 通用字段
pub struct SearchParams {
    pub query: String,
    pub limit: u32,
    // 通用筛选条件
    pub resolution: Option<String>,
    pub color: Option<String>,
    pub orientation: Option<String>,
    pub sort: Option<SortOrder>,
    // 来源特定参数（由 Provider 自行解析）
    pub provider_specific: HashMap<String, String>,
}

// 壁纸信息
pub struct Wallpaper {
    pub id: String,
    pub source: String,
    pub url: String,           // 原始下载 URL
    pub filename: String,
    pub resolution: Option<String>,
    pub file_size: Option<u64>,
    pub photographer: Option<String>,  // 摄影师信息（用于归因）
}

// 排序方式
pub enum SortOrder {
    Latest,
    Popular,
    Relevant,
    Random,
    Views,
    Favorites,
}

// 筛选类型（用于动态 UI）
pub enum FilterType {
    Query,
    Resolution,
    Color,
    Orientation,
    Sort,
    Limit,
    Purity,         // Wallhaven 特有
    Category,       // Wallhaven 特有
    Size,           // Pexels 特有
    TopRange,       // Wallhaven 特有
}
```

### 3.3 TUI 流程设计

**向导式三步流程**:

1. **步骤 1**: 选择壁纸来源
   - 列表显示所有可用来源
   - 标记需要 API Key 的来源
   - 如果来源未配置 API Key，提示配置

2. **步骤 2**: 配置筛选条件
   - 根据所选来源动态显示可用筛选器
   - 输入关键词
   - 选择/输入分辨率
   - 选择/输入颜色
   - 选择排序方式
   - 输入下载数量

3. **步骤 3**: 确认并下载
   - 显示搜索条件摘要
   - 确认后开始搜索和下载
   - 显示下载进度列表

### 3.4 配置文件

**路径**: `~/.config/wallpaper-magpie/config.toml`

```toml
# 下载路径（可配置）
download_path = "~/wallpapers"

# 并发下载数
concurrent_downloads = 3

[sources.unsplash]
enabled = true
api_key = "your-unsplash-access-key"

[sources.pexels]
enabled = false
api_key = "your-pexels-api-key"

[sources.wallhaven]
enabled = true
# API Key 可选，用于访问 NSFW 内容和个性化设置
api_key = ""
```

## 4. CLI 命令设计

```bash
# TUI 向导模式（默认）
wallpaper-magpie

# CLI 快速模式
wallpaper-magpie download \
  --source unsplash \
  --query "nature" \
  --resolution 1920x1080 \
  --limit 10

# 配置管理
wallpaper-magpie config              # 查看配置
wallpaper-magpie config --edit       # 编辑配置
wallpaper-magpie config --reset      # 重置配置

# 查看版本
wallpaper-magpie --version
```

## 5. API 认证方式

| 来源 | 认证方式 | 说明 |
|------|---------|------|
| Unsplash | `Authorization: Client-ID YOUR_ACCESS_KEY` | 注册应用获取 Access Key |
| Pexels | `Authorization: YOUR_API_KEY` | 注册账号获取 API Key |
| Wallhaven | `X-API-Key: YOUR_API_KEY` 或 `?apikey=` | 可选，账号设置中生成 |

## 6. 错误处理

- 使用 `anyhow` 进行错误链传播
- 使用 `thiserror` 定义具体错误类型:
  - `ApiError`: API 调用失败（包含 HTTP 状态码）
  - `ConfigError`: 配置错误
  - `DownloadError`: 下载失败
  - `ProviderError`: 来源适配器错误
  - `RateLimitError`: 速率限制错误（包含重试时间）
- 用户友好错误提示:
  - API Key 无效时提示重新配置
  - 网络错误时自动重试（3 次，指数退避）
  - 速率限制时提示等待时间
  - 403 错误时提示检查 API Key 权限
  - 401 错误时提示需要 API Key（Wallhaven NSFW）

## 6. 首次运行流程

1. 检查配置文件是否存在
2. 不存在则启动 TUI 配置向导:
   - 欢迎界面
   - 选择启用的壁纸来源
   - 为需要 API Key 的来源配置 Key
   - 设置下载路径（默认 `~/wallpapers`）
   - 保存配置
3. 进入主 TUI 向导（步骤 1）

## 7. 安全与隐私

- API Key 存储在用户本地配置文件
- 不收集用户数据
- 所有网络请求使用 HTTPS

## 8. 扩展性

- Provider trait 设计允许轻松添加新壁纸来源
- 配置系统支持新增来源的配置字段
- TUI 动态筛选器根据 Provider 实现自动调整

## 9. 依赖库

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4", features = ["derive"] }
ratatui = "0.24"
crossterm = "0.27"
config = "0.13"
toml = "0.8"
anyhow = "1.0"
thiserror = "1.0"
indicatif = "0.17"
owo-colors = "4"
dirs = "5"
chrono = "0.4"
```

## 10. API 文档参考

详细 API 参数说明请参考采集的文档：
- `docs/unsplash-api.md` - Unsplash API 文档
- `docs/pexels-api.md` - Pexels API 文档
- `docs/wallhaven-api.md` - Wallhaven API 文档

## 11. 实现优先级

1. **P0**: 项目脚手架、配置管理、基础 TUI 框架
2. **P1**: Provider trait + Unsplash 适配器 + 下载功能
3. **P2**: Pexels 和 Wallhaven 适配器
4. **P3**: CLI 命令、错误处理优化、文档
