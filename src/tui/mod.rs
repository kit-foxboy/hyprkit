use anyhow::{Context, Result};
use inquire::{MultiSelect, Select};
use std::path::PathBuf;

use crate::file_ops::*;

mod constants;

// TUI strings and prompts
pub struct TuiStrings;

impl TuiStrings {

    // Information strings
    pub const NO_PACKAGES_FOUND: &str = constants::NO_PACKAGES_FOUND;
    pub const AVAILABLE_PACKAGES_LIST: &str = constants::AVAILABLE_PACKAGES_LIST;
    pub const DOTFILES_DIR: &str = constants::DOTFILES_DIR;
    pub const TARGET_DIR: &str = constants::TARGET_DIR;
    pub const INSTALLING_PACKAGE: &str = constants::INSTALLING_PACKAGE;
    pub const UNINSTALLING_PACKAGE: &str = constants::UNINSTALLING_PACKAGE;
    pub const BUILDING_THEME: &str = constants::BUILDING_THEME;
    pub const THEME_BUILT_SUCCESS: &str = constants::THEME_BUILT_SUCCESS;

    // Error strings
    pub const ERROR_THEME_EXISTS: &str = constants::ERROR_THEME_EXISTS;
    pub const ERROR_FAILED_TO_REMOVE_THEME: &str = constants::ERROR_FAILED_TO_REMOVE_THEME;
    pub const ERROR_FAILED_TO_CREATE_THEME_DIR: &str = constants::ERROR_FAILED_TO_CREATE_THEME_DIR;
    pub const ERROR_FAILED_TO_COPY_THEME: &str = constants::ERROR_FAILED_TO_COPY_THEME;


    // TUI-related print functions
    pub fn print_dir(message: &str, dir: &PathBuf) {
        println!("{}: {}", message, dir.display());
    }

    pub fn print_list(message: &str, items: &[String]) {
        println!("{}", message);
        for item in items {
            println!("  {}", item);
        }
    }
}

// TUI-related prompts and interactions
pub struct TuiPrompts;

impl TuiPrompts {

    /// Prompt user to select config folders to include in a theme
    pub fn select_config_folders(theme_name: &str) -> Result<Vec<String>> {

        let available_folders = fetch_dir_folders(&CONFIG_DIR, &["hyprkit".to_string()])?;

        if available_folders.is_empty() {
            anyhow::bail!("No config folders found in {}", CONFIG_DIR.display());
        }

        println!(
            "Select config folders to include in theme '{}':",
            theme_name
        );

        let selected_folders = MultiSelect::new(constants::CONFIG_FOLDERS_PROMPT, available_folders)
            .with_help_message(constants::MULTISELECT_HELP)
            .prompt()
            .context(constants::ERROR_FAILED_TO_GET_FOLDER_SELECTION)?;

        if selected_folders.is_empty() {
            println!("{}", constants::NO_FOLDERS_SELECTED);
        }

        Ok(selected_folders)
    }

    /// Prompt user to select a theme to install
    pub fn select_theme(themes: &[String]) -> Result<String> {
        let selection = Select::new(constants::SELECT_THEME_PROMPT, themes.to_vec())
            .with_help_message(constants::SELECT_HELP)
            .prompt()
            .context(constants::ERROR_FAILED_TO_GET_THEME_SELECTION)?;

        Ok(selection)
    }

    /// Prompt user if they want to install the theme they just built
    pub fn prompt_install_built_theme(theme_name: &str) -> Result<bool> {
        let options = vec![constants::YES_OPTION.to_string(), constants::NO_OPTION.to_string()];
        
        let selection = Select::new(
            &format!("Do you want to install the theme '{}' now?", theme_name),
            options
        )
        .with_help_message(constants::SELECT_HELP)
        .prompt()
        .context(constants::ERROR_FAILED_TO_GET_INSTALL_CONFIRMATION)?;

        Ok(selection == constants::YES_OPTION)
    }
}
