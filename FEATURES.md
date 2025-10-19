# clbar - Feature Overview

## Implemented Features

### Core Functionality
- **Clash API Integration**: Efficiently fetches proxy groups and nodes via single API call
- **Node Switching**: Quick one-click proxy node switching via tray menu
- **Auto-Refresh**: Periodic proxy group updates (configurable interval)
- **Status Monitoring**: Real-time connection status tracking

### User Interface
- **System Tray Icon**: Persistent tray presence with visual status indicators
  - Grey: Default/starting state
  - Green: Connected and operational
  - Red: Disconnected or error state
- **Context Menu**: Right-click menu with hierarchical proxy group organization
- **Active Node Indicator**: Checkmark (✓) shows currently selected node

### Notifications
- **Success Notifications**: Confirms successful node switches
- **Error Notifications**: Alerts on API failures or connection issues
- **Toast Duration**: 3s for success, 5s for errors

### Configuration
- **TOML Configuration**: User-friendly config file at `~/.config/clbar/config.toml`
- **Customizable Options**:
  - Clash API endpoint URL
  - API authentication secret
  - Proxy group filtering
  - Auto-refresh interval
- **Auto-Generation**: Creates default config on first run

### Auto-Start
- **systemd Integration**: User service for automatic startup
- **Hyprland Compatible**: Works with `exec-once` directive

## Technical Implementation

### API Optimization
- **Single Request Design**: Fetches all proxy groups in one `/proxies` API call
- **Smart Filtering**: Only shows Selector, URLTest, and Fallback type groups
- **Response Time**: <100ms typical response for node switches

### Architecture
- **Async Runtime**: Tokio-based async I/O for efficient network operations
- **Event-Driven**: Non-blocking event loop for UI responsiveness
- **Error Resilience**: Comprehensive error handling with user-friendly messages

### Supported Proxy Types
The application displays these Clash proxy group types:
- **Selector**: Manual node selection groups
- **URLTest**: Automatic latency-based selection groups
- **Fallback**: Failover groups

Individual proxy nodes of all types are displayed within their parent groups.

## Configuration Example

```toml
# Clash API endpoint
clash_api_url = "http://127.0.0.1:9090"

# Optional authentication
clash_secret = ""

# Show only specific groups (empty = all)
proxy_groups = ["proxy", "youtube", "google"]

# Refresh every 30 seconds
refresh_interval_secs = 30
```

## System Requirements

### Runtime Dependencies
- **libnotify**: Desktop notification support
- **GTK3**: System tray rendering
- **Clash/Clash Meta**: Running with API enabled

### Build Dependencies
- **Rust 1.70+**: Rust toolchain with Cargo
- **System libraries**: gtk3, libappindicator

## Performance Characteristics

- **Binary Size**: ~8.6 MB (release build)
- **Memory Usage**: <30 MB typical
- **CPU Usage**: <1% idle, <5% during operations
- **Startup Time**: <500ms
- **Switch Response**: <100ms (excluding network latency)

## Limitations

1. **Wayland/X11**: Primarily designed for Wayland (Hyprland) but may work on X11
2. **Local API Only**: No remote Clash instance support by default
3. **No Config Reload**: Requires restart to apply config changes
4. **Static Icons**: Uses simple colored squares (no SVG/PNG custom icons yet)

## Future Enhancement Ideas

- Custom icon support (SVG/PNG)
- Keyboard shortcuts (global hotkeys)
- Delay/latency display in menu
- Config file hot-reload
- Multi-instance support
- Dark/light theme detection
