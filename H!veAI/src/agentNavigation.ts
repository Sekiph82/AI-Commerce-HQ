export type AgentRouteTarget = {
  projectId: string;
  sessionId: string;
};

const ROUTE_ID = /^[A-Za-z0-9][A-Za-z0-9._:-]{0,255}$/;

export function parseAgentRouteTarget(search: string): AgentRouteTarget | null {
  const params = new URLSearchParams(search);
  const projectValues = params.getAll("projectId");
  const sessionValues = params.getAll("sessionId");
  if (projectValues.length !== 1 || sessionValues.length !== 1) return null;
  const projectId = projectValues[0]?.trim() ?? "";
  const sessionId = sessionValues[0]?.trim() ?? "";
  if (!ROUTE_ID.test(projectId) || !ROUTE_ID.test(sessionId)) return null;
  return { projectId, sessionId };
}

export function agentRouteTarget(target: AgentRouteTarget) {
  return `/agents?projectId=${encodeURIComponent(target.projectId)}&sessionId=${encodeURIComponent(target.sessionId)}`;
}
