import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";
import { StartupIntro } from "../src/components/StartupIntro";

describe("M08.00 presentation bootstrap", () => {
  beforeEach(() => {
    window.sessionStorage.removeItem("hiveai.startup-intro.played");
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
    Object.defineProperty(HTMLMediaElement.prototype, "play", {
      configurable: true,
      value: vi.fn(() => Promise.resolve()),
    });
  });

  it("native_root_shows_intro_overlay_while_app_can_mount", () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<><App /><StartupIntro /></>);
    expect(screen.getByRole("dialog", { name: "H!veAI startup" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Command Center" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Global Overview" })).toBeInTheDocument();
  });

  it("browser_preview_skips_native_intro_playback", () => {
    render(<StartupIntro />);
    expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument();
    expect(HTMLMediaElement.prototype.play).not.toHaveBeenCalled();
  });

  it("ended_video_fades_overlay_without_unmounting_app", async () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    const { container } = render(<><App /><StartupIntro /></>);
    fireEvent.ended(container.querySelector("video") as HTMLVideoElement);
    await waitFor(() => expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument(), { timeout: 1000 });
    expect(screen.getByRole("heading", { name: "Command Center" })).toBeInTheDocument();
  });

  it("media_error_removes_overlay_and_leaves_app_usable", async () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    const { container } = render(<><App /><StartupIntro /></>);
    fireEvent.error(container.querySelector("video") as HTMLVideoElement);
    await waitFor(() => expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument(), { timeout: 1000 });
    expect(screen.getByRole("heading", { name: "Command Center" })).toBeInTheDocument();
  });

  it("session_guard_prevents_replay_when_mounted_again", () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    const first = render(<StartupIntro />);
    expect(screen.getByRole("dialog", { name: "H!veAI startup" })).toBeInTheDocument();
    first.unmount();
    render(<StartupIntro />);
    expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument();
  });

  it("global_app_shell_is_shared_across_routes", () => {
    window.history.pushState({}, "", "/projects");
    const { container } = render(<App />);
    expect(container.querySelector(".app-shell")).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Projects" })).toBeInTheDocument();
    window.history.pushState({}, "", "/settings");
    fireEvent(window, new PopStateEvent("popstate"));
    expect(screen.getByRole("heading", { name: "Settings" })).toBeInTheDocument();
    expect(container.querySelector(".app-shell")).toBeInTheDocument();
  });
});
