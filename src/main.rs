use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::fmt::init;

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
            println!("Listing available themes...");
        }
    }

    Ok(())
}