import {
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";

const records = [
  {
    id: "ai-commerce-hq",
    name: "AI-Commerce-HQ",
    originalPath: "C:\\Projects\\AI-Commerce-HQ",
    normalizedPath: "c:\\projects\\ai-commerce-hq",
    status: "ACTIVE",
    priority: 1,
    preferredBuilder: "Codex",
    preferredAuditor: "GPT Audit",
    taskSourcePolicy: "DISCOVER_STANDARD_FILES",
    registeredAt: "1",
    lastValidatedAt: "2",
    repository: null,
  },
  {
    id: "bulk-edit",
    name: "Bulk-Edit",
    originalPath: "C:\\Projects\\Bulk-Edit",
    normalizedPath: "c:\\projects\\bulk-edit",
    status: "ACTIVE",
    priority: 1,
    preferredBuilder: "Codex",
    preferredAuditor: "GPT Audit",
    taskSourcePolicy: "DISCOVER_STANDARD_FILES",
    registeredAt: "1",
    lastValidatedAt: "2",
    repository: null,
  },
  {
    id: "fmcg",
    name: "fmcg-erp-system",
    originalPath: "C:\\Projects\\fmcg-erp-system",
    normalizedPath: "c:\\projects\\fmcg-erp-system",
    status: "MISSING",
    priority: 0,
    preferredBuilder: null,
    preferredAuditor: null,
    taskSourcePolicy: "DISCOVER_STANDARD_FILES",
    registeredAt: "1",
    lastValidatedAt: "2",
    repository: null,
  },
];

const invoke = vi.hoisted(() => vi.fn());
let liveRecords = records;

function defaultInvoke(command: string, args?: { projectId?: string }) {
  if (command === "hiveai_projects_list") return Promise.resolve(liveRecords);
  if (command === "hiveai_project_get") {
    const found = liveRecords.find((record) => record.id === args?.projectId);
    if (!found) return Promise.reject(new Error("project is not registered"));
    return Promise.resolve(found);
  }
  if (command === "hiveai_frontend_ready") return Promise.resolve(undefined);
  if (command === "hiveai_git_snapshot")
    return Promise.resolve({
      health: "CLEAN",
      currentBranch: "main",
      headSha: "abcdef1234567890",
      stagedFiles: [],
      unstagedFiles: [],
      untrackedFiles: [],
      conflictedFiles: [],
      remotes: [],
      recentCommits: [],
      worktrees: [],
      aheadCount: 0,
      behindCount: 0,
    });
  if (command === "hiveai_git_diff")
    return Promise.resolve({
      text: "",
      binaryFiles: [],
      truncated: false,
      byteLimit: 1,
      lineLimit: 1,
    });
  if (command === "hiveai_database_status")
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
  return Promise.resolve({
    architectureMode: "RUST_NATIVE_NO_SIDECAR",
    sidecarEnabled: false,
    lastError: null,
    legacyCommerceRuntime: {
      componentId: "legacy",
      displayName: "Legacy runtime",
      kind: "LEGACY_COMMERCE",
      state: "DISABLED",
      health: "DISABLED",
      startedAt: null,
      lastHeartbeat: null,
      restartCount: 0,
      lastError: null,
      ownership: "Excluded",
    },
    components: [],
    projects: [],
  });
}

invoke.mockImplementation(defaultInvoke);

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

function renderLive(path = "/") {
  Object.defineProperty(window, "__TAURI_INTERNALS__", {
    configurable: true,
    value: {},
  });
  window.history.pushState({}, "", path);
  return render(<App />);
}

beforeEach(() => {
  liveRecords = records;
  window.sessionStorage.clear();
  vi.stubGlobal("confirm", () => true);
  invoke.mockClear();
  invoke.mockImplementation(defaultInvoke);
});

describe("M07.06 live Registry and Command Center boundary", () => {
  it("command_center_renders_live_project_identity_without_formulab", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { name: "AI-Commerce-HQ" }),
      ).toBeInTheDocument(),
    );
    expect(screen.queryByText("FormuLab")).not.toBeInTheDocument();
  });

  it("command_center_live_project_count_reflects_registry", async () => {
    renderLive();
    await waitFor(() => expect(screen.getByText("3")).toBeInTheDocument());
  });

  it("command_center_project_rail_click_selects_in_place_without_navigation", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("button", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByRole("button", { name: "Bulk-Edit" }));
    expect(window.location.pathname).toBe("/");
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
  });

  it("command_center_selection_can_switch_between_live_projects", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("button", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByRole("button", { name: "Bulk-Edit" }));
    fireEvent.click(screen.getByRole("button", { name: "AI-Commerce-HQ" }));
    expect(
      screen.getByRole("heading", { name: "AI-Commerce-HQ" }),
    ).toBeInTheDocument();
  });

  it("command_center_selected_row_has_accessible_active_state", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("button", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByRole("button", { name: "Bulk-Edit" }));
    expect(screen.getByRole("button", { name: "Bulk-Edit" })).toHaveAttribute(
      "aria-pressed",
      "true",
    );
  });

  it("command_center_open_cockpit_uses_current_selected_project_id", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("button", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByRole("button", { name: "Bulk-Edit" }));
    await waitFor(() =>
      expect(screen.getByRole("heading", { name: "Bulk-Edit" })).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByRole("button", { name: /Open cockpit/ }));
    await waitFor(() =>
      expect(window.location.pathname).toBe("/projects/bulk-edit"),
    );
  });

  it("command_center_selection_persists_when_returning_during_session", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("button", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByRole("button", { name: "Bulk-Edit" }));
    fireEvent.click(screen.getByRole("button", { name: /Open cockpit/ }));
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByRole("link", { name: "Command Center" }));
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
  });

  it("command_center_project_rail_rows_render_name_only", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("button", { name: "fmcg-erp-system" }),
      ).toBeInTheDocument(),
    );
    const row = screen.getByRole("button", { name: "fmcg-erp-system" });
    expect(row).toHaveTextContent("fmcg-erp-system");
    expect(row.querySelector(".project-mark")).toBeNull();
    expect(row.querySelector(".rail-progress")).toBeNull();
    expect(row).not.toHaveTextContent("MISSING");
    expect(row).not.toHaveTextContent("Watch");
  });

  it("command_center_project_rail_long_name_preserves_accessible_full_name", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { name: "AI-Commerce-HQ" }),
      ).toBeInTheDocument(),
    );
    expect(
      screen.getByRole("button", { name: "AI-Commerce-HQ" }),
    ).toHaveAttribute("title", "AI-Commerce-HQ");
  });

  it("command_center_topbar_surfaces_are_distinct", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getByRole("button", { name: "Open command palette" }),
      ).toBeInTheDocument(),
    );
    fireEvent.click(
      screen.getByRole("button", { name: "Open command palette" }),
    );
    expect(
      screen.getByRole("heading", { name: "Command Palette" }),
    ).toBeInTheDocument();
    fireEvent.click(
      screen.getByRole("button", { name: "Close command palette" }),
    );
    fireEvent.click(screen.getByRole("button", { name: "Open AI Assistant" }));
    expect(
      within(screen.getByRole("dialog")).getByRole("heading", {
        name: "AI Assistant",
      }),
    ).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Close AI Assistant" }));
    fireEvent.click(screen.getByRole("button", { name: "Open Notifications" }));
    expect(
      screen.getByRole("heading", { name: "Notifications" }),
    ).toBeInTheDocument();
  });

  it("command_center_live_task_workflow_brief_are_truthful_placeholders", async () => {
    renderLive();
    await waitFor(() =>
      expect(
        screen.getAllByText("Task evidence unavailable").length,
      ).toBeGreaterThan(0),
    );
    expect(screen.getByText("Workflow state unavailable.")).toBeInTheDocument();
    expect(screen.queryByText("Priority: High")).not.toBeInTheDocument();
    expect(
      screen.queryByText("Claude is writing code..."),
    ).not.toBeInTheDocument();
    expect(screen.queryByText("12 tasks completed")).not.toBeInTheDocument();
  });
});

