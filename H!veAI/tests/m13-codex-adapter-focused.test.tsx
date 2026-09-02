import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";

const invoke = vi.hoisted(() => vi.fn());
const records = [{ id: "alpha", name: "Project Alpha", originalPath: "C:\\Projects\\alpha", normalizedPath: "c:\\projects\\alpha", status: "ACTIVE", priority: 0, preferredBuilder: null, preferredAuditor: null, taskSourcePolicy: "DISCOVER_STANDARD_FILES", registeredAt: "2026-08-27T10:00:00Z", lastValidatedAt: "2026-08-27T10:00:00Z", repository: null }];
const readiness = { provider: "CODEX", available: true, version: "codex-cli 0.130.0-alpha.5", readinessState: "VERSION_VERIFIED_AUTH_UNKNOWN", diagnosticCode: "AUTH_READINESS_UNVERIFIED", diagnosticMessage: "authentication is unknown", checkedAt: "2026-08-27T10:00:00Z" };

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

beforeEach(() => {
  Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
  window.sessionStorage.clear();
  window.history.pushState({}, "", "/agents");
  invoke.mockReset();
  invoke.mockImplementation((command: string) => {
    if (command === "hiveai_projects_list") return Promise.resolve(records);
    if (command === "hiveai_codex_readiness") return Promise.resolve(readiness);
    if (command === "hiveai_codex_sessions_list") return Promise.resolve([]);
    return Promise.resolve({});
  });
});

describe("M13 Codex adapter", () => {
  it("renders truthful readiness and selected registered project", async () => {
    render(<App />);
    expect(await screen.findByText("codex-cli 0.130.0-alpha.5")).toBeInTheDocument();
    expect(screen.getAllByText("Project Alpha").length).toBeGreaterThan(0);
    expect(screen.getByText("Unknown until operation")).toBeInTheDocument();
  });

  it("passes only the selected project, bounded task id, and prompt to native start", async () => {
    render(<App />);
    await screen.findByText("codex-cli 0.130.0-alpha.5");
    fireEvent.change(screen.getByLabelText("Codex task ID"), { target: { value: "alpha-task" } });
    fireEvent.change(screen.getByLabelText("Codex prompt"), { target: { value: "inspect x & y | z" } });
    fireEvent.click(screen.getByRole("button", { name: "Start Codex operation" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_codex_start", { request: { projectId: "alpha", taskId: "alpha-task", prompt: "inspect x & y | z" } }));
  });

  it("does not offer an operation in browser preview and reports unsupported resume", async () => {
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
    window.history.pushState({}, "", "/agents");
    render(<App />);
    expect(await screen.findByText("Native H!veAI is required for Codex process operations.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Start Codex operation" })).toBeDisabled();
  });

  it("renders bounded failed-session evidence without protected markers", async () => {
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_codex_readiness") return Promise.resolve(readiness);
      if (command === "hiveai_codex_sessions_list") return Promise.resolve([{
        id: "failed-session",
        provider: "CODEX",
        projectId: "alpha",
        taskId: null,
        operationKind: "CODEX_EXEC",
        state: "FAILED",
        cwd: "C:\\Projects\\alpha",
        startedAt: "2026-08-27T10:01:00Z",
        endedAt: "2026-08-27T10:01:01Z",
        exitCode: 1,
        stdout: "",
        stderr: "[REDACTED SENSITIVE OUTPUT]\\nmodel requires a newer Codex version",
        stdoutTruncated: false,
        stderrTruncated: false,
        diagnosticCode: "CODEX_PROCESS_FAILED",
        diagnosticMessage: "Codex exited with code 1",
      }]);
      return Promise.resolve({});
    });
    render(<App />);
    expect(await screen.findByText("FAILED")).toBeInTheDocument();
    expect(screen.getByText("CODEX_PROCESS_FAILED")).toBeInTheDocument();
    expect(screen.getByText("Codex exited with code 1")).toBeInTheDocument();
    expect(screen.getByText("1")).toBeInTheDocument();
    expect(screen.getByText(/model requires a newer Codex version/)).toBeInTheDocument();
    expect(screen.getByText(/\[REDACTED SENSITIVE OUTPUT\]/)).toBeInTheDocument();
    expect(screen.queryByText(/password=/i)).not.toBeInTheDocument();
  });

  it("keeps completed-session output evidence visible", async () => {
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_codex_readiness") return Promise.resolve(readiness);
      if (command === "hiveai_codex_sessions_list") return Promise.resolve([{
        id: "completed-session",
        provider: "CODEX",
        projectId: "alpha",
        taskId: "alpha-task",
        operationKind: "CODEX_EXEC",
        state: "COMPLETED",
        cwd: "C:\\Projects\\alpha",
        startedAt: "2026-08-27T10:01:00Z",
        endedAt: "2026-08-27T10:01:02Z",
        exitCode: 0,
        stdout: "status: clean",
        stderr: "",
        stdoutTruncated: false,
        stderrTruncated: false,
        diagnosticCode: null,
        diagnosticMessage: null,
      }]);
      return Promise.resolve({});
    });
    render(<App />);
    expect(await screen.findByText("COMPLETED")).toBeInTheDocument();
    expect(screen.getByText("status: clean")).toBeInTheDocument();
  });

  it("renders long persisted JSON output as a wrapped vertical reader", async () => {
    const longPath = `C:\\Projects\\alpha\\${"nested\\".repeat(40)}result.json`;
    const output = JSON.stringify({ type: "command.completed", command: longPath, status: "clean" });
    const longUnrecognizedLine = `unrecognized-path=${"x".repeat(400)}`;
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_projects_list") return Promise.resolve(records);
      if (command === "hiveai_codex_readiness") return Promise.resolve(readiness);
      if (command === "hiveai_codex_sessions_list") return Promise.resolve([{
        id: "long-session",
        provider: "CODEX",
        projectId: "alpha",
        taskId: "alpha-task",
        operationKind: "CODEX_EXEC",
        state: "COMPLETED",
        cwd: "C:\\Projects\\alpha",
        startedAt: "2026-08-27T10:01:00Z",
        endedAt: "2026-08-27T10:01:02Z",
        exitCode: 0,
        stdout: `${output}\n${longUnrecognizedLine}`,
        stderr: "",
        stdoutTruncated: false,
        stderrTruncated: false,
        diagnosticCode: null,
        diagnosticMessage: null,
      }]);
      return Promise.resolve({});
    });
    const { container } = render(<App />);
    const reader = await screen.findByTestId("agent-stdout-reader");
    expect(reader).toHaveClass("agent-output-reader");
    expect(reader).toHaveTextContent("command.completed");
    expect(reader).toHaveTextContent("result.json");
    expect(reader).toHaveTextContent(longUnrecognizedLine);
    expect(reader.querySelector("pre")).toBeNull();
    expect(reader.querySelector(".agent-output-event-content")).toBeTruthy();
    expect(container.querySelector(".agent-sessions-panel")?.textContent).toContain("COMPLETED");
  });
});
