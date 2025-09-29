pub const DEFAULT_VERSION: &str = "1.0";
pub const STATE_FILE_NAME: &str = "state.json";
pub const BACKUPS_DIR_NAME: &str = "backups";
pub const THEMES_DIR_NAME: &str = "themes";
pub const BACKUP_FILE_FORMAT: &str = "backup_{}.json";

// Common error/context messages used in the module
pub const ERR_READ_STATE: &str = "Failed to read state file";
pub const ERR_PARSE_STATE: &str = "Failed to parse state file JSON";
pub const ERR_CREATE_HYPRKIT_DIR: &str = "Failed to create hyprkit directory";
pub const ERR_SERIALIZE_STATE: &str = "Failed to serialize state to JSON";
pub const ERR_WRITE_STATE: &str = "Failed to write state file";
