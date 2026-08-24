import { invoke } from '@tauri-apps/api/core';
import { CheckCircle2, CircleOff, Database, LoaderCircle } from 'lucide-react';
import { useEffect, useState } from 'react';
import type { DatabaseStatus } from '../database';

export function DatabaseStatusPanel() {
  const [status, setStatus] = useState<DatabaseStatus | null>(null);
  const [error, setError] = useState<string | null>(() => '__TAURI_INTERNALS__' in window ? null : 'Tauri desktop database status is unavailable in browser preview.');

  useEffect(() => {
    if (error) return;
    let active = true;
    void invoke<DatabaseStatus>('hiveai_database_status')
      .then(value => { if (active) setStatus(value); })
      .catch(caught => { if (active) setError(caught instanceof Error ? caught.message : String(caught)); });
    return () => { active = false; };
  }, [error]);

  return <section className="database-panel" aria-label="Database status">
    <div className="database-panel-head"><div><p className="eyebrow">Local persistence</p><h2>Database status</h2></div><Database size={18} aria-hidden="true" /></div>
    {status ? <div className="database-ready"><div className="database-ready-icon"><CheckCircle2 size={17} /></div><div><strong>{status.engine} schema v{status.schemaVersion}</strong><span>{status.migrationCount} migrations · {status.databasePath} · foreign keys {status.foreignKeysEnabled ? 'enabled' : 'disabled'}</span></div><span className="database-state">{status.lastMigrationStatus}</span></div> : error ? <div className="database-unavailable"><CircleOff size={17} /><div><strong>Native database unavailable</strong><span>Open the Tauri desktop app to read database readiness.</span></div></div> : <div className="database-unavailable"><LoaderCircle className="spin" size={17} /><div><strong>Reading database status</strong><span>Waiting for the Tauri persistence boundary.</span></div></div>}
  </section>;
}
