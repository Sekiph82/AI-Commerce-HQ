mod migrations;

use migrations::MigrationReport;
use rusqlite::Connection;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

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
        let report = migrations::apply_migrations(&mut connection, migrations::migrations())
            .map_err(|error| format!("run H!veAI database migrations: {error}"))?;
        let foreign_keys_enabled = connection
            .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
            .map_err(|error| format!("inspect H!veAI database safety pragmas: {error}"))?
            == 1;
        Ok(Self {
            status: status_from_report(report, foreign_keys_enabled),
            database_path,
        })
    }

    pub fn status(&self) -> DatabaseStatus {
        self.status.clone()
    }

    pub(crate) fn open_connection(&self) -> Result<Connection, String> {
        let connection = Connection::open(&self.database_path)
            .map_err(|error| format!("open H!veAI database connection: {error}"))?;
        connection
            .pragma_update(None, "foreign_keys", true)
            .map_err(|error| format!("enable H!veAI database foreign keys: {error}"))?;
        Ok(connection)
    }
}

fn status_from_report(report: MigrationReport, foreign_keys_enabled: bool) -> DatabaseStatus {
    DatabaseStatus {
        initialized: true,
        engine: "SQLite".to_string(),
        schema_version: report.schema_version,
        migration_count: report.migration_count,
        database_path: "hiveai.db".to_string(),
        foreign_keys_enabled,
        last_migration_status: report.last_migration_status.to_string(),
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
        assert_eq!(status.schema_version, 3);
        assert!(status.foreign_keys_enabled);
        assert!(directory.path().join("hiveai.db").exists());
    }
}
