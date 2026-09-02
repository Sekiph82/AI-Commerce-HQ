import {
  ArrowLeft,
  ArrowUpRight,
  Check,
  ChevronRight,
  FolderKanban,
  GitBranch,
  MoreHorizontal,
  Plus,
  RefreshCw,
  Search,
  ShieldCheck,
  Terminal,
  UserRound,
  X,
} from "lucide-react";
import React from "react";
import "@xterm/xterm/css/xterm.css";
import type { FitAddon as FitAddonType } from "@xterm/addon-fit";
import type { Terminal as XTermType } from "@xterm/xterm";
import { invoke } from "@tauri-apps/api/core";
import { Link, useNavigate, useParams } from "react-router-dom";
import { activity, attention, projects, queue } from "./fixtures";
import {
  ActivityRow,
  ActorBadge,
  EmptyState,
  ErrorState,
  LoadingState,
  MetricCard,
  PageHeader,
  PrimaryActionButton,
  ProgressIndicator,
  ProjectOperationCard,
  SectionHeader,
  StatusBadge,
} from "./components/ui";
import { RuntimeStatusPanel } from "./components/RuntimeStatusPanel";
import { DatabaseStatusPanel } from "./components/DatabaseStatusPanel";
import { WatcherStatusPanel } from "./components/WatcherStatusPanel";
import { ProjectRegistryCard } from "./components/ProjectRegistryCard";
import {
  archiveProject,
  getRegisteredProject,
  isTauriDesktop,
  listRegisteredProjects,
  refreshWatcherSet,
  registerProject,
  removeProject,
  repairProjectPath,
  updateProjectSettings,
} from "./projectRegistry";
import type { ProjectRecord } from "./projectRegistry";
import { getGitDiff, getGitSnapshot } from "./gitEngine";
import type { GitDiff, GitSnapshot } from "./gitEngine";
import type { Project } from "./types";
import { useProjectRegistry } from "./registryContext";
import { CommandCenterLive } from "./command_center_view";
import { getCommandCenterSnapshot, type CommandCenterProject } from "./commandCenter";
import { getProjectCockpitSnapshot, type ProjectCockpitSnapshot } from "./projectCockpit";
import { getAgentReadiness, listAgentSessions, resizeAgentTerminal, retryAgentSession, startAgentSession, stopAgentSession, type AgentSession, type ProviderReadiness, type SessionProvider } from "./agentSessionCenter";
import { overrideWorkflow, type WorkflowState } from "./workflow";
import {
  addCustomSourcePath,
  discoverTaskSources,
  listCustomSourcePaths,
  listTaskSources,
  removeCustomSourcePath,
  updateCustomSourcePath,
  type CustomSourcePath,
  type DiscoveredProjectSource,
} from "./taskSources";

function Placeholder({
  title,
  description,
}: {
  title: string;
  description: string;
}) {
  return (
    <>
      <PageHeader
        title={title}
        description={description}
        action={
          <button className="secondary-button" type="button">
            <Plus size={15} />
            New view
          </button>
        }
      />
      <div className="placeholder-grid">
        <EmptyState
          title="UI surface ready"
          detail={title === "Task Sources" ? "Browser preview uses no filesystem discovery. Open the native H!veAI build for live sources." : "Live project data becomes available in a later milestone."}
        />
        <div className="placeholder-notes">
          <span className="eyebrow">M02 placeholder</span>
          <h2>Designed for the next workflow step</h2>
          <p>
            This view is intentionally static. It provides the navigation and
            states that future runtime services will populate.
          </p>
        </div>
      </div>
    </>
  );
}

