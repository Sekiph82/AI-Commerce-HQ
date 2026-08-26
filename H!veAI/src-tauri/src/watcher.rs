use crate::db::DatabaseState;
use crate::git_engine::{snapshot as git_snapshot, GitSnapshotRequest};
use crate::project_dashboard::{self, ManifestStatus};
use crate::projects::{fetch_project, list_projects, ProjectListQuery, ProjectRecord};
use notify::event::{CreateKind, ModifyKind, RemoveKind, RenameMode};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

#[cfg(test)]
thread_local! {
    static FAIL_NEXT_WATCH_ATTACH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static FAIL_NEXT_GIT_REFRESH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static FAIL_NEXT_SNAPSHOT_PERSISTENCE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

const CHANNEL_CAPACITY: usize = 512;
const DEBOUNCE_WINDOW: Duration = Duration::from_millis(250);
const REFRESH_WINDOW: Duration = Duration::from_millis(750);
const MAX_REFRESH_ERROR_BYTES: usize = 512;
const TASK_REFRESH_HEALTH_KEY: &str = "m11.task-refresh-health";
const EXCLUDED_DIRS: &[&str] = &[
    ".git/objects",
    ".git/logs",
    "node_modules",
    "target",
    "dist",
    "build",
    ".next",
    "cache",
    "caches",
    "tmp",
    "temp",
];

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatcherStatusSummary {
    pub running: bool,
    pub queue_depth: usize,
    pub queue_capacity: usize,
    pub projects: Vec<ProjectWatcherStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectWatcherStatus {
    pub project_id: String,
    pub state: String,
    pub watcher_health: String,
    pub available: bool,
    pub last_event_at: Option<String>,
    pub last_refresh_at: Option<String>,
    pub evidence_generated_at: Option<String>,
    pub changed_path_count: u64,
    pub rescan_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TaskRefreshHealth {
    pub status: String,
    pub refreshed_at: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NormalizedEventKind {
    Create,
    Modify,
    Remove,
    Rename,
    RescanRequired,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventCategory {
    GitMetadata,
    TaskCandidate,
    Source,
    Config,
    Other,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedEvent {
    pub event_id: String,
    pub project_id: String,
    pub kind: NormalizedEventKind,
    pub relative_path: String,
    pub old_relative_path: Option<String>,
    pub timestamp: String,
    pub source: String,
    pub category_hint: EventCategory,
}

#[derive(Debug)]
struct RawInput {
    project_id: String,
    root: PathBuf,
    single_dashboard: bool,
    event: notify::Result<Event>,
}

#[derive(Debug, Clone)]
struct PendingEvent {
    event: NormalizedEvent,
    last_seen: SystemTime,
}

struct Inner {
    statuses: HashMap<String, ProjectWatcherStatus>,
    watches: HashMap<String, RecommendedWatcher>,
    watch_roots: HashMap<String, PathBuf>,
    watch_scopes: HashMap<String, String>,
    pending_count: usize,
    running: bool,
    last_refresh_mono: HashMap<String, SystemTime>,
}

pub struct WatcherManager {
    database: DatabaseState,
    inner: Arc<Mutex<Inner>>,
    sender: SyncSender<RawInput>,
    app_handle: Option<AppHandle>,
    worker: Mutex<Option<JoinHandle<()>>>,
}

impl WatcherManager {
    pub fn initialize(database: DatabaseState) -> Result<Self, String> {
        Self::initialize_internal(database, None)
    }

    pub fn initialize_with_app_handle(
        database: DatabaseState,
        app_handle: AppHandle,
    ) -> Result<Self, String> {
        Self::initialize_internal(database, Some(app_handle))
    }

    fn initialize_internal(
        database: DatabaseState,
        app_handle: Option<AppHandle>,
    ) -> Result<Self, String> {
        let (sender, receiver) = sync_channel(CHANNEL_CAPACITY);
        let inner = Arc::new(Mutex::new(Inner {
            statuses: HashMap::new(),
            watches: HashMap::new(),
            watch_roots: HashMap::new(),
            watch_scopes: HashMap::new(),
            pending_count: 0,
            running: true,
            last_refresh_mono: HashMap::new(),
        }));
        let worker_inner = Arc::clone(&inner);
        let worker_database = database.clone();
        let worker_app_handle = app_handle.clone();
        let worker_sender = sender.clone();
        let worker = thread::Builder::new()
            .name("hiveai-filesystem-watcher".to_string())
            .spawn(move || {
                worker_loop(
                    worker_database,
                    worker_inner,
                    receiver,
                    worker_app_handle,
                    worker_sender,
                )
            })
            .map_err(|error| format!("start H!veAI filesystem watcher worker: {error}"))?;
        let manager = Self {
            database,
            inner,
            sender,
            app_handle,
            worker: Mutex::new(Some(worker)),
        };
        manager.refresh_from_registry()?;
        Ok(manager)
    }

    pub fn status(&self) -> WatcherStatusSummary {
        let inner = self.inner.lock().expect("watcher state lock");
        let mut projects = inner.statuses.values().cloned().collect::<Vec<_>>();
        projects.sort_by(|left, right| left.project_id.cmp(&right.project_id));
        WatcherStatusSummary {
            running: inner.running,
            queue_depth: inner.pending_count,
            queue_capacity: CHANNEL_CAPACITY,
            projects,
        }
    }

    pub fn project_status(&self, project_id: &str) -> Result<ProjectWatcherStatus, String> {
        self.inner
            .lock()
            .expect("watcher state lock")
            .statuses
            .get(project_id)
            .cloned()
            .ok_or_else(|| "project watcher is not configured".to_string())
    }

    pub fn refresh_from_registry(&self) -> Result<WatcherStatusSummary, String> {
        let projects = list_projects(
            &self.database,
            ProjectListQuery {
                include_archived: Some(false),
                ..Default::default()
            },
        )?;
        let desired = projects
            .iter()
            .map(|project| project.id.clone())
            .collect::<HashSet<_>>();
        let existing = self
            .inner
            .lock()
            .expect("watcher state lock")
            .statuses
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        for project in projects {
            self.configure_project(project)?;
        }
        let mut inner = self.inner.lock().expect("watcher state lock");
        for project_id in existing {
            if !desired.contains(&project_id) {
                inner.watches.remove(&project_id);
                inner.watch_roots.remove(&project_id);
                inner.watch_scopes.remove(&project_id);
                inner.statuses.remove(&project_id);
            }
        }
        drop(inner);
        Ok(self.status())
    }

    pub fn rescan_project(&self, project_id: &str) -> Result<ProjectWatcherStatus, String> {
        let project = fetch_project(&self.database, project_id)?;
        self.configure_project(project)?;
        refresh_project_snapshot(&self.database, &self.inner, project_id, Vec::new(), true)?;
        let _ =
            refresh_task_intelligence(&self.database, project_id, true, self.app_handle.as_ref());
        self.project_status(project_id)
    }

    fn configure_project(&self, project: ProjectRecord) -> Result<(), String> {
        let available = project.status == "ACTIVE" && Path::new(&project.normalized_path).is_dir();
        let single_dashboard = available
            && project_dashboard::resolve(&self.database, &project.id)
                .map(|dashboard| {
                    dashboard.tracking_mode.as_deref() == Some("single-dashboard-watch")
                        && matches!(
                            dashboard.manifest_status,
                            ManifestStatus::Valid | ManifestStatus::Partial
                        )
                })
                .unwrap_or(false);
        configure_project_watcher(&self.inner, &self.sender, project, single_dashboard)
    }
}

fn configure_project_watcher(
    inner: &Arc<Mutex<Inner>>,
    sender: &SyncSender<RawInput>,
    project: ProjectRecord,
    single_dashboard: bool,
) -> Result<(), String> {
    let available = project.status == "ACTIVE" && Path::new(&project.normalized_path).is_dir();
    let status = ProjectWatcherStatus {
        project_id: project.id.clone(),
        state: if available { "WATCHING" } else { "MISSING" }.to_string(),
        watcher_health: if available { "HEALTHY" } else { "DEGRADED" }.to_string(),
        available,
        last_event_at: None,
        last_refresh_at: None,
        evidence_generated_at: None,
        changed_path_count: 0,
        rescan_required: false,
    };
    let callback_inner = Arc::clone(inner);
    let mut inner = inner.lock().expect("watcher state lock");
    inner
        .statuses
        .entry(project.id.clone())
        .and_modify(|current| {
            current.state = status.state.clone();
            current.watcher_health = status.watcher_health.clone();
            current.available = status.available;
        })
        .or_insert(status);
    let root = PathBuf::from(&project.normalized_path);
    let desired_scope = if single_dashboard {
        "SINGLE_DASHBOARD"
    } else {
        "LEGACY_RECURSIVE"
    };
    let root_changed = inner.watch_roots.get(&project.id) != Some(&root);
    let scope_changed =
        inner.watch_scopes.get(&project.id).map(String::as_str) != Some(desired_scope);
    if root_changed || scope_changed {
        inner.watches.remove(&project.id);
        inner.watch_roots.remove(&project.id);
        inner.watch_scopes.remove(&project.id);
    }
    if !available || inner.watches.contains_key(&project.id) {
        if !available {
            inner.watches.remove(&project.id);
            inner.watch_roots.remove(&project.id);
            inner.watch_scopes.remove(&project.id);
        }
        return Ok(());
    }
    let project_id = project.id.clone();
    let project_root = project.normalized_path.clone();
    let sender = sender.clone();
    #[cfg(test)]
    if FAIL_NEXT_WATCH_ATTACH.with(|failpoint| failpoint.replace(false)) {
        if let Some(status) = inner.statuses.get_mut(&project.id) {
            status.state = "DEGRADED".to_string();
            status.watcher_health = "DEGRADED".to_string();
            status.rescan_required = true;
        }
        return Err("test-only watcher attachment failpoint".to_string());
    }
    let mut watcher = match RecommendedWatcher::new(
        move |event| {
            let input = RawInput {
                project_id: project_id.clone(),
                root: PathBuf::from(&project_root),
                single_dashboard,
                event,
            };
            match sender.try_send(input) {
                Ok(()) => {}
                Err(TrySendError::Full(_)) => {
                    if let Ok(mut state) = callback_inner.lock() {
                        if let Some(status) = state.statuses.get_mut(&project_id) {
                            status.rescan_required = true;
                            status.state = "DEGRADED".to_string();
                            status.watcher_health = "OVERFLOW".to_string();
                        }
                    }
                }
                Err(TrySendError::Disconnected(_)) => {}
            }
        },
        Config::default(),
    ) {
        Ok(watcher) => watcher,
        Err(_) => {
            if let Some(status) = inner.statuses.get_mut(&project.id) {
                status.state = "DEGRADED".to_string();
                status.watcher_health = "DEGRADED".to_string();
            }
            return if available {
                Err("watcher backend failed to initialize".to_string())
            } else {
                Ok(())
            };
        }
    };
    let root_path = Path::new(&project.normalized_path);
    let root_mode = if single_dashboard {
        RecursiveMode::NonRecursive
    } else {
        RecursiveMode::Recursive
    };
    if watcher.watch(root_path, root_mode).is_err() {
        if let Some(status) = inner.statuses.get_mut(&project.id) {
            status.state = "DEGRADED".to_string();
            status.watcher_health = "DEGRADED".to_string();
        }
        if available {
            return Err("watcher failed to attach to registered root".to_string());
        }
        return Ok(());
    }
    if single_dashboard {
        let dashboard_dir = root_path.join(".hiveai");
        if watcher
            .watch(&dashboard_dir, RecursiveMode::NonRecursive)
            .is_err()
        {
            if let Some(status) = inner.statuses.get_mut(&project.id) {
                status.state = "DEGRADED".to_string();
                status.watcher_health = "DEGRADED".to_string();
            }
            return Err("single-dashboard watcher failed to attach .hiveai".to_string());
        }
    }
    inner.watches.insert(project.id.clone(), watcher);
    inner.watch_roots.insert(project.id.clone(), root);
    inner
        .watch_scopes
        .insert(project.id, desired_scope.to_string());
    Ok(())
}

impl Drop for WatcherManager {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.running = false;
            inner.watches.clear();
            inner.watch_roots.clear();
            inner.watch_scopes.clear();
        }
        let _ = self.sender.try_send(RawInput {
            project_id: String::new(),
            root: PathBuf::new(),
            single_dashboard: false,
            event: Err(notify::Error::generic("shutdown")),
        });
        if let Ok(mut worker) = self.worker.lock() {
            if let Some(handle) = worker.take() {
                let _ = handle.join();
            }
        }
    }
}

fn worker_loop(
    database: DatabaseState,
    inner: Arc<Mutex<Inner>>,
    receiver: Receiver<RawInput>,
    app_handle: Option<AppHandle>,
    sender: SyncSender<RawInput>,
) {
    let mut pending: HashMap<(String, String), PendingEvent> = HashMap::new();
    loop {
        match receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(input) if input.project_id.is_empty() => break,
            Ok(input) => accept_input(&inner, &mut pending, input),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
        loop {
            match receiver.try_recv() {
                Ok(input) if input.project_id.is_empty() => return,
                Ok(input) => accept_input(&inner, &mut pending, input),
                Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => break,
            }
        }
        let now = SystemTime::now();
        let due = pending
            .iter()
            .filter(|(_, item)| {
                now.duration_since(item.last_seen).unwrap_or_default() >= DEBOUNCE_WINDOW
            })
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>();
        if !due.is_empty() {
            let mut grouped: HashMap<String, Vec<NormalizedEvent>> = HashMap::new();
            for key in due {
                if let Some(item) = pending.remove(&key) {
                    grouped
                        .entry(item.event.project_id.clone())
                        .or_default()
                        .push(item.event);
                }
            }
            for (project_id, events) in grouped {
                if !refresh_allowed(&inner, &project_id) {
                    let deferred_at = SystemTime::now()
                        .checked_sub(REFRESH_WINDOW - DEBOUNCE_WINDOW)
                        .unwrap_or_else(SystemTime::now);
                    for event in events {
                        pending.insert(
                            (project_id.clone(), event.relative_path.clone()),
                            PendingEvent {
                                event,
                                last_seen: deferred_at,
                            },
                        );
                    }
                } else {
                    let dashboard_signal = events
                        .iter()
                        .any(|event| is_project_dashboard_path(&event.relative_path));
                    let dashboard_lifecycle_signal = events
                        .iter()
                        .any(|event| is_project_dashboard_lifecycle_path(&event.relative_path));
                    let single_dashboard = inner
                        .lock()
                        .ok()
                        .and_then(|state| state.watch_scopes.get(&project_id).cloned())
                        .is_some_and(|scope| scope == "SINGLE_DASHBOARD");
                    let relevant = events.iter().any(|event| {
                        matches!(
                            event.category_hint,
                            EventCategory::TaskCandidate | EventCategory::Source
                        )
                    });
                    let refreshed =
                        refresh_project_snapshot(&database, &inner, &project_id, events, false)
                            .is_ok();
                    if refreshed && dashboard_lifecycle_signal {
                        if let Ok(project) = fetch_project(&database, &project_id) {
                            let available = project.status == "ACTIVE"
                                && Path::new(&project.normalized_path).is_dir();
                            let single_dashboard = available
                                && project_dashboard::resolve(&database, &project.id)
                                    .map(|dashboard| {
                                        dashboard.tracking_mode.as_deref()
                                            == Some("single-dashboard-watch")
                                            && matches!(
                                                dashboard.manifest_status,
                                                ManifestStatus::Valid | ManifestStatus::Partial
                                            )
                                    })
                                    .unwrap_or(false);
                            let _ = configure_project_watcher(
                                &inner,
                                &sender,
                                project,
                                single_dashboard,
                            );
                        }
                    }
                    let task_refresh_allowed = relevant && (!single_dashboard || dashboard_signal);
                    if refreshed && task_refresh_allowed {
                        let _ = refresh_task_intelligence(
                            &database,
                            &project_id,
                            false,
                            app_handle.as_ref(),
                        );
                    } else if task_refresh_allowed && !refreshed {
                        let error = "WATCHER_REFRESH_FAILED: filesystem snapshot refresh failed";
                        let _ =
                            persist_task_refresh_health(&database, &project_id, Err(error.into()));
                        emit_task_refresh_event(app_handle.as_ref(), &project_id, false, false);
                    }
                }
            }
            if let Ok(mut state) = inner.lock() {
                state.pending_count = pending.len();
            }
        }
    }
}

fn accept_input(
    inner: &Arc<Mutex<Inner>>,
    pending: &mut HashMap<(String, String), PendingEvent>,
    input: RawInput,
) {
    if input.project_id.is_empty() {
        return;
    }
    let normalized = match input.event {
        Ok(event) => normalize_event_with_mode(
            &input.project_id,
            &input.root,
            &event,
            input.single_dashboard,
        ),
        Err(_) => {
            mark_rescan(inner, &input.project_id);
            return;
        }
    };
    for event in normalized {
        if pending.len() >= CHANNEL_CAPACITY {
            mark_rescan(inner, &input.project_id);
            break;
        }
        let key = (event.project_id.clone(), event.relative_path.clone());
        let now = SystemTime::now();
        pending
            .entry(key)
            .and_modify(|current| {
                current.event.kind = merge_kind(&current.event.kind, &event.kind);
                current.event.old_relative_path = current
                    .event
                    .old_relative_path
                    .clone()
                    .or(event.old_relative_path.clone());
                current.last_seen = now;
            })
            .or_insert(PendingEvent {
                event,
                last_seen: now,
            });
        if let Ok(mut state) = inner.lock() {
            if let Some(status) = state.statuses.get_mut(&input.project_id) {
                status.last_event_at = Some(timestamp());
                status.changed_path_count = status.changed_path_count.saturating_add(1);
            }
            state.pending_count = pending.len();
        }
    }
}

fn refresh_project_snapshot(
    database: &DatabaseState,
    inner: &Arc<Mutex<Inner>>,
    project_id: &str,
    events: Vec<NormalizedEvent>,
    explicit: bool,
) -> Result<(), String> {
    let project = fetch_project(database, project_id)?;
    let available = project.status == "ACTIVE" && Path::new(&project.normalized_path).is_dir();
    let git_relevant = explicit
        || events.iter().any(|event| {
            matches!(event.category_hint, EventCategory::GitMetadata)
                || is_project_dashboard_lifecycle_path(&event.relative_path)
        });
    let git_id = if available
        && project
            .repository
            .as_ref()
            .map(|repository| repository.is_git_repository)
            .unwrap_or(false)
        && git_relevant
    {
        #[cfg(test)]
        if FAIL_NEXT_GIT_REFRESH.with(|failpoint| failpoint.replace(false)) {
            return Err("test-only Git refresh failpoint".to_string());
        }
        let _ = git_snapshot(
            database,
            GitSnapshotRequest {
                project_id: project_id.to_string(),
                persist: Some(true),
            },
        )?;
        latest_git_snapshot_id(
            database,
            project
                .repository
                .as_ref()
                .map(|repository| repository.id.as_str())
                .unwrap_or_default(),
        )?
    } else {
        None
    };
    let now = timestamp();
    let last_event = events.iter().map(|event| event.timestamp.clone()).max();
    let prior_rescan_required = inner
        .lock()
        .expect("watcher state lock")
        .statuses
        .get(project_id)
        .map(|status| status.rescan_required)
        .unwrap_or(false);
    let successful_reconciliation = available && explicit;
    let rescan_required = if successful_reconciliation {
        false
    } else {
        prior_rescan_required
    };
    let health = if !available {
        "MISSING"
    } else if rescan_required {
        "OVERFLOW"
    } else {
        "HEALTHY"
    };
    let connection = database.open_connection()?;
    #[cfg(test)]
    if FAIL_NEXT_SNAPSHOT_PERSISTENCE.with(|failpoint| failpoint.replace(false)) {
        return Err("test-only snapshot persistence failpoint".to_string());
    }
    connection.execute("INSERT INTO project_snapshots (id, project_id, availability, git_snapshot_id, last_filesystem_event_at, last_watcher_refresh_at, evidence_generated_at, changed_path_count, rescan_required, watcher_health, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6, ?7, ?8, ?9, ?6)", rusqlite::params![Uuid::new_v4().to_string(), project_id, if available { "AVAILABLE" } else { "MISSING" }, git_id, last_event, now, events.len() as i64, rescan_required as i64, health]).map_err(|error| format!("persist project snapshot: {error}"))?;
    if let Ok(mut state) = inner.lock() {
        if let Some(status) = state.statuses.get_mut(project_id) {
            status.available = available;
            status.state = if available { "WATCHING" } else { "MISSING" }.to_string();
            status.watcher_health = health.to_string();
            status.last_refresh_at = Some(now.clone());
            status.evidence_generated_at = Some(now);
            status.rescan_required = rescan_required;
        }
        if !available {
            state.watches.remove(project_id);
        }
        state
            .last_refresh_mono
            .insert(project_id.to_string(), SystemTime::now());
    }
    Ok(())
}

fn latest_git_snapshot_id(
    database: &DatabaseState,
    repository_id: &str,
) -> Result<Option<String>, String> {
    database.open_connection()?.query_row("SELECT id FROM git_snapshots WHERE repository_id = ?1 ORDER BY captured_at DESC LIMIT 1", [repository_id], |row| row.get(0)).optional().map_err(|error| format!("read Git snapshot identity: {error}"))
}
fn refresh_allowed(inner: &Arc<Mutex<Inner>>, project_id: &str) -> bool {
    inner
        .lock()
        .expect("watcher state lock")
        .last_refresh_mono
        .get(project_id)
        .map(|last| last.elapsed().unwrap_or_default() >= REFRESH_WINDOW)
        .unwrap_or(true)
}

fn normalize_event(project_id: &str, root: &Path, event: &Event) -> Vec<NormalizedEvent> {
    normalize_event_with_mode(project_id, root, event, false)
}

fn normalize_event_with_mode(
    project_id: &str,
    root: &Path,
    event: &Event,
    single_dashboard: bool,
) -> Vec<NormalizedEvent> {
    let kind = match event.kind {
        EventKind::Create(CreateKind::Any) | EventKind::Create(_) => NormalizedEventKind::Create,
        EventKind::Remove(RemoveKind::Any) | EventKind::Remove(_) => NormalizedEventKind::Remove,
        EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => NormalizedEventKind::Rename,
        EventKind::Modify(_) => NormalizedEventKind::Modify,
        _ => NormalizedEventKind::RescanRequired,
    };
    if kind == NormalizedEventKind::RescanRequired {
        return vec![NormalizedEvent {
            event_id: Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            kind,
            relative_path: String::new(),
            old_relative_path: None,
            timestamp: timestamp(),
            source: "WATCHER".to_string(),
            category_hint: EventCategory::Other,
        }];
    }
    if kind == NormalizedEventKind::Rename
        && event
            .paths
            .iter()
            .any(|path| relative_path(path, root).is_none())
    {
        return Vec::new();
    }
    let paths = event
        .paths
        .iter()
        .filter_map(|path| relative_path(path, root))
        .filter(|path| !is_excluded(path))
        .filter(|path| !single_dashboard || is_project_dashboard_lifecycle_path(path))
        .collect::<Vec<_>>();
    if paths.is_empty() {
        return Vec::new();
    }
    let category = if single_dashboard {
        EventCategory::TaskCandidate
    } else {
        category_hint(&paths[0])
    };
    if kind == NormalizedEventKind::Rename && paths.len() >= 2 {
        return vec![make_event(
            project_id,
            kind,
            paths[1].clone(),
            Some(paths[0].clone()),
            category,
        )];
    }
    paths
        .into_iter()
        .map(|path| make_event(project_id, kind.clone(), path, None, category.clone()))
        .collect()
}

fn make_event(
    project_id: &str,
    kind: NormalizedEventKind,
    relative_path: String,
    old_relative_path: Option<String>,
    category_hint: EventCategory,
) -> NormalizedEvent {
    NormalizedEvent {
        event_id: Uuid::new_v4().to_string(),
        project_id: project_id.to_string(),
        kind,
        relative_path,
        old_relative_path,
        timestamp: timestamp(),
        source: "WATCHER".to_string(),
        category_hint,
    }
}
fn relative_path(path: &Path, root: &Path) -> Option<String> {
    let relative = path.strip_prefix(root).ok()?;
    if root.exists() && !physically_contained(path, root) {
        return None;
    }
    let value = relative.to_string_lossy().replace('\\', "/");
    if value.is_empty() || value == "." || value.split('/').any(|part| part == "..") {
        None
    } else {
        Some(value.trim_start_matches("./").to_string())
    }
}

fn physically_contained(path: &Path, root: &Path) -> bool {
    let Ok(canonical_root) = std::fs::canonicalize(root) else {
        return false;
    };
    let existing = if path.exists() {
        path.to_path_buf()
    } else {
        let mut ancestor = path.to_path_buf();
        while !ancestor.exists() {
            if !ancestor.pop() {
                return false;
            }
        }
        ancestor
    };
    std::fs::canonicalize(existing)
        .map(|canonical| canonical.starts_with(canonical_root))
        .unwrap_or(false)
}
fn is_excluded(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    EXCLUDED_DIRS
        .iter()
        .any(|excluded| lower == *excluded || lower.starts_with(&format!("{excluded}/")))
}
fn category_hint(path: &str) -> EventCategory {
    let lower = path.to_ascii_lowercase();
    if lower.starts_with(".git/") || lower == ".git" {
        EventCategory::GitMetadata
    } else if lower.ends_with("tasks.md")
        || lower.ends_with("/tasks")
        || lower.ends_with("plans.md")
        || lower == ".hiveai"
        || lower.starts_with(".hiveai/")
        || lower.contains("/.hiveai/")
    {
        EventCategory::TaskCandidate
    } else if lower.ends_with(".toml")
        || lower.ends_with(".json")
        || lower.ends_with(".yaml")
        || lower.ends_with(".yml")
    {
        EventCategory::Config
    } else if lower.contains("/src/") || lower.starts_with("src/") {
        EventCategory::Source
    } else {
        EventCategory::Other
    }
}

fn is_project_dashboard_path(path: &str) -> bool {
    path.replace('\\', "/")
        .eq_ignore_ascii_case(project_dashboard::MANIFEST_RELATIVE_PATH)
}

fn is_project_dashboard_lifecycle_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    normalized.eq_ignore_ascii_case(project_dashboard::MANIFEST_RELATIVE_PATH)
        || normalized.eq_ignore_ascii_case(".hiveai")
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct IntelligenceRefreshEvent {
    project_id: String,
    category: String,
    generated_at: String,
    success: bool,
}

fn refresh_task_intelligence(
    database: &DatabaseState,
    project_id: &str,
    explicit: bool,
    app_handle: Option<&AppHandle>,
) -> Result<(), String> {
    let result = crate::task_intelligence::parse(database, project_id);
    let health_result = result
        .as_ref()
        .map(|_| ())
        .map_err(|error| error.to_string());
    persist_task_refresh_health(database, project_id, health_result)?;
    emit_task_refresh_event(app_handle, project_id, explicit, result.is_ok());
    result.map(|_| ())
}

fn emit_task_refresh_event(
    app_handle: Option<&AppHandle>,
    project_id: &str,
    explicit: bool,
    success: bool,
) {
    if let Some(app) = app_handle {
        let event = IntelligenceRefreshEvent {
            project_id: project_id.to_string(),
            category: if explicit { "RESCAN" } else { "TASK_DASHBOARD" }.to_string(),
            generated_at: timestamp(),
            success,
        };
        let _ = app.emit("hiveai-command-center-refresh", event);
    }
}

fn refresh_health_key(project_id: &str) -> String {
    format!("{TASK_REFRESH_HEALTH_KEY}:{project_id}")
}

fn persist_task_refresh_health(
    database: &DatabaseState,
    project_id: &str,
    result: Result<(), String>,
) -> Result<(), String> {
    let refreshed_at = timestamp();
    let error = result.err().map(|mut value| {
        while value.len() > MAX_REFRESH_ERROR_BYTES {
            value.pop();
        }
        value
    });
    let health = TaskRefreshHealth {
        status: if error.is_some() {
            "DEGRADED"
        } else {
            "SUCCESS"
        }
        .into(),
        refreshed_at,
        error,
    };
    let json = serde_json::to_string(&health).map_err(|error| error.to_string())?;
    database
        .open_connection()?
        .execute(
            "INSERT INTO settings (key, value_json, scope, created_at, updated_at) VALUES (?1, ?2, 'PROJECT', ?3, ?3) ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json, updated_at=excluded.updated_at",
            rusqlite::params![refresh_health_key(project_id), json, health.refreshed_at],
        )
        .map_err(|error| format!("persist task refresh health: {error}"))?;
    Ok(())
}

pub fn read_task_refresh_health(
    database: &DatabaseState,
    project_id: &str,
) -> Result<Option<TaskRefreshHealth>, String> {
    let json: Option<String> = database
        .open_connection()?
        .query_row(
            "SELECT value_json FROM settings WHERE key=?1 AND scope='PROJECT'",
            [refresh_health_key(project_id)],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| format!("read task refresh health: {error}"))?;
    json.map(|value| {
        serde_json::from_str(&value).map_err(|error| format!("read task refresh health: {error}"))
    })
    .transpose()
}
fn merge_kind(current: &NormalizedEventKind, next: &NormalizedEventKind) -> NormalizedEventKind {
    match (current, next) {
        (NormalizedEventKind::Create, NormalizedEventKind::Modify) => NormalizedEventKind::Create,
        (_, NormalizedEventKind::Remove) => NormalizedEventKind::Remove,
        (_, NormalizedEventKind::Rename) => NormalizedEventKind::Rename,
        (NormalizedEventKind::RescanRequired, _) | (_, NormalizedEventKind::RescanRequired) => {
            NormalizedEventKind::RescanRequired
        }
        (_, value) => value.clone(),
    }
}
fn mark_rescan(inner: &Arc<Mutex<Inner>>, project_id: &str) {
    if let Ok(mut state) = inner.lock() {
        if let Some(status) = state.statuses.get_mut(project_id) {
            status.rescan_required = true;
            status.state = "DEGRADED".to_string();
            status.watcher_health = "OVERFLOW".to_string();
        }
    }
}
fn timestamp() -> String {
    crate::time::utc_timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_dashboard::MANIFEST_RELATIVE_PATH;
    use std::fs;
    use std::process::Command;
    use tempfile::tempdir;

    fn init_git_repo(root: &Path) {
        let run = |args: &[&str]| {
            let status = Command::new("git")
                .args(args)
                .current_dir(root)
                .status()
                .expect("git command should start");
            assert!(status.success(), "git command failed: {args:?}");
        };
        run(&["init", "--quiet"]);
        run(&["config", "user.email", "test@example.com"]);
        run(&["config", "user.name", "H!veAI Test"]);
        fs::write(root.join("README.md"), "fixture").unwrap();
        run(&["add", "."]);
        run(&["commit", "--quiet", "-m", "fixture"]);
    }

    fn event(project_id: &str, kind: EventKind, path: &str) -> RawInput {
        RawInput {
            project_id: project_id.to_string(),
            root: PathBuf::from("C:\\repo"),
            single_dashboard: false,
            event: Ok(Event::new(kind).add_path(PathBuf::from(format!("C:\\repo\\{path}")))),
        }
    }
    fn inner() -> Arc<Mutex<Inner>> {
        Arc::new(Mutex::new(Inner {
            statuses: HashMap::from([(
                String::from("p"),
                ProjectWatcherStatus {
                    project_id: "p".into(),
                    state: "WATCHING".into(),
                    watcher_health: "HEALTHY".into(),
                    available: true,
                    last_event_at: None,
                    last_refresh_at: None,
                    evidence_generated_at: None,
                    changed_path_count: 0,
                    rescan_required: false,
                },
            )]),
            watches: HashMap::new(),
            watch_roots: HashMap::new(),
            watch_scopes: HashMap::new(),
            pending_count: 0,
            running: true,
            last_refresh_mono: HashMap::new(),
        }))
    }

    #[test]
    fn watcher_r01_r02_production_rescan_and_persistence_matrix() {
        let database_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Watcher Matrix".into()),
            },
        )
        .unwrap();
        let state = inner();
        state.lock().unwrap().statuses.insert(
            project.id.clone(),
            ProjectWatcherStatus {
                project_id: project.id.clone(),
                state: "WATCHING".into(),
                watcher_health: "OVERFLOW".into(),
                available: true,
                last_event_at: None,
                last_refresh_at: None,
                evidence_generated_at: None,
                changed_path_count: 1,
                rescan_required: true,
            },
        );
        let event = NormalizedEvent {
            event_id: Uuid::new_v4().to_string(),
            project_id: project.id.clone(),
            kind: NormalizedEventKind::Modify,
            relative_path: "src/lib.rs".into(),
            old_relative_path: None,
            timestamp: timestamp(),
            source: "test".into(),
            category_hint: EventCategory::Source,
        };
        refresh_project_snapshot(&database, &state, &project.id, vec![event], false).unwrap();
        assert!(state.lock().unwrap().statuses[&project.id].rescan_required);
        refresh_project_snapshot(&database, &state, &project.id, Vec::new(), false).unwrap();
        assert!(state.lock().unwrap().statuses[&project.id].rescan_required);
        refresh_project_snapshot(&database, &state, &project.id, Vec::new(), true).unwrap();
        assert!(!state.lock().unwrap().statuses[&project.id].rescan_required);
        let newest: i64 = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT rescan_required FROM project_snapshots WHERE project_id = ?1 ORDER BY created_at DESC LIMIT 1",
                [&project.id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(newest, 0);
    }

    #[test]
    fn m11a_r05_real_watcher_m09_m11_refresh_preserves_last_good_snapshot() {
        let database_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(
            project_dir.path().join("TASKS.md"),
            "# Work\n- [ ] first task\n",
        )
        .unwrap();
        fs::create_dir_all(project_dir.path().join(".hiveai")).unwrap();
        fs::write(
            project_dir.path().join(".hiveai/PROJECT_DASHBOARD.md"),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Source authorities\nCanonical task source: `TASKS.md`\n",
        )
        .unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("M11A watcher chain".into()),
            },
        )
        .unwrap();
        let state = inner();

        refresh_project_snapshot(&database, &state, &project.id, Vec::new(), true).unwrap();
        refresh_task_intelligence(&database, &project.id, true, None).unwrap();
        let good = crate::task_intelligence::list(&database, &project.id).unwrap();
        assert_eq!(good.tasks.len(), 1);
        assert_eq!(
            read_task_refresh_health(&database, &project.id)
                .unwrap()
                .unwrap()
                .status,
            "SUCCESS"
        );
        let initial = crate::command_center::snapshot(&database).unwrap();
        assert_eq!(initial.projects[0].total_tasks, Some(1));

        database
            .open_connection()
            .unwrap()
            .execute(
                "UPDATE projects SET status='MISSING' WHERE id=?1",
                [&project.id],
            )
            .unwrap();
        assert!(refresh_task_intelligence(&database, &project.id, false, None).is_err());
        let preserved = crate::task_intelligence::list(&database, &project.id).unwrap();
        assert_eq!(preserved.tasks.len(), 1);
        let degraded = read_task_refresh_health(&database, &project.id)
            .unwrap()
            .unwrap();
        assert_eq!(degraded.status, "DEGRADED");
        assert!(degraded.error.is_some());

        database
            .open_connection()
            .unwrap()
            .execute(
                "UPDATE projects SET status='ACTIVE' WHERE id=?1",
                [&project.id],
            )
            .unwrap();
        fs::write(
            project_dir.path().join("TASKS.md"),
            "# Work\n- [ ] refreshed task\n",
        )
        .unwrap();
        let event = NormalizedEvent {
            event_id: Uuid::new_v4().to_string(),
            project_id: project.id.clone(),
            kind: NormalizedEventKind::Modify,
            relative_path: "TASKS.md".into(),
            old_relative_path: None,
            timestamp: timestamp(),
            source: "WATCHER_TEST".into(),
            category_hint: EventCategory::TaskCandidate,
        };
        refresh_project_snapshot(&database, &state, &project.id, vec![event], false).unwrap();
        refresh_task_intelligence(&database, &project.id, false, None).unwrap();
        let refreshed = crate::task_intelligence::list(&database, &project.id).unwrap();
        assert_eq!(refreshed.tasks[0].title, "refreshed task");
        assert_eq!(
            read_task_refresh_health(&database, &project.id)
                .unwrap()
                .unwrap()
                .status,
            "SUCCESS"
        );
        let final_snapshot = crate::command_center::snapshot(&database).unwrap();
        assert_eq!(
            final_snapshot.projects[0]
                .current_task
                .as_ref()
                .unwrap()
                .title,
            "refreshed task"
        );
    }

    #[test]
    fn create_modify_remove_events_normalize() {
        let state = inner();
        let mut pending = HashMap::new();
        accept_input(
            &state,
            &mut pending,
            event("p", EventKind::Create(CreateKind::File), "src/new.rs"),
        );
        accept_input(
            &state,
            &mut pending,
            event(
                "p",
                EventKind::Modify(notify::event::ModifyKind::Data(
                    notify::event::DataChange::Content,
                )),
                "src/new.rs",
            ),
        );
        accept_input(
            &state,
            &mut pending,
            event("p", EventKind::Remove(RemoveKind::File), "src/old.rs"),
        );
        assert_eq!(pending.len(), 2);
        assert!(pending
            .values()
            .any(|item| matches!(item.event.kind, NormalizedEventKind::Create)));
        assert!(pending
            .values()
            .any(|item| matches!(item.event.kind, NormalizedEventKind::Remove)));
    }
    #[test]
    fn rapid_modifications_coalesce_to_one_path() {
        let state = inner();
        let mut pending = HashMap::new();
        for _ in 0..20 {
            accept_input(
                &state,
                &mut pending,
                event(
                    "p",
                    EventKind::Modify(notify::event::ModifyKind::Any),
                    "src/lib.rs",
                ),
            );
        }
        assert_eq!(pending.len(), 1);
        assert_eq!(state.lock().unwrap().statuses["p"].changed_path_count, 20);
    }
    #[test]
    fn excluded_noise_is_ignored_and_relative_paths_are_normalized() {
        let state = inner();
        let mut pending = HashMap::new();
        accept_input(
            &state,
            &mut pending,
            event(
                "p",
                EventKind::Modify(notify::event::ModifyKind::Any),
                "node_modules/pkg/index.js",
            ),
        );
        accept_input(
            &state,
            &mut pending,
            event(
                "p",
                EventKind::Modify(notify::event::ModifyKind::Any),
                "src\\main.rs",
            ),
        );
        assert_eq!(pending.len(), 1);
        assert_eq!(
            pending.values().next().unwrap().event.relative_path,
            "src/main.rs"
        );
    }
    #[test]
    fn bounded_queue_marks_rescan_required() {
        let state = inner();
        let mut pending = HashMap::new();
        for index in 0..=CHANNEL_CAPACITY {
            accept_input(
                &state,
                &mut pending,
                event(
                    "p",
                    EventKind::Create(CreateKind::File),
                    &format!("file-{index}.txt"),
                ),
            );
        }
        assert!(state.lock().unwrap().statuses["p"].rescan_required);
    }

    #[test]
    fn outside_root_paths_are_rejected_fail_closed() {
        let root = PathBuf::from("C:\\repo");
        assert_eq!(
            relative_path(Path::new("C:\\repo\\src\\main.rs"), &root),
            Some("src/main.rs".into())
        );
        assert_eq!(
            relative_path(Path::new("C:\\repository-sibling\\secret.txt"), &root),
            None
        );
        assert_eq!(
            relative_path(Path::new("C:\\repo\\..\\secret.txt"), &root),
            None
        );
        let rename = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Both)))
            .add_path(PathBuf::from("C:\\repo\\old.txt"))
            .add_path(PathBuf::from("C:\\outside\\new.txt"));
        assert!(normalize_event("p", &root, &rename).is_empty());
    }

    #[test]
    fn ordinary_refresh_preserves_overflow_until_explicit_rescan() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Rescan Fixture".into()),
            },
        )
        .unwrap();
        let state = inner();
        state.lock().unwrap().statuses.insert(
            project.id.clone(),
            ProjectWatcherStatus {
                project_id: project.id.clone(),
                state: "DEGRADED".into(),
                watcher_health: "OVERFLOW".into(),
                available: true,
                last_event_at: None,
                last_refresh_at: None,
                evidence_generated_at: None,
                changed_path_count: 1,
                rescan_required: true,
            },
        );
        refresh_project_snapshot(
            &database,
            &state,
            &project.id,
            vec![make_event(
                &project.id,
                NormalizedEventKind::Modify,
                "src/main.rs".into(),
                None,
                EventCategory::Source,
            )],
            false,
        )
        .unwrap();
        assert!(state.lock().unwrap().statuses[&project.id].rescan_required);
        refresh_project_snapshot(&database, &state, &project.id, Vec::new(), true).unwrap();
        assert!(!state.lock().unwrap().statuses[&project.id].rescan_required);
    }
    #[test]
    fn git_and_task_categories_are_hints_only() {
        assert!(matches!(
            category_hint(".git/HEAD"),
            EventCategory::GitMetadata
        ));
        assert!(matches!(
            category_hint("TASKS.md"),
            EventCategory::TaskCandidate
        ));
        assert!(matches!(
            category_hint(".hiveai/PROJECT_DASHBOARD.md"),
            EventCategory::TaskCandidate
        ));
        assert!(matches!(
            category_hint("src/main.rs"),
            EventCategory::Source
        ));
    }

    #[test]
    fn single_dashboard_filter_rejects_internal_source_events_and_accepts_atomic_replace() {
        let root = PathBuf::from("C:\\repo");
        let task_event = Event::new(EventKind::Modify(ModifyKind::Data(
            notify::event::DataChange::Content,
        )))
        .add_path(PathBuf::from("C:\\repo\\TASKS.md"));
        let source_event = Event::new(EventKind::Modify(ModifyKind::Data(
            notify::event::DataChange::Content,
        )))
        .add_path(PathBuf::from("C:\\repo\\src\\lib.rs"));
        let dashboard_event = Event::new(EventKind::Modify(ModifyKind::Data(
            notify::event::DataChange::Content,
        )))
        .add_path(PathBuf::from("C:\\repo\\.hiveai\\PROJECT_DASHBOARD.md"));
        assert!(normalize_event_with_mode("p", &root, &task_event, true).is_empty());
        assert!(normalize_event_with_mode("p", &root, &source_event, true).is_empty());
        assert_eq!(
            normalize_event_with_mode("p", &root, &dashboard_event, true)[0].relative_path,
            ".hiveai/PROJECT_DASHBOARD.md"
        );
        let replace = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Both)))
            .add_path(PathBuf::from("C:\\repo\\.hiveai\\PROJECT_DASHBOARD.tmp"))
            .add_path(PathBuf::from("C:\\repo\\.hiveai\\PROJECT_DASHBOARD.md"));
        assert_eq!(
            normalize_event_with_mode("p", &root, &replace, true)[0].relative_path,
            ".hiveai/PROJECT_DASHBOARD.md"
        );
    }

    #[test]
    fn migrated_project_attaches_single_dashboard_scope_and_refreshes_only_at_dashboard_signal() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        fs::create_dir_all(project_root.path().join(".hiveai")).unwrap();
        fs::write(
            project_root.path().join("TASKS.md"),
            "# Work\n- [ ] first task\n",
        )
        .unwrap();
        fs::write(
            project_root.path().join(MANIFEST_RELATIVE_PATH),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\nrefreshPolicy: project-agent-maintained; H!veAI watches only .hiveai/PROJECT_DASHBOARD.md\n## Source authorities\nCanonical task source: `TASKS.md`\n",
        )
        .unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Single dashboard watcher".into()),
            },
        )
        .unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        assert_eq!(
            manager.inner.lock().unwrap().watch_scopes[&project.id],
            "SINGLE_DASHBOARD"
        );
        manager.rescan_project(&project.id).unwrap();
        assert_eq!(
            crate::task_intelligence::list(&database, &project.id)
                .unwrap()
                .tasks[0]
                .title,
            "first task"
        );
        fs::write(
            project_root.path().join("TASKS.md"),
            "# Work\n- [ ] changed before dashboard signal\n",
        )
        .unwrap();
        manager
            .sender
            .try_send(RawInput {
                project_id: project.id.clone(),
                root: project_root.path().to_path_buf(),
                single_dashboard: true,
                event: Ok(Event::new(EventKind::Modify(ModifyKind::Data(
                    notify::event::DataChange::Content,
                )))
                .add_path(project_root.path().join("TASKS.md"))),
            })
            .unwrap();
        std::thread::sleep(Duration::from_millis(1000));
        assert_eq!(
            crate::task_intelligence::list(&database, &project.id)
                .unwrap()
                .tasks[0]
                .title,
            "first task"
        );
        fs::write(
            project_root.path().join(MANIFEST_RELATIVE_PATH),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\nrefreshPolicy: project-agent-maintained; H!veAI watches only .hiveai/PROJECT_DASHBOARD.md\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\n| Field | Value |\n| --- | --- |\n| Current task | changed at dashboard signal |\n",
        )
        .unwrap();
        manager
            .sender
            .try_send(RawInput {
                project_id: project.id.clone(),
                root: project_root.path().to_path_buf(),
                single_dashboard: true,
                event: Ok(Event::new(EventKind::Modify(ModifyKind::Data(
                    notify::event::DataChange::Content,
                )))
                .add_path(project_root.path().join(MANIFEST_RELATIVE_PATH))),
            })
            .unwrap();
        std::thread::sleep(Duration::from_millis(1400));
        assert_eq!(
            crate::task_intelligence::list(&database, &project.id)
                .unwrap()
                .tasks[0]
                .title,
            "changed before dashboard signal"
        );
        assert_eq!(
            crate::command_center::snapshot(&database).unwrap().projects[0].manifest_status,
            "VALID"
        );
    }

    #[test]
    fn live_dashboard_contract_changes_reconcile_watcher_scope_without_restart() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        fs::write(
            project_root.path().join("TASKS.md"),
            "# Work\n- [ ] legacy task\n",
        )
        .unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Live scope transition".into()),
            },
        )
        .unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        assert_eq!(
            manager.inner.lock().unwrap().watch_scopes[&project.id],
            "LEGACY_RECURSIVE"
        );
        manager.rescan_project(&project.id).unwrap();

        fs::create_dir_all(project_root.path().join(".hiveai")).unwrap();
        fs::write(
            project_root.path().join(MANIFEST_RELATIVE_PATH),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n",
        )
        .unwrap();
        manager
            .sender
            .try_send(RawInput {
                project_id: project.id.clone(),
                root: project_root.path().to_path_buf(),
                single_dashboard: false,
                event: Ok(Event::new(EventKind::Create(CreateKind::File))
                    .add_path(project_root.path().join(MANIFEST_RELATIVE_PATH))),
            })
            .unwrap();
        std::thread::sleep(Duration::from_millis(1200));
        assert_eq!(
            manager.inner.lock().unwrap().watch_scopes[&project.id],
            "SINGLE_DASHBOARD"
        );

        fs::write(
            project_root.path().join("TASKS.md"),
            "# Work\n- [ ] ignored while migrated\n",
        )
        .unwrap();
        manager
            .sender
            .try_send(RawInput {
                project_id: project.id.clone(),
                root: project_root.path().to_path_buf(),
                single_dashboard: true,
                event: Ok(Event::new(EventKind::Modify(ModifyKind::Data(
                    notify::event::DataChange::Content,
                )))
                .add_path(project_root.path().join("TASKS.md"))),
            })
            .unwrap();
        std::thread::sleep(Duration::from_millis(1000));
        assert_eq!(
            crate::task_intelligence::list(&database, &project.id)
                .unwrap()
                .tasks[0]
                .title,
            "legacy task"
        );

        fs::remove_file(project_root.path().join(MANIFEST_RELATIVE_PATH)).unwrap();
        manager
            .sender
            .try_send(RawInput {
                project_id: project.id.clone(),
                root: project_root.path().to_path_buf(),
                single_dashboard: true,
                event: Ok(Event::new(EventKind::Remove(RemoveKind::File))
                    .add_path(project_root.path().join(MANIFEST_RELATIVE_PATH))),
            })
            .unwrap();
        std::thread::sleep(Duration::from_millis(1200));
        assert_eq!(
            manager.inner.lock().unwrap().watch_scopes[&project.id],
            "LEGACY_RECURSIVE"
        );
        fs::write(
            project_root.path().join("TASKS.md"),
            "# Work\n- [ ] legacy resumed\n",
        )
        .unwrap();
        manager
            .sender
            .try_send(RawInput {
                project_id: project.id.clone(),
                root: project_root.path().to_path_buf(),
                single_dashboard: false,
                event: Ok(Event::new(EventKind::Modify(ModifyKind::Data(
                    notify::event::DataChange::Content,
                )))
                .add_path(project_root.path().join("TASKS.md"))),
            })
            .unwrap();
        std::thread::sleep(Duration::from_millis(1100));
        assert_eq!(
            crate::task_intelligence::list(&database, &project.id)
                .unwrap()
                .tasks[0]
                .title,
            "legacy resumed"
        );
        fs::create_dir_all(project_root.path().join(".hiveai")).unwrap();
        fs::write(
            project_root.path().join(MANIFEST_RELATIVE_PATH),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n",
        )
        .unwrap();
        manager.rescan_project(&project.id).unwrap();
        assert_eq!(
            manager.inner.lock().unwrap().watch_scopes[&project.id],
            "SINGLE_DASHBOARD"
        );
        manager.rescan_project(&project.id).unwrap();
        assert_eq!(
            manager.inner.lock().unwrap().watch_scopes[&project.id],
            "SINGLE_DASHBOARD"
        );
    }
    #[test]
    fn missing_status_is_degraded_without_registry_deletion() {
        let state = inner();
        mark_rescan(&state, "p");
        let status = state.lock().unwrap().statuses["p"].clone();
        assert_eq!(status.state, "DEGRADED");
        assert!(status.rescan_required);
    }
    #[test]
    fn shutdown_input_does_not_create_project_events() {
        let state = inner();
        let mut pending = HashMap::new();
        accept_input(
            &state,
            &mut pending,
            RawInput {
                project_id: String::new(),
                root: PathBuf::new(),
                single_dashboard: false,
                event: Err(notify::Error::generic("shutdown")),
            },
        );
        assert!(pending.is_empty());
    }
    #[test]
    fn temporary_root_can_be_created_without_watcher_side_effects() {
        let directory = tempdir().unwrap();
        fs::write(directory.path().join("TASKS.md"), "not parsed").unwrap();
        assert!(directory.path().join("TASKS.md").exists());
    }

    #[test]
    fn notify_backend_receives_temporary_modify_event() {
        let directory = tempdir().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(
            move |event| {
                let _ = sender.send(event);
            },
            Config::default(),
        )
        .unwrap();
        watcher
            .watch(directory.path(), RecursiveMode::Recursive)
            .unwrap();
        fs::write(directory.path().join("event.txt"), "created").unwrap();
        let received = receiver
            .recv_timeout(Duration::from_secs(3))
            .expect("watcher event")
            .unwrap();
        assert!(received
            .paths
            .iter()
            .any(|path| path.ends_with("event.txt")));
    }

    #[test]
    fn manager_marks_missing_root_without_deleting_registry_row() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Watcher Fixture".into()),
            },
        )
        .unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        assert_eq!(
            manager.project_status(&project.id).unwrap().state,
            "WATCHING"
        );
        fs::remove_dir_all(project_root.path()).unwrap();
        let status = manager.rescan_project(&project.id).unwrap();
        assert_eq!(status.state, "MISSING");
        assert_eq!(
            crate::projects::fetch_project(&database, &project.id)
                .unwrap()
                .status,
            "MISSING"
        );
    }

    #[test]
    fn watcher_attachment_failure_preserves_rescan_requirement() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Attach Fail".into()),
            },
        )
        .unwrap();
        manager.inner.lock().unwrap().statuses.insert(
            project.id.clone(),
            ProjectWatcherStatus {
                project_id: project.id.clone(),
                state: "DEGRADED".into(),
                watcher_health: "DEGRADED".into(),
                available: true,
                last_event_at: None,
                last_refresh_at: None,
                evidence_generated_at: None,
                changed_path_count: 0,
                rescan_required: true,
            },
        );
        FAIL_NEXT_WATCH_ATTACH.with(|failpoint| failpoint.set(true));
        assert!(manager.refresh_from_registry().is_err());
        let state = manager.inner.lock().unwrap();
        assert!(state.watches.is_empty());
        assert!(state.watch_roots.is_empty());
        assert!(state.statuses[&project.id].rescan_required);
    }

    #[test]
    fn watcher_git_refresh_failure_preserves_rescan_requirement() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        init_git_repo(project_root.path());
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Git Refresh Fail".into()),
            },
        )
        .unwrap();
        let s = inner();
        s.lock().unwrap().statuses.insert(
            project.id.clone(),
            ProjectWatcherStatus {
                project_id: project.id.clone(),
                state: "DEGRADED".into(),
                watcher_health: "OVERFLOW".into(),
                available: true,
                last_event_at: None,
                last_refresh_at: None,
                evidence_generated_at: None,
                changed_path_count: 0,
                rescan_required: true,
            },
        );
        let git_event = make_event(
            &project.id,
            NormalizedEventKind::Modify,
            ".git/HEAD".into(),
            None,
            EventCategory::GitMetadata,
        );
        FAIL_NEXT_GIT_REFRESH.with(|failpoint| failpoint.set(true));
        assert!(
            refresh_project_snapshot(&database, &s, &project.id, vec![git_event], false).is_err()
        );
        assert!(s.lock().unwrap().statuses[&project.id].rescan_required);
        assert_eq!(
            database
                .open_connection()
                .unwrap()
                .query_row::<i64, _, _>(
                    "SELECT COUNT(*) FROM project_snapshots WHERE project_id = ?1",
                    [&project.id],
                    |row| row.get(0)
                )
                .unwrap(),
            0
        );
    }

    #[test]
    fn watcher_snapshot_persistence_failure_preserves_rescan_requirement() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Persist Fail".into()),
            },
        )
        .unwrap();
        let s = inner();
        s.lock().unwrap().statuses.insert(
            project.id.clone(),
            ProjectWatcherStatus {
                project_id: project.id.clone(),
                state: "DEGRADED".into(),
                watcher_health: "OVERFLOW".into(),
                available: true,
                last_event_at: None,
                last_refresh_at: None,
                evidence_generated_at: None,
                changed_path_count: 0,
                rescan_required: true,
            },
        );
        let event = make_event(
            &project.id,
            NormalizedEventKind::Modify,
            "src/lib.rs".into(),
            None,
            EventCategory::Source,
        );
        FAIL_NEXT_SNAPSHOT_PERSISTENCE.with(|failpoint| failpoint.set(true));
        assert!(refresh_project_snapshot(&database, &s, &project.id, vec![event], false).is_err());
        assert!(s.lock().unwrap().statuses[&project.id].rescan_required);
    }

    #[test]
    fn watcher_healthy_explicit_rescan_no_prior_requirement_deterministic() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Healthy Rescan".into()),
            },
        )
        .unwrap();
        let s = inner();
        s.lock().unwrap().statuses.insert(
            project.id.clone(),
            ProjectWatcherStatus {
                project_id: project.id.clone(),
                state: "WATCHING".into(),
                watcher_health: "HEALTHY".into(),
                available: true,
                last_event_at: None,
                last_refresh_at: None,
                evidence_generated_at: None,
                changed_path_count: 0,
                rescan_required: false,
            },
        );
        refresh_project_snapshot(&database, &s, &project.id, Vec::new(), true).unwrap();
        let status = s.lock().unwrap().statuses[&project.id].clone();
        assert!(!status.rescan_required);
        assert_eq!(status.watcher_health, "HEALTHY");
    }

    #[test]
    fn watcher_positive_in_root_rename_preserves_old_new_paths() {
        let root = tempdir().unwrap();
        let old_path = root.path().join("old.txt");
        let new_path = root.path().join("new.txt");
        fs::write(&old_path, "data").unwrap();
        fs::rename(&old_path, &new_path).unwrap();
        let event = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Both)))
            .add_path(old_path)
            .add_path(new_path);
        let normalized = normalize_event("p", root.path(), &event);
        assert_eq!(normalized.len(), 1);
        assert_eq!(normalized[0].relative_path, "new.txt");
        assert_eq!(normalized[0].old_relative_path.as_deref(), Some("old.txt"));
    }

    #[test]
    fn watcher_unchanged_refresh_does_not_duplicate_watcher() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Dup Check".into()),
            },
        )
        .unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        let before = manager.inner.lock().unwrap();
        let before_watch_count = before.watches.len();
        let before_root = before.watch_roots.get(&project.id).cloned();
        drop(before);
        manager.refresh_from_registry().unwrap();
        let after = manager.inner.lock().unwrap();
        assert_eq!(
            after.watches.len(),
            before_watch_count,
            "refresh must not duplicate watchers"
        );
        assert_eq!(after.watch_roots.get(&project.id).cloned(), before_root);
    }

    #[test]
    fn watcher_repaired_root_refresh_reattaches_to_new_root() {
        let app_data = tempdir().unwrap();
        let old_root = tempdir().unwrap();
        let new_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: old_root.path().to_string_lossy().into_owned(),
                name: Some("Repair Root".into()),
            },
        )
        .unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        assert_eq!(
            manager.project_status(&project.id).unwrap().state,
            "WATCHING"
        );
        crate::projects::repair_project_path(
            &database,
            crate::projects::RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: new_root.path().to_string_lossy().into_owned(),
            },
        )
        .unwrap();
        manager.refresh_from_registry().unwrap();
        let state = manager.inner.lock().unwrap();
        assert_eq!(
            state.watch_roots[&project.id]
                .to_string_lossy()
                .to_ascii_lowercase(),
            std::fs::canonicalize(new_root.path())
                .unwrap()
                .to_string_lossy()
                .to_ascii_lowercase()
        );
        assert_eq!(state.watches.len(), 1);
        assert!(!state.watch_roots.values().any(|root| {
            root.to_string_lossy().to_ascii_lowercase()
                == std::fs::canonicalize(old_root.path())
                    .unwrap()
                    .to_string_lossy()
                    .to_ascii_lowercase()
        }));
    }

    #[test]
    fn watcher_event_on_repaired_root_is_observed() {
        let app_data = tempdir().unwrap();
        let old_root = tempdir().unwrap();
        let new_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: old_root.path().to_string_lossy().into_owned(),
                name: Some("Repaired Event".into()),
            },
        )
        .unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        crate::projects::repair_project_path(
            &database,
            crate::projects::RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: new_root.path().to_string_lossy().into_owned(),
            },
        )
        .unwrap();
        manager.refresh_from_registry().unwrap();
        let before_events = manager.project_status(&project.id).unwrap().last_event_at;
        let before_snapshots: i64 = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT COUNT(*) FROM project_snapshots WHERE project_id = ?1",
                [&project.id],
                |row| row.get(0),
            )
            .unwrap();
        fs::write(new_root.path().join("event.txt"), "observed").unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            let status = manager.project_status(&project.id).unwrap();
            let snapshots: i64 = database
                .open_connection()
                .unwrap()
                .query_row(
                    "SELECT COUNT(*) FROM project_snapshots WHERE project_id = ?1",
                    [&project.id],
                    |row| row.get(0),
                )
                .unwrap();
            if status.last_event_at != before_events && snapshots > before_snapshots {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "new-root event was not observed"
            );
            thread::sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn watcher_git_category_event_persists_snapshot() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        init_git_repo(project_root.path());
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Git Snap".into()),
            },
        )
        .unwrap();
        let s = inner();
        s.lock().unwrap().statuses.insert(
            project.id.clone(),
            ProjectWatcherStatus {
                project_id: project.id.clone(),
                state: "WATCHING".into(),
                watcher_health: "HEALTHY".into(),
                available: true,
                last_event_at: None,
                last_refresh_at: None,
                evidence_generated_at: None,
                changed_path_count: 0,
                rescan_required: false,
            },
        );
        let git_event = make_event(
            &project.id,
            NormalizedEventKind::Modify,
            ".git/HEAD".into(),
            None,
            EventCategory::GitMetadata,
        );
        refresh_project_snapshot(&database, &s, &project.id, vec![git_event], false).unwrap();
        let (count, linked): (i64, i64) = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT COUNT(*), COUNT(git_snapshot_id) FROM project_snapshots WHERE project_id = ?1",
                [&project.id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert!(count >= 1, "Git event must persist a snapshot");
        assert!(
            linked >= 1,
            "Git event snapshot must reference persisted Git snapshot"
        );
    }

    #[test]
    fn watcher_non_git_event_does_not_persist_git_snapshot() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        init_git_repo(project_root.path());
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("No Git Snap".into()),
            },
        )
        .unwrap();
        let s = inner();
        s.lock().unwrap().statuses.insert(
            project.id.clone(),
            ProjectWatcherStatus {
                project_id: project.id.clone(),
                state: "WATCHING".into(),
                watcher_health: "HEALTHY".into(),
                available: true,
                last_event_at: None,
                last_refresh_at: None,
                evidence_generated_at: None,
                changed_path_count: 0,
                rescan_required: false,
            },
        );
        let source_event = make_event(
            &project.id,
            NormalizedEventKind::Modify,
            "src/lib.rs".into(),
            None,
            EventCategory::Source,
        );
        let before: i64 = database.open_connection().unwrap().query_row("SELECT COUNT(*) FROM git_snapshots WHERE repository_id IN (SELECT id FROM repositories WHERE project_id = ?1)", [&project.id], |row| row.get(0)).unwrap();
        refresh_project_snapshot(&database, &s, &project.id, vec![source_event], false).unwrap();
        let git_count: i64 = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT COUNT(*) FROM git_snapshots WHERE repository_id IN (SELECT id FROM repositories WHERE project_id = ?1)",
                [&project.id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            git_count, before,
            "non-Git event must not create git snapshot"
        );
    }

    #[test]
    fn watcher_missing_root_initialization_stays_degraded_preserves_row() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Missing Init".into()),
            },
        )
        .unwrap();
        fs::remove_dir_all(project_root.path()).unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        let status = manager.project_status(&project.id).unwrap();
        assert_eq!(status.state, "MISSING");
        assert_eq!(status.watcher_health, "DEGRADED");
        assert!(crate::projects::fetch_project(&database, &project.id).is_ok());
    }

    #[test]
    fn watcher_manager_drop_releases_worker_and_watchers() {
        let app_data = tempdir().unwrap();
        let project_root = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_root.path().to_string_lossy().into_owned(),
                name: Some("Drop Test".into()),
            },
        )
        .unwrap();
        let manager = WatcherManager::initialize(database.clone()).unwrap();
        assert!(manager.status().running);
        let retained = Arc::clone(&manager.inner);
        drop(manager);
        let state = retained.lock().unwrap();
        assert!(!state.running);
        assert!(state.watches.is_empty());
        assert!(state.watch_roots.is_empty());
    }

    #[test]
    fn watcher_symlink_escape_rejected_by_physical_containment() {
        let root = tempdir().unwrap();
        let outside = tempdir().unwrap();
        let real_file = outside.path().join("secret.txt");
        fs::write(&real_file, "secret").unwrap();
        let inside = root.path().join("src");
        fs::create_dir_all(&inside).unwrap();
        let link = inside.join("escape.txt");
        let result = {
            #[cfg(windows)]
            {
                std::os::windows::fs::symlink_file(&real_file, &link)
            }
            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&real_file, &link)
            }
        };
        if let Err(error) = result {
            println!("UNVERIFIED — link creation denied by environment: {error}");
            return;
        }
        assert!(!physically_contained(&link, root.path()));
    }
}
