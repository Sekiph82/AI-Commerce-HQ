import React from "react";
import {
  isTauriDesktop,
  listRegisteredProjects,
  type ProjectRecord,
} from "./projectRegistry";
import { projects as fixtureProjects } from "./fixtures";
import type { Project } from "./types";

type RegistryContextValue = {
  records: ProjectRecord[];
  projects: Project[];
  loading: boolean;
  error: string | null;
  selectedProjectId: string | null;
  selectProject: (projectId: string | null) => void;
  refresh: () => Promise<void>;
};

const RegistryContext = React.createContext<RegistryContextValue | null>(null);

function toProject(record: ProjectRecord): Project {
  const code =
    record.name
      .replace(/[^A-Za-z0-9]/g, "")
      .slice(0, 2)
      .toUpperCase() || "PR";
  return {
    id: record.id,
    name: record.name,
    code,
    description: record.originalPath,
    phase: record.repository?.isGitRepository
      ? "Registered Git project"
      : "Registered local project",
    task: "No parsed task data yet",
    progress: 0,
    health:
      record.status === "ACTIVE"
        ? "Healthy"
        : record.status === "MISSING"
          ? "Watch"
          : "Blocked",
    state:
      record.status === "ACTIVE" ? "READY_FOR_IMPLEMENTATION" : "WAITING_OWNER",
    actor: "Codex",
    lastAction: "Project registered in H!veAI",
    nextAction: "Add task-source evidence in a later milestone",
    updated: record.lastValidatedAt ?? record.registeredAt,
    metrics: [
      { label: "Status", value: record.status },
      { label: "Priority", value: String(record.priority) },
      {
        label: "Git",
        value: record.repository?.isGitRepository ? "Yes" : "No",
      },
    ],
  };
}

export function RegistryProvider({ children }: { children: React.ReactNode }) {
  const live = isTauriDesktop();
  const [records, setRecords] = React.useState<ProjectRecord[]>([]);
  const [loading, setLoading] = React.useState(live);
  const [error, setError] = React.useState<string | null>(null);
  const [selectedProjectId, setSelectedProjectId] = React.useState<
    string | null
  >(live ? null : (fixtureProjects[0]?.id ?? null));
  const selectProject = React.useCallback((projectId: string | null) => {
    setSelectedProjectId(projectId);
  }, []);
  const refresh = React.useCallback(async () => {
    if (!live) {
      setLoading(false);
      return;
    }
    setLoading(true);
    try {
      setRecords(await listRegisteredProjects());
      setError(null);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setLoading(false);
    }
  }, [live]);
  React.useEffect(() => {
    void refresh();
  }, [refresh]);
  React.useEffect(() => {
    if (!live) return;
    if (
      selectedProjectId &&
      records.some((record) => record.id === selectedProjectId)
    ) {
      return;
    }
    setSelectedProjectId(records[0]?.id ?? null);
  }, [live, records, selectedProjectId]);
  const value = React.useMemo(
    () => ({
      records,
      projects: live ? records.map(toProject) : fixtureProjects,
      loading,
      error,
      selectedProjectId,
      selectProject,
      refresh,
    }),
    [error, live, loading, records, refresh, selectProject, selectedProjectId],
  );
  return (
    <RegistryContext.Provider value={value}>
      {children}
    </RegistryContext.Provider>
  );
}

export function useProjectRegistry() {
  const value = React.useContext(RegistryContext);
  if (!value)
    throw new Error("useProjectRegistry must be used inside RegistryProvider");
  return value;
}