function LegacyCommandCenter() {
  const navigate = useNavigate();
  const [notice, setNotice] = React.useState<string | null>(null);
  const {
    projects: registryProjects,
    records,
    loading: registryLoading,
    selectedProjectId,
    selectProject,
  } = useProjectRegistry();
  const liveProjects = isTauriDesktop() ? registryProjects : projects;
  const current =
    liveProjects.find((project) => project.id === selectedProjectId) ??
    liveProjects[0];
  return (
    <div className="command-center" aria-label="Command Center overview">
      {notice ? (
        <div className="safe-notice" role="status">
          {notice}
          <button
            type="button"
            onClick={() => setNotice(null)}
            aria-label="Dismiss message"
          >
            Dismiss
          </button>
        </div>
      ) : null}
      <header className="command-heading">
        <div>
          <h1>Global Overview</h1>
          <h1 className="sr-only">Command Center</h1>
          <span className="sr-only">Project operations</span>
        </div>
        <button
          className="secondary-button"
          type="button"
          onClick={() =>
            setNotice("Workspace actions are available in a later milestone.")
          }
        >
          <MoreHorizontal size={15} />
          Today
        </button>
      </header>
      <section className="command-kpis" aria-label="Portfolio metrics">
        <MetricCard
          label="Total projects"
          value={isTauriDesktop() ? String(records.length) : "10"}
          detail={isTauriDesktop() ? "Registered projects" : "Across workspace"}
        />
        <MetricCard
          label="Active"
          value={
            isTauriDesktop()
              ? String(
                  records.filter((record) => record.status === "ACTIVE").length,
                ).padStart(2, "0")
              : "07"
          }
          detail="Healthy operations"
          tone="blue"
        />
        <MetricCard
          label="On hold"
          value={
            isTauriDesktop()
              ? String(
                  records.filter((record) => record.status !== "ACTIVE").length,
                ).padStart(2, "0")
              : "02"
          }
          detail="Owner or external gate"
          tone="warning"
        />
        <MetricCard
          label="Completed"
          value={isTauriDesktop() ? "—" : "01"}
          detail="This cycle"
          tone="audit"
        />
        <MetricCard
          label="Total tasks"
          value={isTauriDesktop() ? "—" : "312"}
          detail={
            isTauriDesktop() ? "Task data unavailable" : "Across projects"
          }
          tone="running"
        />
        <MetricCard
          label="Avg health"
          value={isTauriDesktop() ? "—" : "87%"}
          detail="Portfolio signal"
          tone="external"
        />
      </section>
      <div className="command-layout">
        <section className="command-projects panel">
          <SectionHeader
            title="Projects"
            detail={
              registryLoading && isTauriDesktop()
                ? "Loading registry"
                : `${liveProjects.length} registered workspace${liveProjects.length === 1 ? "" : "s"}`
            }
            action={
              <Link className="text-link" to="/projects">
                All <ChevronRight size={13} />
              </Link>
            }
          />
          <div className="project-rail">
            {liveProjects.map((project) => (
              <button
                className={
                  project.id === selectedProjectId
                    ? "project-rail-row project-rail-row-selected"
                    : "project-rail-row"
                }
                type="button"
                key={project.id}
                aria-pressed={project.id === selectedProjectId}
                title={project.name}
                onClick={() => selectProject(project.id)}
              >
                <strong>{project.name}</strong>
              </button>
            ))}
            {!registryLoading && !liveProjects.length ? (
              <span className="rail-empty">No registered projects yet.</span>
            ) : null}
          </div>
          <button
            className="rail-footer"
            type="button"
            onClick={() => navigate("/projects")}
          >
            View all projects <ChevronRight size={13} />
          </button>
        </section>
        <section className="command-cockpit panel">
          <div className="cockpit-title">
            <div>
              <div className="cockpit-inline-label">
                <span className="eyebrow">Current project</span>
                <h2>
                  {current?.name ??
                    (registryLoading
                      ? "Loading registered project"
                      : "No registered project")}
                </h2>
              </div>
              <span>{current?.phase ?? "Registered project identity"}</span>
            </div>
            <StatusBadge state={current?.state ?? "WAITING_OWNER"} />
            <button
              className="secondary-button cockpit-open"
              type="button"
              disabled={!current}
              onClick={() => current && navigate(`/projects/${current.id}`)}
            >
              Open cockpit <ArrowUpRight size={14} />
            </button>
          </div>
          <div className="cockpit-tabs">
            <span className="tab-active">Cockpit</span>
            <span>Tasks</span>
            <span>Workflow</span>
            <span>Audit</span>
            <span>Logs</span>
          </div>
          <div className="cockpit-body">
            <div className="current-task">
              <div className="task-kicker">
                CURRENT TASK <span>{isTauriDesktop() ? "—" : "12 / 28"}</span>
              </div>
              <h3>{current?.task ?? "No parsed task data yet"}</h3>
              <p>
                {isTauriDesktop()
                  ? "Task and workflow details will populate from registered project evidence in a later milestone."
                  : "Intelligent form field suggestions based on user input and context analysis."}
              </p>
              <div className="task-meta">
                {isTauriDesktop() ? (
                  <span>Task evidence unavailable</span>
                ) : (
                  <>
                    <span>
                      Priority: <b>High</b>
                    </span>
                    <span>Type: Feature</span>
                    <span>Est: 3h</span>
                  </>
                )}
              </div>
              <div className="subtask-list">
                {isTauriDesktop() ? (
                  <div>No parsed task evidence yet.</div>
                ) : (
                  <>
                    <div>
                      <span className="check-done">✓</span>12.3.1 Design AI
                      suggestion schema <b>Done</b>
                    </div>
                    <div>
                      <span className="check-done">✓</span>12.3.2 Create
                      suggestion engine <b>Done</b>
                    </div>
                    <div>
                      <span className="check-active" />
                      12.3.4 Implement UI components <b>In progress</b>
                    </div>
                    <div>
                      <span className="check-pending" />
                      12.3.5 Add caching layer <b>Pending</b>
                    </div>
                  </>
                )}
              </div>
            </div>
            <div className="workflow-mini">
              <div className="task-kicker">WORKFLOW STATUS</div>
              {isTauriDesktop() ? (
                <div className="workflow-empty">
                  Workflow state unavailable.
                </div>
              ) : (
                [
                  "Prompt Preparation",
                  "Claude Code Execution",
                  "GPT Audit",
                  "Review & Approval",
                  "Deploy / Complete",
                ].map((step, index) => (
                  <div
                    className={`workflow-step ${index === 1 ? "workflow-active" : index === 0 ? "workflow-done" : ""}`}
                    key={step}
                  >
                    <span>{index + 1}</span>
                    <div>
                      <strong>{step}</strong>
                      <small>
                        {index === 0
                          ? "Prepared by GPT"
                          : index === 1
                            ? "Claude is writing code..."
                            : "Waiting for next gate"}
                      </small>
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>
          <div className="cockpit-bottom">
            <div>
              <SectionHeader
                title="Recent activity"
                detail="Latest project events"
              />
              <div className="compact-activity">
                {isTauriDesktop() ? (
                  <div className="rail-empty">
                    No project activity evidence yet.
                  </div>
                ) : (
                  activity
                    .slice(0, 3)
                    .map((item) => <ActivityRow key={item.id} {...item} />)
                )}
              </div>
            </div>
            <div>
              <SectionHeader title="Project metrics" detail="Current signal" />
              <div className="metric-mini-grid">
                <span>
                  <b>{isTauriDesktop() ? "—" : "23 / 28"}</b>Tasks
                </span>
                <span>
                  <b>{isTauriDesktop() ? "—" : "A"}</b>Code quality
                </span>
                <span>
                  <b>{isTauriDesktop() ? "—" : "92%"}</b>Coverage
                </span>
                <span>
                  <b>{isTauriDesktop() ? "—" : "Good"}</b>Performance
                </span>
              </div>
            </div>
          </div>
        </section>
        <aside className="command-right-rail">
          <section className="right-panel brief-compact">
            <SectionHeader title="AI Engineering Brief" detail="Today" />
            <div className="brief-line">
              <ShieldCheck size={15} />
              <strong>
                {isTauriDesktop() ? records.length : 3} projects registered
              </strong>
            </div>
            <div className="brief-line">
              <ShieldCheck size={15} />
              <strong>
                {isTauriDesktop()
                  ? "Task evidence unavailable"
                  : "12 tasks completed"}
              </strong>
            </div>
            <div className="brief-line attention-line">
              <ShieldCheck size={15} />
              <strong>
                {isTauriDesktop()
                  ? records.filter((record) => record.status !== "ACTIVE")
                      .length
                  : 2}{" "}
                projects need attention
              </strong>
            </div>
            <button
              className="right-action"
              type="button"
              onClick={() => navigate("/audits")}
            >
              Review audit findings <ArrowUpRight size={13} />
            </button>
          </section>
          <section className="right-panel assistant-compact">
            <SectionHeader title="AI Assistant" detail="GPT-4o" />
            <div className="assistant-message">
              Hello! How can I help you today?
            </div>
            <button
              type="button"
              onClick={() => setNotice("Available in a later milestone.")}
            >
              What is the status of this project?
            </button>
            <button
              aria-label={
                isTauriDesktop()
                  ? "Show projects needing attention"
                  : "Show projects needing attention"
              }
              type="button"
              onClick={() => setNotice("Available in a later milestone.")}
            >
              Show projects needing attention
            </button>
          </section>
          <section className="right-panel system-compact">
            <SectionHeader title="System Status" detail="Operational" />
            <div className="system-row">
              <span>Runtime</span>
              <b>Operational</b>
            </div>
            <div className="system-row">
              <span>Database</span>
              <b>Schema v7</b>
            </div>
            <div className="system-row">
              <span>Filesystem</span>
              <b>Watching</b>
            </div>
            <div className="system-row">
              <span>Git engine</span>
              <b>Read-only</b>
            </div>
            <details className="system-detail">
              <summary>Detailed health</summary>
              <RuntimeStatusPanel />
              <DatabaseStatusPanel />
              <WatcherStatusPanel />
            </details>
          </section>
        </aside>
      </div>
    </div>
  );
}

export function CommandCenter() {
  return <CommandCenterLive />;
}

function AttentionCard({
  item,
  onAction,
}: {
  item: (typeof attention)[number];
  onAction: () => void;
}) {
  return (
    <article className="attention-card">
      <div className={`attention-icon attention-${item.icon}`}>
        <ShieldCheck size={16} />
      </div>
      <div>
        <StatusBadge state={item.state} />
        <h3>{item.project}</h3>
        <p>{item.detail}</p>
      </div>
      <button
        type="button"
        className="icon-button"
        onClick={onAction}
        aria-label={`Open ${item.project}`}
      >
        <ChevronRight size={17} />
      </button>
    </article>
  );
}

function WorkQueue() {
  return (
    <div className="queue-table" role="table" aria-label="Active work queue">
      <div className="queue-row queue-head" role="row">
        <span>Project</span>
        <span>Task</span>
        <span>Stage</span>
        <span>Actor</span>
        <span>State</span>
        <span>Updated</span>
      </div>
      {queue.map((item) => (
        <div className="queue-row" role="row" key={item.project}>
          <strong>{item.project}</strong>
          <span>{item.task}</span>
          <span>{item.stage}</span>
          <ActorBadge actor={item.actor} />
          <StatusBadge state={item.state} />
          <time>{item.updated}</time>
        </div>
      ))}
    </div>
  );
}

export function Projects() {
  const navigate = useNavigate();
  const { refresh: refreshRegistry, selectProject } = useProjectRegistry();
  const [records, setRecords] = React.useState<ProjectRecord[]>([]);
  const [search, setSearch] = React.useState("");
  const [status, setStatus] = React.useState("");
  const [sort, setSort] = React.useState<"name" | "priority" | "updated">(
    "name",
  );
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);
  const [dialog, setDialog] = React.useState<"register" | "repair" | null>(
    null,
  );
  const [selected, setSelected] = React.useState<ProjectRecord | null>(null);
  const [refresh, setRefresh] = React.useState(0);
  const load = React.useCallback(() => {
    if (!isTauriDesktop()) {
      setLoading(false);
      setError(
        "Native project registry is unavailable in browser preview. Open the Tauri desktop app to manage registered projects.",
      );
      return;
    }
    setLoading(true);
    void listRegisteredProjects({
      search,
      status: status ? (status as ProjectRecord["status"]) : null,
      sort,
      includeArchived: status === "ARCHIVED",
    })
      .then((value) => {
        setRecords(value);
        setError(null);
      })
      .catch((caught) =>
        setError(caught instanceof Error ? caught.message : String(caught)),
      )
      .finally(() => setLoading(false));
  }, [refresh, search, sort, status]);
  React.useEffect(() => {
    load();
  }, [load]);
  const act = async (operation: () => Promise<unknown>) => {
    try {
      await operation();
      await refreshRegistry();
      setRefresh((value) => value + 1);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    }
  };
  return (
    <>
      <PageHeader
        eyebrow="Project registry"
        title="Projects"
        description="Explicitly registered local folders and read-only repository metadata."
        action={
          <button
            className="primary-button"
            type="button"
            onClick={() => {
              setSelected(null);
              setDialog("register");
            }}
          >
            <Plus size={16} />
            Add project
          </button>
        }
      />
      <div className="registry-layout">
        <section className="registry-main">
          <div className="registry-toolbar">
            <label className="registry-search">
              <Search size={15} />
              <input
                value={search}
                onChange={(event) => setSearch(event.target.value)}
                placeholder="Search registered projects"
                aria-label="Search registered projects"
              />
            </label>
            <select
              value={status}
              onChange={(event) => setStatus(event.target.value)}
              aria-label="Filter project status"
            >
              <option value="">Active and missing</option>
              <option value="ACTIVE">Active</option>
              <option value="MISSING">Missing paths</option>
              <option value="ARCHIVED">Archived</option>
            </select>
            <select
              value={sort}
              onChange={(event) => setSort(event.target.value as typeof sort)}
              aria-label="Sort projects"
            >
              <option value="name">Sort by name</option>
              <option value="priority">Sort by priority</option>
              <option value="updated">Sort by validation</option>
            </select>
          </div>
          {error && isTauriDesktop() ? (
            <div className="safe-notice" role="alert">
              {error}
            </div>
          ) : null}
          {loading ? (
            <LoadingState />
          ) : error && !records.length ? (
            <ErrorState detail={error} />
          ) : records.length ? (
            <div className="registry-grid">
              {records.map((project) => (
                <ProjectRegistryCard
                  key={project.id}
                  project={project}
                  onOpen={() => {
                    selectProject(project.id);
                    navigate(`/projects/${project.id}`);
                  }}
                  onArchive={() => {
                    if (
                      window.confirm(
                        `Archive ${project.name} from active registry views?`,
                      )
                    )
                      void act(() => archiveProject(project.id));
                  }}
                  onRemove={() => {
                    if (
                      window.confirm(
                        `Remove ${project.name} from H!veAI registry? The folder will not be deleted.`,
                      )
                    )
                      void act(() => removeProject(project.id));
                  }}
                  onRepair={() => {
                    const path = window.prompt(
                      "Enter the moved project folder path",
                      project.originalPath,
                    );
                    if (path) {
                      void act(() => repairProjectPath(project.id, path));
                    }
                  }}
                  onPriority={(priority) =>
                    void act(() => updateProjectSettings(project.id, priority))
                  }
                />
              ))}
            </div>
          ) : (
            <EmptyState
              title="No registered projects"
              detail={
                isTauriDesktop()
                  ? "Use Add project to explicitly register an existing local folder."
                  : "Open the Tauri desktop app to load the local project registry."
              }
            />
          )}
        </section>
      </div>
      {dialog ? (
        <ProjectRegistryDialog
          mode={dialog}
          project={selected}
          onClose={() => setDialog(null)}
          onSubmit={async (path, name) => {
            if (dialog === "register")
              await act(() => registerProject(path, name));
            else if (selected)
              await act(() => repairProjectPath(selected.id, path));
            setDialog(null);
          }}
        />
      ) : null}
    </>
  );
}

function ProjectRegistryDialog({
  mode,
  project,
  onClose,
  onSubmit,
}: {
  mode: "register" | "repair";
  project: ProjectRecord | null;
  onClose: () => void;
  onSubmit: (path: string, name: string) => Promise<void>;
}) {
  const [path, setPath] = React.useState(project?.originalPath ?? "");
  const [name, setName] = React.useState(project?.name ?? "");
  const [submitting, setSubmitting] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const submit = async (event: React.FormEvent) => {
    event.preventDefault();
    setSubmitting(true);
    setError(null);
    try {
      await onSubmit(path, name);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setSubmitting(false);
    }
  };
  return (
    <div className="modal-backdrop" role="presentation" onMouseDown={onClose}>
      <div
        className="registry-dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby="registry-dialog-title"
        onMouseDown={(event) => event.stopPropagation()}
      >
        <div className="registry-dialog-head">
          <div>
            <span className="eyebrow">
              {mode === "register" ? "Explicit registration" : "Path repair"}
            </span>
            <h2 id="registry-dialog-title">
              {mode === "register"
                ? "Add existing project"
                : "Repair project path"}
            </h2>
          </div>
          <button
            className="icon-button"
            type="button"
            onClick={onClose}
            aria-label="Close project dialog"
          >
            <X size={17} />
          </button>
        </div>
        <p className="registry-dialog-copy">
          {mode === "register"
            ? "Choose a folder. H!veAI will read metadata only and will not create or modify files there."
            : "Choose the moved folder. H!veAI validates identity before updating the registry path."}
        </p>
        {mode === "register" ? (
          <section
            className="registry-dialog-boundary"
            aria-labelledby="registry-boundary-title"
          >
            <div className="registry-dialog-boundary-head">
              <span className="side-icon">
                <FolderKanban size={16} />
              </span>
              <div>
                <span className="eyebrow">Registry Boundary</span>
                <strong id="registry-boundary-title">Read-only by design</strong>
              </div>
            </div>
            <p>
              H!veAI records project identity and cached Git metadata without
              changing the selected folder.
            </p>
            <div className="registry-dialog-rule">
              <Check size={14} />
              <span>Explicit user action required</span>
            </div>
            <div className="registry-dialog-rule">
              <ShieldCheck size={14} />
              <span>No branch, file, or remote mutation</span>
            </div>
            <div className="registry-dialog-rule">
              <GitBranch size={14} />
              <span>Live Git metadata and status available</span>
            </div>
          </section>
        ) : null}
        <form onSubmit={submit}>
          <label className="field-label">
            Folder path
            <input
              autoFocus
              required
              value={path}
              onChange={(event) => setPath(event.target.value)}
              placeholder="C:\\Users\\you\\Projects\\Example"
            />
          </label>
          {mode === "register" ? (
            <label className="field-label">
              Display name <span>optional</span>
              <input
                value={name}
                onChange={(event) => setName(event.target.value)}
                placeholder="Uses folder name when empty"
              />
            </label>
          ) : null}
          {error ? (
            <div className="safe-notice" role="alert">
              {error}
            </div>
          ) : null}
          <div className="registry-dialog-actions">
            <button
              className="secondary-button"
              type="button"
              onClick={onClose}
            >
              Cancel
            </button>
            <button
              className="primary-button"
              type="submit"
              disabled={submitting}
            >
              {submitting
                ? "Checking..."
                : mode === "register"
                  ? "Register folder"
                  : "Repair path"}
              <ArrowUpRight size={15} />
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export type CockpitLoadFailureKind =
  | "UNKNOWN_PROJECT"
  | "REGISTERED_PROJECT_UNAVAILABLE"
  | "COCKPIT_SNAPSHOT_FAILED";

export function classifyCockpitLoadFailure(
  stage: "registry" | "snapshot",
  project: ProjectRecord | null,
): CockpitLoadFailureKind {
  if (stage === "registry") return "UNKNOWN_PROJECT";
  if (project && project.status !== "ACTIVE") {
    return "REGISTERED_PROJECT_UNAVAILABLE";
  }
  return "COCKPIT_SNAPSHOT_FAILED";
}

function cockpitRouteErrorDetail(
  kind: CockpitLoadFailureKind,
  project: ProjectRecord | null,
) {
  if (kind === "UNKNOWN_PROJECT") return "This project ID is not registered.";
  if (kind === "REGISTERED_PROJECT_UNAVAILABLE") {
    return `Registered project ${project?.name ?? "identity"} is ${project?.status ?? "unavailable"} and its cockpit cannot be loaded.`;
  }
  return "The registered project was found, but its native cockpit snapshot failed to load.";
}

export function ProjectCockpit() {
  const { id } = useParams();
  const { selectProject } = useProjectRegistry();
  const project = projects.find((item) => item.id === id);
  const [snapshot, setSnapshot] = React.useState<ProjectCockpitSnapshot | null>(null);
  const [registeredProject, setRegisteredProject] = React.useState<ProjectRecord | null>(null);
  const [routeError, setRouteError] = React.useState<CockpitLoadFailureKind | null>(null);
  const [routeState, setRouteState] = React.useState<
    "loading" | "ready" | "not-found" | "error"
  >(isTauriDesktop() ? "loading" : project ? "ready" : "not-found");
  const [tab, setTab] = React.useState("Overview");
  const tabs = [
    "Overview",
    "Tasks",
    "Workflow",
    "Agents",
    "Audit",
    "Git",
    "Tests",
    "Activity",
    "Files",
    "Settings",
  ];
  const requestId = React.useRef(0);
  React.useEffect(() => {
    const currentRequest = ++requestId.current;
    setTab("Overview");
    setSnapshot(null);
    setRegisteredProject(null);
    setRouteError(null);
    if (!id || !isTauriDesktop()) {
      setRouteState(project ? "ready" : "not-found");
      return;
    }
    setRouteState("loading");
    let stage: "registry" | "snapshot" = "registry";
    let registeredForRequest: ProjectRecord | null = null;
    void getRegisteredProject(id)
      .then((registered) => {
        registeredForRequest = registered;
        if (currentRequest === requestId.current) setRegisteredProject(registered);
        stage = "snapshot";
        return getProjectCockpitSnapshot(id);
      })
      .then((value) => {
        if (currentRequest === requestId.current) {
          setSnapshot(value);
          selectProject(value.project.id);
          setRouteState("ready");
        }
      })
      .catch(() => {
        if (currentRequest === requestId.current) {
          setRouteError(classifyCockpitLoadFailure(stage, registeredForRequest));
          setRouteState("error");
        }
      });
  }, [id, project, selectProject]);
  if (snapshot)
    return <LiveProjectCockpit snapshot={snapshot} onRefresh={() => {
      if (id) void getProjectCockpitSnapshot(id).then(setSnapshot).catch(() => undefined);
    }} />;
  if (isTauriDesktop())
    return (
      <div className="cockpit-route-state">
        {routeState === "error" ? (
          <ErrorState
            detail={cockpitRouteErrorDetail(
              routeError ?? "COCKPIT_SNAPSHOT_FAILED",
              registeredProject,
            )}
          />
        ) : (
          <>
            <LoadingState />
            <span>Resolving registered project identity...</span>
          </>
        )}
      </div>
    );
  if (!project)
    return (
      <div className="cockpit-route-state">
        <ErrorState detail="This preview project does not exist." />
      </div>
    );
  return (
    <>
      <Link className="back-link" to="/projects">
        <ArrowLeft size={15} />
        Back to projects
      </Link>
      <div className="cockpit-header">
        <div className="project-mark project-mark-large">{project.code}</div>
        <div>
          <p className="eyebrow">Project cockpit · registry preview</p>
          <h1>{project.name}</h1>
          <p>{project.description}</p>
        </div>
        <div className="cockpit-actions">
          <StatusBadge state={project.state} />
          <button
            className="icon-button"
            type="button"
            aria-label="Project options"
          >
            <MoreHorizontal size={18} />
          </button>
        </div>
      </div>
      <nav className="tabs" aria-label="Project cockpit tabs">
        {tabs.map((item) => (
          <button
            className={tab === item ? "tab-active" : ""}
            type="button"
            key={item}
            onClick={() => setTab(item)}
          >
            {item}
          </button>
        ))}
      </nav>
      {tab === "Overview" ? (
        <CockpitOverview project={project} />
      ) : (
        <div className="cockpit-placeholder">
          <EmptyState
            title={`${tab} view is staged`}
            detail="This tab is a polished placeholder. Runtime data arrives in later milestones."
          />
        </div>
      )}
    </>
  );
}

function LiveProjectCockpit({
  snapshot,
  onRefresh,
}: {
  snapshot: ProjectCockpitSnapshot;
  onRefresh: () => void;
}) {
  const navigate = useNavigate();
  const [tab, setTab] = React.useState("Overview");
  const [priority, setPriority] = React.useState(String(snapshot.project.priority));
  const [settingsMessage, setSettingsMessage] = React.useState<string | null>(null);
  const [settingsBusy, setSettingsBusy] = React.useState(false);
  const tabs = ["Overview", "Tasks", "Workflow", "Agents", "Audit", "Git", "Tests", "Activity", "Files", "Settings"];
  const summary = snapshot.projectSummary;
  const materialized = snapshot.dashboard.materialized;
  const savePriority = async () => {
    const nextPriority = Number(priority);
    if (!Number.isInteger(nextPriority) || nextPriority < 0 || nextPriority > 2) {
      setSettingsMessage("Priority must be Normal, High, or Critical.");
      return;
    }
    setSettingsBusy(true);
    setSettingsMessage(null);
    try {
      await updateProjectSettings(snapshot.project.id, nextPriority);
      setSettingsMessage("Registry settings saved.");
      onRefresh();
    } catch (caught) {
      setSettingsMessage(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setSettingsBusy(false);
    }
  };
  const repair = () => {
    const path = window.prompt("Enter the moved project folder path", snapshot.project.originalPath);
    if (!path) return;
    setSettingsBusy(true);
    void repairProjectPath(snapshot.project.id, path)
      .then(() => { setSettingsMessage("Project path repaired."); onRefresh(); })
      .catch((caught) => setSettingsMessage(caught instanceof Error ? caught.message : String(caught)))
      .finally(() => setSettingsBusy(false));
  };
  const archive = () => {
    if (!window.confirm(`Archive ${snapshot.project.name} from active registry views?`)) return;
    setSettingsBusy(true);
    void archiveProject(snapshot.project.id)
      .then(() => navigate("/projects"))
      .catch((caught) => setSettingsMessage(caught instanceof Error ? caught.message : String(caught)))
      .finally(() => setSettingsBusy(false));
  };
  const remove = () => {
    if (!window.confirm(`Remove ${snapshot.project.name} from H!veAI registry? The folder will not be deleted.`)) return;
    setSettingsBusy(true);
    void removeProject(snapshot.project.id)
      .then(() => navigate("/projects"))
      .catch((caught) => setSettingsMessage(caught instanceof Error ? caught.message : String(caught)))
      .finally(() => setSettingsBusy(false));
  };
  return (
    <div className="project-cockpit-live">
      <Link className="back-link" to="/projects"><ArrowLeft size={15} />Back to projects</Link>
      <div className="cockpit-header">
        <div className="project-mark project-mark-large">{snapshot.project.name.slice(0, 2).toUpperCase()}</div>
        <div className="cockpit-identity">
          <p className="eyebrow">Project cockpit / live registered project</p>
          <h1>{snapshot.project.name}</h1>
          <p>{snapshot.project.originalPath}</p>
        </div>
        <div className="cockpit-actions">
          <span className={`registry-status registry-status-${snapshot.project.status.toLowerCase()}`}>{snapshot.project.status}</span>
          <button className="icon-button" type="button" onClick={onRefresh} aria-label="Refresh project cockpit" title="Refresh project cockpit"><RefreshCw size={16} /></button>
        </div>
      </div>
      <nav className="tabs project-cockpit-tabs" aria-label="Project cockpit tabs">
        {tabs.map((item) => <button key={item} type="button" className={tab === item ? "tab-active" : ""} onClick={() => setTab(item)}>{item}</button>)}
      </nav>
      {snapshot.warnings.length ? <div className="safe-notice cockpit-notice" role="status">{snapshot.warnings.slice(0, 3).join(" | ")}</div> : null}
      {tab === "Overview" ? <CockpitLiveOverview snapshot={snapshot} /> : null}
      {tab === "Tasks" ? <CockpitLiveTasks snapshot={snapshot} /> : null}
      {tab === "Workflow" ? <CockpitLiveWorkflow snapshot={snapshot} onRefresh={onRefresh} /> : null}
      {tab === "Agents" ? <CockpitLiveAgents snapshot={snapshot} /> : null}
      {tab === "Audit" ? <CockpitLiveAudits snapshot={snapshot} /> : null}
      {tab === "Git" ? <CockpitLiveGit snapshot={snapshot} /> : null}
      {tab === "Tests" ? <CockpitLiveTests snapshot={snapshot} /> : null}
      {tab === "Activity" ? <CockpitLiveActivity snapshot={snapshot} /> : null}
      {tab === "Files" ? <CockpitLiveFiles snapshot={snapshot} /> : null}
      {tab === "Settings" ? <CockpitLiveSettings snapshot={snapshot} priority={priority} setPriority={setPriority} busy={settingsBusy} message={settingsMessage} onSave={savePriority} onRepair={repair} onArchive={archive} onRemove={remove} /> : null}
      <div className="cockpit-provenance-footer">Snapshot generated {snapshot.generatedAt} / Project-scoped native read model</div>
    </div>
  );
}

function CockpitLiveOverview({ snapshot }: { snapshot: ProjectCockpitSnapshot }) {
  const summary = snapshot.projectSummary;
  const materialized = snapshot.dashboard.materialized;
  const task = summary.currentTask;
  return (
    <>
      <section className="cockpit-live-hero">
        <div>
          <span className="eyebrow">Current task</span>
          <h2>{task?.title ?? materialized.currentTaskTitle ?? "Current task unavailable"}</h2>
          <p>{task ? `Where we are: ${task.parsedStatus}. Source: ${task.sourcePath}.` : "No authoritative current task evidence is available for this project."}</p>
          {summary.currentState ? <span className="cockpit-state-chip">{formatCockpitState(summary.currentState)}</span> : null}
        </div>
        <div className="cockpit-live-progress">
          <span>Milestone progress</span>
          <strong>{summary.progressPercent == null ? "Unknown" : `${summary.progressPercent}%`}</strong>
          {summary.progressPercent != null ? <ProgressIndicator value={summary.progressPercent} /> : <span className="unknown-value">Progress unavailable</span>}
          <span>Health <b className={`health-${summary.health.toLowerCase()}`}>{summary.health}</b></span>
        </div>
      </section>
      <div className="cockpit-live-grid">
        <CockpitPanel title="Project identity" detail="Project Registry authority">
          <CockpitFacts facts={[
            ["Status", snapshot.project.status], ["Path", snapshot.project.originalPath],
            ["Repository", snapshot.project.repository?.githubOwner && snapshot.project.repository.githubRepo ? `${snapshot.project.repository.githubOwner}/${snapshot.project.repository.githubRepo}` : "Unavailable"],
            ["Current milestone", materialized.currentMilestone ?? "Unknown"],
            ["Required actor", summary.allowedActors.join(", ") || materialized.requiredActor || "Unknown"],
          ]} />
        </CockpitPanel>
        <CockpitPanel title="Next action" detail="Existing workflow/dashboard evidence">
          <div className="cockpit-callout"><strong>{summary.nextAction ?? materialized.nextAction ?? "Next action unavailable"}</strong><span>{materialized.waitingOn ? `Waiting on: ${materialized.waitingOn}` : "No verified waiting fact"}</span></div>
        </CockpitPanel>
      </div>
      <div className="cockpit-live-grid">
        <CockpitPanel title="Last completed action" detail="M10 event history when available">
          <div className="cockpit-callout"><strong>{summary.lastAction?.summary ?? "Last completed action unavailable"}</strong><span>{summary.lastAction ? `${summary.lastAction.occurredAt} / ${summary.lastAction.actor ?? "Actor unknown"}` : "No verified event"}</span></div>
        </CockpitPanel>
        <CockpitPanel title="Authority and provenance" detail="Resolved Project Dashboard contract">
          <CockpitFacts facts={[["Manifest", snapshot.dashboard.manifestStatus], ["Task authority", snapshot.dashboard.taskAuthority], ["Provenance", snapshot.dashboard.provenanceMode], ["Canonical task", snapshot.dashboard.canonicalTaskSource ?? "Unavailable"]]} />
        </CockpitPanel>
      </div>
      <CockpitPanel title="Project Dashboard status" detail="Materialized values are evidence, not stronger than M10">
        <CockpitFacts facts={[["Project status", materialized.projectStatus ?? "Unknown"], ["Declared workflow", materialized.declaredWorkflowState ?? "Unknown"], ["Health", materialized.health ?? "Unknown"], ["Last meaningful update", materialized.lastMeaningfulUpdate ?? "Unknown"]]} />
      </CockpitPanel>
      <CockpitPanel title="Dashboard operational evidence" detail="Current work, waits, blockers, and quality remain provenance-bound">
        <CockpitList title="Current work" values={materialized.currentWork.map((item) => `${item.item} / ${item.status} / ${item.ownerActor}`)} empty="No materialized current-work fact" />
        <CockpitList title="Blockers and waiting" values={materialized.blockersWaiting} empty="No verified blocker or waiting fact" />
        <CockpitList title="Quality verification" values={materialized.qualityVerification.map((item) => `${item.label}: ${item.value}`)} empty="No materialized quality fact" />
      </CockpitPanel>
    </>
  );
}

function CockpitLiveTasks({ snapshot }: { snapshot: ProjectCockpitSnapshot }) {
  const tasks = snapshot.taskIntelligence?.tasks ?? [];
  const workflowById = new Map(snapshot.workflow.tasks.map((task) => [task.taskId, task]));
  return <>
    <CockpitPanel title="Canonical tasks" detail={snapshot.taskIntelligence ? `${tasks.length} persisted parsed task(s)` : "Task intelligence unavailable"}>
      {snapshot.taskIntelligenceError ? <div className="safe-notice">Unknown: {snapshot.taskIntelligenceError}</div> : null}
      <div className="cockpit-record-list">{tasks.map((task) => { const workflow = workflowById.get(task.id); return <details className="cockpit-record" key={task.id}><summary><strong>{task.title}</strong><span>{formatCockpitState(workflow?.currentState ?? task.parsedStatus)}</span></summary><CockpitFacts facts={[["Task ID", task.id], ["Status", task.parsedStatus], ["Workflow", workflow?.currentState ?? "Unknown"], ["Source", task.sourcePath], ["Required actor", task.requiredActor ?? "Unknown"], ["Evidence", `${task.evidence.startLine}-${task.evidence.endLine}`]]} /><CockpitList title="Dependencies" values={task.dependencyReferences} empty="No declared dependencies" /><CockpitList title="Blockers" values={task.blockers} empty="No declared blockers" /><CockpitList title="Acceptance criteria" values={task.acceptanceCriteria} empty="No acceptance criteria recorded" /></details>; })}</div>
      {!tasks.length && !snapshot.taskIntelligenceError ? <EmptyState title="No parsed tasks" detail="The selected project's persisted task intelligence contains no tasks." /> : null}
    </CockpitPanel>
    <div className="cockpit-live-grid"><CockpitPanel title="Handoff" detail="M09 structured handoff evidence"><CockpitList title="Current" values={snapshot.taskIntelligence?.handoff?.current ?? []} empty="Unknown" /><CockpitList title="Next" values={snapshot.taskIntelligence?.handoff?.next ?? []} empty="Unknown" /><CockpitList title="Waiting" values={snapshot.taskIntelligence?.handoff?.waiting ?? []} empty="No verified wait" /></CockpitPanel><CockpitPanel title="Task authority" detail="Project Dashboard / M08 / M09"><CockpitFacts facts={[["Authority", snapshot.dashboard.taskAuthority], ["Canonical source", snapshot.dashboard.canonicalTaskSource ?? "Unavailable"], ["Duplicate policy", "Canonical task and workflow evidence take precedence"]]} /></CockpitPanel></div>
  </>;
}

function CockpitLiveWorkflow({ snapshot, onRefresh }: { snapshot: ProjectCockpitSnapshot; onRefresh: () => void }) {
  const current = snapshot.projectSummary.currentState ?? snapshot.dashboard.materialized.declaredWorkflowState;
  const [selectedTaskId, setSelectedTaskId] = React.useState(snapshot.workflow.tasks[0]?.taskId ?? "");
  const [targetState, setTargetState] = React.useState<WorkflowState | "">(snapshot.workflow.tasks[0]?.allowedNextStates[0] ?? "");
  const [rationale, setRationale] = React.useState("");
  const [evidenceReference, setEvidenceReference] = React.useState("");
  const [correctionMessage, setCorrectionMessage] = React.useState<string | null>(null);
  const [correctionBusy, setCorrectionBusy] = React.useState(false);
  const selectedTask = snapshot.workflow.tasks.find((task) => task.taskId === selectedTaskId) ?? snapshot.workflow.tasks[0];
  const submitCorrection = async () => {
    if (!selectedTask || !targetState || !rationale.trim() || !evidenceReference.trim()) {
      setCorrectionMessage("Select a target state, provide a correction rationale, and cite evidence.");
      return;
    }
    setCorrectionBusy(true);
    setCorrectionMessage(null);
    try {
      await overrideWorkflow({
        taskId: selectedTask.taskId,
        expectedFromState: selectedTask.currentState,
        toState: targetState,
        requestId: `cockpit-correction-${Date.now()}`,
        rationale: rationale.trim(),
        evidenceRefs: [{ kind: "EXTERNAL_REFERENCE", id: evidenceReference.trim() }],
      });
      setRationale("");
      setEvidenceReference("");
      setCorrectionMessage("Correction recorded as a workflow event.");
      onRefresh();
    } catch (caught) {
      setCorrectionMessage(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setCorrectionBusy(false);
    }
  };
  return <>
    <CockpitPanel title="Workflow pipeline" detail="M10 canonical state and transition evidence"><div className="cockpit-pipeline"><span className="cockpit-pipeline-current">{current ? formatCockpitState(current) : "Workflow state unknown"}</span>{snapshot.projectSummary.allowedActors.length ? <span>Allowed actors: {snapshot.projectSummary.allowedActors.join(", ")}</span> : <span>Allowed actors unknown</span>}</div><div className="cockpit-record-list">{snapshot.workflow.tasks.map((task) => <div className="cockpit-record" key={task.taskId}><div className="cockpit-record-heading"><strong>{task.title}</strong><span>{formatCockpitState(task.currentState)}</span></div><CockpitFacts facts={[["Task ID", task.taskId], ["Required actor", task.requiredActor ?? "Unknown"], ["Next states", task.allowedNextStates.map(formatCockpitState).join(", ") || "None"], ["Workflow managed", task.workflowManaged ? "Yes" : "No"]]} /></div>)}</div></CockpitPanel>
    <CockpitPanel title="Transition history" detail="Durable task_events; historical rows are preserved"><div className="cockpit-record-list">{snapshot.workflowHistory.map((event) => <div className="cockpit-record" key={event.id}><div className="cockpit-record-heading"><strong>{event.summary}</strong><span>{event.occurredAt}</span></div><CockpitFacts facts={[["Transition", `${event.fromState ?? "Initial"} -> ${event.toState ?? "Unknown"}`], ["Actor", event.actorType ?? "Unknown"], ["Evidence", event.evidenceRefs.map((ref) => `${ref.kind}:${ref.id}`).join(", ") || "None recorded"]]} /></div>)}{!snapshot.workflowHistory.length ? <EmptyState title="No workflow history" detail="No persisted M10 workflow events are available for this project." /> : null}</div></CockpitPanel>
    <CockpitPanel title="Manual correction" detail="Explicit M10 override; rationale and evidence are required"><div className="cockpit-correction-form">{selectedTask ? <><label>Task<select value={selectedTaskId || selectedTask.taskId} onChange={(event) => { const next = snapshot.workflow.tasks.find((task) => task.taskId === event.target.value); setSelectedTaskId(event.target.value); setTargetState(next?.allowedNextStates[0] ?? ""); }} disabled={correctionBusy}>{snapshot.workflow.tasks.map((task) => <option key={task.taskId} value={task.taskId}>{task.title}</option>)}</select></label><label>Target state<select value={targetState} onChange={(event) => setTargetState(event.target.value as WorkflowState)} disabled={correctionBusy}><option value="">Select a state</option>{selectedTask.allowedNextStates.map((state) => <option key={state} value={state}>{formatCockpitState(state)}</option>)}</select></label><label>Rationale<textarea value={rationale} onChange={(event) => setRationale(event.target.value)} disabled={correctionBusy} rows={3} placeholder="Explain the correction." /></label><label>Evidence reference<input value={evidenceReference} onChange={(event) => setEvidenceReference(event.target.value)} disabled={correctionBusy} placeholder="Audit, test, decision, or external reference ID" /></label><button className="secondary-button" type="button" onClick={() => void submitCorrection()} disabled={correctionBusy}><ShieldCheck size={15} />Record correction</button></> : <div className="safe-notice">No workflow-managed task is available; no correction write path is offered.</div>}{correctionMessage ? <div className="safe-notice" role="status">{correctionMessage}</div> : null}</div></CockpitPanel>
  </>;
}

function CockpitLiveAgents({ snapshot }: { snapshot: ProjectCockpitSnapshot }) { return <><CockpitPanel title="Agent sessions" detail="Persisted sessions only; M13/M14 providers are not started here"><div className="cockpit-record-list">{snapshot.agentSessions.map((session) => <div className="cockpit-record" key={session.id}><div className="cockpit-record-heading"><strong>{session.provider}</strong><span>{session.state}</span></div><CockpitFacts facts={[["Session", session.id], ["Task", session.taskId ?? "Freeform / unknown"], ["Started", session.startedAt ?? "Unknown"], ["Ended", session.endedAt ?? "Still open or unknown"]]} /></div>)}{!snapshot.agentSessions.length ? <EmptyState title="No agent sessions" detail="No persisted project-scoped agent session evidence is available." /> : null}</div></CockpitPanel><CockpitPanel title="Permission and wait state" detail="Persisted permission requests only"><div className="cockpit-record-list">{snapshot.permissions.map((permission) => <div className="cockpit-record" key={permission.id}><div className="cockpit-record-heading"><strong>{permission.permissionKind}</strong><span>{permission.state}</span></div><CockpitFacts facts={[["Request", permission.id], ["Session", permission.sessionId ?? "Unknown"], ["Resource", permission.requestedResource ?? "Unknown"], ["Decided by", permission.decidedBy ?? "Unknown"], ["Created", permission.createdAt]]} /></div>)}{!snapshot.permissions.length ? <EmptyState title="No permission or wait evidence" detail="No persisted project-scoped permission requests are available." /> : null}</div></CockpitPanel></>; }

function CockpitLiveAudits({ snapshot }: { snapshot: ProjectCockpitSnapshot }) { return <CockpitPanel title="Audit history" detail="Historical results remain visible"><div className="cockpit-record-list">{snapshot.audits.map((audit) => <details className="cockpit-record" key={audit.id}><summary><strong>{audit.summary ?? `Audit ${audit.result}`}</strong><span>{audit.result}</span></summary><CockpitFacts facts={[["Audit", audit.id], ["Task", audit.taskId ?? "Unknown"], ["Created", audit.createdAt], ["Confidence", audit.confidence == null ? "Unknown" : String(audit.confidence)]]} /><div className="cockpit-finding-list">{audit.findings.map((finding) => <div key={finding.id}><strong>{finding.severity}: {finding.title}</strong><span>{finding.detail ?? "No finding detail"}{finding.filePath ? ` / ${finding.filePath}:${finding.lineNumber ?? "?"}` : ""}</span></div>)}</div></details>)}{!snapshot.audits.length ? <EmptyState title="No audit evidence" detail="No persisted project-scoped audits are available." /> : null}</div></CockpitPanel>; }

function CockpitLiveGit({ snapshot }: { snapshot: ProjectCockpitSnapshot }) { const git = snapshot.git; return <><CockpitPanel title="Git visibility" detail="Read-only local Git Engine snapshot">{snapshot.gitError ? <div className="safe-notice">Unknown or unavailable: {snapshot.gitError}</div> : null}{git ? <><CockpitFacts facts={[["Health", git.health], ["Branch", git.currentBranch ?? (git.detachedHead ? "Detached HEAD" : "Unknown")], ["HEAD", git.headSha ?? "Unknown"], ["Upstream", git.upstream ?? "Unavailable"], ["Ahead / behind", git.aheadCount == null || git.behindCount == null ? "Unavailable" : `${git.aheadCount} / ${git.behindCount}`], ["Snapshot", git.snapshotTimestamp]]} /><CockpitList title="Conflicts" values={git.conflictedFiles} empty="No conflicts" /><CockpitList title="Changed files" values={[...git.stagedFiles, ...git.unstagedFiles].map((file) => `${file.kind}: ${file.path}`)} empty="No staged or unstaged files" /></> : null}</CockpitPanel><CockpitPanel title="Diff evidence" detail="Bounded read-only working-tree diff">{snapshot.gitDiffError ? <div className="safe-notice">Unknown: {snapshot.gitDiffError}</div> : snapshot.gitDiff ? <pre className="cockpit-code">{snapshot.gitDiff.text || "No textual diff"}</pre> : <EmptyState title="Diff unavailable" detail="Diff evidence is unavailable for this repository state." />}</CockpitPanel></>; }

function CockpitLiveTests({ snapshot }: { snapshot: ProjectCockpitSnapshot }) { return <CockpitPanel title="Test-run history" detail="Persisted project-scoped test evidence"><div className="cockpit-record-list">{snapshot.tests.map((test) => <div className="cockpit-record" key={test.id}><div className="cockpit-record-heading"><strong>{test.command}</strong><span>{test.result}</span></div><CockpitFacts facts={[["Run", test.id], ["Task", test.taskId ?? "Unknown"], ["Started", test.startedAt], ["Finished", test.finishedAt ?? "Unknown"]]} /></div>)}{!snapshot.tests.length ? <EmptyState title="No test-run evidence" detail="No persisted tests are available for this project." /> : null}</div></CockpitPanel>; }

function CockpitLiveActivity({ snapshot }: { snapshot: ProjectCockpitSnapshot }) { return <CockpitPanel title="Project activity" detail="Bounded mixed evidence timeline"><div className="cockpit-activity-list">{snapshot.activity.map((item) => <div className="cockpit-activity" key={item.id}><time>{item.occurredAt}</time><div><strong>{item.event}</strong><span>{item.kind} / {item.actor ?? "Actor unknown"} / {item.source}</span></div></div>)}{!snapshot.activity.length ? <EmptyState title="No activity evidence" detail="No persisted meaningful activity is available." /> : null}</div></CockpitPanel>; }

function CockpitLiveFiles({ snapshot }: { snapshot: ProjectCockpitSnapshot }) { return <CockpitPanel title="Relevant files and project context" detail="Bounded M08 inventory plus resolved dashboard roles"><div className="cockpit-file-list">{snapshot.files.map((file) => <div key={`${file.role}:${file.path}`}><strong>{file.path}</strong><span>{file.role} / {file.status} / {file.sourceKind ?? "Project Dashboard"}</span><small>{file.evidence}</small></div>)}{!snapshot.files.length ? <EmptyState title="No file evidence" detail={snapshot.sourcesError ?? "No bounded source inventory is available."} /> : null}</div></CockpitPanel>; }

function CockpitLiveSettings({ snapshot, priority, setPriority, busy, message, onSave, onRepair, onArchive, onRemove }: { snapshot: ProjectCockpitSnapshot; priority: string; setPriority: (value: string) => void; busy: boolean; message: string | null; onSave: () => void; onRepair: () => void; onArchive: () => void; onRemove: () => void }) { return <><CockpitPanel title="Registry settings" detail="Explicit H!veAI registry actions"><CockpitFacts facts={[["Preferred builder", snapshot.project.preferredBuilder ?? "Unassigned"], ["Preferred auditor", snapshot.project.preferredAuditor ?? "Unassigned"], ["Task-source policy", snapshot.project.taskSourcePolicy ?? "Unknown"]]} /><label className="cockpit-setting-field">Priority<select value={priority} onChange={(event) => setPriority(event.target.value)} disabled={busy}><option value="0">Normal</option><option value="1">High</option><option value="2">Critical</option></select></label><button className="primary-button" type="button" onClick={onSave} disabled={busy}><Check size={15} />Save priority</button>{message ? <div className="safe-notice" role="status">{message}</div> : null}</CockpitPanel><CockpitPanel title="Explicit correction and lifecycle actions" detail="No project files are rewritten"><p className="cockpit-muted">Path repair, archive, and registry removal are explicit actions. They affect H!veAI registry state only and preserve the registered folder.</p><div className="cockpit-action-row"><button className="secondary-button" type="button" onClick={onRepair} disabled={busy}><RefreshCw size={15} />Repair path</button><button className="secondary-button" type="button" onClick={onArchive} disabled={busy}><ShieldCheck size={15} />Archive</button><button className="secondary-button" type="button" onClick={onRemove} disabled={busy}><X size={15} />Remove</button></div></CockpitPanel><CockpitPanel title="Manifest and source map" detail="Read-only authority evidence"><CockpitFacts facts={[["Manifest", snapshot.dashboard.manifestPath], ["Status", snapshot.dashboard.manifestStatus], ["Tracking", snapshot.dashboard.trackingMode ?? "Unknown"], ["Refresh policy", snapshot.dashboard.refreshPolicy ?? "Unknown"]]} /><CockpitList title="Warnings" values={snapshot.dashboard.warnings} empty="No manifest warnings" /></CockpitPanel></>; }

function CockpitPanel({ title, detail, children }: { title: string; detail?: string; children: React.ReactNode }) { return <section className="panel cockpit-live-panel"><SectionHeader title={title} detail={detail} />{children}</section>; }
function CockpitFacts({ facts }: { facts: Array<[string, string]> }) { return <div className="cockpit-facts">{facts.map(([label, value]) => <div key={label}><span>{label}</span><strong title={value}>{value}</strong></div>)}</div>; }
function CockpitList({ title, values, empty }: { title: string; values: string[]; empty: string }) { return <div className="cockpit-list"><span className="eyebrow">{title}</span>{values.length ? <ul>{values.map((value, index) => <li key={`${value}-${index}`}>{value}</li>)}</ul> : <span className="cockpit-muted">{empty}</span>}</div>; }
function formatCockpitState(value: string) { return value.replaceAll("_", " "); }

function RegisteredProjectCockpit({ project }: { project: ProjectRecord }) {
  const repository = project.repository;
  return (
    <>
      <Link className="back-link" to="/projects">
        <ArrowLeft size={15} />
        Back to projects
      </Link>
      <div className="cockpit-header">
        <div className="project-mark project-mark-large">
          {project.name.slice(0, 2).toUpperCase()}
        </div>
        <div>
          <p className="eyebrow">Project cockpit · registered metadata</p>
          <h1>{project.name}</h1>
          <p>{project.originalPath}</p>
        </div>
        <div className="cockpit-actions">
          <span
            className={`registry-status registry-status-${project.status.toLowerCase()}`}
          >
            {project.status}
          </span>
        </div>
      </div>
      <div className="registered-cockpit-grid">
        <section className="panel">
          <SectionHeader
            title="Registration identity"
            detail="Persisted by H!veAI Project Registry"
          />
          <div className="registered-detail-list">
            <div>
              <span>Status</span>
              <strong>{project.status}</strong>
            </div>
            <div>
              <span>Priority</span>
              <strong>
                {project.priority === 2
                  ? "Critical"
                  : project.priority === 1
                    ? "High"
                    : "Normal"}
              </strong>
            </div>
            <div>
              <span>Builder</span>
              <strong>{project.preferredBuilder ?? "Unassigned"}</strong>
            </div>
            <div>
              <span>Auditor</span>
              <strong>{project.preferredAuditor ?? "Unassigned"}</strong>
            </div>
          </div>
        </section>
        <section className="panel">
          <SectionHeader
            title="Cached Git metadata"
            detail="Read-only snapshot; live engine arrives in M06"
          />
          {repository?.isGitRepository ? (
            <div className="registered-detail-list">
              <div>
                <span>Repository root</span>
                <strong>{repository.repositoryRoot ?? "Unknown"}</strong>
              </div>
              <div>
                <span>Branch</span>
                <strong>
                  {repository.currentBranch ?? "Detached or unavailable"}
                </strong>
              </div>
              <div>
                <span>Remote</span>
                <strong>{repository.preferredRemoteUrl ?? "No remote"}</strong>
              </div>
              <div>
                <span>GitHub identity</span>
                <strong>
                  {repository.githubOwner && repository.githubRepo
                    ? `${repository.githubOwner}/${repository.githubRepo}`
                    : "Not detected"}
                </strong>
              </div>
            </div>
          ) : (
            <EmptyState
              title="Non-Git folder"
              detail="This registered folder has no Git metadata."
            />
          )}
        </section>
      </div>
    </>
  );
}

function GitStatusSurface({ projectId }: { projectId: string }) {
  const [snapshot, setSnapshot] = React.useState<GitSnapshot | null>(null);
  const [error, setError] = React.useState<string | null>(null);
  const [loading, setLoading] = React.useState(true);
  const refresh = React.useCallback(() => {
    setLoading(true);
    void getGitSnapshot(projectId)
      .then((value) => {
        setSnapshot(value);
        setError(null);
      })
      .catch((caught) =>
        setError(caught instanceof Error ? caught.message : String(caught)),
      )
      .finally(() => setLoading(false));
  }, [projectId]);
  React.useEffect(() => {
    refresh();
  }, [refresh]);
  const shortSha = snapshot?.headSha
    ? snapshot.headSha.slice(0, 7)
    : "Unavailable";
  return (
    <section className="panel git-status-surface">
      <SectionHeader
        title="Live Git status"
        detail="Read-only local snapshot"
        action={
          <button
            className="icon-button"
            type="button"
            onClick={refresh}
            aria-label="Refresh Git status"
            title="Refresh Git status"
          >
            <RefreshCw size={15} />
          </button>
        }
      />
      {loading ? (
        <LoadingState />
      ) : error ? (
        <div className="safe-notice" role="alert">
          {error}
        </div>
      ) : snapshot ? (
        <>
          <div className="git-status-header">
            <strong>
              {snapshot.currentBranch ??
                (snapshot.detachedHead ? "Detached HEAD" : "Unborn repository")}
            </strong>
            <span>{shortSha}</span>
            <span
              className={`git-health git-health-${snapshot.health.toLowerCase()}`}
            >
              {snapshot.health}
            </span>
          </div>
          <div className="git-count-grid">
            <div>
              <strong>{snapshot.stagedFiles.length}</strong>
              <span>Staged</span>
            </div>
            <div>
              <strong>{snapshot.unstagedFiles.length}</strong>
              <span>Unstaged</span>
            </div>
            <div>
              <strong>{snapshot.untrackedFiles.length}</strong>
              <span>Untracked</span>
            </div>
            <div>
              <strong>{snapshot.conflictedFiles.length}</strong>
              <span>Conflicts</span>
            </div>
            <div>
              <strong>{snapshot.aheadCount ?? "—"}</strong>
              <span>Ahead</span>
            </div>
            <div>
              <strong>{snapshot.behindCount ?? "—"}</strong>
              <span>Behind</span>
            </div>
          </div>
          <div className="git-status-footer">
            <span>
              {snapshot.upstream
                ? `Upstream ${snapshot.upstream}`
                : "Ahead/behind unavailable: no upstream configured"}
            </span>
            <span>
              {snapshot.recentCommits.length} recent commits ·{" "}
              {snapshot.worktrees.length} worktree
              {snapshot.worktrees.length === 1 ? "" : "s"}
            </span>
          </div>
          {snapshot.recentCommits[0] ? (
            <p className="git-latest-commit">
              <strong>{snapshot.recentCommits[0].sha.slice(0, 7)}</strong>{" "}
              {snapshot.recentCommits[0].subject}
            </p>
          ) : null}
        </>
      ) : null}
    </section>
  );
}

function CockpitOverview({ project }: { project: Project }) {
  return (
    <>
      <section className="cockpit-summary">
        <div>
          <span className="eyebrow">Current task</span>
          <h2>{project.task}</h2>
          <p>
            Where we are: {project.phase}. Evidence and repository snapshots
            will populate this view when the runtime foundation arrives.
          </p>
          <div className="cockpit-hero-actions">
            <PrimaryActionButton onClick={() => undefined}>
              View task detail
            </PrimaryActionButton>
            <button className="secondary-button" type="button">
              <GitBranch size={15} />
              Open branch view
            </button>
          </div>
        </div>
        <div className="cockpit-progress">
          <span>Milestone progress</span>
          <strong>{project.progress}%</strong>
          <ProgressIndicator value={project.progress} />
          <span className="health-label">
            Health{" "}
            <b className={`health-${project.health.toLowerCase()}`}>
              {project.health}
            </b>
          </span>
        </div>
      </section>
      <div className="cockpit-grid">
        <section className="panel">
          <SectionHeader
            title="Workflow pipeline"
            detail="Canonical state vocabulary"
          />
          <div className="pipeline">
            {[
              "READY_FOR_IMPLEMENTATION",
              project.state,
              "AUDIT_REQUIRED",
              "VERIFY_REQUIRED",
              "TASK_COMPLETE",
            ].map((state, index) => (
              <div
                className={`pipeline-step ${index < 2 ? "pipeline-done" : ""}`}
                key={`${state}-${index}`}
              >
                <span>{index < 1 ? <Check size={14} /> : index + 1}</span>
                <strong>{state.replaceAll("_", " ")}</strong>
                {index < 4 ? <ChevronRight size={14} /> : null}
              </div>
            ))}
          </div>
        </section>
        <section className="panel">
          <SectionHeader title="Project metrics" />
          <div className="mini-metrics">
            {project.metrics.map((metric) => (
              <div key={metric.label}>
                <span>{metric.label}</span>
                <strong>{metric.value}</strong>
              </div>
            ))}
          </div>
        </section>
      </div>
      <div className="cockpit-grid">
        <section className="panel">
          <SectionHeader title="Last completed action" />
          <div className="action-callout">
            <Check size={18} />
            <div>
              <strong>{project.lastAction}</strong>
              <span>
                Evidence will be attached by the local project registry.
              </span>
            </div>
          </div>
        </section>
        <section className="panel">
          <SectionHeader title="Next action" />
          <div className="action-callout action-next">
            <UserRound size={18} />
            <div>
              <strong>{project.nextAction}</strong>
              <span>Required actor: {project.actor}</span>
            </div>
          </div>
        </section>
      </div>
      <section className="panel">
        <SectionHeader title="Recent activity" />
        <div className="activity-list">
          {activity
            .filter((item) => item.project === project.name)
            .map((item) => (
              <ActivityRow key={item.id} {...item} />
            ))}
          <ActivityRow
            time={project.updated}
            project={project.name}
            actor={project.actor}
            event={project.lastAction}
            state={project.state}
          />
        </div>
      </section>
    </>
  );
}

export function ActivityPage() {
  return (
    <>
      <PageHeader
        title="Activity"
        description="A chronological view of workspace events and evidence placeholders."
      />
      <section className="panel activity-page">
        <SectionHeader title="Workspace event feed" detail="5 static events" />
        {activity.map((item) => (
          <ActivityRow key={item.id} {...item} />
        ))}
      </section>
    </>
  );
}
export function Tasks() {
  const desktop = isTauriDesktop();
  const { records, projects, selectedProjectId } = useProjectRegistry();
  const selected = projects.find((project) => project.id === selectedProjectId) ?? null;
  const [sources, setSources] = React.useState<DiscoveredProjectSource[]>([]);
  const [customPaths, setCustomPaths] = React.useState<CustomSourcePath[]>([]);
  const [commandProject, setCommandProject] = React.useState<CommandCenterProject | null>(null);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [path, setPath] = React.useState("");
  const [busy, setBusy] = React.useState(false);
  const request = React.useRef(0);
  const selectedProjectRef = React.useRef(selectedProjectId);

  const refresh = React.useCallback(async (projectId: string, discover = false) => {
    const requestId = ++request.current;
    setLoading(true);
    setError(null);
    try {
      const nextSources = desktop
        ? await (discover ? discoverTaskSources(projectId) : listTaskSources(projectId))
        : [];
      const nextCustom = desktop ? await listCustomSourcePaths(projectId) : [];
      const nextSnapshot = desktop ? await getCommandCenterSnapshot().catch(() => null) : null;
      if (requestId === request.current) {
        setSources(nextSources);
        setCustomPaths(nextCustom);
        setCommandProject(nextSnapshot?.projects?.find((project) => project.projectId === projectId) ?? null);
      }
    } catch (caught) {
      if (requestId === request.current) {
        setSources([]);
        setCustomPaths([]);
        setCommandProject(null);
        setError(caught instanceof Error ? caught.message : String(caught));
      }
    } finally {
      if (requestId === request.current) setLoading(false);
    }
  }, [desktop]);

  React.useEffect(() => {
    selectedProjectRef.current = selectedProjectId;
    request.current += 1;
    if (!selectedProjectId) {
      setSources([]);
      setCustomPaths([]);
      setCommandProject(null);
      setError(null);
      setLoading(false);
      return;
    }
    void refresh(selectedProjectId);
  }, [refresh, selectedProjectId]);

  const rescan = () => selectedProjectId && void refresh(selectedProjectId, true);
  const addPath = async () => {
    if (!desktop || !selectedProjectId || !path.trim()) return;
    const projectId = selectedProjectId;
    const mutationGeneration = request.current;
    setBusy(true);
    setError(null);
    try {
      await addCustomSourcePath(projectId, path.trim());
      setPath("");
      if (selectedProjectRef.current === projectId && request.current === mutationGeneration) await refresh(projectId);
    } catch (caught) {
      if (selectedProjectRef.current === projectId && request.current === mutationGeneration) setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setBusy(false);
    }
  };
  const removePath = async (customPath: CustomSourcePath) => {
    if (!desktop || !selectedProjectId) return;
    const projectId = selectedProjectId;
    const mutationGeneration = request.current;
    setBusy(true);
    try {
      await removeCustomSourcePath(projectId, customPath.id);
      if (selectedProjectRef.current === projectId && request.current === mutationGeneration) await refresh(projectId);
    } catch (caught) {
      if (selectedProjectRef.current === projectId && request.current === mutationGeneration) setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setBusy(false);
    }
  };
  const reorderPath = async (customPath: CustomSourcePath, order: number) => {
    if (!desktop || !selectedProjectId) return;
    const projectId = selectedProjectId;
    const mutationGeneration = request.current;
    setBusy(true);
    try {
      await updateCustomSourcePath({ projectId, pathOrId: customPath.id, order });
      if (selectedProjectRef.current === projectId && request.current === mutationGeneration) await refresh(projectId);
    } catch (caught) {
      if (selectedProjectRef.current === projectId && request.current === mutationGeneration) setError(caught instanceof Error ? caught.message : String(caught));
    } finally { setBusy(false); }
  };

  if (!selectedProjectId || !selected) {
    return <Placeholder title="Task Sources" description="Select a registered project to inspect bounded task-source evidence." />;
  }
  const available = sources.filter((source) => source.status === "AVAILABLE").length;
  const warnings = sources.filter((source) => source.status !== "AVAILABLE").length;
  const contractWarnings = commandProject?.warnings ?? [];
  const refreshLabel = commandProject?.refreshStatus ?? (desktop ? "UNAVAILABLE" : "BROWSER_PREVIEW");
  return (
    <>
      <PageHeader title="Task Sources" description={`Bounded source inventory for ${selected.name}.`} />
      {!desktop ? <div className="fixture-note">Browser preview uses no filesystem discovery. Open the native H!veAI build for live sources.</div> : null}
      {error ? <div className="safe-notice" role="alert">{error}</div> : null}
      <section className="panel task-sources-workspace">
        <div className="task-sources-toolbar">
          <div>
            <span className="eyebrow">Selected project</span>
            <h2>{selected.name}</h2>
            <p>{records.find((record) => record.id === selectedProjectId)?.originalPath ?? selected.description}</p>
          </div>
          <button className="primary-button" type="button" onClick={rescan} disabled={!desktop || loading}>
            <RefreshCw size={14} className={loading ? "spin" : undefined} /> Rescan sources
          </button>
        </div>
        <div className="task-source-summary" aria-label="Task source summary">
          <span><b>{available}</b> available</span>
          <span><b>{warnings}</b> warnings</span>
          <span><b>{customPaths.length}</b> custom paths</span>
        </div>
        <div className="project-intelligence-panel" aria-label="Project Intelligence and Dashboard Contract">
          <SectionHeader title="Project Intelligence / Dashboard Contract" detail="Single project entry contract" />
          <div className="project-intelligence-grid">
            <div><span>Entry contract</span><strong>.hiveai/PROJECT_DASHBOARD.md</strong></div>
            <div><span>Live tracking</span><strong>{commandProject?.trackingMode === "single-dashboard-watch" ? "SINGLE_DASHBOARD" : commandProject ? "LEGACY_FALLBACK" : "UNAVAILABLE"}</strong></div>
            <div><span>Manifest</span><strong>{commandProject?.manifestStatus ?? (desktop ? "UNAVAILABLE" : "BROWSER_PREVIEW")}</strong></div>
            <div><span>Task authority</span><strong>{commandProject?.taskAuthority ?? "UNAVAILABLE"}</strong></div>
            <div><span>Canonical task source</span><strong>{commandProject?.canonicalTaskSource ?? "Unknown"}</strong></div>
            <div><span>M09 refresh</span><strong>{refreshLabel}</strong></div>
            <div><span>Last refresh</span><strong>{commandProject?.refreshAt ?? "Unknown"}</strong></div>
            <div><span>Internal evidence</span><strong>{sources.length} discovered sources</strong></div>
            <div><span>Warnings</span><strong>{contractWarnings.length + warnings}</strong></div>
          </div>
          {contractWarnings.length ? <div className="project-intelligence-warning">{contractWarnings.slice(0, 3).join(" | ")}</div> : null}
        </div>
        <details className="advanced-source-inventory">
          <summary>Advanced source inventory ({sources.length})</summary>
          <p className="advanced-source-note">Internal evidence / provenance. These files are not independent live-watch targets in SINGLE_DASHBOARD mode.</p>
          {loading ? <LoadingState /> : error ? <ErrorState detail="Task source inventory is unavailable for this project." /> : sources.length === 0 ? <EmptyState title="No task source files discovered" detail="Rescan the registered project to inspect bounded task and planning evidence." /> : (
            <div className="task-sources-table" role="table" aria-label="Discovered task sources">
              <div className="task-source-row task-source-head" role="row"><span>Path</span><span>Kind</span><span>Origin</span><span>Authority</span><span>Modified</span><span>Status</span></div>
              {sources.map((source) => <div className="task-source-row" role="row" key={source.id}><span title={source.absolutePath}>{source.relativePath}</span><span>{source.sourceKind}</span><span>{source.origin}</span><span>{source.authorityClass} · {source.priority}</span><span>{source.modifiedAt ? new Date(source.modifiedAt).toLocaleString() : "Unknown"}</span><span className={`source-status source-status-${source.status.toLowerCase()}`}>{source.status}</span></div>)}
            </div>
          )}
        </details>
      </section>
      <section className="panel custom-source-panel">
        <SectionHeader title="Custom source paths" detail="Stored in H!veAI settings" />
        <div className="custom-source-add"><input aria-label="Custom source path" value={path} onChange={(event) => setPath(event.target.value)} placeholder="Relative file or directory inside project root" disabled={!desktop || busy} /><button className="secondary-button" type="button" onClick={addPath} disabled={!desktop || busy || !path.trim()}><Plus size={14} /> Add path</button></div>
        {customPaths.length ? <div className="custom-source-list">{customPaths.map((customPath) => <div key={customPath.id}><span>{customPath.displayPath}</span><small>{customPath.status} · {customPath.order}</small><button className="icon-button" type="button" aria-label={`Move ${customPath.displayPath} earlier`} onClick={() => void reorderPath(customPath, Math.max(0, customPath.order - 1))} disabled={busy || customPath.order === 0}><ChevronRight size={14} /></button><button className="icon-button" type="button" aria-label={`Remove ${customPath.displayPath}`} onClick={() => void removePath(customPath)} disabled={busy}><X size={14} /></button></div>)}</div> : <p className="fixture-note">No custom source paths configured.</p>}
      </section>
    </>
  );
}

function AgentTerminal({ session, onResize }: { session: AgentSession; onResize: (columns: number, rows: number) => void }) {
  const host = React.useRef<HTMLDivElement>(null);
  const terminal = React.useRef<XTermType | null>(null);
  const fitAddon = React.useRef<FitAddonType | null>(null);
  const output = `${session.stdout}${session.stderr ? `\r\n${session.stderr}` : ""}`;
  const outputRef = React.useRef(output);
  outputRef.current = output;
  React.useEffect(() => {
    if (!host.current || typeof window.matchMedia !== "function") return;
    let disposed = false;
    let observer: ResizeObserver | null = null;
    void Promise.all([import("@xterm/addon-fit"), import("@xterm/xterm")]).then(([fitModule, terminalModule]) => {
      if (disposed || !host.current) return;
      const instance = new terminalModule.Terminal({ convertEol: true, disableStdin: true, scrollback: 600, theme: { background: "#0b0e13", foreground: "#d8e2f0", cursor: "#7dd3fc" } });
      const fit = new fitModule.FitAddon();
      instance.loadAddon(fit);
      instance.open(host.current);
      fit.fit();
      instance.write(outputRef.current);
      terminal.current = instance;
      fitAddon.current = fit;
      observer = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(() => { fit.fit(); onResize(instance.cols, instance.rows); });
      observer?.observe(host.current);
    });
    return () => { disposed = true; observer?.disconnect(); terminal.current?.dispose(); terminal.current = null; fitAddon.current = null; };
  }, [onResize]);
  React.useEffect(() => {
    if (!terminal.current) return;
    terminal.current.clear();
    terminal.current.write(output);
  }, [output]);
  return <div className="agent-terminal" ref={host} aria-label={`${session.provider} live terminal`} data-testid="agent-live-terminal" />;
}

function AgentSessionOutput({ label, text, truncated }: { label: "stdout" | "stderr"; text: string; truncated: boolean }) {
  const rows = text.split(/\r?\n/).map((line) => {
    if (!line) return { label: "Output", content: "" };
    try {
      const parsed: unknown = JSON.parse(line);
      const eventLabel = parsed && typeof parsed === "object" && "type" in parsed && typeof parsed.type === "string" ? parsed.type : "JSON event";
      return { label: eventLabel, content: JSON.stringify(parsed, null, 2) ?? line };
    } catch {
      return { label: "Output", content: line };
    }
  });
  return <section className={`agent-output-reader ${label === "stderr" ? "agent-output-reader-error" : ""}`} aria-label={`${label} output`} data-testid={`agent-${label}-reader`}>
    <div className="agent-output-reader-heading">{label === "stderr" ? "Error output" : "Session output"}</div>
    <div className="agent-output-events">{rows.map((row, index) => <div className="agent-output-event" key={`${label}-${index}`}><div className="agent-output-event-label">{index + 1}. {row.label}</div><code className="agent-output-event-content">{row.content}</code></div>)}</div>
    {truncated ? <div className="agent-output-truncated">[{label === "stderr" ? "error output" : "output"} truncated]</div> : null}
  </section>;
}

function ProviderBadge({ provider }: { provider: SessionProvider }) {
  return <span className={`agent-provider-badge agent-provider-${provider.toLowerCase()}`}>{provider}</span>;
}

function AgentReadinessCard({ item }: { item: ProviderReadiness }) {
  return <div className="agent-readiness-card"><div><ProviderBadge provider={item.provider} /><strong>{item.readinessState}</strong></div><span>{item.version ?? "Version unavailable"}</span><small>{item.diagnosticMessage ?? "No diagnostic"}</small></div>;
}

function elapsedLabel(session: AgentSession, now: number) {
  if (!session.startedAt) return "Unknown";
  const started = Date.parse(session.startedAt);
  if (!Number.isFinite(started)) return "Unknown";
  const ended = session.endedAt ? Date.parse(session.endedAt) : now;
  const elapsed = Math.max(0, (Number.isFinite(ended) ? ended : now) - started);
  return `${Math.floor(elapsed / 1000)}s`;
}

export function Agents() {
  const desktop = isTauriDesktop();
  const { records, selectedProjectId, selectProject } = useProjectRegistry();
  const selected = records.find((record) => record.id === selectedProjectId) ?? records[0] ?? null;
  const [readiness, setReadiness] = React.useState<ProviderReadiness[]>([]);
  const [sessions, setSessions] = React.useState<AgentSession[]>([]);
  const [selectedSessionId, setSelectedSessionId] = React.useState<string | null>(null);
  const [provider, setProvider] = React.useState<SessionProvider>(selected?.preferredAgentProvider ?? "CODEX");
  const [prompt, setPrompt] = React.useState("");
  const [taskId, setTaskId] = React.useState("");
  const [error, setError] = React.useState<string | null>(null);
  const [busy, setBusy] = React.useState(false);
  const [git, setGit] = React.useState<GitSnapshot | null>(null);
  const [gitDiff, setGitDiff] = React.useState<GitDiff | null>(null);
  const [now, setNow] = React.useState(() => Date.now());
  const selectedSession = sessions.find((session) => session.id === selectedSessionId) ?? sessions[0] ?? null;
  const selectedReadiness = readiness.find((item) => item.provider === provider);
  const refreshSessions = React.useCallback(async () => {
    if (!desktop || !selected?.id) return;
    try { setSessions(await listAgentSessions(selected.id)); setError(null); } catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); }
  }, [desktop, selected?.id]);
  const refresh = React.useCallback(async () => {
    if (!desktop) return;
    try { setReadiness(await getAgentReadiness()); await refreshSessions(); } catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); }
  }, [desktop, refreshSessions]);
  React.useEffect(() => { setProvider(selected?.preferredAgentProvider ?? "CODEX"); }, [selected?.id, selected?.preferredAgentProvider]);
  React.useEffect(() => { void refresh(); }, [refresh]);
  React.useEffect(() => { if (!desktop || !selected?.id) return; const timer = window.setInterval(() => { void refreshSessions(); }, 750); return () => window.clearInterval(timer); }, [desktop, refreshSessions, selected?.id]);
  React.useEffect(() => {
    if (!selectedSession) { setGit(null); setGitDiff(null); return; }
    void Promise.all([getGitSnapshot(selectedSession.projectId), getGitDiff(selectedSession.projectId)]).then(([snapshot, diff]) => { setGit(Array.isArray(snapshot?.stagedFiles) ? snapshot : null); setGitDiff(typeof diff?.text === "string" ? diff : null); }).catch(() => { setGit(null); setGitDiff(null); });
  }, [selectedSession?.id, selectedSession?.projectId]);
  React.useEffect(() => {
    if (!selectedSession || !["STARTING", "RUNNING", "WAITING_PERMISSION", "STOPPING"].includes(selectedSession.state)) return;
    const timer = window.setInterval(() => setNow(Date.now()), 1000);
    return () => window.clearInterval(timer);
  }, [selectedSession?.id, selectedSession?.state]);
  const start = async () => {
    if (!desktop || !selected || !prompt.trim() || !selectedReadiness?.available) return;
    setBusy(true); setError(null);
    try { const session = await startAgentSession(selected.id, provider, prompt, taskId.trim() || null); setPrompt(""); setTaskId(""); setSelectedSessionId(session.id); await refreshSessions(); } catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); } finally { setBusy(false); }
  };
  const stop = async () => { if (!selectedSession) return; setBusy(true); setError(null); try { await stopAgentSession(selectedSession.projectId, selectedSession.id); await refreshSessions(); } catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); } finally { setBusy(false); } };
  const retry = async () => { if (!selectedSession || !prompt.trim()) return; setBusy(true); setError(null); try { const session = await retryAgentSession(selectedSession, prompt); setPrompt(""); setSelectedSessionId(session.id); await refreshSessions(); } catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); } finally { setBusy(false); } };
  const chooseProvider = async (value: SessionProvider) => { setProvider(value); if (selected) { try { await updateProjectSettings(selected.id, selected.priority, value); } catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); } } };
  const resize = React.useCallback((columns: number, rows: number) => { if (selectedSession?.supportsPty) void resizeAgentTerminal(selectedSession.projectId, selectedSession.id, rows, columns).catch(() => undefined); }, [selectedSession?.id, selectedSession?.projectId, selectedSession?.supportsPty]);
  return <>
    <PageHeader title="Agent Session Center" description="Observe owned Codex and Claude sessions for registered projects." />
    {!desktop ? <div className="fixture-note">Native H!veAI is required for provider sessions.</div> : null}
    {error ? <div className="safe-notice" role="alert">{error}</div> : null}
    <section className="panel agent-center-readiness"><SectionHeader title="Provider readiness" detail="Bounded native adapters" /><div className="agent-readiness-grid">{readiness.map((item) => <AgentReadinessCard key={item.provider} item={item} />)}{!readiness.length ? <div className="fixture-note">Provider readiness is checked in native H!veAI.</div> : null}</div></section>
    <section className="panel agent-operation-panel"><SectionHeader title="Start owned session" detail="Registered project, fixed provider policy" /><label>Project<select aria-label="Agent project" value={selected?.id ?? ""} onChange={(event) => selectProject(event.target.value, true)} disabled={busy || !records.length}>{records.map((record) => <option key={record.id} value={record.id}>{record.name}</option>)}</select></label><label>Provider<select aria-label="Agent provider" value={provider} onChange={(event) => void chooseProvider(event.target.value as SessionProvider)} disabled={busy}><option value="CODEX">Codex</option><option value="CLAUDE">Claude</option></select></label><label>Task ID <span className="agent-field-note">optional, must belong to project</span><input aria-label="Agent task ID" value={taskId} onChange={(event) => setTaskId(event.target.value)} disabled={busy} maxLength={256} /></label><label>Prompt<textarea aria-label="Agent prompt" value={prompt} onChange={(event) => setPrompt(event.target.value)} disabled={busy} maxLength={65536} rows={5} /></label><div className="agent-action-row"><button className="primary-button" type="button" onClick={() => void start()} disabled={!desktop || !selected || !selectedReadiness?.available || !prompt.trim() || busy}><Terminal size={15} /> Start {provider} session</button>{selectedSession && ["FAILED", "STOPPED", "CRASHED"].includes(selectedSession.state) ? <button className="secondary-button" type="button" onClick={() => void retry()} disabled={!prompt.trim() || busy}><RefreshCw size={15} /> Retry as new session</button> : null}</div></section>
    <section className="panel agent-sessions-panel"><SectionHeader title="Active and persisted sessions" detail={selected ? selected.name : "No project selected"} /><div className="agent-session-list">{sessions.map((session) => <button className={`agent-session-row ${session.id === selectedSession?.id ? "agent-session-row-selected" : ""}`} type="button" key={session.id} onClick={() => setSelectedSessionId(session.id)}><span><ProviderBadge provider={session.provider} /><strong>{session.operationKind}</strong></span><span>{session.state}</span><small>{session.startedAt ?? "Unknown"}</small></button>)}{!sessions.length ? <EmptyState title="No agent sessions" detail={selected ? "No persisted Codex or Claude session evidence is available." : "Register a project to use the session center."} /> : null}</div></section>
     {selectedSession ? <section className="panel agent-session-detail"><SectionHeader title="Selected session" detail={`${selectedSession.provider} / ${selectedSession.state}`} /><div className="agent-session-title"><ProviderBadge provider={selectedSession.provider} /><strong>{selectedSession.id}</strong><span>{selectedSession.state}</span></div><div className="agent-session-actions"><button className="secondary-button" type="button" onClick={() => void stop()} disabled={busy || !["STARTING", "RUNNING"].includes(selectedSession.state)}>Stop owned session</button><button className="secondary-button" type="button" onClick={() => setError(`${selectedSession.provider} resume is not supported by the verified provider capability`)} disabled={!selectedSession.supportsResume}>Resume</button></div><CockpitFacts facts={[["Project", selectedSession.projectId], ["Task", selectedSession.taskId ?? "Freeform"], ["Working directory", selectedSession.cwd], ["Started", selectedSession.startedAt ?? "Unknown"], ["Elapsed", selectedSession.elapsedMs == null ? elapsedLabel(selectedSession, now) : `${selectedSession.elapsedMs} ms`], ["Ended", selectedSession.endedAt ?? "Active"], ["Exit code", selectedSession.exitCode == null ? "Active / unknown" : String(selectedSession.exitCode)], ["Diagnostic code", selectedSession.diagnosticCode ?? "None"], ["Diagnostic message", selectedSession.diagnosticMessage ?? "None"], ["Prompt reference", selectedSession.promptReference ?? "Unavailable"], ["Provider version", selectedSession.providerVersion ?? "Unavailable"]]} /><div className="agent-detail-grid"><div><div className="agent-detail-heading">Live output</div><AgentTerminal session={selectedSession} onResize={resize} />{selectedSession.stdout ? <AgentSessionOutput label="stdout" text={selectedSession.stdout} truncated={selectedSession.stdoutTruncated} /> : null}{selectedSession.stderr ? <AgentSessionOutput label="stderr" text={selectedSession.stderr} truncated={selectedSession.stderrTruncated} /> : null}</div><div><div className="agent-detail-heading">Session timeline</div><div className="agent-timeline">{(selectedSession.events ?? []).map((event) => <div className="agent-timeline-row" key={event.id}><time>{event.occurredAt}</time><strong>{event.eventType}</strong><code>{JSON.stringify(event.payload ?? {})}</code></div>)}{!(selectedSession.events ?? []).length ? <span className="cockpit-muted">No durable event evidence available.</span> : null}</div><div className="agent-detail-heading">Changed files / Git authority</div><div className="agent-changed-files">{git?.stagedFiles.map((file) => <span key={`staged-${file.path}`}>STAGED: {file.path}</span>)}{git?.unstagedFiles.map((file) => <span key={`unstaged-${file.path}`}>UNSTAGED: {file.path}</span>)}{git?.untrackedFiles.map((file) => <span key={`untracked-${file}`}>UNTRACKED: {file}</span>)}{git?.conflictedFiles.map((file) => <span key={`conflict-${file}`}>CONFLICT: {file}</span>)}{git && !git.stagedFiles.length && !git.unstagedFiles.length && !git.untrackedFiles.length && !git.conflictedFiles.length ? <span className="cockpit-muted">No Git changes detected by the Git Engine.</span> : null}{!git ? <span className="cockpit-muted">Git evidence unavailable.</span> : null}</div>{gitDiff ? <details className="agent-diff"><summary>View bounded Git diff</summary><pre className="cockpit-code">{gitDiff.text || "No diff content"}</pre>{gitDiff.truncated ? <span className="agent-output-truncated">[Git diff truncated]</span> : null}</details> : null}</div></div><div className="agent-permission-note">Permission model: provider-managed. Claude runs in restricted plan mode; Codex keeps its accepted M13 policy. No generic approval or shell control surface is exposed.</div></section> : null}
  </>;
}

export function Audits() {
  return (
    <Placeholder
      title="Audit Center"
      description="Independent review status across your projects."
    />
  );
}
export function Settings() {
  const desktop = isTauriDesktop();
  const [error, setError] = React.useState<string | null>(null);
  const [restarting, setRestarting] = React.useState(false);

  const restart = async () => {
    if (!desktop || !window.confirm("Restart H!veAI now?")) return;
    setRestarting(true);
    setError(null);
    try {
      await invoke("hiveai_request_restart");
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
      setRestarting(false);
    }
  };

  return (
    <>
      <PageHeader title="Settings" description="Workspace preferences and local-first policy controls." />
      <section className="panel settings-panel">
        <SectionHeader title="Application" detail="Native desktop lifecycle" />
        <p>Restart relaunches the native H!veAI desktop application.</p>
        {error ? <div className="safe-notice" role="alert">{error}</div> : null}
        <button className="primary-button" type="button" onClick={restart} disabled={!desktop || restarting}>
          {restarting ? "Restarting..." : "Restart H!veAI"}
          <RefreshCw size={15} />
        </button>
        {!desktop ? <span className="settings-hint">Desktop app only</span> : null}
      </section>
    </>
  );
}
