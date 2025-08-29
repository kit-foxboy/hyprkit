use std::path::PathBuf;
use anyhow::{Result, Context};
use inquire::{MultiSelect, Select};

use crate::{get_available_packages, get_active_packages, get_config_folders};

pub struct TuiPrompts;

impl TuiPrompts {
    /// Prompt user to select a package to install from available packages
    pub fn select_package_to_install(dotfiles_dir: &PathBuf) -> Result<String> {
        let available_packages = get_available_packages(dotfiles_dir)?;
        
        if available_packages.is_empty() {
            anyhow::bail!("No packages found in {}", dotfiles_dir.display());
        }
        
        Select::new("Select a package to install:", available_packages)
            .with_help_message("Use arrow keys to navigate, enter to confirm")
            .prompt()
            .context("Failed to get package selection")
    }
    
    /// Prompt user to select a package to uninstall from active packages
    pub fn select_package_to_uninstall() -> Result<Option<String>> {
        let active_packages = get_active_packages()?;
        
        if active_packages.is_empty() {
            println!("No packages are currently installed.");
            return Ok(None);
        }
        
        let selection = Select::new("Select a package to uninstall:", active_packages)
            .with_help_message("Use arrow keys to navigate, enter to confirm")
            .prompt()
            .context("Failed to get package selection")?;
        
        Ok(Some(selection))
    }
    
    /// Prompt user to select config folders to include in a theme
    pub fn select_config_folders(theme_name: &str) -> Result<Vec<String>> {
        let home_dir = PathBuf::from(std::env::var("HOME").expect("HOME environment variable not set"));
        let config_dir = home_dir.join(".config");
        
        let available_folders = get_config_folders(&config_dir)?;
        
        if available_folders.is_empty() {
            anyhow::bail!("No config folders found in {}", config_dir.display());
        }
        
        println!("Select config folders to include in theme '{}':", theme_name);
        
        let selected_folders = MultiSelect::new("Config folders:", available_folders)
            .with_help_message("Use spacebar to select/deselect, enter to confirm")
            .prompt()
            .context("Failed to get folder selection")?;
        
        if selected_folders.is_empty() {
            println!("No folders selected, cancelling theme creation.");
        }
        
        Ok(selected_folders)
    }
}
