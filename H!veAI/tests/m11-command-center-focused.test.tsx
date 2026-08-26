import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import React from "react";
import { BrowserRouter } from "react-router-dom";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { CommandCenterLive } from "../src/command_center_view";

const invoke = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn(() => Promise.resolve(() => undefined)) }));
vi.mock("../src/registryContext", () => ({
  useProjectRegistry: () => {
    const [selectedProjectId, setSelectedProjectId] = React.useState("project-1");
    return { records, projects: [], loading: false, error: null, selectedProjectId, selectProject: (projectId: string | null) => setSelectedProjectId(projectId), refresh: async () => undefined };
  },
}));

const records = [
  { id: "project-1", name: "Alpha", originalPath: "C:\\Projects\\Alpha", normalizedPath: "c:\\projects\\alpha", status: "ACTIVE", priority: 1, preferredBuilder: "Codex", preferredAuditor: null, taskSourcePolicy: "DISCOVER_STANDARD_FILES", registeredAt: "1", lastValidatedAt: "2", repository: null },
  { id: "project-2", name: "Beta", originalPath: "C:\\Projects\\Beta", normalizedPath: "c:\\projects\\beta", status: "ACTIVE", priority: 2, preferredBuilder: "Codex", preferredAuditor: null, taskSourcePolicy: "DISCOVER_STANDARD_FILES", registeredAt: "1", lastValidatedAt: "2", repository: null },
];

const snapshot = {
  generatedAt: "2026-08-26T12:00:00Z",
  projects: [
    { projectId: "project-1", name: "Alpha", registryStatus: "ACTIVE", health: "ATTENTION", manifestStatus: "VALID", taskAuthority: "CANONICAL", provenanceMode: "PROJECT_DASHBOARD", canonicalTaskSource: "TASKS.md", currentTask: { taskId: "task-1", title: "Canonical task", sourcePath: "TASKS.md", parsedStatus: "IN_PROGRESS", workflowState: "IMPLEMENTATION", requiredActor: "Codex" }, currentState: "IMPLEMENTATION", lastAction: null, nextAction: "Run focused tests", allowedActors: ["Codex"], totalTasks: 2, activeTasks: 1, completedTasks: 1, progressPercent: 50, warnings: [], refreshStatus: "SUCCESS", refreshAt: "2026-08-26T12:00:00Z", refreshError: null },
    { projectId: "project-2", name: "Beta", registryStatus: "ACTIVE", health: "UNKNOWN", manifestStatus: "ABSENT", taskAuthority: "NOT_CANONICALIZED", provenanceMode: "PROJECT_DASHBOARD", canonicalTaskSource: null, currentTask: null, currentState: null, lastAction: null, nextAction: null, allowedActors: [], totalTasks: null, activeTasks: null, completedTasks: null, progressPercent: null, warnings: ["No verified task source"], refreshStatus: "DEGRADED", refreshAt: "2026-08-26T12:00:00Z", refreshError: "M09 refresh unavailable" },
  ],
  kpis: { projects: 2, activeTasks: 1, needsAttention: 1, running: 0, completedTasks: 1, healthy: 0, healthDetail: "1 attention", authorityDetail: "1 canonical, 1 not canonicalized" },
  attention: [{ id: "attention-1", projectId: "project-1", projectName: "Alpha", taskId: "task-1", title: "Canonical task", state: "AUDIT_REQUIRED", detail: "Review evidence", category: "TASK" }],
  workQueue: [{ id: "queue-1", projectId: "project-1", projectName: "Alpha", taskId: "task-1", task: "Canonical task", stage: "IMPLEMENTATION", state: "RUNNING", actor: "Codex", updatedAt: "2026-08-26T12:00:00Z", attention: false }],
  recentActivity: [{ id: "activity-1", projectId: "project-1", projectName: "Alpha", kind: "TASK", event: "Task parsed", state: "EVIDENCE", actor: "watcher", occurredAt: "2026-08-26T12:00:00Z" }],
  engineeringBrief: { facts: [{ label: "Authority", value: "TASKS.md", source: "PROJECT_DASHBOARD", provenance: { sourceClass: "PROJECT_DASHBOARD", projectId: "project-1", sourcePath: "TASKS.md", evidenceType: "TASK_INTELLIGENCE_SNAPSHOT", evidenceId: null } }], recommendation: null },
  warnings: [],
};

