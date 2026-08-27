import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";
import type { ProjectCockpitSnapshot } from "../src/projectCockpit";

const invoke = vi.hoisted(() => vi.fn());

const project = (id: string, name: string, status: "ACTIVE" | "MISSING" | "ARCHIVED" = "ACTIVE") => ({
  id,
  name,
  originalPath: `C:\\Projects\\${id}`,
  normalizedPath: `c:\\projects\\${id}`,
  status,
  priority: 0,
  preferredBuilder: null,
  preferredAuditor: null,
  taskSourcePolicy: "DISCOVER_STANDARD_FILES",
  registeredAt: "2026-08-27T10:00:00Z",
  lastValidatedAt: "2026-08-27T10:00:00Z",
  repository: null,
});

const records = [project("alpha", "Project Alpha"), project("beta", "Project Beta")];

function snapshotFor(id: string): ProjectCockpitSnapshot {
  const selected = records.find((record) => record.id === id) ?? project(id, id);
  const other = id === "alpha" ? "Project Beta" : "Project Alpha";
  const status = selected.status;
  return {
    project: selected,
    projectSummary: {
      projectId: id,
      health: status === "ACTIVE" ? "HEALTHY" : "UNKNOWN",
      manifestStatus: status === "ACTIVE" ? "AVAILABLE" : "ABSENT",
      taskAuthority: status === "ACTIVE" ? "PROJECT_DASHBOARD" : "UNKNOWN",
      provenanceMode: status === "ACTIVE" ? "PROJECT_DASHBOARD" : "UNKNOWN",
      currentTask: status === "ACTIVE" ? {
        taskId: `${id}-task-1`,
        title: `${selected.name} verified task`,
        sourcePath: "TASKS.md",
        parsedStatus: "IN_PROGRESS",
        workflowState: "IN_PROGRESS",
        requiredActor: "Codex",
      } : null,
      currentState: status === "ACTIVE" ? "IN_PROGRESS" : null,
      lastAction: null,
      nextAction: status === "ACTIVE" ? `Next action for ${selected.name}` : null,
      allowedActors: status === "ACTIVE" ? ["Codex"] : [],
      totalTasks: status === "ACTIVE" ? 1 : null,
      activeTasks: status === "ACTIVE" ? 1 : null,
      completedTasks: status === "ACTIVE" ? 0 : null,
      progressPercent: status === "ACTIVE" ? 25 : null,
      warnings: status === "ACTIVE" ? [] : [`${status} project state`],
      refreshStatus: "CURRENT",
      refreshAt: "2026-08-27T10:00:00Z",
      refreshError: status === "ACTIVE" ? null : "Project Dashboard unavailable",
    },
    dashboard: {
      projectId: id,
      manifestStatus: status === "ACTIVE" ? "AVAILABLE" : "ABSENT",
      manifestPath: ".hiveai/PROJECT_DASHBOARD.md",
      schema: status === "ACTIVE" ? "1" : null,
      projectKey: id,
      repository: null,
      branchPolicy: null,
      dashboardMode: "SINGLE_DASHBOARD",
      trackingMode: "REGISTERED_PROJECT",
      refreshPolicy: "WATCH_PROJECT_DASHBOARD",
      taskAuthority: status === "ACTIVE" ? "PROJECT_DASHBOARD" : "UNKNOWN",
      canonicalTaskSource: status === "ACTIVE" ? "TASKS.md" : null,
      roles: {},
      provenanceMode: status === "ACTIVE" ? "PROJECT_DASHBOARD" : "UNKNOWN",
      materialized: {
        projectStatus: status === "ACTIVE" ? "IMPLEMENTATION" : null,
        health: status === "ACTIVE" ? "HEALTHY" : null,
        currentMilestone: status === "ACTIVE" ? "M12" : null,
        currentTaskTitle: status === "ACTIVE" ? `${selected.name} verified task` : null,
        currentTaskId: status === "ACTIVE" ? `${id}-task-1` : null,
        declaredWorkflowState: status === "ACTIVE" ? "IN_PROGRESS" : null,
        progressRaw: status === "ACTIVE" ? "25%" : null,
        progressPercent: status === "ACTIVE" ? 25 : null,
        requiredActor: status === "ACTIVE" ? "Codex" : null,
        nextAction: status === "ACTIVE" ? `Next action for ${selected.name}` : null,
        waitingOn: null,
        lastMeaningfulUpdate: status === "ACTIVE" ? "2026-08-27" : null,
        currentWork: status === "ACTIVE" ? [{ id: `${id}-work`, item: `${selected.name} work`, status: "IN_PROGRESS", ownerActor: "Codex", evidenceSource: "PROJECT_DASHBOARD" }] : [],
        blockersWaiting: [],
        milestoneSummary: status === "ACTIVE" ? [`${selected.name} summary`] : [],
        qualityVerification: status === "ACTIVE" ? [{ label: "Typecheck", value: "PASS" }] : [],
        recentMeaningfulActivity: status === "ACTIVE" ? [`${selected.name} activity`] : [],
        provenance: status === "ACTIVE" ? [{ label: "Source", value: "PROJECT_DASHBOARD" }] : [],
      },
      warnings: status === "ACTIVE" ? [] : ["Project Dashboard is absent or unavailable"],
    },
    taskIntelligence: null,
    taskIntelligenceError: status === "ACTIVE" ? "No persisted M09 task intelligence" : "M09 unavailable",
    workflow: {
      projectId: id,
      tasks: status === "ACTIVE" ? [{
        taskId: `${id}-task-1`,
        projectId: id,
        title: `${selected.name} workflow task`,
        currentState: "IN_PROGRESS",
        workflowManaged: true,
        sourceActive: true,
        sourceRetired: false,
        allowedNextStates: ["AUDIT_REQUIRED"],
        allowedActors: ["HUMAN"],
        suspensionResumeState: null,
        latestEvent: null,
        attentionRequired: false,
        requiredActor: "HUMAN",
        milestone: "M12",
      }] : [],
    },
    workflowHistory: [],
    git: null,
    gitError: "NON_GIT_PROJECT: Git evidence unavailable",
    gitDiff: null,
    gitDiffError: null,
    sources: [],
    sourcesError: null,
    tests: [],
    audits: [],
    agentSessions: [],
    permissions: [],
    activity: status === "ACTIVE" ? [{ id: `${id}-activity`, kind: "PROJECT_DASHBOARD", event: `${selected.name} activity`, state: null, actor: null, occurredAt: "UNDATED", source: ".hiveai/PROJECT_DASHBOARD.md" }] : [],
    files: [],
    warnings: status === "ACTIVE" ? [] : ["Unknown or unavailable project evidence"],
    generatedAt: "2026-08-27T10:00:00Z",
  };
}

