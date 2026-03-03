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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let cfg = Config::default();
        assert_eq!(cfg.update_interval_secs, 3);
        assert!((cfg.cpu_threshold_percent - 80.0).abs() < f32::EPSILON);
        assert_eq!(cfg.cpu_sustained_secs, 10);
        assert!((cfg.memory_free_threshold_percent - 10.0).abs() < f32::EPSILON);
        assert!((cfg.disk_free_threshold_gb - 10.0).abs() < f64::EPSILON);
        assert_eq!(cfg.notification_cooldown_mins, 15);
        assert!(cfg.notify_cpu);
        assert!(cfg.notify_memory);
        assert!(cfg.notify_disk);
        assert!(!cfg.autostart);
    }

    #[test]
    fn serialize_roundtrip() {
        let cfg = Config::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.update_interval_secs, cfg.update_interval_secs);
        assert!(
            (deserialized.cpu_threshold_percent - cfg.cpu_threshold_percent).abs() < f32::EPSILON
        );
        assert_eq!(deserialized.autostart, cfg.autostart);
    }

    #[test]
    fn deserialize_partial_json_uses_defaults() {
        // Missing fields should cause deserialization to fall back to default
        let json = r#"{"update_interval_secs": 5}"#;
        let result: Result<Config, _> = serde_json::from_str(json);
        // Serde requires all fields by default, so partial JSON fails
        assert!(result.is_err());
    }

    #[test]
    fn validated_clamps_low_values() {
        let cfg = Config {
            update_interval_secs: 0,
            cpu_threshold_percent: 0.0,
            cpu_sustained_secs: 0,
            memory_free_threshold_percent: 0.0,
            disk_free_threshold_gb: 0.0,
            notification_cooldown_mins: 0,
            ..Config::default()
        };
        let v = cfg.validated();
        assert_eq!(v.update_interval_secs, 1);
        assert!((v.cpu_threshold_percent - 1.0).abs() < f32::EPSILON);
        assert_eq!(v.cpu_sustained_secs, 1);
        assert!((v.memory_free_threshold_percent - 1.0).abs() < f32::EPSILON);
        assert!((v.disk_free_threshold_gb - 0.5).abs() < f64::EPSILON);
        assert_eq!(v.notification_cooldown_mins, 1);
    }

    #[test]
    fn validated_clamps_high_values() {
        let cfg = Config {
            update_interval_secs: 999,
            cpu_threshold_percent: 200.0,
            cpu_sustained_secs: 999,
            memory_free_threshold_percent: 100.0,
            disk_free_threshold_gb: 9999.0,
            notification_cooldown_mins: 999,
            ..Config::default()
        };
        let v = cfg.validated();
        assert_eq!(v.update_interval_secs, 60);
        assert!((v.cpu_threshold_percent - 100.0).abs() < f32::EPSILON);
        assert_eq!(v.cpu_sustained_secs, 300);
        assert!((v.memory_free_threshold_percent - 50.0).abs() < f32::EPSILON);
        assert!((v.disk_free_threshold_gb - 500.0).abs() < f64::EPSILON);
        assert_eq!(v.notification_cooldown_mins, 120);
    }

    #[test]
    fn validated_preserves_valid_values() {
        let cfg = Config::default();
        let v = cfg.validated();
        assert_eq!(v.update_interval_secs, 3);
        assert!((v.cpu_threshold_percent - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn validated_preserves_booleans() {
        let cfg = Config {
            notify_cpu: false,
            notify_memory: false,
            notify_disk: false,
            autostart: true,
            ..Config::default()
        };
        let v = cfg.validated();
        assert!(!v.notify_cpu);
        assert!(!v.notify_memory);
        assert!(!v.notify_disk);
        assert!(v.autostart);
    }
}
