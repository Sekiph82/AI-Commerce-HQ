import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";

const invoke = vi.hoisted(() => vi.fn());
const project = { id: "handoff-project", name: "Handoff Fixture", originalPath: "C:\\Projects\\Handoff Fixture", normalizedPath: "C:\\Projects\\Handoff Fixture", status: "ACTIVE", priority: 1, preferredBuilder: null, preferredAuditor: null, taskSourcePolicy: "DISCOVER_STANDARD_FILES", preferredAgentProvider: "CODEX", registeredAt: "2026-09-05T10:00:00Z", lastValidatedAt: "2026-09-05T10:00:00Z", repository: null };
const otherProject = { ...project, id: "other-project", name: "Other Fixture", originalPath: "C:\\Projects\\Other Fixture", normalizedPath: "C:\\Projects\\Other Fixture" };
const readiness = [
  { provider: "CODEX", available: true, version: "codex-cli 0.149.1", readinessState: "VERSION_VERIFIED_AUTH_UNKNOWN", diagnosticCode: null, diagnosticMessage: null, capabilities: [], supportsPty: false, supportsResume: false, checkedAt: "2026-09-05T10:00:00Z" },
  { provider: "CLAUDE", available: true, version: "2.1.248 (Claude Code)", readinessState: "VERSION_VERIFIED_AUTH_UNKNOWN", diagnosticCode: null, diagnosticMessage: null, capabilities: [], supportsPty: false, supportsResume: false, checkedAt: "2026-09-05T10:00:00Z" },
];
const baseSession = { id: "session-1", provider: "CODEX", projectId: project.id, taskId: null, operationKind: "PROMPT_ENGINE_DISPATCH", state: "COMPLETED", cwd: project.originalPath, startedAt: "2026-09-05T10:01:00Z", endedAt: "2026-09-05T10:01:02Z", exitCode: 0, stdout: "", stderr: "", stdoutTruncated: false, stderrTruncated: false, finalResponse: "The exact dispatched result.", finalResponseTruncated: false, finalResponseState: "AVAILABLE", finalResponseRole: "assistant", diagnosticCode: null, diagnosticMessage: null, promptReference: "sha256:fixture", promptBody: "Read-only prompt", promptId: "prompt-1", promptVersionId: "version-1", promptVersion: 1, promptVersionSha256: "prompt-hash", providerVersion: "codex-cli 0.149.1", elapsedMs: 2000, supportsResume: false, supportsPty: false, events: [] };
const context = { projectId: project.id, taskId: null, items: [], includedBytes: 0, omittedCount: 0, sourceCount: 0, manifestSha256: "context-hash" };
const draft = { id: "version-1", promptId: "prompt-1", version: 1, kind: "IMPLEMENTATION", title: "Handoff test", summary: "Read-only handoff test", content: "Read-only prompt", createdBy: "M15_PROMPT_ENGINE", createdAt: "2026-09-05T10:00:00Z", origin: "M15_GENERATOR", contextManifest: context, provenance: { projectId: project.id }, approvalState: "DRAFT", approvedAt: null, approvedBodySha256: null, usedAt: null, selectedProvider: null, dispatchedSessionId: null, supersededAt: null, dispatchState: "AVAILABLE", dispatchReservationId: null, dispatchReservedAt: null, dispatchProvenance: {}, dispatchError: null, bodySha256: "body-hash", isCurrent: true };

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

let sessions: typeof baseSession[] = [baseSession];

