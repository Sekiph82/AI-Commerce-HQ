import { invoke } from '@tauri-apps/api/core';
import { CheckCircle2, CircleOff, Cpu, LoaderCircle, ShieldCheck } from 'lucide-react';
import { useEffect, useState } from 'react';
import type { RuntimeStatus } from '../runtime';

export function RuntimeStatusPanel() {
  const [status, setStatus] = useState<RuntimeStatus | null>(null);
  const [error, setError] = useState<string | null>(() => '__TAURI_INTERNALS__' in window ? null : 'Tauri desktop runtime is unavailable in browser preview.');

  useEffect(() => {
    if (error) return;
    let active = true;
    void invoke<RuntimeStatus>('hiveai_runtime_status')
      .then(value => { if (active) setStatus(value); })
      .catch(caught => { if (active) setError(caught instanceof Error ? caught.message : String(caught)); });
    return () => { active = false; };
  }, [error]);

  return <section className="runtime-panel" aria-label="Runtime status">
    <div className="runtime-panel-head"><div><p className="eyebrow">System boundary</p><h2>Runtime status</h2></div><Cpu size={18} aria-hidden="true" /></div>
    {status ? <>
      <div className="runtime-mode"><span>Active architecture</span><strong>{status.architectureMode.replaceAll('_', ' ')}</strong></div>
      <div className="runtime-components">{status.components.map(component => <div className="runtime-component" key={component.componentId}><div className={`runtime-component-icon runtime-${component.kind.toLowerCase()}`}>{component.kind === 'LEGACY_COMMERCE' ? <CircleOff size={15} /> : component.kind === 'NATIVE_CORE' ? <CheckCircle2 size={15} /> : <ShieldCheck size={15} />}</div><div><strong>{component.displayName}</strong><span>{component.state.replaceAll('_', ' ')} · {component.ownership}</span></div><span className={`runtime-state runtime-state-${component.state.toLowerCase()}`}>{component.state}</span></div>)}</div>
      <p className="runtime-footnote">No sidecar process is configured. Legacy commerce runtime is disabled and excluded from startup.</p>
    </> : error ? <div className="runtime-unavailable"><CircleOff size={17} /><div><strong>Native runtime unavailable</strong><span>Open the Tauri desktop app to read runtime state.</span></div></div> : <div className="runtime-unavailable"><LoaderCircle className="spin" size={17} /><div><strong>Reading native runtime</strong><span>Waiting for the Tauri boundary.</span></div></div>}
  </section>;
}
