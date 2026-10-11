mod config;
mod ergonomics;
mod notifier;
mod updater;

use config::load_config;
use ergonomics::{send_posture_reminder, send_stretch_reminder};
use notifier::{is_dnd, send_hydration_reminder, send_sweet_note};
use std::thread;
use std::time::Duration;
use updater::check_updates_quietly;

fn main() {
    println!("ZoeyOS desktop daemon started");

    let mut loop_counter: u64 = 0;

    loop {
        thread::sleep(Duration::from_secs(60));
        loop_counter += 1;

        let config = load_config();
        if !config.notifications.enabled {
            continue;
        }

        if is_dnd(&config) {
            continue;
        }

        let interval = config.notifications.interval_minutes.max(15);
        if loop_counter % interval == 0 {
            let cycle = (loop_counter / interval) % 4;
            match cycle {
                0 => {
                    if config.notifications.hydration_enabled {
                        send_hydration_reminder();
                    }
                }
                1 => {
                    if config.notifications.sweet_notes_enabled {
                        send_sweet_note();
                    }
                }
                2 => {
                    if config.notifications.posture_enabled {
                        send_posture_reminder();
                    }
                }
                3 => {
                    send_stretch_reminder();
                }
                _ => {}
            }
        }

        let update_ticks = (config.updates.check_interval_hours * 60).max(60);
        if loop_counter % update_ticks == 0 && config.updates.auto_check {
            check_updates_quietly();
        }
    }
}
