use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    pub schema_version: i64,
    pub migration_count: i64,
    pub last_migration_status: &'static str,
}

const INITIAL_SCHEMA: &str = r#"
CREATE TABLE projects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    local_path TEXT,
    default_branch TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    metadata_json TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE repositories (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    remote_url TEXT,
    github_owner TEXT,
    github_repo TEXT,
    default_branch TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE project_sources (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_path TEXT NOT NULL,
    source_kind TEXT NOT NULL,
    content_hash TEXT,
    discovered_at TEXT NOT NULL,
    metadata_json TEXT
);

CREATE TABLE git_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    repository_id TEXT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    branch TEXT,
    head_sha TEXT,
    status_json TEXT,
    captured_at TEXT NOT NULL
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_id TEXT REFERENCES task_sources(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL,
    required_actor TEXT,
    milestone TEXT,
    metadata_json TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE task_dependencies (
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    depends_on_task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    dependency_kind TEXT NOT NULL DEFAULT 'BLOCKS',
    created_at TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);

CREATE TABLE task_sources (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_path TEXT NOT NULL,
    source_kind TEXT NOT NULL,
    locator TEXT,
    content_hash TEXT,
    discovered_at TEXT NOT NULL
);

CREATE TABLE task_events (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    from_state TEXT,
    to_state TEXT,
    actor_type TEXT,
    summary TEXT NOT NULL,
    evidence_json TEXT,
    occurred_at TEXT NOT NULL
);

CREATE TABLE prompts (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
    task_id TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    kind TEXT NOT NULL,
    current_version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE prompt_versions (
    id TEXT PRIMARY KEY NOT NULL,
    prompt_id TEXT NOT NULL REFERENCES prompts(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE (prompt_id, version)
);

CREATE TABLE agent_sessions (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
    task_id TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    provider TEXT NOT NULL,
    state TEXT NOT NULL,
    started_at TEXT,
    ended_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE agent_events (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL REFERENCES agent_sessions(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    payload_json TEXT,
    occurred_at TEXT NOT NULL
);

CREATE TABLE agent_tool_calls (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL REFERENCES agent_sessions(id) ON DELETE CASCADE,
    tool_name TEXT NOT NULL,
    status TEXT NOT NULL,
    input_metadata_json TEXT,
    output_metadata_json TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE permission_requests (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT REFERENCES agent_sessions(id) ON DELETE SET NULL,
    permission_kind TEXT NOT NULL,
    requested_resource TEXT,
    state TEXT NOT NULL,
    decided_by TEXT,
    created_at TEXT NOT NULL,
    decided_at TEXT
);

CREATE TABLE audits (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
    task_id TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    result TEXT NOT NULL,
    summary TEXT,
    confidence REAL,
    created_at TEXT NOT NULL
);

CREATE TABLE audit_findings (
    id TEXT PRIMARY KEY NOT NULL,
    audit_id TEXT NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    severity TEXT NOT NULL,
    title TEXT NOT NULL,
    detail TEXT,
    file_path TEXT,
    line_number INTEGER,
    created_at TEXT NOT NULL
);

CREATE TABLE test_runs (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
    task_id TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    command TEXT NOT NULL,
    result TEXT NOT NULL,
    output_metadata_json TEXT,
    started_at TEXT NOT NULL,
    finished_at TEXT
);

CREATE TABLE alerts (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE CASCADE,
    task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,
    state TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TEXT NOT NULL,
    resolved_at TEXT
);

CREATE TABLE decisions (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE CASCADE,
    task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    audit_id TEXT REFERENCES audits(id) ON DELETE SET NULL,
    decision_kind TEXT NOT NULL,
    decision TEXT NOT NULL,
    rationale TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE github_sync_state (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    resource_kind TEXT NOT NULL,
    resource_cursor TEXT,
    last_synced_at TEXT,
    metadata_json TEXT,
    UNIQUE (project_id, resource_kind)
);

CREATE TABLE settings (
    key TEXT PRIMARY KEY NOT NULL,
    value_json TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT 'WORKSPACE',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
"#;

const INITIAL_INDEXES: &str = r#"
CREATE INDEX idx_repositories_project ON repositories(project_id);
CREATE INDEX idx_project_sources_project ON project_sources(project_id, discovered_at);
CREATE INDEX idx_git_snapshots_repository ON git_snapshots(repository_id, captured_at);
CREATE INDEX idx_tasks_project_state ON tasks(project_id, state, updated_at);
CREATE INDEX idx_tasks_source ON tasks(source_id);
CREATE INDEX idx_task_dependencies_dependency ON task_dependencies(depends_on_task_id);
CREATE INDEX idx_task_sources_project ON task_sources(project_id, discovered_at);
CREATE INDEX idx_task_events_task_time ON task_events(task_id, occurred_at);
CREATE INDEX idx_prompt_versions_prompt ON prompt_versions(prompt_id, version);
CREATE INDEX idx_agent_sessions_project_state ON agent_sessions(project_id, state);
CREATE INDEX idx_agent_events_session_time ON agent_events(session_id, occurred_at);
CREATE INDEX idx_agent_tool_calls_session ON agent_tool_calls(session_id, created_at);
CREATE INDEX idx_permission_requests_state ON permission_requests(state, created_at);
CREATE INDEX idx_audits_project_time ON audits(project_id, created_at);
CREATE INDEX idx_audit_findings_audit ON audit_findings(audit_id, severity);
CREATE INDEX idx_test_runs_project_time ON test_runs(project_id, started_at);
CREATE INDEX idx_alerts_state ON alerts(state, created_at);
CREATE INDEX idx_decisions_project_time ON decisions(project_id, created_at);
CREATE INDEX idx_github_sync_project ON github_sync_state(project_id, last_synced_at);
CREATE INDEX idx_settings_scope ON settings(scope);
"#;

const PROJECT_REGISTRY_FIELDS: &str = r#"
ALTER TABLE projects ADD COLUMN original_path TEXT;
ALTER TABLE projects ADD COLUMN normalized_path TEXT;
ALTER TABLE projects ADD COLUMN registered_at TEXT;
ALTER TABLE projects ADD COLUMN last_validated_at TEXT;
ALTER TABLE projects ADD COLUMN preferred_builder TEXT;
ALTER TABLE projects ADD COLUMN preferred_auditor TEXT;
ALTER TABLE projects ADD COLUMN task_source_policy TEXT;
ALTER TABLE projects ADD COLUMN archived_at TEXT;
ALTER TABLE repositories ADD COLUMN repository_root TEXT;
ALTER TABLE repositories ADD COLUMN is_git_repository INTEGER NOT NULL DEFAULT 0;
ALTER TABLE repositories ADD COLUMN current_branch TEXT;
ALTER TABLE repositories ADD COLUMN head_sha TEXT;
ALTER TABLE repositories ADD COLUMN remote_urls_json TEXT;
"#;

const PROJECT_SNAPSHOT_FIELDS: &str = r#"
CREATE TABLE project_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    availability TEXT NOT NULL,
    git_snapshot_id TEXT REFERENCES git_snapshots(id) ON DELETE SET NULL,
    last_filesystem_event_at TEXT,
    last_watcher_refresh_at TEXT,
    evidence_generated_at TEXT NOT NULL,
    changed_path_count INTEGER NOT NULL DEFAULT 0,
    rescan_required INTEGER NOT NULL DEFAULT 0,
    watcher_health TEXT NOT NULL,
    created_at TEXT NOT NULL
);
CREATE INDEX idx_project_snapshots_project_time ON project_snapshots(project_id, evidence_generated_at);
"#;

const TIMESTAMP_STANDARDIZATION: &str = r#"
UPDATE projects SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE projects SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
UPDATE projects SET registered_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(registered_at AS INTEGER), 'unixepoch') WHERE registered_at GLOB '[0-9]*' AND registered_at NOT GLOB '*[^0-9]*';
UPDATE projects SET last_validated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(last_validated_at AS INTEGER), 'unixepoch') WHERE last_validated_at GLOB '[0-9]*' AND last_validated_at NOT GLOB '*[^0-9]*';
UPDATE projects SET archived_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(archived_at AS INTEGER), 'unixepoch') WHERE archived_at GLOB '[0-9]*' AND archived_at NOT GLOB '*[^0-9]*';
UPDATE repositories SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE repositories SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
UPDATE git_snapshots SET captured_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(captured_at AS INTEGER), 'unixepoch') WHERE captured_at GLOB '[0-9]*' AND captured_at NOT GLOB '*[^0-9]*';
UPDATE project_snapshots SET last_filesystem_event_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(last_filesystem_event_at AS INTEGER), 'unixepoch') WHERE last_filesystem_event_at GLOB '[0-9]*' AND last_filesystem_event_at NOT GLOB '*[^0-9]*';
UPDATE project_snapshots SET last_watcher_refresh_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(last_watcher_refresh_at AS INTEGER), 'unixepoch') WHERE last_watcher_refresh_at GLOB '[0-9]*' AND last_watcher_refresh_at NOT GLOB '*[^0-9]*';
UPDATE project_snapshots SET evidence_generated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(evidence_generated_at AS INTEGER), 'unixepoch') WHERE evidence_generated_at GLOB '[0-9]*' AND evidence_generated_at NOT GLOB '*[^0-9]*';
"#;

const RESIDUAL_TIMESTAMP_STANDARDIZATION: &str = r#"
UPDATE tasks SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE tasks SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
UPDATE task_sources SET discovered_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(discovered_at AS INTEGER), 'unixepoch') WHERE discovered_at GLOB '[0-9]*' AND discovered_at NOT GLOB '*[^0-9]*';
UPDATE task_events SET occurred_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(occurred_at AS INTEGER), 'unixepoch') WHERE occurred_at GLOB '[0-9]*' AND occurred_at NOT GLOB '*[^0-9]*';
UPDATE prompts SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE prompts SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
UPDATE prompt_versions SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE agent_sessions SET started_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(started_at AS INTEGER), 'unixepoch') WHERE started_at GLOB '[0-9]*' AND started_at NOT GLOB '*[^0-9]*';
UPDATE agent_sessions SET ended_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(ended_at AS INTEGER), 'unixepoch') WHERE ended_at GLOB '[0-9]*' AND ended_at NOT GLOB '*[^0-9]*';
UPDATE agent_sessions SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE agent_events SET occurred_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(occurred_at AS INTEGER), 'unixepoch') WHERE occurred_at GLOB '[0-9]*' AND occurred_at NOT GLOB '*[^0-9]*';
UPDATE agent_tool_calls SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE permission_requests SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE audits SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE audit_findings SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE test_runs SET started_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(started_at AS INTEGER), 'unixepoch') WHERE started_at GLOB '[0-9]*' AND started_at NOT GLOB '*[^0-9]*';
UPDATE test_runs SET finished_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(finished_at AS INTEGER), 'unixepoch') WHERE finished_at GLOB '[0-9]*' AND finished_at NOT GLOB '*[^0-9]*';
UPDATE alerts SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE alerts SET resolved_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(resolved_at AS INTEGER), 'unixepoch') WHERE resolved_at GLOB '[0-9]*' AND resolved_at NOT GLOB '*[^0-9]*';
UPDATE decisions SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE github_sync_state SET last_synced_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(last_synced_at AS INTEGER), 'unixepoch') WHERE last_synced_at GLOB '[0-9]*' AND last_synced_at NOT GLOB '*[^0-9]*';
UPDATE settings SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE settings SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
"#;

