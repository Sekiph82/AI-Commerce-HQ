import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";

const invoke = vi.hoisted(() => vi.fn());
const project = { id: "prompt-project", name: "Prompt Fixture", originalPath: "C:\\Projects\\Prompt Fixture", normalizedPath: "C:\\Projects\\Prompt Fixture", status: "ACTIVE", priority: 1, preferredBuilder: null, preferredAuditor: null, taskSourcePolicy: "DISCOVER_STANDARD_FILES", preferredAgentProvider: "CODEX", registeredAt: "2026-09-03T10:00:00Z", lastValidatedAt: "2026-09-03T10:00:00Z", repository: null };
const context = { projectId: project.id, taskId: "task-1", items: [{ class: "TASK", reference: "task:task-1", disposition: "INCLUDED", bytes: 42, reason: null, value: "task" }], includedBytes: 42, omittedCount: 0, sourceCount: 1, manifestSha256: "context-hash" };
const version = { id: "version-1", promptId: "prompt-1", version: 1, kind: "IMPLEMENTATION", title: "Implement fixture", summary: "Read-only fixture goal", content: "Generated prompt body", createdBy: "M15_PROMPT_ENGINE", createdAt: "2026-09-03T10:00:00Z", origin: "M15_GENERATOR", contextManifest: context, provenance: { projectId: project.id }, approvalState: "DRAFT", approvedAt: null, approvedBodySha256: null, usedAt: null, selectedProvider: null, dispatchedSessionId: null, supersededAt: null, dispatchState: "AVAILABLE", dispatchReservationId: null, dispatchReservedAt: null, dispatchProvenance: {}, dispatchError: null, bodySha256: "body-hash", isCurrent: true };

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

beforeEach(() => {
  Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
  window.history.pushState({}, "", "/prompts");
  invoke.mockReset();
  invoke.mockImplementation((command: string) => {
    if (command === "hiveai_projects_list") return Promise.resolve([project]);
    if (command === "hiveai_workflow_project_list") return Promise.resolve({ projectId: project.id, tasks: [{ taskId: "task-1", projectId: project.id, title: "Fixture task", currentState: "BACKLOG", workflowManaged: true, sourceActive: true, sourceRetired: false, allowedNextStates: [], allowedActors: [], suspensionResumeState: null, latestEvent: null, attentionRequired: false, requiredActor: "HUMAN", milestone: "M15" }] });
    if (command === "hiveai_prompt_context_collect") return Promise.resolve(context);
    if (command === "hiveai_prompt_generate") return Promise.resolve(version);
    if (command === "hiveai_prompt_versions") return Promise.resolve([version]);
    if (command === "hiveai_prompt_edit") return Promise.resolve({ ...version, content: "Edited prompt body" });
    if (command === "hiveai_prompt_approve") return Promise.resolve({ ...version, content: "Edited prompt body", approvalState: "APPROVED", approvedBodySha256: "edited-hash" });
    if (command === "hiveai_prompt_dispatch") return Promise.resolve({ prompt: { ...version, content: "Edited prompt body", approvalState: "DISPATCHED", dispatchState: "DISPATCHED" }, session: { id: "session-1", provider: "CODEX" }, promptId: "prompt-1", promptVersionId: "version-1", promptVersion: 1, promptVersionSha256: "edited-hash" });
    return Promise.resolve({});
  });
});

describe("M15 Prompt Engine", () => {
  it("keeps context, generation, edit, approval, and dispatch as explicit steps", async () => {
    render(<App />);
    await waitFor(() => expect(screen.getByLabelText("Prompt project")).toHaveValue(project.id));
    fireEvent.change(screen.getByLabelText("Prompt title"), { target: { value: "Implement fixture" } });
    fireEvent.change(screen.getByLabelText("Prompt summary"), { target: { value: "Read-only fixture goal" } });
    fireEvent.click(screen.getByRole("button", { name: /Refresh context/ }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_prompt_context_collect", { request: { projectId: project.id, taskId: null } }));
    fireEvent.click(screen.getByRole("button", { name: /Generate draft/ }));
    await waitFor(() => expect(screen.getByLabelText("Prompt body editor")).toHaveValue("Generated prompt body"));
    expect(invoke).not.toHaveBeenCalledWith("hiveai_prompt_dispatch", expect.anything());
    fireEvent.change(screen.getByLabelText("Prompt body editor"), { target: { value: "Edited prompt body" } });
    fireEvent.click(screen.getByRole("button", { name: /Save edit/ }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_prompt_edit", expect.objectContaining({ request: expect.objectContaining({ projectId: project.id, promptId: "prompt-1", versionId: "version-1", content: "Edited prompt body" }) })));
    fireEvent.click(screen.getByRole("button", { name: /Approve exact version/ }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_prompt_approve", { request: { projectId: project.id, promptId: "prompt-1", versionId: "version-1" } }));
    fireEvent.click(screen.getByRole("button", { name: /Dispatch approved prompt/ }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_prompt_dispatch", { request: { projectId: project.id, promptId: "prompt-1", versionId: "version-1", provider: "CODEX" } }));
    expect(screen.getByText(/Dispatched CODEX session session-1/)).toBeInTheDocument();
  });
});