beforeEach(() => {
  Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
  window.sessionStorage.clear();
  window.history.pushState({}, "", "/prompts");
  sessions = [baseSession];
  invoke.mockReset();
  invoke.mockImplementation((command: string, args?: { projectId?: string; request?: { provider?: string } }) => {
    if (command === "hiveai_projects_list") return Promise.resolve([project, otherProject]);
    if (command === "hiveai_workflow_project_list") return Promise.resolve({ projectId: project.id, tasks: [] });
    if (command === "hiveai_prompt_context_collect") return Promise.resolve(context);
    if (command === "hiveai_prompt_generate") return Promise.resolve(draft);
    if (command === "hiveai_prompt_versions") return Promise.resolve([draft]);
    if (command === "hiveai_prompt_approve") return Promise.resolve({ ...draft, approvalState: "APPROVED", approvedBodySha256: "body-hash" });
    if (command === "hiveai_prompt_dispatch") { const selectedProvider = args?.request?.provider === "CLAUDE" ? "CLAUDE" : "CODEX"; return Promise.resolve({ prompt: { ...draft, approvalState: "DISPATCHED", dispatchState: "DISPATCHED" }, session: { id: baseSession.id, provider: selectedProvider }, promptId: draft.promptId, promptVersionId: draft.id, promptVersion: 1, promptVersionSha256: "body-hash" }); }
    if (command === "hiveai_agent_readiness") return Promise.resolve(readiness);
    if (command === "hiveai_agent_sessions_list") return Promise.resolve(args?.projectId === project.id ? sessions : []);
    if (command === "hiveai_git_snapshot") return Promise.resolve({ stagedFiles: [], unstagedFiles: [], untrackedFiles: [], conflictedFiles: [] });
    if (command === "hiveai_git_diff") return Promise.resolve({ text: "", truncated: false });
    return Promise.resolve({});
  });
});

async function dispatchFixture(selectedProvider: "CODEX" | "CLAUDE" = "CODEX") {
  render(<App />);
  await waitFor(() => expect(screen.getByLabelText("Prompt project")).toHaveValue(project.id));
  fireEvent.change(screen.getByLabelText("Prompt title"), { target: { value: draft.title } });
  fireEvent.change(screen.getByLabelText("Prompt summary"), { target: { value: draft.summary } });
  fireEvent.click(screen.getByRole("button", { name: /Generate draft/ }));
  await waitFor(() => expect(screen.getByLabelText("Prompt body editor")).toBeInTheDocument());
  fireEvent.click(screen.getByRole("button", { name: /Approve exact version/ }));
  await waitFor(() => expect(screen.getByRole("button", { name: /Dispatch to/ })).toBeEnabled());
  if (selectedProvider === "CLAUDE") fireEvent.click(screen.getByRole("button", { name: "Claude" }));
  fireEvent.click(screen.getByRole("button", { name: new RegExp(`Dispatch to ${selectedProvider === "CODEX" ? "Codex" : "Claude"}`) }));
  await waitFor(() => expect(screen.getByRole("button", { name: /View result in Agents/ })).toBeInTheDocument());
}

describe("M15C Prompt Engine post-dispatch handoff", () => {
  it("shows the exact target and navigation does not redispatch", async () => {
    await dispatchFixture();
    const dispatchCount = invoke.mock.calls.filter(([command]) => command === "hiveai_prompt_dispatch").length;
    fireEvent.click(screen.getByRole("button", { name: /View result in Agents/ }));
    await waitFor(() => expect(window.location.pathname).toBe("/agents"));
    expect(new URLSearchParams(window.location.search).get("projectId")).toBe(project.id);
    expect(new URLSearchParams(window.location.search).get("sessionId")).toBe(baseSession.id);
    expect(invoke.mock.calls.filter(([command]) => command === "hiveai_prompt_dispatch")).toHaveLength(dispatchCount);
    expect(await screen.findByTestId("agent-session-detail")).toHaveTextContent("The exact dispatched result.");
  });

  it("clears a stale handoff when a new draft replaces the dispatched version", async () => {
    await dispatchFixture();
    expect(screen.getByRole("button", { name: /View result in Agents/ })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /Generate draft/ }));
    await waitFor(() => expect(screen.queryByRole("button", { name: /View result in Agents/ })).not.toBeInTheDocument());
  });

  it("clears a stale handoff when the project changes", async () => {
    await dispatchFixture();
    fireEvent.change(screen.getByLabelText("Prompt project"), { target: { value: otherProject.id } });
    expect(screen.queryByRole("button", { name: /View result in Agents/ })).not.toBeInTheDocument();
  });
});

