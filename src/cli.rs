use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "wallmagpie")]
#[command(about = "A CLI + TUI tool for collecting wallpapers")]
#[command(version = "0.1.0")]
#[command(disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download wallpapers using TUI wizard or CLI args
    Download(DownloadArgs),
    /// Manage configuration
    Config(ConfigArgs),
    /// Show help for a source's filters
    Help(HelpArgs),
}

#[derive(Args, Debug, Clone)]
pub struct HelpArgs {
    /// Source name (wallhaven, unsplash, pexels)
    pub source: String,
}

#[derive(Args, Debug, Clone)]
pub struct DownloadArgs {
    /// Wallpaper source (unsplash, pexels, wallhaven)
    #[arg(short, long)]
    pub source: Option<String>,

    /// Search query
    #[arg(short, long)]
    pub query: Option<String>,

    /// Resolution filter (e.g., 1920x1080)
    #[arg(short, long)]
    pub resolution: Option<String>,

    /// Color filter (e.g., blue, #FF0000)
    #[arg(long)]
    pub color: Option<String>,

    /// Orientation filter (landscape, portrait, squarish)
    #[arg(long)]
    pub orientation: Option<String>,

    /// Sort order (latest, popular, relevant, random)
    #[arg(long)]
    pub sort: Option<String>,

    /// Number of wallpapers to download
    #[arg(short, long, default_value = "10")]
    pub limit: u32,

    /// Use TUI wizard mode (default if no args provided)
    #[arg(long)]
    pub wizard: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ConfigArgs {
    /// Edit configuration
    #[arg(long)]
    pub edit: bool,

    /// Reset configuration to defaults
    #[arg(long)]
    pub reset: bool,

    /// Show current configuration
    #[arg(long, default_value = "true")]
    pub show: bool,
}
