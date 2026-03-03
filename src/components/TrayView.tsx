import { useState } from "react";
import { useSystemInfo } from "../hooks/useSystemInfo";
import StatusBar from "./StatusBar";
import Settings from "./Settings";

export default function TrayView() {
  const [view, setView] = useState<"status" | "settings">("status");
  const info = useSystemInfo(3000);

  return (
    <div style={{
      width: 360,
      minHeight: 200,
      background: "rgba(30, 30, 30, 0.95)",
      borderRadius: 10,
      overflow: "hidden",
    }}>
      <div style={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        padding: "10px 20px",
        borderBottom: "1px solid rgba(255,255,255,0.08)",
      }}>
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
