use crate::config::Config;
use crate::monitor::SystemInfo;
use std::sync::Mutex;
use std::time::Instant;

pub struct NotifierState {
    cpu_high_since: Mutex<Option<Instant>>,
    last_cpu_notify: Mutex<Option<Instant>>,
    last_mem_notify: Mutex<Option<Instant>>,
    last_disk_notify: Mutex<Option<Instant>>,
}

impl NotifierState {
    pub fn new() -> Self {
        Self {
            cpu_high_since: Mutex::new(None),
            last_cpu_notify: Mutex::new(None),
            last_mem_notify: Mutex::new(None),
            last_disk_notify: Mutex::new(None),
        }
    }

    pub fn check(&self, info: &SystemInfo, config: &Config) -> Vec<Notification> {
        let mut notifications = Vec::new();
        let cooldown = std::time::Duration::from_secs(config.notification_cooldown_mins * 60);

        if config.notify_cpu {
            let mut high_since = self.cpu_high_since.lock().unwrap();
            if info.cpu_usage >= config.cpu_threshold_percent {
                let start = high_since.get_or_insert_with(Instant::now);
                if start.elapsed().as_secs() >= config.cpu_sustained_secs {
                    let mut last = self.last_cpu_notify.lock().unwrap();
                    if last.is_none_or(|t| t.elapsed() >= cooldown) {
                        notifications.push(Notification {
                            title: "CPU Usage High".into(),
                            body: format!(
                                "CPU at {:.1}% for {}s+. Top: {}",
                                info.cpu_usage, config.cpu_sustained_secs, info.top_cpu_process
                            ),
                        });
                        *last = Some(Instant::now());
                    }
                }
            } else {
                *high_since = None;
            }
        }

        if config.notify_memory {
            let free_percent = 100.0 - info.memory_percent;
            if free_percent < config.memory_free_threshold_percent {
                let mut last = self.last_mem_notify.lock().unwrap();
                if last.is_none_or(|t| t.elapsed() >= cooldown) {
                    notifications.push(Notification {
                        title: "Memory Low".into(),
                        body: format!(
                            "Free memory {:.1}%. Top: {}",
                            free_percent, info.top_mem_process
                        ),
                    });
                    *last = Some(Instant::now());
                }
            }
        }

        if config.notify_disk {
            let free_gb = info.disk_free as f64 / 1_073_741_824.0;
            if free_gb < config.disk_free_threshold_gb {
                let mut last = self.last_disk_notify.lock().unwrap();
                if last.is_none_or(|t| t.elapsed() >= cooldown) {
                    notifications.push(Notification {
                        title: "Disk Space Low".into(),
                        body: format!("Free disk space: {:.1} GB", free_gb),
                    });
                    *last = Some(Instant::now());
                }
            }
        }

        notifications
    }
}

pub struct Notification {
    pub title: String,
    pub body: String,
}
