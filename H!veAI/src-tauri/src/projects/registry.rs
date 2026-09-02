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
    pub preferred_agent_provider: Option<String>,
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
    pub preferred_agent_provider: Option<String>,
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
        "UPDATE projects SET priority = COALESCE(?2, priority), preferred_builder = COALESCE(?3, preferred_builder), preferred_auditor = COALESCE(?4, preferred_auditor), task_source_policy = COALESCE(?5, task_source_policy), preferred_agent_provider = COALESCE(?6, preferred_agent_provider), updated_at = ?7 WHERE id = ?1",
        params![request.project_id, request.priority, request.preferred_builder.and_then(non_empty), request.preferred_auditor.and_then(non_empty), request.task_source_policy.and_then(non_empty), request.preferred_agent_provider.and_then(validate_agent_provider), timestamp()],
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
    if existing_identity.is_none() && git.is_git_repository {
        return Err("legacy registered project has no repository identity; Git repair is rejected until identity is explicitly recovered".to_string());
    }
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
            if old_remote.is_none() && git.preferred_remote_url.is_none() {
                let old = old_head.as_deref().ok_or_else(|| {
                    "repository identity is ambiguous; stored HEAD evidence is required".to_string()
                })?;
                let new = git.head_sha.as_deref().ok_or_else(|| {
                    "repository identity is ambiguous; replacement HEAD evidence is required"
                        .to_string()
                })?;
                if old != new && !is_ancestor(&validated.canonical_path, old, new) {
                    return Err("repository identity is unrelated; stored HEAD is not an ancestor of replacement HEAD".to_string());
                }
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

const PROJECT_SELECT: &str = "SELECT p.id, p.name, COALESCE(p.original_path, p.local_path, ''), COALESCE(p.normalized_path, p.local_path, ''), p.status, p.priority, p.preferred_builder, p.preferred_auditor, p.task_source_policy, p.preferred_agent_provider, COALESCE(p.registered_at, p.created_at), p.last_validated_at, r.id, r.is_git_repository, r.repository_root, r.current_branch, r.head_sha, r.remote_url, r.default_branch, r.github_owner, r.github_repo, r.remote_urls_json FROM projects p LEFT JOIN repositories r ON r.project_id = p.id";

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
    let repository_id: Option<String> = row.get(12)?;
    let repository = repository_id.map(|id| RepositoryRecord {
        id,
        is_git_repository: row.get::<_, i64>(13).unwrap_or_default() == 1,
        repository_root: row.get(14).ok().flatten(),
        current_branch: row.get(15).ok().flatten(),
        head_sha: row.get(16).ok().flatten(),
        preferred_remote_url: row.get(17).ok().flatten(),
        default_branch: row.get(18).ok().flatten(),
        github_owner: row.get(19).ok().flatten(),
        github_repo: row.get(20).ok().flatten(),
        remotes: row
            .get::<_, Option<String>>(21)
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
        preferred_agent_provider: row.get(9)?,
        registered_at: row.get(10)?,
        last_validated_at: row.get(11)?,
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
    let remotes = serde_json::to_string(&git.remotes)
        .map_err(|error| format!("serialize sanitized remote metadata: {error}"))?;
    connection.execute("INSERT INTO repositories (id, project_id, remote_url, github_owner, github_repo, default_branch, created_at, updated_at, repository_root, is_git_repository, current_branch, head_sha, remote_urls_json) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7, ?8, ?9, ?10, ?11, ?12)", params![Uuid::new_v4().to_string(), project_id, git.preferred_remote_url, git.github_owner, git.github_repo, git.default_branch, now, git.repository_root, if git.is_git_repository { 1 } else { 0 }, git.current_branch, git.head_sha, remotes]).map_err(db_error)?;
    Ok(())
}

fn is_ancestor(repository: &Path, old: &str, new: &str) -> bool {
    crate::process_policy::background_command("git")
        .args([
            "-C",
            &repository.to_string_lossy(),
            "merge-base",
            "--is-ancestor",
            old,
            new,
        ])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
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

fn validate_agent_provider(value: String) -> Option<String> {
    let value = value.trim().to_ascii_uppercase();
    matches!(value.as_str(), "CODEX" | "CLAUDE").then_some(value)
}

pub fn ensure_scrubbots_agent_preference(database: &DatabaseState) -> Result<(), String> {
    let connection = database.open_connection()?;
    connection.execute("UPDATE projects SET preferred_agent_provider='CLAUDE', updated_at=?1 WHERE preferred_agent_provider IS NULL AND lower(name) LIKE '%scrubbots%'", [timestamp()]).map_err(db_error)?;
    Ok(())
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
    use std::process::Command;
    use tempfile::tempdir;

    fn database() -> (tempfile::TempDir, DatabaseState) {
        let directory = tempdir().unwrap();
        let path = directory.path().to_path_buf();
        (directory, DatabaseState::initialize(path).unwrap())
    }

    fn git(path: &Path, args: &[&str]) {
        assert!(Command::new("git")
            .args(args)
            .current_dir(path)
            .output()
            .unwrap()
            .status
            .success());
    }

    #[test]
    fn repair_project_path_r03_identity_matrix() {
        let (_db_dir, database) = database();
        let non_git = tempdir().unwrap();
        let moved_non_git = tempdir().unwrap();
        let non_git_project = register_project(
            &database,
            RegisterProjectRequest {
                path: non_git.path().to_string_lossy().into_owned(),
                name: Some("Legacy non Git".into()),
            },
        )
        .unwrap();
        database
            .open_connection()
            .unwrap()
            .execute(
                "DELETE FROM repositories WHERE project_id = ?1",
                [&non_git_project.id],
            )
            .unwrap();
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: non_git_project.id.clone(),
                path: moved_non_git.path().to_string_lossy().into_owned(),
            },
        )
        .is_ok());
        let git_project_dir = tempdir().unwrap();
        git(git_project_dir.path(), &["init", "-q"]);
        git(git_project_dir.path(), &["config", "user.name", "Test"]);
        git(
            git_project_dir.path(),
            &["config", "user.email", "test@example.com"],
        );
        std::fs::write(git_project_dir.path().join("README.md"), "one").unwrap();
        git(git_project_dir.path(), &["add", "README.md"]);
        git(git_project_dir.path(), &["commit", "-qm", "initial"]);
        let git_project = register_project(
            &database,
            RegisterProjectRequest {
                path: git_project_dir.path().to_string_lossy().into_owned(),
                name: Some("Git identity".into()),
            },
        )
        .unwrap();
        let other_git = tempdir().unwrap();
        git(other_git.path(), &["init", "-q"]);
        git(other_git.path(), &["config", "user.name", "Other"]);
        git(
            other_git.path(),
            &["config", "user.email", "other@example.com"],
        );
        std::fs::write(other_git.path().join("OTHER"), "other").unwrap();
        git(other_git.path(), &["add", "OTHER"]);
        git(other_git.path(), &["commit", "-qm", "other"]);
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: git_project.id.clone(),
                path: other_git.path().to_string_lossy().into_owned(),
            },
        )
        .is_err());
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: "unknown-project".into(),
                path: moved_non_git.path().to_string_lossy().into_owned(),
            },
        )
        .is_err());
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
        assert_eq!(
            project
                .repository
                .as_ref()
                .map(|repo| repo.is_git_repository),
            Some(false)
        );
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

    fn git_repo_with_remote(path: &Path, remote_url: &str) {
        git(path, &["init", "-q"]);
        git(path, &["config", "user.name", "Test"]);
        git(path, &["config", "user.email", "test@example.com"]);
        std::fs::write(path.join("README.md"), "init").unwrap();
        git(path, &["add", "README.md"]);
        git(path, &["commit", "-qm", "initial"]);
        git(path, &["remote", "add", "origin", remote_url]);
    }

    fn git_repo_no_remote(path: &Path) {
        git(path, &["init", "-q"]);
        git(path, &["config", "user.name", "Test"]);
        git(path, &["config", "user.email", "test@example.com"]);
        std::fs::write(path.join("README.md"), "init").unwrap();
        git(path, &["add", "README.md"]);
        git(path, &["commit", "-qm", "initial"]);
    }

    fn get_head(path: &Path) -> String {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(path)
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    }

    #[test]
    fn identity_01_legacy_rowless_non_git_to_moved_non_git_accepted() {
        let (_db_dir, database) = database();
        let original = tempdir().unwrap();
        let moved = tempdir().unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: original.path().to_string_lossy().into_owned(),
                name: Some("Legacy Non-Git".into()),
            },
        )
        .unwrap();
        database
            .open_connection()
            .unwrap()
            .execute(
                "DELETE FROM repositories WHERE project_id = ?1",
                [&project.id],
            )
            .unwrap();
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: moved.path().to_string_lossy().into_owned(),
            }
        )
        .is_ok());
    }

    #[test]
    fn identity_02_legacy_rowless_non_git_to_git_rejected() {
        let (_db_dir, database) = database();
        let original = tempdir().unwrap();
        let git_dir = tempdir().unwrap();
        git_repo_no_remote(git_dir.path());
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: original.path().to_string_lossy().into_owned(),
                name: Some("Legacy to Git".into()),
            },
        )
        .unwrap();
        database
            .open_connection()
            .unwrap()
            .execute(
                "DELETE FROM repositories WHERE project_id = ?1",
                [&project.id],
            )
            .unwrap();
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: git_dir.path().to_string_lossy().into_owned(),
            }
        )
        .is_err());
    }

    #[test]
    fn identity_03_explicit_non_git_to_git_rejected() {
        let (_db_dir, database) = database();
        let non_git = tempdir().unwrap();
        let git_dir = tempdir().unwrap();
        git_repo_no_remote(git_dir.path());
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: non_git.path().to_string_lossy().into_owned(),
                name: Some("Non-Git Explicit".into()),
            },
        )
        .unwrap();
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: git_dir.path().to_string_lossy().into_owned(),
            }
        )
        .is_err());
    }

    #[test]
    fn identity_04_same_remote_same_head_accepted() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_with_remote(dir1.path(), "https://github.com/org/repo.git");
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Same Remote Same HEAD".into()),
            },
        )
        .unwrap();
        let dir2 = tempdir().unwrap();
        git_repo_with_remote(dir2.path(), "https://github.com/org/repo.git");
        let head1 = get_head(dir1.path());
        let head2 = get_head(dir2.path());
        if head1 != head2 {
            git(
                dir2.path(),
                &["fetch", &dir1.path().to_string_lossy(), "HEAD"],
            );
            git(dir2.path(), &["reset", "--hard", "FETCH_HEAD"]);
        }
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_ok());
    }

    #[test]
    fn identity_05_same_remote_advanced_head_accepted() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_with_remote(dir1.path(), "https://github.com/org/repo.git");
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Same Remote Advanced HEAD".into()),
            },
        )
        .unwrap();
        let dir2 = tempdir().unwrap();
        git_repo_with_remote(dir2.path(), "https://github.com/org/repo.git");
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_ok());
    }

    #[test]
    fn identity_06_different_remote_rejected() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_with_remote(dir1.path(), "https://github.com/org/repo-a.git");
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Different Remote".into()),
            },
        )
        .unwrap();
        let dir2 = tempdir().unwrap();
        git_repo_with_remote(dir2.path(), "https://github.com/org/repo-b.git");
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_err());
    }

    #[test]
    fn identity_07_remote_disappearance_rejected() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_with_remote(dir1.path(), "https://github.com/org/repo.git");
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Remote Disappear".into()),
            },
        )
        .unwrap();
        let dir2 = tempdir().unwrap();
        git_repo_no_remote(dir2.path());
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_err());
    }

    #[test]
    fn identity_08_no_remote_exact_head_accepted() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_no_remote(dir1.path());
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("No Remote Exact".into()),
            },
        )
        .unwrap();
        let dir2 = tempdir().unwrap();
        let head = get_head(dir1.path());
        git(dir2.path(), &["clone", &dir1.path().to_string_lossy(), "."]);
        git(dir2.path(), &["remote", "remove", "origin"]);
        let head2 = get_head(dir2.path());
        assert_eq!(head, head2);
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_ok());
    }

    #[test]
    fn identity_09_no_remote_ancestor_accepted() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_no_remote(dir1.path());
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("No Remote Ancestor".into()),
            },
        )
        .unwrap();
        let dir2 = tempdir().unwrap();
        git(dir2.path(), &["clone", &dir1.path().to_string_lossy(), "."]);
        git(dir2.path(), &["remote", "remove", "origin"]);
        std::fs::write(dir2.path().join("extra.txt"), "advance").unwrap();
        git(dir2.path(), &["add", "extra.txt"]);
        git(dir2.path(), &["commit", "-qm", "advance"]);
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_ok());
    }

    #[test]
    fn identity_10_unrelated_no_remote_rejected() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_no_remote(dir1.path());
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Unrelated No Remote".into()),
            },
        )
        .unwrap();
        let dir2 = tempdir().unwrap();
        git_repo_no_remote(dir2.path());
        std::fs::write(dir2.path().join("README.md"), "unrelated").unwrap();
        git(dir2.path(), &["add", "README.md"]);
        git(dir2.path(), &["commit", "--amend", "-qm", "unrelated-root"]);
        assert_ne!(get_head(dir1.path()), get_head(dir2.path()));
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_err());
    }

    #[test]
    fn identity_11_missing_stored_old_head_rejected() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_no_remote(dir1.path());
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Missing Stored HEAD".into()),
            },
        )
        .unwrap();
        database
            .open_connection()
            .unwrap()
            .execute(
                "UPDATE repositories SET head_sha = NULL WHERE project_id = ?1",
                [&project.id],
            )
            .unwrap();
        let dir2 = tempdir().unwrap();
        git(dir2.path(), &["clone", &dir1.path().to_string_lossy(), "."]);
        git(dir2.path(), &["remote", "remove", "origin"]);
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_err());
    }

    #[test]
    fn identity_12_credential_bearing_https_remote_sanitizes_same_identity() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        git_repo_with_remote(dir1.path(), "https://user:token@github.com/org/repo.git");
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Credential Remote".into()),
            },
        )
        .unwrap();
        let stored_url: Option<String> = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT remote_url FROM repositories WHERE project_id = ?1",
                [&project.id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            stored_url.as_deref(),
            Some("https://github.com/org/repo.git")
        );
        let dir2 = tempdir().unwrap();
        git_repo_with_remote(dir2.path(), "https://other:secret@github.com/org/repo.git");
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            }
        )
        .is_ok());
    }

    #[test]
    fn identity_13_duplicate_normalized_repair_target_rejected() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        let _p1 = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Project A".into()),
            },
        )
        .unwrap();
        let p2 = register_project(
            &database,
            RegisterProjectRequest {
                path: dir2.path().to_string_lossy().into_owned(),
                name: Some("Project B".into()),
            },
        )
        .unwrap();
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: p2.id.clone(),
                path: dir1.path().to_string_lossy().into_owned(),
            }
        )
        .is_err());
    }

    #[test]
    fn identity_14_unknown_project_id_rejected() {
        let (_db_dir, database) = database();
        let dir = tempdir().unwrap();
        assert!(repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: "nonexistent-id".into(),
                path: dir.path().to_string_lossy().into_owned(),
            }
        )
        .is_err());
    }

    #[test]
    fn identity_15_repair_validation_leaves_candidate_unchanged() {
        let (_db_dir, database) = database();
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        std::fs::write(dir2.path().join("witness.txt"), "untouched").unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: dir1.path().to_string_lossy().into_owned(),
                name: Some("Candidate Unchanged".into()),
            },
        )
        .unwrap();
        repair_project_path(
            &database,
            RepairProjectPathRequest {
                project_id: project.id.clone(),
                path: dir2.path().to_string_lossy().into_owned(),
            },
        )
        .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir2.path().join("witness.txt")).unwrap(),
            "untouched"
        );
        let entries: Vec<_> = std::fs::read_dir(dir2.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(entries, vec!["witness.txt"]);
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
