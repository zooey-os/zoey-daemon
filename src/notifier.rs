use crate::config::{load_quotes, load_state, save_state, ZoeyConfig};
use chrono::{Local, Timelike};
use notify_rust::{Notification, Timeout};
use rand::seq::SliceRandom;

pub fn is_dnd(config: &ZoeyConfig) -> bool {
    if !config.notifications.do_not_disturb.enabled {
        return false;
    }
    let hour = Local::now().hour();
    let start = config.notifications.do_not_disturb.start_hour;
    let end = config.notifications.do_not_disturb.end_hour;

    if start > end {
        hour >= start || hour < end
    } else {
        hour >= start && hour < end
    }
}

pub fn send_hydration_reminder() {
    let quotes = load_quotes();
    let prompt = quotes
        .hydration
        .choose(&mut rand::thread_rng())
        .map(|s| s.as_str())
        .unwrap_or("don't forget to hydrate today!");

    let handle = Notification::new()
        .appname("ZoeyOS")
        .summary("Hydration Check")
        .body(prompt)
        .icon("dialog-information")
        .timeout(Timeout::Milliseconds(12000))
        .action("drank", "Drank Water!")
        .show();

    if let Ok(h) = handle {
        h.wait_for_action(|action| {
            if action == "drank" {
                let mut state = load_state();
                state.water_tracker.cups_logged += 1;
                let _ = save_state(&state);

                let _ = Notification::new()
                    .appname("ZoeyOS")
                    .summary("Good job, babagirl!")
                    .body(&format!(
                        "Logged {} of {} cups for today",
                        state.water_tracker.cups_logged, state.water_tracker.daily_goal
                    ))
                    .timeout(Timeout::Milliseconds(4000))
                    .show();
            }
        });
    }
}

pub fn send_sweet_note() {
    let quotes = load_quotes();
    let mut pool = quotes.sweet;
    pool.extend(quotes.jokes);
    pool.extend(quotes.wellness);

    if let Some(note) = pool.choose(&mut rand::thread_rng()) {
        let _ = Notification::new()
            .appname("ZoeyOS")
            .summary("thinking of zooey")
            .body(note)
            .icon("emblem-favorite")
            .timeout(Timeout::Milliseconds(10000))
            .show();
    }
}