pub fn migrations() -> &'static [Migration] {
    &[
        Migration {
            version: 1,
            name: "initial_hiveai_schema",
            sql: INITIAL_SCHEMA,
        },
        Migration {
            version: 2,
            name: "initial_lookup_indexes",
            sql: INITIAL_INDEXES,
        },
        Migration {
            version: 3,
            name: "project_registry_fields",
            sql: PROJECT_REGISTRY_FIELDS,
        },
        Migration {
            version: 4,
            name: "project_snapshot_fields",
            sql: PROJECT_SNAPSHOT_FIELDS,
        },
        Migration {
            version: 5,
            name: "timestamp_standardization",
            sql: TIMESTAMP_STANDARDIZATION,
        },
        Migration {
            version: 6,
            name: "residual_timestamp_standardization",
            sql: RESIDUAL_TIMESTAMP_STANDARDIZATION,
        },
        Migration {
            version: 7,
            name: "project_snapshot_created_timestamp",
            sql: "UPDATE project_snapshots SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';",
        },
        Migration {
            version: 8,
            name: "project_preferred_agent_provider",
            sql: "ALTER TABLE projects ADD COLUMN preferred_agent_provider TEXT;",
        },
    ]
}

pub fn apply_migrations(conn: &mut Connection, requested: &[Migration]) -> Result<MigrationReport> {
    validate_migration_list(requested)?;
    conn.execute_batch("PRAGMA foreign_keys = ON; CREATE TABLE IF NOT EXISTS migrations (version INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, applied_at TEXT NOT NULL);")?;

    let applied_rows: Vec<(i64, String)> = {
        let mut applied = conn.prepare("SELECT version, name FROM migrations ORDER BY version")?;
        let rows = applied
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<_>>>()?;
        rows
    };
    for (index, (version, name)) in applied_rows.iter().enumerate() {
        let expected = requested.get(index).ok_or_else(|| {
            validation_error(format!("database has unknown migration version {version}"))
        })?;
        if *version != expected.version || name != expected.name {
            return Err(validation_error(format!(
                "migration history mismatch at version {version}"
            )));
        }
    }

    let mut applied_any = false;
    for migration in requested.iter().skip(applied_rows.len()) {
        let tx = conn.transaction()?;
        tx.execute_batch(migration.sql)?;
        tx.execute("INSERT INTO migrations (version, name, applied_at) VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))", params![migration.version, migration.name])?;
        tx.commit()?;
        applied_any = true;
    }

    let schema_version = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM migrations",
        [],
        |row| row.get(0),
    )?;
    let migration_count =
        conn.query_row("SELECT COUNT(*) FROM migrations", [], |row| row.get(0))?;
    Ok(MigrationReport {
        schema_version,
        migration_count,
        last_migration_status: if applied_any {
            "APPLIED"
        } else {
            "ALREADY_CURRENT"
        },
    })
}

