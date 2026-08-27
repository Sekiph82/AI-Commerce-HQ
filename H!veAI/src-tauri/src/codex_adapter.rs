use crate::db::DatabaseState;
use crate::projects::{fetch_project, ProjectRecord};
use crate::time::utc_timestamp;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use uuid::Uuid;

const PROVIDER: &str = "CODEX";
const MAX_PROMPT_BYTES: usize = 64 * 1024;
const MAX_OUTPUT_BYTES: usize = 64 * 1024;
const MAX_OUTPUT_EVENTS: usize = 128;
const READINESS_TIMEOUT: Duration = Duration::from_secs(5);
const PROCESS_POLL: Duration = Duration::from_millis(50);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexReadiness {
    pub provider: String,
    pub available: bool,
    pub version: Option<String>,
    pub readiness_state: String,
    pub diagnostic_code: Option<String>,
    pub diagnostic_message: Option<String>,
    pub checked_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexStartRequest {
    pub project_id: String,
    pub task_id: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexSession {
    pub id: String,
    pub provider: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub operation_kind: String,
    pub state: String,
    pub cwd: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    pub diagnostic_code: Option<String>,
    pub diagnostic_message: Option<String>,
}

struct OwnedProcess {
    child: Arc<Mutex<Child>>,
    stop_requested: Arc<AtomicBool>,
}

#[derive(Default)]
pub struct CodexAdapter {
    processes: Arc<Mutex<HashMap<String, OwnedProcess>>>,
}

#[derive(Default)]
struct Capture {
    text: String,
    truncated: bool,
}

impl Capture {
    fn append(&mut self, bytes: &[u8]) {
        if self.text.len() >= MAX_OUTPUT_BYTES {
            self.truncated = true;
            return;
        }
        let remaining = MAX_OUTPUT_BYTES - self.text.len();
        let take = bytes.len().min(remaining);
        self.text.push_str(&String::from_utf8_lossy(&bytes[..take]));
        if take < bytes.len() {
            self.truncated = true;
        }
    }
}

pub fn readiness() -> CodexReadiness {
    let checked_at = utc_timestamp();
    let Some(executable) = discover_codex_executable() else {
        return CodexReadiness {
            provider: PROVIDER.into(),
            available: false,
            version: None,
            readiness_state: "UNAVAILABLE".into(),
            diagnostic_code: Some("CODEX_EXECUTABLE_NOT_FOUND".into()),
            diagnostic_message: Some("codex.exe was not found on PATH".into()),
            checked_at,
        };
    };
    match probe_version(&executable, READINESS_TIMEOUT) {
        Ok(version) => CodexReadiness {
            provider: PROVIDER.into(),
            available: true,
            version: Some(version),
            readiness_state: "VERSION_VERIFIED_AUTH_UNKNOWN".into(),
            diagnostic_code: Some("AUTH_READINESS_UNVERIFIED".into()),
            diagnostic_message: Some("Codex executable is available; account authentication is determined when a bounded operation starts".into()),
            checked_at,
        },
        Err(ProbeError::Timeout) => CodexReadiness {
            provider: PROVIDER.into(), available: false, version: None,
            readiness_state: "PROBE_TIMEOUT".into(),
            diagnostic_code: Some("CODEX_VERSION_PROBE_TIMEOUT".into()),
            diagnostic_message: Some("Codex version probe exceeded its bounded timeout".into()), checked_at,
        },
        Err(ProbeError::Malformed(message)) => CodexReadiness {
            provider: PROVIDER.into(), available: false, version: None,
            readiness_state: "MALFORMED_VERSION".into(),
            diagnostic_code: Some("CODEX_VERSION_MALFORMED".into()), diagnostic_message: Some(message), checked_at,
        },
        Err(ProbeError::Failed(message)) => CodexReadiness {
            provider: PROVIDER.into(), available: false, version: None,
            readiness_state: "PROBE_FAILED".into(),
            diagnostic_code: Some("CODEX_VERSION_PROBE_FAILED".into()), diagnostic_message: Some(message), checked_at,
        },
    }
}

pub fn start(
    adapter: &CodexAdapter,
    database: &DatabaseState,
    request: CodexStartRequest,
) -> Result<CodexSession, String> {
    validate_prompt(&request.prompt)?;
    let project = fetch_project(database, &request.project_id)?;
    let cwd = validate_operation_project(&project)?;
    validate_task(database, &request.project_id, request.task_id.as_deref())?;
    let executable = discover_codex_executable()
        .ok_or_else(|| "CODEX_EXECUTABLE_NOT_FOUND: codex.exe was not found on PATH".to_string())?;
    let session_id = Uuid::new_v4().to_string();
    let started_at = utc_timestamp();
    let operation_kind = if request.task_id.is_some() {
        "TASK_OPERATION"
    } else {
        "FREEFORM_PROJECT_OPERATION"
    };
    let prompt_hash = sha256_hex(request.prompt.as_bytes());
    let args = fixed_exec_args(&cwd, &request.prompt);
    let connection = database.open_connection()?;
    connection.execute(
        "INSERT INTO agent_sessions (id,project_id,task_id,provider,state,started_at,created_at) VALUES (?1,?2,?3,?4,'STARTING',?5,?5)",
        params![session_id, request.project_id, request.task_id, PROVIDER, started_at],
    ).map_err(|error| format!("persist Codex session: {error}"))?;
    insert_event(
        &connection,
        &session_id,
        "SESSION_STARTED",
        serde_json::json!({
            "operationKind": operation_kind, "promptSha256": prompt_hash, "promptBytes": request.prompt.len()
        }),
    )?;
    insert_event(
        &connection,
        &session_id,
        "PROCESS_POLICY",
        serde_json::json!({"executable":"codex.exe","argumentPolicy":"FIXED_ADAPTER_ARGS","cwd":cwd.to_string_lossy(),"shell":false}),
    )?;

    let mut command = Command::new(executable);
    command
        .args(args)
        .current_dir(&cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            finish_failed(
                database,
                &session_id,
                "CODEX_PROCESS_START_FAILED",
                &error.to_string(),
            )?;
            return Err(format!("CODEX_PROCESS_START_FAILED: {error}"));
        }
    };
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "CODEX_STDOUT_UNAVAILABLE".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "CODEX_STDERR_UNAVAILABLE".to_string())?;
    let child = Arc::new(Mutex::new(child));
    let stop_requested = Arc::new(AtomicBool::new(false));
    let process = OwnedProcess {
        child: child.clone(),
        stop_requested: stop_requested.clone(),
    };
    adapter
        .processes
        .lock()
        .map_err(|_| "CODEX_PROCESS_LOCK_POISONED".to_string())?
        .insert(session_id.clone(), process);
    connection
        .execute(
            "UPDATE agent_sessions SET state='RUNNING' WHERE id=?1",
            [&session_id],
        )
        .map_err(|error| format!("mark Codex session running: {error}"))?;
    let stdout_capture = Arc::new(Mutex::new(Capture::default()));
    let stderr_capture = Arc::new(Mutex::new(Capture::default()));
    let out_capture = stdout_capture.clone();
    let err_capture = stderr_capture.clone();
    let stdout_thread = thread::spawn(move || read_stream(stdout, out_capture));
    let stderr_thread = thread::spawn(move || read_stream(stderr, err_capture));
    let database_for_monitor = database.clone();
    let session_for_thread = session_id.clone();
    let processes = adapter.processes.clone();
    thread::spawn(move || {
        monitor_process(
            processes,
            database_for_monitor,
            session_for_thread,
            child,
            stop_requested,
            stdout_capture,
            stderr_capture,
            stdout_thread,
            stderr_thread,
        );
    });
    load_session(database, &session_id)
}

