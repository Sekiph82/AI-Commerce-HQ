export type WorkflowState =
  | 'BACKLOG'
  | 'READY_FOR_IMPLEMENTATION'
  | 'CODEX_RUNNING'
  | 'CLAUDE_RUNNING'
  | 'AUDIT_REQUIRED'
  | 'AUDIT_PASSED'
  | 'AUDIT_FAILED'
  | 'FIX_REQUIRED'
  | 'VERIFY_REQUIRED'
  | 'WAITING_OWNER'
  | 'WAITING_EXTERNAL'
  | 'BLOCKED'
  | 'FAILED'
  | 'TASK_COMPLETE';

export type Actor = 'Human' | 'Codex' | 'Claude' | 'GPT Audit' | 'CI' | 'External';

export type Project = {
  id: string;
  name: string;
  code: string;
  description: string;
  phase: string;
  task: string;
  progress: number;
  health: 'Healthy' | 'Watch' | 'Blocked';
  state: WorkflowState;
  actor: Actor;
  lastAction: string;
  nextAction: string;
  updated: string;
  metrics: { label: string; value: string }[];
};

export type ActivityEvent = {
  id: string;
  time: string;
  project: string;
  actor: Actor;
  event: string;
  state: WorkflowState;
};
