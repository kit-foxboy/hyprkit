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
    CURRENT_LINK="$HOME/.config/HyprKit/current"
    if [ -L "$CURRENT_LINK" ]; then
        # Get the target of the symlink and extract theme name
        target=$(readlink "$CURRENT_LINK")
        theme_name=$(basename "$target")
        echo "Active theme: $theme_name"
    else
        echo "No active theme (current symlink not found)"
    fi
}

# Function to restore most recent backup
restore_backup() {
    BACKUP_DIR="$HOME/.config/HyprKit"
    
    if [ ! -d "$BACKUP_DIR" ]; then
        echo "No HyprKit backup directory found at $BACKUP_DIR"
        exit 1
    fi
    
    # Find the most recent backup directory
    latest_backup=$(ls -1t "$BACKUP_DIR" | grep "backup_" | head -n 1)
    
    if [ -z "$latest_backup" ]; then
        echo "No backups found in $BACKUP_DIR"
        exit 1
    fi
    
    echo "Restoring backup: $latest_backup"
    
    # Remove current symlink if it exists
    CURRENT_LINK="$HOME/.config/HyprKit/current"
    if [ -L "$CURRENT_LINK" ]; then
        rm "$CURRENT_LINK"
    fi
    
    # Clear out current .config contents (except HyprKit folder)
    echo "Clearing current configuration..."
    for item in "$HOME/.config"/*; do
        if [ -e "$item" ]; then
            item_name=$(basename "$item")
            if [[ "$item_name" != "HyprKit" ]]; then
                echo "Removing current $item_name..."
                rm -rf "$item"
            fi
        fi
    done
    
    # Restore all backed up configuration
    echo "Restoring complete configuration..."
    for item in "$BACKUP_DIR/$latest_backup"/*; do
        if [ -e "$item" ]; then
            item_name=$(basename "$item")
            # Skip the theme state file during this copy
            if [[ "$item_name" != ".hyprkit_theme_state" ]]; then
                echo "Restoring $item_name..."
                cp -r "$item" "$HOME/.config/"
            fi
        fi
    done
    
    # Restore the theme state if it was backed up
    if [ -f "$BACKUP_DIR/$latest_backup/.hyprkit_theme_state" ]; then
        theme_target=$(cat "$BACKUP_DIR/$latest_backup/.hyprkit_theme_state")
        if [ -d "$theme_target" ]; then
            echo "Restoring theme state: $(basename "$theme_target")"
            ln -s "$theme_target" "$HOME/.config/HyprKit/current"
        else
            echo "Warning: Backed up theme no longer exists: $theme_target"
            echo "No theme is currently active"
        fi
    else
        echo "No theme state found in backup (pre-HyprKit backup)"
    fi

    # Clean up the restored backup folder
    rm -rf "$BACKUP_DIR/$latest_backup"
    
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
    CURRENT_LINK="$HOME/.config/HyprKit/current"
    
    # Symlink all folders in the current theme to .config, overwriting existing
    for file in "$CURRENT_LINK"/*; do
        if [ -d "$file" ] && [[ "$(basename "$file")" != "theme.toml" ]]; then
            config_name=$(basename "$file")
            target_path="$HOME/.config/$config_name"
            
            # Remove existing config if it exists (file or directory)
            if [ -e "$target_path" ] || [ -L "$target_path" ]; then
                echo "Removing existing $config_name..."
                rm -rf "$target_path"
            fi
            
            echo "Linking $config_name..."
            ln -sf "$THEME_DIR/$(basename "$file")" "$target_path"
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
BACKUP_DIR="$HOME/.config/HyprKit/backup_$BACKUP_DATE"
CURRENT_LINK="$HOME/.config/HyprKit/current"

# Create HyprKit directory and backup directory
mkdir -p "$HOME/.config/HyprKit"
mkdir -p "$BACKUP_DIR"

# Backup ALL config folders (complete snapshot)
echo "Creating complete backup of ~/.config..."
for item in "$HOME/.config"/*; do
    if [ -e "$item" ]; then
        item_name=$(basename "$item")
        # Skip our own backup directory to avoid recursion
        if [[ "$item_name" != "HyprKit" ]]; then
            echo "Backing up $item_name..."
            cp -r "$item" "$BACKUP_DIR/"
        fi
    fi
done

# Backup the current symlink state if it exists
if [ -L "$CURRENT_LINK" ]; then
    current_target=$(readlink "$CURRENT_LINK")
    echo "$current_target" > "$BACKUP_DIR/.hyprkit_theme_state"
    echo "Backed up current theme state: $(basename "$current_target")"
    # Remove the existing symlink
    rm "$CURRENT_LINK"
fi

# Create new symlink to the selected theme
ln -s "$APP_DIR/dotfiles/$THEME_NAME" "$CURRENT_LINK"

# Link the new theme files to the config directory
link_theme "$APP_DIR/dotfiles/$THEME_NAME"

# Ask user about logout instead of just reloading
ask_logout