function defaultInvoke(command: string, args?: { projectId?: string; request?: { projectId?: string; priority?: number } }) {
  if (command === "hiveai_projects_list") return Promise.resolve(records);
  if (command === "hiveai_project_get") {
    const selected = records.find((record) => record.id === args?.projectId);
    return selected ? Promise.resolve(selected) : Promise.reject(new Error("project is not registered"));
  }
  if (command === "hiveai_project_cockpit_snapshot") {
    const id = args?.projectId;
    return id && records.some((record) => record.id === id)
      ? Promise.resolve(snapshotFor(id))
      : Promise.reject(new Error("project is not registered"));
  }
  if (command === "hiveai_project_update_settings") return Promise.resolve(project(args?.request?.projectId ?? "alpha", "Project Alpha"));
  if (command === "hiveai_frontend_ready") return Promise.resolve(undefined);
  if (command === "hiveai_database_status") return Promise.resolve({ initialized: true, engine: "SQLite", schemaVersion: 7, migrationCount: 7, databasePath: "hiveai.db", foreignKeysEnabled: true, lastMigrationStatus: "ALREADY_CURRENT", journalMode: "WAL", busyTimeoutMs: 5000, synchronous: "NORMAL", integrityStatus: "ok" });
  return Promise.resolve({ architectureMode: "RUST_NATIVE_NO_SIDECAR", sidecarEnabled: false, lastError: null, legacyCommerceRuntime: { componentId: "legacy", displayName: "Legacy", kind: "LEGACY_COMMERCE", state: "DISABLED", health: "DISABLED", startedAt: null, lastHeartbeat: null, restartCount: 0, lastError: null, ownership: "Excluded" }, components: [], projects: [] });
}

invoke.mockImplementation(defaultInvoke);
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

function renderLive(path: string) {
  Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
  window.history.pushState({}, "", path);
  return render(<App />);
}

beforeEach(() => {
  window.sessionStorage.clear();
  vi.stubGlobal("confirm", () => true);
  invoke.mockClear();
  invoke.mockImplementation(defaultInvoke);
});

