# Auroran Neon Waybar Custom Layout

## Overview
This is a custom Waybar layout that implements a unique workspace arrangement with a diamond separator in the center, following the Auroran Neon theme aesthetic.

## Features
- **Neon Theme**: Full Auroran Neon color palette with gradients and glow effects
- **Responsive Design**: Hover effects and smooth transitions
- **Worspace Crystals**: Squares are boring! Use beautiful crystals for workplaces

## Color Palette
- **Cyber Green**: `#00FF88` - Primary accent color
- **Electric Blue**: `#00D4FF` - Secondary accent color
- **Neon Purple**: `#8A2BE2` - Hover and special states
- **Hot Pink**: `#FF1493` - Urgent and warning states
- **Cyber Orange**: `#FF6B35` - System information modules
- **Dark Background**: `rgba(13, 17, 23, 0.9)` - Main background
- **Module Background**: `rgba(28, 33, 40, 0.8)` - Module containers

## Files
- `config` - Main Waybar configuration (JSON)
- `style.css` - Complete styling with neon theme
- `../assets/diamond.svg` - Custom diamond separator graphic
- `demo.sh` - Demonstration script
- `config.backup` - Backup of original configuration

## Module Configuration

### Left Workspaces (1-5)
```json
"hyprland/workspaces#left": {
    "persistent-workspaces": {
        "1": [], "2": [], "3": [], "4": [], "5": []
    }
}
```

### Right Workspaces (6-10)
```json
"hyprland/workspaces#right": {
    "persistent-workspaces": {
        "6": [], "7": [], "8": [], "9": [], "10": []
    }
}
```

### Diamond Separator
```json
"custom/diamond": {
    "format": " ",
    "tooltip": false
}
```

## Installation
1. Copy this entire folder to your waybar config directory
2. Restart waybar: `pkill waybar && waybar &`
3. Enjoy your new neon desktop!

## Customization
- Modify workspace numbers in `persistent-workspaces` sections
- Change the diamond SVG for different center graphics
- Adjust colors in the CSS variables
- Add more modules to left/right sections as needed

## Dependencies
- Waybar (latest version recommended)
- Hyprland window manager
- Font Awesome 6 (for icons)
- Comfortaa font (for text)

---
*Part of the Auroran Neon theme collection for Hyprland*
