# ht-linux

A terminal-based Linux system monitor with an Outlook-style interface, built with Rust and Ratatui.

## Features

- **Tree Navigator** - Left pane with collapsible system categories
- **Dynamic Tabs** - Open multiple views simultaneously, switch between them
- **Live Monitoring** - Auto-refresh every 2 seconds
- **Vim-like Commands** - `:q` to quit, `:help` for help
- **Device Details** - View detailed info for any device in a popup

### System Views

| View | Description |
|------|-------------|
| Overview | Hostname, OS, kernel version, uptime, CPU/memory summary |
| CPU | Overall usage gauge + per-core breakdown |
| Memory | RAM and swap usage with detailed breakdown |
| Disks | Mounted partitions with usage bars |
| Network | Interface list with RX/TX statistics |
| Processes | Process list sorted by CPU usage (htop-like) |
| Devices | Block devices, USB, PCI, and input devices |
| Logs | System logs from dmesg |

## Installation

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled

### Run with Nix

```bash
git clone https://github.com/houdatahbaz/ht-linux.git
cd ht-linux
nix develop
cargo run
```

### Build Release

```bash
nix develop
cargo build --release
./target/release/ht-linux
```

## Usage

### Keyboard Controls

#### Navigation
| Key | Action |
|-----|--------|
| `Tab` | Switch focus between tree and tabs |
| `j` / `↓` | Move down / Select next item |
| `k` / `↑` | Move up / Select previous item |
| `l` / `→` / `Enter` | Open tab / Next tab / Select |
| `h` / `←` | Previous tab |
| `1-9` | Quick switch to tab by number |

#### Actions
| Key | Action |
|-----|--------|
| `w` | Close current tab |
| `Enter` | Open device details (in Devices view) |
| `?` | Toggle help overlay |
| `Esc` | Close popup / Cancel command |

#### Vim Commands
| Command | Action |
|---------|--------|
| `:q` | Quit application |
| `:quit` | Quit application |
| `:help` | Show help |

### Workflow

1. Use `j`/`k` to navigate the tree on the left
2. Press `Enter` or `l` to open a view as a tab
3. Press `Tab` to switch focus to the tabs area
4. Use `h`/`l` to switch between open tabs
5. In the Devices view, select a device and press `Enter` to see details
6. Press `:q` to quit

## Project Structure

```
ht-linux/
├── flake.nix              # Nix development environment
├── Cargo.toml             # Rust dependencies
└── src/
    ├── main.rs            # Entry point
    ├── app.rs             # Application state and input handling
    ├── events.rs          # Event types
    ├── system/
    │   └── mod.rs         # System data collection
    └── ui/
        ├── mod.rs         # Main UI drawing
        ├── tree.rs        # Tree navigator widget
        ├── tabs.rs        # Tab panel widget
        └── widgets/       # Individual view widgets
            ├── overview.rs
            ├── cpu.rs
            ├── memory.rs
            ├── disk.rs
            ├── network.rs
            ├── processes.rs
            ├── devices.rs
            └── logs.rs
```

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [tokio](https://tokio.rs/) - Async runtime
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information
- [serde_json](https://github.com/serde-rs/json) - JSON parsing for device info

## License

MIT
