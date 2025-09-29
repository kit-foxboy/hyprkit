pub mod theme_state;

use std::fs;
use std::path::Path;
use anyhow::Result;

use crate::file_ops::{self, *};
use crate::tui::{TuiPrompts, TuiStrings};
use crate::theme_state::HyprkitState;

/// List available themes in the hyprkit themes directory
pub fn fetch_available_themes() -> Result<Vec<String>> {
    let themes = fetch_dir_folders(&file_ops::THEMES_DIR, &[])?;
    Ok(themes)
}

/// Install a theme using stack-based approach with direct stow
pub fn install_theme(theme: &str) -> Result<()> {
    let mut state = HyprkitState::load()?;
    let themes_dir = HyprkitState::themes_dir_path();
    let theme_dir = themes_dir.join(theme);

    // Validate theme exists
    check_valid_dir(&theme_dir)?;

    println!("{}{}", TuiStrings::INSTALLING_PACKAGE, theme);

    // Step 1: Backup current state if we have an active theme
    let current_theme = state.active_theme.clone();
    if let Some(ref current_theme) = current_theme {
        backup_current_state(&mut state, current_theme)?;
    }

    // Step 2: Uninstall current theme if exists
    if let Some(ref current_theme) = current_theme {
        uninstall_theme_direct(current_theme)?;
    }

    // Step 3: Install new theme using stow directly from themes directory
    execute_stow_command_direct(&themes_dir, &CONFIG_DIR, theme, false)?;

    // Step 4: Update state
    state.set_active_theme(Some(theme.to_string()))?;

    println!("✓ Theme '{}' installed successfully", theme);
    Ok(())
}

/// Uninstall a theme using stack-based approach
pub fn uninstall_theme(theme: &str) -> Result<()> {
    let mut state = HyprkitState::load()?;

    println!("{}{}", TuiStrings::UNINSTALLING_PACKAGE, theme);

    // Check if this theme is currently active
    if state.active_theme.as_deref() == Some(theme) {
        uninstall_theme_direct(theme)?;
        state.set_active_theme(None)?;
    }

    println!("✓ Theme '{}' uninstalled successfully", theme);
    Ok(())
}

/// Build a new theme from selected config folders
pub fn build_theme(name: &str) -> Result<()> {

    // Validate theme directory
    let themes_dir = HyprkitState::themes_dir_path();
    let theme_dir = themes_dir.join(name);
    if theme_dir.exists() {
        anyhow::bail!(TuiStrings::ERROR_THEME_EXISTS);
    }
    
    // Interactive selection of folders
    let selected_folders = TuiPrompts::select_config_folders(name)?;
    if selected_folders.is_empty() {
        return Ok(());
    }

    // Move selected folders to the theme directory
    println!("{}", TuiStrings::BUILDING_THEME);
    move_folders_to_target(&selected_folders, &CONFIG_DIR, &theme_dir)?;
    println!("{}{}", TuiStrings::THEME_BUILT_SUCCESS, name);

    // Ask if user wants to install the theme they just built
    if TuiPrompts::prompt_install_built_theme(name)? {
        println!();
        install_theme(name)?;
    }
    
    Ok(())
}

/// Backup the current state by copying affected files
fn backup_current_state(state: &mut HyprkitState, current_theme: &str) -> Result<()> {
    let files = HyprkitState::get_theme_files(current_theme)?;
    let backup_id = state.push_backup(
        current_theme.to_string(),
        format!("Backup before switching from theme '{}'", current_theme),
        files.clone(),
    )?;

    let backup_dir = state.backup_path(&backup_id);
    fs::create_dir_all(&backup_dir)?;

    // Copy each affected file to backup directory
    for file in &files {
        let source = CONFIG_DIR.join(file);
        let dest = backup_dir.join(file);
        
        // Create parent directory if needed
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }

        // Copy file if it exists
        if source.exists() && source.is_file() {
            fs::copy(&source, &dest)?;
        }
    }

    Ok(())
}

/// Directly uninstall a theme using stow
fn uninstall_theme_direct(theme: &str) -> Result<()> {
    let themes_dir = HyprkitState::themes_dir_path();
    execute_stow_command_direct(&themes_dir, &CONFIG_DIR, theme, true)
}

/// Undo the last theme operation by popping from backup stack
pub fn undo_last_operation() -> Result<()> {
    let mut state = HyprkitState::load()?;
    
    let backup = match state.pop_backup()? {
        Some(backup) => backup,
        None => {
            println!("No operations to undo");
            return Ok(());
        }
    };

    println!("Undoing: {}", backup.description);

    // Remove current active theme if any
    if let Some(current_theme) = &state.active_theme {
        uninstall_theme_direct(current_theme)?;
    }

    // Restore files from backup
    let backup_dir = state.backup_path(&backup.id);
    for file in &backup.files {
        let source = backup_dir.join(file);
        let dest = CONFIG_DIR.join(file);

        if source.exists() {
            // Create parent directory if needed
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source, &dest)?;
        }
    }

    // Clean up backup directory
    if backup_dir.exists() {
        fs::remove_dir_all(&backup_dir)?;
    }

    // Determine new active theme (could be None or the previous theme in stack)
    let new_active = if backup.name == "original" {
        None
    } else {
        Some(backup.name)
    };
    
    state.set_active_theme(new_active)?;

    println!("✓ Successfully undid last operation");
    Ok(())
}

/// Reset to original configuration by clearing all backups
pub fn reset_to_original() -> Result<()> {
    let mut state = HyprkitState::load()?;

    if !state.has_backups() {
        println!("No backups found - nothing to reset");
        return Ok(());
    }

    println!("Resetting to original configuration...");

    // Remove current active theme
    if let Some(current_theme) = &state.active_theme {
        uninstall_theme_direct(current_theme)?;
    }

    // Find original backup (should be first in stack)
    let original_backup = state.backup_stack.first().cloned();
    
    if let Some(original) = original_backup {
        if original.name == "original" {
            // Restore original files
            let backup_dir = state.backup_path(&original.id);
            for file in &original.files {
                let source = backup_dir.join(file);
                let dest = CONFIG_DIR.join(file);

                if source.exists() {
                    if let Some(parent) = dest.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(&source, &dest)?;
                }
            }
        }
    }

    // Clear all backups
    let backups_dir = HyprkitState::backups_dir_path();
    if backups_dir.exists() {
        fs::remove_dir_all(&backups_dir)?;
    }

    state.clear_backups()?;
    state.set_active_theme(None)?;

    println!("✓ Successfully reset to original configuration");
    Ok(())
}
