use notify_rust::{Notification, Timeout};
use rand::seq::SliceRandom;

const POSTURE_PROMPTS: &[&str; 3] = &[
    "Desk posture check! Drop your shoulders away from your ears",
    "Gentle reminder to uncross your legs and roll your shoulders back",
    "Chin up, cutie patootie! Make sure you aren't leaning into the monitor",
];

const STRETCH_PROMPTS: &[&str; 3] = &[
    "You've been at your desk for a while, babagirl! Stand up and take a quick stretch",
    "20-20-20 break: Look at something 20 feet away foe 20 seconds to rest your eyes",
    "Time to stand up, stretch your arms, and shake out your hands :>",
];

pub fn send_posture_reminder() {
    if let Some(prompt) = POSTURE_PROMPTS.choose(&mut rand::thread_rng()) {
        let _ = Notification::new()
            .appname("ZoeyOS")
            .summary("Desk Posture Check")
            .body(prompt)
            .icon("preferences-desktop-display")
            .timeout(Timeout::Milliseconds(9000))
            .show();
    }
}

pub fn send_stretch_reminder() {
    if let Some(prompt) = STRETCH_PROMPTS.choose(&mut rand::thread_rng()) {
        let _ = Notification::new()
            .appname("ZoeyOS")
            .summary("Stretch & Eye Rest")
            .body(prompt)
            .icon("user-available")
            .timeout(Timeout::Milliseconds(10000))
            .show();
    }
}
