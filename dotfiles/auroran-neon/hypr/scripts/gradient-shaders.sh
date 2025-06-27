#!/bin/bash

# ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
# ┃          Dynamic Gradient Shader System (Border-Matched)    ┃
# ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

# Dynamically switches global shaders based on focused window to match border colors

SHADER_DIR="$HOME/.config/hypr/shaders"

# App to shader mapping (matches border color themes)
declare -A APP_SHADERS=(
    # 💻 CODING - Cyber Green
    ["code"]="cyber_green_gradient.frag"
    ["Code"]="cyber_green_gradient.frag"
    ["codium"]="cyber_green_gradient.frag"
    ["VSCodium"]="cyber_green_gradient.frag"
    ["neovim"]="cyber_green_gradient.frag"
    ["nvim"]="cyber_green_gradient.frag"
    ["vim"]="cyber_green_gradient.frag"
    ["jetbrains-.*"]="cyber_green_gradient.frag"
    ["sublime_text"]="cyber_green_gradient.frag"
    
    # 🌐 BROWSERS - Electric Blue
    ["firefox"]="electric_blue_gradient.frag"
    ["Firefox"]="electric_blue_gradient.frag"
    ["chromium"]="electric_blue_gradient.frag"
    ["Chromium"]="electric_blue_gradient.frag"
    ["brave-browser"]="neon_pink_gradient.frag"
    ["Brave-browser"]="neon_pink_gradient.frag"
    ["LibreWolf"]="electric_blue_gradient.frag"
    
    # 📁 FILE MANAGERS - Auroran Gold
    ["thunar"]="auroran_gold_gradient.frag"
    ["Thunar"]="auroran_gold_gradient.frag"
    ["nemo"]="auroran_gold_gradient.frag"
    ["Nemo"]="auroran_gold_gradient.frag"
    ["nautilus"]="auroran_gold_gradient.frag"
    ["Nautilus"]="auroran_gold_gradient.frag"
    ["dolphin"]="auroran_gold_gradient.frag"
    ["Dolphin"]="auroran_gold_gradient.frag"
    
    # 💬 COMMUNICATION - Neon Purple
    ["discord"]="neon_purple_gradient.frag"
    ["Discord"]="neon_purple_gradient.frag"
    ["telegram"]="neon_purple_gradient.frag"
    ["Telegram"]="neon_purple_gradient.frag"
    ["slack"]="neon_purple_gradient.frag"
    ["Slack"]="neon_purple_gradient.frag"
    ["signal"]="neon_purple_gradient.frag"
    ["Signal"]="neon_purple_gradient.frag"
    
    # 🎮 GAMING - Neon Pink
    ["steam"]="neon_pink_gradient.frag"
    ["Steam"]="neon_pink_gradient.frag"
    ["lutris"]="neon_pink_gradient.frag"
    ["Lutris"]="neon_pink_gradient.frag"
    ["heroic"]="neon_pink_gradient.frag"
    ["Heroic"]="neon_pink_gradient.frag"
    ["bottles"]="neon_pink_gradient.frag"
    ["Bottles"]="neon_pink_gradient.frag"
    
    # 🎵 MEDIA - Cyber Orange
    ["vlc"]="cyber_orange_gradient.frag"
    ["VLC"]="cyber_orange_gradient.frag"
    ["mpv"]="cyber_orange_gradient.frag"
    ["MPV"]="cyber_orange_gradient.frag"
    ["celluloid"]="cyber_orange_gradient.frag"
    ["Celluloid"]="cyber_orange_gradient.frag"
    ["spotify"]="cyber_orange_gradient.frag"
    ["Spotify"]="cyber_orange_gradient.frag"
    
    # 🖥️ TERMINALS - Electric Cyan
    ["kitty"]="electric_cyan_gradient.frag"
    ["Kitty"]="electric_cyan_gradient.frag"
    ["alacritty"]="neon_pink_gradient.frag"
    ["Alacritty"]="neon_pink_gradient.frag"
    ["wezterm"]="electric_cyan_gradient.frag"
    ["WezTerm"]="electric_cyan_gradient.frag"
    ["foot"]="electric_cyan_gradient.frag"
    ["Foot"]="electric_cyan_gradient.frag"
)

