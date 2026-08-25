use crate::db::DatabaseState;
use crate::projects::fetch_project;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
#[cfg(test)]
use std::sync::{Mutex, OnceLock};

pub const MAX_VISITED_ENTRIES: usize = 4096;

pub const MAX_DISCOVERY_DEPTH: usize = 4;
pub const MAX_CANDIDATE_FILES: usize = 512;
pub const MAX_SOURCE_BYTES: u64 = 2 * 1024 * 1024;
pub const MAX_CUSTOM_PATHS: usize = 64;

const IGNORE_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "dist",
    "build",
    "target",
    ".next",
    "coverage",
    ".cache",
    ".venv",
    "venv",
    "vendor",
];
const ROOT_NAMES: &[&str] = &[
    "tasks.md",
    "task.md",
    "plans.md",
    "plan.md",
    "progress.md",
    "roadmap.md",
    "agents.md",
    "claude.md",
    "handoff.md",
    "session_handoff.md",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredProjectSource {
    pub id: String,
    pub project_id: String,
    pub relative_path: String,
    pub absolute_path: String,
    pub source_kind: String,
    pub origin: String,
    pub status: String,
    pub authority_class: String,
    pub priority: i64,
    pub size_bytes: Option<u64>,
    pub modified_at: Option<String>,
    pub discovered_at: String,
    pub content_hash: Option<String>,
    pub depth: usize,
    pub warnings: Vec<String>,
    #[serde(default)]
    pub schema_version: u32,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub source_order: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CustomSourcePath {
    pub id: String,
    pub project_id: String,
    pub display_path: String,
    pub normalized_path: String,
    pub status: String,
    pub order: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomPathRequest {
    pub project_id: String,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomPathUpdateRequest {
    pub project_id: String,
    pub path_or_id: String,
    pub path: Option<String>,
    pub order: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredCustomPath {
    id: String,
    display_path: String,
    normalized_path: String,
    #[serde(default)]
    order: i64,
}

struct DiscoveryBudget {
    visited_entries: usize,
    candidate_limit: bool,
    work_limit: bool,
    depth_limit: bool,
    warnings: BTreeSet<String>,
}

impl DiscoveryBudget {
    fn new() -> Self {
        Self {
            visited_entries: 0,
            candidate_limit: false,
            work_limit: false,
            depth_limit: false,
            warnings: BTreeSet::new(),
        }
    }

    fn visit(&mut self) -> bool {
        if self.visited_entries >= MAX_VISITED_ENTRIES {
            self.work_limit = true;
            self.warnings.insert(format!(
                "visited-entry limit reached ({MAX_VISITED_ENTRIES})"
            ));
            return false;
        }
        self.visited_entries += 1;
        true
    }

    fn candidate_available(&mut self, current: usize) -> bool {
        if current >= MAX_CANDIDATE_FILES {
            self.candidate_limit = true;
            self.warnings
                .insert(format!("candidate limit reached ({MAX_CANDIDATE_FILES})"));
            return false;
        }
        true
    }

    fn depth_allowed(&mut self, depth: usize) -> bool {
        if depth > MAX_DISCOVERY_DEPTH {
            self.depth_limit = true;
            self.warnings
                .insert(format!("depth limit reached ({MAX_DISCOVERY_DEPTH})"));
            return false;
        }
        true
    }
}

pub fn discover(
    database: &DatabaseState,
    project_id: &str,
) -> Result<Vec<DiscoveredProjectSource>, String> {
    let project = discovery_project(database, project_id)?;
    let root = physical_root(Path::new(&project.normalized_path))?;
    let custom = load_custom_paths(database, project_id)?;
    let now = crate::time::utc_timestamp();
    let mut candidates = Vec::new();
    let mut budget = DiscoveryBudget::new();
    discover_standard(&root, project_id, &now, &mut candidates, &mut budget);
    for path in &custom {
        discover_custom(&root, path, project_id, &now, &mut candidates, &mut budget);
    }
    candidates.sort_by(|a, b| source_sort_key(a).cmp(&source_sort_key(b)));
    if !budget.warnings.is_empty() {
        candidates.push(discovery_warning(&root, project_id, &now, &budget.warnings));
    }
    reconcile(database, project_id, &candidates)?;
    list(database, project_id)
}

pub fn list(
    database: &DatabaseState,
    project_id: &str,
) -> Result<Vec<DiscoveredProjectSource>, String> {
    let project = discovery_project(database, project_id)?;
    if !Path::new(&project.normalized_path).exists() {
        return Err("registered project root is unavailable".to_string());
    }
    let connection = database.open_connection()?;
    let mut statement = connection
        .prepare("SELECT id, project_id, source_path, source_kind, content_hash, discovered_at, metadata_json FROM project_sources WHERE project_id = ?1")
        .map_err(db_error)?;
    let mut rows = statement.query([project_id]).map_err(db_error)?;
    let mut result = Vec::new();
    while let Some(row) = rows.next().map_err(db_error)? {
        let metadata: String = row.get(6).map_err(db_error)?;
        let Ok(mut source) = serde_json::from_str::<DiscoveredProjectSource>(&metadata) else {
            continue;
        };
        source.id = row.get(0).map_err(db_error)?;
        source.project_id = row.get(1).map_err(db_error)?;
        source.relative_path = row.get(2).map_err(db_error)?;
        source.source_kind = row.get(3).map_err(db_error)?;
        source.content_hash = row.get(4).map_err(db_error)?;
        source.discovered_at = row.get(5).map_err(db_error)?;
        result.push(source);
    }
    result.sort_by(|a, b| source_sort_key(a).cmp(&source_sort_key(b)));
    Ok(result)
}

pub fn custom_paths_list(
    database: &DatabaseState,
    project_id: &str,
) -> Result<Vec<CustomSourcePath>, String> {
    let root = physical_root(Path::new(
        &discovery_project(database, project_id)?.normalized_path,
    ))?;
    Ok(load_custom_paths(database, project_id)?
        .into_iter()
        .map(|path| CustomSourcePath {
            id: path.id,
            project_id: project_id.to_string(),
            status: path_status(&root, &path.normalized_path),
            display_path: path.display_path,
            normalized_path: path.normalized_path,
            order: path.order,
        })
        .collect())
}

pub fn custom_path_add(
    database: &DatabaseState,
    request: CustomPathRequest,
) -> Result<Vec<CustomSourcePath>, String> {
    let project = discovery_project(database, &request.project_id)?;
    let root = physical_root(Path::new(&project.normalized_path))?;
    let normalized = normalize_candidate(&root, &request.path)?;
    let mut paths = load_custom_paths(database, &request.project_id)?;
    let same_path = |path: &StoredCustomPath| {
        normalize_for_compare(&path.normalized_path) == normalize_for_compare(&normalized)
    };
    if paths.len() >= MAX_CUSTOM_PATHS && !paths.iter().any(same_path) {
        return Err(format!(
            "custom source path limit reached ({MAX_CUSTOM_PATHS})"
        ));
    }
    if !paths.iter().any(same_path) {
        paths.push(StoredCustomPath {
            id: stable_id(&format!("{}|CUSTOM|{}", request.project_id, normalized)),
            display_path: request.path.trim().to_string(),
            normalized_path: normalized,
            order: paths.iter().map(|path| path.order).max().unwrap_or(-1) + 1,
        });
        save_custom_paths(database, &request.project_id, &paths)?;
    }
    custom_paths_list(database, &request.project_id)
}

pub fn custom_path_remove(
    database: &DatabaseState,
    project_id: &str,
    path_or_id: &str,
) -> Result<Vec<CustomSourcePath>, String> {
    discovery_project(database, project_id)?;
    let mut paths = load_custom_paths(database, project_id)?;
    let before = paths.len();
    let normalized_target = normalize_for_compare(path_or_id);
    paths.retain(|path| {
        path.id != path_or_id && normalize_for_compare(&path.normalized_path) != normalized_target
    });
    if paths.len() == before {
        return Err("custom source path is not configured".to_string());
    }
    save_custom_paths(database, project_id, &paths)?;
    custom_paths_list(database, project_id)
}

pub fn custom_path_update(
    database: &DatabaseState,
    request: CustomPathUpdateRequest,
) -> Result<Vec<CustomSourcePath>, String> {
    let project = discovery_project(database, &request.project_id)?;
    let root = physical_root(Path::new(&project.normalized_path))?;
    let mut paths = load_custom_paths(database, &request.project_id)?;
    let index = paths
        .iter()
        .position(|path| {
            path.id == request.path_or_id
                || normalize_for_compare(&path.normalized_path)
                    == normalize_for_compare(&request.path_or_id)
        })
        .ok_or_else(|| "custom source path is not configured".to_string())?;
    if let Some(new_path) = request.path.as_deref() {
        let normalized = normalize_candidate(&root, new_path)?;
        if paths.iter().enumerate().any(|(candidate, path)| {
            candidate != index
                && normalize_for_compare(&path.normalized_path)
                    == normalize_for_compare(&normalized)
        }) {
            return Err("custom source path is already configured".into());
        }
        paths[index].display_path = new_path.trim().to_string();
        paths[index].normalized_path = normalized;
        paths[index].id = stable_id(&format!(
            "{}|CUSTOM|{}",
            request.project_id, paths[index].normalized_path
        ));
    }
    if let Some(order) = request.order {
        paths[index].order = order.max(0);
    }
    paths.sort_by_key(|path| (path.order, normalize_for_compare(&path.normalized_path)));
    for (order, path) in paths.iter_mut().enumerate() {
        path.order = order as i64;
    }
    save_custom_paths(database, &request.project_id, &paths)?;
    custom_paths_list(database, &request.project_id)
}

fn discover_standard(
    root: &Path,
    project_id: &str,
    now: &str,
    output: &mut Vec<DiscoveredProjectSource>,
    budget: &mut DiscoveryBudget,
) {
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            if !budget.visit() {
                break;
            }
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|value| value.to_str()) {
                let lower = name.to_ascii_lowercase();
                if ROOT_NAMES.contains(&lower.as_str())
                    || (lower.contains("handoff") && lower.ends_with(".md"))
                {
                    collect_file(
                        root, &path, project_id, "STANDARD", now, output, 0, None, None, budget,
                    );
                }
            }
        }
    }
    for directory in ["tasks", "plans", "handoffs", ".hiveai"] {
        let path = root.join(directory);
        if path.is_dir() {
            walk_bounded(
                root, &path, project_id, "STANDARD", now, output, 0, None, None, budget,
            );
        }
    }
}

fn discover_custom(
    root: &Path,
    custom: &StoredCustomPath,
    project_id: &str,
    now: &str,
    output: &mut Vec<DiscoveredProjectSource>,
    budget: &mut DiscoveryBudget,
) {
    let path = root.join(&custom.normalized_path);
    if path.exists() {
        if path.is_dir() {
            walk_bounded(
                root,
                &path,
                project_id,
                "CUSTOM",
                now,
                output,
                0,
                Some(&custom.id),
                Some(custom.order),
                budget,
            );
        } else {
            collect_file(
                root,
                &path,
                project_id,
                "CUSTOM",
                now,
                output,
                path_depth(root, &path),
                Some(&custom.id),
                Some(custom.order),
                budget,
            );
        }
    } else {
        output.push(source_for_missing(
            root,
            &custom.normalized_path,
            project_id,
            now,
            &custom.id,
            custom.order,
        ));
    }
}

fn walk_bounded(
    root: &Path,
    directory: &Path,
    project_id: &str,
    origin: &str,
    now: &str,
    output: &mut Vec<DiscoveredProjectSource>,
    depth: usize,
    custom_id: Option<&str>,
    source_order: Option<i64>,
    budget: &mut DiscoveryBudget,
) {
    if !budget.depth_allowed(depth) || !budget.candidate_available(output.len()) {
        return;
    }
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        if !budget.visit() {
            break;
        }
        if !budget.candidate_available(output.len()) {
            break;
        }
        let path = entry.path();
        let Ok(physical) = fs::canonicalize(&path) else {
            continue;
        };
        if ensure_contained(root, &physical).is_err() {
            continue;
        }
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| IGNORE_DIRS.contains(&name.to_ascii_lowercase().as_str()))
                .unwrap_or(false)
            {
                continue;
            }
            if depth >= MAX_DISCOVERY_DEPTH {
                budget.depth_limit = true;
                budget
                    .warnings
                    .insert(format!("depth limit reached ({MAX_DISCOVERY_DEPTH})"));
                continue;
            }
            walk_bounded(
                root,
                &path,
                project_id,
                origin,
                now,
                output,
                depth + 1,
                custom_id,
                source_order,
                budget,
            );
        } else if path.is_file() && is_plausible_source(&path) {
            collect_file(
                root,
                &path,
                project_id,
                origin,
                now,
                output,
                path_depth(root, &path),
                custom_id,
                source_order,
                budget,
            );
        }
    }
}

