import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { RegistryProvider, useProjectRegistry } from "../src/registryContext";
import { Tasks } from "../src/pages";

const invoke = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

const records = [
  { id: "project-a", name: "Project A", originalPath: "C:\\Projects\\A", normalizedPath: "c:\\projects\\a", status: "ACTIVE", priority: 1, preferredBuilder: "Codex", preferredAuditor: "GPT Audit", taskSourcePolicy: "DISCOVER_STANDARD_FILES", registeredAt: "1", lastValidatedAt: "2", repository: null },
  { id: "project-b", name: "Project B", originalPath: "C:\\Projects\\B", normalizedPath: "c:\\projects\\b", status: "ACTIVE", priority: 1, preferredBuilder: "Codex", preferredAuditor: "GPT Audit", taskSourcePolicy: "DISCOVER_STANDARD_FILES", registeredAt: "1", lastValidatedAt: "2", repository: null },
];
const source = (projectId: string, path = "TASKS.md") => ({ id: `${projectId}-${path}`, projectId, relativePath: path, absolutePath: `C:\\Projects\\${projectId}\\${path}`, sourceKind: "TASKS", origin: "STANDARD", status: "AVAILABLE", authorityClass: "TASKS", priority: 10, sizeBytes: 20, modifiedAt: "2026-08-25T10:00:00Z", discoveredAt: "2026-08-25T10:01:00Z", contentHash: "hash", depth: 0, warnings: [], schemaVersion: 1, owner: "M08_TASK_SOURCE_DISCOVERY", sourceOrder: null });

function defaultInvoke(command: string, args?: { projectId?: string }) {
  if (command === "hiveai_projects_list") return Promise.resolve(records);
  if (command === "hiveai_task_sources_list") return Promise.resolve([source(args?.projectId ?? "project-a")]);
  if (command === "hiveai_task_source_custom_paths_list") return Promise.resolve([]);
  if (command === "hiveai_task_sources_discover") return Promise.resolve([source(args?.projectId ?? "project-a", "handoffs/current.md")]);
  if (command === "hiveai_task_source_custom_path_add" || command === "hiveai_task_source_custom_path_remove" || command === "hiveai_task_source_custom_path_update") return Promise.resolve([]);
  return Promise.resolve(undefined);
}

function SelectionHarness() {
  const { selectProject } = useProjectRegistry();
  return <div><button type="button" onClick={() => selectProject("project-a")}>Select A</button><button type="button" onClick={() => selectProject("project-b")}>Select B</button><Tasks /></div>;
}

function renderTasks() {
  Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
  return render(<RegistryProvider><MemoryRouter initialEntries={["/tasks"]}><Tasks /></MemoryRouter></RegistryProvider>);
}

beforeEach(() => {
  invoke.mockReset();
  invoke.mockImplementation(defaultInvoke);
  delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
});

