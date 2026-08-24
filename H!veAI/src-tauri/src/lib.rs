use serde::Serialize;
use tauri::{Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};

mod runtime;
use runtime::{RuntimeStatus, RuntimeSupervisor};

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
fn hiveai_runtime_status(supervisor: tauri::State<'_, RuntimeSupervisor>) -> RuntimeStatus {
    log::info!("H!veAI runtime status requested.");
    supervisor.status()
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
            hiveai_request_restart,
            hiveai_runtime_status
        ])
        .setup(|app| {
            app.manage(RuntimeSupervisor::new());
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map(|path| path.to_string_lossy().into_owned())
                .unwrap_or_else(|error| format!("unavailable: {error}"));

            log::info!("H!veAI Tauri 2 foundation started; app_data_dir={app_data_dir}");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running H!veAI Tauri application");
}
