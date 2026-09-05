import { invoke } from "@tauri-apps/api/core";

export type PromptKind = "IMPLEMENTATION" | "REMEDIATION" | "AUDIT_SUPPORT";
export type ContextDisposition = "INCLUDED" | "OMITTED" | "TRUNCATED" | "STALE" | "UNAVAILABLE" | "EXCLUDED";
export type ContextItem = { class: string; reference: string; disposition: ContextDisposition; bytes: number; reason: string | null; value: string | null };
export type ContextManifest = { projectId: string; taskId: string | null; items: ContextItem[]; includedBytes: number; omittedCount: number; sourceCount: number; manifestSha256: string };
export type PromptVersion = {
  id: string; promptId: string; version: number; kind: PromptKind; title: string | null; summary: string | null;
  content: string; createdBy: string; createdAt: string; origin: string; contextManifest: ContextManifest | null;
  provenance: Record<string, unknown>; approvalState: string; approvedAt: string | null; approvedBodySha256: string | null;
  usedAt: string | null; selectedProvider: "CODEX" | "CLAUDE" | null; dispatchedSessionId: string | null; supersededAt: string | null;
  dispatchState: string; dispatchReservationId: string | null; dispatchReservedAt: string | null; dispatchProvenance: Record<string, unknown>; dispatchError: string | null;
  bodySha256: string; isCurrent: boolean;
};
export type PromptRecord = { id: string; projectId: string | null; taskId: string | null; kind: PromptKind; currentVersion: number; createdAt: string; updatedAt: string; current: PromptVersion | null };
export type PromptDispatchResult = { prompt: PromptVersion; session: { id: string; provider: "CODEX" | "CLAUDE" }; promptId: string; promptVersionId: string; promptVersion: number; promptVersionSha256: string };

export const collectPromptContext = (projectId: string, taskId: string | null) => invoke<ContextManifest>("hiveai_prompt_context_collect", { request: { projectId, taskId } });
export const generatePrompt = (request: { projectId: string; taskId: string | null; kind: PromptKind; title: string; summary: string; findingIds?: string[] }) => invoke<PromptVersion>("hiveai_prompt_generate", { request });
export const listPrompts = (projectId: string) => invoke<PromptRecord[]>("hiveai_prompts_list", { projectId });
export const listPromptVersions = (projectId: string, promptId: string) => invoke<PromptVersion[]>("hiveai_prompt_versions", { projectId, promptId });
export const editPrompt = (request: { projectId: string; promptId: string; versionId: string; content: string; title?: string; summary?: string }) => invoke<PromptVersion>("hiveai_prompt_edit", { request });
export const approvePrompt = (projectId: string, promptId: string, versionId: string) => invoke<PromptVersion>("hiveai_prompt_approve", { request: { projectId, promptId, versionId } });
export const dispatchPrompt = (projectId: string, promptId: string, versionId: string, provider: "CODEX" | "CLAUDE") => invoke<PromptDispatchResult>("hiveai_prompt_dispatch", { request: { projectId, promptId, versionId, provider } });
