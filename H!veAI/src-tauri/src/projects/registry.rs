use super::detection::{detect_git_metadata, GitMetadata};
use super::paths::validate_project_path;
use crate::db::DatabaseState;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectListQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub sort: Option<String>,
    pub include_archived: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterProjectRequest {
    pub path: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectSettingsRequest {
    pub project_id: String,
    pub priority: Option<i64>,
    pub preferred_builder: Option<String>,
    pub preferred_auditor: Option<String>,
    pub task_source_policy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepairProjectPathRequest {
    pub project_id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRecord {
    pub id: String,
    pub name: String,
    pub original_path: String,
    pub normalized_path: String,
    pub status: String,
    pub priority: i64,
    pub preferred_builder: Option<String>,
    pub preferred_auditor: Option<String>,
    pub task_source_policy: Option<String>,
    pub registered_at: String,
    pub last_validated_at: Option<String>,
    pub repository: Option<RepositoryRecord>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryRecord {
    pub id: String,
    pub is_git_repository: bool,
    pub repository_root: Option<String>,
    pub current_branch: Option<String>,
    pub head_sha: Option<String>,
    pub preferred_remote_url: Option<String>,
    pub default_branch: Option<String>,
    pub github_owner: Option<String>,
    pub github_repo: Option<String>,
    pub remotes: Vec<super::detection::RemoteMetadata>,
}

pub fn list_projects(
    database: &DatabaseState,
    query: ProjectListQuery,
) -> Result<Vec<ProjectRecord>, String> {
    let connection = database.open_connection()?;
    let mut statement = connection.prepare(PROJECT_SELECT).map_err(db_error)?;
    let rows = statement.query_map([], map_project).map_err(db_error)?;
    let mut projects = rows.collect::<Result<Vec<_>, _>>().map_err(db_error)?;
    let search = query.search.unwrap_or_default().trim().to_ascii_lowercase();
    let status = query.status.unwrap_or_default().trim().to_ascii_uppercase();
    let include_archived = query.include_archived.unwrap_or(false);
    projects.retain(|project| {
        if !include_archived && project.status == "ARCHIVED" {
            return false;
        }
        if !status.is_empty() && project.status != status {
            return false;
        }
        if search.is_empty() {
            return true;
        }
        let haystack = format!(
            "{} {} {}",
            project.name,
            project.original_path,
            project
                .repository
                .as_ref()
                .and_then(|repository| repository.preferred_remote_url.clone())
                .unwrap_or_default()
        )
        .to_ascii_lowercase();
        haystack.contains(&search)
    });
    match query.sort.as_deref() {
        Some("priority") => projects.sort_by_key(|project| Reverse(project.priority)),
        Some("updated") => projects
            .sort_by_key(|project| Reverse(project.last_validated_at.clone().unwrap_or_default())),
        _ => projects.sort_by_key(|project| project.name.to_ascii_lowercase()),
    }
    Ok(projects)
}

pub fn register_project(
    database: &DatabaseState,
    request: RegisterProjectRequest,
) -> Result<ProjectRecord, String> {
    let validated = validate_project_path(&request.path)?;
    let git = detect_git_metadata(&validated.canonical_path);
    let connection = database.open_connection()?;
    ensure_no_duplicate(&connection, &validated.normalized_path, None)?;
    let now = timestamp();
    let project_id = Uuid::new_v4().to_string();
    let name = request
        .name
        .and_then(non_empty)
        .unwrap_or_else(|| folder_name(&validated.canonical_path));
    let tx = connection.unchecked_transaction().map_err(db_error)?;
    tx.execute(
        "INSERT INTO projects (id, name, local_path, status, priority, created_at, updated_at, original_path, normalized_path, registered_at, last_validated_at, task_source_policy) VALUES (?1, ?2, ?3, 'ACTIVE', 0, ?4, ?4, ?5, ?6, ?4, ?4, 'DISCOVER_STANDARD_FILES')",
        params![project_id, name, validated.canonical_path.to_string_lossy(), now, validated.display_path, validated.normalized_path],
    ).map_err(db_error)?;
    insert_repository(&tx, &project_id, &git, &now)?;
    tx.commit().map_err(db_error)?;
    fetch_project(database, &project_id)
}

pub fn fetch_project(database: &DatabaseState, project_id: &str) -> Result<ProjectRecord, String> {
    let connection = database.open_connection()?;
    let query = format!("{PROJECT_SELECT} WHERE p.id = ?1");
    connection
        .query_row(&query, [project_id], map_project)
        .map_err(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => "project is not registered".to_string(),
            other => db_error(other),
        })
}

pub fn update_project_settings(
    database: &DatabaseState,
    request: UpdateProjectSettingsRequest,
) -> Result<ProjectRecord, String> {
    let connection = database.open_connection()?;
    let updated = connection.execute(
        "UPDATE projects SET priority = COALESCE(?2, priority), preferred_builder = COALESCE(?3, preferred_builder), preferred_auditor = COALESCE(?4, preferred_auditor), task_source_policy = COALESCE(?5, task_source_policy), updated_at = ?6 WHERE id = ?1",
        params![request.project_id, request.priority, request.preferred_builder.and_then(non_empty), request.preferred_auditor.and_then(non_empty), request.task_source_policy.and_then(non_empty), timestamp()],
    ).map_err(db_error)?;
    if updated == 0 {
        return Err("project is not registered".to_string());
    }
    fetch_project(database, &request.project_id)
}

pub fn archive_project(
    database: &DatabaseState,
    project_id: &str,
) -> Result<ProjectRecord, String> {
    let connection = database.open_connection()?;
    let updated = connection.execute("UPDATE projects SET status = 'ARCHIVED', archived_at = ?2, updated_at = ?2 WHERE id = ?1", params![project_id, timestamp()]).map_err(db_error)?;
    if updated == 0 {
        return Err("project is not registered".to_string());
    }
    fetch_project(database, project_id)
}

pub fn remove_project(database: &DatabaseState, project_id: &str) -> Result<(), String> {
    let connection = database.open_connection()?;
    let deleted = connection
        .execute("DELETE FROM projects WHERE id = ?1", [project_id])
        .map_err(db_error)?;
    if deleted == 0 {
        return Err("project is not registered".to_string());
    }
    Ok(())
}

pub fn repair_project_path(
    database: &DatabaseState,
    request: RepairProjectPathRequest,
) -> Result<ProjectRecord, String> {
    let validated = validate_project_path(&request.path)?;
    let git = detect_git_metadata(&validated.canonical_path);
    let connection = database.open_connection()?;
    ensure_no_duplicate(
        &connection,
        &validated.normalized_path,
        Some(&request.project_id),
    )?;
    let existing_identity: Option<(i64, Option<String>, Option<String>)> = connection
        .query_row(
            "SELECT is_git_repository, remote_url, head_sha FROM repositories WHERE project_id = ?1",
            [&request.project_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()
        .map_err(db_error)?;
    if let Some((old_is_git, old_remote, old_head)) = existing_identity {
        if (old_is_git == 1) != git.is_git_repository {
            return Err("repository type changed while repairing project path".to_string());
        }
        if git.is_git_repository {
            match (old_remote.as_deref(), git.preferred_remote_url.as_deref()) {
                (Some(old), Some(new)) if old != new => {
                    return Err(
                        "new path repository remote identity does not match the registered project"
                            .to_string(),
                    );
                }
                (None, Some(_)) | (Some(_), None) => {
                    return Err("repository remote identity is ambiguous; repair requires an explicit matching remote".to_string());
                }
                _ => {}
            }
            // A matching sanitized remote is the durable repository identity; a moved
            // checkout may legitimately advance its HEAD between registry observations.
            // When no remote exists, the commit remains the only strong identity signal.
            if old_remote.is_none()
                && git.preferred_remote_url.is_none()
                && old_head != git.head_sha
            {
                return Err("repository identity is ambiguous; matching remote or HEAD evidence is required".to_string());
            }
        }
    }
    let now = timestamp();
    let tx = connection.unchecked_transaction().map_err(db_error)?;
    let updated = tx.execute("UPDATE projects SET local_path = ?2, original_path = ?3, normalized_path = ?4, status = 'ACTIVE', archived_at = NULL, last_validated_at = ?5, updated_at = ?5 WHERE id = ?1", params![request.project_id, validated.canonical_path.to_string_lossy(), validated.display_path, validated.normalized_path, now]).map_err(db_error)?;
    if updated == 0 {
        return Err("project is not registered".to_string());
    }
    tx.execute(
        "DELETE FROM repositories WHERE project_id = ?1",
        [&request.project_id],
    )
    .map_err(db_error)?;
    insert_repository(&tx, &request.project_id, &git, &now)?;
    tx.commit().map_err(db_error)?;
    fetch_project(database, &request.project_id)
}

const PROJECT_SELECT: &str = "SELECT p.id, p.name, COALESCE(p.original_path, p.local_path, ''), COALESCE(p.normalized_path, p.local_path, ''), p.status, p.priority, p.preferred_builder, p.preferred_auditor, p.task_source_policy, COALESCE(p.registered_at, p.created_at), p.last_validated_at, r.id, r.is_git_repository, r.repository_root, r.current_branch, r.head_sha, r.remote_url, r.default_branch, r.github_owner, r.github_repo, r.remote_urls_json FROM projects p LEFT JOIN repositories r ON r.project_id = p.id";

fn map_project(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProjectRecord> {
    let normalized_path: String = row.get(3)?;
    let stored_status: String = row.get(4)?;
    let status = if stored_status == "ARCHIVED" {
        stored_status
    } else if Path::new(&normalized_path).exists() {
        stored_status
    } else {
        "MISSING".to_string()
    };
    let repository_id: Option<String> = row.get(11)?;
    let repository = repository_id.map(|id| RepositoryRecord {
        id,
        is_git_repository: row.get::<_, i64>(12).unwrap_or_default() == 1,
        repository_root: row.get(13).ok().flatten(),
        current_branch: row.get(14).ok().flatten(),
        head_sha: row.get(15).ok().flatten(),
        preferred_remote_url: row.get(16).ok().flatten(),
        default_branch: row.get(17).ok().flatten(),
        github_owner: row.get(18).ok().flatten(),
        github_repo: row.get(19).ok().flatten(),
        remotes: row
            .get::<_, Option<String>>(20)
            .ok()
            .flatten()
            .and_then(|value| serde_json::from_str(&value).ok())
            .unwrap_or_default(),
    });
    Ok(ProjectRecord {
        id: row.get(0)?,
        name: row.get(1)?,
        original_path: row.get(2)?,
        normalized_path,
        status,
        priority: row.get(5)?,
        preferred_builder: row.get(6)?,
        preferred_auditor: row.get(7)?,
        task_source_policy: row.get(8)?,
        registered_at: row.get(9)?,
        last_validated_at: row.get(10)?,
        repository,
    })
}

fn ensure_no_duplicate(
    connection: &Connection,
    normalized_path: &str,
    except_id: Option<&str>,
) -> Result<(), String> {
    let duplicate: Option<String> = connection
        .query_row(
            "SELECT id FROM projects WHERE normalized_path = ?1 AND (?2 IS NULL OR id != ?2)",
            params![normalized_path, except_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(db_error)?;
    if duplicate.is_some() {
        Err("a project with this normalized path is already registered".to_string())
    } else {
        Ok(())
    }
}

fn insert_repository(
    connection: &rusqlite::Transaction<'_>,
    project_id: &str,
    git: &GitMetadata,
    now: &str,
) -> Result<(), String> {
    if !git.is_git_repository {
        return Ok(());
    }
    let remotes = serde_json::to_string(&git.remotes)
        .map_err(|error| format!("serialize sanitized remote metadata: {error}"))?;
    connection.execute("INSERT INTO repositories (id, project_id, remote_url, github_owner, github_repo, default_branch, created_at, updated_at, repository_root, is_git_repository, current_branch, head_sha, remote_urls_json) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7, ?8, ?9, ?10, ?11, ?12)", params![Uuid::new_v4().to_string(), project_id, git.preferred_remote_url, git.github_owner, git.github_repo, git.default_branch, now, git.repository_root, if git.is_git_repository { 1 } else { 0 }, git.current_branch, git.head_sha, remotes]).map_err(db_error)?;
    Ok(())
}

fn folder_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("Unnamed project")
        .to_string()
}
fn non_empty(value: String) -> Option<String> {
    let value = value.trim().to_string();
    (!value.is_empty()).then_some(value)
}
fn timestamp() -> String {
    crate::time::utc_timestamp()
}
fn db_error(error: rusqlite::Error) -> String {
    format!("project registry database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DatabaseState;
    use tempfile::tempdir;

    fn database() -> (tempfile::TempDir, DatabaseState) {
        let directory = tempdir().unwrap();
        let path = directory.path().to_path_buf();
        (directory, DatabaseState::initialize(path).unwrap())
    }

    #[test]
    fn registration_duplicate_and_non_git_are_deterministic() {
        let (_db_dir, database) = database();
        let project_dir = tempdir().unwrap();
        let request = RegisterProjectRequest {
            path: project_dir.path().to_string_lossy().into_owned(),
            name: Some("Fixture Project".to_string()),
        };
        let project = register_project(&database, request.clone()).unwrap();
        assert_eq!(project.name, "Fixture Project");
        assert!(project.repository.is_none());
        assert!(register_project(&database, request).is_err());
    }

    #[test]
    fn archive_remove_and_repair_never_delete_folders() {
        let (_db_dir, database) = database();
        let original = tempdir().unwrap();
        let moved = tempdir().unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: original.path().to_string_lossy().into_owned(),
                name: None,
            },
        )
        .unwrap();
        archive_project(&database, &project.id).unwrap();
        assert!(original.path().exists());
        repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: moved.path().to_string_lossy().into_owned(),
            },
        )
        .unwrap();
        assert!(original.path().exists());
        assert!(moved.path().exists());
        remove_project(&database, &project.id).unwrap();
        assert!(original.path().exists());
        assert!(moved.path().exists());
    }

    #[test]
    fn list_search_filter_and_missing_state_work() {
        let (_db_dir, database) = database();
        let project_dir = tempdir().unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Searchable".to_string()),
            },
        )
        .unwrap();
        assert_eq!(
            list_projects(
                &database,
                ProjectListQuery {
                    search: Some("search".to_string()),
                    ..Default::default()
                }
            )
            .unwrap()
            .len(),
            1
        );
        std::fs::remove_dir(project_dir.path()).unwrap();
        let missing = fetch_project(&database, &project.id).unwrap();
        assert_eq!(missing.status, "MISSING");
        assert_eq!(
            list_projects(
                &database,
                ProjectListQuery {
                    status: Some("MISSING".to_string()),
                    ..Default::default()
                }
            )
            .unwrap()
            .len(),
            1
        );
    }
}
