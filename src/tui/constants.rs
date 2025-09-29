// TUI Information strings
pub const NO_PACKAGES_FOUND: &str = "No packages found in";
pub const AVAILABLE_PACKAGES_LIST: &str = "Available Packages:";
pub const DOTFILES_DIR: &str = "Dotfiles directory:";
pub const TARGET_DIR: &str = "Target directory:";
pub const INSTALLING_PACKAGE: &str = "Installing package: ";
pub const UNINSTALLING_PACKAGE: &str = "Uninstalling package: ";
pub const BUILDING_THEME: &str = "Building theme...";
pub const THEME_BUILT_SUCCESS: &str = "✓ Successfully built theme: ";

// TUI Prompts and Help
pub const CONFIG_FOLDERS_PROMPT: &str = "Config folders:";
pub const SELECT_THEME_PROMPT: &str = "Select a theme to install:";
pub const MULTISELECT_HELP: &str = "Use spacebar to select/deselect, enter to confirm";
pub const SELECT_HELP: &str = "Use arrow keys to navigate, enter to confirm";
pub const NO_FOLDERS_SELECTED: &str = "No folders selected, cancelling theme creation.";
pub const YES_OPTION: &str = "Yes";
pub const NO_OPTION: &str = "No";

// Error strings specific to TUI
pub const ERROR_THEME_EXISTS: &str = "Theme already exists.";
pub const ERROR_FAILED_TO_REMOVE_THEME: &str = "Failed to remove existing active theme.";
pub const ERROR_FAILED_TO_CREATE_THEME_DIR: &str = "Failed to create theme directory.";
pub const ERROR_FAILED_TO_COPY_THEME: &str = "Failed to copy theme to active directory.";
pub const ERROR_FAILED_TO_GET_FOLDER_SELECTION: &str = "Failed to get folder selection";
pub const ERROR_FAILED_TO_GET_THEME_SELECTION: &str = "Failed to get theme selection";
pub const ERROR_FAILED_TO_GET_INSTALL_CONFIRMATION: &str = "Failed to get install confirmation";