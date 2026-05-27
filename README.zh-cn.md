# wallmagpie

<div align="center">

[English](README.md) | [简体中文](README.zh-cn.md)

</div>

一款用 Rust 编写的 CLI + TUI 壁纸收集工具，支持从 Unsplash、Pexels 和 Wallhaven 下载壁纸。

## 功能特性

- **多源支持**：Unsplash、Pexels、Wallhaven
- **TUI 向导**：交互式三步向导（选择来源 → 配置筛选条件 → 下载）
- **CLI 模式**：通过命令行参数快速下载
- **并发下载**：可配置的并行下载数量
- **动态筛选**：来源特定的筛选选项
- **进度追踪**：实时下载进度显示

## 安装

```bash
cargo install --path .
```

## 使用方法

### TUI 向导（默认）
```bash
wallmagpie
```

### CLI 模式
```bash
wallmagpie download --source unsplash --query "nature" --limit 10
```

### 配置管理
```bash
wallmagpie config              # 显示配置
wallmagpie config --edit       # 编辑配置（TUI）
wallmagpie config --reset       # 重置为默认值
```

## 配置说明

配置文件位于 `./config.toml`（当前工作目录）：

```toml
download_path = "./wallpapers"
concurrent_downloads = 3

[sources.unsplash]
enabled = true
api_key = "your-unsplash-access-key"

[sources.pexels]
enabled = false
api_key = "your-pexels-api-key"

[sources.wallhaven]
enabled = true
api_key = ""  # 可选
```

## API 密钥

- **Unsplash**：在 https://unsplash.com/developers 注册
- **Pexels**：在 https://www.pexels.com/api/ 注册
- **Wallhaven**：可选，在账户设置中生成

## 许可证

MIT
