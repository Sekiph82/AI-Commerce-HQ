import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import App from "../src/App";
import { StartupIntro } from "../src/components/StartupIntro";

const invoke = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

let startupClaim: boolean | Error = true;

const stylesSource = readFileSync("src/styles.css", "utf8");
const mainSource = readFileSync("src/main.tsx", "utf8");
const introSource = readFileSync("src/components/StartupIntro.tsx", "utf8");

describe("M08.00B presentation remediation", () => {
  beforeEach(() => {
    invoke.mockReset();
    startupClaim = true;
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_startup_intro_claim") {
        return startupClaim instanceof Error
          ? Promise.reject(startupClaim)
          : Promise.resolve(startupClaim);
      }
      if (command === "hiveai_projects_list") return Promise.resolve([]);
      if (command === "hiveai_database_status") {
        return Promise.resolve({
          initialized: true,
          engine: "SQLite",
          schemaVersion: 7,
          migrationCount: 7,
          databasePath: "hiveai.db",
          foreignKeysEnabled: true,
          lastMigrationStatus: "ALREADY_CURRENT",
          journalMode: "WAL",
          busyTimeoutMs: 5000,
          synchronous: "NORMAL",
          integrityStatus: "ok",
        });
      }
      if (command === "hiveai_watcher_status") {
        return Promise.resolve({ running: true, queueDepth: 0, queueCapacity: 512, projects: [] });
      }
      if (command === "hiveai_runtime_status") {
        return Promise.resolve({
          architectureMode: "RUST_NATIVE_NO_SIDECAR",
          sidecarEnabled: false,
          lastError: null,
          legacyCommerceRuntime: null,
          components: [],
          projects: [],
        });
      }
      return Promise.resolve(undefined);
    });
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
    Object.defineProperty(HTMLMediaElement.prototype, "play", {
      configurable: true,
      value: vi.fn(() => Promise.resolve()),
    });
  });

  it("native_claim_true_renders_fullscreen_intro_while_app_is_mounted", async () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<><App /><StartupIntro /></>);
    expect(screen.getByRole("heading", { name: "Command Center" })).toBeInTheDocument();
    expect(screen.getByRole("dialog", { name: "H!veAI startup" })).toHaveClass("startup-intro");
    expect(await screen.findByLabelText("H!veAI opening video")).not.toHaveAttribute("controls");
    expect(invoke).toHaveBeenCalledWith("hiveai_startup_intro_claim");
  });

  it("claim_false_dismisses_without_playback_and_keeps_app_usable", async () => {
    startupClaim = false;
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<><App /><StartupIntro /></>);
    await waitFor(() => expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument());
    expect(HTMLMediaElement.prototype.play).not.toHaveBeenCalled();
    expect(screen.getByRole("heading", { name: "Command Center" })).toBeInTheDocument();
  });

  it("claim_failure_fails_open_to_a_usable_app", async () => {
    startupClaim = new Error("claim unavailable");
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<><App /><StartupIntro /></>);
    await waitFor(() => expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument());
    expect(screen.getByRole("heading", { name: "Command Center" })).toBeInTheDocument();
  });

  it("ended_video_fades_overlay_without_unmounting_app", async () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    const { container } = render(<><App /><StartupIntro /></>);
    const video = await screen.findByLabelText("H!veAI opening video");
    fireEvent.ended(video);
    expect(container.querySelector(".startup-intro-closing")).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument(), { timeout: 1000 });
    expect(screen.getByRole("heading", { name: "Command Center" })).toBeInTheDocument();
  });

  it("media_error_fades_overlay_and_leaves_app_usable", async () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<><App /><StartupIntro /></>);
    fireEvent.error(await screen.findByLabelText("H!veAI opening video"));
    await waitFor(() => expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument(), { timeout: 1000 });
    expect(screen.getByRole("heading", { name: "Command Center" })).toBeInTheDocument();
  });

  it("route_navigation_does_not_issue_another_native_claim", async () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<><App /><StartupIntro /></>);
    await screen.findByLabelText("H!veAI opening video");
    fireEvent.click(screen.getByRole("link", { name: "Projects" }));
    await screen.findByRole("heading", { name: "Projects" });
    expect(invoke.mock.calls.filter(([command]) => command === "hiveai_startup_intro_claim")).toHaveLength(1);
  });

  it("browser_preview_does_not_invoke_native_claim", () => {
    render(<StartupIntro />);
    expect(screen.queryByRole("dialog", { name: "H!veAI startup" })).not.toBeInTheDocument();
    expect(invoke).not.toHaveBeenCalled();
  });

  it("background_and_intro_css_are_outside_normal_flow", () => {
    expect(stylesSource).toContain(".main-area::before");
    expect(stylesSource).not.toContain(".app-shell::before");
    expect(stylesSource).toMatch(/\.startup-intro\{[^}]*position:fixed/);
    expect(stylesSource).toMatch(/\.startup-intro\{[^}]*inset:0/);
    expect(stylesSource).toMatch(/\.startup-intro\{[^}]*overflow:hidden/);
    expect(stylesSource).toContain(".startup-intro video");
    expect(stylesSource).toContain("object-fit:contain");
    expect(introSource).not.toContain("sessionStorage");
  });

  it("frontend_ready_remains_independent_of_intro_completion", () => {
    expect(mainSource).toContain('invoke("hiveai_frontend_ready")');
    expect(mainSource).toContain("<App />");
    expect(mainSource).toContain("<StartupIntro />");
  });
});