fn collect_file(
    root: &Path,
    path: &Path,
    project_id: &str,
    origin: &str,
    now: &str,
    output: &mut Vec<DiscoveredProjectSource>,
    _depth: usize,
    custom_id: Option<&str>,
    source_order: Option<i64>,
    budget: &mut DiscoveryBudget,
) {
    let actual_depth = path_depth(root, path);
    if actual_depth > MAX_DISCOVERY_DEPTH {
        budget.depth_limit = true;
        budget
            .warnings
            .insert(format!("depth limit reached ({MAX_DISCOVERY_DEPTH})"));
        return;
    }
    if !budget.candidate_available(output.len()) || !is_plausible_source(path) {
        return;
    }
    let Ok(physical) = fs::canonicalize(path) else {
        return;
    };
    if ensure_contained(root, &physical).is_err() || !physical.is_file() {
        return;
    }
    let Ok(relative) = path.strip_prefix(root) else {
        return;
    };
    let relative_path = slash_path(relative);
    let metadata = match fs::metadata(&physical) {
        Ok(value) => value,
        Err(_) => {
            output.push(source_with_status(
                root,
                project_id,
                &relative_path,
                origin,
                now,
                "UNREADABLE",
                "OTHER_TASK_SOURCE".to_string(),
                0,
                custom_id,
                None,
                None,
                vec!["metadata unavailable".into()],
                source_order,
            ));
            return;
        }
    };
    let size = metadata.len();
    let modified = metadata
        .modified()
        .ok()
        .map(|value| chrono::DateTime::<chrono::Utc>::from(value).to_rfc3339());
    if size > MAX_SOURCE_BYTES {
        output.push(source_with_status(
            root,
            project_id,
            &relative_path,
            origin,
            now,
            "TOO_LARGE",
            classify(path).0,
            classify(path).1,
            custom_id,
            Some(size),
            modified,
            vec![format!(
                "source exceeds {MAX_SOURCE_BYTES} byte hash/read limit"
            )],
            source_order,
        ));
        return;
    }
    let hash = match bounded_hash(&physical) {
        Ok(value) => Some(value),
        Err(_) => {
            output.push(source_with_status(
                root,
                project_id,
                &relative_path,
                origin,
                now,
                "UNREADABLE",
                classify(path).0,
                classify(path).1,
                custom_id,
                Some(size),
                modified,
                vec!["source could not be read".into()],
                source_order,
            ));
            return;
        }
    };
    let (kind, priority) = if origin == "CUSTOM" {
        ("CUSTOM".to_string(), 0)
    } else {
        let (kind, priority) = classify(path);
        (kind, priority)
    };
    output.push(
        source_with_status(
            root,
            project_id,
            &relative_path,
            origin,
            now,
            "AVAILABLE",
            kind,
            priority,
            custom_id,
            Some(size),
            modified,
            Vec::new(),
            source_order,
        )
        .with_hash(hash.expect("available source hash")),
    );
}

