use serde::Serialize;
use std::sync::Mutex;
use sysinfo::{Disks, System};

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub memory_percent: f32,
    pub disk_total: u64,
    pub disk_free: u64,
    pub disk_percent_used: f32,
    pub top_cpu_process: String,
    pub top_mem_process: String,
}

pub struct MonitorState {
    pub system: Mutex<System>,
    pub disks: Mutex<Disks>,
}

impl MonitorState {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            system: Mutex::new(sys),
            disks: Mutex::new(Disks::new_with_refreshed_list()),
        }
    }

    pub fn refresh_and_get(&self) -> SystemInfo {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let mut disks = self.disks.lock().unwrap();
        disks.refresh(true);

        let cpu_usage = sys.global_cpu_usage();

        let memory_total = sys.total_memory();
        let memory_used = sys.used_memory();
        let memory_percent = if memory_total > 0 {
            (memory_used as f32 / memory_total as f32) * 100.0
        } else {
            0.0
        };

        let (disk_total, disk_free) = disks
            .iter()
            .filter(|d| {
                let mp = d.mount_point();
                mp == std::path::Path::new("/") || mp == std::path::Path::new("C:\\")
            })
            .map(|d| (d.total_space(), d.available_space()))
            .next()
            .or_else(|| {
                disks
                    .iter()
                    .max_by_key(|d| d.total_space())
                    .map(|d| (d.total_space(), d.available_space()))
            })
            .unwrap_or((0, 0));

        let disk_percent_used = if disk_total > 0 {
            ((disk_total - disk_free) as f32 / disk_total as f32) * 100.0
        } else {
            0.0
        };

        let top_cpu_process = sys
            .processes()
            .values()
            .max_by(|a, b| a.cpu_usage().partial_cmp(&b.cpu_usage()).unwrap())
            .map(|p| format!("{} ({:.1}%)", p.name().to_string_lossy(), p.cpu_usage()))
            .unwrap_or_default();

        let top_mem_process = sys
            .processes()
            .values()
            .max_by_key(|p| p.memory())
            .map(|p| {
                format!(
                    "{} ({:.0} MB)",
                    p.name().to_string_lossy(),
                    p.memory() as f64 / 1_048_576.0
                )
            })
            .unwrap_or_default();

        SystemInfo {
            cpu_usage,
            memory_total,
            memory_used,
            memory_percent,
            disk_total,
            disk_free,
            disk_percent_used,
            top_cpu_process,
            top_mem_process,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitor_returns_valid_metrics() {
        let monitor = MonitorState::new();
        // Need a small delay for CPU measurement accuracy
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        let info = monitor.refresh_and_get();

        assert!(info.memory_total > 0, "memory_total should be > 0");
        assert!(info.memory_used > 0, "memory_used should be > 0");
        assert!(info.memory_used <= info.memory_total);
        assert!(info.memory_percent >= 0.0 && info.memory_percent <= 100.0);
        assert!(info.disk_total > 0, "disk_total should be > 0");
        assert!(info.disk_free <= info.disk_total);
        assert!(info.disk_percent_used >= 0.0 && info.disk_percent_used <= 100.0);
        assert!(info.cpu_usage >= 0.0 && info.cpu_usage <= 100.0 * 128.0); // per-core can exceed 100
    }

    #[test]
    fn system_info_serializes() {
        let info = SystemInfo {
            cpu_usage: 42.5,
            memory_total: 16_000_000_000,
            memory_used: 8_000_000_000,
            memory_percent: 50.0,
            disk_total: 500_000_000_000,
            disk_free: 250_000_000_000,
            disk_percent_used: 50.0,
            top_cpu_process: "test (50.0%)".into(),
            top_mem_process: "test (500 MB)".into(),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("42.5"));
        assert!(json.contains("memory_total"));
    }

    #[test]
    fn multiple_refreshes_succeed() {
        let monitor = MonitorState::new();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        for _ in 0..3 {
            let info = monitor.refresh_and_get();
            assert!(info.memory_total > 0);
        }
    }
}
