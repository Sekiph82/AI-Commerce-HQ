import { invoke } from "@tauri-apps/api/core";
import type { GitDiff, GitSnapshot } from "./gitEngine";
import type { ProjectRecord } from "./projectRegistry";
import type { DiscoveredProjectSource } from "./taskSources";
import type { TaskIntelligenceSnapshot } from "./taskIntelligence";
import type { WorkflowEvent, WorkflowTask } from "./workflow";

export type CockpitTestRun = {
  id: string;
  taskId: string | null;
  command: string;
  result: string;
  outputMetadata: string | null;
  startedAt: string;
  finishedAt: string | null;
};

export type CockpitAuditFinding = {
  id: string;
  severity: string;
  title: string;
  detail: string | null;
  filePath: string | null;
  lineNumber: number | null;
  createdAt: string;
};

export type CockpitAudit = {
  id: string;
  taskId: string | null;
  result: string;
  summary: string | null;
  confidence: number | null;
  createdAt: string;
  findings: CockpitAuditFinding[];
};

export type CockpitAgentSession = {
  id: string;
  taskId: string | null;
  provider: string;
  state: string;
  startedAt: string | null;
  endedAt: string | null;
  createdAt: string;
};

export type CockpitPermission = {
  id: string;
  sessionId: string | null;
  permissionKind: string;
  requestedResource: string | null;
  state: string;
  decidedBy: string | null;
  createdAt: string;
  decidedAt: string | null;
};

export type CockpitActivity = {
  id: string;
  kind: string;
  event: string;
  state: string | null;
  actor: string | null;
  occurredAt: string;
  source: string;
};

export type CockpitFileEntry = {
  path: string;
  role: string;
  status: string;
  sourceKind: string | null;
  evidence: string;
};

export type ProjectDashboardResolution = {
  projectId: string;
  manifestStatus: string;
  manifestPath: string;
  schema: string | null;
  projectKey: string | null;
  repository: string | null;
  branchPolicy: string | null;
  dashboardMode: string | null;
  trackingMode: string | null;
  refreshPolicy: string | null;
  taskAuthority: string;
  canonicalTaskSource: string | null;
  roles: Record<string, Array<{ path: string; role: string; status: string; exists: boolean; contained: boolean }>>;
  provenanceMode: string;
  materialized: {
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
  warnings: string[];
};

export type ProjectCockpitSnapshot = {
  project: ProjectRecord;
  projectSummary: {
    projectId: string;
    health: string;
    manifestStatus: string;
    taskAuthority: string;
    provenanceMode: string;
    currentTask: { taskId: string; title: string; sourcePath: string; parsedStatus: string; workflowState: string | null; requiredActor: string | null } | null;
    currentState: string | null;
    lastAction: { summary: string; occurredAt: string; actor: string | null } | null;
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
  dashboard: ProjectDashboardResolution;
  taskIntelligence: TaskIntelligenceSnapshot | null;
  taskIntelligenceError: string | null;
  workflow: { projectId: string; tasks: WorkflowTask[] };
  workflowHistory: WorkflowEvent[];
  git: GitSnapshot | null;
  gitError: string | null;
  gitDiff: GitDiff | null;
  gitDiffError: string | null;
  sources: DiscoveredProjectSource[];
  sourcesError: string | null;
  tests: CockpitTestRun[];
  audits: CockpitAudit[];
  agentSessions: CockpitAgentSession[];
  permissions: CockpitPermission[];
  activity: CockpitActivity[];
  files: CockpitFileEntry[];
  warnings: string[];
  generatedAt: string;
};

export const getProjectCockpitSnapshot = (projectId: string) =>
  invoke<ProjectCockpitSnapshot>("hiveai_project_cockpit_snapshot", { projectId });
