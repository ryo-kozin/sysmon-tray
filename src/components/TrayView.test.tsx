import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, beforeEach } from "vitest";
import { mockInvoke } from "../test/tauri-mock";
import TrayView from "./TrayView";

describe("TrayView", () => {
  beforeEach(() => {
    mockInvoke.mockClear();
  });

  it("renders the app title", () => {
    render(<TrayView />);
    expect(screen.getByText("System Monitor")).toBeInTheDocument();
  });

  it("shows loading state initially then data", async () => {
    render(<TrayView />);
    await waitFor(() => {
      expect(screen.getByText("CPU")).toBeInTheDocument();
    });
  });

  it("renders settings button with accessible label", () => {
    render(<TrayView />);
    expect(screen.getByRole("button", { name: "Settings" })).toBeInTheDocument();
  });

  it("switches to settings view on gear click", async () => {
    const user = userEvent.setup();
    render(<TrayView />);
    await user.click(screen.getByRole("button", { name: "Settings" }));
    await waitFor(() => {
      expect(screen.getByText("Settings")).toBeInTheDocument();
      expect(screen.getByRole("button", { name: "Save" })).toBeInTheDocument();
    });
  });

  it("fetches config to set polling interval", async () => {
    render(<TrayView />);
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith("get_config");
    });
  });
});
