import { RefreshCw, ShieldCheck, WifiOff } from 'lucide-react';
import React from 'react';
import { isTauriDesktop } from '../projectRegistry';
import { getWatcherStatus, refreshWatcherSet } from '../watcher';
import type { WatcherStatusSummary } from '../watcher';
import { LoadingState } from './ui';

export function WatcherStatusPanel() {
  const [summary, setSummary] = React.useState<WatcherStatusSummary | null>(null);
  const [error, setError] = React.useState<string | null>(null);
  const [loading, setLoading] = React.useState(true);
  const load = React.useCallback((refresh = false) => { if (!isTauriDesktop()) { setLoading(false); return; } setLoading(true); const request = refresh ? refreshWatcherSet() : getWatcherStatus(); void request.then(value => { setSummary(value); setError(null); }).catch(caught => setError(caught instanceof Error ? caught.message : String(caught))).finally(() => setLoading(false)); }, []);
  React.useEffect(() => { load(); }, [load]);
  const missing = summary?.projects.filter(project => project.state === 'MISSING').length ?? 0;
  const degraded = summary?.projects.filter(project => project.state === 'DEGRADED' || project.rescanRequired).length ?? 0;
  return <section className="panel watcher-status-panel"><div className="section-header"><div><h2>Filesystem watcher</h2><span>Registry-scoped project evidence refresh</span></div><button className="icon-button" type="button" onClick={() => load(true)} aria-label="Refresh filesystem watchers" title="Refresh filesystem watchers"><RefreshCw size={15} /></button></div>{!isTauriDesktop() ? <div className="watcher-unavailable"><WifiOff size={16} /><span>Native watcher status is available in the Tauri desktop app.</span></div> : loading ? <LoadingState /> : error ? <div className="safe-notice" role="alert">{error}</div> : summary ? <><div className="watcher-health-row"><span className={`watcher-pulse ${summary.running ? 'watcher-pulse-live' : ''}`} /><strong>{summary.running ? 'Watching registered roots' : 'Watcher paused'}</strong><span>{summary.projects.length} project{summary.projects.length === 1 ? '' : 's'}</span></div><div className="watcher-metrics"><div><strong>{summary.projects.length - missing - degraded}</strong><span>Healthy</span></div><div><strong>{missing}</strong><span>Missing</span></div><div><strong>{degraded}</strong><span>Degraded</span></div><div><strong>{summary.queueDepth}/{summary.queueCapacity}</strong><span>Queue</span></div></div><div className="watcher-footer"><ShieldCheck size={14} /><span>{degraded || missing ? 'Review project paths or request a safe rescan.' : 'Changes are debounced and refresh evidence locally.'}</span></div></> : null}</section>;
}