fn source_with_status(
    root: &Path,
    project_id: &str,
    relative: &str,
    origin: &str,
    now: &str,
    status: &str,
    kind: String,
    priority: i64,
    _custom_id: Option<&str>,
    size: Option<u64>,
    modified: Option<String>,
    warnings: Vec<String>,
    source_order: Option<i64>,
) -> DiscoveredProjectSource {
    DiscoveredProjectSource {
        id: stable_id(&format!(
            "{project_id}|{origin}|{}",
            normalize_for_compare(relative)
        )),
        project_id: project_id.to_string(),
        relative_path: relative.to_string(),
        absolute_path: root
            .join(relative.replace('/', &std::path::MAIN_SEPARATOR.to_string()))
            .to_string_lossy()
            .into_owned(),
        source_kind: kind,
        origin: origin.to_string(),
        status: status.to_string(),
        authority_class: authority_for(priority),
        priority,
        size_bytes: size,
        modified_at: modified,
        discovered_at: now.to_string(),
        content_hash: None,
        depth: relative.matches('/').count(),
        warnings,
        schema_version: 1,
        owner: "M08_TASK_SOURCE_DISCOVERY".into(),
        source_order,
    }
}

impl DiscoveredProjectSource {
    fn with_hash(mut self, hash: String) -> Self {
        self.content_hash = Some(hash);
        self
    }
}

