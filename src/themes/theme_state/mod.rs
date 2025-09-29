use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod constants;
#[cfg(test)]
mod tests;

use constants::*;
use crate::file_ops::HYPRKIT_DIR;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub files: Vec<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HyprkitState {
    pub active_theme: Option<String>,
    pub backup_stack: Vec<BackupEntry>,
    pub id: u32,
    pub version: String,
}

impl Default for HyprkitState {
    fn default() -> Self {
        HyprkitState {
            active_theme: None,
            backup_stack: Vec::new(),
            id: 0,
            version: DEFAULT_VERSION.to_string(),
        }
    }
}

impl HyprkitState {
    /// Load state from JSON file, creating default if it doesn't exist
    pub fn load() -> Result<Self> {
        Self::load_from_path(Self::state_file_path())
    }

    /// Load state from a specific path (for testing)
    pub fn load_from_path(state_path: PathBuf) -> Result<Self> {
        if !state_path.exists() {
            let state = HyprkitState::default();
            state.save_to_path(state_path)?;
            return Ok(state);
        }

        let content = fs::read_to_string(&state_path)
            .context(ERR_READ_STATE)?;
        
        serde_json::from_str(&content)
            .context(ERR_PARSE_STATE)
    }

    /// Save state to JSON file
    pub fn save(&self) -> Result<()> {
        self.save_to_path(Self::state_file_path())
    }

    /// Save state to a specific path (for testing)
    pub fn save_to_path(&self, state_path: PathBuf) -> Result<()> {
        // Ensure the directory exists
        if let Some(parent) = state_path.parent() {
            fs::create_dir_all(parent)
                .context(ERR_CREATE_HYPRKIT_DIR)?;
        }

        let json = serde_json::to_string_pretty(self)
            .context(ERR_SERIALIZE_STATE)?;
        
        fs::write(&state_path, json)
            .context(ERR_WRITE_STATE)
    }

    /// Get the path to the state file
    pub fn state_file_path() -> PathBuf {
        HYPRKIT_DIR.join(STATE_FILE_NAME)
    }

    /// Get the path to the backups directory
    pub fn backups_dir_path() -> PathBuf {
        HYPRKIT_DIR.join(BACKUPS_DIR_NAME)
    }

    /// Get the path to the themes directory
    pub fn themes_dir_path() -> PathBuf {
        HYPRKIT_DIR.join(THEMES_DIR_NAME)
    }

    /// Generate next backup ID
    fn get_id(&mut self) -> String {
        self.id += 1;
        format!("{:03}", self.id)
    }

    /// Push a new backup entry onto the stack
    pub fn push_backup(&mut self, name: String, description: String, files: Vec<String>) -> Result<String> {
        let id = self.push_backup_no_save(name, description, files);
        self.save()?;
        Ok(id)
    }

    /// Push a new backup entry onto the stack without saving (for testing)
    pub fn push_backup_no_save(&mut self, name: String, description: String, files: Vec<String>) -> String {
        let id = self.get_id();
        let entry = BackupEntry {
            id: id.clone(),
            name,
            description,
            timestamp: Utc::now(),
            files,
            metadata: None,
        };

        self.backup_stack.push(entry);
        id
    }

    /// Pop the most recent backup entry from the stack
    pub fn pop_backup(&mut self) -> Result<Option<BackupEntry>> {
        let entry = self.backup_stack.pop();
        self.save()?;
        Ok(entry)
    }

    /// Peek at the top of the backup stack without removing it
    pub fn peek_backup(&self) -> Option<&BackupEntry> {
        self.backup_stack.last()
    }

    /// Get backup entry by ID
    pub fn get_backup(&self, id: &str) -> Option<&BackupEntry> {
        self.backup_stack.iter().find(|entry| entry.id == id)
    }

    /// Clear all backups
    pub fn clear_backups(&mut self) -> Result<()> {
        self.backup_stack.clear();
        self.id = 0;
        self.save()
    }

    /// Set the active theme
    pub fn set_active_theme(&mut self, theme: Option<String>) -> Result<()> {
        self.active_theme = theme;
        self.save()
    }

    /// Get the backup directory path for a specific backup ID
    pub fn backup_path(&self, id: &str) -> PathBuf {
        Self::backups_dir_path().join(format!("{} {}", BACKUP_FILE_FORMAT, id))
    }

    /// Get list of files that would be affected by installing a theme
    pub fn get_theme_files(theme_name: &str) -> Result<Vec<String>> {
        let theme_path = Self::themes_dir_path().join(theme_name);
        if !theme_path.exists() {
            return Ok(Vec::new());
        }

        let mut files = Vec::new();
        Self::collect_theme_files(&theme_path, &theme_path, &mut files)?;
        Ok(files)
    }

    /// Recursively collect all files in a theme directory
    fn collect_theme_files(base_path: &PathBuf, current_path: &PathBuf, files: &mut Vec<String>) -> Result<()> {
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                Self::collect_theme_files(base_path, &path, files)?;
            } else if path.is_file() {
                // Get relative path from theme root
                if let Ok(rel_path) = path.strip_prefix(base_path) {
                    files.push(rel_path.to_string_lossy().to_string());
                }
            }
        }
        Ok(())
    }

    /// Check if there are any backups on the stack
    pub fn has_backups(&self) -> bool {
        !self.backup_stack.is_empty()
    }

    /// Get the number of backups on the stack
    pub fn backup_count(&self) -> usize {
        self.backup_stack.len()
    }
}
