use std::fs;

use anyhow::Result;
use clap::{Parser, Subcommand};
use hyprkit::config::{PathManager, THEME_FILE};
use tracing_subscriber::fmt::init;
use walkdir::WalkDir;
use toml::Table;

#[derive(Parser)]
#[command(name = "hyprkit", about = "A theme and state manager for Hyprland")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    // Serve over HTTP
    ServeHttp {
        #[arg(short, long, default_value = "3000")]
        port: u16,
        #[arg(long)]
        host: Option<String>
    },

    // Serve over IPC for direct communication
    ServeIPC {
        #[arg(long)]
        socket_path: Option<String>
    },

    // Apply a theme - CLI direct
    Apply {
        #[arg(short, long)]
        theme: String,
        #[arg(long)]
        variant: Option<String>,
    },

    // List available themes - CLI direct
    List
}

#[tokio::main]
async fn main() -> Result<()> {

    // Initialize logging
    init();

    // Set up paths
    let path_manager = PathManager::new().await?;
    
    let cli = Cli::parse();
    match cli.command {
        Commands::ServeHttp { port, host } => {
            // Serve HTTP
            println!("Serving HTTP on {}:{}", host.as_deref().unwrap_or("localhost"), port);
        }
        Commands::ServeIPC { socket_path } => {
            // Serve IPC
            if let Some(path) = socket_path {
                println!("Serving IPC on socket: {}", path);
            } else {
                println!("Serving IPC on default socket");
            }
        }
        Commands::Apply { theme, variant } => {
            // Apply theme
            println!("Applying theme: {}", theme);
            if let Some(variant) = variant {
                println!("With variant: {}", variant);
            }
        }
        Commands::List => {
            // List themes
            let themes = read_themes(path_manager).await?;
            if themes.is_empty() {
                println!("No themes available.");
                //TODO: Offer to create one from current configs
            } else {
                println!("Available themes:");
                for theme in themes {
                    println!("- {}", theme);
                }
            }
        }
    }

    Ok(())
}

async fn read_themes(path_manager: PathManager) -> Result<Vec<String>> {
    let mut themes = Vec::new();
    
    // TODO: Use async file reading if needed, but for now we can use sync since it's just listing directories
    // TODO: Determine whether to follow symlinks or not
    for entry in WalkDir::new(path_manager.theme_dir).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_dir() && entry.file_name() == THEME_FILE {
            
            // Read the theme file for metadata and extract the theme name
            let theme: Result<Table, _> = fs::read_to_string(entry.path())?.parse();
            match theme {
                Ok(toml) => {
                    if let Some(theme_name) = toml.get("theme_name").and_then(|v| v.as_str()) {
                        themes.push(theme_name.to_string());
                        continue;
                    }
                    tracing::warn!("Theme file {} does not contain a valid theme_name", entry.path().display());  
                }
                Err(e) => {
                    tracing::error!("Failed to parse theme file {}: {}", entry.path().display(), e);
                }
            }
        }
    }

    Ok(themes)
}