import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { StartupIntro } from "../src/components/StartupIntro";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const tauriInternalsKey = "__TAURI_INTERNALS__";

function setNative(value: boolean) {
  if (value) {
    Object.defineProperty(window, tauriInternalsKey, {
      configurable: true,
      value: {},
    });
  } else {
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
  }
}

describe("pre-M10 native UX hotfix", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setNative(false);
    vi.spyOn(HTMLMediaElement.prototype, "play").mockResolvedValue(undefined);
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
    setNative(false);
  });

  it("prepares claimed native startup playback for audible media", async () => {
    setNative(true);
    vi.mocked(invoke).mockResolvedValueOnce(true);

    render(<StartupIntro />);
    const video = await screen.findByLabelText("H!veAI opening video");

    await waitFor(() => expect(video).toHaveProperty("muted", false));
    expect(video).toHaveProperty("volume", 1);
    expect(HTMLMediaElement.prototype.play).toHaveBeenCalled();
    expect(video).not.toHaveAttribute("muted");
  });

  it("dismisses safely when claimed startup playback ends", async () => {
    setNative(true);
    vi.mocked(invoke).mockResolvedValueOnce(true);

    render(<StartupIntro />);
    const video = await screen.findByLabelText("H!veAI opening video");
    vi.useFakeTimers();
    await act(async () => {
      fireEvent.ended(video);
      vi.advanceTimersByTime(280);
    });

    expect(screen.queryByLabelText("H!veAI opening video")).not.toBeInTheDocument();
  });

  it("does not run native intro behavior in browser preview", () => {
    render(<StartupIntro />);
    expect(screen.queryByRole("video", { name: "H!veAI opening video" })).not.toBeInTheDocument();
    expect(invoke).not.toHaveBeenCalled();
    expect(HTMLMediaElement.prototype.play).not.toHaveBeenCalled();
  });

  it("does not replay after the native process claim is consumed", async () => {
    setNative(true);
    vi.mocked(invoke)
      .mockResolvedValueOnce(true)
      .mockResolvedValueOnce(false);

    const first = render(<StartupIntro />);
    await screen.findByLabelText("H!veAI opening video");
    first.rerender(<StartupIntro />);
    await waitFor(() => expect(invoke).toHaveBeenCalledTimes(1));
    expect(screen.getByLabelText("H!veAI opening video")).toBeInTheDocument();
    expect(HTMLMediaElement.prototype.play).toHaveBeenCalledTimes(1);
  });
});

describe("native WebView2 audio configuration", () => {
  it("preserves WRY defaults while enabling audible autoplay", async () => {
    const config = await import("../src-tauri/tauri.conf.json");
    const args = config.default.app.windows[0].additionalBrowserArgs;
    expect(args).toContain("--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection");
    expect(args).toContain("--autoplay-policy=no-user-gesture-required");
  });
});
