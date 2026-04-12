#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::TcpStream;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Manager, State};

struct BackendProcess(Arc<Mutex<Option<Child>>>);

/// Returns true if something is already listening on port 8765.
/// Used to avoid spawning a second backend when dev.py already started one.
fn backend_port_open() -> bool {
    TcpStream::connect_timeout(
        &"127.0.0.1:8765".parse().unwrap(),
        Duration::from_millis(400),
    )
    .is_ok()
}

/// Resolve and spawn the bundled backend.exe from app resources.
fn spawn_backend(app_handle: &tauri::AppHandle) -> Option<Child> {
    let data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    // Production: bundled backend.exe lives in the app resource directory
    let bundled = app_handle
        .path_resolver()
        .resolve_resource("backend.exe");

    if let Some(path) = bundled {
        if path.exists() {
            match Command::new(&path)
                .env("HQ_DATA_DIR", &data_dir)
                .spawn()
            {
                Ok(child) => {
                    println!("[HQ] Backend started from resources: {:?}", path);
                    return Some(child);
                }
                Err(e) => eprintln!("[HQ] Failed to start bundled backend: {}", e),
            }
        }
    }

    // Development fallback: run Python directly (never reached in packaged app).
    // In Tauri dev mode the exe lives at src-tauri/target/debug/<name>.exe,
    // so the project root is three directories up.
    #[cfg(debug_assertions)]
    {
        let project_root = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf())) // debug/
            .and_then(|p| p.parent().map(|p| p.to_path_buf())) // target/
            .and_then(|p| p.parent().map(|p| p.to_path_buf())) // src-tauri/
            .and_then(|p| p.parent().map(|p| p.to_path_buf())) // project root
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        let script = project_root.join("backend").join("main.py");
        println!("[HQ] Dev: looking for backend at {:?}", script);

        for py in &["python", "python3", "py"] {
            if let Ok(child) = Command::new(py)
                .arg(&script)
                .current_dir(&project_root.join("backend"))
                .env("HQ_DATA_DIR", &data_dir)
                .spawn()
            {
                println!("[HQ] Dev: started Python backend via {}", py);
                return Some(child);
            }
        }
    }

    eprintln!("[HQ] Could not start backend — app will continue without it.");
    None
}

/// Poll the health endpoint until the backend responds or we time out.
async fn wait_for_backend(timeout_secs: u64) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap_or_default();

    for _ in 0..timeout_secs {
        if client
            .get("http://localhost:8765/api/health")
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
        {
            return true;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    false
}

/// Invoked by the frontend to check backend health.
#[tauri::command]
async fn get_backend_status() -> bool {
    wait_for_backend(3).await
}

/// Invoked by the frontend crash-recovery UI.
#[tauri::command]
async fn restart_backend(
    state: State<'_, BackendProcess>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // Kill current process
    {
        let mut lock = state.0.lock().map_err(|e| e.to_string())?;
        if let Some(mut child) = lock.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    // Brief pause so port 8765 is freed
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Respawn
    let new_child = spawn_backend(&app_handle);
    {
        let mut lock = state.0.lock().map_err(|e| e.to_string())?;
        *lock = new_child;
    }

    // Wait for health, then tell the frontend
    let ready = wait_for_backend(30).await;
    app_handle
        .emit_all("backend-ready", ready)
        .map_err(|e| e.to_string())?;

    if ready {
        Ok("Backend restarted successfully.".into())
    } else {
        Err("Backend restarted but did not become healthy within 30 s.".into())
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            // Only spawn the backend if nothing is already listening on port 8765.
            // When launched via start-dev.bat / dev.py the backend is already running,
            // so Tauri must reuse it rather than spawning a second process.
            let child = if backend_port_open() {
                println!("[HQ] Port 8765 already in use — reusing existing backend.");
                None
            } else {
                spawn_backend(&handle)
            };
            app.manage(BackendProcess(Arc::new(Mutex::new(child))));

            // In a background task: wait for health then emit "backend-ready"
            // so the splash screen can proceed.
            let handle2 = handle.clone();
            tauri::async_runtime::spawn(async move {
                let ready = wait_for_backend(45).await;
                if ready {
                    println!("[HQ] Backend is healthy — notifying frontend.");
                } else {
                    eprintln!("[HQ] Backend health check timed out after 45 s.");
                }
                let _ = handle2.emit_all("backend-ready", ready);
            });

            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::Destroyed = event.event() {
                // Kill the backend when the last window closes
                if let Some(state) = event.window().try_state::<BackendProcess>() {
                    if let Ok(mut lock) = state.0.lock() {
                        if let Some(mut child) = lock.take() {
                            let _ = child.kill();
                            let _ = child.wait();
                            println!("[HQ] Backend process terminated on exit.");
                        }
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![get_backend_status, restart_backend])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
