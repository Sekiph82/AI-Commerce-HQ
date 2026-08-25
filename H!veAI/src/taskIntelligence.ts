import { invoke } from "@tauri-apps/api/core";

export type ParserWarning = { code: string; message: string; sourcePath: string | null };
export type TaskEvidenceLocator = {
  sourcePath: string;
  contentHash: string;
  startLine: number;
  endLine: number;
  headingPath: string[];
  locatorText: string | null;
};
export type TaskConfidence = { score: number; reasons: string[] };
export type ParsedTask = {
  id: string;
  projectId: string;
  sourceId: string;
  sourcePath: string;
  sourceKind: string;
  title: string;
  parsedStatus: string;
  storageState: string;
  explicitTaskId: string | null;
  milestone: string | null;
  requiredActor: string | null;
  blockers: string[];
  dependencyReferences: string[];
  nextStep: string | null;
  ownerGate: string | null;
  externalWait: string | null;
  acceptanceCriteria: string[];
  confidence: TaskConfidence;
  evidence: TaskEvidenceLocator;
  adapterId: string;
  warnings: string[];
};
export type HandoffSummary = {
  current: string[];
  next: string[];
  blockers: string[];
  waiting: string[];
  evidence: TaskEvidenceLocator[];
};
export type TaskIntelligenceSnapshot = {
  projectId: string;
  parsedAt: string;
  adapter: { id: string; evidence: string; conventionMatched: boolean };
  tasks: ParsedTask[];
  handoff: HandoffSummary | null;
  warnings: ParserWarning[];
};

export const parseTaskIntelligence = (projectId: string) =>
  invoke<TaskIntelligenceSnapshot>("hiveai_task_intelligence_parse", { projectId });

export const listTaskIntelligence = (projectId: string) =>
  invoke<TaskIntelligenceSnapshot>("hiveai_task_intelligence_list", { projectId });