# Get active window class safely
get_active_window_class() {
    hyprctl activewindow -j 2>/dev/null | jq -r '.class' 2>/dev/null || echo ""
}

# Apply shader based on window class
apply_matching_shader() {
    local window_class="$1"
    local shader_file=""
    
    # Find matching shader for the window class
    for app_pattern in "${!APP_SHADERS[@]}"; do
        if [[ "$window_class" =~ $app_pattern ]]; then
            shader_file="${APP_SHADERS[$app_pattern]}"
            break
        fi
    done
    
    # Apply the shader or disable if no match
    if [[ -n "$shader_file" && -f "$SHADER_DIR/$shader_file" ]]; then
        echo "🎨 Applying $(basename "$shader_file" .frag) for: $window_class"
        hyprctl keyword decoration:screen_shader "$SHADER_DIR/$shader_file"
    else
        echo "🔄 No shader for: $window_class (neutral mode)"
        hyprctl keyword decoration:screen_shader ""
    fi
}

# Monitor window focus and apply shaders dynamically
monitor_and_apply() {
    echo "🌈 Starting Dynamic Gradient Shader System..."
    echo "📊 Monitoring window focus for automatic gradient matching..."
    
    # Handle initial window
    local active_window=$(get_active_window_class)
    if [[ -n "$active_window" && "$active_window" != "null" ]]; then
        apply_matching_shader "$active_window"
    fi
    
    # Monitor for window focus events
    socat -u UNIX-CONNECT:/tmp/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock - 2>/dev/null | \
    while read -r line; do
        if [[ "$line" =~ ^activewindow ]]; then
            local window_class=$(echo "$line" | sed 's/^activewindow>>//' | cut -d',' -f1)
            if [[ -n "$window_class" ]]; then
                apply_matching_shader "$window_class"
            fi
        fi
    done
}

# Show current status
show_status() {
    echo "🌈 Dynamic Gradient Shader System Status"
    echo "========================================"
    
    local active_window=$(get_active_window_class)
    echo "🪟 Active Window: $active_window"
    
    local current_shader=$(hyprctl getoption decoration:screen_shader -j 2>/dev/null | jq -r '.str' 2>/dev/null)
    if [[ "$current_shader" == "[[EMPTY]]" || -z "$current_shader" ]]; then
        echo "🎨 Current Shader: None (neutral mode)"
    else
        echo "🎨 Current Shader: $(basename "$current_shader")"
    fi
}

# Main command handling
case "${1:-help}" in
    "monitor")
        monitor_and_apply
        ;;
    "apply")
        if [[ -n "$2" ]]; then
            apply_matching_shader "$2"
        else
            echo "❌ Usage: $0 apply <window_class>"
        fi
        ;;
    "test")
        active_window=$(get_active_window_class)
        if [[ -n "$active_window" && "$active_window" != "null" ]]; then
            echo "🧪 Testing gradient shader for: $active_window"
            apply_matching_shader "$active_window"
        else
            echo "❌ Could not detect active window"
        fi
        ;;
    "status")
        show_status
        ;;
    "off")
        echo "🔄 Disabling gradient shaders..."
        hyprctl keyword decoration:screen_shader ""
        ;;
    *)
        echo "🌈 Dynamic Gradient Shader System"
        echo "================================="
        echo ""
        echo "🎨 Automatically applies gradient tinting that matches"
        echo "   per-app border colors by switching shaders dynamically."
        echo ""
        echo "📋 Commands:"
        echo "   monitor  - Start monitoring and applying shaders"
        echo "   apply    - Apply shader to specific window class"
        echo "   test     - Test shader on current active window"
        echo "   status   - Show current shader status"
        echo "   off      - Disable all gradient shaders"
        echo ""
        echo "💡 Examples:"
        echo "   $0 monitor             # Start dynamic monitoring"
        echo "   $0 test                # Test current window"
        echo "   $0 apply firefox       # Apply blue gradient to Firefox"
        ;;
esac
