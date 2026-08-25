import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { BrowserRouter } from "react-router-dom";
import { AppShell } from "../src/components/Shell";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("../src/registryContext", () => ({
  useProjectRegistry: () => ({ records: [], selectProject: vi.fn() }),
}));

describe("Akilta footer link", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, "__TAURI_INTERNALS__", {
      configurable: true,
      value: {},
    });
  });

  it("preserves the exact sentence and exposes Akilta as a native link", () => {
    render(
      <BrowserRouter>
        <AppShell>content</AppShell>
      </BrowserRouter>,
    );

    const footer = screen.getByRole("contentinfo");
    expect(footer.textContent).toBe(
      "Built with ♥ for maximum productivity by Akilta",
    );
    const link = screen.getByRole("link", { name: "Akilta" });
    expect(link).toHaveAttribute("href", "https://www.akilta.com/");

    fireEvent.click(link);
    expect(invoke).toHaveBeenCalledWith("hiveai_open_akilta");
  });
});
