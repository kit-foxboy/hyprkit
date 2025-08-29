use std::path::PathBuf;

use anyhow::Context;
use futures::stream::TryStreamExt;
use serde::Deserialize;
use toml;

use crate::config::{PathManager, THEME_FILE};

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub theme_name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub licence: Option<String>,
    pub required_packages: Option<Vec<String>>,
    pub min_hyprland_version: Option<String>,
    pub hypr_configs: Option<Vec<PathBuf>>,
}

pub async fn read_themes(path_manager: &PathManager) -> Result<Vec<Theme>, anyhow::Error> {
    let mut themes = Vec::new();
    let mut theme_folders = tokio_stream::wrappers::ReadDirStream::new(
        tokio::fs::read_dir(path_manager.get_theme_dir()).await?,
    );

    // Iterate through each entry in the theme directory
    while let Some(entry) = theme_folders.try_next().await? {
        // Ensure the entry is a directory
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        // Parse the theme file
        let file = entry.path().join(THEME_FILE);
        let content = tokio::fs::read_to_string(file)
            .await
            .with_context(|| format!("Failed to read theme file: {}", entry.path().display()))?;
        let theme: Theme = toml::from_str(&content)
            .with_context(|| format!("Failed to parse theme file: {}", entry.path().display()))?;
        themes.push(theme);
    }

    Ok(themes)
}
