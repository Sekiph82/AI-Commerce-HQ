import { ArrowUpRight, RefreshCw, Search, ShieldCheck } from "lucide-react";
import React from "react";
import { Link, useNavigate } from "react-router-dom";
import { DatabaseStatusPanel } from "./components/DatabaseStatusPanel";
import { RuntimeStatusPanel } from "./components/RuntimeStatusPanel";
import { WatcherStatusPanel } from "./components/WatcherStatusPanel";
import { LoadingState, MetricCard, SectionHeader } from "./components/ui";
import {
  getCommandCenterSnapshot,
  listenForCommandCenterRefresh,
  previewSnapshot,
  registryFallback,
  type CommandCenterSnapshot,
} from "./commandCenter";
import { isTauriDesktop } from "./projectRegistry";
import { useProjectRegistry } from "./registryContext";

const count = (value: number | null | undefined) => value == null ? "-" : String(value);

function Activity({ snapshot }: { snapshot: CommandCenterSnapshot }) {
  const [search, setSearch] = React.useState("");
  const [kind, setKind] = React.useState("ALL");
  const kinds = Array.from(new Set(snapshot.recentActivity.map((item) => item.kind)));
  const items = snapshot.recentActivity.filter((item) => {
    const text = `${item.projectName} ${item.event} ${item.kind} ${item.actor ?? ""}`.toLowerCase();
    return (!search || text.includes(search.toLowerCase())) && (kind === "ALL" || kind === item.kind);
  });
  return <section className="panel command-activity-filter">
    <SectionHeader title="Recent Activity" detail={`${items.length} bounded events`} />
    <div className="activity-filter-controls"><Search size={14} /><input aria-label="Search recent activity" value={search} onChange={(event) => setSearch(event.target.value)} placeholder="Search activity" /><select aria-label="Filter activity type" value={kind} onChange={(event) => setKind(event.target.value)}><option value="ALL">All types</option>{kinds.map((value) => <option value={value} key={value}>{value}</option>)}</select></div>
    <div className="activity-list">{items.slice(0, 50).map((item) => <div className="activity-row" key={item.id}><time>{item.occurredAt}</time><div className="activity-copy"><strong>{item.event}</strong><span>{item.projectName} | {item.kind}{item.actor ? ` | ${item.actor}` : ""}</span></div><span className="status-badge">{item.state ?? "EVIDENCE"}</span></div>)}{!items.length ? <div className="rail-empty">No matching activity evidence.</div> : null}</div>
  </section>;
}

