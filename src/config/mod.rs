mod paths;

pub use paths::*;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Serialize, Deserialize};

pub const THEME_FILE: &str = "theme.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub theme_dir: PathBuf,
    pub config_dir: PathBuf,
    pub user_override_file: PathBuf
}

impl Config {
    pub fn new() -> Result<Self> {
        // TODO: Implement loading from XDG dirs or default locations
        todo!()
    }
}