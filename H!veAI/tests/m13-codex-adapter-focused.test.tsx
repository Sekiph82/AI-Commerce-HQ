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
});
