use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use wallpaper_magpie::app::App;
use wallpaper_magpie::cli::{Cli, Commands, DownloadArgs};
use wallpaper_magpie::config::AppConfig;
use wallpaper_magpie::download::DownloadManager;
use wallpaper_magpie::event::EventHandler;
use wallpaper_magpie::filter_config::get_filters_for_source;
use wallpaper_magpie::models::{FilterFieldType, SearchParams, SortOrder};
use wallpaper_magpie::providers;
use tokio::sync::mpsc;

mod cli;
mod config;
mod error;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|_| {
        disable_raw_mode().ok();
        let _ = io::stdout().execute(LeaveAlternateScreen);
    }));

    let cli = Cli::parse();

    match cli.command {
        Commands::Download(args) => {
            if args.wizard || args.source.is_none() {
                run_tui().await?;
            } else {
                run_cli_download(args).await?;
            }
        }
        Commands::Config(args) => {
            if args.reset {
                let config = AppConfig::default();
                config.save()?;
                println!("Configuration reset to defaults");
            } else if args.edit {
                run_tui_config().await?;
            } else {
                let config = AppConfig::load()?;
                println!("{}", toml::to_string_pretty(&config)?);
            }
        }
        Commands::Help(args) => {
            print_source_help(&args.source);
        }
    }

    Ok(())
}

fn cleanup_terminal() {
    disable_raw_mode().ok();
    let _ = io::stdout().execute(LeaveAlternateScreen);
}

async fn run_tui() -> Result<()> {
    let _ = AppConfig::load()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let force_quit = Arc::new(AtomicBool::new(false));
    let fq = force_quit.clone();

    thread::spawn(move || {
        signal_hook::flag::register(signal_hook::consts::SIGINT, fq.clone()).ok();
        signal_hook::flag::register(signal_hook::consts::SIGTERM, fq.clone()).ok();
    });

    let mut app = App::new();
    let event_handler = EventHandler::new(250);
    let terminal = Arc::new(tokio::sync::Mutex::new(terminal));

    loop {
        {
            let mut term = terminal.lock().await;
            term.draw(|f| app.draw(f))?;
        }

        if force_quit.load(Ordering::SeqCst) {
            drop(terminal);
            cleanup_terminal();
            return Ok(());
        }

        match event_handler.next_event()? {
            wallpaper_magpie::event::AppEvent::Tick => {}
            wallpaper_magpie::event::AppEvent::Key(key) => {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('c')
                    && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL)
                {
                    drop(terminal);
                    cleanup_terminal();
                    return Ok(());
                }
                app.handle_input(key).await;
            }
        }

        if app.current_step == wallpaper_magpie::app::AppStep::Downloading
            && app.download_task.is_none()
        {
            let wallpapers = match app.start_download().await {
                Ok(w) => w,
                Err(e) => {
                    app.set_error(e.to_string());
                    app.current_step = wallpaper_magpie::app::AppStep::ConfirmAndDownload;
                    if let Some(ref mut page) = app.page_three {
                        page.is_downloading = false;
                    }
                    continue;
                }
            };

            if let Err(e) = app.begin_download_task(wallpapers) {
                app.set_error(e.to_string());
                app.current_step = wallpaper_magpie::app::AppStep::ConfirmAndDownload;
                if let Some(ref mut page) = app.page_three {
                    page.is_downloading = false;
                }
            }
        }

        app.poll_download_progress();

        if app.is_download_complete() {
            if let Err(e) = app.finalize_download().await {
                app.set_error(e.to_string());
                app.current_step = wallpaper_magpie::app::AppStep::ConfirmAndDownload;
            }
        }

        if app.should_quit {
            break;
        }
    }

    drop(terminal);
    cleanup_terminal();
    Ok(())
}

async fn run_cli_download(args: DownloadArgs) -> Result<()> {
    let config = AppConfig::load()?;

    let source_name = args.source.unwrap();
    let api_key = config.get_api_key(&source_name).unwrap_or_default();
    let provider = providers::create_provider(&source_name, &api_key)
        .ok_or_else(|| anyhow::anyhow!("Failed to create provider - check API key"))?;

    let params = SearchParams {
        query: args.query.unwrap_or_default(),
        limit: args.limit,
        resolution: args.resolution,
        color: args.color,
        orientation: args.orientation,
        sort: args.sort.and_then(|s| match s.as_str() {
            "latest" => Some(SortOrder::Latest),
            "popular" => Some(SortOrder::Popular),
            "relevant" => Some(SortOrder::Relevant),
            "random" => Some(SortOrder::Random),
            _ => None,
        }),
        provider_specific: HashMap::new(),
    };

    println!("Searching {} for '{}'...", source_name, params.query);
    let wallpapers = provider.search(&params).await?;

    println!("Found {} wallpapers", wallpapers.len());

    let download_path = config.expand_download_path().join(&source_name);
    let manager = DownloadManager::new(config.concurrent_downloads);

    let (progress_tx, mut progress_rx) = mpsc::channel(100);
    let cancel_token = Arc::new(AtomicBool::new(false));

    let download_handle = tokio::spawn(async move {
        manager
            .download_wallpapers(provider, wallpapers, download_path, progress_tx, cancel_token)
            .await
    });

    while let Some(progress) = progress_rx.recv().await {
        println!("Progress: {}/{}", progress.completed, progress.total);
    }

    let results = download_handle.await??;

    let success_count = results.iter().filter(|(_, r)| r.is_ok()).count();
    println!("Downloaded {}/{} wallpapers", success_count, results.len());

    Ok(())
}

async fn run_tui_config() -> Result<()> {
    println!("TUI config editor not yet implemented");
    Ok(())
}

fn print_source_help(source: &str) {
    if let Some(filters) = get_filters_for_source(source) {
        println!("{} 支持的过滤器:\n", filters.source_name);
        for field in &filters.fields {
            let type_str = match &field.filter_type {
                FilterFieldType::Text => "字符串".to_string(),
                FilterFieldType::Number => "数字".to_string(),
                FilterFieldType::Enum { options } => format!("枚举 ({})", options.join(", ")),
            };
            let default_str = field
                .default_value
                .as_ref()
                .map(|v| format!(" [默认: {}]", v))
                .unwrap_or_default();
            println!(
                "{:<15} {} - {}{}",
                field.name, field.display_name, type_str, default_str
            );
        }
    } else {
        eprintln!("未知源: {}", source);
        println!("\n可用源: wallhaven, unsplash, pexels");
    }
}
