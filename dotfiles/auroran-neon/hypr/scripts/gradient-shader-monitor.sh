#!/bin/bash

# ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
# ┃           Auroran Neon Dynamic Shader Monitor            ┃
# ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

# Background service that monitors window focus changes and applies
# matching gradient shaders automatically

SHADER_DIR="$HOME/.config/hypr/shaders"
LOG_FILE="/tmp/auroran-shader-monitor.log"
LAST_WINDOW_CLASS=""

# Ensure we have required dependencies
command -v hyprctl >/dev/null 2>&1 || { echo "hyprctl not found"; exit 1; }

# App-to-shader mapping for gradient tinting
declare -A APP_SHADERS=(
    # 💻 CODING APPLICATIONS - Cyber Green
    ["code"]="cyber_green_gradient.frag"
    ["Code"]="cyber_green_gradient.frag"
    ["codium"]="cyber_green_gradient.frag"
    ["VSCodium"]="cyber_green_gradient.frag"
    ["neovim"]="cyber_green_gradient.frag"
    ["nvim"]="cyber_green_gradient.frag"
    ["vim"]="cyber_green_gradient.frag"
    ["sublime_text"]="cyber_green_gradient.frag"
    ["subl"]="cyber_green_gradient.frag"
    
    # 🌐 BROWSERS - Electric Blue
    ["firefox"]="electric_blue_gradient.frag"
    ["Firefox"]="electric_blue_gradient.frag"
    ["chromium"]="electric_blue_gradient.frag"
    ["Chromium"]="electric_blue_gradient.frag"
    ["brave-browser"]="electric_blue_gradient.frag"
    ["Brave-browser"]="electric_blue_gradient.frag"
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
    ["net.lutris.Lutris"]="neon_pink_gradient.frag"
    ["org.lutris.Lutris"]="neon_pink_gradient.frag"
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
    ["alacritty"]="electric_cyan_gradient.frag"
    ["Alacritty"]="electric_cyan_gradient.frag"
    ["wezterm"]="electric_cyan_gradient.frag"
    ["WezTerm"]="electric_cyan_gradient.frag"
    ["foot"]="electric_cyan_gradient.frag"
    ["Foot"]="electric_cyan_gradient.frag"
)

# Function to log with timestamp
log_message() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" >> "$LOG_FILE"
}

# Function to get active window class safely (without jq dependency)
get_active_window_class() {
    local window_info
    window_info=$(hyprctl activewindow 2>/dev/null)
    if [[ $? -eq 0 && -n "$window_info" ]]; then
        # Extract class using grep and sed (avoiding jq dependency)
        echo "$window_info" | grep "class:" | sed 's/.*class: \([^[:space:]]*\).*/\1/' | head -1
    else
        echo ""
    fi
}

# Function to apply shader for window class
apply_shader_for_class() {
    local window_class="$1"
    local shader_file=""
    
    # Skip if no window class or same as last
    if [[ -z "$window_class" || "$window_class" == "$LAST_WINDOW_CLASS" ]]; then
        return 0
    fi
    
    LAST_WINDOW_CLASS="$window_class"
    
    # Find matching shader for the window class
    for app_pattern in "${!APP_SHADERS[@]}"; do
        if [[ "$window_class" =~ ^${app_pattern}$ ]]; then
            shader_file="${APP_SHADERS[$app_pattern]}"
            break
        fi
    done
    
    # Apply the shader or disable if no match
    if [[ -n "$shader_file" && -f "$SHADER_DIR/$shader_file" ]]; then
        log_message "🎨 Applying shader: $shader_file for window: $window_class"
        hyprctl keyword decoration:screen_shader "$SHADER_DIR/$shader_file" >/dev/null 2>&1
    else
        # No specific shader, disable screen shader for neutral appearance
        log_message "🔄 Disabling shader for window: $window_class"
        hyprctl keyword decoration:screen_shader "" >/dev/null 2>&1
    fi
}

