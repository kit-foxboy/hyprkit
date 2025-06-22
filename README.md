# 🦊 HyprKit - Hyprland Theme Manager

*"I DON'T KNOW! I'M STUPID AND GAY!"* - Kit, probably

---

## What is this mess?

HyprKit is Kit's egocentric attempt at making a Hyprland theme manager that (probably) doesn't suck completely. It's powered by bottled insanity, questionable life choices, and an unhealthy amount of UwU energy. 

Huge thanks to the amazing people working on **CachyOS** for their incredible distro and slick default theme, and to **mylinuxforwork** for the fantastic ML4W dotfiles that inspired this chaos!

Does it work? *Probably!* Will it set your computer on fire? *Hopefully not!*

```
         /\     /\  
        (  \\_//  ) 
         (  ^.^  ) < Let's make your desktop foxy!
          \  v  /  
           -----   
```

## Features (That Actually Work)

- 🎨 **Theme Switching** - Apply themes instantly
- 📋 **Active Theme Display** - See what disaster you're currently running
- ⏪ **Backup & Restore** - Because indecisiveness is a way of life
- ✨ **ASCII Art** - Because why not add more visual noise?

## Installation (AKA "How to let me infect your system with silliness")

1. **Clone this beautiful disaster:**
   ```bash
   git clone <your-repo-url> ~/HyprKit
   cd ~/HyprKit
   ```

2. **Execute order 66:**
   ```bash
   # Make scripts executable (because apparently that's important)
   chmod +x hyprkit.sh
   chmod +x scripts/set-theme.sh
   ```

3. **Run Kit's masterpiece:**
   ```bash
   ./hyprkit.sh
   ```

## Usage

### Interactive Mode (Recommended for LOSERS)
```bash
./hyprkit.sh
```

### Direct Commands (For the Brave/Stupid >:3)
```bash
# Set a theme (crossing fingers optional)
./scripts/set-theme.sh <theme-name>

# See what's currently active
./scripts/set-theme.sh --active

# Restore backup (when you inevitably break something)
./scripts/set-theme.sh --restore

# Get help (I hear that all the time!)
./scripts/set-theme.sh --help
```

## Directory Structure (Leaves actual theme structure up to the designer)

```
HyprKit/
├── hyprkit.sh               # Main interface (Start here)
├── scripts/
│   └── set-theme.sh         # The actual magic happens here
├── themes/
│   ├── cachyos/             # CachyOS team's beautiful default theme
│   └── ml4w/                # ML4W dotfiles theme (mylinuxforwork)
│       ├── theme.toml       # Theme metadata
│       ├── hypr/            # Hyprland configs
│       ├── waybar/          # Waybar configs  
│       ├── wofi/            # Wofi configs
│       └── ...              # Other config directories
├── backup/                  # Your safety net
└── current -> themes/...    # Symlink to active theme
```

## How It Works

1. **Symlink Magic** - Kit creates a `current` symlink pointing to your chosen theme
2. **Config Backup** - Your existing configs get safely tucked away (with timestamps!)
3. **Theme Application** - New configs get linked to `~/.config/`
4. **Hyprctl Reload** - Hyprland gets told about the changes
5. **Profit???** - Your desktop looks foxy (hopefully)

## Adding New Themes (Advanced Chaos Engineering)

1. Create a new directory in `themes/`
2. Add your config directories (hypr, waybar, wofi, etc.)
3. Include a `theme.toml` file (currently decorative, but I have plans!)
4. Cross your fingers and hope I doesn't break it

Example theme structure:
```
themes/your-awesome-theme/
├── theme.toml           # Theme info (name, description, etc.)
├── hypr/               # Hyprland configuration
├── waybar/             # Waybar configuration  
├── wofi/               # Wofi configuration
└── ...                 # Whatever else you need
```

## Planned Features (Kit's Delusional Ambitions)

- 🎨 **Automatic GTK Theme Matching** - Because consistency is overrated
- 📦 **Package Requirements & Auto-Install** - Let Kit handle your dependencies (what could go wrong giving my script root perms?)
- 🔧 **Theme Validation** - Make sure themes aren't completely broken before applying them
- 🌈 **More ASCII Art** - Because clearly there's not enough visual chaos
- 🎵 **Sound Effects** - Because why shouldn't theme switching go "boop"?

## Requirements (The Boring Stuff)

- **Hyprland** - Obviously, since this is a Hyprland theme manager
- **GTK** - Optional if Gnome themes are on the menu
- **Bash** - For running Kit's beautiful scripts
- **Working Brain Cells** - Optional, Kit doesn't have many either

## FAQ

**Q: Will this break my system?**
A: Probably not! I tested it on my system and it only caught fire twice.

**Q: Why so many fox references?**
A: Because I'm a furry and that's where my world begins and ends most days

**Q: Is Kit actually stupid and gay?**
A: Well stupid and bisexual just doesn't sound as charming

**Q: Can I add my own themes?**
A: Please do! This isn't an ambitious or proprietary project, so go nuts

**Q: What if something goes wrong?**
A: Always do your own backups

## Support and Contacts

- 🐛 **Issues**: Open a GitHub issue and I'll ignore it with style
- 💬 **Discussions**: Love a good chat
- 📧 **Email**: foxykit@gmail.com is your best bet but my health is very poor
- **Discord**: The best way to reach me, join the Flying Bucket Discord server: https://discord.gg/qC8pU4UST5
- **Ko-fi**: If you want to see what creative endeavors I'm up to or even kick a few bucks my way: https://ko-fi.com/kitkabbit4209

## License

This project is licensed under the world famous "Do Whatever The Hell You Want, This is for Fun" license.

```
         /\     /\  
        (  \\_//  ) 
         (  o.o  ) < Thanks for reading this disaster!
          \  ^  /    Now go make your desktop foxy! 🦊✨
           -----   
```

**P.S.** - Jokes aside, I really do hope you enjoy this dumb little experiment

## Credits & Thanks 🙏

- **CachyOS Team** - For their incredible distribution and the beautiful default theme that ships with it
- **mylinuxforwork** - For the amazing ML4W dotfiles project that inspired much of this work: https://github.com/mylinuxforwork/dotfiles
- **The Hyprland Community** - For building such an awesome compositor
- **Everyone who contributes themes** - Well that is if anyone decides to X3
