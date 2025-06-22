#!/bin/bash
# HyprKit - Simple theme manager for Hyprland
# Main menu interface

# Colors for better UI
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to display header
show_header() {
    clear
    echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║                       ${PURPLE}HyprKit${CYAN}                         ║${NC}"
    echo -e "${CYAN}║               ${YELLOW}Hyprland Theme Manager${CYAN}                  ║${NC}"
    echo -e "${CYAN}║           ${GREEN}Powered by Kit's bottled insanity!${CYAN}          ║${NC}"
    echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${YELLOW}         /\\     /\\  ${NC}"
    echo -e "${YELLOW}        (  \\\\_//  ) ${NC}"
    echo -e "${YELLOW}         (  ^.^  ) ${GREEN}< Let's make your desktop foxy!${NC}"
    echo -e "${YELLOW}          \\  v  /  ${NC}"
    echo -e "${YELLOW}           -----   ${NC}"
    echo ""
}

# Function to list available themes
list_themes() {
    echo -e "${YELLOW}🦊 Available Themes:${NC}"
    echo ""
    local count=1
    for theme in themes/*/; do
        if [ -d "$theme" ]; then
            theme_name=$(basename "$theme")
            echo -e "  ${GREEN}$count)${NC} $theme_name ${CYAN}✨${NC}"
            ((count++))
        fi
    done
    echo ""
    echo -e "${PURPLE}Pick whichever makes your tail wag and hope I don't set your computer on fire! UwU${NC}"
    echo ""
}

# Function to get current active theme
show_active_theme() {
    echo -e "${CYAN}🦊 Current Theme Status:${NC}"
    ./scripts/set-theme.sh --active
    echo ""
}

# Function to show main menu
show_menu() {
    echo -e "${BLUE}Options:${NC}"
    echo -e "  ${GREEN}1)${NC} Set Theme ${YELLOW}🎨${NC}"
    echo -e "  ${GREEN}2)${NC} Show Active Theme ${YELLOW}📋${NC}"
    echo -e "  ${GREEN}3)${NC} Restore Last Backup ${YELLOW}⏪${NC}"
    echo -e "  ${GREEN}4)${NC} List Available Themes ${YELLOW}📜${NC}"
    echo -e "  ${GREEN}5)${NC} Help ${YELLOW}❓${NC}"
    echo -e "  ${RED}q)${NC} Quit ${YELLOW}👋${NC}"
    echo ""
    echo -e "${PURPLE}Whatcha gonna do? *swishes tail* ^w^${NC}"
}

# Function to set theme
set_theme() {
    show_header
    list_themes
    
    echo -e "${YELLOW}Enter theme name (or 'b' to go back):${NC}"
    read -p "> " theme_choice
    
    if [[ "$theme_choice" == "b" ]]; then
        return
    fi
    
    if [[ -z "$theme_choice" ]]; then
        echo -e "${RED}That's not a theme! SMH my head${NC}"
        read -p "Press Enter to continue..."
        return
    fi
    
    echo ""
    echo -e "${CYAN}Kit's applying theme: $theme_choice ✨🦊${NC}"
    ./scripts/set-theme.sh "$theme_choice"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Theme set successfully!${NC}"
    else
        echo -e "${RED}Failed to set theme. I AM DISHONORED XwX${NC}"
    fi
    
    read -p "Press Enter to continue..."
}

# Function to restore backup
restore_backup() {
    show_header
    echo -e "${YELLOW}Kit's digging up the most recent backup... 🦊🔍${NC}"
    echo ""
    ./scripts/set-theme.sh --restore
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Backup restored successfully! Was there ever any dobut?${NC}"
    else
        echo -e "${RED}Failed to restore backup! Kit's shiny new toy needs work...${NC}"
    fi
    
    read -p "Press Enter to continue..."
}

# Function to show help
show_help() {
    show_header
    echo -e "${YELLOW}🦊 Kit's HyprKit Help Guide:${NC}"
    echo ""
    echo -e "${CYAN}What is HyprKit?${NC}"
    echo "HyprKit is Kit's awesome theme manager for Hyprland! It lets you instantly "
    echo "switch between different configuration themes if you're as indecisive as he is!"
    echo ""
    echo -e "${CYAN}Commands:${NC}"
    echo "• Set Theme: Apply a theme from the themes directory"
    echo "• Show Active: Display currently active theme"
    echo "• Restore Backup: Restore the most recent configuration backup"
    echo "• List Themes: Show all available themes"
    echo ""
    echo -e "${CYAN}Direct Script Usage:${NC}"
    echo "  ./scripts/set-theme.sh <theme_name>    # Set a theme"
    echo "  ./scripts/set-theme.sh --active        # Show active theme"
    echo "  ./scripts/set-theme.sh --restore       # Restore backup"
    echo "  ./scripts/set-theme.sh --help          # Show script help"
    echo ""
    echo -e "${PURPLE}Enjoy some foxy mad science >:3${NC}"
    echo ""
    read -p "Press Enter to continue..."
}

# Main program loop
main() {
    while true; do
        show_header
        show_active_theme
        show_menu
        
        read -p "🦊 Choose an option: " choice
        
        case $choice in
            1)
                set_theme
                ;;
            2)
                show_header
                show_active_theme
                read -p "Press Enter to continue..."
                ;;
            3)
                restore_backup
                ;;
            4)
                show_header
                list_themes
                read -p "Press Enter to continue..."
                ;;
            5)
                show_help
                ;;
            q|Q)
                echo ""
                echo -e "${YELLOW}         /\\     /\\  ${NC}"
                echo -e "${YELLOW}        (  \\\\_//  ) ${NC}"
                echo -e "${YELLOW}         (  ^.^  ) ${GREEN}< Thanks for using HyprKit!${NC}"
                echo -e "${YELLOW}          \\  0  /  ${GREEN}  Your desktop looks foxy! 🦊✨${NC}"
                echo -e "${YELLOW}           -----   ${NC}"
                echo ""
                exit 0
                ;;
            *)
                echo -e "${RED}Invalid option. Please try again.${NC}"
                read -p "Press Enter to continue..."
                ;;
        esac
    done
}

# Check if we're in the right directory
if [ ! -d "themes" ] || [ ! -f "scripts/set-theme.sh" ]; then
    echo -e "${RED}🦊?? Kit's lost (shocker)! Please run this script from the HyprKit root directory${NC}"
    exit 1
fi

# Start the main program
main
