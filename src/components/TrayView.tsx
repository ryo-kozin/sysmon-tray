import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSystemInfo } from "../hooks/useSystemInfo";
import StatusBar from "./StatusBar";
import Settings from "./Settings";

export default function TrayView() {
  const [view, setView] = useState<"status" | "settings">("status");
  const [intervalMs, setIntervalMs] = useState(3000);
  const info = useSystemInfo(intervalMs);

  useEffect(() => {
    invoke<{ update_interval_secs: number }>("get_config").then((cfg) => {
      setIntervalMs(cfg.update_interval_secs * 1000);
    });
  }, [view]);

  return (
    <div
      style={{
        width: 360,
        minHeight: 200,
        background: "rgba(30, 30, 30, 0.95)",
        borderRadius: 10,
        overflow: "hidden",
      }}
    >
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          padding: "10px 20px",
          borderBottom: "1px solid rgba(255,255,255,0.08)",
        }}
      >
        <span style={{ fontSize: 13, fontWeight: 700 }}>System Monitor</span>
        {view === "status" && (
          <button
            onClick={() => setView("settings")}
            style={{
              background: "none",
              border: "none",
              color: "#e0e0e0",
              cursor: "pointer",
              fontSize: 16,
              padding: 0,
              opacity: 0.6,
            }}
            title="Settings"
            aria-label="Settings"
          >
            &#9881;
          </button>
        )}
      </div>

      {view === "status" ? (
        info ? (
          <StatusBar info={info} />
        ) : (
          <div style={{ padding: 20, textAlign: "center", fontSize: 12, opacity: 0.5 }}>
            Loading...
          </div>
        )
      ) : (
        <Settings onBack={() => setView("status")} />
      )}
    </div>
  );
}