fn source_for_missing(
    root: &Path,
    relative: &str,
    project_id: &str,
    now: &str,
    custom_id: &str,
    source_order: i64,
) -> DiscoveredProjectSource {
    source_with_status(
        root,
        project_id,
        relative,
        "CUSTOM",
        now,
        "MISSING",
        "CUSTOM".into(),
        0,
        Some(custom_id),
        None,
        None,
        vec!["configured custom path is missing".into()],
        Some(source_order),
    )
}

fn discovery_warning(
    root: &Path,
    project_id: &str,
    now: &str,
    warnings: &BTreeSet<String>,
) -> DiscoveredProjectSource {
    source_with_status(
        root,
        project_id,
        "[discovery-warning]",
        "SYSTEM",
        now,
        "LIMIT_REACHED",
        "DISCOVERY_WARNING".into(),
        i64::MAX,
        None,
        None,
        None,
        warnings.iter().cloned().collect(),
        None,
    )
}

fn source_sort_key(
    source: &DiscoveredProjectSource,
) -> (u8, i64, i64, std::cmp::Reverse<String>, String) {
    (
        if source.origin == "CUSTOM" { 0 } else { 1 },
        source.source_order.unwrap_or(i64::MAX),
        source.priority,
        std::cmp::Reverse(source.modified_at.clone().unwrap_or_default()),
        source.relative_path.to_ascii_lowercase(),
    )
}

fn discovery_project(
    database: &DatabaseState,
    project_id: &str,
) -> Result<crate::projects::ProjectRecord, String> {
    let project = fetch_project(database, project_id)?;
    match project.status.as_str() {
        "ACTIVE" => Ok(project),
        "MISSING" => Err("registered project root is unavailable".into()),
        "ARCHIVED" => Err("project is archived".into()),
        _ => Err(format!(
            "project status does not permit discovery: {}",
            project.status
        )),
    }
}

fn classify(path: &Path) -> (String, i64) {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let parent_kind = path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if parent_kind == "tasks" {
        return ("TASKS".into(), 10);
    }
    if parent_kind == "plans" {
        return ("PLAN".into(), 40);
    }
    if parent_kind == "handoffs" {
        return ("HANDOFF".into(), 20);
    }
    if name == "tasks.md" || name == "tasks.md" || name == "task.md" {
        ("TASKS".into(), 10)
    } else if name.contains("handoff") {
        ("HANDOFF".into(), 20)
    } else if name == "progress.md" {
        ("PROGRESS".into(), 30)
    } else if name == "plan.md" || name == "plans.md" {
        ("PLAN".into(), 40)
    } else if name == "roadmap.md" {
        ("ROADMAP".into(), 50)
    } else if name == "agents.md" {
        ("AGENTS".into(), 60)
    } else if name == "claude.md" {
        ("CLAUDE".into(), 60)
    } else if path.components().any(|part| {
        part.as_os_str()
            .to_string_lossy()
            .eq_ignore_ascii_case(".hiveai")
    }) {
        ("HIVEAI_CONFIG".into(), 70)
    } else {
        ("OTHER_TASK_SOURCE".into(), 80)
    }
}

