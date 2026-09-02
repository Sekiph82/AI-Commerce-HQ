import { invoke } from "@tauri-apps/api/core";

export type SessionProvider = "CODEX" | "CLAUDE";

export type ProviderReadiness = {
  provider: SessionProvider;
  available: boolean;
  version: string | null;
  readinessState: string;
  diagnosticCode: string | null;
  diagnosticMessage: string | null;
  capabilities: string[];
  supportsPty: boolean;
  supportsResume: boolean;
  checkedAt: string;
};

export type SessionEvent = {
  sequence: number;
  id: string;
  eventType: string;
  payload: Record<string, unknown> | null;
  occurredAt: string;
};

export type AgentSession = {
  id: string;
  provider: SessionProvider;
  projectId: string;
  taskId: string | null;
  operationKind: string;
  state: string;
  cwd: string;
  startedAt: string | null;
  endedAt: string | null;
  exitCode: number | null;
  stdout: string;
  stderr: string;
  stdoutTruncated: boolean;
  stderrTruncated: boolean;
  diagnosticCode: string | null;
  diagnosticMessage: string | null;
  promptReference: string | null;
  providerVersion: string | null;
  elapsedMs: number | null;
  supportsResume: boolean;
  supportsPty: boolean;
  events: SessionEvent[];
};

export function getAgentReadiness() {
  return invoke<ProviderReadiness[]>("hiveai_agent_readiness");
}

export function listAgentSessions(projectId: string) {
  return invoke<AgentSession[]>("hiveai_agent_sessions_list", { projectId });
}

export function startAgentSession(projectId: string, provider: SessionProvider, prompt: string, taskId: string | null) {
  return invoke<AgentSession>("hiveai_agent_start", { request: { projectId, provider, prompt, taskId } });
}

export function stopAgentSession(projectId: string, sessionId: string) {
  return invoke<AgentSession>("hiveai_agent_stop", { projectId, sessionId });
}

export function retryAgentSession(session: AgentSession, prompt: string) {
  return invoke<AgentSession>("hiveai_agent_retry", { request: { sourceSessionId: session.id, provider: session.provider, projectId: session.projectId, taskId: session.taskId, prompt } });
}

export function resumeAgentSession(projectId: string, sessionId: string) {
  return invoke<AgentSession>("hiveai_agent_resume", { projectId, sessionId });
}

export function decideAgentPermission(projectId: string, sessionId: string, permissionId: string, decision: "APPROVE" | "DENY") {
  return invoke<void>("hiveai_agent_permission_decision", { projectId, sessionId, permissionId, decision });
}

export function resizeAgentTerminal(projectId: string, sessionId: string, rows: number, columns: number) {
  return invoke<void>("hiveai_agent_resize", { projectId, sessionId, rows, columns });
}
