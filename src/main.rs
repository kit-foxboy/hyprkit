use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod constants;
mod file_ops;
mod themes;
mod tui;
#[cfg(test)]
mod tests;

use file_ops::*;
use themes::*;
use tui::TuiStrings;
use themes::theme_state::HyprkitState;

use crate::tui::TuiPrompts;

#[derive(Parser)]
#[command(name = "hyprkit", about = crate::constants::APP_DESCRIPTION)]
struct Cli {
    // Directory containing dotfile packages (default: .config)
    #[arg(short, long)]
    dir: Option<PathBuf>,

    // Target directory (default: .config/hyprkit)
    #[arg(short, long)]
    target: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Install/link a theme
    Install {
        theme: Option<String>,
    },

    // Uninstall/unlink a theme
    Uninstall {
        theme: Option<String>,
    },

    // Build a new theme from existing config folders
    Build {
        name: String,
    },

    // List available packages
    List,

    // Show active theme info
    Status,

    // Undo the last theme operation (pop from backup stack)
    Undo,

    // Reset to original configuration (clear all backups)
    Reset,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { theme } => {
            // Install specified theme if provided
            if let Some(t) = theme {
                install_theme(&t)?;
                return Ok(());
            }

            // Otherwise prompt user to select a theme
            let themes = fetch_available_themes()?;
            TuiPrompts::select_theme(&themes)
                .and_then(|t| install_theme(&t))?;
        }
        Commands::Uninstall { theme } => {
            // Uninstall specified theme if provided
            if let Some(t) = theme {
                uninstall_theme(&t)?;
                return Ok(());
            }

            // Otherwise prompt user to select a theme
            let themes = fetch_available_themes()?;
            TuiPrompts::select_theme(&themes)
                .and_then(|t| uninstall_theme(&t))?;
        }
        Commands::Build { name } => {
            build_theme(&name)?;
        }
        Commands::List => {
            list_packages(&HyprkitState::themes_dir_path())?;
        }
        Commands::Status => {
            show_status(&HyprkitState::themes_dir_path())?;
        }
        Commands::Undo => {
            themes::undo_last_operation()?;
        }
        Commands::Reset => {
            themes::reset_to_original()?;
        }
    }

    Ok(())
}

fn list_packages(dotfiles_dir: &PathBuf) -> Result<()> {
    let mut packages = fetch_dir_folders(dotfiles_dir, &[])?;

    if packages.is_empty() {
        TuiStrings::print_dir(TuiStrings::NO_PACKAGES_FOUND, dotfiles_dir);
    } else {
        packages.sort();
        TuiStrings::print_list(TuiStrings::AVAILABLE_PACKAGES_LIST, &packages);
    }

    Ok(())
}

fn show_status(themes_dir: &PathBuf) -> Result<()> {
    let state = HyprkitState::load()?;
    
    // Show current active theme
    match &state.active_theme {
        Some(theme) => println!("{} {}", constants::ACTIVE_THEME, theme),
        None => println!("{}", constants::NO_THEME_ACTIVE),
    }
    
    // Show backup count
    if state.has_backups() {
        println!("Backup stack: {} entries", state.backup_stack.len());
    } else {
        println!("{}", constants::NO_BACKUPS_IN_STACK);
    }
    
    println!("Themes directory: {}", themes_dir.display());
    
    // List all available themes
    list_packages(themes_dir)?;

    Ok(())
}
