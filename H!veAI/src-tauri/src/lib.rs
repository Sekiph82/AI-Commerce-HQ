use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};

mod db;
mod external_browser;
mod git_engine;
mod projects;
mod runtime;
mod task_intelligence;
mod task_sources;
mod time;
mod watcher;
mod workflow;
use db::{DatabaseState, DatabaseStatus};
use projects::{
    ProjectListQuery, ProjectRecord, RegisterProjectRequest, RepairProjectPathRequest,
    UpdateProjectSettingsRequest,
};
use runtime::{RuntimeStatus, RuntimeSupervisor};
use task_intelligence::TaskIntelligenceSnapshot;
use task_sources::{
    CustomPathRequest, CustomPathUpdateRequest, CustomSourcePath, DiscoveredProjectSource,
};
use watcher::{ProjectWatcherStatus, WatcherManager, WatcherStatusSummary};
use workflow::{
    WorkflowEvent, WorkflowHistoryQuery, WorkflowOverrideRequest, WorkflowProjectList,
    WorkflowProjectListQuery, WorkflowTask, WorkflowTransitionRequest,
};

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

#[derive(Default)]
struct StartupIntroState {
    claimed: AtomicBool,
}

impl StartupIntroState {
    fn claim(&self) -> bool {
        !self.claimed.swap(true, Ordering::AcqRel)
    }
}

fn claim_startup_intro(state: &StartupIntroState) -> bool {
    state.claim()
}

#[tauri::command]
fn hiveai_native_status(app: tauri::AppHandle) -> NativeStatus {
    log::info!("H!veAI native status requested.");

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
fn hiveai_request_restart(app: tauri::AppHandle) {
    log::info!("H!veAI restart requested through native foundation command.");
    let _ = app.emit("hiveai-restart-requested", ());
    app.request_restart();
}

#[tauri::command]
fn hiveai_frontend_ready() {
    log::info!("HIVEAI_FRONTEND_READY");
}

#[tauri::command]
fn hiveai_open_akilta() -> Result<(), String> {
    external_browser::open_akilta()
}

#[tauri::command]
fn hiveai_startup_intro_claim(state: tauri::State<'_, StartupIntroState>) -> bool {
    claim_startup_intro(&state)
}

#[tauri::command]
fn hiveai_runtime_status(supervisor: tauri::State<'_, RuntimeSupervisor>) -> RuntimeStatus {
    log::info!("H!veAI runtime status requested.");
    supervisor.status()
}

#[tauri::command]
fn hiveai_database_status(database: tauri::State<'_, DatabaseState>) -> DatabaseStatus {
    log::info!("H!veAI database status requested.");
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

#[tauri::command]
fn hiveai_task_sources_discover(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
) -> Result<Vec<DiscoveredProjectSource>, String> {
    task_sources::discover(&database, &project_id)
}

#[tauri::command]
fn hiveai_task_sources_list(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
) -> Result<Vec<DiscoveredProjectSource>, String> {
    task_sources::list(&database, &project_id)
}

#[tauri::command]
fn hiveai_task_source_custom_paths_list(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
) -> Result<Vec<CustomSourcePath>, String> {
    task_sources::custom_paths_list(&database, &project_id)
}

#[tauri::command]
fn hiveai_task_source_custom_path_add(
    database: tauri::State<'_, DatabaseState>,
    request: CustomPathRequest,
) -> Result<Vec<CustomSourcePath>, String> {
    task_sources::custom_path_add(&database, request)
}

#[tauri::command]
fn hiveai_task_source_custom_path_remove(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
    path_or_id: String,
) -> Result<Vec<CustomSourcePath>, String> {
    task_sources::custom_path_remove(&database, &project_id, &path_or_id)
}

#[tauri::command]
fn hiveai_task_source_custom_path_update(
    database: tauri::State<'_, DatabaseState>,
    request: CustomPathUpdateRequest,
) -> Result<Vec<CustomSourcePath>, String> {
    task_sources::custom_path_update(&database, request)
}

#[tauri::command]
fn hiveai_task_intelligence_parse(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
) -> Result<TaskIntelligenceSnapshot, String> {
    task_intelligence::parse(&database, &project_id)
}

#[tauri::command]
fn hiveai_task_intelligence_list(
    database: tauri::State<'_, DatabaseState>,
    project_id: String,
) -> Result<TaskIntelligenceSnapshot, String> {
    task_intelligence::list(&database, &project_id)
}

#[tauri::command]
fn hiveai_workflow_task_get(
    database: tauri::State<'_, DatabaseState>,
    task_id: String,
) -> Result<WorkflowTask, String> {
    workflow::task_get(&database, task_id)
}

#[tauri::command]
fn hiveai_workflow_project_list(
    database: tauri::State<'_, DatabaseState>,
    query: WorkflowProjectListQuery,
) -> Result<WorkflowProjectList, String> {
    workflow::project_list(&database, query)
}

#[tauri::command]
fn hiveai_workflow_history(
    database: tauri::State<'_, DatabaseState>,
    query: WorkflowHistoryQuery,
) -> Result<Vec<WorkflowEvent>, String> {
    workflow::history(&database, query)
}

#[tauri::command]
fn hiveai_workflow_transition(
    database: tauri::State<'_, DatabaseState>,
    request: WorkflowTransitionRequest,
) -> Result<WorkflowEvent, String> {
    workflow::transition(&database, request)
}

#[tauri::command]
fn hiveai_workflow_override(
    database: tauri::State<'_, DatabaseState>,
    request: WorkflowOverrideRequest,
) -> Result<WorkflowEvent, String> {
    workflow::override_state(&database, request)
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
            hiveai_open_akilta,
            hiveai_startup_intro_claim,
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
            hiveai_watcher_rescan,
            hiveai_task_sources_discover,
            hiveai_task_sources_list,
            hiveai_task_source_custom_paths_list,
            hiveai_task_source_custom_path_add,
            hiveai_task_source_custom_path_remove,
            hiveai_task_source_custom_path_update,
            hiveai_task_intelligence_parse,
            hiveai_task_intelligence_list,
            hiveai_workflow_task_get,
            hiveai_workflow_project_list,
            hiveai_workflow_history,
            hiveai_workflow_transition,
            hiveai_workflow_override
        ])
        .setup(|app| {
            app.manage(StartupIntroState::default());
            app.manage(RuntimeSupervisor::new());
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|error| format!("resolve H!veAI app-data directory: {error}"))?;
            let database = DatabaseState::initialize(app_data_dir.clone())
                .map_err(|error| format!("H!veAI persistence initialization failed: {error}"))?;
            workflow::recover_stale(&database)
                .map_err(|error| format!("H!veAI workflow recovery failed: {error}"))?;
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

#[cfg(test)]
mod startup_intro_tests {
    use super::{claim_startup_intro, StartupIntroState};

    #[test]
    fn fresh_state_first_claim_is_true() {
        assert!(claim_startup_intro(&StartupIntroState::default()));
    }

    #[test]
    fn same_state_second_claim_is_false() {
        let state = StartupIntroState::default();
        assert!(claim_startup_intro(&state));
        assert!(!claim_startup_intro(&state));
    }

    #[test]
    fn separately_constructed_state_claims_fresh_lifecycle() {
        let first = StartupIntroState::default();
        let second = StartupIntroState::default();
        assert!(claim_startup_intro(&first));
        assert!(claim_startup_intro(&second));
    }

    #[test]
    fn production_claim_path_uses_native_state() {
        let state = StartupIntroState::default();
        assert!(claim_startup_intro(&state));
        assert!(!claim_startup_intro(&state));
    }
}
