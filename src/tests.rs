use tempfile::TempDir;
use std::{fs, path::PathBuf};

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::file_ops::*;

    // struct TestEnv {
    //     config_dir: PathBuf,
    //     hyprkit_dir: PathBuf,
    //     themes_dir: PathBuf,
    // }

    // fn setup() -> TestEnv {
    //     let temp_dir = TempDir::new().unwrap();
    //     let config_dir = temp_dir.path().join("config");
    //     let hyprkit_dir = config_dir.join("hyprkit");
    //     let themes_dir = hyprkit_dir.join("themes");

    //     fs::create_dir_all(&config_dir).unwrap();
    //     fs::create_dir_all(&hyprkit_dir).unwrap();
    //     fs::create_dir_all(&themes_dir).unwrap();

    //     TestEnv {
    //         config_dir,
    //         hyprkit_dir,
    //         themes_dir,
    //     }
    // }

    // #[test]
    // fn test_fetch_available_themes_empty() {
    //     let env = setup();
    //     let themes = fetch_dir_folders(&env.themes_dir, &[]).unwrap();
    //     assert!(themes.is_empty());
    // }

    // #[test]
    // fn test_fetch_available_themes_with_themes() {
    //     let env = setup();
    //     let theme_names = vec!["theme1", "theme2", "theme3"];

    //     for theme in &theme_names {
    //         fs::create_dir_all(env.themes_dir.join(theme)).unwrap();
    //     }

    //     let themes = fetch_dir_folders(&env.themes_dir, &[]).unwrap();
    //     assert_eq!(themes.len(), theme_names.len());
    //     for theme in &theme_names {
    //         assert!(themes.contains(&theme.to_string()));
    //     }
    // }

    // #[test]
    // fn test_valid_dir_checks() {
    //     let env = setup();

    //     // Valid directory
    //     assert!(check_valid_dir(&env.themes_dir).is_ok());

    //     // Non-existent directory
    //     let non_existent_dir = env.config_dir.join("nonexistent");
    //     assert!(check_valid_dir(&non_existent_dir).is_err());

    //     // Path that is not a directory (create a file)
    //     let file_path = env.config_dir.join("file.txt");
    //     fs::write(&file_path, "test").unwrap();
    //     assert!(check_valid_dir(&file_path).is_err());
    // }
}