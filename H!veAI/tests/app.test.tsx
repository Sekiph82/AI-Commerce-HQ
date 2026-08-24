import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import App from '../src/App';
import { StatusBadge, ProjectOperationCard } from '../src/components/ui';
import { projects } from '../src/fixtures';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn((command: string) => Promise.resolve(command === 'hiveai_database_status' ? {
    initialized: true, engine: 'SQLite', schemaVersion: 2, migrationCount: 2, databasePath: 'hiveai.db', foreignKeysEnabled: true, lastMigrationStatus: 'APPLIED',
  } : {
    architectureMode: 'RUST_NATIVE_NO_SIDECAR', sidecarEnabled: false, lastError: null,
    legacyCommerceRuntime: { componentId: 'legacy', displayName: 'Legacy AI-Commerce-HQ runtime', kind: 'LEGACY_COMMERCE', state: 'DISABLED', health: 'DISABLED', startedAt: null, lastHeartbeat: null, restartCount: 0, lastError: null, ownership: 'Excluded from H!veAI startup' },
    components: [{ componentId: 'native', displayName: 'H!veAI native core', kind: 'NATIVE_CORE', state: 'HEALTHY', health: 'HEALTHY', startedAt: '1', lastHeartbeat: null, restartCount: 0, lastError: null, ownership: 'H!veAI Rust native core' }, { componentId: 'legacy', displayName: 'Legacy AI-Commerce-HQ runtime', kind: 'LEGACY_COMMERCE', state: 'DISABLED', health: 'DISABLED', startedAt: null, lastHeartbeat: null, restartCount: 0, lastError: null, ownership: 'Excluded from H!veAI startup' }],
  })),
}));

function renderApp(path = '/') { window.history.pushState({}, '', path); return render(<App />); }

describe('H!veAI M02 UI shell', () => {
  it('renders the application shell and command center', () => { renderApp(); expect(screen.getByText('H!veAI')).toBeInTheDocument(); expect(screen.getByRole('heading', { name: 'Command Center' })).toBeInTheDocument(); expect(screen.getByText('Project operations')).toBeInTheDocument(); });
  it('exposes sidebar navigation and primary routes', () => { renderApp('/projects'); expect(screen.getByRole('heading', { name: 'Projects' })).toBeInTheDocument(); expect(screen.getByRole('link', { name: 'Projects' })).toHaveClass('nav-active'); });
  it('renders all canonical status badge semantics', () => { render(<StatusBadge state="AUDIT_REQUIRED" />); expect(screen.getByText('Audit required')).toBeInTheDocument(); });
  it('renders a project operation card with progress and actor', () => { render(<ProjectOperationCard project={projects[0]} onAction={() => undefined} />); expect(screen.getByText('Connect project snapshot events')).toBeInTheDocument(); expect(screen.getByText('Codex')).toBeInTheDocument(); });
  it('renders a project cockpit route', () => { renderApp('/projects/formulab'); expect(screen.getByRole('heading', { name: 'FormuLab' })).toBeInTheDocument(); expect(screen.getByRole('button', { name: 'Overview' })).toBeInTheDocument(); });
  it('opens and closes the mock command palette without executing operations', () => { renderApp(); fireEvent.click(screen.getByRole('button', { name: 'Open command palette' })); expect(screen.getByRole('dialog')).toBeInTheDocument(); expect(screen.getByText('Mock navigation only')).toBeInTheDocument(); fireEvent.click(screen.getByRole('button', { name: 'Close command palette' })); expect(screen.queryByRole('dialog')).not.toBeInTheDocument(); });
  it('shows a safe placeholder message for attention actions', () => { renderApp(); fireEvent.click(screen.getByRole('button', { name: 'Open Scrubbots placeholder' })); expect(screen.getByRole('status')).toHaveTextContent('Available in a later milestone.'); });
  it('shows truthful native runtime status and disabled legacy runtime', async () => { Object.defineProperty(window, '__TAURI_INTERNALS__', { configurable: true, value: {} }); renderApp(); await waitFor(() => expect(screen.getByText('RUST NATIVE NO SIDECAR')).toBeInTheDocument()); expect(screen.getAllByText('DISABLED').length).toBeGreaterThan(0); expect(screen.getByText('No sidecar process is configured. Legacy commerce runtime is disabled and excluded from startup.')).toBeInTheDocument(); delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__; });
  it('shows truthful native database readiness', async () => { Object.defineProperty(window, '__TAURI_INTERNALS__', { configurable: true, value: {} }); renderApp(); await waitFor(() => expect(screen.getByText('SQLite schema v2')).toBeInTheDocument()); expect(screen.getByText('2 migrations · hiveai.db · foreign keys enabled')).toBeInTheDocument(); delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__; });
});
