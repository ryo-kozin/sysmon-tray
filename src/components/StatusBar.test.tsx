import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import StatusBar from "./StatusBar";
import type { SystemInfo } from "../hooks/useSystemInfo";

const info: SystemInfo = {
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

describe("StatusBar", () => {
  it("renders CPU, Memory, and Disk bars", () => {
    render(<StatusBar info={info} />);
    expect(screen.getByText("CPU")).toBeInTheDocument();
    expect(screen.getByText("Memory")).toBeInTheDocument();
    expect(screen.getByText("Disk")).toBeInTheDocument();
  });

  it("displays CPU percentage", () => {
    render(<StatusBar info={info} />);
    expect(screen.getByText("42.5%")).toBeInTheDocument();
  });

  it("displays top process info", () => {
    render(<StatusBar info={info} />);
    expect(screen.getByText(/node \(12\.3%\)/)).toBeInTheDocument();
    expect(screen.getByText(/chrome \(1024 MB\)/)).toBeInTheDocument();
  });

  it("renders progress bars with correct aria attributes", () => {
    render(<StatusBar info={info} />);
    const bars = screen.getAllByRole("progressbar");
    expect(bars).toHaveLength(3);
    expect(bars[0]).toHaveAttribute("aria-valuenow", "42.5");
    expect(bars[0]).toHaveAttribute("aria-valuemin", "0");
    expect(bars[0]).toHaveAttribute("aria-valuemax", "100");
  });

  it("uses warning color for high CPU", () => {
    const highCpu = { ...info, cpu_usage: 85 };
    render(<StatusBar info={highCpu} />);
    const bars = screen.getAllByRole("progressbar");
    // High CPU should have red color (#ef4444)
    const inner = bars[0].firstElementChild as HTMLElement;
    expect(inner.style.background).toBe("rgb(239, 68, 68)");
  });
});
