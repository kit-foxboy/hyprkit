use std::fs;
use std::path::PathBuf;
use std::process::Command;
use anyhow::{Result, Context};

use crate::file_ops::{copy_dir_recursive, move_dir_recursive};
use crate::tui::TuiPrompts;

/// Install a package using stow
pub fn install_package(dotfiles_dir: &PathBuf, target_dir: &PathBuf, package: &str) -> Result<()> {
    let package_dir = dotfiles_dir.join(package);
    
    if !package_dir.exists() {
        anyhow::bail!("Package '{}' not found in {}", package, dotfiles_dir.display());
    }
    
    if !package_dir.is_dir() {
        anyhow::bail!("'{}' is not a directory", package);
    }
    
    // Create active directory for stow operations
    let home_dir = PathBuf::from(std::env::var("HOME").expect("HOME environment variable not set"));
    let hyprkit_dir = home_dir.join(".config/hyprkit");
    let active_dir = hyprkit_dir.join("active");
    let active_package_dir = active_dir.join(package);
    
    // Clean up any existing active package
    if active_package_dir.exists() {
        fs::remove_dir_all(&active_package_dir)
            .context("Failed to remove existing active package")?;
    }
    
    fs::create_dir_all(&active_dir)
        .context("Failed to create active directory")?;
    
    println!("Installing package: {}", package);
    
    // Copy theme to active directory
    copy_dir_recursive(&package_dir, &active_package_dir)
        .context("Failed to copy package to active directory")?;
    
    // Run stow from the active directory
    let output = Command::new("stow")
        .arg("--dir")
        .arg(&active_dir)
        .arg("--target")
        .arg(target_dir)
        .arg(package)
        .output()
        .context("Failed to execute stow command. Make sure stow is installed.")?;
    
    if output.status.success() {
        println!("✓ Successfully installed '{}'", package);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to install '{}': {}", package, stderr);
    }
    
    Ok(())
}

/// Uninstall a package using stow
pub fn uninstall_package(target_dir: &PathBuf, package: &str) -> Result<()> {
    let home_dir = PathBuf::from(std::env::var("HOME").expect("HOME environment variable not set"));
    let hyprkit_dir = home_dir.join(".config/hyprkit");
    let active_dir = hyprkit_dir.join("active");
    
    println!("Uninstalling package: {}", package);
    
    // Run stow delete from the active directory
    let output = Command::new("stow")
        .arg("--dir")
        .arg(&active_dir)
        .arg("--target")
        .arg(target_dir)
        .arg("--delete")
        .arg(package)
        .output()
        .context("Failed to execute stow command. Make sure stow is installed.")?;
    
    if output.status.success() {
        // Remove the package from active directory after successful uninstall
        let active_package_dir = active_dir.join(package);
        if active_package_dir.exists() {
            fs::remove_dir_all(&active_package_dir)
                .context("Failed to remove package from active directory")?;
        }
        println!("✓ Successfully uninstalled '{}'", package);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to uninstall '{}': {}", package, stderr);
    }
    
    Ok(())
}

/// Build a new theme from selected config folders
pub fn build_theme(name: &str) -> Result<()> {
    let home_dir = PathBuf::from(std::env::var("HOME").expect("HOME environment variable not set"));
    let config_dir = home_dir.join(".config");
    let hyprkit_dir = config_dir.join("hyprkit");
    let theme_dir = hyprkit_dir.join(name);
    
    // Create the theme directory
    if theme_dir.exists() {
        anyhow::bail!("Theme '{}' already exists in {}", name, hyprkit_dir.display());
    }
    
    // Interactive selection of folders
    let selected_folders = TuiPrompts::select_config_folders(name)?;
    
    if selected_folders.is_empty() {
        return Ok(());
    }
    
    fs::create_dir_all(&theme_dir)
        .context("Failed to create theme directory")?;
    
    println!("Building theme '{}' from selected config folders...", name);
    
    for folder in &selected_folders {
        let source_folder = config_dir.join(folder);
        let target_folder = theme_dir.join(".config").join(folder);
        
        if !source_folder.exists() {
            eprintln!("Warning: Config folder '{}' no longer exists, skipping", folder);
            continue;
        }
        
        if !source_folder.is_dir() {
            eprintln!("Warning: '{}' is not a directory, skipping", folder);
            continue;
        }
        
        // Create parent directories
        if let Some(parent) = target_folder.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory structure for {}", folder))?;
        }
        
        // Move the folder instead of copying
        move_dir_recursive(&source_folder, &target_folder)
            .context(format!("Failed to move folder '{}'", folder))?;
        
        println!("✓ Moved {} to theme", folder);
    }
    
    println!("✓ Successfully built theme '{}' in {}", name, theme_dir.display());
    println!("You can now install it with: hyprkit install {}", name);
    
    Ok(())
}
