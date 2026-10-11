use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct ZoeyConfig {
    #[serde(default)]
    pub notifications: NotificationSettings,
    #[serde(default)]
    pub updates: UpdateSettings,
}

impl Default for ZoeyConfig {
    fn default() -> Self {
        Self {
            notifications: NotificationSettings::default(),
            updates: UpdateSettings::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct NotificationSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_interval")]
    pub interval_minutes: u64,
    #[serde(default = "default_true")]
    pub hydration_enabled: bool,
    #[serde(default = "default_true")]
    pub posture_enabled: bool,
    #[serde(default = "default_true")]
    pub sweet_notes_enabled: bool,
    #[serde(default)]
    pub do_not_disturb: DndSettings,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_minutes: 60,
            hydration_enabled: true,
            posture_enabled: true,
            sweet_notes_enabled: true,
            do_not_disturb: DndSettings::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DndSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_start_hour")]
    pub start_hour: u32,
    #[serde(default = "default_end_hour")]
    pub end_hour: u32,
}

impl Default for DndSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            start_hour: 23,
            end_hour: 8,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateSettings {
    #[serde(default = "default_true")]
    pub auto_check: bool,
    #[serde(default = "default_check_hours")]
    pub check_interval_hours: u64,
}

impl Default for UpdateSettings {
    fn default() -> Self {
        Self {
            auto_check: true,
            check_interval_hours: 12,
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_interval() -> u64 {
    60
}
fn default_start_hour() -> u32 {
    23
}
fn default_end_hour() -> u32 {
    8
}
fn default_check_hours() -> u64 {
    12
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct QuotesCatalog {
    #[serde(default)]
    pub sweet: Vec<String>,
    #[serde(default)]
    pub hydration: Vec<String>,
    #[serde(default)]
    pub jokes: Vec<String>,
    #[serde(default)]
    pub wellness: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoeyState {
    pub water_tracker: WaterTracker,
    pub system: SystemState,
}

impl Default for ZoeyState {
    fn default() -> Self {
        Self {
            water_tracker: WaterTracker {
                date: Local::now().format("%Y-%m-%d").to_string(),
                cups_logged: 0,
                daily_goal: 8,
            },
            system: SystemState {
                last_update_check_epoch: 0,
                pending_updates_count: 0,
                last_sos_timestamp: 0,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WaterTracker {
    pub date: String,
    pub cups_logged: u32,
    pub daily_goal: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemState {
    pub last_update_check_epoch: u64,
    pub pending_updates_count: u32,
    pub last_sos_timestamp: u64,
}

pub fn zooey_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".config/zooey")
}

pub fn load_config() -> ZoeyConfig {
    let path = zooey_dir().join("config.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn load_quotes() -> QuotesCatalog {
    let path = zooey_dir().join("quotes.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn load_state() -> ZoeyState {
    let path = zooey_dir().join("state.json");
    let mut state: ZoeyState = fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default();

    let today = Local::now().format("%Y-%m-%d").to_string();
    if state.water_tracker.date != today {
        state.water_tracker.date = today;
        state.water_tracker.cups_logged = 0;
        let _ = save_state(&state);
    }

    state
}

pub fn save_state(state: &ZoeyState) -> std::io::Result<()> {
    let dir = zooey_dir();
    fs::create_dir_all(&dir)?;
    let path = dir.join("state.json");
    let json = serde_json::to_string_pretty(state)?;
    fs::write(path, json)
}