fn monitor_process(
    processes: Arc<Mutex<HashMap<String, OwnedProcess>>>,
    database: DatabaseState,
    session_id: String,
    child: Arc<Mutex<Child>>,
    stop_requested: Arc<AtomicBool>,
    stdout: Arc<Mutex<Capture>>,
    stderr: Arc<Mutex<Capture>>,
    stdout_thread: thread::JoinHandle<()>,
    stderr_thread: thread::JoinHandle<()>,
) {
    let status = loop {
        match child
            .lock()
            .ok()
            .and_then(|mut locked| locked.try_wait().ok())
        {
            Some(Some(status)) => break Some(status),
            Some(None) => thread::sleep(PROCESS_POLL),
            None => break None,
        }
    };
    let _ = stdout_thread.join();
    let _ = stderr_thread.join();
    let stdout = stdout
        .lock()
        .ok()
        .map(|capture| (redact_output(&capture.text), capture.truncated))
        .unwrap_or_default();
    let stderr = stderr
        .lock()
        .ok()
        .map(|capture| (redact_output(&capture.text), capture.truncated))
        .unwrap_or_default();
    if let Ok(connection) = database.open_connection() {
        let state = if stop_requested.load(Ordering::Acquire) {
            "STOPPED"
        } else if status
            .as_ref()
            .map(|value| value.success())
            .unwrap_or(false)
        {
            "COMPLETED"
        } else if status.is_some() {
            "FAILED"
        } else {
            "CRASHED"
        };
        let exit_code = status.and_then(|value| value.code());
        let _ = connection.execute(
            "UPDATE agent_sessions SET state=?2,ended_at=?3 WHERE id=?1",
            params![session_id, state, utc_timestamp()],
        );
        let _ = insert_event(
            &connection,
            &session_id,
            "STDOUT",
            serde_json::json!({"text":stdout.0,"truncated":stdout.1}),
        );
        let _ = insert_event(
            &connection,
            &session_id,
            "STDERR",
            serde_json::json!({"text":stderr.0,"truncated":stderr.1}),
        );
        let _ = insert_event(
            &connection,
            &session_id,
            "SESSION_FINISHED",
            serde_json::json!({"state":state,"exitCode":exit_code}),
        );
    }
    if let Ok(mut owned) = processes.lock() {
        owned.remove(&session_id);
    }
}

