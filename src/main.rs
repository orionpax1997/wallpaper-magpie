use anyhow::Result;
use clap::Parser;

pub mod app;
pub mod event;
pub mod ui;

pub mod cli;
pub mod config;
pub mod download;
pub mod error;
pub mod models;
pub mod providers;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download(args) => {
            if args.wizard || args.source.is_none() {
                println!("Starting TUI wizard...");
            } else {
                println!("CLI download: {:?}", args);
            }
            Ok(())
        }
        Commands::Config(args) => {
            if args.reset {
                let cfg = config::AppConfig::default();
                cfg.save()?;
                println!("Configuration reset to defaults");
            } else if args.edit {
                println!("Opening configuration editor...");
            } else {
                let cfg = config::AppConfig::load()?;
                println!("{}", toml::to_string_pretty(&cfg)?);
            }
            Ok(())
        }
    }
}
