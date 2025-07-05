set -e

echo "Uninstalling Shrust..."

echo "Removing binary via cargo..."
cargo uninstall shrust-shot || echo "Binary may not be installed via cargo, skipping..."

CONFIG_DIR="$HOME/.config/shrust"
if [ -d "$CONFIG_DIR" ]; then
    echo "Removing config directory at $CONFIG_DIR"
    rm -rf "$CONFIG_DIR"
else
    echo "No config directory found at $CONFIG_DIR"
fi

BASHRC="$HOME/.bashrc"
if grep -q 'export PATH="\$HOME/.cargo/bin:\$PATH"' "$BASHRC"; then
    echo "Cleaning up PATH entry in ~/.bashrc"
    sed -i '/export PATH="\$HOME\/.cargo\/bin:\$PATH"/d' "$BASHRC"
    echo "Please restart your shell or run: source ~/.bashrc"
fi

echo ""
echo "Shrust has been uninstalled."