describe("M15D Prompt Engine result placement", () => {
  it("keeps the successful result and handoff directly below the dispatch controls", async () => {
    await dispatchFixture();
    const dispatchPanel = document.querySelector(".prompt-dispatch-panel");
    expect(dispatchPanel).not.toBeNull();
    expect(dispatchPanel).toContainElement(screen.getByText(/Dispatched CODEX session/));
    expect(dispatchPanel).toContainElement(screen.getByRole("button", { name: /View result in Agents/ }));
    expect(document.querySelector(".safe-notice.prompt-notice")).not.toBeInTheDocument();
    expect(dispatchPanel?.querySelector(".prompt-dispatch-row")?.nextElementSibling).toHaveClass("prompt-dispatch-result");
  });

  it("uses the same local result surface for Claude", async () => {
    await dispatchFixture("CLAUDE");
    const dispatchPanel = document.querySelector(".prompt-dispatch-panel");
    expect(dispatchPanel).toContainElement(screen.getByText(/Dispatched CLAUDE session/));
    expect(dispatchPanel).toContainElement(screen.getByRole("button", { name: /View result in Agents/ }));
    expect(document.querySelector(".safe-notice.prompt-notice")).not.toBeInTheDocument();
  });
});

describe("M15C Agents route targeting", () => {
  it("selects the exact target session and keeps later manual selection", async () => {
    const second = { ...baseSession, id: "session-2", finalResponse: "The manually selected result." };
    sessions = [baseSession, second];
    window.history.pushState({}, "", `/agents?projectId=${project.id}&sessionId=${baseSession.id}`);
    render(<App />);
    await waitFor(() => expect(screen.getByTestId("agent-session-detail")).toHaveTextContent("The exact dispatched result."));
    fireEvent.click(screen.getAllByRole("button", { name: /View CODEX PROMPT_ENGINE_DISPATCH COMPLETED/ })[1]);
    await waitFor(() => expect(screen.getByTestId("agent-session-detail")).toHaveTextContent("The manually selected result."));
  });

  it("rejects a target session from another project", async () => {
    sessions = [{ ...baseSession, projectId: otherProject.id }];
    window.history.pushState({}, "", `/agents?projectId=${project.id}&sessionId=${baseSession.id}`);
    render(<App />);
    expect(await screen.findByText("The dispatched session is not persisted under that registered project.")).toBeInTheDocument();
    expect(screen.queryByTestId("agent-session-detail")).not.toBeInTheDocument();
  });

  it("fails safely for an invalid session target and preserves Agents", async () => {
    window.history.pushState({}, "", `/agents?projectId=${project.id}&sessionId=missing-session`);
    render(<App />);
    expect(await screen.findByText("The dispatched session is not persisted under that registered project.")).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Agent Session Center" })).toBeInTheDocument();
  });

  it("keeps a running targeted session selected while polling updates it", async () => {
    let calls = 0;
    invoke.mockImplementation((command: string, args?: { projectId?: string; request?: { provider?: string } }) => {
      if (command === "hiveai_projects_list") return Promise.resolve([project]);
      if (command === "hiveai_agent_readiness") return Promise.resolve(readiness);
      if (command === "hiveai_agent_sessions_list") { calls += 1; return Promise.resolve([{ ...baseSession, state: calls > 1 ? "COMPLETED" : "RUNNING", endedAt: calls > 1 ? baseSession.endedAt : null }]); }
      if (command === "hiveai_git_snapshot") return Promise.resolve({ stagedFiles: [], unstagedFiles: [], untrackedFiles: [], conflictedFiles: [] });
      if (command === "hiveai_git_diff") return Promise.resolve({ text: "", truncated: false });
      return Promise.resolve({});
    });
    window.history.pushState({}, "", `/agents?projectId=${project.id}&sessionId=${baseSession.id}`);
    render(<App />);
    await waitFor(() => expect(screen.getByTestId("agent-session-detail")).toHaveTextContent("CODEX conversation"));
    expect(screen.getByTestId("agent-session-detail")).toHaveTextContent("The exact dispatched result.");
  });
});
