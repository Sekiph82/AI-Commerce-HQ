import { invoke } from "@tauri-apps/api/core";

export type CodexReadiness = {
  provider: "CODEX";
  available: boolean;
  version: string | null;
  readinessState: string;
  diagnosticCode: string | null;
  diagnosticMessage: string | null;
  checkedAt: string;
};

export type CodexSession = {
  id: string;
  provider: "CODEX";
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
};

export function getCodexReadiness() {
  return invoke<CodexReadiness>("hiveai_codex_readiness");
}

export function listCodexSessions(projectId: string) {
  return invoke<CodexSession[]>("hiveai_codex_sessions_list", { projectId });
}

export function startCodexSession(projectId: string, prompt: string, taskId: string | null) {
  return invoke<CodexSession>("hiveai_codex_start", { request: { projectId, prompt, taskId } });
}

export function stopCodexSession(sessionId: string) {
  return invoke<CodexSession>("hiveai_codex_stop", { sessionId });
}

export function resumeCodexSession(sessionId: string) {
  return invoke<CodexSession>("hiveai_codex_resume", { sessionId });
}
