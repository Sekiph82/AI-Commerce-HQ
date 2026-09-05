mod migrations;

use migrations::MigrationReport;
use rusqlite::Connection;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[cfg(test)]
thread_local! {
    static FAIL_NEXT_BACKUP_PUBLICATION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    pub initialized: bool,
    pub engine: String,
    pub schema_version: i64,
    pub migration_count: i64,
    pub database_path: String,
    pub foreign_keys_enabled: bool,
    pub last_migration_status: String,
    pub journal_mode: String,
    pub busy_timeout_ms: i64,
    pub synchronous: String,
    pub integrity_status: String,
}

#[derive(Debug, Clone)]
pub struct DatabaseState {
    status: DatabaseStatus,
    database_path: PathBuf,
}

impl DatabaseState {
    pub fn initialize(app_data_dir: PathBuf) -> Result<Self, String> {
        fs::create_dir_all(&app_data_dir)
            .map_err(|error| format!("create H!veAI app-data directory: {error}"))?;
        let database_path = app_data_dir.join("hiveai.db");
        let existing_non_empty = database_path
            .metadata()
            .map(|m| m.len() > 0)
            .unwrap_or(false);
        if existing_non_empty {
            integrity_preflight(&database_path)?;
        }
        let mut connection = Connection::open(&database_path)
            .map_err(|error| format!("open H!veAI database: {error}"))?;
        configure_connection(&connection)?;
        let integrity = "ok".to_string();
        let current_version = connection
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM migrations",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0);
        let latest_version = migrations::migrations()
            .last()
            .map(|migration| migration.version)
            .unwrap_or(0);
        if database_path
            .metadata()
            .map(|metadata| metadata.len() > 0)
            .unwrap_or(false)
            && current_version < latest_version
        {
            create_migration_backup(&connection, &database_path)?;
        }
        let report = migrations::apply_migrations(&mut connection, migrations::migrations())
            .map_err(|error| format!("run H!veAI database migrations: {error}"))?;
        let foreign_keys_enabled = connection
            .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
            .map_err(|error| format!("inspect H!veAI database safety pragmas: {error}"))?
            == 1;
        Ok(Self {
            status: status_from_report(report, foreign_keys_enabled, integrity),
            database_path,
        })
    }

    pub fn status(&self) -> DatabaseStatus {
        self.status.clone()
    }

    pub(crate) fn open_connection(&self) -> Result<Connection, String> {
        let connection = Connection::open(&self.database_path)
            .map_err(|error| format!("open H!veAI database connection: {error}"))?;
        configure_connection(&connection)?;
        Ok(connection)
    }
}

fn integrity_preflight(database_path: &PathBuf) -> Result<(), String> {
    let flags =
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX;
    let preflight = Connection::open_with_flags(database_path, flags)
        .map_err(|error| format!("open H!veAI database for integrity preflight: {error}"))?;
    let integrity: String = preflight
        .query_row("PRAGMA quick_check(1)", [], |row| row.get(0))
        .map_err(|error| format!("check H!veAI database integrity: {error}"))?;
    if integrity != "ok" {
        return Err(format!(
            "H!veAI database integrity check failed: {integrity}"
        ));
    }
    Ok(())
}

fn configure_connection(connection: &Connection) -> Result<(), String> {
    connection
        .pragma_update(None, "foreign_keys", true)
        .map_err(|e| format!("enable H!veAI database foreign keys: {e}"))?;
    connection
        .pragma_update(None, "journal_mode", "WAL")
        .map_err(|e| format!("enable H!veAI database WAL mode: {e}"))?;
    connection
        .pragma_update(None, "busy_timeout", 5000i64)
        .map_err(|e| format!("set H!veAI database busy timeout: {e}"))?;
    connection
        .pragma_update(None, "synchronous", "NORMAL")
        .map_err(|e| format!("set H!veAI database synchronous mode: {e}"))?;
    Ok(())
}

