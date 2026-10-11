# Zoey Daemon

A lightweight, ambient background desktop companion written in Rust for ZoeyOS.

`zoey-daemon` runs quietly in the background as a user service to look after Zoey while she's at her desk--sending gentle health reminders, tracking daily hydration, and keeping her PC fresh.

---

## Features

- **Desk Ergonomics & Eye Care**: Periodic gentle posture checks and 20-20-20 eye-rest stretch prompts.
- **Interactive Hydration Toasts**: Desktop notifications with a clickable `[Drank Water!]` button that increments daily progress in `~/.config/zooey/state.json`.
- **Dynamic Sweet Notes**: Pulls rotating love notes, jokes, and affirmations from `~/.config/zooey/quotes.json`.
- **Quiet System Care**: Checks for pending Flatpak updates quietly in the background without interrupting her.
- **Do Not Disturb Awareness**: Automatically sleeps during bedtime hours configured in `config.json`.
- **Featherweight**: Pure native Rust using D-Bus notifications (< 5MB RSS memory footprint).

---

## Configuration

`zoey-daemon` reads and writes to `~/.config/zooey/`:

| File | Purpose |
| :--- | :--- |
| `config.json` | Notification interval, DND sleep hours, and feature toggles |
| `quotes.json` | Catalog of sweet notes, hydration prompts, and jokes |
| `state.json` | Real-time water tracker progress and update flags |

---

## Usage & Installation

### Build & Run

```bash
cargo build --release
./target/release/zoey-daemon
```

### Install as a Systemd User Service

To automatically start on login:

```bash
# 1. Copy binary to local path
sudo cp target/release/zoey-daemon /usr/local/bin

# 2. Install and enable systemd user unit
mkdir -p ~/.config/systemd/user
cp zoey-daemon.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now zoey-daemon.service
```

### Check Logs

```bash
journalctl --user -u zoey-daemon -f
```
