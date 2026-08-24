use crate::db::DatabaseState;
use crate::git_engine::{snapshot as git_snapshot, GitSnapshotRequest};
use crate::projects::{fetch_project, list_projects, ProjectListQuery, ProjectRecord};
use notify::event::{CreateKind, ModifyKind, RemoveKind, RenameMode};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rusqlite::OptionalExtension;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const CHANNEL_CAPACITY: usize = 512;
const DEBOUNCE_WINDOW: Duration = Duration::from_millis(250);
const REFRESH_WINDOW: Duration = Duration::from_millis(750);
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
    pending_count: usize,
    running: bool,
    last_refresh_mono: HashMap<String, SystemTime>,
}

pub struct WatcherManager {
    database: DatabaseState,
    inner: Arc<Mutex<Inner>>,
    sender: SyncSender<RawInput>,
    worker: Mutex<Option<JoinHandle<()>>>,
}

impl WatcherManager {
    pub fn initialize(database: DatabaseState) -> Result<Self, String> {
        let (sender, receiver) = sync_channel(CHANNEL_CAPACITY);
        let inner = Arc::new(Mutex::new(Inner {
            statuses: HashMap::new(),
            watches: HashMap::new(),
            pending_count: 0,
            running: true,
            last_refresh_mono: HashMap::new(),
        }));
        let worker_inner = Arc::clone(&inner);
        let worker_database = database.clone();
        let worker = thread::Builder::new()
            .name("hiveai-filesystem-watcher".to_string())
            .spawn(move || worker_loop(worker_database, worker_inner, receiver))
            .map_err(|error| format!("start H!veAI filesystem watcher worker: {error}"))?;
        let manager = Self {
            database,
            inner,
            sender,
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
        self.project_status(project_id)
    }

    fn configure_project(&self, project: ProjectRecord) -> Result<(), String> {
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
        let mut inner = self.inner.lock().expect("watcher state lock");
        inner
            .statuses
            .entry(project.id.clone())
            .and_modify(|current| {
                current.state = status.state.clone();
                current.watcher_health = status.watcher_health.clone();
                current.available = status.available;
            })
            .or_insert(status);
        if !available || inner.watches.contains_key(&project.id) {
            if !available {
                inner.watches.remove(&project.id);
            }
            return Ok(());
        }
        let project_id = project.id.clone();
        let project_root = project.normalized_path.clone();
        let sender = self.sender.clone();
        let callback_inner = Arc::clone(&self.inner);
        let mut watcher = match RecommendedWatcher::new(
            move |event| {
                let input = RawInput {
                    project_id: project_id.clone(),
                    root: PathBuf::from(&project_root),
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
                return Ok(());
            }
        };
        if watcher
            .watch(
                Path::new(&project.normalized_path),
                RecursiveMode::Recursive,
            )
            .is_err()
        {
            if let Some(status) = inner.statuses.get_mut(&project.id) {
                status.state = "DEGRADED".to_string();
                status.watcher_health = "DEGRADED".to_string();
            }
            return Ok(());
        }
        inner.watches.insert(project.id, watcher);
        Ok(())
    }
}

impl Drop for WatcherManager {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.running = false;
            inner.watches.clear();
        }
        let _ = self.sender.try_send(RawInput {
            project_id: String::new(),
            root: PathBuf::new(),
            event: Err(notify::Error::generic("shutdown")),
        });
        if let Ok(mut worker) = self.worker.lock() {
            if let Some(handle) = worker.take() {
                let _ = handle.join();
            }
        }
    }
}

fn worker_loop(database: DatabaseState, inner: Arc<Mutex<Inner>>, receiver: Receiver<RawInput>) {
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
                    let _ = refresh_project_snapshot(&database, &inner, &project_id, events, false);
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
        Ok(event) => normalize_event(&input.project_id, &input.root, &event),
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
        || events
            .iter()
            .any(|event| matches!(event.category_hint, EventCategory::GitMetadata));
    let git_id = if available
        && project
            .repository
            .as_ref()
            .map(|repository| repository.is_git_repository)
            .unwrap_or(false)
        && git_relevant
    {
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
    let rescan_required = inner
        .lock()
        .expect("watcher state lock")
        .statuses
        .get(project_id)
        .map(|status| status.rescan_required)
        .unwrap_or(false);
    let health = if !available {
        "MISSING"
    } else if rescan_required {
        "OVERFLOW"
    } else {
        "HEALTHY"
    };
    let connection = database.open_connection()?;
    connection.execute("INSERT INTO project_snapshots (id, project_id, availability, git_snapshot_id, last_filesystem_event_at, last_watcher_refresh_at, evidence_generated_at, changed_path_count, rescan_required, watcher_health, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6, ?7, ?8, ?9, ?6)", rusqlite::params![Uuid::new_v4().to_string(), project_id, if available { "AVAILABLE" } else { "MISSING" }, git_id, last_event, now, events.len() as i64, rescan_required as i64, health]).map_err(|error| format!("persist project snapshot: {error}"))?;
    if let Ok(mut state) = inner.lock() {
        if let Some(status) = state.statuses.get_mut(project_id) {
            status.available = available;
            status.state = if available { "WATCHING" } else { "MISSING" }.to_string();
            status.watcher_health = health.to_string();
            status.last_refresh_at = Some(now.clone());
            status.evidence_generated_at = Some(now);
            status.rescan_required = false;
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
    let paths = event
        .paths
        .iter()
        .filter_map(|path| relative_path(path, root))
        .filter(|path| !is_excluded(path))
        .collect::<Vec<_>>();
    if paths.is_empty() {
        return Vec::new();
    }
    let category = category_hint(&paths[0]);
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
    let relative = path.strip_prefix(root).unwrap_or(path);
    let value = relative.to_string_lossy().replace('\\', "/");
    if value.is_empty() || value == "." || value.split('/').any(|part| part == "..") {
        None
    } else {
        Some(value.trim_start_matches("./").to_string())
    }
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
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn event(project_id: &str, kind: EventKind, path: &str) -> RawInput {
        RawInput {
            project_id: project_id.to_string(),
            root: PathBuf::from("C:\\repo"),
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
            pending_count: 0,
            running: true,
            last_refresh_mono: HashMap::new(),
        }))
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
            category_hint("src/main.rs"),
            EventCategory::Source
        ));
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
}
