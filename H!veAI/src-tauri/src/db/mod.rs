mod migrations;

use migrations::MigrationReport;
use rusqlite::Connection;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

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
        let mut connection = Connection::open(&database_path)
            .map_err(|error| format!("open H!veAI database: {error}"))?;
        configure_connection(&connection)?;
        let integrity: String = connection
            .query_row("PRAGMA quick_check(1)", [], |row| row.get(0))
            .map_err(|error| format!("check H!veAI database integrity: {error}"))?;
        if integrity != "ok" {
            return Err(format!(
                "H!veAI database integrity check failed: {integrity}"
            ));
        }
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
            .run_to_completion(5, Duration::from_millis(25), None)
            .map_err(|error| format!("backup H!veAI database before migration: {error}"))
    };
    destination
        .close()
        .map_err(|(_, error)| format!("close H!veAI migration backup: {error}"))?;
    backup_result?;
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
        assert_eq!(status.schema_version, 7);
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
}
