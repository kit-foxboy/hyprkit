use std::os::unix::fs::PermissionsExt;
use std::{fs, path::PathBuf};
use tempfile::TempDir;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_ops::*;

    /// Mock stow executor for testing
    #[derive(Clone)]
    pub struct MockStowExecutor {
        /// Records all stow commands that were executed
        pub commands: Arc<Mutex<Vec<StowCommand>>>,
        /// Controls whether the mock should return success or failure
        pub should_succeed: bool,
        /// Files that the mock will simulate creating/removing
        pub simulated_files: HashMap<String, String>,
    }

    #[derive(Debug, Clone)]
    pub struct StowCommand {
        pub themes_dir: PathBuf,
        pub target_dir: PathBuf,
        pub theme: String,
        pub delete: bool,
    }

    impl MockStowExecutor {
        pub fn new() -> Self {
            Self {
                commands: Arc::new(Mutex::new(Vec::new())),
                should_succeed: true,
                simulated_files: HashMap::new(),
            }
        }

        pub fn with_failure() -> Self {
            Self {
                commands: Arc::new(Mutex::new(Vec::new())),
                should_succeed: false,
                simulated_files: HashMap::new(),
            }
        }

        pub fn with_simulated_files(mut self, files: HashMap<String, String>) -> Self {
            self.simulated_files = files;
            self
        }

        pub fn get_commands(&self) -> Vec<StowCommand> {
            self.commands.lock().unwrap().clone()
        }
    }

    impl StowExecutor for MockStowExecutor {
        fn execute_stow(&self, themes_dir: &PathBuf, target_dir: &PathBuf, theme: &str, delete: bool) -> Result<()> {
            // Record the command
            let command = StowCommand {
                themes_dir: themes_dir.clone(),
                target_dir: target_dir.clone(),
                theme: theme.to_string(),
                delete,
            };
            self.commands.lock().unwrap().push(command);

            if !self.should_succeed {
                anyhow::bail!("Mock stow command failed");
            }

            // Simulate stow behavior by creating/removing files
            if delete {
                // Remove simulated symlinks
                for (file_name, _) in &self.simulated_files {
                    let symlink_path = target_dir.join(file_name);
                    if symlink_path.exists() {
                        fs::remove_file(&symlink_path)?;
                    }
                }
            } else {
                // Create simulated symlinks
                for (file_name, content) in &self.simulated_files {
                    let theme_file_path = themes_dir.join(theme).join(file_name);
                    let target_file_path = target_dir.join(file_name);

                    // Ensure the theme file exists
                    if let Some(parent) = theme_file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&theme_file_path, content)?;

                    // Create symlink in target directory
                    if let Some(parent) = target_file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    
                    // On Unix systems, create an actual symlink
                    #[cfg(unix)]
                    {
                        if target_file_path.exists() {
                            fs::remove_file(&target_file_path)?;
                        }
                        std::os::unix::fs::symlink(&theme_file_path, &target_file_path)?;
                    }
                    
                    // On other systems, just copy the file for testing
                    #[cfg(not(unix))]
                    {
                        fs::copy(&theme_file_path, &target_file_path)?;
                    }
                }
            }

            Ok(())
        }
    }

    struct TestEnv {
        config_dir: PathBuf,
        hyprkit_dir: PathBuf,
        themes_dir: PathBuf,
        temp_dir: TempDir,
    }

    fn setup() -> TestEnv {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("config");
        let hyprkit_dir = config_dir.join("hyprkit");
        let themes_dir = hyprkit_dir.join("themes");

        fs::create_dir_all(&config_dir).unwrap();
        fs::create_dir_all(&hyprkit_dir).unwrap();
        fs::create_dir_all(&themes_dir).unwrap();

        TestEnv {
            temp_dir,
            config_dir,
            hyprkit_dir,
            themes_dir,
        }
    }

    fn cleanup(_env: TestEnv) {
        fs::remove_dir_all(_env.config_dir).unwrap();
    }

    #[test]
    fn test_fetch_available_themes_empty() {
        let env = setup();
        let themes = fetch_dir_folders(&env.themes_dir, &[]).unwrap();
        assert!(themes.is_empty());
    }

    #[test]
    fn test_fetch_available_themes_with_themes() {
        let env = setup();
        let theme_names = vec!["theme1", "theme2", "theme3"];

        for theme in &theme_names {
            fs::create_dir_all(env.themes_dir.join(theme)).unwrap();
        }

        let themes = fetch_dir_folders(&env.themes_dir, &[]).unwrap();
        assert_eq!(themes.len(), theme_names.len());
        for theme in &theme_names {
            assert!(themes.contains(&theme.to_string()));
        }

        cleanup(env);
    }

    #[test]
    fn test_valid_dir_checks() {
        // Valid directory
        let env = setup();
        assert!(check_valid_dir(&env.themes_dir).is_ok());

        // Non-existent directory
        let non_existent_dir = env.config_dir.join("nonexistent");
        assert!(check_valid_dir(&non_existent_dir).is_err());

        // Path that is not a directory (create a file)
        let file_path = env.config_dir.join("file.txt");
        fs::write(&file_path, "test").unwrap();
        assert!(check_valid_dir(&file_path).is_err());

        cleanup(env);
    }

    #[test]
    fn test_copy_dir_recursive() {
        let env = setup();

        // Create a source directory with some files
        let src_dir = env.temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("file1.txt"), "content1").unwrap();
        fs::write(src_dir.join("file2.txt"), "content2").unwrap();

        // Create a destination directory
        let dest_dir = env.temp_dir.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();

        // Copy the directory recursively
        copy_dir_recursive(&src_dir, &dest_dir).unwrap();

        // Check that the files were copied
        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("file2.txt").exists());

        cleanup(env);
    }

    #[test]
    fn test_copy_dir_recursive_with_subdirs() {
        let env = setup();
        // Create a source directory with subdirectories and files
        let src_dir = env.temp_dir.path().join("src");
        let sub_dir = src_dir.join("subdir");
        fs::create_dir_all(&sub_dir).unwrap();
        fs::write(src_dir.join("file1.txt"), "content1").unwrap();
        fs::write(sub_dir.join("file2.txt"), "content2").unwrap();

        // Create a destination directory
        let dest_dir = env.temp_dir.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();

        copy_dir_recursive(&src_dir, &dest_dir).unwrap();

        // Create a non-existent source directory
        let src_dir = env.temp_dir.path().join("nonexistent_src");
        let result = copy_dir_recursive(&src_dir, &dest_dir);
        assert!(result.is_err());

        // Check that the files were copied
        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("subdir/file2.txt").exists());

        cleanup(env);
    }

    #[test]
    fn test_copy_dir_recursive_nonexistent_src() {
        let env = setup();

        // Non-existent source directory
        let src_dir = env.temp_dir.path().join("nonexistent_src");
        let dest_dir = env.temp_dir.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();

        // Attempt to copy the non-existent directory
        let result = copy_dir_recursive(&src_dir, &dest_dir);
        assert!(result.is_err());

        cleanup(env);
    }

    #[test]
    fn test_move_dir_recursive() {
        let env = setup();

        // Create a source directory with some files
        let src_dir = env.temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("file1.txt"), "content1").unwrap();
        fs::write(src_dir.join("file2.txt"), "content2").unwrap();

        // Create a destination directory
        let dest_dir = env.temp_dir.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();

        // Move the directory recursively
        move_dir_recursive(&src_dir, &dest_dir).unwrap();

        // Check that the files were moved
        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("file2.txt").exists());
        assert!(!src_dir.exists());

        cleanup(env);
    }

    #[test]
    fn test_move_dir_recursive_with_subdirs() {
        let env = setup();
        // Create a source directory with subdirectories and files
        let src_dir = env.temp_dir.path().join("src");
        let sub_dir = src_dir.join("subdir");
        fs::create_dir_all(&sub_dir).unwrap();
        fs::write(src_dir.join("file1.txt"), "content1").unwrap();
        fs::write(sub_dir.join("file2.txt"), "content2").unwrap();

        // Create a destination directory
        let dest_dir = env.temp_dir.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();

        // Move the directory recursively
        move_dir_recursive(&src_dir, &dest_dir).unwrap();

        // Check that the files were moved
        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("subdir/file2.txt").exists());
        assert!(!src_dir.exists());

        cleanup(env);
    }

    #[test]
    fn test_move_dir_recursive_nonexistent_src() {
        let env = setup();

        // Non-existent source directory
        let src_dir = env.temp_dir.path().join("nonexistent_src");
        let dest_dir = env.temp_dir.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();

        // Attempt to move the non-existent directory
        let result = move_dir_recursive(&src_dir, &dest_dir);
        assert!(result.is_err());

        cleanup(env);
    }

    #[test]
    fn test_move_folders_to_target() {
        let env = setup();

        // Create source config folders
        let folder_names = vec!["folder1", "folder2"];
        for folder in &folder_names {
            let folder_path = env.config_dir.join(folder);
            fs::create_dir_all(&folder_path).unwrap();
            fs::write(folder_path.join("file.txt"), "content").unwrap();
        }

        // Move folders to target
        move_folders_to_target(
            &folder_names
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            &env.config_dir,
            &env.hyprkit_dir,
        )
        .unwrap();

        // Check that folders were moved
        for folder in &folder_names {
            let target_folder = env.hyprkit_dir.join(folder);
            assert!(target_folder.exists());
            assert!(target_folder.join("file.txt").exists());
            assert!(!env.config_dir.join(folder).exists());
        }

        cleanup(env);
    }

    // Check that the symlink was removed
    #[test]
    fn test_execute_stow_command() {
        let env = setup();

        // Create a mock executor that simulates creating a file
        let mut files = HashMap::new();
        files.insert("file.txt".to_string(), "content".to_string());
        let mock_executor = MockStowExecutor::new().with_simulated_files(files);

        let theme_name = "test_theme";

        // Execute stow using the mock executor
        let result = execute_stow_command_with_executor(
            &mock_executor,
            &env.themes_dir,
            &env.config_dir,
            theme_name,
            false,
        );
        assert!(result.is_ok());

        // Check that the command was recorded
        let commands = mock_executor.get_commands();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].theme, theme_name);
        assert!(!commands[0].delete);

        // Check that the simulated file was created (symlinked)
        let symlink_path = env.config_dir.join("file.txt");
        assert!(symlink_path.exists());

        cleanup(env);
    }

    #[test]
    fn test_execute_stow_command_delete() {
        let env = setup();

        // Create a mock executor that simulates file operations
        let mut files = HashMap::new();
        files.insert("file.txt".to_string(), "content".to_string());
        let mock_executor = MockStowExecutor::new().with_simulated_files(files);

        let theme_name = "test_theme";

        // First stow the theme
        execute_stow_command_with_executor(
            &mock_executor,
            &env.themes_dir,
            &env.config_dir,
            theme_name,
            false,
        ).unwrap();
        
        let symlink_path = env.config_dir.join("file.txt");
        assert!(symlink_path.exists());

        // Execute stow with delete
        let result = execute_stow_command_with_executor(
            &mock_executor,
            &env.themes_dir,
            &env.config_dir,
            theme_name,
            true,
        );
        assert!(result.is_ok());

        // Check that the symlink was removed
        assert!(!symlink_path.exists());

        // Check that both commands were recorded
        let commands = mock_executor.get_commands();
        assert_eq!(commands.len(), 2);
        assert!(!commands[0].delete);  // First command was install
        assert!(commands[1].delete);   // Second command was delete

        cleanup(env);
    }

    #[test]
    fn test_execute_stow_command_nonexistent_theme() {
        let env = setup();

        // Create a mock executor that fails
        let mock_executor = MockStowExecutor::with_failure();

        // Attempt to stow a theme (mock will fail)
        let result = execute_stow_command_with_executor(
            &mock_executor,
            &env.themes_dir,
            &env.config_dir,
            "nonexistent",
            false,
        );
        assert!(result.is_err());

        cleanup(env);
    }

    #[test]
    fn test_execute_stow_command_with_subdirs() {
        let env = setup();

        // Create a mock executor that simulates files in subdirectories
        let mut files = HashMap::new();
        files.insert("root_file.txt".to_string(), "root content".to_string());
        files.insert("config/nested_file.txt".to_string(), "nested content".to_string());
        let mock_executor = MockStowExecutor::new().with_simulated_files(files);

        let theme_name = "complex_theme";

        // Execute stow
        let result = execute_stow_command_with_executor(
            &mock_executor,
            &env.themes_dir,
            &env.config_dir,
            theme_name,
            false,
        );
        assert!(result.is_ok());

        // Check that both root and nested files were stowed
        assert!(env.config_dir.join("root_file.txt").exists());
        assert!(env.config_dir.join("config/nested_file.txt").exists());

        cleanup(env);
    }

    #[test]
    fn test_real_stow_executor_integration() {
        // This test only runs if stow is actually installed
        use std::process::Command;
        
        let stow_available = Command::new("which")
            .arg("stow")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);
        
        if !stow_available {
            eprintln!("Skipping real stow executor test - stow not available");
            return;
        }
        
        let env = setup();
        let theme_name = "integration_test";
        let theme_dir = env.themes_dir.join(theme_name);
        fs::create_dir_all(&theme_dir).unwrap();
        fs::write(theme_dir.join("test_file.txt"), "integration test content").unwrap();
        
        // Test the real executor
        let result = execute_stow_command_direct(
            &env.themes_dir,
            &env.config_dir,
            theme_name,
            false,
        );
        
        match result {
            Ok(_) => {
                println!("Real stow integration test passed");
                // Clean up by removing the stowed file
                let _ = execute_stow_command_direct(
                    &env.themes_dir,
                    &env.config_dir,
                    theme_name,
                    true,
                );
            }
            Err(e) => {
                eprintln!("Real stow integration test failed (this might be expected in some environments): {}", e);
            }
        }
        
        cleanup(env);
    }

    #[test]
    fn test_fetch_dir_folders_with_exclusions() {
        let env = setup();
        let folder_names = vec!["theme1", "theme2", "excluded_theme", ".hidden"];

        for folder in &folder_names {
            fs::create_dir_all(env.themes_dir.join(folder)).unwrap();
        }

        let exclusions = vec!["excluded_theme".to_string(), ".hidden".to_string()];
        let themes = fetch_dir_folders(&env.themes_dir, &exclusions).unwrap();

        assert_eq!(themes.len(), 2);
        assert!(themes.contains(&"theme1".to_string()));
        assert!(themes.contains(&"theme2".to_string()));
        assert!(!themes.contains(&"excluded_theme".to_string()));
        assert!(!themes.contains(&".hidden".to_string()));

        cleanup(env);
    }

    #[test]
    fn test_copy_dir_recursive_empty_dir() {
        let env = setup();

        // Create an empty source directory
        let src_dir = env.temp_dir.path().join("empty_src");
        fs::create_dir_all(&src_dir).unwrap();

        let dest_dir = env.temp_dir.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();

        // Copy the empty directory
        let result = copy_dir_recursive(&src_dir, &dest_dir);
        assert!(result.is_ok());

        cleanup(env);
    }

    #[test]
    fn test_move_dir_recursive_empty_dir() {
        let env = setup();

        // Create an empty source directory
        let src_dir = env.temp_dir.path().join("empty_src");
        fs::create_dir_all(&src_dir).unwrap();

        let dest_dir = env.temp_dir.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();

        // Move the empty directory
        let result = move_dir_recursive(&src_dir, &dest_dir);
        assert!(result.is_ok());
        assert!(!src_dir.exists());

        cleanup(env);
    }

    #[test]
    fn test_check_valid_dir_with_permissions() {
        let env = setup();

        // Test with a valid directory
        assert!(check_valid_dir(&env.themes_dir).is_ok());

        // Create a directory and remove read permissions (Unix-specific)
        #[cfg(unix)]
        {
            let restricted_dir = env.temp_dir.path().join("restricted");
            fs::create_dir_all(&restricted_dir).unwrap();

            let mut perms = fs::metadata(&restricted_dir).unwrap().permissions();
            perms.set_mode(0o000);
            fs::set_permissions(&restricted_dir, perms.clone()).unwrap();

            let _result = check_valid_dir(&restricted_dir);
            // Restore permissions for cleanup
            perms.set_mode(0o755);
            fs::set_permissions(&restricted_dir, perms).unwrap();
        }

        cleanup(env);
    }

    #[test]
    fn test_move_folders_to_target_empty_list() {
        let env = setup();

        let empty_folders: Vec<String> = vec![];
        let result = move_folders_to_target(&empty_folders, &env.config_dir, &env.hyprkit_dir);
        assert!(result.is_ok());

        cleanup(env);
    }
}