fn create_migration_backup(connection: &Connection, database_path: &PathBuf) -> Result<(), String> {
    let backup = database_path.with_extension("db.pre-migration.bak");
    let previous = database_path.with_extension("db.pre-migration.bak.prev");
    let temporary = database_path.with_extension("db.pre-migration.tmp");
    let _ = fs::remove_file(&temporary);
    let mut destination = Connection::open(&temporary)
        .map_err(|error| format!("open H!veAI migration backup: {error}"))?;
    let backup_result = {
        let backup_handle = rusqlite::backup::Backup::new(connection, &mut destination)
            .map_err(|error| format!("backup H!veAI database before migration: {error}"))?;
        backup_handle
            .run_to_completion(1024, Duration::from_millis(1), None)
            .map_err(|error| format!("backup H!veAI database before migration: {error}"))
    };
    destination
        .close()
        .map_err(|(_, error)| format!("close H!veAI migration backup: {error}"))?;
    backup_result?;
    #[cfg(test)]
    if FAIL_NEXT_BACKUP_PUBLICATION.with(|failpoint| failpoint.replace(false)) {
        let _ = fs::remove_file(&temporary);
        return Err("test-only backup publication failpoint".to_string());
    }
    let had_backup = backup.exists();
    if had_backup {
        let _ = fs::remove_file(&previous);
        if let Err(error) = fs::rename(&backup, &previous) {
            let _ = fs::remove_file(&temporary);
            return Err(format!("rotate H!veAI database migration backup: {error}"));
        }
    }
    if let Err(error) = fs::rename(&temporary, &backup) {
        let _ = fs::remove_file(&temporary);
        if had_backup {
            let _ = fs::rename(&previous, &backup);
        }
        return Err(format!("publish H!veAI database migration backup: {error}"));
    }
    Ok(())
}

