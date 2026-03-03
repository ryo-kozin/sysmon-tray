import type { SystemInfo } from "../hooks/useSystemInfo";

interface Props {
  info: SystemInfo;
}

function formatBytes(bytes: number): string {
  const gb = bytes / 1_073_741_824;
  if (gb >= 1) return `${gb.toFixed(1)} GB`;
  return `${(bytes / 1_048_576).toFixed(0)} MB`;
}

function Bar({ label, percent, detail, color }: {
  label: string;
  percent: number;
  detail: string;
  color: string;
}) {
  return (
    <div style={{ marginBottom: 12 }}>
      <div style={{ display: "flex", justifyContent: "space-between", fontSize: 12, marginBottom: 4 }}>
        <span style={{ fontWeight: 600 }}>{label}</span>
        <span style={{ opacity: 0.7 }}>{detail}</span>
      </div>
      <div style={{
        height: 6,
        borderRadius: 3,
        background: "rgba(255,255,255,0.1)",
        overflow: "hidden",
      }}>
        <div style={{
          height: "100%",
          width: `${Math.min(percent, 100)}%`,
          borderRadius: 3,
          background: color,
          transition: "width 0.3s ease",
        }} />
      </div>
    </div>
  );
}

export default function StatusBar({ info }: Props) {
  const cpuColor = info.cpu_usage > 80 ? "#ef4444" : info.cpu_usage > 50 ? "#f59e0b" : "#22c55e";
  const memColor = info.memory_percent > 90 ? "#ef4444" : info.memory_percent > 70 ? "#f59e0b" : "#3b82f6";
  const diskColor = info.disk_percent_used > 90 ? "#ef4444" : info.disk_percent_used > 70 ? "#f59e0b" : "#8b5cf6";

  return (
    <div style={{ padding: "16px 20px" }}>
      <Bar
        label="CPU"
        percent={info.cpu_usage}
        detail={`${info.cpu_usage.toFixed(1)}%`}
        color={cpuColor}
      />
      <Bar
        label="Memory"
        percent={info.memory_percent}
        detail={`${formatBytes(info.memory_used)} / ${formatBytes(info.memory_total)}`}
        color={memColor}
      />
      <Bar
        label="Disk"
        percent={info.disk_percent_used}
        detail={`${formatBytes(info.disk_free)} free`}
        color={diskColor}
      />
      <div style={{ fontSize: 11, opacity: 0.5, marginTop: 8 }}>
        <div>Top CPU: {info.top_cpu_process}</div>
        <div>Top Mem: {info.top_mem_process}</div>
      </div>
    </div>
  );
}
