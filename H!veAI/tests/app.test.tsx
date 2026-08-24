import { fireEvent, render, screen } from '@testing-library/react';
import App from '../src/App';
import { StatusBadge, ProjectOperationCard } from '../src/components/ui';
import { projects } from '../src/fixtures';

function renderApp(path = '/') { window.history.pushState({}, '', path); return render(<App />); }

describe('H!veAI M02 UI shell', () => {
  it('renders the application shell and command center', () => { renderApp(); expect(screen.getByText('H!veAI')).toBeInTheDocument(); expect(screen.getByRole('heading', { name: 'Command Center' })).toBeInTheDocument(); expect(screen.getByText('Project operations')).toBeInTheDocument(); });
  it('exposes sidebar navigation and primary routes', () => { renderApp('/projects'); expect(screen.getByRole('heading', { name: 'Projects' })).toBeInTheDocument(); expect(screen.getByRole('link', { name: 'Projects' })).toHaveClass('nav-active'); });
  it('renders all canonical status badge semantics', () => { render(<StatusBadge state="AUDIT_REQUIRED" />); expect(screen.getByText('Audit required')).toBeInTheDocument(); });
  it('renders a project operation card with progress and actor', () => { render(<ProjectOperationCard project={projects[0]} onAction={() => undefined} />); expect(screen.getByText('Connect project snapshot events')).toBeInTheDocument(); expect(screen.getByText('Codex')).toBeInTheDocument(); });
  it('renders a project cockpit route', () => { renderApp('/projects/formulab'); expect(screen.getByRole('heading', { name: 'FormuLab' })).toBeInTheDocument(); expect(screen.getByRole('button', { name: 'Overview' })).toBeInTheDocument(); });
  it('opens and closes the mock command palette without executing operations', () => { renderApp(); fireEvent.click(screen.getByRole('button', { name: 'Open command palette' })); expect(screen.getByRole('dialog')).toBeInTheDocument(); expect(screen.getByText('Mock navigation only')).toBeInTheDocument(); fireEvent.click(screen.getByRole('button', { name: 'Close command palette' })); expect(screen.queryByRole('dialog')).not.toBeInTheDocument(); });
  it('shows a safe placeholder message for attention actions', () => { renderApp(); fireEvent.click(screen.getByRole('button', { name: 'Open Scrubbots placeholder' })); expect(screen.getByRole('status')).toHaveTextContent('Available in a later milestone.'); });
});