fn status_from_report(
    report: MigrationReport,
    foreign_keys_enabled: bool,
    integrity_status: String,
) -> DatabaseStatus {
    DatabaseStatus {
        initialized: true,
        engine: "SQLite".to_string(),
        schema_version: report.schema_version,
        migration_count: report.migration_count,
        database_path: "hiveai.db".to_string(),
        foreign_keys_enabled,
        last_migration_status: report.last_migration_status.to_string(),
        journal_mode: "WAL".to_string(),
        busy_timeout_ms: 5000,
        synchronous: "NORMAL".to_string(),
        integrity_status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn app_data_database_initialization_reports_relative_path() {
        let directory = tempdir().expect("temp directory");
        let state = DatabaseState::initialize(directory.path().to_path_buf())
            .expect("database initializes");
        let status = state.status();
        assert!(status.initialized);
        assert_eq!(status.database_path, "hiveai.db");
        assert_eq!(status.schema_version, 12);
        assert!(status.foreign_keys_enabled);
        assert!(directory.path().join("hiveai.db").exists());
        assert_eq!(status.journal_mode, "WAL");
        assert_eq!(status.busy_timeout_ms, 5000);
        assert_eq!(status.synchronous, "NORMAL");
        assert_eq!(status.integrity_status, "ok");
    }

    #[test]
    fn migration_backup_is_atomic_and_bounded_to_database_path() {
        let directory = tempdir().expect("temp directory");
        let database = directory.path().join("hiveai.db");
        let source = Connection::open(&database).expect("open fixture database");
        configure_connection(&source).expect("configure fixture database");
        source
            .execute("CREATE TABLE fixture (value TEXT)", [])
            .expect("create fixture");
        source
            .execute("INSERT INTO fixture VALUES ('wal-preserved')", [])
            .expect("insert fixture");
        create_migration_backup(&source, &database).expect("backup should publish");
        drop(source);
        let backup =
            Connection::open(database.with_extension("db.pre-migration.bak")).expect("open backup");
        assert_eq!(
            backup
                .query_row("SELECT value FROM fixture", [], |row| row
                    .get::<_, String>(0))
                .unwrap(),
            "wal-preserved"
        );
        assert!(!database.with_extension("db.pre-migration.tmp").exists());
    }

    #[test]
    fn sqlite_committed_wal_data_survives_repeated_bounded_backup() {
        let directory = tempdir().expect("temp directory");
        let database = directory.path().join("hiveai.db");
        let source = Connection::open(&database).expect("open fixture database");
        configure_connection(&source).expect("configure fixture database");
        source
            .execute("CREATE TABLE fixture (value TEXT)", [])
            .unwrap();
        source
            .execute("INSERT INTO fixture VALUES ('first')", [])
            .unwrap();
        create_migration_backup(&source, &database).unwrap();
        source
            .execute("INSERT INTO fixture VALUES ('second')", [])
            .unwrap();
        create_migration_backup(&source, &database).unwrap();
        drop(source);
        let backup = Connection::open(database.with_extension("db.pre-migration.bak")).unwrap();
        let count: i64 = backup
            .query_row("SELECT COUNT(*) FROM fixture", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2);
        assert!(database
            .with_extension("db.pre-migration.bak.prev")
            .exists());
    }

    #[test]
    fn sqlite_healthy_quick_check_and_per_connection_safety_policy() {
        let directory = tempdir().expect("temp directory");
        let state = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let connection = state.open_connection().unwrap();
        assert_eq!(
            connection
                .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            1
        );
        assert_eq!(
            connection
                .query_row("PRAGMA journal_mode", [], |row| row.get::<_, String>(0))
                .unwrap()
                .to_ascii_lowercase(),
            "wal"
        );
        assert_eq!(
            connection
                .query_row("PRAGMA busy_timeout", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            5000
        );
        assert_eq!(
            connection
                .query_row("PRAGMA quick_check(1)", [], |row| row.get::<_, String>(0))
                .unwrap(),
            "ok"
        );
    }

    #[test]
    fn sqlite_corrupt_db_preflight_fails_without_changing_source_bytes() {
        let directory = tempdir().expect("temp directory");
        let db_path = directory.path().join("hiveai.db");
        let corrupt_bytes: Vec<u8> = b"THIS IS NOT A VALID SQLITE DATABASE FILE AT ALL".to_vec();
        fs::write(&db_path, &corrupt_bytes).unwrap();
        let before = fs::read(&db_path).unwrap();
        let result = DatabaseState::initialize(directory.path().to_path_buf());
        assert!(result.is_err());
        let after = fs::read(&db_path).unwrap();
        assert_eq!(before, after, "corrupt source bytes must be preserved");
    }

    #[test]
    fn sqlite_busy_locked_contention_respects_configured_bounds() {
        let directory = tempdir().expect("temp directory");
        let state = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let holder = state.open_connection().unwrap();
        holder.execute_batch("BEGIN IMMEDIATE").unwrap();
        let contender = state.open_connection().unwrap();
        let started = std::time::Instant::now();
        let result = contender.execute(
            "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('busy', 'busy', 'now', 'now')",
            [],
        );
        let elapsed = started.elapsed();
        assert!(result.is_err(), "contending write must return BUSY/LOCKED");
        let message = result.unwrap_err().to_string().to_ascii_uppercase();
        assert!(
            message.contains("BUSY") || message.contains("LOCKED"),
            "unexpected SQLite error: {message}"
        );
        assert!(
            elapsed >= Duration::from_millis(4500),
            "contention returned before configured bound: {elapsed:?}"
        );
        assert!(
            elapsed <= Duration::from_millis(7000),
            "contention exceeded bounded timeout: {elapsed:?}"
        );
        holder.execute_batch("ROLLBACK").unwrap();
    }

    #[test]
    fn sqlite_backup_publication_failure_preserves_source_and_prior_backup() {
        let directory = tempdir().expect("temp directory");
        let db_path = directory.path().join("hiveai.db");
        let source = Connection::open(&db_path).unwrap();
        configure_connection(&source).unwrap();
        source.execute("CREATE TABLE fixture (v TEXT)", []).unwrap();
        source
            .execute("INSERT INTO fixture VALUES ('original')", [])
            .unwrap();
        create_migration_backup(&source, &db_path).unwrap();
        let bak = db_path.with_extension("db.pre-migration.bak");
        assert!(bak.exists(), "first backup must be created");
        let first_count: i64 = Connection::open(&bak)
            .unwrap()
            .query_row("SELECT COUNT(*) FROM fixture", [], |r| r.get(0))
            .unwrap();
        assert_eq!(first_count, 1);
        let before_source = fs::read(&db_path).unwrap();
        let before_backup = fs::read(&bak).unwrap();
        source
            .execute("INSERT INTO fixture VALUES ('second')", [])
            .unwrap();
        FAIL_NEXT_BACKUP_PUBLICATION.with(|failpoint| failpoint.set(true));
        let result = create_migration_backup(&source, &db_path);
        assert!(
            result.is_err(),
            "publication failpoint must return an error"
        );
        assert_eq!(
            fs::read(&db_path).unwrap(),
            before_source,
            "source bytes changed"
        );
        assert_eq!(
            fs::read(&bak).unwrap(),
            before_backup,
            "prior good backup changed"
        );
        assert!(
            !db_path.with_extension("db.pre-migration.tmp").exists(),
            "temporary backup artifact remained"
        );
        drop(source);
        assert!(db_path.exists(), "source database must be preserved");
    }

    #[test]
    fn sqlite_backup_failure_prevents_migration_mutation() {
        let directory = tempdir().expect("temp directory");
        let db_path = directory.path().join("hiveai.db");
        let mut source = Connection::open(&db_path).unwrap();
        migrations::apply_migrations(&mut source, &migrations::migrations()[..6]).unwrap();
        source.execute("INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p6', 'P6', 'now', 'now')", []).unwrap();
        source.execute("INSERT INTO project_snapshots (id, project_id, availability, evidence_generated_at, watcher_health, created_at) VALUES ('s6', 'p6', 'AVAILABLE', 'now', 'HEALTHY', '1700000000')", []).unwrap();
        create_migration_backup(&source, &db_path).unwrap();
        drop(source);
        FAIL_NEXT_BACKUP_PUBLICATION.with(|failpoint| failpoint.set(true));
        let state_result = DatabaseState::initialize(directory.path().to_path_buf());
        assert!(
            state_result.is_err(),
            "injected backup failure must block initialization"
        );
        let reopened = Connection::open(&db_path).unwrap();
        let version: i64 = reopened
            .query_row("SELECT MAX(version) FROM migrations", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version, 6, "v7 must not be committed after backup failure");
        let created_at: String = reopened
            .query_row(
                "SELECT created_at FROM project_snapshots WHERE id = 's6'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            created_at, "1700000000",
            "v7 timestamp conversion must not run"
        );
        assert_eq!(
            reopened
                .query_row::<i64, _, _>(
                    "SELECT COUNT(*) FROM migrations WHERE version = 7",
                    [],
                    |row| row.get(0)
                )
                .unwrap(),
            0
        );
    }

    #[test]
    fn sqlite_open_connection_reports_fk_wal_busy_timeout_and_synchronous() {
        let directory = tempdir().expect("temp directory");
        let state = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let connection = state.open_connection().unwrap();
        assert_eq!(
            connection
                .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            1,
            "foreign keys must be enabled"
        );
        assert_eq!(
            connection
                .query_row("PRAGMA journal_mode", [], |row| row.get::<_, String>(0))
                .unwrap()
                .to_ascii_lowercase(),
            "wal",
            "journal mode must be WAL"
        );
        assert_eq!(
            connection
                .query_row("PRAGMA busy_timeout", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            5000,
            "busy timeout must be 5000ms"
        );
        assert_eq!(
            connection
                .query_row("PRAGMA synchronous", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            1,
            "synchronous must be NORMAL (1)"
        );
    }
}
