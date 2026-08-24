import { invoke } from '@tauri-apps/api/core';

export type ProjectWatcherStatus = {
  projectId: string;
  state: 'WATCHING' | 'MISSING' | 'DEGRADED' | 'PAUSED';
  watcherHealth: 'HEALTHY' | 'DEGRADED' | 'OVERFLOW' | 'MISSING';
  available: boolean;
  lastEventAt: string | null;
  lastRefreshAt: string | null;
  evidenceGeneratedAt: string | null;
  changedPathCount: number;
  rescanRequired: boolean;
};

export type WatcherStatusSummary = { running: boolean; queueDepth: number; queueCapacity: number; projects: ProjectWatcherStatus[] };

export function getWatcherStatus() { return invoke<WatcherStatusSummary>('hiveai_watcher_status'); }
export function refreshWatcherSet() { return invoke<WatcherStatusSummary>('hiveai_watcher_refresh_set'); }
export function rescanProject(projectId: string) { return invoke<ProjectWatcherStatus>('hiveai_watcher_rescan', { projectId }); }
