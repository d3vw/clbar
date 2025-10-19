#!/bin/bash
set -e

echo "Building clbar..."
cargo build --release

echo "Installing binary to ~/.cargo/bin..."
mkdir -p ~/.cargo/bin
cp target/release/clbar ~/.cargo/bin/

echo "Installing systemd service..."
mkdir -p ~/.config/systemd/user
cp clbar.service ~/.config/systemd/user/

echo "Reloading systemd daemon..."
systemctl --user daemon-reload

echo ""
echo "Installation complete!"
echo ""
echo "To start clbar now:"
echo "  systemctl --user start clbar.service"
echo ""
echo "To enable auto-start on login:"
echo "  systemctl --user enable clbar.service"
echo ""
echo "To run manually:"
echo "  clbar"
