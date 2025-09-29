// Directory names
pub const HYPR_DIR: &str = "hypr";
pub const HYPRKIT_DIR: &str = "hyprkit";
pub const THEMES_DIR: &str = "themes";
pub const BACKUPS_DIR: &str = "backups";
pub const STATE_FILE: &str = "state.json";

// File operation strings
pub const FAILED_TO_READ_DIRECTORY: &str = "Failed to read directory";

// Stow command strings
pub const STOW_COMMAND: &str = "stow";
pub const STOW_ARG_DIR: &str = "--dir";
pub const STOW_ARG_TARGET: &str = "--target";
pub const STOW_ARG_DELETE: &str = "--delete";
pub const FAILED_TO_EXECUTE_STOW: &str = "Failed to execute stow command. Make sure stow is installed.";

// Error strings specific to file operations
pub const ERROR_CONFIG_DIR_NOT_FOUND: &str = "Failed to get config directory";
pub const ERROR_HYPRKIT_DIR_NOT_FOUND: &str = "Failed to get or create .hyprkit directory";