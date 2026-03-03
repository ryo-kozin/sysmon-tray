import { vi } from "vitest";
import type { SystemInfo } from "../hooks/useSystemInfo";

export const mockSystemInfo: SystemInfo = {
  cpu_usage: 42.5,
  memory_total: 16_000_000_000,
  memory_used: 8_000_000_000,
  memory_percent: 50.0,
  disk_total: 500_000_000_000,
  disk_free: 250_000_000_000,
  disk_percent_used: 50.0,
  top_cpu_process: "node (12.3%)",
  top_mem_process: "chrome (1024 MB)",
};

export const mockConfig = {
  update_interval_secs: 3,
  cpu_threshold_percent: 80,
  cpu_sustained_secs: 10,
  memory_free_threshold_percent: 10,
  disk_free_threshold_gb: 10,
  notification_cooldown_mins: 15,
  notify_cpu: true,
  notify_memory: true,
  notify_disk: true,
  autostart: false,
};

export const mockInvoke = vi.fn(async (cmd: string) => {
  switch (cmd) {
    case "get_system_info":
      return mockSystemInfo;
    case "get_config":
      return mockConfig;
    case "save_config":
      return null;
    case "check_notifications":
      return [];
    default:
      return null;
  }
});

vi.mock("@tauri-apps/api/core", () => ({
  invoke: mockInvoke,
}));
