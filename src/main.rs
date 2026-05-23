use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
pub mod error;
mod models;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Download(args) => {
            println!("Download command: {:?}", args);
            Ok(())
        }
        cli::Commands::Config(args) => {
            println!("Config command: {:?}", args);
            Ok(())
        }
    }
}
