import React from 'react';
import { createRoot } from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import './styles.css';

type NativeStatus = {
  productName: string;
  identifier: string;
  version: string;
  platform: string;
  appDataDir: string | null;
  logDir: string | null;
};

function App() {
  const [status, setStatus] = React.useState<NativeStatus | null>(null);
  const [error, setError] = React.useState<string | null>(null);
  const [restartPending, setRestartPending] = React.useState(false);

  const loadStatus = React.useCallback(async () => {
    try {
      setError(null);
      const nativeStatus = await invoke<NativeStatus>('hiveai_native_status');
      setStatus(nativeStatus);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    }
  }, []);

  React.useEffect(() => {
    void loadStatus();
  }, [loadStatus]);

  async function requestRestart() {
    setRestartPending(true);
    try {
      await invoke('hiveai_request_restart');
    } catch (caught) {
      setRestartPending(false);
      setError(caught instanceof Error ? caught.message : String(caught));
    }
  }

  return (
    <main className="foundation-shell">
      <section className="foundation-panel" aria-labelledby="foundation-title">
        <p className="eyebrow">M01 Foundation</p>
        <h1 id="foundation-title">H!veAI</h1>
        <p className="subtitle">AI Development Command Center</p>

        <dl className="status-grid">
          <div>
            <dt>Foundation status</dt>
            <dd>{status ? 'Native IPC online' : 'Checking native IPC'}</dd>
          </div>
          <div>
            <dt>App identity</dt>
            <dd>{status?.identifier ?? 'ai.hiveai.desktop'}</dd>
          </div>
          <div>
            <dt>Platform</dt>
            <dd>{status?.platform ?? 'pending'}</dd>
          </div>
          <div>
            <dt>Version</dt>
            <dd>{status?.version ?? '0.1.0'}</dd>
          </div>
        </dl>

        {status ? (
          <pre className="status-json">{JSON.stringify(status, null, 2)}</pre>
        ) : null}

        {error ? <p className="error">{error}</p> : null}

        <div className="actions">
          <button type="button" onClick={loadStatus}>Refresh status</button>
          <button type="button" onClick={requestRestart} disabled={restartPending}>
            {restartPending ? 'Restart requested' : 'Request restart'}
          </button>
        </div>
      </section>
    </main>
  );
}

createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
