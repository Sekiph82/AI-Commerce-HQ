use crate::db::DatabaseState;
use crate::projects::{fetch_project, ProjectRecord};
use crate::time::utc_timestamp;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{Read, Write};
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
const MAX_SESSION_EVENTS: usize = MAX_OUTPUT_EVENTS * 2 + 16;
const READINESS_TIMEOUT: Duration = Duration::from_secs(5);
const STOP_GRACE: Duration = Duration::from_millis(750);
const STOP_ESCALATION_TIMEOUT: Duration = Duration::from_secs(2);
const PROCESS_POLL: Duration = Duration::from_millis(50);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgentProvider {
    Codex,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterReadiness {
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
pub struct AdapterStartRequest {
    pub project_id: String,
    pub task_id: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterSession {
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

pub trait AgentAdapter {
    fn provider(&self) -> AgentProvider;
    fn readiness(&self) -> AdapterReadiness;
    fn start(
        &self,
        database: &DatabaseState,
        request: AdapterStartRequest,
    ) -> Result<AdapterSession, String>;
    fn list(
        &self,
        database: &DatabaseState,
        project_id: &str,
    ) -> Result<Vec<AdapterSession>, String>;
    fn stop(&self, database: &DatabaseState, session_id: &str) -> Result<AdapterSession, String>;
    fn resume(&self, database: &DatabaseState, session_id: &str) -> Result<AdapterSession, String>;
    fn reconcile(&self, database: &DatabaseState) -> Result<(), String>;
}

pub type CodexReadiness = AdapterReadiness;
pub type CodexStartRequest = AdapterStartRequest;
pub type CodexSession = AdapterSession;

struct OwnedProcess {
    child: Arc<Mutex<Child>>,
    pid: u32,
    stop_requested: Arc<AtomicBool>,
    escalation_requested: Arc<AtomicBool>,
}

#[derive(Default)]
pub struct CodexAdapter {
    processes: Arc<Mutex<HashMap<String, OwnedProcess>>>,
}

#[derive(Default)]
struct Capture {
    text: String,
    retained_bytes: usize,
    event_count: usize,
    truncated: bool,
}

impl Capture {
    fn append(&mut self, bytes: &[u8]) -> Option<(String, usize, bool)> {
        if self.event_count >= MAX_OUTPUT_EVENTS {
            self.truncated = true;
            return None;
        }
        let redacted = redact_output(&String::from_utf8_lossy(bytes));
        let bytes = redacted.as_bytes();
        let remaining = MAX_OUTPUT_BYTES.saturating_sub(self.retained_bytes);
        let take = bytes.len().min(remaining);
        if take == 0 {
            self.truncated = true;
            return None;
        }
        let text = String::from_utf8_lossy(&bytes[..take]).into_owned();
        self.text.push_str(&text);
        self.retained_bytes += take;
        self.event_count += 1;
        if take < bytes.len() {
            self.truncated = true;
        }
        Some((text, self.event_count, self.truncated))
    }
}

impl AgentAdapter for CodexAdapter {
    fn provider(&self) -> AgentProvider {
        AgentProvider::Codex
    }
    fn readiness(&self) -> AdapterReadiness {
        readiness()
    }
    fn start(
        &self,
        database: &DatabaseState,
        request: AdapterStartRequest,
    ) -> Result<AdapterSession, String> {
        start(self, database, request)
    }
    fn list(
        &self,
        database: &DatabaseState,
        project_id: &str,
    ) -> Result<Vec<AdapterSession>, String> {
        list(database, project_id)
    }
    fn stop(&self, database: &DatabaseState, session_id: &str) -> Result<AdapterSession, String> {
        stop(self, database, session_id)
    }
    fn resume(&self, database: &DatabaseState, session_id: &str) -> Result<AdapterSession, String> {
        resume(database, session_id)
    }
    fn reconcile(&self, database: &DatabaseState) -> Result<(), String> {
        reconcile(database)
    }
}

pub fn readiness() -> CodexReadiness {
    let checked_at = utc_timestamp();
    let Some(executable) = discover_codex_executable() else {
        return AdapterReadiness {
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
        Ok(version) => AdapterReadiness { provider: PROVIDER.into(), available: true, version: Some(version), readiness_state: "VERSION_VERIFIED_AUTH_UNKNOWN".into(), diagnostic_code: Some("AUTH_READINESS_UNVERIFIED".into()), diagnostic_message: Some("Codex executable is available; account authentication is determined when a bounded operation starts".into()), checked_at },
        Err(ProbeError::Timeout) => unavailable_readiness("PROBE_TIMEOUT", "CODEX_VERSION_PROBE_TIMEOUT", "Codex version probe exceeded its bounded timeout", checked_at),
        Err(ProbeError::Malformed(message)) => unavailable_readiness("MALFORMED_VERSION", "CODEX_VERSION_MALFORMED", &message, checked_at),
        Err(ProbeError::Failed(message)) => unavailable_readiness("PROBE_FAILED", "CODEX_VERSION_PROBE_FAILED", &message, checked_at),
    }
}

fn unavailable_readiness(
    state: &str,
    code: &str,
    message: &str,
    checked_at: String,
) -> AdapterReadiness {
    AdapterReadiness {
        provider: PROVIDER.into(),
        available: false,
        version: None,
        readiness_state: state.into(),
        diagnostic_code: Some(code.into()),
        diagnostic_message: Some(message.into()),
        checked_at,
    }
}

pub fn start(
    adapter: &CodexAdapter,
    database: &DatabaseState,
    request: CodexStartRequest,
) -> Result<CodexSession, String> {
    provider_dispatch(PROVIDER).map_err(|error| error.to_string())?;
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
    let connection = database.open_connection()?;
    connection.execute("INSERT INTO agent_sessions (id,project_id,task_id,provider,state,started_at,created_at) VALUES (?1,?2,?3,?4,'STARTING',?5,?5)", params![session_id, request.project_id, request.task_id, PROVIDER, started_at]).map_err(|error| format!("persist Codex session: {error}"))?;
    insert_event(
        &connection,
        &session_id,
        "SESSION_STARTED",
        serde_json::json!({"operationKind":operation_kind,"promptSha256":sha256_hex(request.prompt.as_bytes()),"promptBytes":request.prompt.len()}),
    )?;
    insert_event(
        &connection,
        &session_id,
        "PROCESS_POLICY",
        serde_json::json!({"executable":"codex.exe","argumentPolicy":"FIXED_ADAPTER_ARGS","cwd":cwd.to_string_lossy(),"shell":false,"promptTransport":"STDIN_BOUNDED"}),
    )?;
    let mut command = Command::new(executable);
    command
        .args(fixed_exec_args(&cwd))
        .current_dir(&cwd)
        .stdin(Stdio::piped())
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
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "CODEX_STDIN_UNAVAILABLE".to_string())?;
    let prompt = request.prompt.clone();
    let pid = child.id();
    thread::spawn(move || {
        let _ = stdin.write_all(prompt.as_bytes());
    });
    let child = Arc::new(Mutex::new(child));
    let stop_requested = Arc::new(AtomicBool::new(false));
    let escalation_requested = Arc::new(AtomicBool::new(false));
    adapter
        .processes
        .lock()
        .map_err(|_| "CODEX_PROCESS_LOCK_POISONED".to_string())?
        .insert(
            session_id.clone(),
            OwnedProcess {
                child: child.clone(),
                pid,
                stop_requested: stop_requested.clone(),
                escalation_requested: escalation_requested.clone(),
            },
        );
    connection
        .execute(
            "UPDATE agent_sessions SET state='RUNNING' WHERE id=?1",
            [&session_id],
        )
        .map_err(|error| format!("mark Codex session running: {error}"))?;
    let stdout_capture = Arc::new(Mutex::new(Capture::default()));
    let stderr_capture = Arc::new(Mutex::new(Capture::default()));
    let stdout_capture_for_monitor = stdout_capture.clone();
    let stderr_capture_for_monitor = stderr_capture.clone();
    let database_for_stdout = database.clone();
    let database_for_stderr = database.clone();
    let session_for_stdout = session_id.clone();
    let session_for_stderr = session_id.clone();
    let stdout_thread = thread::spawn(move || {
        read_stream(
            stdout,
            stdout_capture,
            database_for_stdout,
            session_for_stdout,
            "STDOUT",
        )
    });
    let stderr_thread = thread::spawn(move || {
        read_stream(
            stderr,
            stderr_capture,
            database_for_stderr,
            session_for_stderr,
            "STDERR",
        )
    });
    let processes = adapter.processes.clone();
    let database_for_monitor = database.clone();
    let session_for_monitor = session_id.clone();
    thread::spawn(move || {
        monitor_process(
            processes,
            database_for_monitor,
            session_for_monitor,
            child,
            stop_requested,
            escalation_requested,
            stdout_capture_for_monitor,
            stderr_capture_for_monitor,
            stdout_thread,
            stderr_thread,
        )
    });
    load_session(database, &session_id)
}

fn monitor_process(
    processes: Arc<Mutex<HashMap<String, OwnedProcess>>>,
    database: DatabaseState,
    session_id: String,
    child: Arc<Mutex<Child>>,
    stop_requested: Arc<AtomicBool>,
    escalation_requested: Arc<AtomicBool>,
    stdout_capture: Arc<Mutex<Capture>>,
    stderr_capture: Arc<Mutex<Capture>>,
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
    if let Ok(connection) = database.open_connection() {
        let state = if status.is_some() && stop_requested.load(Ordering::Acquire) {
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
        let termination = if escalation_requested.load(Ordering::Acquire) {
            "ESCALATED"
        } else if stop_requested.load(Ordering::Acquire) {
            "GRACEFUL_UNSUPPORTED_OR_NOT_NEEDED"
        } else {
            "NATURAL"
        };
        let stdout_state = stdout_capture
            .lock()
            .ok()
            .map(|capture| {
                (
                    capture.truncated,
                    capture.retained_bytes,
                    capture.event_count,
                )
            })
            .unwrap_or_default();
        let stderr_state = stderr_capture
            .lock()
            .ok()
            .map(|capture| {
                (
                    capture.truncated,
                    capture.retained_bytes,
                    capture.event_count,
                )
            })
            .unwrap_or_default();
        let _ = connection.execute(
            "UPDATE agent_sessions SET state=?2,ended_at=?3 WHERE id=?1",
            params![session_id, state, utc_timestamp()],
        );
        let _ = insert_event(
            &connection,
            &session_id,
            "SESSION_FINISHED",
            serde_json::json!({"state":state,"exitCode":exit_code,"termination":termination,"stdoutTruncated":stdout_state.0,"stderrTruncated":stderr_state.0,"stdoutBytes":stdout_state.1,"stderrBytes":stderr_state.1,"stdoutEvents":stdout_state.2,"stderrEvents":stderr_state.2}),
        );
    }
    if let Ok(mut owned) = processes.lock() {
        owned.remove(&session_id);
    }
}

fn read_stream<R: Read>(
    mut reader: R,
    capture: Arc<Mutex<Capture>>,
    database: DatabaseState,
    session_id: String,
    channel: &str,
) {
    let Ok(connection) = database.open_connection() else {
        return;
    };
    let mut buffer = [0u8; 4096];
    loop {
        let Ok(size) = reader.read(&mut buffer) else {
            break;
        };
        if size == 0 {
            break;
        }
        let event = capture
            .lock()
            .ok()
            .and_then(|mut target| target.append(&buffer[..size]));
        if let Some((text, sequence, truncated)) = event {
            let _ = insert_event(
                &connection,
                &session_id,
                "STREAM_OUTPUT",
                serde_json::json!({"channel":channel,"sequence":sequence,"text":text,"truncated":truncated}),
            );
        }
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

fn fixed_exec_args(cwd: &Path) -> Vec<String> {
    vec![
        "exec".into(),
        "--json".into(),
        "--sandbox".into(),
        "workspace-write".into(),
        "--cd".into(),
        cwd.to_string_lossy().into_owned(),
        "--ephemeral".into(),
        "--skip-git-repo-check".into(),
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
            "UPDATE agent_sessions SET state='FAILED',ended_at=?2 WHERE id=?1",
            params![session_id, utc_timestamp()],
        )
        .map_err(|error| format!("persist Codex failure: {error}"))?;
    insert_event(
        &connection,
        session_id,
        "SESSION_FINISHED",
        serde_json::json!({"diagnosticCode":code,"diagnosticMessage":message}),
    )
}

fn load_session(database: &DatabaseState, session_id: &str) -> Result<CodexSession, String> {
    let connection = database.open_connection()?;
    let row = connection.query_row("SELECT s.id,s.project_id,s.task_id,s.state,s.started_at,s.ended_at,p.original_path FROM agent_sessions s LEFT JOIN projects p ON p.id=s.project_id WHERE s.id=?1", [session_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, String>(3)?, row.get::<_, Option<String>>(4)?, row.get::<_, Option<String>>(5)?, row.get::<_, Option<String>>(6)?))).map_err(|error| format!("read Codex session: {error}"))?;
    let mut stdout = String::new();
    let mut stderr = String::new();
    let mut stdout_truncated = false;
    let mut stderr_truncated = false;
    let mut diagnostic_code = None;
    let mut diagnostic_message = None;
    let mut exit_code = None;
    let mut events = connection.prepare("SELECT event_type,payload_json FROM agent_events WHERE session_id=?1 ORDER BY occurred_at ASC,id ASC LIMIT ?2").map_err(|error| format!("read Codex events: {error}"))?;
    let event_rows = events
        .query_map(params![session_id, MAX_SESSION_EVENTS as i64], |row| {
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
            "STREAM_OUTPUT" => {
                let text = value
                    .get("text")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default();
                if value.get("channel").and_then(serde_json::Value::as_str) == Some("STDERR") {
                    stderr.push_str(text);
                    stderr_truncated |= value
                        .get("truncated")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false);
                } else {
                    stdout.push_str(text);
                    stdout_truncated |= value
                        .get("truncated")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false);
                }
            }
            "STDOUT" => {
                stdout.push_str(
                    value
                        .get("text")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or_default(),
                );
                stdout_truncated |= value
                    .get("truncated")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
            }
            "STDERR" => {
                stderr.push_str(
                    value
                        .get("text")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or_default(),
                );
                stderr_truncated |= value
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
                stdout_truncated |= value
                    .get("stdoutTruncated")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
                stderr_truncated |= value
                    .get("stderrTruncated")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
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
        cwd: row.6.unwrap_or_default(),
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
    let connection = database.open_connection()?;
    insert_event(
        &connection,
        session_id,
        "STOP_REQUESTED",
        serde_json::json!({"gracefulAttempted":false,"gracefulResult":"UNSUPPORTED","diagnosticCode":"CODEX_GRACEFUL_STOP_UNSUPPORTED","gracePeriodMs":STOP_GRACE.as_millis()}),
    )?;
    drop(processes);
    let grace_deadline = Instant::now() + STOP_GRACE;
    while Instant::now() < grace_deadline {
        if process_has_exited(adapter, session_id) {
            return load_session(database, session_id);
        }
        thread::sleep(PROCESS_POLL);
    }
    let processes = adapter
        .processes
        .lock()
        .map_err(|_| "CODEX_PROCESS_LOCK_POISONED".to_string())?;
    let Some(process) = processes.get(session_id) else {
        return load_session(database, session_id);
    };
    process.escalation_requested.store(true, Ordering::Release);
    escalate_owned_process(process)?;
    let connection = database.open_connection()?;
    insert_event(
        &connection,
        session_id,
        "STOP_ESCALATED",
        serde_json::json!({"method":"OWNED_PROCESS_TREE","graceful":false,"diagnosticCode":"CODEX_GRACEFUL_STOP_UNSUPPORTED"}),
    )?;
    drop(processes);
    let deadline = Instant::now() + STOP_ESCALATION_TIMEOUT;
    while Instant::now() < deadline {
        let session = load_session(database, session_id)?;
        if !["STARTING", "RUNNING"].contains(&session.state.as_str()) {
            return Ok(session);
        }
        thread::sleep(PROCESS_POLL);
    }
    load_session(database, session_id)
}

fn process_has_exited(adapter: &CodexAdapter, session_id: &str) -> bool {
    adapter
        .processes
        .lock()
        .ok()
        .and_then(|map| {
            map.get(session_id).and_then(|process| {
                process
                    .child
                    .lock()
                    .ok()
                    .and_then(|mut child| child.try_wait().ok())
            })
        })
        .is_some_and(|status| status.is_some())
}

fn escalate_owned_process(process: &OwnedProcess) -> Result<(), String> {
    let output = Command::new("taskkill.exe")
        .args(owned_tree_escalation_args(process.pid))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("CODEX_OWNED_TREE_ESCALATION_FAILED: {error}"))?;
    if output.status.success()
        || String::from_utf8_lossy(&output.stderr)
            .to_ascii_lowercase()
            .contains("not found")
    {
        Ok(())
    } else {
        Err(format!(
            "CODEX_OWNED_TREE_ESCALATION_FAILED: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
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
    let mut statement = connection.prepare("SELECT id FROM agent_sessions WHERE provider='CODEX' AND state IN ('STARTING','RUNNING','WAITING_PERMISSION','WAITING_USER')").map_err(|error| format!("read stale Codex sessions: {error}"))?;
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
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn common_contract_dispatches_codex_and_rejects_unallowlisted_provider() {
        let adapter = CodexAdapter::default();
        assert_eq!(adapter.provider(), AgentProvider::Codex);
        assert_eq!(provider_dispatch("CODEX"), Ok(AgentProvider::Codex));
        assert_eq!(
            provider_dispatch("CLAUDE"),
            Err("ADAPTER_PROVIDER_UNSUPPORTED")
        );
    }

    #[test]
    fn prompt_metacharacters_and_flags_are_one_data_argument() {
        let args = fixed_exec_args(Path::new("C:\\registered"));
        assert!(!args
            .iter()
            .any(|arg| arg.contains("danger") || arg.contains("&") || arg.contains("|")));
        assert!(!args.iter().any(|arg| arg == "/C" || arg == "cmd"));
    }

    #[test]
    fn output_is_incremental_structured_bounded_and_redacted_before_persistence() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO projects (id,name,local_path,status,priority,created_at,updated_at,original_path,normalized_path,registered_at) VALUES ('p','Project','C:\\project','ACTIVE',0,'now','now','C:\\project','c:\\project','now')", []).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,created_at) VALUES ('s','p','CODEX','RUNNING','now')", []).unwrap();
        drop(connection);
        let capture = Arc::new(Mutex::new(Capture::default()));
        let data = b"first\napi_key=secret\n";
        read_stream(
            std::io::Cursor::new(data),
            capture,
            database.clone(),
            "s".into(),
            "STDOUT",
        );
        let connection = database.open_connection().unwrap();
        let payload: String = connection
            .query_row(
                "SELECT payload_json FROM agent_events WHERE session_id='s'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(payload.contains("first"));
        assert!(payload.contains("REDACTED"));
        assert!(!payload.contains("secret"));
    }

    #[test]
    fn stdout_stderr_and_final_exit_evidence_remain_distinct() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO projects (id,name,local_path,status,priority,created_at,updated_at,original_path,normalized_path,registered_at) VALUES ('p','Project','C:\\project','ACTIVE',0,'now','now','C:\\project','c:\\project','now')", []).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,started_at,created_at) VALUES ('s','p','CODEX','RUNNING','now','now')", []).unwrap();
        drop(connection);
        read_stream(
            std::io::Cursor::new(b"stdout-line"),
            Arc::new(Mutex::new(Capture::default())),
            database.clone(),
            "s".into(),
            "STDOUT",
        );
        read_stream(
            std::io::Cursor::new(b"stderr-line"),
            Arc::new(Mutex::new(Capture::default())),
            database.clone(),
            "s".into(),
            "STDERR",
        );
        let connection = database.open_connection().unwrap();
        insert_event(
            &connection,
            "s",
            "SESSION_FINISHED",
            serde_json::json!({"state":"FAILED","exitCode":7,"termination":"NATURAL"}),
        )
        .unwrap();
        let session = load_session(&database, "s").unwrap();
        assert_eq!(session.stdout, "stdout-line");
        assert_eq!(session.stderr, "stderr-line");
        assert_eq!(session.exit_code, Some(7));
    }

    #[test]
    fn capture_enforces_byte_and_event_caps_truthfully() {
        let mut capture = Capture::default();
        for _ in 0..(MAX_OUTPUT_EVENTS + 4) {
            let _ = capture.append(b"x");
        }
        assert_eq!(capture.event_count, MAX_OUTPUT_EVENTS);
        assert!(capture.truncated);
        let mut bytes = Capture::default();
        let _ = bytes.append(&vec![b'a'; MAX_OUTPUT_BYTES + 1]);
        assert_eq!(bytes.retained_bytes, MAX_OUTPUT_BYTES);
        assert!(bytes.truncated);
    }

    #[test]
    fn controlled_long_running_fixture_persists_first_output_before_exit() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO projects (id,name,local_path,status,priority,created_at,updated_at,original_path,normalized_path,registered_at) VALUES ('p','Project','C:\\project','ACTIVE',0,'now','now','C:\\project','c:\\project','now')", []).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,created_at) VALUES ('s','p','CODEX','RUNNING','now')", []).unwrap();
        drop(connection);
        let mut child = Command::new("cmd.exe")
            .args([
                "/C",
                "echo first-output & ping 127.0.0.1 -n 4 > nul & echo second-output",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let stdout = child.stdout.take().unwrap();
        let handle = thread::spawn({
            let db = database.clone();
            move || {
                read_stream(
                    stdout,
                    Arc::new(Mutex::new(Capture::default())),
                    db,
                    "s".into(),
                    "STDOUT",
                )
            }
        });
        let deadline = Instant::now() + Duration::from_secs(2);
        let mut observed = false;
        while Instant::now() < deadline {
            let connection = database.open_connection().unwrap();
            let count: i64 = connection.query_row("SELECT COUNT(*) FROM agent_events WHERE session_id='s' AND event_type='STREAM_OUTPUT'", [], |row| row.get(0)).unwrap();
            if count > 0 {
                observed = true;
                break;
            }
            thread::sleep(Duration::from_millis(25));
        }
        assert!(
            observed,
            "first stream event must be readable before fixture exit"
        );
        let _ = child.kill();
        let _ = child.wait();
        handle.join().unwrap();
    }

    #[test]
    fn stop_transport_is_owned_and_graceful_limitation_is_explicit() {
        let adapter = CodexAdapter::default();
        assert!(adapter.processes.lock().unwrap().is_empty());
        let args = owned_tree_escalation_args(42);
        assert_eq!(args, vec!["/PID", "42", "/T", "/F"]);
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        assert_eq!(
            stop(&adapter, &database, "unowned").unwrap_err(),
            "CODEX_SESSION_NOT_OWNED"
        );
    }

    #[test]
    fn owned_stop_escalates_after_bounded_unsupported_graceful_attempt() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO projects (id,name,local_path,status,priority,created_at,updated_at,original_path,normalized_path,registered_at) VALUES ('p','Project','C:\\project','ACTIVE',0,'now','now','C:\\project','c:\\project','now')", []).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,started_at,created_at) VALUES ('s','p','CODEX','RUNNING','now','now')", []).unwrap();
        drop(connection);
        let child = Command::new("cmd.exe")
            .args(["/C", "ping 127.0.0.1 -n 30 > nul"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let pid = child.id();
        let child = Arc::new(Mutex::new(child));
        let stop_requested = Arc::new(AtomicBool::new(false));
        let escalation_requested = Arc::new(AtomicBool::new(false));
        let adapter = CodexAdapter::default();
        adapter.processes.lock().unwrap().insert(
            "s".into(),
            OwnedProcess {
                child: child.clone(),
                pid,
                stop_requested: stop_requested.clone(),
                escalation_requested: escalation_requested.clone(),
            },
        );
        let monitor = thread::spawn({
            let processes = adapter.processes.clone();
            let db = database.clone();
            move || {
                monitor_process(
                    processes,
                    db,
                    "s".into(),
                    child,
                    stop_requested,
                    escalation_requested,
                    Arc::new(Mutex::new(Capture::default())),
                    Arc::new(Mutex::new(Capture::default())),
                    thread::spawn(|| {}),
                    thread::spawn(|| {}),
                )
            }
        });
        let session = stop(&adapter, &database, "s").unwrap();
        monitor.join().unwrap();
        assert_eq!(session.state, "STOPPED");
        assert!(session.exit_code.is_some());
        let connection = database.open_connection().unwrap();
        let events: Vec<String> = connection
            .prepare("SELECT event_type FROM agent_events WHERE session_id='s' ORDER BY occurred_at ASC,id ASC")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(|row| row.unwrap())
            .collect();
        assert!(events.iter().any(|event| event == "STOP_REQUESTED"));
        assert!(events.iter().any(|event| event == "STOP_ESCALATED"));
        assert!(events.iter().any(|event| event == "SESSION_FINISHED"));
        assert!(adapter.processes.lock().unwrap().is_empty());
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
}

fn provider_dispatch(provider: &str) -> Result<AgentProvider, &'static str> {
    match provider {
        "CODEX" => Ok(AgentProvider::Codex),
        _ => Err("ADAPTER_PROVIDER_UNSUPPORTED"),
    }
}

fn owned_tree_escalation_args(pid: u32) -> Vec<String> {
    vec!["/PID".into(), pid.to_string(), "/T".into(), "/F".into()]
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
