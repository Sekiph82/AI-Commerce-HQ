import { Archive, GitBranch, GitFork, MapPin, MoreHorizontal, RefreshCw, Trash2 } from 'lucide-react';
import type { ProjectRecord } from '../projectRegistry';

function statusLabel(status: ProjectRecord['status']) {
  return status === 'ACTIVE' ? 'Active' : status === 'MISSING' ? 'Path missing' : 'Archived';
}

export function ProjectRegistryCard({ project, onOpen, onArchive, onRemove, onRepair, onPriority }: {
  project: ProjectRecord;
  onOpen: () => void;
  onArchive: () => void;
  onRemove: () => void;
  onRepair: () => void;
  onPriority: (priority: number) => void;
}) {
  const repository = project.repository;
  return <article className={`registry-card registry-card-${project.status.toLowerCase()}`}>
    <div className="registry-card-top"><div className="registry-project-mark">{project.name.slice(0, 2).toUpperCase()}</div><div className="registry-card-title"><div><h2>{project.name}</h2><span className={`registry-status registry-status-${project.status.toLowerCase()}`}>{statusLabel(project.status)}</span></div><button className="icon-button" type="button" aria-label={`More actions for ${project.name}`}><MoreHorizontal size={17} /></button></div></div>
    <div className="registry-path"><MapPin size={14} aria-hidden="true" /><span title={project.originalPath}>{project.originalPath}</span></div>
    <div className="registry-meta-grid"><div><span>Repository</span><strong>{repository?.isGitRepository ? <><GitFork size={13} />Git repository</> : 'Non-Git folder'}</strong></div><div><span>Branch</span><strong>{repository?.currentBranch ? <><GitBranch size={13} />{repository.currentBranch}</> : 'Not detected'}</strong></div><div><span>Priority</span><select aria-label={`Priority for ${project.name}`} value={project.priority} onChange={event => onPriority(Number(event.target.value))}><option value={0}>Normal</option><option value={1}>High</option><option value={2}>Critical</option></select></div></div>
    <div className="registry-remote">{repository?.preferredRemoteUrl ? <><GitFork size={13} /><span title={repository.preferredRemoteUrl}>{repository.githubOwner && repository.githubRepo ? `${repository.githubOwner}/${repository.githubRepo}` : repository.preferredRemoteUrl}</span></> : <><span className="registry-dot" />No remote detected</>}</div>
    <div className="registry-settings"><span>Builder <b>{project.preferredBuilder ?? 'Unassigned'}</b></span><span>Auditor <b>{project.preferredAuditor ?? 'Unassigned'}</b></span></div>
    <div className="registry-card-foot"><button className="secondary-button" type="button" onClick={onOpen}>Open cockpit</button><div className="registry-icon-actions">{project.status === 'MISSING' ? <button className="icon-button" type="button" onClick={onRepair} aria-label={`Repair path for ${project.name}`} title="Repair path"><RefreshCw size={15} /></button> : null}{project.status !== 'ARCHIVED' ? <button className="icon-button" type="button" onClick={onArchive} aria-label={`Archive ${project.name}`} title="Archive"><Archive size={15} /></button> : null}<button className="icon-button registry-danger" type="button" onClick={onRemove} aria-label={`Remove ${project.name} from registry`} title="Remove from registry"><Trash2 size={15} /></button></div></div>
  </article>;
}
