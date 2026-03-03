import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, beforeEach } from "vitest";
import { mockInvoke } from "../test/tauri-mock";
import Settings from "./Settings";

describe("Settings", () => {
  const onBack = vi.fn();

  beforeEach(() => {
    mockInvoke.mockClear();
    onBack.mockClear();
  });

  it("loads and displays config values", async () => {
    render(<Settings onBack={onBack} />);
    await waitFor(() => {
      expect(screen.getByLabelText("Update interval (sec)")).toHaveValue(3);
    });
    expect(screen.getByLabelText("CPU threshold (%)")).toHaveValue(80);
    expect(screen.getByLabelText("Cooldown (min)")).toHaveValue(15);
  });

  it("renders all toggle switches", async () => {
    render(<Settings onBack={onBack} />);
    await waitFor(() => {
      expect(screen.getByLabelText("CPU notification")).toBeInTheDocument();
    });
    expect(screen.getByLabelText("Memory notification")).toBeInTheDocument();
    expect(screen.getByLabelText("Disk notification")).toBeInTheDocument();
    expect(screen.getByLabelText("Launch at login")).toBeInTheDocument();
  });

  it("calls save_config on Save click", async () => {
    const user = userEvent.setup();
    render(<Settings onBack={onBack} />);
    await waitFor(() => {
      expect(screen.getByLabelText("Update interval (sec)")).toBeInTheDocument();
    });
    await user.click(screen.getByRole("button", { name: "Save" }));
    expect(mockInvoke).toHaveBeenCalledWith("save_config", expect.objectContaining({}));
    expect(onBack).toHaveBeenCalled();
  });

  it("calls onBack when Back is clicked", async () => {
    const user = userEvent.setup();
    render(<Settings onBack={onBack} />);
    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Back" })).toBeInTheDocument();
    });
    await user.click(screen.getByRole("button", { name: "Back" }));
    expect(onBack).toHaveBeenCalled();
  });
});
