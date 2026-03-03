import { useState, useEffect, useId } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Config {
  update_interval_secs: number;
  cpu_threshold_percent: number;
  cpu_sustained_secs: number;
  memory_free_threshold_percent: number;
  disk_free_threshold_gb: number;
  notification_cooldown_mins: number;
  notify_cpu: boolean;
  notify_memory: boolean;
  notify_disk: boolean;
  autostart: boolean;
}

interface Props {
  onBack: () => void;
}

function NumberInput({
  label,
  value,
  onChange,
  min,
  max,
  step,
}: {
  label: string;
  value: number;
  onChange: (v: number) => void;
  min?: number;
  max?: number;
  step?: number;
}) {
  const id = useId();
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        marginBottom: 10,
      }}
    >
      <label htmlFor={id} style={{ fontSize: 12 }}>
        {label}
      </label>
      <input
        id={id}
        type="number"
        value={value}
        min={min}
        max={max}
        step={step ?? 1}
        onChange={(e) => onChange(Number(e.target.value))}
        style={{
          width: 70,
          padding: "2px 6px",
          fontSize: 12,
          background: "rgba(255,255,255,0.1)",
          border: "1px solid rgba(255,255,255,0.2)",
          borderRadius: 4,
          color: "#e0e0e0",
        }}
      />
    </div>
  );
}

function Toggle({
  label,
  checked,
  onChange,
}: {
  label: string;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  const id = useId();
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        marginBottom: 10,
      }}
    >
      <label htmlFor={id} style={{ fontSize: 12 }}>
        {label}
      </label>
      <input
        id={id}
        type="checkbox"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
      />
    </div>
  );
}

export default function Settings({ onBack }: Props) {
  const [config, setConfig] = useState<Config | null>(null);

  useEffect(() => {
    invoke<Config>("get_config").then(setConfig);
  }, []);

  if (!config) return null;

  const update = (patch: Partial<Config>) => {
    setConfig({ ...config, ...patch });
  };

  const save = async () => {
    await invoke("save_config", { newConfig: config });
    onBack();
  };

  return (
    <div style={{ padding: "16px 20px" }}>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          marginBottom: 16,
        }}
      >
        <h3 style={{ fontSize: 14, margin: 0 }}>Settings</h3>
        <button
          onClick={save}
          style={{
            padding: "4px 12px",
            fontSize: 12,
            background: "#3b82f6",
            border: "none",
            borderRadius: 4,
            color: "#fff",
            cursor: "pointer",
          }}
        >
          Save
        </button>
      </div>

      <NumberInput
        label="Update interval (sec)"
        value={config.update_interval_secs}
        onChange={(v) => update({ update_interval_secs: v })}
        min={1}
        max={60}
      />
      <NumberInput
        label="CPU threshold (%)"
        value={config.cpu_threshold_percent}
        onChange={(v) => update({ cpu_threshold_percent: v })}
        min={1}
        max={100}
      />
      <NumberInput
        label="CPU sustained (sec)"
        value={config.cpu_sustained_secs}
        onChange={(v) => update({ cpu_sustained_secs: v })}
        min={1}
        max={300}
      />
      <NumberInput
        label="Memory free threshold (%)"
        value={config.memory_free_threshold_percent}
        onChange={(v) => update({ memory_free_threshold_percent: v })}
        min={1}
        max={50}
      />
      <NumberInput
        label="Disk free threshold (GB)"
        value={config.disk_free_threshold_gb}
        onChange={(v) => update({ disk_free_threshold_gb: v })}
        min={1}
        max={100}
        step={0.5}
      />
      <NumberInput
        label="Cooldown (min)"
        value={config.notification_cooldown_mins}
        onChange={(v) => update({ notification_cooldown_mins: v })}
        min={1}
        max={120}
      />

      <div style={{ borderTop: "1px solid rgba(255,255,255,0.1)", marginTop: 12, paddingTop: 12 }}>
        <Toggle
          label="CPU notification"
          checked={config.notify_cpu}
          onChange={(v) => update({ notify_cpu: v })}
        />
        <Toggle
          label="Memory notification"
          checked={config.notify_memory}
          onChange={(v) => update({ notify_memory: v })}
        />
        <Toggle
          label="Disk notification"
          checked={config.notify_disk}
          onChange={(v) => update({ notify_disk: v })}
        />
        <Toggle
          label="Launch at login"
          checked={config.autostart}
          onChange={(v) => update({ autostart: v })}
        />
      </div>

      <button
        onClick={onBack}
        style={{
          marginTop: 8,
          padding: "4px 12px",
          fontSize: 12,
          background: "transparent",
          border: "1px solid rgba(255,255,255,0.2)",
          borderRadius: 4,
          color: "#e0e0e0",
          cursor: "pointer",
        }}
      >
        Back
      </button>
    </div>
  );
}