describe("M08 Task Sources live workspace", () => {
  it("native_tasks_uses_selected_live_registry_project", async () => {
    renderTasks();
    await screen.findByText("TASKS.md");
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_task_sources_list", { projectId: "project-a" }));
  });

  it("shows_loading_before_source_response", async () => {
    let resolve!: (value: unknown) => void;
    invoke.mockImplementation((command: string) => command === "hiveai_projects_list" ? Promise.resolve(records) : command === "hiveai_task_sources_list" ? new Promise((done) => { resolve = done; }) : Promise.resolve([]));
    renderTasks();
    expect(await screen.findByText("Loading workspace")).toBeInTheDocument();
    resolve([source("project-a")]);
  });

  it("renders_real_source_metadata_and_rescan_refreshes", async () => {
    renderTasks();
    expect(await screen.findByText("TASKS.md")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /Rescan sources/i }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_task_sources_discover", { projectId: "project-a" }));
    expect(screen.getByText("STANDARD")).toBeInTheDocument();
    expect(screen.getByText("AVAILABLE")).toBeInTheDocument();
  });

  it("custom_add_command_uses_native_boundary", async () => {
    renderTasks();
    await screen.findByText("TASKS.md");
    const input = screen.getByRole("textbox", { name: "Custom source path" });
    fireEvent.change(input, { target: { value: "evidence.md" } });
    fireEvent.click(screen.getByRole("button", { name: /Add path/i }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_task_source_custom_path_add", { request: { projectId: "project-a", path: "evidence.md" } }));
    expect(screen.queryByText("No task source files discovered")).not.toBeInTheDocument();
  });

  it("empty_response_renders_truthful_empty_ui", async () => {
    invoke.mockImplementation((command: string) => command === "hiveai_projects_list" ? Promise.resolve(records) : command === "hiveai_task_sources_list" ? Promise.resolve([]) : command === "hiveai_task_source_custom_paths_list" ? Promise.resolve([]) : Promise.resolve(undefined));
    renderTasks();
    expect(await screen.findByText("No task source files discovered")).toBeInTheDocument();
    expect(screen.queryByText(/tasks completed/i)).not.toBeInTheDocument();
  });

  it("project_change_requests_new_selected_project_inventory", async () => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<RegistryProvider><MemoryRouter initialEntries={["/tasks"]}><SelectionHarness /></MemoryRouter></RegistryProvider>);
    expect(await screen.findByText("TASKS.md")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Select B" }));
    await waitFor(() => expect(screen.getByText("Project B")).toBeInTheDocument());
    expect(invoke).toHaveBeenCalledWith("hiveai_task_sources_list", { projectId: "project-b" });
  });

  it("browser_preview_does_not_invoke_native_filesystem_commands", async () => {
    render(<RegistryProvider><MemoryRouter initialEntries={["/tasks"]}><Tasks /></MemoryRouter></RegistryProvider>);
    expect(await screen.findByText(/Browser preview uses no filesystem discovery/)).toBeInTheDocument();
    expect(invoke.mock.calls.some(([command]) => String(command).startsWith("hiveai_task_source"))).toBe(false);
  });

  it("shows_custom_path_status", async () => {
    invoke.mockImplementation((command: string) => command === "hiveai_projects_list" ? Promise.resolve(records) : command === "hiveai_task_sources_list" ? Promise.resolve([]) : command === "hiveai_task_source_custom_paths_list" ? Promise.resolve([{ id: "custom", displayPath: "evidence.md", normalizedPath: "evidence.md", status: "MISSING" }]) : Promise.resolve([]));
    renderTasks();
    expect(await screen.findByText(/MISSING/)).toBeInTheDocument();
  });

  it("does_not_render_task_workflow_columns", async () => {
    renderTasks();
    await screen.findByText("TASKS.md");
    expect(screen.queryByText(/progress|workflow|owner/i)).not.toBeInTheDocument();
  });

  it("rescan_refreshes_inventory_for_selected_project", async () => {
    renderTasks();
    await screen.findByText("TASKS.md");
    fireEvent.click(screen.getByRole("button", { name: /Rescan sources/i }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_task_sources_discover", { projectId: "project-a" }));
  });

  it("keeps_custom_path_input_scoped_to_workspace", async () => {
    renderTasks();
    expect(await screen.findByRole("textbox", { name: "Custom source path" })).toBeInTheDocument();
    expect(screen.queryByRole("textbox", { name: /task title/i })).not.toBeInTheDocument();
  });

  it("renders_source_kind_and_origin_columns", async () => {
    renderTasks();
    await screen.findByText("TASKS.md");
    expect(screen.getByText("TASKS")).toBeInTheDocument();
    expect(screen.getByText("STANDARD")).toBeInTheDocument();
  });

  it("renders_selected_project_identity_from_registry", async () => {
    renderTasks();
    expect(await screen.findByRole("heading", { name: "Project A" })).toBeInTheDocument();
    expect(screen.getByText("Selected project")).toBeInTheDocument();
  });

  it("delayed_project_a_response_cannot_replace_project_b_inventory", async () => {
    let resolveA!: (value: unknown) => void;
    let resolveB!: (value: unknown) => void;
    invoke.mockImplementation((command: string, args?: { projectId?: string }) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_task_source_custom_paths_list") return Promise.resolve([]);
      if (command === "hiveai_task_sources_list" && args?.projectId === "project-a") return new Promise((resolve) => { resolveA = resolve; });
      if (command === "hiveai_task_sources_list" && args?.projectId === "project-b") return new Promise((resolve) => { resolveB = resolve; });
      return Promise.resolve([]);
    });
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<RegistryProvider><MemoryRouter initialEntries={["/tasks"]}><SelectionHarness /></MemoryRouter></RegistryProvider>);
    await screen.findByRole("button", { name: "Select A" });
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_task_sources_list", { projectId: "project-a" }));
    fireEvent.click(screen.getByRole("button", { name: "Select B" }));
    resolveB([source("project-b", "B-TASKS.md")]);
    expect(await screen.findByText("B-TASKS.md")).toBeInTheDocument();
    resolveA([source("project-a", "A-TASKS.md")]);
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(screen.getByText("B-TASKS.md")).toBeInTheDocument();
    expect(screen.queryByText("A-TASKS.md")).not.toBeInTheDocument();
  });

  it("stale_custom_add_completion_cannot_refresh_project_a_after_project_b_selection", async () => {
    let resolveAdd!: (value: unknown) => void;
    invoke.mockImplementation((command: string, args?: { projectId?: string }) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_task_source_custom_paths_list") return Promise.resolve([]);
      if (command === "hiveai_task_sources_list") return Promise.resolve([source(args?.projectId ?? "project-a", `${args?.projectId ?? "project-a"}-TASKS.md`)]);
      if (command === "hiveai_task_source_custom_path_add") return new Promise((resolve) => { resolveAdd = resolve; });
      return Promise.resolve([]);
    });
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<RegistryProvider><MemoryRouter initialEntries={["/tasks"]}><SelectionHarness /></MemoryRouter></RegistryProvider>);
    await screen.findByText("project-a-TASKS.md");
    fireEvent.change(screen.getByRole("textbox", { name: "Custom source path" }), { target: { value: "custom.md" } });
    fireEvent.click(screen.getByRole("button", { name: /Add path/i }));
    fireEvent.click(screen.getByRole("button", { name: "Select B" }));
    expect(await screen.findByText("project-b-TASKS.md")).toBeInTheDocument();
    resolveAdd([]);
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(screen.getByText("project-b-TASKS.md")).toBeInTheDocument();
  });

  it("stale_custom_remove_completion_cannot_refresh_project_a_after_project_b_selection", async () => {
    let resolveRemove!: (value: unknown) => void;
    const custom = { id: "custom-a", projectId: "project-a", displayPath: "custom.md", normalizedPath: "custom.md", status: "CONFIGURED", order: 0 };
    invoke.mockImplementation((command: string, args?: { projectId?: string }) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_task_source_custom_paths_list") return Promise.resolve(args?.projectId === "project-a" ? [custom] : []);
      if (command === "hiveai_task_sources_list") return Promise.resolve([source(args?.projectId ?? "project-a", `${args?.projectId ?? "project-a"}-TASKS.md`)]);
      if (command === "hiveai_task_source_custom_path_remove") return new Promise((resolve) => { resolveRemove = resolve; });
      return Promise.resolve([]);
    });
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    render(<RegistryProvider><MemoryRouter initialEntries={["/tasks"]}><SelectionHarness /></MemoryRouter></RegistryProvider>);
    await screen.findByRole("button", { name: "Remove custom.md" });
    fireEvent.click(screen.getByRole("button", { name: "Remove custom.md" }));
    fireEvent.click(screen.getByRole("button", { name: "Select B" }));
    expect(await screen.findByText("project-b-TASKS.md")).toBeInTheDocument();
    resolveRemove([]);
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(screen.getByText("project-b-TASKS.md")).toBeInTheDocument();
  });

  it("rejected_native_list_renders_truthful_error_ui", async () => {
    invoke.mockImplementation((command: string) => command === "hiveai_projects_list" ? Promise.resolve(records) : command === "hiveai_task_sources_list" ? Promise.reject(new Error("registered project root is unavailable")) : Promise.resolve([]));
    renderTasks();
    expect(await screen.findByRole("alert")).toHaveTextContent("registered project root is unavailable");
  });

  it("rescan_replaces_visible_source_row_with_discover_response", async () => {
    renderTasks();
    await screen.findByText("TASKS.md");
    fireEvent.click(screen.getByRole("button", { name: /Rescan sources/i }));
    expect(await screen.findByText("handoffs/current.md")).toBeInTheDocument();
    expect(screen.queryByText("TASKS.md")).not.toBeInTheDocument();
  });

  it("custom_remove_executes_and_refreshes_visible_inventory", async () => {
    const custom = { id: "custom-a", projectId: "project-a", displayPath: "custom.md", normalizedPath: "custom.md", status: "CONFIGURED", order: 0 };
    let configured = true;
    invoke.mockImplementation((command: string, args?: { projectId?: string }) => command === "hiveai_projects_list" ? Promise.resolve(records) : command === "hiveai_task_sources_list" ? Promise.resolve([source(args?.projectId ?? "project-a")]) : command === "hiveai_task_source_custom_paths_list" ? Promise.resolve(configured ? [custom] : []) : command === "hiveai_task_source_custom_path_remove" ? (configured = false, Promise.resolve([])) : Promise.resolve([]));
    renderTasks();
    await screen.findByRole("button", { name: "Remove custom.md" });
    fireEvent.click(screen.getByRole("button", { name: "Remove custom.md" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_task_source_custom_path_remove", { projectId: "project-a", pathOrId: "custom-a" }));
    expect(await screen.findByText("No custom source paths configured.")).toBeInTheDocument();
  });

  it("custom_update_reorder_executes_and_refreshes_visible_inventory", async () => {
    const custom = { id: "custom-b", projectId: "project-a", displayPath: "custom.md", normalizedPath: "custom.md", status: "CONFIGURED", order: 1 };
    invoke.mockImplementation((command: string) => command === "hiveai_projects_list" ? Promise.resolve(records) : command === "hiveai_task_sources_list" ? Promise.resolve([source("project-a")]) : command === "hiveai_task_source_custom_paths_list" ? Promise.resolve([custom]) : Promise.resolve([]));
    renderTasks();
    await screen.findByRole("button", { name: "Move custom.md earlier" });
    fireEvent.click(screen.getByRole("button", { name: "Move custom.md earlier" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_task_source_custom_path_update", { request: { projectId: "project-a", pathOrId: "custom-b", order: 0 } }));
  });
});
