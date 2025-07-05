set -e

echo "Installing shrust..."

cargo install --path . --force

if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    echo "Added ~/.cargo/bin to your PATH in ~/.bashrc"
    source ~/.bashrc
fi

CONFIG_DIR="$HOME/.config/shrust"
CONFIG_FILE="$CONFIG_DIR/config.toml"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "Creating default config at $CONFIG_FILE"
    mkdir -p "$CONFIG_DIR"
    cat <<EOF > "$CONFIG_FILE"
save_dir = "${HOME}/Pictures"
filename_format = "screenshot-%Y-%m-%d-%H-%M-%S"
extension = "png"
copy_to_clipboard = true
open_editor = true
EOF
else
    echo "Config already exists at $CONFIG_FILE"
fi

echo ""
echo "Installation complete!"
echo ""
echo "To bind shrust to a hotkey in Sway, add this to your config (~/.config/sway/config):"
echo ""
echo "    bindsym \$mod+Shift+s exec shrust"
echo "    exec_always dunst"
echo ""
echo "Then reload sway with: Mod+Shift+c"
