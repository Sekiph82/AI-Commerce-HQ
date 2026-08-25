import { invoke } from "@tauri-apps/api/core";

export type SourceStatus = "AVAILABLE" | "MISSING" | "TOO_LARGE" | "UNREADABLE" | "LIMIT_REACHED" | "OUTSIDE_ROOT";

export type DiscoveredProjectSource = {
  id: string;
  projectId: string;
  relativePath: string;
  absolutePath: string;
  sourceKind: string;
  origin: "STANDARD" | "CUSTOM" | "SYSTEM";
  status: SourceStatus;
  authorityClass: string;
  priority: number;
  sizeBytes: number | null;
  modifiedAt: string | null;
  discoveredAt: string;
  contentHash: string | null;
  depth: number;
  warnings: string[];
  schemaVersion: number;
  owner: string;
  sourceOrder: number | null;
};

export type CustomSourcePath = {
  id: string;
  projectId: string;
  displayPath: string;
  normalizedPath: string;
  status: string;
  order: number;
};

export const listTaskSources = (projectId: string) =>
  invoke<DiscoveredProjectSource[]>("hiveai_task_sources_list", { projectId });

export const discoverTaskSources = (projectId: string) =>
  invoke<DiscoveredProjectSource[]>("hiveai_task_sources_discover", { projectId });

export const listCustomSourcePaths = (projectId: string) =>
  invoke<CustomSourcePath[]>("hiveai_task_source_custom_paths_list", { projectId });

export const addCustomSourcePath = (projectId: string, path: string) =>
  invoke<CustomSourcePath[]>("hiveai_task_source_custom_path_add", {
    request: { projectId, path },
  });

export const removeCustomSourcePath = (projectId: string, pathOrId: string) =>
  invoke<CustomSourcePath[]>("hiveai_task_source_custom_path_remove", {
    projectId,
    pathOrId,
  });

export const updateCustomSourcePath = (request: { projectId: string; pathOrId: string; path?: string; order?: number }) =>
  invoke<CustomSourcePath[]>("hiveai_task_source_custom_path_update", { request });
