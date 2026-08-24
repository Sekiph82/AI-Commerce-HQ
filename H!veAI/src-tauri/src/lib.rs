use serde::Serialize;
use tauri::{Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};

mod db;
mod git_engine;
mod projects;
mod runtime;
mod time;
mod watcher;
use db::{DatabaseState, DatabaseStatus};
use projects::{
    ProjectListQuery, ProjectRecord, RegisterProjectRequest, RepairProjectPathRequest,
    UpdateProjectSettingsRequest,
};
use runtime::{RuntimeStatus, RuntimeSupervisor};
use watcher::{ProjectWatcherStatus, WatcherManager, WatcherStatusSummary};

use git_engine::{GitDiff, GitDiffRequest, GitSnapshot, GitSnapshotRequest, MutationStatus};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NativeStatus {
    product_name: String,
    identifier: String,
    version: String,
    platform: String,
    app_data_dir: Option<String>,
    log_dir: Option<String>,
}

#[tauri::command]
fn hiveai_native_status(app: tauri::AppHandle) -> NativeStatus {
    log::info!("H!veAI native status requested.");
    log::info!("HIVEAI_FRONTEND_READY");

    let config = app.config();
    let package = app.package_info();
    let app_data_dir = app
        .path()
        .app_data_dir()
        .ok()
        .map(|path| path.to_string_lossy().into_owned());
    let log_dir = app
        .path()
        .app_log_dir()
        .ok()
        .map(|path| path.to_string_lossy().into_owned());

    NativeStatus {
        product_name: config
            .product_name
            .clone()
            .unwrap_or_else(|| "H!veAI".to_string()),
        identifier: config.identifier.clone(),
        version: package.version.to_string(),
        platform: std::env::consts::OS.to_string(),
        app_data_dir,
        log_dir,
    }
}

#[tauri::command]
fn hiveai_frontend_ready() {
    log::info!("HIVEAI_FRONTEND_READY");
}

#[tauri::command]
fn hiveai_request_restart(app: tauri::AppHandle) {
    log::info!("H!veAI restart requested through native foundation command.");
    let _ = app.emit("hiveai-restart-requested", ());
    app.request_restart();
}

#[tauri::command]
fn hiveai_runtime_status(supervisor: tauri::State<'_, RuntimeSupervisor>) -> RuntimeStatus {
    log::info!("H!veAI runtime status requested.");
    supervisor.status()
}

#[tauri::command]
fn hiveai_database_status(database: tauri::State<'_, DatabaseState>) -> DatabaseStatus {
    log::info!("H!veAI database status requested.");
    log::info!("HIVEAI_FRONTEND_READY");
    database.status()
}

#[tauri::command]
fn hiveai_projects_list(
    database: tauri::State<'_, DatabaseState>,
    query: Option<ProjectListQuery>,
) -> Result<Vec<ProjectRecord>, String> {
    projects::list_projects(&database, query.unwrap_or_default())
}

#[tauri::command]
fn hiveai_project_register(
    database: tauri::State<'_, DatabaseState>,
    request: RegisterProjectRequest,
) -> Result<ProjectRecord, String> {
    projects::register_project(&database, request)
}

#[tauri::command]
fn hiveai_project_get(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
) -> Result<ProjectRecord, String> {
    projects::fetch_project(&database, &project_id)
}

#[tauri::command]
fn hiveai_project_update_settings(
    database: tauri::State<'_, DatabaseState>,
    request: UpdateProjectSettingsRequest,
) -> Result<ProjectRecord, String> {
    projects::update_project_settings(&database, request)
}

#[tauri::command]
fn hiveai_project_archive(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
) -> Result<ProjectRecord, String> {
    projects::archive_project(&database, &project_id)
}

#[tauri::command]
fn hiveai_project_remove_from_registry(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
) -> Result<(), String> {
    projects::remove_project(&database, &project_id)
}

#[tauri::command]
fn hiveai_project_repair_path(
    database: tauri::State<'_, DatabaseState>,
    request: RepairProjectPathRequest,
) -> Result<ProjectRecord, String> {
    projects::repair_project_path(&database, request)
}

#[tauri::command]
fn hiveai_git_snapshot(
    database: tauri::State<'_, DatabaseState>,
    request: GitSnapshotRequest,
) -> Result<GitSnapshot, String> {
    git_engine::snapshot(&database, request)
}

#[tauri::command]
fn hiveai_git_diff(
    database: tauri::State<'_, DatabaseState>,
    request: GitDiffRequest,
) -> Result<GitDiff, String> {
    git_engine::diff(&database, request)
}

#[tauri::command]
fn hiveai_git_mutation_status() -> MutationStatus {
    git_engine::mutation_status()
}

#[tauri::command]
fn hiveai_watcher_status(watcher: tauri::State<'_, WatcherManager>) -> WatcherStatusSummary {
    watcher.status()
}

#[tauri::command]
fn hiveai_watcher_project_status(
    watcher: tauri::State<'_, WatcherManager>,
    project_id: String,
) -> Result<ProjectWatcherStatus, String> {
    watcher.project_status(&project_id)
}

#[tauri::command]
fn hiveai_watcher_refresh_set(
    watcher: tauri::State<'_, WatcherManager>,
) -> Result<WatcherStatusSummary, String> {
    watcher.refresh_from_registry()
}

#[tauri::command]
fn hiveai_watcher_rescan(
    watcher: tauri::State<'_, WatcherManager>,
    project_id: String,
) -> Result<ProjectWatcherStatus, String> {
    watcher.rescan_project(&project_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(Target::new(TargetKind::LogDir {
                    file_name: Some("hiveai".into()),
                }))
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            hiveai_native_status,
            hiveai_frontend_ready,
            hiveai_request_restart,
            hiveai_runtime_status,
            hiveai_database_status,
            hiveai_projects_list,
            hiveai_project_register,
            hiveai_project_get,
            hiveai_project_update_settings,
            hiveai_project_archive,
            hiveai_project_remove_from_registry,
            hiveai_project_repair_path,
            hiveai_git_snapshot,
            hiveai_git_diff,
            hiveai_git_mutation_status,
            hiveai_watcher_status,
            hiveai_watcher_project_status,
            hiveai_watcher_refresh_set,
            hiveai_watcher_rescan
        ])
        .setup(|app| {
            app.manage(RuntimeSupervisor::new());
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|error| format!("resolve H!veAI app-data directory: {error}"))?;
            let database = DatabaseState::initialize(app_data_dir.clone())
                .map_err(|error| format!("H!veAI persistence initialization failed: {error}"))?;
            let database_status = database.status();
            let watcher_manager = WatcherManager::initialize(database.clone())
                .map_err(|error| format!("H!veAI watcher initialization failed: {error}"))?;
            app.manage(database);
            app.manage(watcher_manager);

            log::info!(
                "H!veAI Tauri 2 foundation started; app_data_dir={}; schema_version={}",
                app_data_dir.to_string_lossy(),
                database_status.schema_version
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running H!veAI Tauri application");
}
