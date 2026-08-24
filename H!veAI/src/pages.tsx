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
import { getGitSnapshot } from "./gitEngine";
import type { GitSnapshot } from "./gitEngine";
import type { Project } from "./types";
import { useProjectRegistry } from "./registryContext";

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
          detail="Live project data becomes available in a later milestone."
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

export function CommandCenter() {
  const navigate = useNavigate();
  const [notice, setNotice] = React.useState<string | null>(null);
  const {
    projects: registryProjects,
    records,
    loading: registryLoading,
  } = useProjectRegistry();
  const liveProjects = isTauriDesktop() ? registryProjects : projects;
  const current = liveProjects[0];
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
                className="project-rail-row"
                type="button"
                key={project.id}
                onClick={() => navigate(`/projects/${project.id}`)}
              >
                <span className="project-mark">{project.code}</span>
                <span className="project-rail-copy">
                  <strong>{project.name}</strong>
                  <small>{project.phase}</small>
                  <span className="rail-progress">
                    <i style={{ width: `${project.progress}%` }} />
                  </span>
                </span>
                <b>
                  {isTauriDesktop()
                    ? (records.find((record) => record.id === project.id)
                        ?.status ?? "—")
                    : `${project.progress}%`}
                </b>
                <span
                  className={`rail-health health-${project.health.toLowerCase()}`}
                >
                  {project.health}
                </span>
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
                CURRENT TASK <span>12 / 28</span>
              </div>
              <h3>{current?.task ?? "No parsed task data yet"}</h3>
              <p>
                {isTauriDesktop()
                  ? "Task and workflow details will populate from registered project evidence in a later milestone."
                  : "Intelligent form field suggestions based on user input and context analysis."}
              </p>
              <div className="task-meta">
                <span>
                  Priority: <b>High</b>
                </span>
                <span>Type: Feature</span>
                <span>Est: 3h</span>
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
              {[
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
              ))}
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
              <strong>12 tasks completed</strong>
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
                  : "Open Scrubbots placeholder"
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
        aria-label={`Open ${item.project} placeholder`}
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
  const { refresh: refreshRegistry } = useProjectRegistry();
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
                  onOpen={() => navigate(`/projects/${project.id}`)}
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
        <aside className="registry-aside">
          <section className="registry-side-panel">
            <div className="registry-side-heading">
              <span className="side-icon">
                <FolderKanban size={16} />
              </span>
              <div>
                <span className="eyebrow">Registry boundary</span>
                <h2>Read-only by design</h2>
              </div>
            </div>
            <p>
              H!veAI records identity and cached Git metadata without changing
              the selected folder.
            </p>
            <div className="registry-rule">
              <Check size={14} />
              <span>Explicit user action required</span>
            </div>
            <div className="registry-rule">
              <ShieldCheck size={14} />
              <span>No branch, file, or remote mutation</span>
            </div>
            <div className="registry-rule">
              <GitBranch size={14} />
              <span>Live Git engine arrives in M06</span>
            </div>
          </section>
          <section className="registry-side-panel registry-side-accent">
            <span className="eyebrow">Current view</span>
            <strong>
              {records.length} registered project
              {records.length === 1 ? "" : "s"}
            </strong>
            <span>
              Search, sort, and status filters use persisted registry data.
            </span>
          </section>
        </aside>
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

export function ProjectCockpit() {
  const { id } = useParams();
  const project = projects.find((item) => item.id === id);
  const [registered, setRegistered] = React.useState<ProjectRecord | null>(
    null,
  );
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
  React.useEffect(() => {
    if (!id || !isTauriDesktop()) return;
    let active = true;
    setRouteState("loading");
    setRegistered(null);
    void getRegisteredProject(id)
      .then((value) => {
        if (active) {
          setRegistered(value);
          setRouteState("ready");
        }
      })
      .catch(() => {
        if (active) setRouteState("error");
      });
    return () => {
      active = false;
    };
  }, [id]);
  if (registered)
    return (
      <>
        <RegisteredProjectCockpit project={registered} />
        <GitStatusSurface projectId={registered.id} />
      </>
    );
  if (isTauriDesktop())
    return (
      <div className="cockpit-route-state">
        {routeState === "error" ? (
          <ErrorState detail="Registered project was not found or could not be loaded." />
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
  return (
    <Placeholder
      title="Tasks"
      description="A normalized task workspace for future project sources."
    />
  );
}
export function Agents() {
  return (
    <Placeholder
      title="Agent Sessions"
      description="A safe surface for future Codex and Claude sessions."
    />
  );
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
  return (
    <Placeholder
      title="Settings"
      description="Workspace preferences and local-first policy controls."
    />
  );
}
