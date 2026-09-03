use crate::codex_adapter::{AgentAdapter, CodexAdapter, CodexSession, CodexStartRequest};
use crate::final_response::{FinalResponseCapture, FinalResponseState, ProviderKind};
use crate::process_policy::background_command;
use crate::projects::{fetch_project, ProjectRecord};
use crate::stream_sanitizer::StreamRedactor;
use crate::time::utc_timestamp;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use uuid::Uuid;

const MAX_PROMPT_BYTES: usize = 64 * 1024;
const MAX_OUTPUT_BYTES: usize = 64 * 1024;
const MAX_OUTPUT_EVENTS: usize = 128;
const PROBE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionProvider {
    Codex,
    Claude,
}

impl SessionProvider {
    fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_ascii_uppercase().as_str() {
            "CODEX" => Ok(Self::Codex),
            "CLAUDE" => Ok(Self::Claude),
            _ => Err("AGENT_PROVIDER_UNSUPPORTED".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderReadiness {
    pub provider: String,
    pub available: bool,
    pub version: Option<String>,
    pub readiness_state: String,
    pub diagnostic_code: Option<String>,
    pub diagnostic_message: Option<String>,
    pub capabilities: Vec<String>,
    pub supports_pty: bool,
    pub supports_resume: bool,
    pub checked_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentStartRequest {
    pub provider: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRetryRequest {
    pub source_session_id: String,
    pub provider: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEvent {
    pub sequence: usize,
    pub id: String,
    pub event_type: String,
    pub payload: Value,
    pub occurred_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSession {
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
    pub final_response: Option<String>,
    pub final_response_truncated: bool,
    pub final_response_state: String,
    pub final_response_role: Option<String>,
    pub diagnostic_code: Option<String>,
    pub diagnostic_message: Option<String>,
    pub prompt_reference: Option<String>,
    pub prompt_body: Option<String>,
    pub provider_version: Option<String>,
    pub elapsed_ms: Option<u64>,
    pub supports_resume: bool,
    pub supports_pty: bool,
    pub events: Vec<SessionEvent>,
}

#[derive(Default)]
pub struct AgentSessionCenter {
    claude_processes: Arc<Mutex<std::collections::HashMap<String, OwnedClaudeProcess>>>,
}

struct OwnedClaudeProcess {
    _child: Arc<Mutex<Child>>,
    pid: u32,
    stop_requested: Arc<AtomicBool>,
}

#[derive(Default)]
struct Capture {
    text: String,
    bytes: usize,
    events: usize,
    truncated: bool,
}

impl Capture {
    fn append(&mut self, text: &str) -> Option<String> {
        if self.events >= MAX_OUTPUT_EVENTS {
            self.truncated = true;
            return None;
        }
        let remaining = MAX_OUTPUT_BYTES.saturating_sub(self.bytes);
        let take = text.as_bytes().len().min(remaining);
        if take == 0 {
            self.truncated = true;
            return None;
        }
        let retained = String::from_utf8_lossy(&text.as_bytes()[..take]).into_owned();
        self.text.push_str(&retained);
        self.bytes += take;
        self.events += 1;
        if take < text.len() {
            self.truncated = true;
        }
        Some(retained)
    }
}

fn validate_prompt(prompt: &str) -> Result<(), String> {
    if prompt.trim().is_empty() {
        return Err("AGENT_PROMPT_EMPTY".into());
    }
    if prompt.len() > MAX_PROMPT_BYTES {
        return Err("AGENT_PROMPT_TOO_LARGE".into());
    }
    Ok(())
}

fn validate_operation_project(project: &ProjectRecord) -> Result<PathBuf, String> {
    if project.status != "ACTIVE" {
        return Err(match project.status.as_str() {
            "MISSING" => "AGENT_PROJECT_MISSING",
            "ARCHIVED" => "AGENT_PROJECT_ARCHIVED",
            _ => "AGENT_PROJECT_NOT_ACTIVE",
        }
        .into());
    }
    let path = PathBuf::from(&project.normalized_path);
    if !path.is_dir() {
        return Err("AGENT_PROJECT_PATH_UNAVAILABLE".into());
    }
    path.canonicalize()
        .map_err(|error| format!("AGENT_PROJECT_PATH_INVALID: {error}"))
}

fn validate_task(
    database: &crate::db::DatabaseState,
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
        .map_err(|error| format!("AGENT_TASK_VALIDATION_FAILED: {error}"))?
        == 1;
    belongs
        .then_some(())
        .ok_or_else(|| "AGENT_TASK_PROJECT_MISMATCH_OR_MISSING".into())
}

pub fn readiness() -> Vec<ProviderReadiness> {
    vec![codex_readiness(), claude_readiness()]
}

fn codex_readiness() -> ProviderReadiness {
    let ready = crate::codex_adapter::readiness();
    ProviderReadiness {
        provider: "CODEX".into(),
        available: ready.available,
        version: ready.version,
        readiness_state: ready.readiness_state,
        diagnostic_code: ready.diagnostic_code,
        diagnostic_message: ready.diagnostic_message,
        capabilities: vec![
            "START".into(),
            "LIST".into(),
            "STOP".into(),
            "BOUNDED_OUTPUT".into(),
        ],
        supports_pty: false,
        supports_resume: false,
        checked_at: ready.checked_at,
    }
}

fn claude_readiness() -> ProviderReadiness {
    let checked_at = utc_timestamp();
    let resolution = resolve_claude_executable();
    let Some(executable) = resolution.selected else {
        return ProviderReadiness {
            provider: "CLAUDE".into(),
            available: false,
            version: None,
            readiness_state: "UNAVAILABLE".into(),
            diagnostic_code: Some("CLAUDE_EXECUTABLE_NOT_FOUND".into()),
            diagnostic_message: Some("No direct native claude.exe was found on PATH or the bounded native install fallbacks".into()),
            capabilities: vec!["LIST".into()],
            supports_pty: false,
            supports_resume: false,
            checked_at,
        };
    };
    match probe_claude_version(&executable) {
        Ok(version) => ProviderReadiness {
            provider: "CLAUDE".into(),
            available: true,
            version: Some(version),
            readiness_state: "VERSION_VERIFIED_AUTH_UNKNOWN".into(),
            diagnostic_code: Some("AUTH_UNKNOWN".into()),
            diagnostic_message: Some(
                "Claude authentication is determined only by a bounded operation".into(),
            ),
            capabilities: vec![
                "START".into(),
                "LIST".into(),
                "STOP".into(),
                "BOUNDED_STREAM_JSON".into(),
                "PLAN_PERMISSION_MODE".into(),
            ],
            supports_pty: false,
            supports_resume: false,
            checked_at,
        },
        Err(code) => ProviderReadiness {
            provider: "CLAUDE".into(),
            available: false,
            version: None,
            readiness_state: "UNAVAILABLE".into(),
            diagnostic_code: Some(code),
            diagnostic_message: Some(
                "The bounded Claude version probe did not return valid version evidence".into(),
            ),
            capabilities: vec!["LIST".into()],
            supports_pty: false,
            supports_resume: false,
            checked_at,
        },
    }
}

fn resolve_claude_executable() -> Resolution {
    let mut entries = std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .unwrap_or_default();
    #[cfg(windows)]
    {
        if let Some(root) = std::env::var_os("USERPROFILE") {
            entries.push(PathBuf::from(root).join(r".local\bin"));
        }
        if let Some(root) = std::env::var_os("APPDATA") {
            entries.push(PathBuf::from(root).join("npm"));
        }
        if let Some(root) = std::env::var_os("LOCALAPPDATA") {
            entries.push(PathBuf::from(root).join(r"Microsoft\WinGet\Links"));
        }
    }
    resolve_claude_from_entries(entries)
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Resolution {
    selected: Option<PathBuf>,
    skipped_candidates: usize,
}

fn resolve_claude_from_entries(entries: Vec<PathBuf>) -> Resolution {
    let mut result = Resolution::default();
    let mut seen = HashSet::new();
    for entry in entries {
        for name in claude_names() {
            let candidate = entry.join(name);
            if !candidate.is_file() || !seen.insert(candidate.clone()) {
                continue;
            }
            if !is_direct_claude_candidate(name) || !is_native_claude_candidate(&candidate) {
                result.skipped_candidates += 1;
                continue;
            }
            result.selected = Some(candidate);
            return result;
        }
    }
    result
}

#[cfg(windows)]
fn claude_names() -> &'static [&'static str] {
    &["claude.exe"]
}
#[cfg(not(windows))]
fn claude_names() -> &'static [&'static str] {
    &["claude", "claude.exe"]
}
#[cfg(windows)]
fn is_direct_claude_candidate(name: &str) -> bool {
    name.eq_ignore_ascii_case("claude.exe")
}
#[cfg(not(windows))]
fn is_direct_claude_candidate(_name: &str) -> bool {
    true
}

fn is_native_claude_candidate(path: &Path) -> bool {
    #[cfg(windows)]
    {
        let Ok(bytes) = std::fs::read(path) else {
            return false;
        };
        return bytes.len() >= 64 && &bytes[..2] == b"MZ";
    }
    #[cfg(not(windows))]
    {
        path.is_file()
    }
}

fn probe_claude_version(path: &Path) -> Result<String, String> {
    let mut child = background_command(path)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| "CLAUDE_VERSION_PROBE_FAILED".to_string())?;
    let deadline = Instant::now() + PROBE_TIMEOUT;
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|_| "CLAUDE_VERSION_PROBE_FAILED".to_string())?
        {
            if !status.success() {
                return Err("CLAUDE_VERSION_PROBE_FAILED".into());
            }
            let mut output = String::new();
            if let Some(mut stdout) = child.stdout.take() {
                let _ = stdout.read_to_string(&mut output);
            }
            let first = output.lines().next().unwrap_or_default().trim();
            if first.len() <= 256 && first.to_ascii_lowercase().contains("claude") {
                return Ok(first.into());
            }
            return Err("CLAUDE_VERSION_MALFORMED".into());
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err("CLAUDE_VERSION_PROBE_TIMEOUT".into());
        }
        thread::sleep(Duration::from_millis(25));
    }
}

pub fn start(
    center: &AgentSessionCenter,
    codex: &CodexAdapter,
    database: &crate::db::DatabaseState,
    request: AgentStartRequest,
) -> Result<AgentSession, String> {
    let provider = SessionProvider::parse(&request.provider)?;
    validate_prompt(&request.prompt)?;
    let project = fetch_project(database, &request.project_id)?;
    let cwd = validate_operation_project(&project)?;
    validate_task(database, &request.project_id, request.task_id.as_deref())?;
    match provider {
        SessionProvider::Codex => codex
            .start(
                database,
                CodexStartRequest {
                    project_id: request.project_id,
                    task_id: request.task_id,
                    prompt: request.prompt,
                },
            )
            .map(AgentSession::from_codex),
        SessionProvider::Claude => start_claude(center, database, request, cwd),
    }
}

fn start_claude(
    center: &AgentSessionCenter,
    database: &crate::db::DatabaseState,
    request: AgentStartRequest,
    cwd: PathBuf,
) -> Result<AgentSession, String> {
    let executable = resolve_claude_executable()
        .selected
        .ok_or("CLAUDE_EXECUTABLE_NOT_FOUND")?;
    let version = probe_claude_version(&executable).ok();
    let session_id = Uuid::new_v4().to_string();
    let started_at = utc_timestamp();
    let operation_kind = if request.task_id.is_some() {
        "TASK_OPERATION"
    } else {
        "FREEFORM_PROJECT_OPERATION"
    };
    let connection = database.open_connection()?;
    connection.execute("INSERT INTO agent_sessions (id,project_id,task_id,provider,state,started_at,created_at,prompt_body) VALUES (?1,?2,?3,'CLAUDE','STARTING',?4,?4,?5)", params![session_id, request.project_id, request.task_id, started_at, request.prompt]).map_err(|error| format!("persist Claude session: {error}"))?;
    insert_event(
        &connection,
        &session_id,
        "SESSION_STARTED",
        json!({"operationKind":operation_kind,"promptSha256":sha256_hex(request.prompt.as_bytes()),"promptBytes":request.prompt.len(),"providerVersion":version}),
    )?;
    insert_event(
        &connection,
        &session_id,
        "PROCESS_POLICY",
        json!({"executable":"claude.exe","argumentPolicy":"FIXED_CLAUDE_PRINT_ARGS","args":["--print","--output-format","stream-json","--verbose","--no-session-persistence","--permission-mode","plan","--restricted"],"cwd":cwd.to_string_lossy(),"shell":false,"promptTransport":"STDIN_BOUNDED"}),
    )?;
    let mut command = background_command(&executable);
    command
        .args(fixed_claude_args())
        .current_dir(&cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().map_err(|error| {
        let _ = finish_failed(
            database,
            &session_id,
            "CLAUDE_PROCESS_SPAWN_FAILED",
            &error.to_string(),
        );
        format!("CLAUDE_PROCESS_SPAWN_FAILED: {error}")
    })?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(request.prompt.as_bytes())
            .map_err(|error| format!("CLAUDE_PROMPT_WRITE_FAILED: {error}"))?;
    }
    let stdout = child.stdout.take().ok_or("CLAUDE_STDOUT_UNAVAILABLE")?;
    let stderr = child.stderr.take().ok_or("CLAUDE_STDERR_UNAVAILABLE")?;
    let pid = child.id();
    let child = Arc::new(Mutex::new(child));
    let stop_requested = Arc::new(AtomicBool::new(false));
    center
        .claude_processes
        .lock()
        .map_err(|_| "CLAUDE_PROCESS_LOCK_POISONED")?
        .insert(
            session_id.clone(),
            OwnedClaudeProcess {
                _child: child.clone(),
                pid,
                stop_requested: stop_requested.clone(),
            },
        );
    let connection = database.open_connection()?;
    connection
        .execute(
            "UPDATE agent_sessions SET state='RUNNING' WHERE id=?1",
            [&session_id],
        )
        .map_err(|error| format!("mark Claude session running: {error}"))?;
    let database = database.clone();
    let database_for_thread = database.clone();
    let processes = center.claude_processes.clone();
    let session_for_thread = session_id.clone();
    thread::spawn(move || {
        run_claude_session(
            database_for_thread,
            processes,
            session_for_thread,
            child,
            stop_requested,
            stdout,
            stderr,
        )
    });
    load_session(&database, &session_id)
}

fn fixed_claude_args() -> Vec<String> {
    vec![
        "--print".into(),
        "--output-format".into(),
        "stream-json".into(),
        "--verbose".into(),
        "--no-session-persistence".into(),
        "--permission-mode".into(),
        "plan".into(),
        "--restricted".into(),
    ]
}

fn validate_pty_dimensions(rows: u16, columns: u16) -> Result<(), String> {
    if rows == 0 || columns == 0 || rows > 500 || columns > 500 {
        return Err("AGENT_PTY_RESIZE_INVALID".into());
    }
    Ok(())
}

fn run_claude_session(
    database: crate::db::DatabaseState,
    processes: Arc<Mutex<std::collections::HashMap<String, OwnedClaudeProcess>>>,
    session_id: String,
    child: Arc<Mutex<Child>>,
    stop_requested: Arc<AtomicBool>,
    stdout: impl Read + Send + 'static,
    stderr: impl Read + Send + 'static,
) {
    let (sender, receiver) = sync_channel::<(bool, Vec<u8>)>(64);
    let out_sender = sender.clone();
    let out_thread = thread::spawn(move || read_stream(stdout, true, out_sender));
    let err_thread = thread::spawn(move || read_stream(stderr, false, sender));
    let wait_thread =
        thread::spawn(move || child.lock().ok().and_then(|mut value| value.wait().ok()));
    let mut stdout_capture = Capture::default();
    let mut stderr_capture = Capture::default();
    let mut stdout_redactor = StreamRedactor::default();
    let mut stderr_redactor = StreamRedactor::default();
    let mut final_response = FinalResponseCapture::default();
    let Ok(connection) = database.open_connection() else {
        let _ = out_thread.join();
        let _ = err_thread.join();
        let _ = wait_thread.join();
        if let Ok(mut map) = processes.lock() {
            map.remove(&session_id);
        }
        return;
    };
    for (is_stdout, bytes) in receiver {
        let redactor = if is_stdout {
            &mut stdout_redactor
        } else {
            &mut stderr_redactor
        };
        let capture = if is_stdout {
            &mut stdout_capture
        } else {
            &mut stderr_capture
        };
        for text in redactor.push(&bytes) {
            if is_stdout {
                final_response.observe(ProviderKind::Claude, &text);
            }
            if let Some(retained) = capture.append(&text) {
                let event_type = if is_stdout {
                    "STREAM_STDOUT"
                } else {
                    "STREAM_STDERR"
                };
                let _ = insert_event(
                    &connection,
                    &session_id,
                    event_type,
                    json!({"text":retained,"channel":if is_stdout { "stdout" } else { "stderr" }}),
                );
            }
        }
    }
    let _ = out_thread.join();
    let _ = err_thread.join();
    for (is_stdout, redactor, capture) in [
        (true, &mut stdout_redactor, &mut stdout_capture),
        (false, &mut stderr_redactor, &mut stderr_capture),
    ] {
        for text in redactor.finish() {
            if is_stdout {
                final_response.observe(ProviderKind::Claude, &text);
            }
            if let Some(retained) = capture.append(&text) {
                let event_type = if is_stdout {
                    "STREAM_STDOUT"
                } else {
                    "STREAM_STDERR"
                };
                let _ = insert_event(
                    &connection,
                    &session_id,
                    event_type,
                    json!({"text":retained,"channel":if is_stdout { "stdout" } else { "stderr" }}),
                );
            }
        }
    }
    let status = wait_thread.join().ok().flatten();
    let state = if stop_requested.load(Ordering::Acquire) {
        "STOPPED"
    } else if status.as_ref().is_some_and(|value| value.success()) {
        "COMPLETED"
    } else {
        "FAILED"
    };
    let (code, message) = if state == "FAILED" {
        (
            Some("CLAUDE_PROCESS_FAILED"),
            Some("Claude exited before producing a successful terminal result"),
        )
    } else if state == "COMPLETED" && final_response.state() == FinalResponseState::Unavailable {
        (
            Some("CLAUDE_FINAL_RESPONSE_UNAVAILABLE"),
            Some(
                "Claude exited successfully, but no dedicated final assistant response was captured",
            ),
        )
    } else {
        (None, None)
    };
    let _ = finalize_claude(
        &database,
        &session_id,
        state,
        status.and_then(|value| value.code()),
        &stdout_capture,
        &stderr_capture,
        &final_response,
        code,
        message,
    );
    if let Ok(mut map) = processes.lock() {
        map.remove(&session_id);
    }
}

fn read_stream(
    mut reader: impl Read,
    stdout: bool,
    sender: std::sync::mpsc::SyncSender<(bool, Vec<u8>)>,
) {
    let mut buffer = [0_u8; 4096];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => {
                if sender.send((stdout, buffer[..size].to_vec())).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

fn finalize_claude(
    database: &crate::db::DatabaseState,
    session_id: &str,
    state: &str,
    exit_code: Option<i32>,
    stdout: &Capture,
    stderr: &Capture,
    final_response: &FinalResponseCapture,
    diagnostic_code: Option<&str>,
    diagnostic_message: Option<&str>,
) -> Result<(), String> {
    let connection = database.open_connection()?;
    let final_text = final_response.text();
    let final_state = final_response.state().as_str();
    let mut effective_code = diagnostic_code;
    let mut effective_message = diagnostic_message;
    if let Err(error) = connection.execute(
        "UPDATE agent_sessions SET state=?2,ended_at=?3,final_response=?4,final_response_truncated=?5,final_response_state=?6,final_response_role=CASE WHEN ?4 IS NULL THEN NULL ELSE 'assistant' END WHERE id=?1",
        params![session_id, state, utc_timestamp(), final_text, final_response.truncated(), final_state],
    ) {
        effective_code = Some("CLAUDE_FINAL_RESPONSE_PERSISTENCE_DEGRADED");
        effective_message = Some("Claude completed, but the dedicated final assistant response could not be persisted");
        log::warn!("finalize Claude session response persistence failed: {error}");
    }
    insert_event(
        &connection,
        session_id,
        "SESSION_FINISHED",
        json!({"state":state,"exitCode":exit_code,"stdoutTruncated":stdout.truncated,"stderrTruncated":stderr.truncated,"finalResponseState":final_state,"finalResponseTruncated":final_response.truncated(),"diagnosticCode":effective_code,"diagnosticMessage":effective_message}),
    )?;
    Ok(())
}

fn finish_failed(
    database: &crate::db::DatabaseState,
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
        .map_err(|error| format!("persist agent failure: {error}"))?;
    insert_event(
        &connection,
        session_id,
        "SESSION_FINISHED",
        json!({"diagnosticCode":code,"diagnosticMessage":message}),
    )
}

pub fn list(
    center: &AgentSessionCenter,
    codex: &CodexAdapter,
    database: &crate::db::DatabaseState,
    project_id: &str,
) -> Result<Vec<AgentSession>, String> {
    let _ = fetch_project(database, project_id)?;
    let mut sessions = Vec::new();
    for item in codex.list(database, project_id)? {
        sessions.push(
            load_session(database, &item.id).unwrap_or_else(|_| AgentSession::from_codex(item)),
        );
    }
    let connection = database.open_connection()?;
    let mut statement = connection.prepare("SELECT id FROM agent_sessions WHERE project_id=?1 AND provider='CLAUDE' ORDER BY COALESCE(started_at,created_at) DESC,id DESC LIMIT 50").map_err(|error| format!("read Claude sessions: {error}"))?;
    let ids = statement
        .query_map([project_id], |row| row.get::<_, String>(0))
        .map_err(|error| format!("read Claude session ids: {error}"))?;
    for id in ids {
        sessions.push(load_session(
            database,
            &id.map_err(|error| error.to_string())?,
        )?);
    }
    sessions.sort_by(|left, right| right.started_at.cmp(&left.started_at));
    let _ = center;
    Ok(sessions)
}

fn load_session_for_project(
    database: &crate::db::DatabaseState,
    project_id: &str,
    session_id: &str,
) -> Result<AgentSession, String> {
    let session = load_session(database, session_id)?;
    if session.project_id != project_id {
        return Err("AGENT_SESSION_PROJECT_MISMATCH".into());
    }
    Ok(session)
}

pub fn session_events(
    database: &crate::db::DatabaseState,
    project_id: &str,
    session_id: &str,
) -> Result<Vec<SessionEvent>, String> {
    let session = load_session_for_project(database, project_id, session_id)?;
    let connection = database.open_connection()?;
    let _ = session;
    load_events(&connection, session_id)
}

pub fn stop(
    center: &AgentSessionCenter,
    codex: &CodexAdapter,
    database: &crate::db::DatabaseState,
    project_id: &str,
    session_id: &str,
) -> Result<AgentSession, String> {
    let session = load_session_for_project(database, project_id, session_id)?;
    match SessionProvider::parse(&session.provider)? {
        SessionProvider::Codex => codex
            .stop(database, session_id)
            .map(AgentSession::from_codex),
        SessionProvider::Claude => {
            let map = center
                .claude_processes
                .lock()
                .map_err(|_| "CLAUDE_PROCESS_LOCK_POISONED")?;
            let process = map.get(session_id).ok_or("AGENT_SESSION_NOT_OWNED")?;
            process.stop_requested.store(true, Ordering::Release);
            let connection = database.open_connection()?;
            insert_event(
                &connection,
                session_id,
                "STOP_REQUESTED",
                json!({"gracefulAttempted":false,"gracefulResult":"UNSUPPORTED","ownedPid":process.pid}),
            )?;
            let _ = background_command("taskkill.exe")
                .args(["/PID", &process.pid.to_string(), "/T", "/F"])
                .status();
            Ok(load_session(database, session_id)?)
        }
    }
}

pub fn retry(
    center: &AgentSessionCenter,
    codex: &CodexAdapter,
    database: &crate::db::DatabaseState,
    request: AgentRetryRequest,
) -> Result<AgentSession, String> {
    let source = load_session(database, &request.source_session_id)?;
    if source.project_id != request.project_id
        || source.provider != request.provider.trim().to_ascii_uppercase()
        || source.task_id != request.task_id
    {
        return Err("AGENT_RETRY_PROVENANCE_MISMATCH".into());
    }
    let session = start(
        center,
        codex,
        database,
        AgentStartRequest {
            provider: request.provider,
            project_id: request.project_id,
            task_id: request.task_id,
            prompt: request.prompt,
        },
    )?;
    let connection = database.open_connection()?;
    insert_event(
        &connection,
        &session.id,
        "RETRY_PROVENANCE",
        json!({"sourceSessionId":source.id,"sourceProvider":source.provider,"sourceProjectId":source.project_id,"sourceTaskId":source.task_id,"retryTimestamp":utc_timestamp()}),
    )?;
    load_session(database, &session.id)
}

pub fn resume(
    database: &crate::db::DatabaseState,
    project_id: &str,
    session_id: &str,
) -> Result<AgentSession, String> {
    let session = load_session_for_project(database, project_id, session_id)?;
    Err(format!(
        "RESUME_UNSUPPORTED: {} has no verified provider resume capability",
        session.provider
    ))
}

pub fn resize(
    database: &crate::db::DatabaseState,
    project_id: &str,
    session_id: &str,
    rows: u16,
    columns: u16,
) -> Result<(), String> {
    validate_pty_dimensions(rows, columns)?;
    let session = load_session_for_project(database, project_id, session_id)?;
    if session.supports_pty {
        Ok(())
    } else {
        Err("AGENT_PTY_RESIZE_UNSUPPORTED_NON_PTY".into())
    }
}

pub fn reconcile(database: &crate::db::DatabaseState) -> Result<(), String> {
    let connection = database.open_connection()?;
    let mut statement = connection.prepare("SELECT id FROM agent_sessions WHERE provider='CLAUDE' AND state IN ('STARTING','RUNNING','WAITING_PERMISSION','STOPPING')").map_err(|error| format!("read stale Claude sessions: {error}"))?;
    let ids = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    for id in ids {
        connection
            .execute(
                "UPDATE agent_sessions SET state='CRASHED',ended_at=?2 WHERE id=?1",
                params![id, utc_timestamp()],
            )
            .map_err(|error| format!("reconcile Claude session: {error}"))?;
        insert_event(
            &connection,
            &id,
            "PROCESS_ORPHANED",
            json!({"diagnosticCode":"CLAUDE_PROCESS_NOT_OWNED_AFTER_RESTART"}),
        )?;
    }
    Ok(())
}

fn load_session(
    database: &crate::db::DatabaseState,
    session_id: &str,
) -> Result<AgentSession, String> {
    let connection = database.open_connection()?;
    let row = connection.query_row("SELECT id,project_id,task_id,provider,state,started_at,ended_at,created_at,prompt_body,final_response,final_response_truncated,final_response_state,final_response_role FROM agent_sessions WHERE id=?1", [session_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, String>(3)?, row.get::<_, String>(4)?, row.get::<_, Option<String>>(5)?, row.get::<_, Option<String>>(6)?, row.get::<_, String>(7)?, row.get::<_, Option<String>>(8)?, row.get::<_, Option<String>>(9)?, row.get::<_, bool>(10)?, row.get::<_, String>(11)?, row.get::<_, Option<String>>(12)?))).optional().map_err(|error| format!("read agent session: {error}"))?.ok_or("AGENT_SESSION_NOT_FOUND")?;
    let events = load_events(&connection, session_id)?;
    let mut stdout = String::new();
    let mut stderr = String::new();
    let mut exit_code = None;
    let mut stdout_truncated = false;
    let mut stderr_truncated = false;
    let mut diagnostic_code = None;
    let mut diagnostic_message = None;
    let mut prompt_reference = None;
    let mut provider_version = None;
    for event in &events {
        match event.event_type.as_str() {
            "SESSION_STARTED" => {
                prompt_reference = event
                    .payload
                    .get("promptSha256")
                    .and_then(Value::as_str)
                    .map(str::to_string);
                provider_version = event
                    .payload
                    .get("providerVersion")
                    .and_then(Value::as_str)
                    .map(str::to_string);
            }
            "PROCESS_POLICY" => {
                provider_version = event
                    .payload
                    .get("providerVersion")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .or(provider_version);
            }
            "STREAM_STDOUT" => stdout.push_str(
                event
                    .payload
                    .get("text")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
            ),
            "STREAM_STDERR" => stderr.push_str(
                event
                    .payload
                    .get("text")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
            ),
            "SESSION_FINISHED" => {
                exit_code = event
                    .payload
                    .get("exitCode")
                    .and_then(Value::as_i64)
                    .map(|value| value as i32);
                stdout_truncated |= event
                    .payload
                    .get("stdoutTruncated")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                stderr_truncated |= event
                    .payload
                    .get("stderrTruncated")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                diagnostic_code = event
                    .payload
                    .get("diagnosticCode")
                    .and_then(Value::as_str)
                    .map(str::to_string);
                diagnostic_message = event
                    .payload
                    .get("diagnosticMessage")
                    .and_then(Value::as_str)
                    .map(str::to_string);
            }
            _ => {}
        }
    }
    let elapsed_ms = row
        .5
        .as_deref()
        .and_then(|value| parse_timestamp_ms(value))
        .map(|started| {
            row.6
                .as_deref()
                .and_then(parse_timestamp_ms)
                .unwrap_or_else(current_timestamp_ms)
                .saturating_sub(started)
                .max(0) as u64
        });
    Ok(AgentSession {
        id: row.0,
        provider: row.3.clone(),
        project_id: row.1,
        task_id: row.2.clone(),
        operation_kind: if row.2.is_some() {
            "TASK_OPERATION".into()
        } else {
            "FREEFORM_PROJECT_OPERATION".into()
        },
        state: row.4,
        cwd: project_cwd(database, session_id)?,
        started_at: row.5,
        ended_at: row.6,
        exit_code,
        stdout,
        stderr,
        stdout_truncated,
        stderr_truncated,
        final_response: row.9,
        final_response_truncated: row.10,
        final_response_state: row.11,
        final_response_role: row.12,
        diagnostic_code,
        diagnostic_message,
        prompt_reference,
        prompt_body: row.8,
        provider_version,
        elapsed_ms,
        supports_resume: false,
        supports_pty: false,
        events,
    })
}

fn project_cwd(database: &crate::db::DatabaseState, session_id: &str) -> Result<String, String> {
    let connection = database.open_connection()?;
    let path: Option<String> = connection.query_row("SELECT p.normalized_path FROM projects p JOIN agent_sessions s ON s.project_id=p.id WHERE s.id=?1", [session_id], |row| row.get(0)).optional().map_err(|error| error.to_string())?;
    Ok(path.unwrap_or_default())
}

fn load_events(
    connection: &rusqlite::Connection,
    session_id: &str,
) -> Result<Vec<SessionEvent>, String> {
    let mut statement = connection.prepare("SELECT id,event_type,payload_json,occurred_at FROM agent_events WHERE session_id=?1 ORDER BY rowid ASC LIMIT 256").map_err(|error| format!("read agent events: {error}"))?;
    let rows = statement
        .query_map([session_id], |row| {
            let payload: Option<String> = row.get(2)?;
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                payload.unwrap_or_else(|| "null".into()),
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|error| error.to_string())?;
    rows.enumerate()
        .map(|(index, row)| {
            let (id, event_type, payload, occurred_at) = row.map_err(|error| error.to_string())?;
            Ok(SessionEvent {
                sequence: index + 1,
                id,
                event_type,
                payload: serde_json::from_str(&payload).unwrap_or(Value::Null),
                occurred_at,
            })
        })
        .collect()
}

fn insert_event(
    connection: &rusqlite::Connection,
    session_id: &str,
    event_type: &str,
    payload: Value,
) -> Result<(), String> {
    connection.execute("INSERT INTO agent_events (id,session_id,event_type,payload_json,occurred_at) VALUES (?1,?2,?3,?4,?5)", params![Uuid::new_v4().to_string(), session_id, event_type, payload.to_string(), utc_timestamp()]).map_err(|error| format!("persist agent event: {error}"))?;
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn current_timestamp_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}
fn parse_timestamp_ms(value: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|date| date.timestamp_millis())
}
impl AgentSession {
    fn from_codex(session: CodexSession) -> Self {
        Self {
            id: session.id,
            provider: session.provider,
            project_id: session.project_id,
            task_id: session.task_id,
            operation_kind: session.operation_kind,
            state: session.state,
            cwd: session.cwd,
            started_at: session.started_at,
            ended_at: session.ended_at,
            exit_code: session.exit_code,
            stdout: session.stdout,
            stderr: session.stderr,
            stdout_truncated: session.stdout_truncated,
            stderr_truncated: session.stderr_truncated,
            final_response: session.final_response,
            final_response_truncated: session.final_response_truncated,
            final_response_state: session.final_response_state,
            final_response_role: session.final_response_role,
            diagnostic_code: session.diagnostic_code,
            diagnostic_message: session.diagnostic_message,
            prompt_reference: None,
            prompt_body: session.prompt_body,
            provider_version: None,
            elapsed_ms: None,
            supports_resume: false,
            supports_pty: false,
            events: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream_sanitizer::sanitize_record;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn provider_parser_rejects_unallowlisted_values() {
        assert_eq!(
            SessionProvider::parse("CLAUDE").unwrap(),
            SessionProvider::Claude
        );
        assert_eq!(
            SessionProvider::parse("CODEX").unwrap(),
            SessionProvider::Codex
        );
        assert_eq!(
            SessionProvider::parse("powershell").unwrap_err(),
            "AGENT_PROVIDER_UNSUPPORTED"
        );
    }

    #[test]
    fn claude_resolver_skips_wrappers_and_selects_direct_native_candidate() {
        let directory = tempdir().unwrap();
        fs::write(directory.path().join("claude"), "wrapper").unwrap();
        fs::write(directory.path().join("claude.exe"), b"MZ".repeat(40)).unwrap();
        let result = resolve_claude_from_entries(vec![directory.path().to_path_buf()]);
        assert_eq!(result.selected.unwrap().file_name().unwrap(), "claude.exe");
    }

    #[test]
    fn redaction_is_safe_across_split_sensitive_stream_chunks() {
        let mut redactor = StreamRedactor::default();
        assert!(redactor.push(b"authorization: Bearer ").is_empty());
        let output = redactor.push(b"secret-value\n");
        assert_eq!(
            output,
            vec!["authorization: Bearer [REDACTED SENSITIVE VALUE]\n"]
        );
        for marker in [
            "api_key=hidden",
            "apikey=hidden",
            "token=hidden",
            "password=hidden",
            "secret=hidden",
            "sk-hidden",
        ] {
            let sanitized = sanitize_record(format!("{marker}\n").as_bytes());
            assert!(!sanitized.contains("hidden"));
            assert!(!sanitized.contains("sk-hidden"));
        }
    }

    #[test]
    fn sanitized_claude_assistant_survives_persist_and_reload() {
        let database_directory = tempdir().unwrap();
        let project_directory = tempdir().unwrap();
        let database =
            crate::db::DatabaseState::initialize(database_directory.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_directory.path().to_string_lossy().into(),
                name: Some("Claude evidence fixture".into()),
            },
        )
        .unwrap();
        let session_id = "claude-reload-fixture";
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO agent_sessions (id,project_id,provider,state,created_at,prompt_body) VALUES (?1,?2,'CLAUDE','COMPLETED',?3,?4)",
                params![session_id, project.id, utc_timestamp(), "Inspect the project read-only."],
            )
            .unwrap();
        let raw = json!({
            "type": "assistant",
            "message": {"role": "assistant", "content": [{"type": "text", "text": "The project is healthy."}]},
            "api_key": "sk-never-persist"
        });
        let sanitized = sanitize_record(format!("{raw}\n").as_bytes());
        insert_event(
            &connection,
            session_id,
            "STREAM_STDOUT",
            json!({"text":sanitized,"channel":"stdout"}),
        )
        .unwrap();
        drop(connection);
        let reloaded = load_session(&database, session_id).unwrap();
        assert!(reloaded.stdout.contains("The project is healthy."));
        assert!(!reloaded.stdout.contains("sk-never-persist"));
        assert!(reloaded.stdout.contains("REDACTED SENSITIVE VALUE"));

        let mut final_response = FinalResponseCapture::default();
        final_response.observe(
            ProviderKind::Claude,
            &json!({"type":"assistant","message":{"content":[{"type":"text","text":"I'll inspect the repository."}]}}).to_string(),
        );
        final_response.observe(
            ProviderKind::Claude,
            &json!({"type":"result","result":"The durable final answer is available."}).to_string(),
        );
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "UPDATE agent_sessions SET final_response=?2,final_response_truncated=?3,final_response_state=?4,final_response_role='assistant' WHERE id=?1",
                params![session_id, final_response.text(), final_response.truncated(), final_response.state().as_str()],
            )
            .unwrap();
        let reloaded = load_session(&database, session_id).unwrap();
        assert_eq!(
            reloaded.final_response.as_deref(),
            Some("The durable final answer is available.")
        );
        assert_eq!(reloaded.final_response_state, "AVAILABLE");
    }

    #[test]
    fn bounded_capture_never_exceeds_output_limit() {
        let mut capture = Capture::default();
        let _ = capture.append(&"x".repeat(MAX_OUTPUT_BYTES + 10));
        assert!(capture.text.len() <= MAX_OUTPUT_BYTES);
        assert!(capture.truncated);
    }

    #[test]
    fn pty_resize_rejects_invalid_dimensions_before_transport() {
        for dimensions in [(0, 80), (24, 0), (501, 80), (24, 501)] {
            assert_eq!(
                validate_pty_dimensions(dimensions.0, dimensions.1).unwrap_err(),
                "AGENT_PTY_RESIZE_INVALID"
            );
        }
        assert!(validate_pty_dimensions(24, 80).is_ok());
    }

    fn project_record(status: &str, path: &Path) -> ProjectRecord {
        ProjectRecord {
            id: "project-fixture".into(),
            name: "Project fixture".into(),
            original_path: path.to_string_lossy().into(),
            normalized_path: path.to_string_lossy().into(),
            status: status.into(),
            priority: 0,
            preferred_builder: None,
            preferred_auditor: None,
            task_source_policy: None,
            preferred_agent_provider: None,
            registered_at: "2026-01-01T00:00:00.000Z".into(),
            last_validated_at: None,
            repository: None,
        }
    }

    #[test]
    fn operation_project_requires_active_available_registered_root() {
        let directory = tempdir().unwrap();
        assert!(validate_operation_project(&project_record("ACTIVE", directory.path())).is_ok());
        for (status, error) in [
            ("MISSING", "AGENT_PROJECT_MISSING"),
            ("ARCHIVED", "AGENT_PROJECT_ARCHIVED"),
            ("UNKNOWN", "AGENT_PROJECT_NOT_ACTIVE"),
        ] {
            assert_eq!(
                validate_operation_project(&project_record(status, directory.path())).unwrap_err(),
                error
            );
        }
        assert_eq!(
            validate_operation_project(&project_record(
                "ACTIVE",
                &directory.path().join("unavailable")
            ))
            .unwrap_err(),
            "AGENT_PROJECT_PATH_UNAVAILABLE"
        );
    }

    #[test]
    fn task_and_session_operations_reject_cross_project_identity() {
        let db_directory = tempdir().unwrap();
        let project_a_directory = tempdir().unwrap();
        let project_b_directory = tempdir().unwrap();
        fs::write(
            project_a_directory.path().join("TASKS.md"),
            "# Work\n- [ ] A task\n",
        )
        .unwrap();
        fs::write(
            project_b_directory.path().join("TASKS.md"),
            "# Work\n- [ ] B task\n",
        )
        .unwrap();
        let database =
            crate::db::DatabaseState::initialize(db_directory.path().to_path_buf()).unwrap();
        let project_a = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_a_directory.path().to_string_lossy().into(),
                name: Some("Project A".into()),
            },
        )
        .unwrap();
        let project_b = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_b_directory.path().to_string_lossy().into(),
                name: Some("Project B".into()),
            },
        )
        .unwrap();
        let task_a = crate::task_intelligence::parse(&database, &project_a.id)
            .unwrap()
            .tasks[0]
            .id
            .clone();
        assert_eq!(
            validate_task(&database, &project_b.id, Some(&task_a)).unwrap_err(),
            "AGENT_TASK_PROJECT_MISMATCH_OR_MISSING"
        );
        let session_id = "session-project-a";
        database
            .open_connection()
            .unwrap()
            .execute(
                "INSERT INTO agent_sessions (id, project_id, task_id, provider, state, created_at) VALUES (?1, ?2, ?3, 'CLAUDE', 'FAILED', ?4)",
                params![session_id, project_a.id, task_a, utc_timestamp()],
            )
            .unwrap();
        assert_eq!(
            load_session_for_project(&database, &project_b.id, session_id).unwrap_err(),
            "AGENT_SESSION_PROJECT_MISMATCH"
        );
        let center = AgentSessionCenter::default();
        let codex = CodexAdapter::default();
        assert_eq!(
            session_events(&database, &project_b.id, session_id).unwrap_err(),
            "AGENT_SESSION_PROJECT_MISMATCH"
        );
        assert_eq!(
            stop(&center, &codex, &database, &project_b.id, session_id).unwrap_err(),
            "AGENT_SESSION_PROJECT_MISMATCH"
        );
        assert_eq!(
            resume(&database, &project_b.id, session_id).unwrap_err(),
            "AGENT_SESSION_PROJECT_MISMATCH"
        );
        assert_eq!(
            resize(&database, &project_b.id, session_id, 24, 80).unwrap_err(),
            "AGENT_SESSION_PROJECT_MISMATCH"
        );
        assert_eq!(
            retry(
                &center,
                &codex,
                &database,
                AgentRetryRequest {
                    source_session_id: session_id.into(),
                    provider: "CLAUDE".into(),
                    project_id: project_b.id.clone(),
                    task_id: Some(task_a),
                    prompt: "cross-project retry must fail".into(),
                },
            )
            .unwrap_err(),
            "AGENT_RETRY_PROVENANCE_MISMATCH"
        );
    }

    #[test]
    fn claude_invocation_is_fixed_and_prompt_is_not_an_argument() {
        assert_eq!(
            fixed_claude_args(),
            [
                "--print",
                "--output-format",
                "stream-json",
                "--verbose",
                "--no-session-persistence",
                "--permission-mode",
                "plan",
                "--restricted",
            ]
        );
    }

    #[cfg(all(windows, feature = "pty-support"))]
    #[test]
    fn owned_pty_fixture_has_bounded_lifecycle_and_output() {
        use portable_pty::{native_pty_system, CommandBuilder, PtySize};
        let system = native_pty_system();
        let pair = system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();
        let mut command = CommandBuilder::new("cmd.exe");
        command.args(["/C", "echo", "m14-pty-fixture"]);
        let mut child = pair.slave.spawn_command(command).unwrap();
        drop(pair.slave);
        let status = child.wait().unwrap();
        let mut reader = pair.master.try_clone_reader().unwrap();
        drop(pair.master);
        let mut output = Vec::new();
        reader.read_to_end(&mut output).unwrap();
        assert!(status.success());
        assert!(String::from_utf8_lossy(&output).contains("m14-pty-fixture"));
        assert!(output.len() < MAX_OUTPUT_BYTES);
    }
}
