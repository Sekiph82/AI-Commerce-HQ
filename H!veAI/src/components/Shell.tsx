import { AnimatePresence, motion } from 'framer-motion';
import { Activity, Bell, Bot, Boxes, Command, FolderKanban, LayoutDashboard, Menu, PanelLeftClose, Search, Settings, ShieldCheck, Sparkles, X } from 'lucide-react';
import { NavLink, useLocation, useNavigate } from 'react-router-dom';
import { useEffect, useState } from 'react';
import type React from 'react';
import hiveaiSmallLogo from '../assets/hiveai-small-logo.png';
import hiveaiTextLogo from '../assets/hiveai-text-logo.png';
import akiltaWordmark from '../assets/akilta-wordmark.svg';

const MotionDiv = motion.div as React.ComponentType<any>;

const navigation = [
  { to: '/', label: 'Command Center', icon: LayoutDashboard }, { to: '/projects', label: 'Projects', icon: FolderKanban },
  { to: '/tasks', label: 'Tasks', icon: Boxes }, { to: '/agents', label: 'Agents', icon: Bot }, { to: '/audits', label: 'Audit Center', icon: ShieldCheck },
  { to: '/activity', label: 'Activity', icon: Activity }, { to: '/settings', label: 'Settings', icon: Settings },
];

export function AppShell({ children }: { children: React.ReactNode }) {
  const [sidebarOpen, setSidebarOpen] = useState(false); const [paletteOpen, setPaletteOpen] = useState(false); const navigate = useNavigate(); const location = useLocation();
  useEffect(() => { const onKeyDown = (event: KeyboardEvent) => { if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k') { event.preventDefault(); setPaletteOpen(true); } if (event.key === 'Escape') { setPaletteOpen(false); setSidebarOpen(false); } }; window.addEventListener('keydown', onKeyDown); return () => window.removeEventListener('keydown', onKeyDown); }, []);
  const title = location.pathname.startsWith('/projects/') ? 'Project Cockpit' : navigation.find(item => item.to === location.pathname)?.label ?? 'Command Center';
  const commands = navigation.filter(item => item.to !== '/').map(item => ({ ...item, title: `Open ${item.label}` })).concat({ to: '/', label: 'Command Center', title: 'Go to Command Center', icon: LayoutDashboard });
  const go = (to: string) => { navigate(to); setPaletteOpen(false); setSidebarOpen(false); };
  return <div className="app-shell">
    <aside className={`sidebar ${sidebarOpen ? 'sidebar-open' : ''}`}>
      <div className="brand"><img className="brand-logo" src={hiveaiSmallLogo} alt="H!veAI emblem" /><div className="brand-wordmark"><img src={hiveaiTextLogo} alt="H!veAI" /><span>Development command center</span><span className="sr-only">H!veAI</span></div><button className="icon-button sidebar-close" type="button" onClick={() => setSidebarOpen(false)} aria-label="Close navigation"><X size={17} /></button></div>
      <div className="nav-label">Workspace</div><nav aria-label="Primary navigation">{navigation.map(item => { const Icon = item.icon; return <NavLink key={item.to} to={item.to} end={item.to === '/'} onClick={() => setSidebarOpen(false)} className={({ isActive }) => `nav-item ${isActive ? 'nav-active' : ''}`}><Icon size={17} aria-hidden="true" /><span>{item.label}</span>{item.label === 'Audit Center' ? <span className="nav-count">2</span> : null}</NavLink>; })}</nav>
      <div className="nav-label shortcut-label">Project shortcuts</div><div className="project-shortcuts"><button type="button" onClick={() => go('/projects/formulab')}><span className="shortcut-mark">FL</span>FormuLab</button><button type="button" onClick={() => go('/projects/fmcg-erp')}><span className="shortcut-mark">FE</span>FMCG ERP</button><button type="button" onClick={() => go('/projects/scrubbots')}><span className="shortcut-mark">SB</span>Scrubbots</button></div>
      <div className="sidebar-bottom"><div className="system-status"><span className="status-pulse" />Local foundation online</div><span className="version">H!veAI 0.1.0 · M05</span><div className="akilta-footer"><img src={akiltaWordmark} alt="Akilta" /><span>Built with ♥ for maximum productivity by Akilta</span></div></div>
    </aside>
    <main className={`main-area ${location.pathname === '/' ? 'main-command' : ''}`}><header className="topbar"><button className="icon-button mobile-menu" type="button" onClick={() => setSidebarOpen(true)} aria-label="Open navigation"><Menu size={19} /></button><div className="topbar-title"><span className="crumb">Workspace /</span><strong>{title}</strong></div><div className="topbar-actions"><button className="search-trigger" type="button" onClick={() => setPaletteOpen(true)}><Search size={16} /><span>Search workspace</span><kbd>Ctrl K</kbd></button><button className="icon-button" type="button" onClick={() => setPaletteOpen(true)} aria-label="Open command palette"><Command size={17} /></button><button className="icon-button assistant-button" type="button" onClick={() => setPaletteOpen(true)} aria-label="Open assistant placeholder"><Sparkles size={17} /></button><span className="sync-status"><span className="status-pulse" />Synced</span><button className="icon-button" type="button" onClick={() => setPaletteOpen(true)} aria-label="View notifications"><Bell size={17} /><span className="notification-dot" /></button></div></header><div className="content-scroll"><AnimatePresence mode="wait"><MotionDiv key={location.pathname} className="page-frame" initial={{ opacity: 0, y: 5 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -5 }} transition={{ duration: 0.18 }}>{children}</MotionDiv></AnimatePresence></div></main>
    {paletteOpen ? <div className="modal-backdrop" role="presentation" onMouseDown={() => setPaletteOpen(false)}><MotionDiv className="command-palette" role="dialog" aria-modal="true" aria-labelledby="palette-title" initial={{ opacity: 0, scale: .98, y: -8 }} animate={{ opacity: 1, scale: 1, y: 0 }} onMouseDown={(event: React.MouseEvent) => event.stopPropagation()}><div className="palette-head"><Search size={18} /><input autoFocus placeholder="Search commands..." aria-label="Search commands" /><button className="icon-button" type="button" onClick={() => setPaletteOpen(false)} aria-label="Close command palette"><PanelLeftClose size={17} /></button></div><p id="palette-title" className="palette-label">Navigation</p><div className="command-list">{commands.map(item => { const Icon = item.icon; return <button key={item.to} type="button" onClick={() => go(item.to)}><Icon size={17} /><span>{item.title}</span><kbd>↵</kbd></button>; })}</div><div className="palette-foot"><span>Mock navigation only</span><kbd>ESC</kbd></div></MotionDiv></div> : null}
  </div>;
}
