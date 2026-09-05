import React from "react";
import { Check, FileText, RefreshCw, Send, ShieldCheck, WandSparkles } from "lucide-react";
import { EmptyState, PageHeader, SectionHeader } from "./components/ui";
import { useProjectRegistry } from "./registryContext";
import { isTauriDesktop } from "./projectRegistry";
import { listWorkflowTasks, type WorkflowTask } from "./workflow";
import { approvePrompt, collectPromptContext, dispatchPrompt, editPrompt, generatePrompt, listPromptVersions, type ContextManifest, type PromptKind, type PromptVersion } from "./promptEngine";

const TASK_TITLE_LIMIT = 72;

function taskLabel(task: WorkflowTask) {
  const title = task.title.length > TASK_TITLE_LIMIT ? `${task.title.slice(0, TASK_TITLE_LIMIT - 1)}...` : task.title;
  return `${title} · ${task.currentState}`;
}

function PromptTaskPicker({ tasks, value, onChange, disabled }: { tasks: WorkflowTask[]; value: string; onChange: (value: string) => void; disabled: boolean }) {
  const selected = tasks.find((task) => task.taskId === value);
  const fullTitle = selected?.title ?? "Freeform project operation";
  return <label className="prompt-task-picker">Task <span className="agent-field-note">optional for a project operation</span><select aria-label="Prompt task" aria-describedby="prompt-task-title-detail" title={fullTitle} value={value} onChange={(event) => onChange(event.target.value)} disabled={disabled}><option value="">Freeform project operation</option>{tasks.map((task) => <option key={task.taskId} value={task.taskId} title={task.title}>{taskLabel(task)}</option>)}</select><span id="prompt-task-title-detail" className="sr-only">Full task title: {fullTitle}</span></label>;
}

