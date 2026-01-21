# ht-linux

A terminal-based Linux system monitor with an Outlook-style interface, built with Rust and Ratatui.

## Features

- **Quick Shortcuts** - Press `c` for CPU, `m` for Memory, `p` for Processes, etc.
- **Tree Navigator** - Left pane with system categories
- **Live Monitoring** - Auto-refresh every 2 seconds
- **Vim-like Commands** - `:q` to quit, `:help` for help
- **Process Management** - Kill processes directly from the Processes view

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

#### Quick Shortcuts
Jump directly to any view by pressing its shortcut key:

| Key | View |
|-----|------|
| `*` | Overview |
| `c` | CPU |
| `m` | Memory |
| `d` | Disks |
| `n` | Network |
| `p` | Processes |
| `v` | Devices |
| `l` | Logs |

#### Navigation
| Key | Action |
|-----|--------|
| `Tab` | Switch focus between panes |
| `j` / `↓` | Move down / Select next item |
| `k` / `↑` | Move up / Select previous item |
| `Enter` | Open selected item / View details |

#### Actions
| Key | Action |
|-----|--------|
| `w` | Close current tab |
| `x` / `Delete` | Kill selected process (in Processes view) |
| `?` | Toggle help overlay |
| `Esc` | Close popup / Cancel command |

#### Vim Commands
| Command | Action |
|---------|--------|
| `:q` | Quit application |
| `:help` | Show help |

### Workflow

1. Press a shortcut key (`c`, `m`, `p`, etc.) to jump to any view
2. Or use `j`/`k` to navigate the tree and `Enter` to open
3. In the Processes view, press `x` to kill a selected process
4. In the Devices view, press `Enter` to see device details
5. Press `:q` to quit

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
