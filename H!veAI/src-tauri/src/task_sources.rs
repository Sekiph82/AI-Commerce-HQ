use crate::db::DatabaseState;
use crate::projects::fetch_project;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CustomSourcePath {
    pub id: String,
    pub project_id: String,
    pub display_path: String,
    pub normalized_path: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomPathRequest {
    pub project_id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredCustomPath {
    id: String,
    display_path: String,
    normalized_path: String,
}

pub fn discover(
    database: &DatabaseState,
    project_id: &str,
) -> Result<Vec<DiscoveredProjectSource>, String> {
    let project = fetch_project(database, project_id)?;
    let root = physical_root(Path::new(&project.normalized_path))?;
    let custom = load_custom_paths(database, project_id)?;
    let now = crate::time::utc_timestamp();
    let mut candidates = Vec::new();
    discover_standard(&root, project_id, &now, &mut candidates);
    for path in &custom {
        discover_custom(&root, path, project_id, &now, &mut candidates);
    }
    candidates.sort_by(|a, b| {
        (
            a.priority,
            a.relative_path.to_ascii_lowercase(),
            a.origin.clone(),
        )
            .cmp(&(
                b.priority,
                b.relative_path.to_ascii_lowercase(),
                b.origin.clone(),
            ))
    });
    reconcile(database, project_id, &candidates)?;
    list(database, project_id)
}

pub fn list(
    database: &DatabaseState,
    project_id: &str,
) -> Result<Vec<DiscoveredProjectSource>, String> {
    let project = fetch_project(database, project_id)?;
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
        let mut source: DiscoveredProjectSource = serde_json::from_str(&metadata)
            .map_err(|error| format!("decode task source metadata: {error}"))?;
        source.id = row.get(0).map_err(db_error)?;
        source.project_id = row.get(1).map_err(db_error)?;
        source.relative_path = row.get(2).map_err(db_error)?;
        source.source_kind = row.get(3).map_err(db_error)?;
        source.content_hash = row.get(4).map_err(db_error)?;
        source.discovered_at = row.get(5).map_err(db_error)?;
        result.push(source);
    }
    result.sort_by(|a, b| {
        (a.priority, a.relative_path.to_ascii_lowercase())
            .cmp(&(b.priority, b.relative_path.to_ascii_lowercase()))
    });
    Ok(result)
}

pub fn custom_paths_list(
    database: &DatabaseState,
    project_id: &str,
) -> Result<Vec<CustomSourcePath>, String> {
    fetch_project(database, project_id)?;
    let root = Path::new(&fetch_project(database, project_id)?.normalized_path).to_path_buf();
    Ok(load_custom_paths(database, project_id)?
        .into_iter()
        .map(|path| CustomSourcePath {
            id: path.id,
            project_id: project_id.to_string(),
            status: path_status(&root, &path.normalized_path),
            display_path: path.display_path,
            normalized_path: path.normalized_path,
        })
        .collect())
}

pub fn custom_path_add(
    database: &DatabaseState,
    request: CustomPathRequest,
) -> Result<Vec<CustomSourcePath>, String> {
    let project = fetch_project(database, &request.project_id)?;
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
    fetch_project(database, project_id)?;
    let mut paths = load_custom_paths(database, project_id)?;
    let before = paths.len();
    paths.retain(|path| {
        path.id != path_or_id && path.normalized_path != normalize_for_compare(path_or_id)
    });
    if paths.len() == before {
        return Err("custom source path is not configured".to_string());
    }
    save_custom_paths(database, project_id, &paths)?;
    custom_paths_list(database, project_id)
}

fn discover_standard(
    root: &Path,
    project_id: &str,
    now: &str,
    output: &mut Vec<DiscoveredProjectSource>,
) {
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|value| value.to_str()) {
                let lower = name.to_ascii_lowercase();
                if ROOT_NAMES.contains(&lower.as_str())
                    || (lower.ends_with("handoff.md") && lower.len() > "handoff.md".len())
                {
                    collect_file(root, &path, project_id, "STANDARD", now, output, 0, None);
                }
            }
        }
    }
    for directory in ["tasks", "plans", "handoffs", ".hiveai"] {
        let path = root.join(directory);
        if path.is_dir() {
            walk_bounded(root, &path, project_id, "STANDARD", now, output, 0, None);
        }
    }
}

fn discover_custom(
    root: &Path,
    custom: &StoredCustomPath,
    project_id: &str,
    now: &str,
    output: &mut Vec<DiscoveredProjectSource>,
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
            );
        }
    } else {
        output.push(source_for_missing(
            root,
            &custom.normalized_path,
            project_id,
            now,
            &custom.id,
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
) {
    if depth > MAX_DISCOVERY_DEPTH || output.len() >= MAX_CANDIDATE_FILES {
        return;
    }
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        if output.len() >= MAX_CANDIDATE_FILES {
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
            walk_bounded(
                root,
                &path,
                project_id,
                origin,
                now,
                output,
                depth + 1,
                custom_id,
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
) {
    if output.len() >= MAX_CANDIDATE_FILES || !is_plausible_source(path) {
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
    )
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
    } else if path.is_dir() || path.is_file() {
        "CONFIGURED".into()
    } else {
        "UNREADABLE".into()
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
    tx.execute(
        "DELETE FROM project_sources WHERE project_id = ?1",
        [project_id],
    )
    .map_err(db_error)?;
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
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
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
}
