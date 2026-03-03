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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::monitor::SystemInfo;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn make_info(cpu: f32, mem_percent: f32, disk_free_bytes: u64) -> SystemInfo {
        SystemInfo {
            cpu_usage: cpu,
            memory_total: 16_000_000_000,
            memory_used: (16_000_000_000.0 * mem_percent / 100.0) as u64,
            memory_percent: mem_percent,
            disk_total: 500_000_000_000,
            disk_free: disk_free_bytes,
            disk_percent_used: 0.0,
            top_cpu_process: "test (50.0%)".into(),
            top_mem_process: "test (500 MB)".into(),
        }
    }

    #[test]
    fn no_notifications_when_all_normal() {
        let notifier = NotifierState::new();
        let config = Config::default();
        // CPU 20%, memory 50% used (50% free), 100GB disk free
        let info = make_info(20.0, 50.0, 100 * 1_073_741_824);
        let result = notifier.check(&info, &config);
        assert!(result.is_empty());
    }

    #[test]
    fn no_cpu_notification_before_sustained() {
        let notifier = NotifierState::new();
        let config = Config {
            cpu_sustained_secs: 10,
            ..Config::default()
        };
        // CPU high but not sustained yet (first check)
        let info = make_info(95.0, 50.0, 100 * 1_073_741_824);
        let result = notifier.check(&info, &config);
        assert!(
            result.is_empty(),
            "should not notify on first high CPU check"
        );
    }

    #[test]
    fn memory_low_triggers_notification() {
        let notifier = NotifierState::new();
        // threshold: 10% free; memory 95% used → 5% free
        let config = Config::default();
        let info = make_info(20.0, 95.0, 100 * 1_073_741_824);
        let result = notifier.check(&info, &config);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "Memory Low");
    }

    #[test]
    fn disk_low_triggers_notification() {
        let notifier = NotifierState::new();
        // threshold: 10 GB; 5 GB free
        let config = Config::default();
        let info = make_info(20.0, 50.0, 5 * 1_073_741_824);
        let result = notifier.check(&info, &config);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "Disk Space Low");
    }

    #[test]
    fn memory_cooldown_prevents_repeat() {
        let notifier = NotifierState::new();
        let config = Config {
            notification_cooldown_mins: 15,
            ..Config::default()
        };
        let info = make_info(20.0, 95.0, 100 * 1_073_741_824);

        let first = notifier.check(&info, &config);
        assert_eq!(first.len(), 1);

        // Second check immediately → cooldown blocks
        let second = notifier.check(&info, &config);
        assert!(
            second.is_empty(),
            "cooldown should prevent repeat notification"
        );
    }

    #[test]
    fn disk_cooldown_prevents_repeat() {
        let notifier = NotifierState::new();
        let config = Config::default();
        let info = make_info(20.0, 50.0, 5 * 1_073_741_824);

        let first = notifier.check(&info, &config);
        assert_eq!(first.len(), 1);

        let second = notifier.check(&info, &config);
        assert!(second.is_empty());
    }

    #[test]
    fn disabled_notifications_not_sent() {
        let notifier = NotifierState::new();
        let config = Config {
            notify_cpu: false,
            notify_memory: false,
            notify_disk: false,
            ..Config::default()
        };
        // All thresholds exceeded but notifications disabled
        let info = make_info(99.0, 99.0, 1_073_741_824);
        let result = notifier.check(&info, &config);
        assert!(result.is_empty());
    }

    #[test]
    fn cpu_resets_when_drops_below_threshold() {
        let notifier = NotifierState::new();
        let config = Config::default();

        // CPU high
        let info_high = make_info(95.0, 50.0, 100 * 1_073_741_824);
        notifier.check(&info_high, &config);

        // CPU drops below
        let info_low = make_info(20.0, 50.0, 100 * 1_073_741_824);
        notifier.check(&info_low, &config);

        // cpu_high_since should be reset (None)
        let high_since = notifier.cpu_high_since.lock().unwrap();
        assert!(high_since.is_none());
    }

    #[test]
    fn multiple_alerts_can_fire_simultaneously() {
        let notifier = NotifierState::new();
        let config = Config {
            cpu_sustained_secs: 0, // instant trigger for testing
            ..Config::default()
        };
        // CPU high (sustained_secs=0 means immediate), memory low, disk low
        let info = make_info(95.0, 95.0, 1_073_741_824);

        // First call: cpu_high_since is set, but elapsed is 0 which >= 0
        let result = notifier.check(&info, &config);
        // Memory and Disk should fire; CPU may fire too since sustained=0
        assert!(result.len() >= 2, "expected at least memory + disk alerts");
    }
}
