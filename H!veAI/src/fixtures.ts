import type { ActivityEvent, Project } from './types';

// M02 fixtures are static presentation data. They are deliberately not read from live repositories.
export const projects: Project[] = [
  {
    id: 'formulab', name: 'FormuLab', code: 'FL', description: 'Formula intelligence workspace',
    phase: 'M07 Watcher', task: 'Connect project snapshot events', progress: 72, health: 'Healthy',
    state: 'CODEX_RUNNING', actor: 'Codex', lastAction: 'Watcher contract reviewed',
    nextAction: 'Complete snapshot fixture coverage', updated: '8 min ago',
    metrics: [{ label: 'Open tasks', value: '12' }, { label: 'Audit score', value: '94%' }, { label: 'Sessions', value: '2' }],
  },
  {
    id: 'fmcg-erp', name: 'FMCG ERP', code: 'FE', description: 'Operations planning and inventory',
    phase: 'M04 Database', task: 'Review migration recovery notes', progress: 48, health: 'Watch',
    state: 'AUDIT_REQUIRED', actor: 'GPT Audit', lastAction: 'Schema migration completed',
    nextAction: 'Review audit findings', updated: '22 min ago',
    metrics: [{ label: 'Open tasks', value: '8' }, { label: 'Audit score', value: 'Pending' }, { label: 'Sessions', value: '1' }],
  },
  {
    id: 'scrubbots', name: 'Scrubbots', code: 'SB', description: 'Robotics operations platform',
    phase: 'M03 Runtime', task: 'Confirm worker boundary decision', progress: 31, health: 'Blocked',
    state: 'WAITING_EXTERNAL', actor: 'External', lastAction: 'Requested vendor capability matrix',
    nextAction: 'Wait for vendor response', updated: '1 hr ago',
    metrics: [{ label: 'Open tasks', value: '19' }, { label: 'Audit score', value: '88%' }, { label: 'Sessions', value: '0' }],
  },
  {
    id: 'packlab', name: 'PackLab 3D', code: 'P3', description: 'Packaging workflow design tool',
    phase: 'M02 UI Shell', task: 'Validate cockpit navigation model', progress: 84, health: 'Healthy',
    state: 'TASK_COMPLETE', actor: 'Human', lastAction: 'UI route map approved',
    nextAction: 'Begin runtime architecture review', updated: '2 hrs ago',
    metrics: [{ label: 'Open tasks', value: '4' }, { label: 'Audit score', value: '98%' }, { label: 'Sessions', value: '1' }],
  },
];

export const activity: ActivityEvent[] = [
  { id: 'a1', time: '8 min ago', project: 'FormuLab', actor: 'Codex', event: 'Implementation finished', state: 'CODEX_RUNNING' },
  { id: 'a2', time: '22 min ago', project: 'FMCG ERP', actor: 'GPT Audit', event: 'Audit requested', state: 'AUDIT_REQUIRED' },
  { id: 'a3', time: '41 min ago', project: 'PackLab 3D', actor: 'CI', event: 'Tests passed', state: 'AUDIT_PASSED' },
  { id: 'a4', time: '1 hr ago', project: 'Scrubbots', actor: 'External', event: 'Task moved to WAITING_EXTERNAL', state: 'WAITING_EXTERNAL' },
  { id: 'a5', time: '2 hrs ago', project: 'PackLab 3D', actor: 'Human', event: 'Branch changed', state: 'TASK_COMPLETE' },
];

export const attention = [
  { label: 'WAITING EXTERNAL', state: 'WAITING_EXTERNAL' as const, project: 'Scrubbots', detail: 'Vendor capability matrix is pending.', icon: 'clock' },
  { label: 'AUDIT REQUIRED', state: 'AUDIT_REQUIRED' as const, project: 'FMCG ERP', detail: 'Migration recovery notes need an independent review.', icon: 'shield' },
  { label: 'CLAUDE READY', state: 'READY_FOR_IMPLEMENTATION' as const, project: 'FormuLab', detail: 'A reviewed prompt is ready for a future session.', icon: 'sparkles' },
  { label: 'WAITING OWNER', state: 'WAITING_OWNER' as const, project: 'PackLab 3D', detail: 'A design gate needs a human decision.', icon: 'user' },
];

export const queue = [
  { project: 'FormuLab', task: 'Connect project snapshot events', stage: 'Implementation', actor: 'Codex' as const, state: 'CODEX_RUNNING' as const, updated: '8 min ago' },
  { project: 'FMCG ERP', task: 'Review migration recovery notes', stage: 'Audit', actor: 'GPT Audit' as const, state: 'AUDIT_REQUIRED' as const, updated: '22 min ago' },
  { project: 'Scrubbots', task: 'Confirm worker boundary decision', stage: 'External wait', actor: 'External' as const, state: 'WAITING_EXTERNAL' as const, updated: '1 hr ago' },
  { project: 'PackLab 3D', task: 'Validate cockpit navigation model', stage: 'Complete', actor: 'Human' as const, state: 'TASK_COMPLETE' as const, updated: '2 hrs ago' },
];