describe("M12 project cockpit", () => {
  it("renders the selected project's live native read model and keeps evidence scoped", async () => {
    renderLive("/projects/alpha");
    expect(await screen.findByRole("heading", { name: "Project Alpha" })).toBeInTheDocument();
    expect(screen.getByText("Project Alpha verified task")).toBeInTheDocument();
    expect(screen.queryByText("Project Beta verified task")).not.toBeInTheDocument();
    expect(screen.getAllByText("PROJECT_DASHBOARD").length).toBeGreaterThan(0);
    fireEvent.click(screen.getByRole("button", { name: "Activity", exact: true }));
    expect(screen.getByText("UNDATED")).toBeInTheDocument();
  });

  it("ignores a stale earlier project snapshot after route changes", async () => {
    let resolveAlpha: ((value: ProjectCockpitSnapshot) => void) | undefined;
    invoke.mockImplementation((command: string, args?: { projectId?: string }) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_project_get" || command === "hiveai_project_cockpit_snapshot") {
        const id = args?.projectId ?? "alpha";
        if (id === "alpha") return new Promise((resolve) => { resolveAlpha = resolve; });
        return Promise.resolve(command === "hiveai_project_get" ? records[1] : snapshotFor("beta"));
      }
      return defaultInvoke(command, args);
    });
    renderLive("/projects/alpha");
    await waitFor(() => expect(screen.getByText(/Resolving registered project identity/)).toBeInTheDocument());
    window.history.pushState({}, "", "/projects/beta");
    fireEvent(window, new PopStateEvent("popstate"));
    expect(await screen.findByRole("heading", { name: "Project Beta" })).toBeInTheDocument();
    resolveAlpha?.(snapshotFor("alpha"));
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(screen.getByRole("heading", { name: "Project Beta" })).toBeInTheDocument();
    expect(screen.queryByText("Project Alpha verified task")).not.toBeInTheDocument();
  });

  it("renders unavailable and archived project truth without fake zeros", async () => {
    const archived = project("archived", "Archived Project", "ARCHIVED");
    records.push(archived);
    renderLive("/projects/archived");
    expect(await screen.findByRole("heading", { name: "Archived Project" })).toBeInTheDocument();
    expect(screen.getAllByText("ARCHIVED").length).toBeGreaterThan(0);
    expect(screen.getAllByText(/Unknown|unavailable/i).length).toBeGreaterThan(0);
    expect(screen.queryByText("0 tasks")).not.toBeInTheDocument();
    records.pop();
  });

  it("exposes all M12 views and keeps settings writes explicit", async () => {
    renderLive("/projects/alpha");
    await screen.findByRole("heading", { name: "Project Alpha" });
    const tabs = new Map([
      ["Tasks", "Canonical tasks"],
      ["Workflow", "Workflow pipeline"],
      ["Agents", "Agent sessions"],
      ["Audit", "Audit history"],
      ["Git", "Git visibility"],
      ["Tests", "Test-run history"],
      ["Activity", "Project activity"],
      ["Files", "Relevant files"],
      ["Settings", "Registry settings"],
    ]);
    for (const [tab, heading] of tabs) {
      fireEvent.click(screen.getByRole("button", { name: tab, exact: true }));
      await waitFor(() => expect(screen.getByRole("heading", { name: new RegExp(heading) })).toBeInTheDocument());
    }
    expect(invoke.mock.calls.filter(([command]) => command === "hiveai_project_update_settings")).toHaveLength(0);
    fireEvent.click(screen.getByRole("button", { name: "Save priority" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_project_update_settings", { request: { projectId: "alpha", priority: 0 } }));
    expect(screen.getByRole("status")).toHaveTextContent("Registry settings saved.");
    expect(within(screen.getByRole("status")).queryByText(/Project Beta/)).not.toBeInTheDocument();
  });

  it("requires rationale and records an explicit workflow correction event", async () => {
    renderLive("/projects/alpha");
    await screen.findByRole("heading", { name: "Project Alpha" });
    fireEvent.click(screen.getByRole("button", { name: "Workflow", exact: true }));
    fireEvent.click(screen.getByRole("button", { name: "Record correction" }));
    expect(screen.getByRole("status")).toHaveTextContent("cite evidence");
    fireEvent.change(screen.getByLabelText("Rationale"), { target: { value: "Human approved the state correction with audit evidence." } });
    fireEvent.change(screen.getByLabelText("Evidence reference"), { target: { value: "audit-123" } });
    fireEvent.click(screen.getByRole("button", { name: "Record correction" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_workflow_override", expect.objectContaining({ request: expect.objectContaining({ taskId: "alpha-task-1", toState: "AUDIT_REQUIRED", rationale: "Human approved the state correction with audit evidence.", evidenceRefs: [{ kind: "EXTERNAL_REFERENCE", id: "audit-123" }] }) })));
    expect(screen.getByRole("status")).toHaveTextContent("Correction recorded as a workflow event.");
  });
});