fn validate_migration_list(requested: &[Migration]) -> Result<()> {
    for (index, migration) in requested.iter().enumerate() {
        let expected = index as i64 + 1;
        if migration.version != expected {
            return Err(validation_error(format!(
                "migration versions must be contiguous; expected {expected}, got {}",
                migration.version
            )));
        }
    }
    Ok(())
}

fn validation_error(message: String) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(MigrationValidationError(message)))
}

#[derive(Debug)]
struct MigrationValidationError(String);

impl fmt::Display for MigrationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for MigrationValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::path::Path;
    use tempfile::tempdir;

    fn temp_connection() -> (tempfile::TempDir, Connection) {
        let directory = tempdir().expect("temp directory");
        let connection =
            Connection::open(directory.path().join("test-hiveai.db")).expect("temp database");
        (directory, connection)
    }

    #[test]
    fn fresh_database_reaches_latest_version() {
        let (_directory, mut connection) = temp_connection();
        let report = apply_migrations(&mut connection, migrations()).expect("migrations apply");
        assert_eq!(report.schema_version, 8);
        assert_eq!(report.migration_count, 8);
        assert_eq!(report.last_migration_status, "APPLIED");
    }

    #[test]
    fn rerunning_migrations_is_idempotent() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("first apply");
        let report = apply_migrations(&mut connection, migrations()).expect("second apply");
        assert_eq!(report.last_migration_status, "ALREADY_CURRENT");
        assert_eq!(report.migration_count, 8);
    }

    #[test]
    fn migration_v7_converts_numeric_snapshot_timestamp_from_v6_fixture() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, &migrations()[..6]).expect("v1-v6 apply");
        connection
            .execute(
                "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p7', 'P7', 'now', 'now')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO project_snapshots (id, project_id, availability, evidence_generated_at, watcher_health, created_at) VALUES ('s7', 'p7', 'AVAILABLE', 'now', 'HEALTHY', '1700000000')",
                [],
            )
            .unwrap();
        apply_migrations(&mut connection, migrations()).expect("v7 apply");
        let value: String = connection
            .query_row(
                "SELECT created_at FROM project_snapshots WHERE id = 's7'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(value.starts_with("2023-11-14T22:13:20."));
    }

    #[test]
    fn migration_v7_preserves_valid_and_malformed_snapshot_timestamps() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, &migrations()[..6]).expect("v1-v6 apply");
        connection
            .execute(
                "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p8', 'P8', 'now', 'now')",
                [],
            )
            .unwrap();
        for (id, value) in [
            ("valid", "2024-01-01T00:00:00.123Z"),
            ("bad", "not-a-timestamp"),
        ] {
            connection
                .execute(
                    "INSERT INTO project_snapshots (id, project_id, availability, evidence_generated_at, watcher_health, created_at) VALUES (?1, 'p8', 'AVAILABLE', 'now', 'HEALTHY', ?2)",
                    params![id, value],
                )
                .unwrap();
        }
        apply_migrations(&mut connection, migrations()).expect("v7 apply");
        let values: Vec<String> = connection
            .prepare("SELECT created_at FROM project_snapshots ORDER BY id")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(values, vec!["not-a-timestamp", "2024-01-01T00:00:00.123Z"]);
    }

    #[test]
    fn migration_v7_real_fixture_is_idempotent_and_history_safe() {
        let (_directory, mut connection) = temp_connection();
        let first = apply_migrations(&mut connection, migrations()).expect("first apply");
        let second = apply_migrations(&mut connection, migrations()).expect("rerun");
        assert_eq!(first.schema_version, 8);
        assert_eq!(second.last_migration_status, "ALREADY_CURRENT");
        let mismatch = [Migration {
            version: 1,
            name: "wrong",
            sql: "",
        }];
        assert!(apply_migrations(&mut connection, &mismatch).is_err());
    }

    #[test]
    fn migration_history_is_inspectable() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        let rows: Vec<(i64, String)> = connection
            .prepare("SELECT version, name FROM migrations ORDER BY version")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(
            rows,
            vec![
                (1, "initial_hiveai_schema".to_string()),
                (2, "initial_lookup_indexes".to_string()),
                (3, "project_registry_fields".to_string()),
                (4, "project_snapshot_fields".to_string()),
                (5, "timestamp_standardization".to_string()),
                (6, "residual_timestamp_standardization".to_string()),
                (7, "project_snapshot_created_timestamp".to_string())
            ]
        );
    }

    #[test]
    fn foreign_keys_are_enabled() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        assert_eq!(
            connection
                .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            1
        );
    }

    #[test]
    fn required_tables_and_indexes_exist() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        for table in [
            "projects",
            "repositories",
            "project_sources",
            "git_snapshots",
            "tasks",
            "task_dependencies",
            "task_sources",
            "task_events",
            "prompts",
            "prompt_versions",
            "agent_sessions",
            "agent_events",
            "agent_tool_calls",
            "permission_requests",
            "audits",
            "audit_findings",
            "test_runs",
            "alerts",
            "decisions",
            "github_sync_state",
            "settings",
            "migrations",
        ] {
            assert_eq!(
                connection
                    .query_row(
                        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                        [table],
                        |row| row.get::<_, i64>(0)
                    )
                    .unwrap(),
                1,
                "missing table {table}"
            );
        }
        for index in [
            "idx_tasks_project_state",
            "idx_task_dependencies_dependency",
            "idx_prompt_versions_prompt",
            "idx_agent_sessions_project_state",
            "idx_audit_findings_audit",
            "idx_github_sync_project",
        ] {
            assert_eq!(
                connection
                    .query_row(
                        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'index' AND name = ?1",
                        [index],
                        |row| row.get::<_, i64>(0)
                    )
                    .unwrap(),
                1,
                "missing index {index}"
            );
        }
    }

    #[test]
    fn migration_failure_rolls_back_transaction() {
        let (_directory, mut connection) = temp_connection();
        let failing = [
            Migration {
                version: 1,
                name: "ok",
                sql: "CREATE TABLE safe_table (id TEXT PRIMARY KEY);",
            },
            Migration {
                version: 2,
                name: "fails",
                sql:
                    "CREATE TABLE broken_table (id TEXT PRIMARY KEY); SELECT * FROM missing_table;",
            },
        ];
        assert!(apply_migrations(&mut connection, &failing).is_err());
        assert_eq!(connection.query_row("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'safe_table'", [], |row| row.get::<_, i64>(0)).unwrap(), 1);
        assert_eq!(connection.query_row("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'broken_table'", [], |row| row.get::<_, i64>(0)).unwrap(), 0);
        assert_eq!(
            connection
                .query_row("SELECT COUNT(*) FROM migrations", [], |row| row
                    .get::<_, i64>(0))
                .unwrap(),
            1
        );
    }

    #[test]
    fn incorrect_versioned_database_fails_safely() {
        let (_directory, mut connection) = temp_connection();
        connection.execute_batch("CREATE TABLE migrations (version INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, applied_at TEXT NOT NULL); INSERT INTO migrations VALUES (2, 'wrong', 'now');").unwrap();
        assert!(apply_migrations(&mut connection, migrations()).is_err());
    }

    #[test]
    fn tests_use_isolated_temp_path_not_repository_path() {
        let (directory, _connection) = temp_connection();
        assert!(directory.path().is_dir());
        assert!(!directory
            .path()
            .starts_with(Path::new(env!("CARGO_MANIFEST_DIR"))));
    }

    #[test]
    fn representative_foreign_key_relationships_are_enforced() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        connection.execute("INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p1', 'Project', 'now', 'now')", []).unwrap();
        connection.execute("INSERT INTO repositories (id, project_id, created_at, updated_at) VALUES ('r1', 'p1', 'now', 'now')", []).unwrap();
        connection.execute("INSERT INTO tasks (id, project_id, title, state, created_at, updated_at) VALUES ('t1', 'p1', 'Task', 'BACKLOG', 'now', 'now')", []).unwrap();
        connection.execute("INSERT INTO prompts (id, project_id, task_id, kind, created_at, updated_at) VALUES ('pr1', 'p1', 't1', 'BUILD', 'now', 'now')", []).unwrap();
        connection.execute("INSERT INTO prompt_versions (id, prompt_id, version, content, created_by, created_at) VALUES ('pv1', 'pr1', 1, 'content', 'test', 'now')", []).unwrap();
        connection.execute("INSERT INTO audits (id, project_id, task_id, result, created_at) VALUES ('a1', 'p1', 't1', 'PASS', 'now')", []).unwrap();
        connection.execute("INSERT INTO audit_findings (id, audit_id, severity, title, created_at) VALUES ('f1', 'a1', 'LOW', 'Finding', 'now')", []).unwrap();
        assert!(connection.execute("INSERT INTO repositories (id, project_id, created_at, updated_at) VALUES ('bad', 'missing', 'now', 'now')", []).is_err());
        assert!(connection.execute("INSERT INTO prompt_versions (id, prompt_id, version, content, created_by, created_at) VALUES ('badpv', 'missing', 1, 'content', 'test', 'now')", []).is_err());
        assert!(connection.execute("INSERT INTO audit_findings (id, audit_id, severity, title, created_at) VALUES ('badf', 'missing', 'LOW', 'Finding', 'now')", []).is_err());
    }
}
