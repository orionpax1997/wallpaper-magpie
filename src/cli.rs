use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "wallpaper-magpie")]
#[command(about = "A CLI + TUI tool for collecting wallpapers from multiple sources")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Download(DownloadArgs),
    Config(ConfigArgs),
}

#[derive(Parser, Debug)]
pub struct DownloadArgs {
    #[arg(short, long)]
    pub source: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[arg(short, long)]
    pub show: bool,
}