describe("M07.07 live-Registry / route-race boundary", () => {
  it("newly_registered_project_refresh_appears_in_rail", async () => {
    liveRecords = records.slice(0, 2);
    invoke.mockImplementation((command: string, args?: { projectId?: string; request?: { path?: string; name?: string | null } }) => {
      if (command === "hiveai_project_register") {
        liveRecords = [...liveRecords, records[2]];
        return Promise.resolve(records[2]);
      }
      return defaultInvoke(command, args);
    });
    renderLive("/projects");
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { name: "AI-Commerce-HQ" }),
      ).toBeInTheDocument(),
    );
    expect(screen.queryByRole("heading", { name: "fmcg-erp-system" })).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Add project" }));
    fireEvent.change(screen.getByLabelText("Folder path"), { target: { value: "C:\\Projects\\fmcg-erp-system" } });
    fireEvent.change(screen.getByLabelText(/Display name/), { target: { value: "fmcg-erp-system" } });
    fireEvent.click(screen.getByRole("button", { name: /Register folder/ }));
    await waitFor(() => expect(liveRecords).toHaveLength(3));
    fireEvent.click(screen.getByRole("link", { name: "Command Center" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "fmcg-erp-system" })).toBeInTheDocument());
  });

  it("sidebar_shortcuts_derive_from_live_registry_ids", async () => {
    renderLive();
    await waitFor(() =>
      expect(screen.getByText("Project shortcuts")).toBeInTheDocument(),
    );
    const shortcuts = screen
      .getByText("Project shortcuts")
      .closest("div")
      ?.parentElement?.querySelectorAll(".project-shortcuts button");
    expect(shortcuts).toBeDefined();
    if (shortcuts) {
      const names = Array.from(shortcuts).map((s) => s.textContent);
      expect(names.some((n) => n?.includes("AI-Commerce-HQ"))).toBe(true);
    }
  });

  it("archive_remove_refresh_removes_from_rail_and_shortcuts", async () => {
    invoke.mockImplementation((command: string, args?: { projectId?: string }) => {
      if (command === "hiveai_project_archive" && args?.projectId === "fmcg") {
        liveRecords = liveRecords.filter((record) => record.id !== "fmcg");
        return Promise.resolve(records[2]);
      }
      return defaultInvoke(command, args);
    });
    renderLive("/projects");
    await waitFor(() =>
      expect(
        screen.getByRole("button", { name: "Archive fmcg-erp-system" }),
      ).toBeInTheDocument(),
    );
    expect(screen.getAllByText("fmcg-erp-system").length).toBeGreaterThan(0);
    fireEvent.click(screen.getByRole("button", { name: "Archive fmcg-erp-system" }));
    await waitFor(() => expect(screen.queryAllByText("fmcg-erp-system")).toHaveLength(0));
    fireEvent.click(screen.getByRole("link", { name: "Command Center" }));
    await waitFor(() => expect(screen.queryByRole("button", { name: "fmcg-erp-system" })).not.toBeInTheDocument());
    expect(screen.queryAllByText("fmcg-erp-system")).toHaveLength(0);
  });

  it("pending_project_lookup_renders_loading_without_unrelated_identity", async () => {
    let resolveGet: ((value: unknown) => void) | null = null;
    invoke.mockImplementation(
      async (command: string, args?: { projectId?: string }) => {
        if (command === "hiveai_projects_list") return records;
        if (command === "hiveai_project_get") {
          return new Promise((resolve) => {
            resolveGet = resolve;
          });
        }
        return defaultInvoke(command, args);
      },
    );
    renderLive("/projects/ai-commerce-hq");
    await waitFor(() =>
      expect(
        screen.getByText(/Resolving registered project identity/),
      ).toBeInTheDocument(),
    );
    expect(screen.queryByText("FormuLab")).not.toBeInTheDocument();
    if (resolveGet) {
      resolveGet(records[0]);
    }
  });

  it("delayed_ai_commerce_hq_lookup_never_flashes_formulab", async () => {
    let resolveGet: ((value: unknown) => void) | null = null;
    invoke.mockImplementation(
      async (command: string, args?: { projectId?: string }) => {
        if (command === "hiveai_projects_list") return records;
        if (command === "hiveai_project_get") {
          return new Promise((resolve) => {
            resolveGet = resolve;
          });
        }
        return defaultInvoke(command, args);
      },
    );
    renderLive("/projects/ai-commerce-hq");
    await waitFor(() =>
      expect(
        screen.getByText(/Resolving registered project identity/),
      ).toBeInTheDocument(),
    );
    expect(screen.queryByText("FormuLab")).not.toBeInTheDocument();
    if (resolveGet) {
      resolveGet(records[0]);
      await waitFor(() =>
        expect(
          screen.getByRole("heading", { name: "AI-Commerce-HQ" }),
        ).toBeInTheDocument(),
      );
      expect(screen.queryByText("FormuLab")).not.toBeInTheDocument();
    }
  });

  it("stale_earlier_route_lookup_cannot_overwrite_newer_route", async () => {
    let firstResolve: ((value: unknown) => void) | null = null;
    let callCount = 0;
    invoke.mockImplementation(
      async (command: string, args?: { projectId?: string }) => {
        if (command === "hiveai_projects_list") return records;
        if (command === "hiveai_project_get") {
          callCount++;
          if (callCount === 1) {
            return new Promise((resolve) => {
              firstResolve = resolve;
            });
          }
          const found = records.find((r) => r.id === args?.projectId);
          if (!found) throw new Error("project is not registered");
          return found;
        }
        return defaultInvoke(command, args);
      },
    );
    renderLive("/projects/ai-commerce-hq");
    await waitFor(() =>
      expect(
        screen.getByText(/Resolving registered project identity/),
      ).toBeInTheDocument(),
    );
    window.history.pushState({}, "", "/projects/bulk-edit");
    fireEvent(window, new PopStateEvent("popstate"));
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { name: "Bulk-Edit" }),
      ).toBeInTheDocument(),
    );
    if (firstResolve) {
      firstResolve(records[0]);
    }
    expect(
      screen.getByRole("heading", { name: "Bulk-Edit" }),
    ).toBeInTheDocument();
    expect(
      screen.queryByRole("heading", { name: "AI-Commerce-HQ" }),
    ).not.toBeInTheDocument();
  });

  it("unknown_registered_id_renders_error_not_formulab", async () => {
    renderLive("/projects/nonexistent-project-id");
    await waitFor(() => {
      const errorText = screen.queryByText(
        /not found|not registered|could not be loaded/i,
      );
      expect(errorText).toBeInTheDocument();
    });
    expect(screen.queryByText("FormuLab")).not.toBeInTheDocument();
  });

  it("git_status_async_completion_cannot_change_project_identity", async () => {
    let gitResolve: ((value: unknown) => void) | null = null;
    invoke.mockImplementation(
      (command: string, args?: { projectId?: string }) => {
        if (command === "hiveai_git_snapshot") {
          return new Promise((resolve) => {
            gitResolve = resolve;
          });
        }
        return defaultInvoke(command, args);
      },
    );
    renderLive("/projects/ai-commerce-hq");
    await waitFor(() =>
      expect(
        screen.getByRole("heading", { name: "AI-Commerce-HQ" }),
      ).toBeInTheDocument(),
    );
    if (gitResolve) {
      gitResolve({
        health: "CLEAN",
        currentBranch: "main",
        headSha: "abc",
        stagedFiles: [],
        unstagedFiles: [],
        untrackedFiles: [],
        conflictedFiles: [],
        remotes: [],
        recentCommits: [],
        worktrees: [],
        aheadCount: 0,
        behindCount: 0,
      });
    }
    expect(screen.queryByText("FormuLab")).not.toBeInTheDocument();
    expect(
      screen.getByRole("heading", { name: "AI-Commerce-HQ" }),
    ).toBeInTheDocument();
  });

  it("browser_preview_fixtures_remain_isolated_from_tauri_live_mode", () => {
    delete (window as Window & { __TAURI_INTERNALS__?: unknown })
      .__TAURI_INTERNALS__;
    window.history.pushState({}, "", "/");
    const { container } = render(<App />);
    expect(container).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Preview / Native data unavailable" })).toBeInTheDocument();
    expect(invoke).not.toHaveBeenCalledWith("hiveai_projects_list", expect.anything());
    expect(screen.queryByRole("heading", { name: "AI-Commerce-HQ" })).not.toBeInTheDocument();
    expect(screen.queryByText("Resolving registered project identity")).not.toBeInTheDocument();
  });

  it("settings_renders_native_restart_action_in_tauri", async () => {
    renderLive("/settings");
    expect(await screen.findByRole("heading", { name: "Application" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Restart H!veAI" })).toBeInTheDocument();
  });

  it("settings_cancelled_restart_does_not_invoke_native_command", async () => {
    vi.stubGlobal("confirm", () => false);
    renderLive("/settings");
    fireEvent.click(await screen.findByRole("button", { name: "Restart H!veAI" }));
    expect(invoke).not.toHaveBeenCalledWith("hiveai_request_restart");
  });

  it("settings_confirmed_restart_invokes_native_command_once", async () => {
    vi.stubGlobal("confirm", () => true);
    renderLive("/settings");
    fireEvent.click(await screen.findByRole("button", { name: "Restart H!veAI" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_request_restart"));
    expect(invoke.mock.calls.filter(([command]) => command === "hiveai_request_restart")).toHaveLength(1);
  });

  it("settings_browser_preview_does_not_invoke_native_restart", async () => {
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
    window.history.pushState({}, "", "/settings");
    render(<App />);
    const button = await screen.findByRole("button", { name: "Restart H!veAI" });
    expect(button).toBeDisabled();
    fireEvent.click(button);
    expect(invoke).not.toHaveBeenCalledWith("hiveai_request_restart");
  });

  it("settings_navigation_reaches_restart_surface", async () => {
    renderLive("/");
    fireEvent.click(screen.getByRole("link", { name: "Settings" }));
    expect(await screen.findByRole("button", { name: "Restart H!veAI" })).toBeInTheDocument();
  });

  it("sidebar_uses_one_combined_brand_asset_without_legacy_copy", async () => {
    renderLive("/");
    const brands = await screen.findAllByRole("img", { name: "H!veAI" });
    expect(brands).toHaveLength(1);
    expect(brands[0]).toHaveAttribute("src", expect.stringContaining("hiveai-logo"));
    expect(screen.queryByRole("img", { name: "H!veAI emblem" })).not.toBeInTheDocument();
    expect(screen.queryByText("Development command center")).not.toBeInTheDocument();
  });

  it("product_chrome_omits_milestone_suffix", async () => {
    renderLive("/");
    expect(await screen.findByText("H!veAI 0.1.0")).toBeInTheDocument();
    expect(screen.queryByText(/M07\.06/)).not.toBeInTheDocument();
  });

  it("projects_uses_live_git_metadata_copy", async () => {
    renderLive("/projects");
    expect(await screen.findByText("Live Git metadata and status available")).toBeInTheDocument();
    expect(screen.queryByText("Live Git engine arrives in M06")).not.toBeInTheDocument();
  });
});
