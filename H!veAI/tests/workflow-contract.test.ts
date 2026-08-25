import { beforeEach, describe, expect, it, vi } from 'vitest';

const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock('@tauri-apps/api/core', () => ({ invoke }));

import {
  ACTOR_TYPES,
  WORKFLOW_HISTORY_MAX_LIMIT,
  WORKFLOW_STATES,
  getWorkflowHistory,
  transitionWorkflow,
  validateWorkflowLimit,
} from '../src/workflow';

describe('M10 TypeScript native contract', () => {
  beforeEach(() => invoke.mockReset());

  it('uses the exact native command names and canonical strings', async () => {
    expect(WORKFLOW_STATES).toContain('RE_AUDIT_REQUIRED');
    expect(ACTOR_TYPES).toEqual(['HUMAN', 'CODEX', 'CLAUDE', 'GPT_AUDIT', 'CI', 'EXTERNAL', 'SYSTEM']);
    await getWorkflowHistory('task-1', 10);
    expect(invoke).toHaveBeenCalledWith('hiveai_workflow_history', { query: { taskId: 'task-1', limit: 10 } });
  });

  it('sends expected state and request identity with mutations', async () => {
    await transitionWorkflow({
      taskId: 'task-1', expectedFromState: 'BACKLOG', toState: 'PLANNING_REQUIRED',
      actorType: 'HUMAN', requestId: 'request-1', summary: 'plan',
    });
    expect(invoke).toHaveBeenCalledWith('hiveai_workflow_transition', {
      request: expect.objectContaining({ expectedFromState: 'BACKLOG', requestId: 'request-1', evidenceRefs: [] }),
    });
  });

  it('rejects unbounded history/list limits', () => {
    expect(validateWorkflowLimit()).toBe(100);
    expect(validateWorkflowLimit(WORKFLOW_HISTORY_MAX_LIMIT)).toBe(WORKFLOW_HISTORY_MAX_LIMIT);
    expect(() => validateWorkflowLimit(0)).toThrow();
    expect(() => validateWorkflowLimit(WORKFLOW_HISTORY_MAX_LIMIT + 1)).toThrow();
  });

  it('does not invent browser-preview workflow state', () => {
    expect(Object.keys(globalThis)).not.toContain('hiveaiWorkflowState');
    expect(invoke).not.toHaveBeenCalled();
  });
});
