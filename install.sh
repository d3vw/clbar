#!/bin/bash
set -e

# Check if clbar service is running and stop it
SERVICE_RUNNING=false
if systemctl --user is-active --quiet clbar.service; then
    echo "Stopping clbar service..."
    systemctl --user stop clbar.service
    SERVICE_RUNNING=true
fi

# Also kill any manually running clbar processes
if pgrep -x clbar > /dev/null; then
    echo "Stopping running clbar processes..."
    pkill -x clbar || true
    sleep 1
fi

echo "Building clbar..."
cargo build --release

echo "Installing binary to ~/.cargo/bin..."
mkdir -p ~/.cargo/bin
cp -f target/release/clbar ~/.cargo/bin/

echo "Installing systemd service..."
mkdir -p ~/.config/systemd/user
cp -f clbar.service ~/.config/systemd/user/

echo "Reloading systemd daemon..."
systemctl --user daemon-reload

echo ""
echo "Installation complete!"
echo ""

# Restart service if it was running before
if [ "$SERVICE_RUNNING" = true ]; then
    echo "Restarting clbar service..."
    systemctl --user start clbar.service
    echo ""
    echo "clbar service has been restarted."
else
    echo "To start clbar now:"
    echo "  systemctl --user start clbar.service"
    echo ""
    echo "To enable auto-start on login:"
    echo "  systemctl --user enable clbar.service"
    echo ""
    echo "To run manually:"
    echo "  clbar"
fi
