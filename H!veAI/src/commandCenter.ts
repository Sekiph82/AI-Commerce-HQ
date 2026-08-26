import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { ProjectRecord } from "./projectRegistry";

export type ManifestStatus = "VALID" | "PARTIAL" | "ABSENT" | "MALFORMED" | "STALE" | "UNAVAILABLE";
export type TaskAuthority = "CANONICAL" | "NOT_CANONICALIZED" | "FALLBACK_M08_M09";

export type CommandCenterTask = {
  taskId: string;
  title: string;
  sourcePath: string;
  parsedStatus: string;
  workflowState: string | null;
  requiredActor: string | null;
};

export type CommandCenterAction = { summary: string; occurredAt: string; actor: string | null };
export type MaterializedDashboardStatus = {
  projectStatus: string | null;
  health: string | null;
  currentMilestone: string | null;
  currentTaskTitle: string | null;
  currentTaskId: string | null;
  declaredWorkflowState: string | null;
  progressRaw: string | null;
  progressPercent: number | null;
  requiredActor: string | null;
  nextAction: string | null;
  waitingOn: string | null;
  lastMeaningfulUpdate: string | null;
  currentWork: Array<{ id: string; item: string; status: string; ownerActor: string; evidenceSource: string }>;
  blockersWaiting: string[];
  milestoneSummary: string[];
  qualityVerification: Array<{ label: string; value: string }>;
  recentMeaningfulActivity: string[];
  provenance: Array<{ label: string; value: string }>;
};
export type CommandCenterProject = {
  projectId: string;
  name: string;
  registryStatus: string;
  health: "MISSING" | "BLOCKED" | "ATTENTION" | "RUNNING" | "HEALTHY" | "UNKNOWN";
  manifestStatus: ManifestStatus;
  trackingMode: string | null;
  taskAuthority: TaskAuthority;
  provenanceMode: string;
  materialized: MaterializedDashboardStatus;
  canonicalTaskSource: string | null;
  currentTask: CommandCenterTask | null;
  currentState: string | null;
  lastAction: CommandCenterAction | null;
  nextAction: string | null;
  allowedActors: string[];
  totalTasks: number | null;
  activeTasks: number | null;
  completedTasks: number | null;
  progressPercent: number | null;
  warnings: string[];
  refreshStatus: string | null;
  refreshAt: string | null;
  refreshError: string | null;
};

export type AttentionItem = { id: string; projectId: string; projectName: string; taskId: string | null; title: string; state: string; detail: string; category: string };
export type WorkQueueItem = { id: string; projectId: string; projectName: string; taskId: string; task: string; stage: string; state: string; actor: string | null; updatedAt: string | null; attention: boolean };
export type ActivityItem = { id: string; projectId: string; projectName: string; kind: string; event: string; state: string | null; actor: string | null; occurredAt: string };
export type BriefFact = { label: string; value: string; source: string; provenance: { sourceClass: string; projectId: string | null; sourcePath: string | null; evidenceType: string | null; evidenceId: string | null } };
export type CommandCenterSnapshot = {
  generatedAt: string;
  projects: CommandCenterProject[];
  kpis: { projects: number; activeTasks: number | null; needsAttention: number | null; running: number | null; completedTasks: number | null; healthy: number; healthDetail: string; authorityDetail: string };
  attention: AttentionItem[];
  workQueue: WorkQueueItem[];
  recentActivity: ActivityItem[];
  engineeringBrief: { facts: BriefFact[]; recommendation: string | null };
  warnings: string[];
};

export type CommandCenterRefreshEvent = { projectId: string; category: string; generatedAt: string; success: boolean };

export const getCommandCenterSnapshot = () => invoke<CommandCenterSnapshot>("hiveai_command_center_snapshot");
export const listenForCommandCenterRefresh = (handler: (event: CommandCenterRefreshEvent) => void): Promise<UnlistenFn> =>
  listen<CommandCenterRefreshEvent>("hiveai-command-center-refresh", (event) => handler(event.payload));

export function registryFallback(records: ProjectRecord[]): CommandCenterSnapshot {
  const projects = records.map((record) => ({
    projectId: record.id, name: record.name, registryStatus: record.status, health: "UNKNOWN" as const,
    manifestStatus: "UNAVAILABLE" as const, trackingMode: null, taskAuthority: "FALLBACK_M08_M09" as const, provenanceMode: "REGISTRY_ONLY",
    materialized: { projectStatus: null, health: null, currentMilestone: null, currentTaskTitle: null, currentTaskId: null, declaredWorkflowState: null, progressRaw: null, progressPercent: null, requiredActor: null, nextAction: null, waitingOn: null, lastMeaningfulUpdate: null, currentWork: [], blockersWaiting: [], milestoneSummary: [], qualityVerification: [], recentMeaningfulActivity: [], provenance: [] },
    canonicalTaskSource: null, currentTask: null, currentState: null, lastAction: null, nextAction: null, allowedActors: [],
    totalTasks: null, activeTasks: null, completedTasks: null, progressPercent: null, warnings: ["Live Command Center snapshot is unavailable; showing Registry identity only."], refreshStatus: "UNAVAILABLE", refreshAt: null, refreshError: null,
  }));
  return { generatedAt: new Date().toISOString(), projects, kpis: { projects: records.length, activeTasks: null, needsAttention: null, running: null, completedTasks: null, healthy: 0, healthDetail: `${records.length} registered`, authorityDetail: "Task evidence unavailable" }, attention: [], workQueue: [], recentActivity: [], engineeringBrief: { facts: [], recommendation: null }, warnings: ["Live Command Center snapshot is unavailable; showing Registry identity only."] };
}

export function previewSnapshot(): CommandCenterSnapshot {
  return {
    generatedAt: new Date().toISOString(),
    projects: [],
    kpis: { projects: 0, activeTasks: null, needsAttention: null, running: null, completedTasks: null, healthy: 0, healthDetail: "Native data unavailable", authorityDetail: "Native data unavailable" },
    attention: [], workQueue: [], recentActivity: [],
    engineeringBrief: { facts: [], recommendation: null },
    warnings: ["Browser preview does not have access to the native Project Registry or local task evidence."],
  };
}