fn authority_for(priority: i64) -> String {
    match priority {
        0 => "CUSTOM",
        10 => "TASKS",
        20 => "HANDOFF",
        30 => "PROGRESS",
        40 => "PLAN",
        50 => "ROADMAP",
        60 => "INSTRUCTION",
        _ => "BOUNDED",
    }
    .into()
}
fn is_plausible_source(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("md" | "markdown" | "json" | "yaml" | "yml" | "toml")
    )
}
fn bounded_hash(path: &Path) -> Result<String, String> {
    #[cfg(test)]
    if FAIL_UNREADABLE_PATH
        .get_or_init(|| Mutex::new(None))
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|target| {
            path.file_name().and_then(|name| name.to_str()) == Some(target.as_str())
        })
    {
        return Err("test unreadable failpoint".into());
    }
    let file = fs::File::open(path).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    file.take(MAX_SOURCE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > MAX_SOURCE_BYTES {
        return Err("source exceeded bounded read".into());
    }
    let mut digest = Sha256::new();
    digest.update(bytes);
    Ok(digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn physical_root(path: &Path) -> Result<PathBuf, String> {
    fs::canonicalize(path)
        .map_err(|error| format!("registered project root is unavailable: {error}"))
}
fn normalize_candidate(root: &Path, value: &str) -> Result<String, String> {
    let input = PathBuf::from(value.trim());
    let candidate = if input.is_absolute() {
        input.clone()
    } else {
        root.join(&input)
    };
    if candidate.exists() {
        let physical = fs::canonicalize(&candidate)
            .map_err(|error| format!("custom source path cannot be resolved: {error}"))?;
        ensure_contained(root, &physical)?;
        return Ok(slash_path(physical.strip_prefix(root).map_err(|_| {
            "custom source path escapes project root".to_string()
        })?));
    }
    let relative = if input.is_absolute() {
        input
            .strip_prefix(root)
            .map_err(|_| {
                "custom source path must remain inside the registered project root".to_string()
            })?
            .to_path_buf()
    } else {
        input
    };
    if relative
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err("custom source path traversal escapes project root".into());
    }
    let normalized = relative
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().into_owned()),
            Component::CurDir => None,
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/");
    if normalized.is_empty() || normalized == "." {
        return Err("custom source path must identify a file or directory".into());
    }
    Ok(normalized)
}
fn ensure_contained(root: &Path, target: &Path) -> Result<(), String> {
    if target == root || target.starts_with(root) {
        Ok(())
    } else {
        Err("custom source path escapes the registered project root".into())
    }
}
fn normalize_for_compare(value: &str) -> String {
    value
        .replace('\\', "/")
        .trim_matches('/')
        .to_ascii_lowercase()
}
fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
fn path_depth(root: &Path, path: &Path) -> usize {
    path.strip_prefix(root)
        .map(|value| value.components().count().saturating_sub(1))
        .unwrap_or_default()
}
fn path_status(root: &Path, relative: &str) -> String {
    let path = root.join(relative.replace('/', &std::path::MAIN_SEPARATOR.to_string()));
    if !path.exists() {
        "MISSING".into()
    } else {
        match fs::canonicalize(&path) {
            Ok(physical) if ensure_contained(root, &physical).is_ok() => {
                if physical.is_dir() || physical.is_file() {
                    "CONFIGURED".into()
                } else {
                    "UNREADABLE".into()
                }
            }
            Ok(_) => "OUTSIDE_ROOT".into(),
            Err(_) => "UNREADABLE".into(),
        }
    }
}
fn stable_id(value: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(value.as_bytes());
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn load_custom_paths(
    database: &DatabaseState,
    project_id: &str,
) -> Result<Vec<StoredCustomPath>, String> {
    let connection = database.open_connection()?;
    let value: Option<String> = connection
        .query_row(
            "SELECT value_json FROM settings WHERE key = ?1 AND scope = 'PROJECT'",
            [settings_key(project_id)],
            |row| row.get(0),
        )
        .optional()
        .map_err(db_error)?;
    Ok(value
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default())
}
fn save_custom_paths(
    database: &DatabaseState,
    project_id: &str,
    paths: &[StoredCustomPath],
) -> Result<(), String> {
    let connection = database.open_connection()?;
    let json = serde_json::to_string(paths)
        .map_err(|error| format!("encode custom source paths: {error}"))?;
    connection.execute("INSERT INTO settings (key, value_json, scope, created_at, updated_at) VALUES (?1, ?2, 'PROJECT', ?3, ?3) ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json, updated_at = excluded.updated_at", params![settings_key(project_id), json, crate::time::utc_timestamp()]).map_err(db_error)?;
    Ok(())
}
fn settings_key(project_id: &str) -> String {
    format!("task_sources.custom_paths.{project_id}")
}
fn reconcile(
    database: &DatabaseState,
    project_id: &str,
    sources: &[DiscoveredProjectSource],
) -> Result<(), String> {
    let connection = database.open_connection()?;
    let tx = connection.unchecked_transaction().map_err(db_error)?;
    let mut ids = Vec::new();
    {
        let mut statement = tx
            .prepare("SELECT id, metadata_json FROM project_sources WHERE project_id = ?1")
            .map_err(db_error)?;
        let mut rows = statement.query([project_id]).map_err(db_error)?;
        while let Some(row) = rows.next().map_err(db_error)? {
            let id: String = row.get(0).map_err(db_error)?;
            let metadata: String = row.get(1).map_err(db_error)?;
            let value: serde_json::Value = serde_json::from_str(&metadata).unwrap_or_default();
            let owned = value.get("owner").and_then(serde_json::Value::as_str)
                == Some("M08_TASK_SOURCE_DISCOVERY");
            let compatible = value.get("relativePath").is_some()
                && matches!(
                    value.get("origin").and_then(serde_json::Value::as_str),
                    Some("STANDARD" | "CUSTOM" | "SYSTEM")
                );
            if owned || compatible {
                ids.push(id);
            }
        }
    }
    for id in ids {
        tx.execute("DELETE FROM project_sources WHERE id = ?1", [id])
            .map_err(db_error)?;
    }
    for source in sources {
        let metadata = serde_json::to_string(source)
            .map_err(|error| format!("encode task source metadata: {error}"))?;
        tx.execute("INSERT INTO project_sources (id, project_id, source_path, source_kind, content_hash, discovered_at, metadata_json) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", params![source.id, project_id, source.relative_path, source.source_kind, source.content_hash, source.discovered_at, metadata]).map_err(db_error)?;
    }
    tx.commit().map_err(db_error)
}
fn db_error(error: rusqlite::Error) -> String {
    format!("task source database error: {error}")
}

#[cfg(test)]
static FAIL_UNREADABLE_PATH: OnceLock<Mutex<Option<String>>> = OnceLock::new();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{archive_project, register_project, RegisterProjectRequest};
    use tempfile::tempdir;

    fn fixture() -> (tempfile::TempDir, tempfile::TempDir, DatabaseState, String) {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Sources".into()),
            },
        )
        .unwrap();
        (db_dir, project_dir, database, project.id)
    }
    fn write(path: &Path, value: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, value).unwrap();
    }

    fn sql_count(db: &DatabaseState, sql: &str, id: &str) -> i64 {
        db.open_connection()
            .unwrap()
            .query_row(sql, [id], |row| row.get(0))
            .unwrap()
    }

    #[test]
    fn root_tasks_discovery() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("TASKS.md"), "task source");
        assert_eq!(discover(&db, &id).unwrap()[0].source_kind, "TASKS");
    }
    #[test]
    fn case_insensitive_standard_filename_has_no_duplicate() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("tasks.md"), "one");
        assert_eq!(
            discover(&db, &id)
                .unwrap()
                .iter()
                .filter(|s| s.source_kind == "TASKS")
                .count(),
            1
        );
    }
    #[test]
    fn bounded_tasks_and_handoffs_discovery() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("tasks/a.md"), "a");
        write(&root.path().join("handoffs/t.md"), "b");
        let rows = discover(&db, &id).unwrap();
        assert_eq!(rows.len(), 2);
    }
    #[test]
    fn ignored_trees_are_not_traversed() {
        let (_db_dir, root, db, id) = fixture();
        for dir in IGNORE_DIRS {
            write(&root.path().join(dir).join("bad.md"), "bad");
        }
        assert!(discover(&db, &id).unwrap().is_empty());
    }
    #[test]
    fn outside_and_parent_escape_are_rejected() {
        let (_db_dir, root, db, id) = fixture();
        let outside = tempdir().unwrap();
        assert!(custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: outside.path().to_string_lossy().into_owned()
            }
        )
        .is_err());
        assert!(custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id,
                path: "../outside.md".into()
            }
        )
        .is_err());
        drop(root);
    }
    #[test]
    fn safe_custom_file_and_missing_path_are_persisted() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("evidence.md"), "x");
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "evidence.md".into(),
            },
        )
        .unwrap();
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "future.md".into(),
            },
        )
        .unwrap();
        let rows = discover(&db, &id).unwrap();
        assert!(rows
            .iter()
            .any(|s| s.origin == "CUSTOM" && s.status == "AVAILABLE"));
        assert!(rows
            .iter()
            .any(|s| s.relative_path == "future.md" && s.status == "MISSING"));
    }
    #[test]
    fn custom_path_equivalent_inputs_dedupe() {
        let (_db_dir, _root, db, id) = fixture();
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "evidence.md".into(),
            },
        )
        .unwrap();
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "./EVIDENCE.MD".into(),
            },
        )
        .unwrap();
        assert_eq!(load_custom_paths(&db, &id).unwrap().len(), 1);
    }
    #[test]
    fn repeated_discovery_is_idempotent_and_hash_changes() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("TASKS.md"), "one");
        let first = discover(&db, &id).unwrap();
        let second = discover(&db, &id).unwrap();
        assert_eq!(first[0].id, second[0].id);
        write(&root.path().join("TASKS.md"), "two");
        assert_ne!(
            first[0].content_hash,
            discover(&db, &id).unwrap()[0].content_hash
        );
    }
    #[test]
    fn deleted_standard_is_reconciled_and_custom_remains_missing() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("TASKS.md"), "x");
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "custom.md".into(),
            },
        )
        .unwrap();
        discover(&db, &id).unwrap();
        fs::remove_file(root.path().join("TASKS.md")).unwrap();
        let rows = discover(&db, &id).unwrap();
        assert!(!rows.iter().any(|s| s.source_kind == "TASKS"));
        assert!(rows.iter().any(|s| s.status == "MISSING"));
    }
    #[test]
    fn oversized_source_is_bounded() {
        let (_db_dir, root, db, id) = fixture();
        let file = fs::File::create(root.path().join("TASKS.md")).unwrap();
        file.set_len(MAX_SOURCE_BYTES + 1).unwrap();
        assert_eq!(discover(&db, &id).unwrap()[0].status, "TOO_LARGE");
    }
    #[test]
    fn non_git_discovery_does_not_write_tasks() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("TASKS.md"), "x");
        discover(&db, &id).unwrap();
        let connection = db.open_connection().unwrap();
        assert_eq!(
            connection
                .query_row(
                    "SELECT COUNT(*) FROM tasks WHERE project_id = ?1",
                    [&id],
                    |row| row.get::<_, i64>(0)
                )
                .unwrap(),
            0
        );
        assert_eq!(
            connection
                .query_row(
                    "SELECT COUNT(*) FROM task_sources WHERE project_id = ?1",
                    [&id],
                    |row| row.get::<_, i64>(0)
                )
                .unwrap(),
            0
        );
    }

    #[test]
    fn custom_directory_and_remove_are_production_backed() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("evidence/a.md"), "a");
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "evidence".into(),
            },
        )
        .unwrap();
        assert_eq!(discover(&db, &id).unwrap()[0].origin, "CUSTOM");
        custom_path_remove(&db, &id, "evidence").unwrap();
        assert!(discover(&db, &id).unwrap().is_empty());
    }

    #[test]
    fn missing_project_is_bounded_error() {
        let (_db_dir, _root, db, _id) = fixture();
        assert!(discover(&db, "missing-project")
            .unwrap_err()
            .contains("not registered"));
    }

    #[test]
    fn unavailable_registered_root_is_bounded_error() {
        let (_db_dir, root, db, id) = fixture();
        let path = root.path().to_path_buf();
        drop(root);
        assert!(discover(&db, &id)
            .unwrap_err()
            .contains("registered project root is unavailable"));
        assert!(!path.exists());
    }

    #[test]
    fn source_order_is_deterministic_by_priority_then_path() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("ROADMAP.md"), "r");
        write(&root.path().join("TASKS.md"), "t");
        write(&root.path().join("plans/one.md"), "p");
        let rows = discover(&db, &id).unwrap();
        assert_eq!(
            rows.iter()
                .map(|row| row.source_kind.as_str())
                .collect::<Vec<_>>(),
            vec!["TASKS", "PLAN", "ROADMAP"]
        );
    }

    #[test]
    fn discovery_does_not_mutate_registered_project_tree() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("TASKS.md"), "unchanged");
        let before = fs::read(root.path().join("TASKS.md")).unwrap();
        discover(&db, &id).unwrap();
        assert_eq!(before, fs::read(root.path().join("TASKS.md")).unwrap());
        assert!(!root.path().join(".hiveai").exists());
    }

    #[test]
    fn metadata_contains_authority_priority_depth_and_hash() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("TASKS.md"), "metadata");
        let row = discover(&db, &id).unwrap().remove(0);
        assert_eq!(row.authority_class, "TASKS");
        assert_eq!(row.priority, 10);
        assert_eq!(row.depth, 0);
        assert!(row
            .content_hash
            .as_ref()
            .is_some_and(|hash| hash.len() == 64));
    }

    #[test]
    fn nested_source_depth_is_bounded() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("tasks/a/b/c/d.md"), "bounded");
        let rows = discover(&db, &id).unwrap();
        assert_eq!(rows.len(), 1);
        assert!(rows[0].depth <= MAX_DISCOVERY_DEPTH);
    }

    #[test]
    fn custom_path_listing_reports_available_and_missing() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("evidence.md"), "evidence");
        for path in ["evidence.md", "future.md"] {
            custom_path_add(
                &db,
                CustomPathRequest {
                    project_id: id.clone(),
                    path: path.into(),
                },
            )
            .unwrap();
        }
        let paths = custom_paths_list(&db, &id).unwrap();
        assert_eq!(paths.len(), 2);
        assert!(paths.iter().any(|path| path.status == "CONFIGURED"));
        assert!(paths.iter().any(|path| path.status == "MISSING"));
    }

    #[test]
    fn candidate_file_limit_is_enforced() {
        let (_db_dir, root, db, id) = fixture();
        for index in 0..(MAX_CANDIDATE_FILES + 2) {
            write(&root.path().join(format!("tasks/{index}.md")), "x");
        }
        let rows = discover(&db, &id).unwrap();
        assert!(rows.len() <= MAX_CANDIDATE_FILES + 1);
        assert!(rows.iter().any(|row| row.source_kind == "DISCOVERY_WARNING"
            && row
                .warnings
                .iter()
                .any(|warning| warning.contains("candidate limit"))));
    }

    #[test]
    fn root_handoff_variant_is_classified_as_handoff() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("SESSION_HANDOFF.md"), "handoff");
        let row = discover(&db, &id).unwrap().remove(0);
        assert_eq!(row.source_kind, "HANDOFF");
        assert_eq!(row.authority_class, "HANDOFF");
    }

    #[test]
    fn symlink_escape_is_rejected_or_records_environment_limit() {
        let (_db_dir, root, db, id) = fixture();
        let outside = tempdir().unwrap();
        write(&outside.path().join("outside.md"), "outside");
        let link = root.path().join("custom-link.md");
        match std::os::windows::fs::symlink_file(outside.path().join("outside.md"), &link) {
            Ok(()) => {
                assert!(custom_path_add(
                    &db,
                    CustomPathRequest {
                        project_id: id.clone(),
                        path: "custom-link.md".into()
                    }
                )
                .is_err());
            }
            Err(error) => eprintln!("UNVERIFIED symlink/junction containment: {error}"),
        }
    }

    #[test]
    fn depth_limit_warning_rejects_first_source_beyond_boundary() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("tasks/a/b/c/d.md"), "allowed");
        write(&root.path().join("tasks/a/b/c/d/e.md"), "rejected");
        let rows = discover(&db, &id).unwrap();
        assert!(rows.iter().any(|row| row.relative_path.ends_with("d.md")));
        assert!(!rows.iter().any(|row| row.relative_path.ends_with("e.md")));
        assert!(rows.iter().any(|row| row.source_kind == "DISCOVERY_WARNING"
            && row
                .warnings
                .iter()
                .any(|warning| warning.contains("depth limit"))));
    }

    #[test]
    fn visited_entry_limit_warning_is_structured() {
        let (_db_dir, root, db, id) = fixture();
        for index in 0..(MAX_VISITED_ENTRIES + 4) {
            write(&root.path().join(format!("noise-{index}.txt")), "noise");
        }
        let rows = discover(&db, &id).unwrap();
        assert!(rows.iter().any(|row| row.source_kind == "DISCOVERY_WARNING"
            && row
                .warnings
                .iter()
                .any(|warning| warning.contains("visited-entry limit"))));
    }

    #[test]
    fn project_sources_persist_owner_schema_hash_and_idempotent_identity() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("TASKS.md"), "one");
        let first = discover(&db, &id).unwrap();
        let first_id = first
            .iter()
            .find(|row| row.source_kind == "TASKS")
            .unwrap()
            .id
            .clone();
        assert_eq!(sql_count(&db, "SELECT COUNT(*) FROM project_sources WHERE project_id = ?1 AND json_extract(metadata_json, '$.owner') = 'M08_TASK_SOURCE_DISCOVERY'", &id), 1);
        discover(&db, &id).unwrap();
        assert_eq!(sql_count(&db, "SELECT COUNT(*) FROM project_sources WHERE project_id = ?1 AND json_extract(metadata_json, '$.owner') = 'M08_TASK_SOURCE_DISCOVERY'", &id), 1);
        let connection = db.open_connection().unwrap();
        let (stored_id, owner, version, hash): (String, String, i64, String) = connection.query_row("SELECT id, json_extract(metadata_json, '$.owner'), json_extract(metadata_json, '$.schemaVersion'), content_hash FROM project_sources WHERE project_id = ?1 AND source_kind = 'TASKS'", [&id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).unwrap();
        assert_eq!(stored_id, first_id);
        assert_eq!(owner, "M08_TASK_SOURCE_DISCOVERY");
        assert_eq!(version, 1);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn unrelated_legacy_project_source_survives_reconciliation() {
        let (_db_dir, root, db, id) = fixture();
        let connection = db.open_connection().unwrap();
        connection.execute("INSERT INTO project_sources (id, project_id, source_path, source_kind, content_hash, discovered_at, metadata_json) VALUES ('legacy', ?1, 'legacy.txt', 'LEGACY', NULL, 'now', '{\"legacy\":true}')", [&id]).unwrap();
        write(&root.path().join("TASKS.md"), "source");
        discover(&db, &id).unwrap();
        assert_eq!(
            sql_count(
                &db,
                "SELECT COUNT(*) FROM project_sources WHERE id = 'legacy' AND project_id = ?1",
                &id
            ),
            1
        );
    }

    #[test]
    fn custom_available_to_missing_reconciliation_is_persisted() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("custom.md"), "custom");
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "custom.md".into(),
            },
        )
        .unwrap();
        let available = discover(&db, &id).unwrap();
        assert!(available
            .iter()
            .any(|row| row.origin == "CUSTOM" && row.status == "AVAILABLE"));
        fs::remove_file(root.path().join("custom.md")).unwrap();
        let missing = discover(&db, &id).unwrap();
        assert!(missing
            .iter()
            .any(|row| row.origin == "CUSTOM" && row.status == "MISSING"));
    }

    #[test]
    fn unreadable_failpoint_preserves_other_valid_source() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("tasks/__fail_unreadable__.md"), "bad");
        write(&root.path().join("TASKS.md"), "good");
        *FAIL_UNREADABLE_PATH
            .get_or_init(|| Mutex::new(None))
            .lock()
            .unwrap() = Some("__fail_unreadable__.md".into());
        let rows = discover(&db, &id).unwrap();
        *FAIL_UNREADABLE_PATH
            .get_or_init(|| Mutex::new(None))
            .lock()
            .unwrap() = None;
        assert!(
            rows.iter()
                .any(|row| row.relative_path == "tasks/__fail_unreadable__.md"
                    && row.status == "UNREADABLE"),
            "{rows:#?}"
        );
        assert!(rows
            .iter()
            .any(|row| row.relative_path == "TASKS.md" && row.status == "AVAILABLE"));
        assert_eq!(sql_count(&db, "SELECT COUNT(*) FROM project_sources WHERE project_id = ?1 AND source_path = 'TASKS.md'", &id), 1);
    }

    #[test]
    fn custom_update_order_remove_equivalence_and_containment_are_safe() {
        let (_db_dir, root, db, id) = fixture();
        write(&root.path().join("a.md"), "a");
        write(&root.path().join("b.md"), "b");
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "a.md".into(),
            },
        )
        .unwrap();
        custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "b.md".into(),
            },
        )
        .unwrap();
        let updated = custom_path_update(
            &db,
            CustomPathUpdateRequest {
                project_id: id.clone(),
                path_or_id: "b.md".into(),
                path: Some("a-renamed.md".into()),
                order: Some(0),
            },
        )
        .unwrap();
        assert_eq!(updated[0].normalized_path, "a-renamed.md");
        assert!(custom_path_update(
            &db,
            CustomPathUpdateRequest {
                project_id: id.clone(),
                path_or_id: "a-renamed.md".into(),
                path: Some("../outside.md".into()),
                order: None
            }
        )
        .is_err());
        assert!(custom_path_remove(&db, &id, "A-RENAMED.MD").is_ok());
    }

    #[test]
    fn archived_project_rejects_all_discovery_mutations() {
        let (_db_dir, _root, db, id) = fixture();
        archive_project(&db, &id).unwrap();
        assert!(discover(&db, &id).unwrap_err().contains("archived"));
        assert!(custom_path_add(
            &db,
            CustomPathRequest {
                project_id: id.clone(),
                path: "x.md".into()
            }
        )
        .unwrap_err()
        .contains("archived"));
    }
}
