import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";

const invoke = vi.hoisted(() => vi.fn());
const records = [{ id: "scrubbots", name: "ScrubBots", originalPath: "C:\\Projects\\ScrubBots", normalizedPath: "C:\\Projects\\ScrubBots", status: "ACTIVE", priority: 1, preferredBuilder: null, preferredAuditor: null, taskSourcePolicy: "DISCOVER_STANDARD_FILES", preferredAgentProvider: "CLAUDE", registeredAt: "2026-09-02T10:00:00Z", lastValidatedAt: "2026-09-02T10:00:00Z", repository: null }];
const readiness = [
  { provider: "CODEX", available: true, version: "codex-cli 0.149.1", readinessState: "VERSION_VERIFIED_AUTH_UNKNOWN", diagnosticCode: "AUTH_UNKNOWN", diagnosticMessage: "Codex authentication is determined by operation", capabilities: ["START", "LIST", "STOP"], supportsPty: false, supportsResume: false, checkedAt: "2026-09-02T10:00:00Z" },
  { provider: "CLAUDE", available: true, version: "2.1.248 (Claude Code)", readinessState: "VERSION_VERIFIED_AUTH_UNKNOWN", diagnosticCode: "AUTH_UNKNOWN", diagnosticMessage: "Claude authentication is determined by operation", capabilities: ["START", "LIST", "STOP", "BOUNDED_STREAM_JSON"], supportsPty: false, supportsResume: false, checkedAt: "2026-09-02T10:00:00Z" },
];
const session = { id: "claude-session", provider: "CLAUDE", projectId: "scrubbots", taskId: null, operationKind: "FREEFORM_PROJECT_OPERATION", state: "COMPLETED", cwd: "C:\\Projects\\ScrubBots", startedAt: "2026-09-02T10:01:00Z", endedAt: "2026-09-02T10:01:04Z", exitCode: 0, stdout: "read-only repository summary", stderr: "", stdoutTruncated: false, stderrTruncated: false, diagnosticCode: null, diagnosticMessage: null, promptReference: "sha256:fixture", providerVersion: "2.1.248 (Claude Code)", elapsedMs: 4000, supportsResume: false, supportsPty: false, events: [{ sequence: 1, id: "event-1", eventType: "SESSION_STARTED", payload: { providerVersion: "2.1.248 (Claude Code)" }, occurredAt: "2026-09-02T10:01:00Z" }] };

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

beforeEach(() => {
  Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
  window.sessionStorage.clear();
  window.history.pushState({}, "", "/agents");
  invoke.mockReset();
  invoke.mockImplementation((command: string) => {
    if (command === "hiveai_projects_list") return Promise.resolve(records);
    if (command === "hiveai_agent_readiness") return Promise.resolve(readiness);
    if (command === "hiveai_agent_sessions_list") return Promise.resolve([]);
    if (command === "hiveai_git_snapshot") return Promise.resolve({ stagedFiles: [], unstagedFiles: [], untrackedFiles: [], conflictedFiles: [] });
    if (command === "hiveai_git_diff") return Promise.resolve({ text: "", truncated: false });
    if (command === "hiveai_project_update_settings") return Promise.resolve({ ...records[0], preferredAgentProvider: "CLAUDE" });
    return Promise.resolve({});
  });
});

describe("M14 Agent Session Center", () => {
  it("shows both provider readiness cards and the persisted ScrubBots preference", async () => {
    render(<App />);
    expect(await screen.findByText("2.1.248 (Claude Code)")).toBeInTheDocument();
    expect(screen.getByText("codex-cli 0.149.1")).toBeInTheDocument();
    await waitFor(() => expect(screen.getByLabelText("Agent provider")).toHaveValue("CLAUDE"));
  });

  it("persists provider choice and dispatches only provider-neutral start data", async () => {
    render(<App />);
    const provider = await screen.findByLabelText("Agent provider");
    await waitFor(() => expect(provider).toHaveValue("CLAUDE"));
    fireEvent.change(provider, { target: { value: "CODEX" } });
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_project_update_settings", { request: { projectId: "scrubbots", priority: 1, preferredAgentProvider: "CODEX" } }));
    fireEvent.change(screen.getByLabelText("Agent prompt"), { target: { value: "inspect the repository without modifying files" } });
    fireEvent.click(screen.getByRole("button", { name: "Start CODEX session" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_agent_start", { request: { projectId: "scrubbots", provider: "CODEX", taskId: null, prompt: "inspect the repository without modifying files" } }));
    const startCall = invoke.mock.calls.find(([command]) => command === "hiveai_agent_start");
    expect(startCall?.[1]?.request).not.toHaveProperty("executable");
    expect(startCall?.[1]?.request).not.toHaveProperty("args");
    expect(startCall?.[1]?.request).not.toHaveProperty("shell");
    expect(startCall?.[1]?.request).not.toHaveProperty("pid");
  });

  it("renders the shared session timeline, vertical reader, and Git authority diff", async () => {
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_agent_readiness") return Promise.resolve(readiness);
      if (command === "hiveai_agent_sessions_list") return Promise.resolve([session]);
      if (command === "hiveai_git_snapshot") return Promise.resolve({ stagedFiles: [], unstagedFiles: [], untrackedFiles: ["README.md"], conflictedFiles: [] });
      if (command === "hiveai_git_diff") return Promise.resolve({ text: "diff --git a/README.md b/README.md", truncated: false });
      return Promise.resolve({});
    });
    render(<App />);
    const viewButton = await screen.findByRole("button", { name: /View CLAUDE FREEFORM_PROJECT_OPERATION COMPLETED/i });
    expect(screen.queryByTestId("agent-session-detail")).not.toBeInTheDocument();
    expect(screen.queryByText("SESSION_STARTED")).not.toBeInTheDocument();
    fireEvent.click(viewButton);
    expect(screen.getByTestId("agent-stdout-reader")).toHaveTextContent("read-only repository summary");
    expect(screen.queryByText("SESSION_STARTED")).not.toBeInTheDocument();
    fireEvent.click(screen.getByText("Timeline"));
    expect(await screen.findByText("SESSION STARTED")).toBeInTheDocument();
    fireEvent.click(screen.getByText("Git evidence"));
    expect(await screen.findByText("UNTRACKED: README.md")).toBeInTheDocument();
    fireEvent.click(screen.getByText("View bounded Git diff"));
    expect(await screen.findByText("diff --git a/README.md b/README.md")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Resume" })).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Close session details" }));
    expect(screen.queryByTestId("agent-session-detail")).not.toBeInTheDocument();
  });

  it("keeps persisted sessions compact and permits only one explicit detail", async () => {
    const secondSession = { ...session, id: "codex-session", provider: "CODEX", operationKind: "CODEX_EXEC" };
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_agent_readiness") return Promise.resolve(readiness);
      if (command === "hiveai_agent_sessions_list") return Promise.resolve([session, secondSession]);
      return Promise.resolve({ stagedFiles: [], unstagedFiles: [], untrackedFiles: [], conflictedFiles: [] });
    });
    render(<App />);
    expect(await screen.findByRole("button", { name: /View CLAUDE FREEFORM_PROJECT_OPERATION COMPLETED/i })).toBeInTheDocument();
    expect(screen.queryByTestId("agent-session-detail")).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /View CLAUDE FREEFORM_PROJECT_OPERATION COMPLETED/i }));
    expect(screen.getByTestId("agent-session-detail")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /View CODEX CODEX_EXEC COMPLETED/i }));
    expect(screen.getAllByTestId("agent-session-detail")).toHaveLength(1);
    expect(screen.getByText("CODEX session")).toBeInTheDocument();
  });
});
