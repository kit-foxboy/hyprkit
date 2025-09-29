# 🦊 HyprKit - Dotfile Theme Manager with Stack-Based Backups

A Rust-based CLI tool for managing dotfile themes on Hyprland with safe backup and rollback capabilities that will hopefully serve as a foundation for rapid ricing

## What is this?

HyprKit is at the moment a basic bitch Hyprland theme manager with higher ambitions like being able to persist window rules, sizing, monitor or workspace assignment, etc. in your configs for persistance that doesn't override anything heavy. It provides a complete backup stack system, allowing you to safely switch between themes and roll back changes at any time. Built specifically for Hyprland users who enjoy experimenting and making their environment explode as frequently as I do. Will connect with the hyprkit shell for some really cool stuff planned  if I ever get my life and health even a hair under control. It's probably redudant and unremarkable with the other million dotfile managers but I had to learn rust for a job and love how DIY Linux is anyway. And I wrote it. That alone makes this a better investment in gold for your retirement*

*Not legally binding, foxes lie

## 🚀 Key Features

- 🎨 **Theme Management** - Install, uninstall, and switch between dotfile themes
- 📚 **Stack-Based Backups** - Every operation creates a backup for safe rollbacks, I am famous for appreciating stacked things after all
- ⏪ **Undo System** - Undo any theme operation or reset to original configuration
- 🔗 **GNU Stow Integration** - Reliable symlink management under the hood, (GNUwU)
- � **JSON State Persistence** - Track current theme and backup history
- 🎯 **Interactive TUI** - User-friendly prompts when theme not specified, (I almost called it the Hawk_Tui module so count your blessings >:3)
- �️ **Theme Building** - Create new themes from existing configurations
- 🦀 **An Excuse to learn Rust** - I wrote this bitch in rust even though only one client has ever needed me to know, I'll use any excuse to learn new stuff, even this funky language
- 👀 **Future integration with a planned GUI** - Designed to be paired with a keyboard centric GUI for devs who want to keep them paws on the keys

## 📖 Usage

```bash
# Install a theme (with backup of current state)
hyprkit install <theme-name>
hyprkit install  # Interactive selection

# Uninstall a theme (with backup)
hyprkit uninstall <theme-name>  
hyprkit uninstall  # Interactive selection

# Build a new theme from current config
hyprkit build <new-theme-name>

# List available themes
hyprkit list

# Show current status and backup info
hyprkit status

# Undo the last operation (pops from backup stack)
hyprkit undo

# Reset to original configuration (clears all backups)
hyprkit reset
```

## 🔄 Backup System

You know how stacks work, you squoosh one operation in and boing it out in reverse order to undo things:

- **Every operation** creates a backup before making changes
- **Undo system** lets you roll back the last operation
- **Reset function** returns to your original configuration
- **JSON persistence** maintains backup history across sessions
- **ISO 8601 timestamps** ensure proper backup ordering

Example workflow:
```bash
# Start with original config
hyprkit install nord-theme     # Backup created, nord-theme active
hyprkit install dracula-theme  # Backup created, dracula-theme active  
hyprkit undo                   # Back to nord-theme
hyprkit undo                   # Back to original config
hyprkit reset                  # Clear all backups, original config
```

## 📁 Directory Structure
(These trees are so annoying to type out wtf)

```
~/.config/hyprkit/
├── state.json           # Current theme & backup stack
├── backups/             # Backup storage
│   ├── backup_001/
│   └── backup_002/
└── themes/              # Theme definitions
    ├── nord-theme/
    │   ├── hypr/
    │   ├── waybar/  
    │   └── kitty/
    ├── dracula-theme/
    │   ├── hypr/
    │   ├── waybar/
    │   └── rofi/
    └── custom-theme/
        └── ...
```

Each theme is a directory containing dotfile structures that will be symlinked to `~/.config/` using GNU Stow.

## 🏗️ Architecture

HyprKit is built with a modular Rust architecture:

- **Dependency Injection** - Trait-based design for testability
- **Error Handling** - Comprehensive `anyhow` context chains  
- **Modular Constants and Testing** - Organized string management and tests per module, (No idea if this is the ideal paradigm in rust but people argue about it a lot on forums and dev sites so I'm going to do what I think makes sense)
- **Mock Testing** - Full test coverage with simulated stow operations, (Daddy always said it was a sin to kill a mockingtest)
- **JSON State** - Persistent backup stack and theme tracking with room for metadata in the future as this project is still a chaotic spare time thing I'm making up very much on the spot

## 🛠️ Requirements

- **GNU Stow** (`sudo pacman -S stow` on Arch, `sudo apt install stow` on Ubuntu/Debian) I'm on a variant of arch btw UwU
- **Rust 2021** (for building from source)

## 📦 Installation

```bash
# Clone the repository
git clone https://github.com/kit-foxboy/hyprkit.git
cd hyprkit

# Build and install
cargo build --release
sudo cp target/release/hyprkit /usr/local/bin/

# Or install with cargo
cargo install --path .
```

## 🧪 Testing

The project includes comprehensive test coverage with mock dependency injection:

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test themes::
cargo test file_ops::

# Run with output
cargo test -- --show-output
```

## 🎨 Example Themes

Check the `themes/` directory for example theme structures. Each theme demonstrates different ways to organize Hyprland configurations, from minimal setups to complete desktop environments.

## 🤝 Contributing

HyprKit follows architectural patterns I'm not even sure are best practice in Rust, follow where it is, feel free to educate me where I done goofed

- Dependency injection patterns
- Error handling standards  
- Per-module tests and constants
- Learn as you go mentality

## 📄 License

This project is licensed under the do whatever the hell you want with it if you give me headpats and tell everyone I'm cool license
