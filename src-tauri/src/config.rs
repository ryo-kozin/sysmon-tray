use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub update_interval_secs: u64,
    pub cpu_threshold_percent: f32,
    pub cpu_sustained_secs: u64,
    pub memory_free_threshold_percent: f32,
    pub disk_free_threshold_gb: f64,
    pub notification_cooldown_mins: u64,
    pub notify_cpu: bool,
    pub notify_memory: bool,
    pub notify_disk: bool,
    pub autostart: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update_interval_secs: 3,
            cpu_threshold_percent: 80.0,
            cpu_sustained_secs: 10,
            memory_free_threshold_percent: 10.0,
            disk_free_threshold_gb: 10.0,
            notification_cooldown_mins: 15,
            notify_cpu: true,
            notify_memory: true,
            notify_disk: true,
            autostart: false,
        }
    }
}

impl Config {
    fn config_path() -> PathBuf {
        let dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("system-monitor");
        fs::create_dir_all(&dir).ok();
        dir.join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        match fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => {
                let config = Self::default();
                config.save();
                config
            }
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Ok(data) = serde_json::to_string_pretty(self) {
            fs::write(path, data).ok();
        }
    }
}

impl Config {
    pub fn validated(mut self) -> Self {
        self.update_interval_secs = self.update_interval_secs.clamp(1, 60);
        self.cpu_threshold_percent = self.cpu_threshold_percent.clamp(1.0, 100.0);
        self.cpu_sustained_secs = self.cpu_sustained_secs.clamp(1, 300);
        self.memory_free_threshold_percent = self.memory_free_threshold_percent.clamp(1.0, 50.0);
        self.disk_free_threshold_gb = self.disk_free_threshold_gb.clamp(0.5, 500.0);
        self.notification_cooldown_mins = self.notification_cooldown_mins.clamp(1, 120);
        self
    }
}

pub struct ConfigState(pub Mutex<Config>);
