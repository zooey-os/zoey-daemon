use crate::config::{load_state, save_state};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn check_updates_quietly() {
    let mut pending = 0;

    if let Ok(output) = Command::new("flatpak")
        .args(["remote-ls", "--updates"])
        .output()
    {
        if output.status.success() {
            let lines = String::from_utf8_lossy(&output.stdout);
            pending += lines.lines().filter(|l| !l.trim().is_empty()).count() as u32;
        }
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut state = load_state();
    state.system.last_update_check_epoch = now;
    state.system.pending_updates_count = pending;
    let _ = save_state(&state);
}
