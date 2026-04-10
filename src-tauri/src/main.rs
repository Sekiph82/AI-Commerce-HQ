#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Manager, State};

struct BackendProcess(Arc<Mutex<Option<Child>>>);

/// Starts the Python backend sidecar. Tries bundled exe first, falls back to `python`.
fn start_backend(app_handle: &tauri::AppHandle) -> Option<Child> {
    let data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    // Try bundled backend first (production)
    let bundled_path = app_handle
        .path_resolver()
        .resolve_resource("backend/backend.exe")
        .or_else(|| app_handle.path_resolver().resolve_resource("backend/backend"));

    let child = if let Some(path) = bundled_path {
        if path.exists() {
            Command::new(&path)
                .env("HQ_DATA_DIR", &data_dir)
                .spawn()
                .ok()
        } else {
            None
        }
    } else {
        None
    };

    if child.is_some() {
        return child;
    }

    // Development fallback: try python / python3 in the project directory
    let backend_script = std::env::current_dir()
        .unwrap_or_default()
        .join("backend")
        .join("main.py");

    for python_cmd in &["python", "python3", "py"] {
        if let Ok(c) = Command::new(python_cmd)
            .arg(&backend_script)
            .env("HQ_DATA_DIR", &data_dir)
            .spawn()
        {
            println!("[HQ] Started Python backend via {}", python_cmd);
            return Some(c);
        }
    }

    eprintln!("[HQ] WARNING: Could not start Python backend. Please start it manually.");
    None
}

/// Poll until the backend is ready (max 30 seconds)
async fn wait_for_backend() -> bool {
    let client = reqwest::Client::new();
    for _ in 0..30 {
        if client
            .get("http://localhost:8765/api/health")
            .timeout(Duration::from_secs(1))
            .send()
            .await
            .is_ok()
        {
            return true;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    false
}

#[tauri::command]
fn get_backend_status() -> bool {
    // Simple sync check
    reqwest::blocking::get("http://localhost:8765/api/health")
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

#[tauri::command]
async fn restart_backend(state: State<'_, BackendProcess>, app_handle: tauri::AppHandle) -> Result<String, String> {
    let mut lock = state.0.lock().unwrap();
    if let Some(mut child) = lock.take() {
        let _ = child.kill();
    }
    let new_child = start_backend(&app_handle);
    *lock = new_child;
    Ok("Restarted".to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            // Start backend in background
            let child = start_backend(&handle);
            app.manage(BackendProcess(Arc::new(Mutex::new(child))));

            // Wait for backend in a separate thread, then open window
            let handle2 = handle.clone();
            tauri::async_runtime::spawn(async move {
                println!("[HQ] Waiting for backend to be ready...");
                let ready = wait_for_backend().await;
                if ready {
                    println!("[HQ] Backend ready!");
                } else {
                    eprintln!("[HQ] Backend did not start in time. App will continue.");
                }
                // The window is already open from Tauri default behaviour.
                // We just emit readiness event to the frontend.
                handle2
                    .emit_all("backend-ready", ready)
                    .unwrap_or_default();
            });

            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::Destroyed = event.event() {
                // App closing - kill backend
                if let Some(state) = event.window().try_state::<BackendProcess>() {
                    let mut lock = state.0.lock().unwrap();
                    if let Some(mut child) = lock.take() {
                        let _ = child.kill();
                        println!("[HQ] Backend process terminated.");
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![get_backend_status, restart_backend])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
