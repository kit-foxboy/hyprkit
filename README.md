# 🦊 HyprKit - Simple Dotfile Manager

A simple dotfile manager built in Rust that uses GNU Stow for symlink management.

## What is this?

HyprKit is a straightforward command-line tool for managing your dotfiles using GNU Stow. It provides a simple interface to install and uninstall dotfile packages, making it easy to manage different configurations for various applications.

## Features

- 📦 **Package Management** - Install and uninstall dotfile packages
- 🔗 **Stow Integration** - Uses GNU Stow for reliable symlink management  
- 📋 **Package Listing** - See available and installed packages
- 🎯 **Flexible Paths** - Custom source and target directories

## Usage

```bash
# Install a package
hyprkit install <package-name>

# Uninstall a package  
hyprkit uninstall <package-name>

# List available packages
hyprkit list

# Show status
hyprkit status

# Custom directories
hyprkit --dir ~/my-dotfiles --target ~/custom-target install vim
```

## Directory Structure

```
dotfiles/
├── vim/              # Package: vim configuration
│   └── .vimrc
├── zsh/              # Package: zsh configuration  
│   ├── .zshrc
│   └── .zsh_aliases
└── hypr/             # Package: Hyprland configuration
    └── .config/
        └── hypr/
            └── hyprland.conf
```

## Requirements

- GNU Stow (`sudo pacman -S stow` on Arch, `sudo apt install stow` on Ubuntu/Debian)
- Rust (for building from source)

## Installation

```bash
# Clone the repository
git clone https://github.com/kit-foxboy/hyprkit.git
cd hyprkit

# Build and install
cargo build --release
sudo cp target/release/hyprkit /usr/local/bin/
```

## Examples

The `themes/` directory contains example dotfile packages that demonstrate the structure.
