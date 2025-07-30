#!/bin/bash

# ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
# ┃              Auroran Neon Shader Manager                    ┃
# ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

# Easy management interface for the dynamic shader system

MONITOR_SCRIPT="$HOME/.config/hypr/scripts/gradient-shader-monitor.sh"
SHADER_DIR="$HOME/.config/hypr/shaders"

# Color codes for pretty output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${PURPLE}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${PURPLE}║${NC}                ${CYAN}🌈 Auroran Neon Shader Manager${NC}                ${PURPLE}║${NC}"
    echo -e "${PURPLE}╚══════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_status() {
    echo -e "${BLUE}📊 System Status:${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    "$MONITOR_SCRIPT" status
    echo ""
    
    # Show available shaders
    echo -e "${BLUE}🎨 Available Gradient Shaders:${NC}"
    if [[ -d "$SHADER_DIR" ]]; then
        local shader_count=0
        for shader in "$SHADER_DIR"/*_gradient.frag; do
            if [[ -f "$shader" ]]; then
                local name=$(basename "$shader" .frag)
                local color_name=$(echo "$name" | sed 's/_gradient$//' | tr '_' ' ' | sed 's/\b\w/\U&/g')
                echo "   🎭 $color_name"
                ((shader_count++))
            fi
        done
        echo "   📁 Total: $shader_count gradient shaders available"
    else
        echo -e "   ${RED}❌ Shader directory not found: $SHADER_DIR${NC}"
    fi
    echo ""
}

show_manual_controls() {
    echo -e "${BLUE}🎛️ Manual Shader Controls:${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Available gradient shaders:"
    local counter=1
    for shader in "$SHADER_DIR"/*_gradient.frag; do
        if [[ -f "$shader" ]]; then
            local name=$(basename "$shader" .frag)
            local color_name=$(echo "$name" | sed 's/_gradient$//' | tr '_' ' ')
            echo "  $counter) $color_name"
            ((counter++))
        fi
    done
    echo "  0) Disable shader"
    echo ""
    read -p "Select shader to apply (0-$((counter-1))): " choice
    
    if [[ "$choice" == "0" ]]; then
        hyprctl keyword decoration:screen_shader ""
        echo -e "${GREEN}✅ Shader disabled${NC}"
    elif [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" -le $((counter-1)) ] && [ "$choice" -gt 0 ]; then
        local selected_shader=$(find "$SHADER_DIR" -name "*_gradient.frag" | sort | sed -n "${choice}p")
        if [[ -f "$selected_shader" ]]; then
            hyprctl keyword decoration:screen_shader "$selected_shader"
            echo -e "${GREEN}✅ Applied: $(basename "$selected_shader")${NC}"
        fi
    else
        echo -e "${RED}❌ Invalid selection${NC}"
    fi
}

show_help() {
    echo -e "${BLUE}📖 Available Commands:${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo -e "${GREEN}Monitor Control:${NC}"
    echo "  start     - Start the background shader monitor"
    echo "  stop      - Stop the background shader monitor"
    echo "  restart   - Restart the monitor service"
    echo "  status    - Show detailed system status"
    echo ""
    echo -e "${GREEN}Manual Control:${NC}"
    echo "  apply     - Manually select and apply a shader"
    echo "  test      - Test shader detection on current window"
    echo "  disable   - Disable all shaders"
    echo ""
    echo -e "${GREEN}Information:${NC}"
    echo "  logs      - Show recent monitor logs"
    echo "  shaders   - List all available gradient shaders"
    echo "  help      - Show this help message"
    echo ""
    echo -e "${CYAN}💡 The monitor automatically applies gradient shaders that match your border colors!${NC}"
}

list_shaders() {
    echo -e "${BLUE}🎨 Available Gradient Shaders:${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    declare -A SHADER_DESCRIPTIONS=(
        ["cyber_green_gradient"]="💻 Coding applications - Matrix-inspired green ambiance"
        ["electric_blue_gradient"]="🌐 Browsers - Immersive blue web browsing experience"
        ["auroran_gold_gradient"]="📁 File managers - Warm golden file exploration"
        ["neon_purple_gradient"]="💬 Communication - Mystical purple social atmosphere"
        ["neon_pink_gradient"]="🎮 Gaming - High-energy pink gaming excitement"
        ["cyber_orange_gradient"]="🎵 Media - Vibrant orange entertainment experience"
        ["electric_cyan_gradient"]="🖥️ Terminals - Cyberpunk cyan hacker aesthetic"
    )
    
    for shader in "$SHADER_DIR"/*_gradient.frag; do
        if [[ -f "$shader" ]]; then
            local name=$(basename "$shader" .frag)
            local description="${SHADER_DESCRIPTIONS[$name]:-"🎨 Custom gradient shader"}"
            local size=$(stat -c%s "$shader" 2>/dev/null || echo "0")
            echo "  🎭 $name.frag"
            echo "     $description"
            echo "     📦 Size: ${size} bytes"
            echo ""
        fi
    done
}

# Main command handling
case "${1:-status}" in
    "start")
        print_header
        echo -e "${GREEN}🚀 Starting Auroran Neon Shader Monitor...${NC}"
        "$MONITOR_SCRIPT" start
        ;;
    "stop")
        print_header
        echo -e "${YELLOW}🛑 Stopping Auroran Neon Shader Monitor...${NC}"
        "$MONITOR_SCRIPT" stop
        ;;
    "restart")
        print_header
        echo -e "${BLUE}🔄 Restarting Auroran Neon Shader Monitor...${NC}"
        "$MONITOR_SCRIPT" restart
        ;;
    "status")
        print_header
        print_status
        ;;
    "apply"|"manual")
        print_header
        show_manual_controls
        ;;
    "test")
        print_header
        echo -e "${CYAN}🧪 Testing shader detection...${NC}"
        "$MONITOR_SCRIPT" test
        ;;
    "disable")
        print_header
        echo -e "${YELLOW}🔄 Disabling all shaders...${NC}"
        hyprctl keyword decoration:screen_shader ""
        echo -e "${GREEN}✅ All shaders disabled${NC}"
        ;;
    "logs")
        print_header
        echo -e "${BLUE}📝 Recent Monitor Logs:${NC}"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        "$MONITOR_SCRIPT" logs
        ;;
    "shaders")
        print_header
        list_shaders
        ;;
    "help"|"--help"|"-h")
        print_header
        show_help
        ;;
    *)
        print_header
        echo -e "${RED}❌ Unknown command: $1${NC}"
        echo ""
        show_help
        ;;
esac
