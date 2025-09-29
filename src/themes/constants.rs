// Theme operation strings
pub const ACTIVE_DIR: &str = "active";

// Theme messages
pub const THEME_INSTALLED_SUCCESS: &str = "✓ Theme '{}' installed successfully";
pub const THEME_UNINSTALLED_SUCCESS: &str = "✓ Theme '{}' uninstalled successfully";
pub const BACKING_UP_CURRENT_STATE: &str = "Backing up current state...";
pub const UNDOING_OPERATION: &str = "Undoing: {}";
pub const UNDO_SUCCESS: &str = "✓ Successfully undid last operation";
pub const RESET_SUCCESS: &str = "✓ Successfully reset to original configuration";

// Undo/Reset messages
pub const NO_OPERATIONS_TO_UNDO: &str = "No operations to undo";
pub const NO_BACKUPS_TO_RESET: &str = "No backups found - nothing to reset";
pub const RESETTING_TO_ORIGINAL: &str = "Resetting to original configuration...";

// Backup descriptions
pub const BACKUP_BEFORE_SWITCH: &str = "Backup before switching from theme '{}'";
pub const ORIGINAL_BACKUP: &str = "original";

// State file and directory names
pub const STATE_FILE: &str = "state.json";
pub const BACKUPS_DIR: &str = "backups";
pub const THEMES_DIR: &str = "themes";

// State management messages
pub const FAILED_TO_READ_STATE_FILE: &str = "Failed to read state file";
pub const FAILED_TO_PARSE_STATE_JSON: &str = "Failed to parse state file JSON";
pub const FAILED_TO_CREATE_HYPRKIT_DIR: &str = "Failed to create hyprkit directory";
pub const FAILED_TO_SERIALIZE_STATE: &str = "Failed to serialize state to JSON";
pub const FAILED_TO_WRITE_STATE_FILE: &str = "Failed to write state file";

// Version and defaults
pub const STATE_VERSION: &str = "1.0";
pub const BACKUP_DIR_PREFIX: &str = "backup";
pub const BACKUP_ID_FORMAT: &str = "{:03}_{}";