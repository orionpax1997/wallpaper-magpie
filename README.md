# wallmagpie

<div align="center">

[English](README.md) | [简体中文](README.zh-cn.md)

</div>

A CLI + TUI wallpaper collection tool written in Rust. Supports downloading wallpapers from Unsplash, Pexels, and Wallhaven.

## Features

- **Multi-source support**: Unsplash, Pexels, Wallhaven
- **TUI Wizard**: Interactive three-step wizard (select source → configure filters → download)
- **CLI mode**: Quick downloads via command-line arguments
- **Concurrent downloads**: Configurable parallel downloads
- **Dynamic filtering**: Source-specific filter options
- **Progress tracking**: Real-time download progress

## Installation

```bash
cargo install --path .
```

## Usage

### TUI Wizard (default)
```bash
wallmagpie
```

### CLI Mode
```bash
wallmagpie download --source unsplash --query "nature" --limit 10
```

### Configuration
```bash
wallmagpie config              # Show config
wallmagpie config --edit       # Edit config (TUI)
wallmagpie config --reset      # Reset to defaults
```

## Configuration

Configuration is stored at `./config.toml` (current working directory):

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
api_key = ""  # Optional
```

## API Keys

- **Unsplash**: Register at https://unsplash.com/developers
- **Pexels**: Register at https://www.pexels.com/api/
- **Wallhaven**: Optional, generate in account settings

## License

MIT
