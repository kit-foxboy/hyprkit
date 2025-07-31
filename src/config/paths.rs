use std::path::PathBuf;

use tokio::fs::try_exists;

pub struct PathManager {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub theme_dir: PathBuf,
}

impl PathManager {

    pub async fn new() -> Result<Self, anyhow::Error> {
        
        let config_dir = PathBuf::from(std::env::var("XDG_CONFIG_HOME").unwrap_or_else({
            eprintln!("XDG_CONFIG_HOME environment variable not set, using default ~/.config");
            let home_dir = std::env::var("HOME").unwrap();
            |_| String::from(home_dir + "/.config")
        }) + "/Hyprkit/");

        let data_dir = PathBuf::from(std::env::var("XDG_DATA_HOME").unwrap_or_else({
            eprintln!("XDG_DATA_HOME environment variable not set, using default ~/.local/share");
            let home_dir = std::env::var("HOME").unwrap();
            |_| String::from(home_dir + "/.local/share")
        }) + "/Hyprkit/");

        let theme_dir = data_dir.join("themes/");

        // Create directories if they do not exist
        if !try_exists(&config_dir).await.unwrap_or(false) {
            std::fs::create_dir_all(&config_dir).map_err(|e| {
                anyhow::anyhow!("Failed to create config directory {}", e)
            })?;
        }
        if !try_exists(&data_dir).await.unwrap_or(false) {
            std::fs::create_dir_all(&data_dir).map_err(|e| {
                anyhow::anyhow!("Failed to create data directory {}", e)
            })?;
        }
        if !try_exists(&theme_dir).await.unwrap_or(false) {
            std::fs::create_dir_all(&theme_dir).map_err(|e| {
                anyhow::anyhow!("Failed to create theme directory {}", e)
            })?;
        }

        Ok(Self {
            config_dir,
            data_dir,
            theme_dir,
        })
    }

    pub fn get_config_dir(&self) -> &PathBuf {
        &self.config_dir
    }
    pub fn get_data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
    pub fn get_theme_dir(&self) -> &PathBuf {
        &self.theme_dir
    }
}