import { invoke } from '@tauri-apps/api/core';

export type RegistryStatus = 'ACTIVE' | 'MISSING' | 'ARCHIVED';

export type RemoteMetadata = { name: string; url: string };

export type RepositoryRecord = {
  id: string;
  isGitRepository: boolean;
  repositoryRoot: string | null;
  currentBranch: string | null;
  headSha: string | null;
  preferredRemoteUrl: string | null;
  defaultBranch: string | null;
  githubOwner: string | null;
  githubRepo: string | null;
  remotes: RemoteMetadata[];
};

export type ProjectRecord = {
  id: string;
  name: string;
  originalPath: string;
  normalizedPath: string;
  status: RegistryStatus;
  priority: number;
  preferredBuilder: string | null;
  preferredAuditor: string | null;
  taskSourcePolicy: string | null;
  preferredAgentProvider?: 'CODEX' | 'CLAUDE' | null;
  registeredAt: string;
  lastValidatedAt: string | null;
  repository: RepositoryRecord | null;
};

export type ProjectListQuery = {
  search?: string;
  status?: RegistryStatus | null;
  sort?: 'name' | 'priority' | 'updated';
  includeArchived?: boolean;
};

export const isTauriDesktop = () => '__TAURI_INTERNALS__' in window;

export function listRegisteredProjects(query: ProjectListQuery = {}) {
  return invoke<ProjectRecord[]>('hiveai_projects_list', { query });
}

export function registerProject(path: string, name: string) {
  return invoke<ProjectRecord>('hiveai_project_register', { request: { path, name: name || null } });
}

export function getRegisteredProject(projectId: string) {
  return invoke<ProjectRecord>('hiveai_project_get', { projectId });
}

export function archiveProject(projectId: string) {
  return invoke<ProjectRecord>('hiveai_project_archive', { projectId });
}

export function removeProject(projectId: string) {
  return invoke<void>('hiveai_project_remove_from_registry', { projectId });
}

export function repairProjectPath(projectId: string, path: string) {
  return invoke<ProjectRecord>('hiveai_project_repair_path', { request: { projectId, path } });
}

export function updateProjectSettings(projectId: string, priority: number, preferredAgentProvider?: 'CODEX' | 'CLAUDE' | null) {
  return invoke<ProjectRecord>('hiveai_project_update_settings', { request: { projectId, priority, preferredAgentProvider } });
}

export function refreshWatcherSet() {
  return invoke('hiveai_watcher_refresh_set');
}
