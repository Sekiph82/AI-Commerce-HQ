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
invoke.mockImplementation(
  async (command: string, args?: { projectId?: string }) => {
    if (command === "hiveai_projects_list") return records;
    if (command === "hiveai_project_get") {
      return (
        records.find((record) => record.id === args?.projectId) ?? records[0]
      );
    }
    if (command === "hiveai_frontend_ready") return undefined;
    if (command === "hiveai_git_snapshot")
      return {
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
      };
    if (command === "hiveai_git_diff")
      return {
        text: "",
        binaryFiles: [],
        truncated: false,
        byteLimit: 1,
        lineLimit: 1,
      };
    if (command === "hiveai_database_status")
      return {
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
      };
    return {
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
    };
  },
);

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
  invoke.mockClear();
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
    expect(
      screen.getByRole("heading", { name: "Bulk-Edit" }),
    ).toBeInTheDocument();
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
        screen.getByRole("button", { name: "AI-Commerce-HQ" }),
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
