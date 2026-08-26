import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { BrowserRouter } from "react-router-dom";
import { AppShell } from "../src/components/Shell";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("../src/registryContext", () => ({
  useProjectRegistry: () => ({ records: [], selectProject: vi.fn() }),
}));

describe("Akilta topbar link", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, "__TAURI_INTERNALS__", {
      configurable: true,
      value: {},
    });
  });

  it("moves the exact sentence into one native topbar target", () => {
    render(
      <BrowserRouter>
        <AppShell>content</AppShell>
      </BrowserRouter>,
    );

    expect(screen.queryByRole("contentinfo")).toBeNull();
    const link = screen.getByRole("link", { name: /Akilta/ });
    expect(link.textContent).toBe(
      "Built with ♥ for maximum productivity by Akilta",
    );
    expect(link.querySelector("img")).not.toBeNull();
    expect(link).toHaveAttribute("href", "https://www.akilta.com/");
    expect(link).toHaveAttribute("title", "Developed by Akilta");
    expect(link.closest(".topbar")).not.toBeNull();
    expect(screen.getByText("Workspace /")).toBeInTheDocument();
    expect(screen.getByText("Search workspace")).toBeInTheDocument();

    fireEvent.click(link);
    expect(invoke).toHaveBeenCalledWith("hiveai_open_akilta");
  });
});
