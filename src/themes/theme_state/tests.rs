#[cfg(test)]
mod tests {
    use crate::themes::theme_state::HyprkitState;

    use super::*;
    use tempfile::TempDir;

    /// Test default state
    #[test]
    fn test_default_state() {
        let state = HyprkitState::default();
        assert!(state.active_theme.is_none());
        assert!(state.backup_stack.is_empty());
        assert_eq!(state.id, 0);
        assert_eq!(state.version, "1.0");
    }

    /// Test theme push
    #[test]
    fn test_state_push() {
        let mut state = HyprkitState::default();
        assert_eq!(state.backup_count(), 0);
        
        state.push_backup(
            "test".to_string(),
            "Test backup".to_string(),
            vec!["file1.conf".to_string()]
        ).unwrap();
        
        assert_eq!(state.backup_count(), 1);
        assert_eq!(state.id, 1);
    }

    /// Test saving and loading state
    #[test]
    fn test_state_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let state_file_path = temp_dir.path().join("test_state.json");
       
        let mut state = HyprkitState::default();
        state.set_active_theme(Some("dark".to_string())).unwrap();
        state.push_backup(
            "test".to_string(),
            "Test backup".to_string(),
            vec!["file1.conf".to_string()]
        ).unwrap();
        
        // Save to the temporary file
        state.save_to_path(state_file_path.clone()).unwrap();

        // Load from the same temporary file
        let loaded_state = HyprkitState::load_from_path(state_file_path).unwrap();
        assert_eq!(loaded_state.active_theme, Some("dark".to_string()));
        assert_eq!(loaded_state.backup_count(), 1);
    }
}