# Function to start monitoring
start_monitor() {
    log_message "🌈 Starting Auroran Neon Dynamic Shader Monitor..."
    log_message "📊 Monitoring window focus changes for automatic shader switching..."
    
    # Ensure shader directory exists
    if [[ ! -d "$SHADER_DIR" ]]; then
        log_message "❌ Shader directory not found: $SHADER_DIR"
        exit 1
    fi
    
    # Handle initial window
    local active_window
    active_window=$(get_active_window_class)
    if [[ -n "$active_window" ]]; then
        apply_shader_for_class "$active_window"
    fi
    
    # Monitor for window focus events using a polling approach (fallback for no socat)
    log_message "🔄 Starting window monitoring (polling mode)..."
    
    # Main monitoring loop with polling
    while true; do
        local current_window
        current_window=$(get_active_window_class)
        
        if [[ -n "$current_window" && "$current_window" != "$LAST_WINDOW_CLASS" ]]; then
            apply_shader_for_class "$current_window"
        fi
        
        # Poll every 500ms for responsiveness without being too resource-intensive
        sleep 0.5
    done
}

# Function to stop monitor
stop_monitor() {
    log_message "🛑 Stopping Auroran Neon Dynamic Shader Monitor..."
    
    # Kill any existing monitor processes
    pkill -f "auroran-shader-monitor" 2>/dev/null
    
    # Disable any active shader
    hyprctl keyword decoration:screen_shader "" >/dev/null 2>&1
    
    log_message "✅ Monitor stopped and shaders disabled"
}

# Function to check status
check_status() {
    if pgrep -f "auroran-shader-monitor" >/dev/null; then
        echo "🟢 Auroran Neon Shader Monitor is RUNNING"
        
        local active_window
        active_window=$(get_active_window_class)
        echo "🪟 Current Window: ${active_window:-"Unknown"}"
        
        local current_shader
        current_shader=$(hyprctl getoption decoration:screen_shader 2>/dev/null | grep -o '"[^"]*"' | sed 's/"//g' | head -1)
        if [[ -n "$current_shader" && "$current_shader" != "[[EMPTY]]" && "$current_shader" != "" ]]; then
            echo "🎨 Active Shader: $(basename "$current_shader")"
        else
            echo "🎨 Active Shader: None (neutral mode)"
        fi
        
        if [[ -f "$LOG_FILE" ]]; then
            echo "📝 Recent log entries:"
            tail -5 "$LOG_FILE" | sed 's/^/   /'
        fi
    else
        echo "🔴 Auroran Neon Shader Monitor is NOT RUNNING"
    fi
}

# Handle different commands
case "${1:-start}" in
    "start")
        # Stop any existing instance first
        stop_monitor
        
        # Start monitoring in background
        start_monitor &
        disown
        
        echo "🚀 Auroran Neon Dynamic Shader Monitor started in background"
        echo "📝 Logs: $LOG_FILE"
        ;;
    "stop")
        stop_monitor
        ;;
    "restart")
        stop_monitor
        sleep 1
        "$0" start
        ;;
    "status")
        check_status
        ;;
    "logs")
        if [[ -f "$LOG_FILE" ]]; then
            tail -20 "$LOG_FILE"
        else
            echo "📝 No log file found at: $LOG_FILE"
        fi
        ;;
    "test")
        # Test current window detection
        local active_window
        active_window=$(get_active_window_class)
        echo "🧪 Testing shader detection for current window: ${active_window:-"Unknown"}"
        if [[ -n "$active_window" ]]; then
            apply_shader_for_class "$active_window"
            echo "✅ Test completed - check if shader was applied"
        else
            echo "❌ Could not detect active window"
        fi
        ;;
    *)
        echo "🌈 Auroran Neon Dynamic Shader Monitor"
        echo "====================================="
        echo ""
        echo "🎨 Automatically applies gradient shaders based on active window"
        echo ""
        echo "📋 Available Commands:"
        echo "   start    - Start monitoring in background (default)"
        echo "   stop     - Stop monitoring and disable shaders"
        echo "   restart  - Restart the monitoring service"
        echo "   status   - Show current monitor status"
        echo "   logs     - Show recent log entries"
        echo "   test     - Test shader detection on current window"
        echo ""
        echo "💡 The monitor runs automatically in the background and switches"
        echo "   gradient shaders to match the border colors of your active app!"
        ;;
esac
