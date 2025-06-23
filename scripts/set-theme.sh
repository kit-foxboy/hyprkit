#!/bin/bash
# Sets the hyperland theme based on the provided argument.

# Function to show usage
show_usage() {
    echo "Usage: $0 <theme_name>"
    echo "       $0 --restore     # Restore most recent backup"
    echo "       $0 --active      # Show active theme name"
    echo "       $0 --help        # Show this help"
}

# Function to get active theme name
get_active_theme() {
    if [ -L "current" ]; then
        # Get the target of the symlink and extract theme name
        target=$(readlink current)
        theme_name=$(basename "$target")
        echo "Active theme: $theme_name"
    else
        echo "No active theme (current symlink not found)"
    fi
}

# Function to restore most recent backup
restore_backup() {
    if [ ! -d "backup" ]; then
        echo "No backup directory found"
        exit 1
    fi
    
    # Find the most recent backup directory
    latest_backup=$(ls -1t backup/ | head -n 1)
    
    if [ -z "$latest_backup" ]; then
        echo "No backups found"
        exit 1
    fi
    
    echo "Restoring backup: $latest_backup"
    
    # First, clean up current theme components to avoid conflicts
    if [ -L "current" ]; then
        echo "Cleaning up current theme components..."
        for file in current/*; do
            if [ -d "$file" ] && [[ "$(basename "$file")" != "theme.toml" ]]; then
                config_name=$(basename "$file")
                if [ -e "$HOME/.config/$config_name" ]; then
                    echo "Removing current $config_name..."
                    rm -rf "$HOME/.config/$config_name"
                fi
            fi
        done
        
        # Remove the current symlink
        rm current
    fi
    
    # Restore each backed up configuration
    for item in "backup/$latest_backup"/*; do
        if [ -d "$item" ]; then
            config_name=$(basename "$item")
            echo "Restoring $config_name..."
            cp -r "$item" "$HOME/.config/"
        fi
    done
    
    # Restore the theme state if it was backed up
    if [ -f "backup/$latest_backup/.hyprkit_theme_state" ]; then
        theme_target=$(cat "backup/$latest_backup/.hyprkit_theme_state")
        if [ -d "$theme_target" ]; then
            echo "Restoring theme state: $(basename "$theme_target")"
            ln -s "$theme_target" current
        else
            echo "Warning: Backed up theme no longer exists: $theme_target"
            echo "No theme is currently active"
        fi
    else
        echo "No theme state found in backup (pre-HyprKit backup)"
    fi

    # clean up the current backup folder
    rm -rf backup/$latest_backup
    
    echo "Backup restored successfully"
    
    # Ask about logout for backup restore too
    echo ""
    read -p "Would you like to log out to ensure all changes take effect? (y/N): " logout_choice
    
    case "$logout_choice" in
        [Yy]|[Yy][Ee][Ss])
            echo "Logging out in 3 seconds..."
            sleep 1
            echo "3..."
            sleep 1
            echo "2..."
            sleep 1
            echo "1..."
            sleep 1
            
            if command -v hyprctl >/dev/null 2>&1; then
                hyprctl dispatch exit
            elif command -v loginctl >/dev/null 2>&1; then
                loginctl terminate-user "$USER"
            elif command -v pkill >/dev/null 2>&1; then
                pkill -KILL -u "$USER"
            else
                echo "Could not find a logout command. Please log out manually."
            fi
            ;;
        *)
            echo "Skipping logout. You may need to log out manually for all changes to take effect."
            hyprctl reload
            ;;
    esac
}

link_theme() {
    THEME_DIR="$1"
    # Link the new theme to the hyperland config directory
    for file in current/*; do
        if [[ "$(basename "$file")" != "theme.toml" ]]; then
            config_name=$(basename "$file")
            echo "Linking $config_name..."
            ln -sf "$THEME_DIR/$(basename "$file")" "$HOME/.config/$config_name"
        fi
    done
}

# Function to ask user about logout
ask_logout() {
    echo ""
    echo "Theme applied successfully!"
    echo ""
    echo "Note: Some theme changes (especially autostart scripts and system-wide configs)"
    echo "require a full logout/login to take effect properly."
    echo ""
    read -p "Would you like to log out now? (y/N): " logout_choice
    
    case "$logout_choice" in
        [Yy]|[Yy][Ee][Ss])
            echo "Logging out in 3 seconds..."
            sleep 1
            echo "3..."
            sleep 1
            echo "2..."
            sleep 1
            echo "1..."
            sleep 1
            
            # Try different logout methods depending on what's available
            if command -v hyprctl >/dev/null 2>&1; then
                hyprctl dispatch exit
            elif command -v loginctl >/dev/null 2>&1; then
                loginctl terminate-user "$USER"
            elif command -v pkill >/dev/null 2>&1; then
                pkill -KILL -u "$USER"
            else
                echo "Could not find a logout command. Please log out manually."
            fi
            ;;
        *)
            echo "Skipping logout. You may need to log out manually for all changes to take effect."
            hyprctl reload
            ;;
    esac
}

# Handle command line arguments
case "$1" in
    --restore)
        restore_backup
        exit 0
        ;;
    --active)
        get_active_theme
        exit 0
        ;;
    --help)
        show_usage
        exit 0
        ;;
    "")
        show_usage
        exit 1
        ;;
    -*)
        echo "Unknown option: $1"
        show_usage
        exit 1
        ;;
esac

# check if the theme directory exists
if [ ! -d "./dotfiles/$1" ]; then
    echo "Theme '$1' does not exist."
    exit 1
fi

THEME_NAME="$1"
APP_DIR="$(dirname "$(dirname "$(realpath "$0")")")"

# Get current datetime for backup folder name
BACKUP_DATE=$(date +"%Y%m%d_%H%M%S")
BACKUP_DIR="backup/backup_$BACKUP_DATE"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Backup current theme configs
if [ -L "current" ]; then
    echo "Backing up current theme configurations..."
    for file in current/*; do
        if [ -d "$file" ] && [[ "$(basename "$file")" != "theme.toml" ]]; then
            config_name=$(basename "$file")
            if [ -d "$HOME/.config/$config_name" ]; then
                echo "Backing up $config_name..."
                mv "$HOME/.config/$config_name" "$BACKUP_DIR/"
            fi
        fi
    done
    
    # Backup the current symlink state
    current_target=$(readlink current)
    echo "$current_target" > "$BACKUP_DIR/.hyprkit_theme_state"
    echo "Backed up current theme state: $(basename "$current_target")"
    
    # Remove the existing symlink
    rm current
fi

# Create new symlink to the selected theme
ln -s "./dotfiles/$THEME_NAME" current

# Link the new theme files to the config directory
link_theme "$APP_DIR/current"

# Ask user about logout instead of just reloading
ask_logout