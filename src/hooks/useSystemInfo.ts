import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface SystemInfo {
  cpu_usage: number;
  memory_total: number;
  memory_used: number;
  memory_percent: number;
  disk_total: number;
  disk_free: number;
  disk_percent_used: number;
  top_cpu_process: string;
  top_mem_process: string;
}

export function useSystemInfo(intervalMs: number) {
  const [info, setInfo] = useState<SystemInfo | null>(null);
  const timerRef = useRef<ReturnType<typeof setInterval>>();

  useEffect(() => {
    const fetch = async () => {
      try {
        const data = await invoke<SystemInfo>("get_system_info");
        setInfo(data);
        await invoke("check_notifications");
      } catch {
        // Backend not ready yet
      }
    };

    fetch();
    timerRef.current = setInterval(fetch, intervalMs);
    return () => clearInterval(timerRef.current);
  }, [intervalMs]);

  return info;
}