export function CommandCenterLive() {
  const desktop = isTauriDesktop();
  const navigate = useNavigate();
  const { selectedProjectId, selectProject, records } = useProjectRegistry();
  const [snapshot, setSnapshot] = React.useState<CommandCenterSnapshot | null>(null);
  const [loading, setLoading] = React.useState(desktop);
  const [error, setError] = React.useState<string | null>(null);
  const generation = React.useRef(0);
  const refresh = React.useCallback(() => {
    const currentGeneration = ++generation.current;
    if (!desktop) {
      setSnapshot(previewSnapshot());
      setLoading(false);
      return;
    }
    setLoading(true);
    void getCommandCenterSnapshot().then((next) => {
      if (currentGeneration !== generation.current) return;
      setSnapshot(next && next.projects ? next : registryFallback(records));
      setError(null);
    }).catch((caught) => {
      if (currentGeneration !== generation.current) return;
      setSnapshot(registryFallback(records));
      setError(caught instanceof Error ? caught.message : String(caught));
    }).finally(() => {
      if (currentGeneration === generation.current) setLoading(false);
    });
  }, [desktop, records]);
  React.useEffect(() => { refresh(); }, [refresh]);
  React.useEffect(() => {
    if (!desktop) return;
    const internals = (window as Window & { __TAURI_INTERNALS__?: { transformCallback?: unknown } }).__TAURI_INTERNALS__;
    if (typeof internals?.transformCallback !== "function") return;
    let active = true;
    let cleanup: (() => void) | undefined;
    void listenForCommandCenterRefresh(() => { if (active) refresh(); }).then((unlisten) => { if (active) cleanup = unlisten; else unlisten(); }).catch(() => undefined);
    return () => { active = false; cleanup?.(); };
  }, [desktop, refresh]);
  const data = snapshot && snapshot.kpis && Array.isArray(snapshot.projects) ? snapshot : (desktop ? registryFallback(records) : previewSnapshot());
  const current = data.projects.find((project) => project.projectId === selectedProjectId) ?? data.projects[0] ?? null;
  const currentName = current?.name ?? (!desktop ? "Preview / Native data unavailable" : null);
  React.useEffect(() => {
    if (data.projects.length && (!selectedProjectId || !data.projects.some((project) => project.projectId === selectedProjectId))) selectProject(data.projects[0].projectId, true);
  }, [data.projects, selectedProjectId, selectProject]);
  return <div className="command-center" aria-label="Command Center overview">
    {error ? <div className="safe-notice" role="alert">{error}</div> : null}
    {data.warnings.slice(0, 3).map((warning) => <div className="safe-notice" role="alert" key={warning}>{warning}</div>)}
    <header className="command-heading"><div><h1>Global Overview</h1><h1 className="sr-only">Command Center</h1><span className="sr-only">Project operations</span></div><button className="secondary-button" type="button" onClick={refresh} disabled={loading}><RefreshCw size={15} className={loading ? "spin" : undefined} /> Refresh</button></header>
    <section className="command-kpis" aria-label="Portfolio metrics"><MetricCard label="Projects" value={String(data.kpis.projects)} detail="Registered portfolio" /><MetricCard label="Active tasks" value={count(data.kpis.activeTasks)} detail="Authoritative only" tone="blue" /><MetricCard label="Needs attention" value={count(data.kpis.needsAttention)} detail="Workflow and registry" tone="warning" /><MetricCard label="Running" value={count(data.kpis.running)} detail="Real M10 states" tone="running" /><MetricCard label="Completed tasks" value={count(data.kpis.completedTasks)} detail="Authoritative only" tone="audit" /><MetricCard label="Portfolio health" value={data.kpis.healthDetail} detail={data.kpis.authorityDetail} tone="external" /></section>
    <div className="command-layout">
      <section className="command-projects panel"><SectionHeader title="Projects" detail={`${data.projects.length} registered workspace${data.projects.length === 1 ? "" : "s"}`} action={<Link className="text-link" to="/projects">All <ArrowUpRight size={13} /></Link>} /><div className="project-rail">{data.projects.map((project) => <button className={project.projectId === selectedProjectId ? "project-rail-row project-rail-row-selected" : "project-rail-row"} type="button" key={project.projectId} aria-pressed={project.projectId === selectedProjectId} title={project.name} onClick={() => selectProject(project.projectId, true)}><strong>{project.name}</strong></button>)}{loading ? <LoadingState /> : !data.projects.length ? <span className="rail-empty">{desktop ? "No registered projects yet." : "Native project data unavailable in browser preview."}</span> : null}</div><button className="rail-footer" type="button" onClick={() => navigate("/projects")}>View all projects <ArrowUpRight size={13} /></button></section>
      <section className="command-cockpit panel"><div className="cockpit-title"><div><div className="cockpit-inline-label"><span className="eyebrow">Current project</span><h2>{currentName ?? (loading ? "Loading registered project" : "No registered project")}</h2></div><span>{current ? `${current.registryStatus} | ${current.provenanceMode}` : "Native project identity unavailable"}</span></div><span className={`health-label health-${current?.health.toLowerCase() ?? "unknown"}`}>{current?.health ?? "UNKNOWN"}</span><button className="secondary-button cockpit-open" type="button" disabled={!current} onClick={() => current && navigate(`/projects/${current.projectId}`)}>Open cockpit <ArrowUpRight size={14} /></button></div><div className="cockpit-tabs"><span className="tab-active">Cockpit</span><span>Tasks</span><span>Workflow</span><span>Audit</span><span>Logs</span></div><div className="cockpit-body"><div className="current-task"><div className="task-kicker">CURRENT TASK <span>{current?.totalTasks == null ? "-" : `${current.completedTasks ?? 0} / ${current.totalTasks}`}</span></div><h3>{current?.taskAuthority === "NOT_CANONICALIZED" ? "TASK AUTHORITY NOT YET CANONICALIZED" : current?.currentTask?.title ?? "No active task evidence"}</h3><p>{current?.taskAuthority === "NOT_CANONICALIZED" ? "No canonical task ledger is declared for this project." : current?.canonicalTaskSource ? `Authority: ${current.canonicalTaskSource}` : current ? `Provenance: ${current.provenanceMode}` : "Browser preview does not have native project evidence."}</p><div className="task-meta">{current ? <><span>Health: {current.health}</span><span>Manifest: {current.manifestStatus}</span><span>State: {current.currentState ?? "No workflow state"}</span>{current.currentTask?.requiredActor ? <span>Required actor: {current.currentTask.requiredActor}</span> : null}{current.refreshStatus === "DEGRADED" ? <span>Refresh: degraded{current.refreshError ? ` | ${current.refreshError}` : ""}</span> : null}</> : <span>Native project evidence unavailable</span>}</div><div className="subtask-list">{current?.lastAction ? <div>{current.lastAction.summary}<b>{current.lastAction.occurredAt}</b></div> : current?.nextAction ? <div>{current.nextAction}<b>Next action</b></div> : <div>No workflow action evidence.</div>}</div></div><div className="workflow-mini"><div className="task-kicker">WORKFLOW STATUS</div>{current?.currentState ? <div className="workflow-step workflow-active"><span>1</span><div><strong>{current.currentState}</strong><small>{current.nextAction ?? "No next workflow transition recorded"}</small></div></div> : <div className="workflow-empty">Workflow state unavailable.</div>}</div></div><div className="cockpit-bottom"><div><SectionHeader title="Recent activity" detail="Latest project events" /><div className="compact-activity">{data.recentActivity.filter((item) => item.projectId === current?.projectId).slice(0, 3).map((item) => <div className="activity-row" key={item.id}><time>{item.occurredAt}</time><div className="activity-copy"><strong>{item.event}</strong><span>{item.kind}{item.actor ? ` | ${item.actor}` : ""}</span></div></div>)}{!data.recentActivity.some((item) => item.projectId === current?.projectId) ? <div className="rail-empty">No project activity evidence.</div> : null}</div></div><div><SectionHeader title="Project metrics" detail="Current signal" /><div className="metric-mini-grid"><span><b>{count(current?.activeTasks)}</b>Active</span><span><b>{count(current?.completedTasks)}</b>Completed</span><span><b>{current?.progressPercent == null ? "-" : `${current.progressPercent}%`}</b>Progress</span><span><b>{current?.taskAuthority ?? "-"}</b>Authority</span></div></div></div></section>
      <aside className="command-right-rail"><section className="right-panel brief-compact"><SectionHeader title="AI Engineering Brief" detail="Factual inputs" />{data.engineeringBrief.facts.map((fact) => <div className="brief-line" key={fact.label}><ShieldCheck size={15} /><div><strong>{fact.label}: {fact.value}</strong><small>{fact.source}{fact.provenance.sourcePath ? ` | ${fact.provenance.sourcePath}` : ` | ${fact.provenance.sourceClass}`}</small></div></div>)}{!data.engineeringBrief.facts.length ? <div className="brief-line">Native factual brief unavailable.</div> : null}</section><section className="right-panel assistant-compact" aria-label="Needs Your Attention"><SectionHeader title="Needs Your Attention" detail={`${data.attention.length} items`} />{data.attention.slice(0, 5).map((item) => <button className="attention-line" type="button" key={item.id} onClick={() => item.projectId && selectProject(item.projectId, true)}><strong>{item.projectName || "Portfolio evidence"}</strong><span>{item.state} | {item.title}</span></button>)}{!data.attention.length ? <div className="assistant-message">No attention items.</div> : null}</section><section className="right-panel queue-compact"><SectionHeader title="Active Work Queue" detail={`${data.workQueue.length} bounded items`} />{data.workQueue.slice(0, 5).map((item) => <div className="queue-mini-row" key={item.id}><strong>{item.projectName}</strong><span>{item.stage} | {item.task}</span></div>)}{!data.workQueue.length ? <div className="assistant-message">No active work evidence.</div> : null}</section><section className="right-panel system-compact"><SectionHeader title="System Status" detail="Native panels" /><div className="system-row"><span>Snapshot</span><b>{snapshot ? "Current" : "Unavailable"}</b></div><div className="system-row"><span>Warnings</span><b>{data.warnings.length}</b></div><details className="system-detail"><summary>Detailed health</summary><RuntimeStatusPanel /><DatabaseStatusPanel /><WatcherStatusPanel /></details></section></aside>
    </div>
    <Activity snapshot={data} />
  </div>;
}
