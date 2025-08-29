use std::fs;
use std::path::PathBuf;

use anyhow::{Result, Context};
use clap::{Parser, Subcommand};

mod tui;
mod file_ops;
mod theme_ops;

use tui::TuiPrompts;
use theme_ops::{install_package, uninstall_package, build_theme};

#[derive(Parser)]
#[command(name = "hyprkit", about = "A simple dotfile manager using stow")]
struct Cli {
    /// Directory containing dotfile packages (default: current directory)
    #[arg(short, long)]
    dir: Option<PathBuf>,
    
    /// Target directory (default: $HOME)
    #[arg(short, long)]
    target: Option<PathBuf>,
    
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Install/link a dotfile package
    Install {
        /// Name of the package to install (optional - will prompt if not provided)
        package: Option<String>,
    },
    
    /// Uninstall/unlink a dotfile package
    Uninstall {
        /// Name of the package to uninstall (optional - will prompt if not provided)
        package: Option<String>,
    },
    
    /// Build a new theme from existing config folders
    Build {
        /// Name for the new theme
        name: String,
    },
    
    /// List available packages
    List,
    
    /// Show which packages are currently installed
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let home_dir = PathBuf::from(std::env::var("HOME").expect("HOME environment variable not set"));
    let dotfiles_dir = cli.dir.unwrap_or_else(|| home_dir.join(".config/hyprkit"));
    let target_dir = cli.target.unwrap_or_else(|| home_dir.clone());
    
    match cli.command {
        Commands::Install { package } => {
            let package_name = if let Some(name) = package {
                name
            } else {
                TuiPrompts::select_package_to_install(&dotfiles_dir)?
            };
            install_package(&dotfiles_dir, &target_dir, &package_name)?;
        }
        Commands::Uninstall { package } => {
            let package_name = if let Some(name) = package {
                name
            } else {
                if let Some(selected) = TuiPrompts::select_package_to_uninstall()? {
                    selected
                } else {
                    return Ok(());
                }
            };
            uninstall_package(&target_dir, &package_name)?;
        }
        Commands::Build { name } => {
            build_theme(&name)?;
        }
        Commands::List => {
            list_packages(&dotfiles_dir)?;
        }
        Commands::Status => {
            show_status(&dotfiles_dir, &target_dir)?;
        }
    }

    Ok(())
}

fn list_packages(dotfiles_dir: &PathBuf) -> Result<()> {
    if !dotfiles_dir.exists() {
        anyhow::bail!("Dotfiles directory {} does not exist", dotfiles_dir.display());
    }
    
    let entries = fs::read_dir(dotfiles_dir)
        .context("Failed to read dotfiles directory")?;
    
    let mut packages = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                packages.push(name.to_string());
            }
        }
    }
    
    if packages.is_empty() {
        println!("No packages found in {}", dotfiles_dir.display());
    } else {
        println!("Available packages in {}:", dotfiles_dir.display());
        packages.sort();
        for package in packages {
            println!("  {}", package);
        }
    }
    
    Ok(())
}

fn show_status(dotfiles_dir: &PathBuf, target_dir: &PathBuf) -> Result<()> {
    // This is a simplified status check - we could make it more sophisticated
    println!("Dotfiles directory: {}", dotfiles_dir.display());
    println!("Target directory: {}", target_dir.display());
    
    // List all packages and show basic info
    list_packages(dotfiles_dir)?;
    
    Ok(())
}

pub fn get_config_folders(config_dir: &PathBuf) -> Result<Vec<String>> {
    if !config_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut folders = Vec::new();
    let entries = fs::read_dir(config_dir)
        .context("Failed to read config directory")?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Skip hyprkit directory and hidden directories
                if name != "hyprkit" && !name.starts_with('.') {
                    folders.push(name.to_string());
                }
            }
        }
    }
    
    folders.sort();
    Ok(folders)
}

pub fn get_available_packages(dotfiles_dir: &PathBuf) -> Result<Vec<String>> {
    if !dotfiles_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut packages = Vec::new();
    let entries = fs::read_dir(dotfiles_dir)
        .context("Failed to read dotfiles directory")?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Skip hidden directories and active directory
                if !name.starts_with('.') && name != "active" {
                    packages.push(name.to_string());
                }
            }
        }
    }
    
    packages.sort();
    Ok(packages)
}

pub fn get_active_packages() -> Result<Vec<String>> {
    let home_dir = PathBuf::from(std::env::var("HOME").expect("HOME environment variable not set"));
    let active_dir = home_dir.join(".config/hyprkit/active");
    
    if !active_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut packages = Vec::new();
    let entries = fs::read_dir(&active_dir)
        .context("Failed to read active directory")?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                packages.push(name.to_string());
            }
        }
    }
    
    packages.sort();
    Ok(packages)
}