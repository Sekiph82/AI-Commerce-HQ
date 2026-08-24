import { motion } from 'framer-motion';
import { ArrowUpRight, CheckCircle2, CircleAlert, Clock3, FileCheck2, LoaderCircle, UserRound, XCircle } from 'lucide-react';
import type { Actor, Project, WorkflowState } from '../types';
import type React from 'react';

const MotionArticle = motion.article as React.ComponentType<any>;
const MotionDiv = motion.div as React.ComponentType<any>;

const stateMeta: Record<WorkflowState, { label: string; tone: string; icon: typeof CheckCircle2 }> = {
  BACKLOG: { label: 'Backlog', tone: 'neutral', icon: Clock3 }, READY_FOR_IMPLEMENTATION: { label: 'Ready', tone: 'blue', icon: ArrowUpRight },
  CODEX_RUNNING: { label: 'Codex running', tone: 'running', icon: LoaderCircle }, CLAUDE_RUNNING: { label: 'Claude running', tone: 'running', icon: LoaderCircle },
  AUDIT_REQUIRED: { label: 'Audit required', tone: 'audit', icon: FileCheck2 }, AUDIT_PASSED: { label: 'Audit passed', tone: 'success', icon: CheckCircle2 },
  AUDIT_FAILED: { label: 'Audit failed', tone: 'danger', icon: XCircle }, FIX_REQUIRED: { label: 'Fix required', tone: 'danger', icon: CircleAlert },
  VERIFY_REQUIRED: { label: 'Verify required', tone: 'warning', icon: CircleAlert }, WAITING_OWNER: { label: 'Waiting owner', tone: 'human', icon: UserRound },
  WAITING_EXTERNAL: { label: 'Waiting external', tone: 'external', icon: Clock3 }, BLOCKED: { label: 'Blocked', tone: 'danger', icon: XCircle },
  FAILED: { label: 'Failed', tone: 'danger', icon: XCircle }, TASK_COMPLETE: { label: 'Task complete', tone: 'success', icon: CheckCircle2 },
};

export function StatusBadge({ state }: { state: WorkflowState }) {
  const meta = stateMeta[state]; const Icon = meta.icon;
  return <span className={`status-badge status-${meta.tone}`}><Icon size={13} aria-hidden="true" />{meta.label}</span>;
}

export function ActorBadge({ actor }: { actor: Actor }) { return <span className="actor-badge"><span className={`actor-dot actor-${actor.toLowerCase().replace(' ', '-')}`} />{actor}</span>; }

export function PageHeader({ eyebrow, title, description, action }: { eyebrow?: string; title: string; description?: string; action?: React.ReactNode }) {
  return <header className="page-header"><div><p className="eyebrow">{eyebrow ?? 'H!veAI workspace'}</p><h1>{title}</h1>{description ? <p className="page-description">{description}</p> : null}</div>{action ? <div>{action}</div> : null}</header>;
}

export function SectionHeader({ title, detail, action }: { title: string; detail?: string; action?: React.ReactNode }) { return <div className="section-header"><div><h2>{title}</h2>{detail ? <span>{detail}</span> : null}</div>{action}</div>; }

export function MetricCard({ label, value, detail, tone = 'default' }: { label: string; value: string; detail?: string; tone?: string }) { return <MotionArticle className={`metric-card metric-${tone}`} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }}><span>{label}</span><strong>{value}</strong>{detail ? <small>{detail}</small> : null}</MotionArticle>; }

export function ProgressIndicator({ value }: { value: number }) { return <div className="progress-wrap"><div className="progress-track"><MotionDiv className="progress-fill" initial={{ width: 0 }} animate={{ width: `${value}%` }} transition={{ duration: 0.7 }} /></div><span>{value}%</span></div>; }

export function PrimaryActionButton({ children, onClick, ariaLabel }: { children: React.ReactNode; onClick?: () => void; ariaLabel?: string }) { return <button className="primary-button" type="button" onClick={onClick} aria-label={ariaLabel}>{children}<ArrowUpRight size={15} aria-hidden="true" /></button>; }

export function EmptyState({ title, detail }: { title: string; detail: string }) { return <div className="state-panel"><div className="state-icon"><CircleAlert size={18} /></div><strong>{title}</strong><span>{detail}</span></div>; }
export function LoadingState() { return <div className="state-panel"><LoaderCircle className="spin" size={20} /><strong>Loading workspace</strong></div>; }
export function ErrorState({ detail }: { detail: string }) { return <div className="state-panel error-state"><XCircle size={20} /><strong>Unable to load view</strong><span>{detail}</span></div>; }

export function ProjectOperationCard({ project, onAction }: { project: Project; onAction: () => void }) {
  return <MotionArticle className="operation-card" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }}>
    <div className="operation-top"><div className="project-mark">{project.code}</div><div className="operation-title"><h3>{project.name}</h3><span>{project.phase}</span></div><StatusBadge state={project.state} /></div>
    <p className="operation-task">{project.task}</p><ProgressIndicator value={project.progress} />
    <div className="operation-meta"><span>Health <b className={`health-${project.health.toLowerCase()}`}>{project.health}</b></span><ActorBadge actor={project.actor} /><span>{project.updated}</span></div>
    <div className="operation-foot"><div><small>Next action</small><span>{project.nextAction}</span></div><PrimaryActionButton onClick={onAction}>Open cockpit</PrimaryActionButton></div>
  </MotionArticle>;
}

export function ActivityRow({ time, project, actor, event, state }: { time: string; project: string; actor: Actor; event: string; state: WorkflowState }) { return <div className="activity-row"><div className="activity-line" /><time>{time}</time><div className="activity-copy"><strong>{event}</strong><span>{project}</span></div><ActorBadge actor={actor} /><StatusBadge state={state} /></div>; }