export function PromptEnginePage() {
  const { records } = useProjectRegistry();
  const active = records.filter((record) => record.status === "ACTIVE");
  const [projectId, setProjectId] = React.useState(active[0]?.id ?? "");
  const [taskId, setTaskId] = React.useState("");
  const [tasks, setTasks] = React.useState<WorkflowTask[]>([]);
  const [kind, setKind] = React.useState<PromptKind>("IMPLEMENTATION");
  const [title, setTitle] = React.useState("");
  const [summary, setSummary] = React.useState("");
  const [findingIds, setFindingIds] = React.useState<string[]>([]);
  const [context, setContext] = React.useState<ContextManifest | null>(null);
  const [version, setVersion] = React.useState<PromptVersion | null>(null);
  const [history, setHistory] = React.useState<PromptVersion[]>([]);
  const [provider, setProvider] = React.useState<"CODEX" | "CLAUDE">("CODEX");
  const [busy, setBusy] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [notice, setNotice] = React.useState<string | null>(null);
  const desktop = isTauriDesktop();
  const selectedProject = active.find((record) => record.id === projectId);

  React.useEffect(() => {
    if (!projectId || !desktop) return;
    void listWorkflowTasks(projectId).then((result) => setTasks(result.tasks)).catch(() => setTasks([]));
  }, [desktop, projectId]);
  React.useEffect(() => { if (!active.some((record) => record.id === projectId)) setProjectId(active[0]?.id ?? ""); }, [active, projectId]);
  React.useEffect(() => { setProvider(selectedProject?.preferredAgentProvider ?? "CODEX"); }, [projectId, selectedProject?.preferredAgentProvider]);
  const run = async (action: () => Promise<void>) => { setBusy(true); setError(null); setNotice(null); try { await action(); } catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); } finally { setBusy(false); } };
  const refreshContext = () => run(async () => { const next = await collectPromptContext(projectId, taskId || null); setContext(next); setNotice("Bounded context refreshed."); });
  const generate = () => run(async () => { const next = await generatePrompt({ projectId, taskId: taskId || null, kind, title, summary, findingIds: findingIds.length ? findingIds : undefined }); setVersion(next); setContext(next.contextManifest); setHistory(await listPromptVersions(projectId, next.promptId)); setNotice("Draft generated. Review and approve it before dispatch."); });
  const save = () => version ? run(async () => { const next = await editPrompt({ projectId, promptId: version.promptId, versionId: version.id, content: version.content, title: title || undefined, summary: summary || undefined }); setVersion(next); setHistory(await listPromptVersions(projectId, next.promptId)); setNotice(next.version === version.version ? "Draft updated." : "A new immutable version was created."); }) : undefined;
  const approve = () => version ? run(async () => { const next = await approvePrompt(projectId, version.promptId, version.id); setVersion(next); setHistory(await listPromptVersions(projectId, next.promptId)); setNotice("Exact prompt version approved. Dispatch remains a separate action."); }) : undefined;
  const dispatch = () => version ? run(async () => { const result = await dispatchPrompt(projectId, version.promptId, version.id, provider); setVersion(result.prompt); setHistory(await listPromptVersions(projectId, result.promptId)); setNotice(`Dispatched ${provider} session ${result.session.id} with exact version ${result.promptVersion}.`); }) : undefined;
  const canGenerate = Boolean(desktop && projectId && title.trim() && summary.trim() && !busy);
  return <>
    <PageHeader title="Prompt Engine" description="Build bounded, reviewable prompts with exact provider provenance." />
    {!desktop ? <div className="fixture-note">Native H!veAI is required for prompt persistence and provider dispatch.</div> : null}
    {error ? <div className="safe-notice" role="alert">{error}</div> : null}
    {notice ? <div className="safe-notice prompt-notice" role="status"><Check size={15} />{notice}</div> : null}
    <div className="prompt-engine-flow">
      <section className="panel prompt-setup-panel"><SectionHeader title="1. Context and goal" detail="Registry-backed project and task authority" />
        <label>Project<select aria-label="Prompt project" value={projectId} onChange={(event) => { setProjectId(event.target.value); setTaskId(""); setContext(null); setVersion(null); }} disabled={busy || !active.length}>{active.map((record) => <option key={record.id} value={record.id}>{record.name}</option>)}</select></label>
        <PromptTaskPicker tasks={tasks} value={taskId} onChange={setTaskId} disabled={busy || !projectId} />
        <label>Prompt kind<select aria-label="Prompt kind" value={kind} onChange={(event) => setKind(event.target.value as PromptKind)} disabled={busy}><option value="IMPLEMENTATION">Implementation</option><option value="REMEDIATION">Remediation</option><option value="AUDIT_SUPPORT">Audit support</option></select></label>
        <label>Title<input aria-label="Prompt title" value={title} onChange={(event) => setTitle(event.target.value)} maxLength={512} disabled={busy} /></label>
        <label>Summary<textarea aria-label="Prompt summary" value={summary} onChange={(event) => setSummary(event.target.value)} maxLength={4096} rows={3} disabled={busy} /></label>
        <div className="prompt-action-row"><button className="secondary-button" type="button" onClick={() => void refreshContext()} disabled={!projectId || busy}><RefreshCw size={15} /> Refresh context</button><button className="primary-button" type="button" onClick={() => void generate()} disabled={!canGenerate}><WandSparkles size={15} /> Generate draft</button></div>
        {context ? <details className="prompt-evidence" open><summary><ShieldCheck size={14} /> Context manifest</summary><div className="prompt-stats"><span>{context.items.filter((item) => item.disposition === "INCLUDED").length} included</span><span>{context.includedBytes} bytes</span><span>{context.omittedCount} omitted</span></div><div className="prompt-context-list">{context.items.map((item) => item.class === "AUDIT_FINDING" ? <label key={`${item.class}-${item.reference}`} className="prompt-finding-option"><input type="checkbox" checked={findingIds.includes(item.reference.replace(/^finding:/, ""))} onChange={(event) => { const id = item.reference.replace(/^finding:/, ""); setFindingIds((current) => event.target.checked ? [...current, id] : current.filter((currentId) => currentId !== id)); }} /><strong>{item.disposition}</strong><span>{item.class} · {item.reference}</span></label> : <div key={`${item.class}-${item.reference}`}><strong>{item.disposition}</strong><span>{item.class} · {item.reference}</span></div>)}</div></details> : <EmptyState title="No context snapshot" detail="Refresh or generate to inspect bounded evidence." />}
      </section>
      <section className="panel prompt-review-panel"><SectionHeader title="2. Review and approve" detail={version ? `Version ${version.version} · ${version.approvalState}` : "Generated draft appears here"} />
        {version ? <><label>Prompt body<textarea aria-label="Prompt body editor" className="prompt-editor" value={version.content} onChange={(event) => setVersion({ ...version, content: event.target.value })} disabled={busy || version.approvalState !== "DRAFT"} /></label><div className="prompt-action-row"><button className="secondary-button" type="button" onClick={() => void save()} disabled={busy || version.approvalState !== "DRAFT"}><FileText size={15} /> Save edit</button><button className="primary-button" type="button" onClick={() => void approve()} disabled={busy || version.approvalState !== "DRAFT"}><Check size={15} /> Approve exact version</button></div></> : <EmptyState title="Draft pending" detail="Choose a project, define the goal, and generate a draft. Nothing dispatches automatically." />}
      </section>
      <section className="panel prompt-dispatch-panel"><SectionHeader title="3. Provider and dispatch" detail={version?.approvalState === "APPROVED" ? "Choose one provider, then dispatch the exact approved version" : "Approval is required before dispatch"} />
        <div className="prompt-dispatch-row"><div className="prompt-provider-control" role="group" aria-label="Prompt provider"><span className="prompt-control-label">Provider</span><div className="prompt-provider-options">{(["CODEX", "CLAUDE"] as const).map((option) => <button key={option} className={`prompt-provider-option${provider === option ? " prompt-provider-option-selected" : ""}`} type="button" aria-pressed={provider === option} onClick={() => setProvider(option)} disabled={busy}>{option === "CODEX" ? "Codex" : "Claude"}</button>)}</div></div><button className="primary-button" type="button" onClick={() => void dispatch()} disabled={busy || !version || version.approvalState !== "APPROVED" || version.dispatchState !== "AVAILABLE"}><Send size={15} /> Dispatch to {provider === "CODEX" ? "Codex" : "Claude"}</button></div>
        {version ? <details className="prompt-evidence"><summary>Version history and provenance</summary><div className="prompt-version-list">{history.map((item) => <div key={item.id}><strong>v{item.version}</strong><span>{item.approvalState} · {item.dispatchState}</span><code>{item.bodySha256.slice(0, 16)}...</code><small>{item.dispatchedSessionId ? `Session ${item.dispatchedSessionId}` : item.dispatchError ?? "Not dispatched"}</small></div>)}</div><dl className="prompt-provenance"><div><dt>Context</dt><dd>{version.contextManifest?.manifestSha256 ?? "Unavailable"}</dd></div><div><dt>Origin</dt><dd>{version.origin}</dd></div><div><dt>Approved hash</dt><dd>{version.approvedBodySha256 ?? "Not approved"}</dd></div></dl></details> : <div className="prompt-dispatch-note">The dispatch control stays inactive until a human approves an exact prompt version.</div>}
      </section>
    </div>
  </>;
}