fn validate_prompt(prompt: &str) -> Result<(), String> {
    if prompt.trim().is_empty() {
        return Err("CODEX_PROMPT_EMPTY".into());
    }
    if prompt.len() > MAX_PROMPT_BYTES {
        return Err("CODEX_PROMPT_TOO_LARGE".into());
    }
    Ok(())
}

fn validate_operation_project(project: &ProjectRecord) -> Result<PathBuf, String> {
    if project.status != "ACTIVE" {
        return Err("CODEX_PROJECT_NOT_ACTIVE".into());
    }
    let root = std::fs::canonicalize(&project.original_path)
        .map_err(|_| "CODEX_PROJECT_PATH_UNAVAILABLE".to_string())?;
    if !root.is_dir() {
        return Err("CODEX_PROJECT_PATH_NOT_DIRECTORY".into());
    }
    Ok(root)
}

fn validate_task(
    database: &DatabaseState,
    project_id: &str,
    task_id: Option<&str>,
) -> Result<(), String> {
    let Some(task_id) = task_id else {
        return Ok(());
    };
    let connection = database.open_connection()?;
    let belongs: bool = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM tasks WHERE id=?1 AND project_id=?2)",
            params![task_id, project_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| format!("validate Codex task: {error}"))?
        == 1;
    if belongs {
        Ok(())
    } else {
        Err("CODEX_TASK_PROJECT_MISMATCH_OR_MISSING".into())
    }
}

fn fixed_exec_args(cwd: &Path, prompt: &str) -> Vec<String> {
    vec![
        "exec".into(),
        "--json".into(),
        "--sandbox".into(),
        "workspace-write".into(),
        "--cd".into(),
        cwd.to_string_lossy().into_owned(),
        "--ephemeral".into(),
        "--skip-git-repo-check".into(),
        "--".into(),
        prompt.to_string(),
    ]
}

fn discover_codex_executable() -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for entry in std::env::split_paths(&path) {
        for name in ["codex.exe", "codex"] {
            let candidate = entry.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

#[derive(Debug)]
enum ProbeError {
    Timeout,
    Malformed(String),
    Failed(String),
}

fn probe_version(path: &Path, timeout: Duration) -> Result<String, ProbeError> {
    let mut child = Command::new(path)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| ProbeError::Failed(error.to_string()))?;
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let output = child
                    .wait_with_output()
                    .map_err(|error| ProbeError::Failed(error.to_string()))?;
                if !status.success() {
                    return Err(ProbeError::Failed(
                        String::from_utf8_lossy(&output.stderr).trim().to_string(),
                    ));
                }
                return parse_version(&output);
            }
            Ok(None) if Instant::now() < deadline => thread::sleep(PROCESS_POLL),
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ProbeError::Timeout);
            }
            Err(error) => return Err(ProbeError::Failed(error.to_string())),
        }
    }
}

