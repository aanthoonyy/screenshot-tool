# 📸 shrust

**shrust** is a minimal Wayland-native screenshot tool written in Rust.

---

## ✨ Features

- Region selection with `slurp`
- Screenshot capture with `grim`
- Automatic filename generation
- Clipboard copy with `wl-copy`
- Clickable notification with `dunstify`
- Optional editing with `swappy`
- Configuration via `~/.config/shrust/config.toml`

---

## ⚙️ Installation

Make sure you have the following tools installed:

- Rust
- `grim`
- `slurp`
- `wl-clipboard`
- `dunst`
- `swappy`

---

## Quick Install
```bash
git clone https://github.com/aanthoonyy/shrust.git
cd shrust
chmod +x install.sh
./install.sh
```

### Manual Install

```bash
git clone https://github.com/aanthoonyy/shrust.git
cd shrust
cargo install --path .
```

Make sure your `$HOME/.cargo/bin` is in your `$PATH`:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

## 🛠 Configuration

By default, `shrust` uses sensible defaults, but you can customize it:

**~/.config/shrust/config.toml**
```toml
save_dir = "/home/yourname/Pictures"
filename_format = "screenshot-%Y-%m-%d-%H-%M-%S"
extension = "png"
copy_to_clipboard = true
open_editor = true
```

---

## 🖱 Usage

Take a screenshot by running:

```bash
shrust
```

You'll be prompted to select a region. It will:

- Save the screenshot to your configured folder
- Copy it to clipboard
- Send a notification
- If the notification is clicked, open in `swappy`

---

## 🛑 Troubleshooting

If notifications don't work:

```bash
dunst &
```

Ensure `dunst` is running in the background.

If `shrust` fails to run, make sure the dependencies (`grim`, `slurp`, etc.) are installed and in your `$PATH`.

---

## 🛎 Setup Sway hotkey

Add to your Sway config (`~/.config/sway/config`):

```text
bindsym $mod+Shift+s exec shrust
exec_always dunst
```

Then reload Sway:

```bash
swaymsg reload
```

Now hit **Super+Shift+S** to screenshot instantly!

## 📦 License

MIT

---