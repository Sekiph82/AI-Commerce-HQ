import { AnimatePresence, motion } from "framer-motion";
import { invoke } from "@tauri-apps/api/core";
import {
  Activity,
  Bell,
  Bot,
  Boxes,
  Command,
  FolderKanban,
  LayoutDashboard,
  Menu,
  Search,
  Settings,
  ShieldCheck,
  Sparkles,
  WandSparkles,
  X,
} from "lucide-react";
import { NavLink, useLocation, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import type React from "react";
import hiveaiLogo from "../assets/hiveai-logo.png";
import akiltaWordmark from "../assets/akilta-wordmark.svg";
import { useProjectRegistry } from "../registryContext";
import { isTauriDesktop } from "../projectRegistry";

const MotionDiv = motion.div as React.ComponentType<any>;

const navigation = [
  { to: "/", label: "Command Center", icon: LayoutDashboard },
  { to: "/projects", label: "Projects", icon: FolderKanban },
  { to: "/tasks", label: "Tasks", icon: Boxes },
  { to: "/agents", label: "Agents", icon: Bot },
  { to: "/prompts", label: "Prompt Engine", icon: WandSparkles },
  { to: "/audits", label: "Audit Center", icon: ShieldCheck },
  { to: "/activity", label: "Activity", icon: Activity },
  { to: "/settings", label: "Settings", icon: Settings },
];

type Surface = "palette" | "assistant" | "notifications" | null;

export function AppShell({ children }: { children: React.ReactNode }) {
  const { records, selectProject } = useProjectRegistry();
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [surface, setSurface] = useState<Surface>(null);
  const navigate = useNavigate();
  const location = useLocation();
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "k") {
        event.preventDefault();
        setSurface("palette");
      }
      if (event.key === "Escape") {
        setSurface(null);
        setSidebarOpen(false);
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);
  const title = location.pathname.startsWith("/projects/")
    ? "Project Cockpit"
    : (navigation.find((item) => item.to === location.pathname)?.label ??
      "Command Center");
  const commands = navigation
    .filter((item) => item.to !== "/")
    .map((item) => ({ ...item, title: `Open ${item.label}` }))
    .concat({
      to: "/",
      label: "Command Center",
      title: "Go to Command Center",
      icon: LayoutDashboard,
    });
  const go = (to: string) => {
    navigate(to);
    setSurface(null);
    setSidebarOpen(false);
  };
  const openAkilta = (event: React.MouseEvent<HTMLAnchorElement>) => {
    if (!isTauriDesktop()) return;
    event.preventDefault();
    void invoke("hiveai_open_akilta");
  };
  return (
    <div className="app-shell">
      <aside className={`sidebar ${sidebarOpen ? "sidebar-open" : ""}`}>
        <div className="brand">
          <img
            className="brand-logo"
            src={hiveaiLogo}
            alt="H!veAI"
          />
          <button
            className="icon-button sidebar-close"
            type="button"
            onClick={() => setSidebarOpen(false)}
            aria-label="Close navigation"
          >
            <X size={17} />
          </button>
        </div>
        <div className="nav-label">Workspace</div>
        <nav aria-label="Primary navigation">
          {navigation.map((item) => {
            const Icon = item.icon;
            return (
              <NavLink
                key={item.to}
                to={item.to}
                end={item.to === "/"}
                onClick={() => setSidebarOpen(false)}
                className={({ isActive }) =>
                  `nav-item ${isActive ? "nav-active" : ""}`
                }
              >
                <Icon size={17} aria-hidden="true" />
                <span>{item.label}</span>
                {item.label === "Audit Center" ? (
                  <span className="nav-count">2</span>
                ) : null}
              </NavLink>
            );
          })}
        </nav>
        <div className="nav-label shortcut-label">Project shortcuts</div>
        <div className="project-shortcuts">
          {records.length ? (
            records.slice(0, 4).map((record) => (
              <button
                type="button"
                key={record.id}
                onClick={() => {
                  selectProject(record.id);
                  go(`/projects/${record.id}`);
                }}
              >
                <span className="shortcut-mark">
                  {record.name
                    .replace(/[^A-Za-z0-9]/g, "")
                    .slice(0, 2)
                    .toUpperCase() || "PR"}
                </span>
                {record.name}
              </button>
            ))
          ) : (
            <span className="shortcut-empty">No registered projects</span>
          )}
        </div>
        <div className="sidebar-bottom">
          <div className="system-status">
            <span className="status-pulse" />
            Local foundation online
          </div>
          <span className="version">H!veAI 0.1.0</span>
        </div>
      </aside>
      <main
        className={`main-area ${location.pathname === "/" ? "main-command" : ""}`}
      >
        <header className="topbar">
          <button
            className="icon-button mobile-menu"
            type="button"
            onClick={() => setSidebarOpen(true)}
            aria-label="Open navigation"
          >
            <Menu size={19} />
          </button>
          <div className="topbar-title">
            <span className="crumb">Workspace /</span>
            <strong>{title}</strong>
          </div>
          <a
            className="topbar-akilta"
            href="https://www.akilta.com/"
            onClick={openAkilta}
            rel="noreferrer"
            title="Developed by Akilta"
          >
            <img src={akiltaWordmark} alt="Akilta" />
            <span className="topbar-akilta-copy">
              Built with <b>♥</b> for maximum productivity by Akilta
            </span>
          </a>
          <div className="topbar-actions">
            <button
              className="search-trigger"
              type="button"
              onClick={() => setSurface("palette")}
            >
              <Search size={16} />
              <span>Search workspace</span>
              <kbd>Ctrl K</kbd>
            </button>
            <button
              className="icon-button"
              type="button"
              onClick={() => setSurface("palette")}
              aria-label="Open command palette"
              title="Open command palette"
            >
              <Command size={17} />
            </button>
            <button
              className="icon-button assistant-button"
              type="button"
              onClick={() => setSurface("assistant")}
              aria-label="Open AI Assistant"
              title="Open AI Assistant"
            >
              <Sparkles size={17} />
            </button>
            <span className="sync-status">
              <span className="status-pulse" />
              Synced
            </span>
            <button
              className="icon-button"
              type="button"
              onClick={() => setSurface("notifications")}
              aria-label="Open Notifications"
              title="Open Notifications"
            >
              <Bell size={17} />
              <span className="notification-dot" />
            </button>
          </div>
        </header>
        <div className="content-scroll">
          <AnimatePresence mode="wait">
            <MotionDiv
              key={location.pathname}
              className="page-frame"
              initial={{ opacity: 0, y: 5 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -5 }}
              transition={{ duration: 0.18 }}
            >
              {children}
            </MotionDiv>
          </AnimatePresence>
        </div>
      </main>
      {surface ? (
        <div
          className="modal-backdrop"
          role="presentation"
          onMouseDown={() => setSurface(null)}
        >
          <div
            className={`surface-panel surface-${surface}`}
            role="dialog"
            aria-modal="true"
            aria-labelledby="surface-title"
            onMouseDown={(event) => event.stopPropagation()}
          >
            <div className="palette-head">
              <span className="surface-icon">
                {surface === "palette" ? (
                  <Search size={18} />
                ) : surface === "assistant" ? (
                  <Sparkles size={18} />
                ) : (
                  <Bell size={18} />
                )}
              </span>
              <h2 id="surface-title">
                {surface === "palette"
                  ? "Command Palette"
                  : surface === "assistant"
                    ? "AI Assistant"
                    : "Notifications"}
              </h2>
              <button
                className="icon-button"
                type="button"
                onClick={() => setSurface(null)}
                aria-label={
                  surface === "palette"
                    ? "Close command palette"
                    : surface === "assistant"
                      ? "Close AI Assistant"
                      : "Close notifications"
                }
              >
                <X size={17} />
              </button>
            </div>
            {surface === "palette" ? (
              <>
                <input
                  autoFocus
                  placeholder="Search commands..."
                  aria-label="Search commands"
                />
                <p className="palette-label">Navigation</p>
                <div className="command-list">
                  {commands.map((item) => {
                    const Icon = item.icon;
                    return (
                      <button
                        key={item.to}
                        type="button"
                        onClick={() => go(item.to)}
                      >
                        <Icon size={17} />
                        <span>{item.title}</span>
                        <kbd>↵</kbd>
                      </button>
                    );
                  })}
                </div>
              </>
            ) : (
              <div className="surface-placeholder">
                <strong>
                  {surface === "assistant"
                    ? "AI assistance is bounded to a later milestone."
                    : "No new notifications."}
                </strong>
                <span>
                  {surface === "assistant"
                    ? "The native foundation is online and ready for future task-aware assistance."
                    : "Notifications will appear here when audit and runtime events are available."}
                </span>
              </div>
            )}
          </div>
          <div className="palette-foot">
            <span>Mock navigation only</span>
            <kbd>ESC</kbd>
          </div>
        </div>
      ) : null}
    </div>
  );
}