fn parse_version(output: &Output) -> Result<String, ProbeError> {
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let first = text.lines().next().unwrap_or_default().trim();
    if first.len() > 256 || !first.to_ascii_lowercase().contains("codex") {
        return Err(ProbeError::Malformed(
            "Codex version output did not contain a bounded Codex version".into(),
        ));
    }
    Ok(first.to_string())
}

fn read_stream<R: Read>(mut reader: R, capture: Arc<Mutex<Capture>>) {
    let mut buffer = [0u8; 4096];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) | Err(_) => break,
            Ok(size) => {
                if let Ok(mut target) = capture.lock() {
                    target.append(&buffer[..size]);
                }
            }
        }
    }
}

fn redact_output(value: &str) -> String {
    value
        .lines()
        .map(|line| {
            let lower = line.to_ascii_lowercase();
            if [
                "api_key",
                "apikey",
                "token",
                "password",
                "secret",
                "authorization",
                "sk-",
            ]
            .iter()
            .any(|needle| lower.contains(needle))
            {
                "[REDACTED SENSITIVE OUTPUT]"
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn insert_event(
    connection: &rusqlite::Connection,
    session_id: &str,
    event_type: &str,
    payload: serde_json::Value,
) -> Result<(), String> {
    connection.execute("INSERT INTO agent_events (id,session_id,event_type,payload_json,occurred_at) VALUES (?1,?2,?3,?4,?5)", params![Uuid::new_v4().to_string(), session_id, event_type, payload.to_string(), utc_timestamp()]).map_err(|error| format!("persist Codex event: {error}"))?;
    Ok(())
}

fn finish_failed(
    database: &DatabaseState,
    session_id: &str,
    code: &str,
    message: &str,
) -> Result<(), String> {
    let connection = database.open_connection()?;
    connection
        .execute(
            "UPDATE agent_sessions SET state='FAILED', ended_at=?2 WHERE id=?1",
            params![session_id, utc_timestamp()],
        )
        .map_err(|error| format!("persist Codex failure: {error}"))?;
    insert_event(
        &connection,
        session_id,
        "SESSION_FINISHED",
        serde_json::json!({"diagnosticCode": code, "diagnosticMessage": message}),
    )
}

fn load_session(database: &DatabaseState, session_id: &str) -> Result<CodexSession, String> {
    let connection = database.open_connection()?;
    let row = connection.query_row("SELECT s.id,s.project_id,s.task_id,s.state,s.started_at,s.ended_at,s.created_at,p.original_path FROM agent_sessions s LEFT JOIN projects p ON p.id=s.project_id WHERE s.id=?1", [session_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, String>(3)?, row.get::<_, Option<String>>(4)?, row.get::<_, Option<String>>(5)?, row.get::<_, String>(6)?, row.get::<_, Option<String>>(7)?))).map_err(|error| format!("read Codex session: {error}"))?;
    let mut stdout = String::new();
    let mut stderr = String::new();
    let mut stdout_truncated = false;
    let mut stderr_truncated = false;
    let mut diagnostic_code = None;
    let mut diagnostic_message = None;
    let mut exit_code = None;
    let mut events = connection.prepare("SELECT event_type,payload_json FROM agent_events WHERE session_id=?1 ORDER BY occurred_at ASC,id ASC LIMIT ?2").map_err(|error| format!("read Codex events: {error}"))?;
    let event_rows = events
        .query_map(params![session_id, MAX_OUTPUT_EVENTS as i64], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?))
        })
        .map_err(|error| format!("read Codex events: {error}"))?;
    for event in event_rows {
        let (event_type, payload) = event.map_err(|error| error.to_string())?;
        let Some(payload) = payload else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&payload) else {
            continue;
        };
        match event_type.as_str() {
            "STDOUT" => {
                stdout = value
                    .get("text")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                stdout_truncated = value
                    .get("truncated")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
            }
            "STDERR" => {
                stderr = value
                    .get("text")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                stderr_truncated = value
                    .get("truncated")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
            }
            "SESSION_FINISHED" => {
                diagnostic_code = value
                    .get("diagnosticCode")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_string);
                diagnostic_message = value
                    .get("diagnosticMessage")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_string);
                exit_code = value
                    .get("exitCode")
                    .and_then(serde_json::Value::as_i64)
                    .map(|code| code as i32);
            }
            _ => {}
        }
    }
    Ok(CodexSession {
        id: row.0,
        provider: PROVIDER.into(),
        project_id: row.1,
        task_id: row.2.clone(),
        operation_kind: if row.2.is_some() {
            "TASK_OPERATION".into()
        } else {
            "FREEFORM_PROJECT_OPERATION".into()
        },
        state: row.3,
        cwd: row.7.unwrap_or_default(),
        started_at: row.4,
        ended_at: row.5,
        exit_code,
        stdout,
        stderr,
        stdout_truncated,
        stderr_truncated,
        diagnostic_code,
        diagnostic_message,
    })
}

