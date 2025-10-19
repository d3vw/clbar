# clbar - Clash Node Switcher Tray App

A lightweight system tray application for Hyprland (Arch Linux) that enables quick switching of Clash proxy group nodes via a right-click context menu.

## Features

- **System Tray Integration**: Persistent tray icon with right-click menu
- **Quick Node Switching**: One-click switching between proxy nodes
- **Visual Status Indicators**: Icon changes color based on connection status
  - Green: Connected and operational
  - Red: Connection failed or error state
- **Toast Notifications**: Desktop notifications for successful switches and errors
- **Auto-Refresh**: Periodically updates proxy group information
- **Minimal Overhead**: <100ms response time, minimal CPU/RAM usage
- **Wayland Native**: Built for Hyprland with GTK4 support

## Requirements

- Arch Linux with Hyprland
- Clash or Clash Meta running with API enabled
- Rust toolchain (for building from source)

## Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/clbar.git
cd clbar
```

2. Build the project:
```bash
cargo build --release
```

3. Install the binary:
```bash
sudo cp target/release/clbar /usr/local/bin/
```

4. (Optional) Install systemd service for auto-start:
```bash
mkdir -p ~/.config/systemd/user
cp clbar.service ~/.config/systemd/user/
systemctl --user enable clbar.service
systemctl --user start clbar.service
```

### From AUR

```bash
yay -S clbar
```

## Configuration

On first run, clbar creates a default configuration file at `~/.config/clbar/config.toml`:

```toml
# Clash API URL (default: http://127.0.0.1:9090)
clash_api_url = "http://127.0.0.1:9090"

# Clash API secret (leave empty if no authentication required)
clash_secret = ""

# Proxy groups to display in tray menu
# Empty list = show all groups
# Specify groups to filter: ["PROXY", "Fallback"]
proxy_groups = ["PROXY"]

# Auto-refresh interval in seconds
refresh_interval_secs = 30
```

### Configuration Options

- **clash_api_url**: The URL of your Clash API endpoint
- **clash_secret**: Authentication token for Clash API (if required)
- **proxy_groups**: List of proxy groups to show in menu (empty = all groups)
- **refresh_interval_secs**: How often to refresh proxy group information

## Usage

### Starting the Application

**Note**: If `clbar` command is not found, ensure `~/.cargo/bin` is in your PATH:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # for bash
# or
set -Ux fish_user_paths $HOME/.cargo/bin $fish_user_paths  # for fish
```

Run directly:
```bash
clbar
```

Or use the full path:
```bash
~/.cargo/bin/clbar
```

Or enable auto-start with systemd (recommended):
```bash
systemctl --user enable --now clbar.service
```

### Using the Tray Menu

1. Right-click the tray icon to open the context menu
2. Select a proxy group to view available nodes
3. Click on a node to switch to it
4. Use "Refresh" to manually update proxy groups
5. Use "Quit" to exit the application

### Auto-Start with Hyprland

Add to your `~/.config/hypr/hyprland.conf`:

```conf
exec-once = clbar
```

Or use the systemd service (recommended):

```conf
exec-once = systemctl --user start clbar.service
```

## Clash API Configuration

Ensure your Clash configuration has the API enabled:

```yaml
# Clash config.yaml
external-controller: 127.0.0.1:9090
secret: ""  # Optional: set a secret for authentication
```

If you use a secret, update your clbar config:

```toml
clash_secret = "your-secret-here"
```

## Troubleshooting

### Tray Icon Not Appearing

- Ensure you're running Hyprland with a status bar that supports tray icons (e.g., Waybar)
- Check if the tray module is enabled in your Waybar config

### Connection Errors

- Verify Clash is running: `curl http://127.0.0.1:9090/proxies`
- Check the Clash API URL in config matches your setup
- Ensure no firewall is blocking local connections

### Notifications Not Working

- Install `libnotify` if not already installed: `sudo pacman -S libnotify`
- Check your notification daemon is running

### Viewing Logs

When running via systemd:
```bash
journalctl --user -u clbar.service -f
```

## Development

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

## Technical Details

- **Language**: Rust
- **UI Framework**: tray-icon (Wayland/GTK4)
- **HTTP Client**: reqwest (async)
- **Notifications**: notify-rust
- **Runtime**: Tokio (async)

## License

MIT License - see LICENSE file for details

## Contributing

Contributions welcome! Please open an issue or pull request.

## Credits

Built for the Hyprland community on Arch Linux.
