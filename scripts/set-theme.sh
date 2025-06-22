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
    
    # Restore each backed up configuration
    for item in "backup/$latest_backup"/*; do
        if [ -d "$item" ]; then
            config_name=$(basename "$item")
            echo "Restoring $config_name..."
            
            # Remove current config and restore backup
            if [ -e "$HOME/.config/$config_name" ]; then
                rm -rf "$HOME/.config/$config_name"
            fi
            cp -r "$item" "$HOME/.config/"
        fi
    done

    # clean up the current backup folder
    rm -rf backup/$latest_backup
    
    echo "Backup restored successfully"
    hyprctl reload
}

link_theme() {
    THEME_DIR="$1"
    # copy the theme to the hyperland config directory
    for file in current/*; do
        # check if a folder already exists in the config directory
        if [ -d "$HOME/.config/$(basename "$file")" ]; then
            # move the existing folder to the backup directory
            mv "$HOME/.config/$(basename "$file")" "$BACKUP_DIR/"
        fi

        if [[ "$(basename "$file")" != "theme.toml" ]]; then
            ln -sf "$THEME_DIR/$( basename "$file")" "$HOME/.config/$(basename "$file")"
        fi
    done
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
if [ ! -d "./themes/$1" ]; then
    echo "Theme '$1' does not exist."
    exit 1
fi

THEME_NAME="$1"
APP_DIR="$(dirname "$(dirname "$(realpath "$0")")")"

# check if 'current' symlink exists
if [ -L "current" ]; then
    # remove the existing symlink
    rm current
fi

# create a new symlink to the selected theme
ln -s "./themes/$THEME_NAME" current

# Get current datetime for backup folder name
BACKUP_DATE=$(date +"%Y%m%d_%H%M%S")
BACKUP_DIR="backup/backup_$BACKUP_DATE"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Link the theme files to the hyperland config directory
link_theme "$APP_DIR/current"

hyprctl reload