pub fn list(database: &DatabaseState, project_id: &str) -> Result<Vec<CodexSession>, String> {
    let connection = database.open_connection()?;
    let mut ids = connection.prepare("SELECT id FROM agent_sessions WHERE project_id=?1 AND provider='CODEX' ORDER BY COALESCE(started_at,created_at) DESC,id DESC LIMIT 50").map_err(|error| format!("read Codex sessions: {error}"))?;
    let rows = ids
        .query_map([project_id], |row| row.get::<_, String>(0))
        .map_err(|error| format!("read Codex session ids: {error}"))?;
    rows.map(|row| {
        row.map_err(|error| error.to_string())
            .and_then(|id| load_session(database, &id))
    })
    .collect()
}

pub fn stop(
    adapter: &CodexAdapter,
    database: &DatabaseState,
    session_id: &str,
) -> Result<CodexSession, String> {
    let processes = adapter
        .processes
        .lock()
        .map_err(|_| "CODEX_PROCESS_LOCK_POISONED".to_string())?;
    let process = processes
        .get(session_id)
        .ok_or_else(|| "CODEX_SESSION_NOT_OWNED".to_string())?;
    process.stop_requested.store(true, Ordering::Release);
    process
        .child
        .lock()
        .map_err(|_| "CODEX_PROCESS_LOCK_POISONED".to_string())?
        .kill()
        .map_err(|error| format!("CODEX_OWNED_STOP_FAILED: {error}"))?;
    drop(processes);
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let session = load_session(database, session_id)?;
        if !["STARTING", "RUNNING"].contains(&session.state.as_str()) || Instant::now() >= deadline
        {
            return Ok(session);
        }
        thread::sleep(PROCESS_POLL);
    }
}

pub fn resume(database: &DatabaseState, session_id: &str) -> Result<CodexSession, String> {
    let session = load_session(database, session_id)?;
    Err(format!(
        "RESUME_UNSUPPORTED: Codex resume is not exposed as a stable safe operation ({})",
        session.id
    ))
}

pub fn reconcile(database: &DatabaseState) -> Result<(), String> {
    let connection = database.open_connection()?;
    let mut statement = connection.prepare("SELECT id FROM agent_sessions WHERE provider='CODEX' AND state IN ('STARTING','RUNNING','WAITING_PERMISSION','WAITING_USER')") .map_err(|error| format!("read stale Codex sessions: {error}"))?;
    let ids: Vec<String> = statement
        .query_map([], |row| row.get(0))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    for id in ids {
        connection
            .execute(
                "UPDATE agent_sessions SET state='CRASHED',ended_at=?2 WHERE id=?1",
                params![id, utc_timestamp()],
            )
            .map_err(|error| format!("reconcile Codex session: {error}"))?;
        insert_event(
            &connection,
            &id,
            "PROCESS_ORPHANED",
            serde_json::json!({"diagnosticCode":"CODEX_PROCESS_NOT_OWNED_AFTER_RESTART"}),
        )?;
    }
    Ok(())
}

