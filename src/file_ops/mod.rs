use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::LazyLock;
extern crate dirs;

mod constants;

#[cfg(test)]
pub mod tests;

/// Trait for executing stow commands - allows for dependency injection and mocking
pub trait StowExecutor {
    fn execute_stow(&self, themes_dir: &PathBuf, target_dir: &PathBuf, theme: &str, delete: bool) -> Result<()>;
}

/// Real stow executor that calls the actual stow command
pub struct RealStowExecutor;

impl StowExecutor for RealStowExecutor {
    fn execute_stow(&self, themes_dir: &PathBuf, target_dir: &PathBuf, theme: &str, delete: bool) -> Result<()> {
        let mut cmd = Command::new(constants::STOW_COMMAND);
        cmd.arg(constants::STOW_ARG_DIR)
            .arg(themes_dir)
            .arg(constants::STOW_ARG_TARGET)
            .arg(target_dir)
            .arg(theme);
        
        if delete {
            cmd.arg(constants::STOW_ARG_DELETE);
        }

        let output = cmd.output()
            .context(constants::FAILED_TO_EXECUTE_STOW)?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to execute stow command: {}", stderr);
        }
    }
}

/// File I/O constants
// pub const HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| dirs::home_dir().expect("Failed to get home directory"));
pub const CONFIG_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::config_dir().expect(constants::ERROR_CONFIG_DIR_NOT_FOUND).join(constants::HYPR_DIR));
pub const HYPRKIT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::config_dir()
        .expect(constants::ERROR_HYPRKIT_DIR_NOT_FOUND)
        .join(constants::HYPRKIT_DIR)
});
pub const THEMES_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    HYPRKIT_DIR.join("themes") // Remove the duplicate HYPRKIT_DIR join
});

/// Check for valid package directories
pub fn check_valid_dir(dir: &PathBuf) -> Result<()> {
    if !dir.exists() {
        anyhow::bail!("Directory {} not found", dir.display());
    }

    if !dir.is_dir() {
        anyhow::bail!("'{}' is not a directory", dir.display());
    }

    Ok(())
}

/// Fetch the contents of a directory
pub fn fetch_dir_folders(dir: &PathBuf, exclude: &[String]) -> Result<Vec<String>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut folders = Vec::new();
    let entries = fs::read_dir(dir).context(constants::FAILED_TO_READ_DIRECTORY)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if !name.starts_with('.') && !exclude.contains(&name.to_string()) {
                    folders.push(name.to_string());
                }
            }
        }
    }

    folders.sort();
    Ok(folders)
}

/// Recursively copy a directory and all its contents
pub fn copy_dir_recursive(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path).context(format!(
                "Failed to copy file {} to {}",
                src_path.display(),
                dest_path.display()
            ))?;
        }
    }

    Ok(())
}

/// Move a directory by copying it and then removing the source
pub fn move_dir_recursive(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    // First copy the directory structure
    copy_dir_recursive(src, dest)?;

    // Then remove the source directory
    fs::remove_dir_all(src).context(format!(
        "Failed to remove source directory {}",
        src.display()
    ))?;

    Ok(())
}

/// Move list of folders to target
pub fn move_folders_to_target(folders: &[String], source: &PathBuf, target: &PathBuf) -> Result<()> {
    
    for folder in folders {
        let source_folder = source.join(folder);
        let target_folder = target.join(folder);

        if !source_folder.exists() {
            eprintln!("Warning: Config folder '{}' no longer exists, skipping", folder);
            continue;
        }
        
        if !source_folder.is_dir() {
            eprintln!("Warning: '{}' is not a directory, skipping", folder);
            continue;
        }

        if let Some(parent) = target_folder.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory structure for {}", folder))?;
        }

        move_dir_recursive(&source_folder, &target_folder).context(format!("Failed to move folder {}", folder))?;
    }

    Ok(())
}

/// Execute stow command directly from themes directory using dependency injection
pub fn execute_stow_command_direct(themes_dir: &PathBuf, target_dir: &PathBuf, theme: &str, delete: bool) -> Result<()> {
    let executor = RealStowExecutor;
    executor.execute_stow(themes_dir, target_dir, theme, delete)
}

/// Execute stow command with custom executor (for testing)
pub fn execute_stow_command_with_executor<T: StowExecutor>(
    executor: &T,
    themes_dir: &PathBuf, 
    target_dir: &PathBuf, 
    theme: &str, 
    delete: bool
) -> Result<()> {
    executor.execute_stow(themes_dir, target_dir, theme, delete)
}