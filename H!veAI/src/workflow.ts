import { invoke } from '@tauri-apps/api/core';

export const WORKFLOW_STATES = [
  'BACKLOG', 'PLANNING_REQUIRED', 'PROMPT_REQUIRED', 'PROMPT_READY',
  'READY_FOR_IMPLEMENTATION', 'BUILDER_RUNNING', 'IMPLEMENTATION_COMPLETE',
  'AUDIT_REQUIRED', 'AUDIT_RUNNING', 'AUDIT_PASSED', 'VERIFY_REQUIRED',
  'VERIFY_RUNNING', 'TASK_COMPLETE', 'AUDIT_FAILED', 'FIX_REQUIRED',
  'RE_AUDIT_REQUIRED', 'BLOCKED', 'WAITING_HUMAN', 'WAITING_EXTERNAL', 'DESIGN_GATE',
] as const;
export type WorkflowState = typeof WORKFLOW_STATES[number];
export const ACTOR_TYPES = ['HUMAN', 'CODEX', 'CLAUDE', 'GPT_AUDIT', 'CI', 'EXTERNAL', 'SYSTEM'] as const;
export type ActorType = typeof ACTOR_TYPES[number];
export type EvidenceKind = 'PROMPT' | 'AGENT_SESSION' | 'AUDIT' | 'TEST_RUN' | 'DECISION' | 'GIT_SNAPSHOT' | 'TASK_SOURCE' | 'EXTERNAL_REFERENCE';
export type EvidenceRef = { kind: EvidenceKind; id: string; locator?: string | null };

export type WorkflowEvent = {
  id: string; taskId: string; eventType: string; fromState: WorkflowState | null; toState: WorkflowState | null;
  actorType: ActorType | null; summary: string; evidenceRefs: EvidenceRef[]; occurredAt: string;
};
export type WorkflowTask = {
  taskId: string; projectId: string; title: string; currentState: WorkflowState; workflowManaged: boolean;
  sourceActive: boolean; sourceRetired: boolean; allowedNextStates: WorkflowState[]; allowedActors: ActorType[];
  suspensionResumeState: WorkflowState | null; latestEvent: WorkflowEvent | null; attentionRequired: boolean;
  requiredActor: string | null; milestone: string | null;
};
export type WorkflowTransitionRequest = {
  taskId: string; expectedFromState: WorkflowState; toState: WorkflowState; actorType: ActorType;
  requestId: string; summary: string; evidenceRefs?: EvidenceRef[];
};
export type WorkflowOverrideRequest = {
  taskId: string; expectedFromState: WorkflowState; toState: WorkflowState; requestId: string;
  rationale: string; evidenceRefs?: EvidenceRef[];
};

export const WORKFLOW_HISTORY_DEFAULT_LIMIT = 100;
export const WORKFLOW_HISTORY_MAX_LIMIT = 500;
export function validateWorkflowLimit(limit = WORKFLOW_HISTORY_DEFAULT_LIMIT) {
  if (!Number.isInteger(limit) || limit < 1 || limit > WORKFLOW_HISTORY_MAX_LIMIT) {
    throw new Error(`workflow history/list limit must be 1..=${WORKFLOW_HISTORY_MAX_LIMIT}`);
  }
  return limit;
}

export const getWorkflowTask = (taskId: string) => invoke<WorkflowTask>('hiveai_workflow_task_get', { taskId });
export const listWorkflowTasks = (projectId: string, limit = WORKFLOW_HISTORY_DEFAULT_LIMIT) =>
  invoke<{ projectId: string; tasks: WorkflowTask[] }>('hiveai_workflow_project_list', { query: { projectId, limit: validateWorkflowLimit(limit) } });
export const getWorkflowHistory = (taskId: string, limit = WORKFLOW_HISTORY_DEFAULT_LIMIT) =>
  invoke<WorkflowEvent[]>('hiveai_workflow_history', { query: { taskId, limit: validateWorkflowLimit(limit) } });
export const transitionWorkflow = (request: WorkflowTransitionRequest) =>
  invoke<WorkflowEvent>('hiveai_workflow_transition', { request: { ...request, evidenceRefs: request.evidenceRefs ?? [] } });
export const overrideWorkflow = (request: WorkflowOverrideRequest) =>
  invoke<WorkflowEvent>('hiveai_workflow_override', { request: { ...request, evidenceRefs: request.evidenceRefs ?? [] } });