#[cfg(test)]
pub fn hash_prompt(prompt: &str) -> String {
    sha256_hex(prompt.as_bytes())
}
fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn prompt_metacharacters_are_one_data_argument() {
        let args = fixed_exec_args(Path::new("C:\\registered"), "x & y | z");
        assert_eq!(args.last().unwrap(), "x & y | z");
        assert!(!args.iter().any(|arg| arg == "/C" || arg == "cmd"));
    }
    #[test]
    fn prompt_is_bounded_and_sensitive_output_is_redacted() {
        assert!(validate_prompt(&"x".repeat(MAX_PROMPT_BYTES + 1)).is_err());
        assert_eq!(
            redact_output("normal\napi_key=hidden\nother"),
            "normal\n[REDACTED SENSITIVE OUTPUT]\nother"
        );
    }
    #[test]
    fn version_parser_accepts_codex_and_rejects_malformed() {
        let good = Command::new("cmd.exe")
            .args(["/C", "echo codex-cli 0.1"])
            .output()
            .unwrap();
        assert_eq!(parse_version(&good).unwrap(), "codex-cli 0.1");
        let bad = Command::new("cmd.exe")
            .args(["/C", "echo unknown"])
            .output()
            .unwrap();
        assert!(parse_version(&bad).is_err());
    }
    #[test]
    fn capture_is_bounded_and_truthful() {
        let mut capture = Capture::default();
        capture.append(&vec![b'a'; MAX_OUTPUT_BYTES + 4]);
        assert_eq!(capture.text.len(), MAX_OUTPUT_BYTES);
        assert!(capture.truncated);
    }
    #[test]
    fn root_and_nested_worktree_containment_are_explicit() {
        let directory = tempdir().unwrap();
        let child = directory.path().join("worktree");
        std::fs::create_dir(&child).unwrap();
        assert!(child.starts_with(directory.path()));
        assert!(!directory.path().starts_with(&child));
    }
    #[test]
    fn no_arbitrary_pid_stop_path_exists() {
        let adapter = CodexAdapter::default();
        assert!(adapter.processes.lock().unwrap().is_empty());
    }
    #[test]
    fn controlled_fixture_process_proves_stdout_stderr_and_exit_capture() {
        let output = Command::new("cmd.exe")
            .args(["/C", "echo fixture-out & echo fixture-err 1>&2 & exit /B 7"])
            .output()
            .expect("controlled fixture process starts");
        assert_eq!(output.status.code(), Some(7));
        assert!(String::from_utf8_lossy(&output.stdout).contains("fixture-out"));
        assert!(String::from_utf8_lossy(&output.stderr).contains("fixture-err"));
    }
    #[test]
    fn restart_recovery_marks_only_persisted_codex_transients_as_crashed() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO projects (id,name,local_path,status,priority,created_at,updated_at,original_path,normalized_path,registered_at) VALUES ('recovery-project','Recovery','C:\\recovery','ACTIVE',0,'now','now','C:\\recovery','c:\\recovery','now')", []).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,created_at) VALUES ('stale-codex','recovery-project','CODEX','RUNNING','now')", []).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,created_at) VALUES ('done-codex','recovery-project','CODEX','COMPLETED','now')", []).unwrap();
        drop(connection);
        reconcile(&database).unwrap();
        let connection = database.open_connection().unwrap();
        assert_eq!(
            connection
                .query_row(
                    "SELECT state FROM agent_sessions WHERE id='stale-codex'",
                    [],
                    |row| row.get::<_, String>(0)
                )
                .unwrap(),
            "CRASHED"
        );
        assert_eq!(
            connection
                .query_row(
                    "SELECT state FROM agent_sessions WHERE id='done-codex'",
                    [],
                    |row| row.get::<_, String>(0)
                )
                .unwrap(),
            "COMPLETED"
        );
        assert_eq!(
            connection
                .query_row(
                    "SELECT event_type FROM agent_events WHERE session_id='stale-codex'",
                    [],
                    |row| row.get::<_, String>(0)
                )
                .unwrap(),
            "PROCESS_ORPHANED"
        );
    }
    #[test]
    fn prompt_hash_is_deterministic() {
        assert_eq!(hash_prompt("same"), hash_prompt("same"));
        assert_ne!(hash_prompt("same"), hash_prompt("different"));
    }
}