describe("M11 Command Center evidence surface", () => {
  beforeEach(() => {
    invoke.mockReset();
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_command_center_snapshot") return Promise.resolve(snapshot);
      if (command === "hiveai_database_status") return Promise.resolve({ initialized: true, engine: "SQLite", schemaVersion: 7, migrationCount: 7, databasePath: "hiveai.db", foreignKeysEnabled: true, lastMigrationStatus: "ALREADY_CURRENT", journalMode: "WAL", busyTimeoutMs: 5000, synchronous: "NORMAL", integrityStatus: "ok" });
      if (command === "hiveai_watcher_status") return Promise.resolve({ running: true, queueDepth: 0, queueCapacity: 512, projects: [] });
      if (command === "hiveai_runtime_status") return Promise.resolve({ architectureMode: "RUST_NATIVE_NO_SIDECAR", sidecarEnabled: false, lastError: null, legacyCommerceRuntime: null, components: [], projects: [] });
      return Promise.resolve(undefined);
    });
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    window.history.pushState({}, "", "/");
  });

  afterEach(() => {
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
  });

  const renderCommandCenter = () => render(<BrowserRouter><CommandCenterLive /></BrowserRouter>);

  it("renders bounded native snapshot evidence and excludes recommendation generation", async () => {
    renderCommandCenter();
    expect(await screen.findByText("Canonical task")).toBeInTheDocument();
    expect(screen.getByText("1 canonical, 1 not canonicalized")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Beta" }));
    await waitFor(() => expect(screen.getByText("TASK AUTHORITY NOT YET CANONICALIZED")).toBeInTheDocument());
    expect(screen.queryByText(/GPT-4o/)).not.toBeInTheDocument();
    expect(screen.queryByText(/recommendation/i)).not.toBeInTheDocument();
  });

  it("keeps rail selection in place and opens the selected cockpit explicitly", async () => {
    renderCommandCenter();
    await screen.findByText("Canonical task");
    fireEvent.click(screen.getByRole("button", { name: "Beta" }));
    expect(screen.getByRole("heading", { name: "Beta" })).toBeInTheDocument();
    expect(window.location.pathname).toBe("/");
    fireEvent.click(screen.getByRole("button", { name: /Open cockpit/i }));
    await waitFor(() => expect(window.location.pathname).toBe("/projects/project-2"));
  });

  it("keeps home activity compact and leaves full history to the Activity route", async () => {
    renderCommandCenter();
    await screen.findAllByText("Task parsed");
    expect(screen.getByText("Recent activity")).toBeInTheDocument();
    expect(screen.queryByRole("textbox", { name: "Search recent activity" })).not.toBeInTheDocument();
    expect(screen.getByText("View activity")).toBeInTheDocument();
    expect(screen.getByText("Active Work Queue")).toBeInTheDocument();
  });

  it("uses a neutral browser preview identity without fixture actions", async () => {
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
    renderCommandCenter();
    expect(await screen.findByText("Preview / Native data unavailable")).toBeInTheDocument();
    expect(screen.queryByText(/FormuLab|Scrubbots/)).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Scrubbots|placeholder/i })).not.toBeInTheDocument();
  });

  it("keeps representative desktop home layouts bounded", async () => {
    for (const width of [1280, 1600]) {
      Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
      const view = renderCommandCenter();
      expect(view.container.querySelector(".command-activity-filter")).toBeNull();
      expect(view.container.querySelector(".project-rail")).toBeInTheDocument();
      expect(view.container.querySelector(".command-right-rail")).toBeInTheDocument();
      expect(screen.getByText("Active Work Queue")).toBeInTheDocument();
      view.unmount();
    }
  });
});
