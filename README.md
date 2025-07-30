# 🦊 HyprKit - Hyprland Theme & State Management
## GOALS
- 🎨 **Theme Switching** - Apply themes instantly
- ⏪ **Backup & Restore** - Because indecisiveness is a way of life
- 🔍 **Proper Separation of Concerns** - User overrides that persist between aesthetic theme changes. No more resetting your monitor config just because you want a new look! 
- 💾 **Live State Persistence** - Remembers changes like floating window positions, gaps, etc. and intelligently writes them to your configs 
- 🤝 **API Layer** - Easily interface with the GUI of your choice and modify persistent state as you will. I'm planning on using AGS for my rig but with IPC and HTTP support, it should work for all kinds of mad science

Warning: This is highly experimental and being made while I'm incredibly ill. I do want to make it a cool portfolio project or my first big community foray but manage expectations on timeline and true thoroughness. 
*"I DON'T KNOW! I'M STUPID AND GAY!"* - Kit

---

## What is this thingie? (Now with 100% more Rust!)

HyprKit started as my egocentric bash script experiment and has evolved into a proper Rust-powered Hyprland theme and state management system that (probably) won't be useless. It's powered by bottled insanity and questionable life choices, but now with memory safety and just... so many ampersands! 

**Current Status: Complete Rust Rewrite in Progress!** 🦀

What started as simple theme switching has grown into a full desktop state management system. Because this is pretty much fly by wire tinkering.

Does it work? *Eventually!* Will it set your computer on fire? *I invoke my 5th ammendment right against self incrimination*

```
         /\     /\  
        (  \\_//  ) 
         (  ^.^  ) < Let's make your desktop foxy!
          \  v  /  
           -----   
```

## Directory Structure (Note that themes are intended to be loaded from the user's .themes/hyprkit folder and is only here as an example of how a theme is structured. The backups and such are stored in .local, and the actual writing occurs live in the .config/hypr directory. Things like waybar and wlogout are not kept in state management)

```
HyprKit/
├── Cargo.toml               # Rust project configuration
├── src/
│   ├── main.rs              # CLI interface and command routing
│   ├── lib.rs               # Core library modules
│   ├── api/                 # HTTP and IPC API implementations
│   │   ├── mod.rs
│   │   ├── http.rs          # REST API for web interfaces
│   │   ├── ipc.rs           # Unix socket communication
│   │   └── types.rs         # Shared API types
│   └── config/              # Configuration management
│       ├── mod.rs
│       └── paths.rs         # XDG directory handling
├── themes/
│   ├── cachyos/             # CachyOS team's beautiful default theme
│   └── auroran-neon/        # Kit's custom cyberpunk theme
│       ├── theme.toml       # Theme metadata
│       ├── assets/          # Theme icons, wallpapers, sounds, etc.
|       └── dotfiles/        # The actual meat and potatoes configs
│           ├── hypr/        # Hyprland configs
│           ├── waybar/      # Waybar configs  
│           ├── wofi/        # Wofi configs
│           └── ...          # Other config directories
```

## 🗺️ Development Roadmap

### 🚧 **Phase 1: Core Rust Foundation** (Current)
- [x] Basic CLI structure with clap
- [x] Async runtime setup with tokio  
- [x] Project structure and module organization
- [ ] Theme discovery and validation
- [ ] Config file parsing
- [ ] Basic theme switching functionality
- [ ] Backup and restore system
- [ ] Live state monitoring and persistence

### 🎨 **Phase 2: Theme & State Features**
- [ ] Theme variants and customization
- [ ] State persistence for floating window positions, workspaces, etc.
- [ ] Per-application window rules persistence
- [ ] Buffer on state changes to prevent too much I/O
- [ ] Theme dependency validation

### 🌐 **Phase 2: API Layer** 
- [ ] IPC socket communication
- [ ] HTTP REST API implementation
- [ ] Error handling and response types

### 🔮 **Phase 4: Desktop Integration**
- [ ] GTK theme support
- [ ] Qt theme support
- [ ] Icon theme support  
- [ ] Cursor theme support
- [ ] Color palette extraction and generation if applicable

## How It Works
I'll let you know when I do lol. I have plans but it's highly subject to experimentation and documentation deep dives.

## Adding New Themes

1. Create a new directory in `themes/`
2. Add your config directories (hypr, waybar, wofi, etc.)
3. Include a `theme.toml` file for dependencies primarily
4. Cross your fingers and hope I don't break it


## Requirements

- **Hyprland** - Obviously, since this is a Hyprland theme manager
- **Rust** - For building the new shiny binary (1.75+ recommended)
- **GTK** - Optional if Gnome themes are on the menu
- **Working Brain Cells** - Optional, I doesn't have many neither XwX

## FAQ

**Q: Will this break my system?**
A: Probably not! I tested it on my system and it only caught fire twice.

**Q: Why so many fox references?**
A: Because I'm a furry and that's where my world begins and ends most days

**Q: Is Kit actually stupid and gay?**
A: Well stupid and bisexual just doesn't sound as charming

**Q: Can I add my own themes?**
A: Please do! This is designed to handle any set of hyprland dotfiles in theory

**Q: What if something goes wrong?**
A: Always do your own backups, this is beyond experimental

## Support and Contacts

- 🐛 **Issues**: Open a GitHub issue and I'll ignore it with style
- 💬 **Discussions**: Love a good chat
- 📧 **Email**: foxykit@gmail.com is your best bet but my health is very poor
- **Discord**: The best way to reach me, join the Flying Bucket Discord server: https://discord.gg/qC8pU4UST5
- **Ko-fi**: If you want to see what creative endeavors I'm up to or even kick a few bucks my way: https://ko-fi.com/kitkabbit4209

## License

This project is licensed under the world famous "Do Whatever The Hell You Want, This is for Fun, Future Efficiency, and Self Education" license.

```
         /\     /\  
        (  \\_//  ) 
         (  o.o  ) < Thanks for reading this product of really bad insomnia!
          \  ^  /    Now go make your desktop foxy! 🦊✨
           -----   
```

**P.S.** - Jokes aside, I really do hope you enjoy this dumb little experiment

## Credits & Thanks 🙏
- **CachyOS Team** - For their incredible distribution and the beautiful default theme that ships with it
- **mylinuxforwork** - For the amazing ML4W dotfiles project that inspired much of this work: https://github.com/mylinuxforwork/dotfiles
- **The Hyprland Community** - For building such an awesome compositor
- **The Flying Bucket** - My online friends who've been unreal in their support and kindness in a time where I've been too sick to be the social butterfly I used to
- **Everyone who sees this as worthwhile in any way** - Well that is if anyone decides it is X3
