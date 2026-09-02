use crate::db::DatabaseState;
use crate::process_policy::background_command;
use crate::projects::{fetch_project, ProjectRecord};
use crate::time::utc_timestamp;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Output, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use uuid::Uuid;

const PROVIDER: &str = "CODEX";
const MAX_PROMPT_BYTES: usize = 64 * 1024;
const MAX_OUTPUT_BYTES: usize = 64 * 1024;
const MAX_OUTPUT_EVENTS: usize = 128;
const MAX_SESSION_EVENTS: usize = MAX_OUTPUT_EVENTS * 2 + 16;
const MAX_REDACTION_CARRY_BYTES: usize = 4096;
const PERSIST_QUEUE_CAPACITY: usize = 32;
const PERSIST_RETRY_ATTEMPTS: usize = 3;
const PERSIST_RETRY_BACKOFF: [Duration; 2] = [Duration::from_millis(10), Duration::from_millis(25)];
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
    pub prompt_body: Option<String>,
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

#[derive(Debug, Clone, Copy, Default)]
struct ChannelPersistenceStats {
    bytes: usize,
    events: usize,
}

#[derive(Debug, Clone, Default)]
struct PersistenceStats {
    stdout: ChannelPersistenceStats,
    stderr: ChannelPersistenceStats,
    failures: usize,
    degraded: bool,
    diagnostic_code: Option<String>,
    diagnostic_message: Option<String>,
}

#[derive(Default)]
struct Capture {
    text: String,
    retained_bytes: usize,
    event_count: usize,
    truncated: bool,
}

impl Capture {
    fn append(&mut self, redacted_text: &str) -> Option<(String, usize, bool)> {
        if self.event_count >= MAX_OUTPUT_EVENTS {
            self.truncated = true;
            return None;
        }
        let bytes = redacted_text.as_bytes();
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

#[derive(Default)]
struct StreamRedactor {
    carry: Vec<u8>,
    discard_until_newline: bool,
}

impl StreamRedactor {
    fn push(&mut self, bytes: &[u8]) -> Vec<String> {
        let mut output = Vec::new();
        let mut remaining = bytes;
        while !remaining.is_empty() {
            if self.discard_until_newline {
                if let Some(index) = remaining.iter().position(|byte| *byte == b'\n') {
                    self.discard_until_newline = false;
                    remaining = &remaining[index + 1..];
                } else {
                    return output;
                }
            } else {
                self.carry.extend_from_slice(remaining);
                remaining = &[];
                while let Some(index) = self.carry.iter().position(|byte| *byte == b'\n') {
                    let record: Vec<u8> = self.carry.drain(..=index).collect();
                    output.push(redact_record(&record));
                }
                if self.carry.len() > MAX_REDACTION_CARRY_BYTES {
                    output.push("[REDACTED SENSITIVE OUTPUT]".to_string());
                    self.carry.clear();
                    self.discard_until_newline = true;
                }
            }
        }
        output
    }

    fn finish(&mut self) -> Vec<String> {
        if self.discard_until_newline {
            self.carry.clear();
            self.discard_until_newline = false;
            return Vec::new();
        }
        if self.carry.is_empty() {
            return Vec::new();
        }
        let record = std::mem::take(&mut self.carry);
        vec![redact_record(&record)]
    }
}

fn redact_record(record: &[u8]) -> String {
    let text = String::from_utf8_lossy(record);
    let has_newline = text.ends_with('\n');
    let content = text.trim_end_matches('\n').trim_end_matches('\r');
    if contains_sensitive_marker(content) {
        let mut redacted = "[REDACTED SENSITIVE OUTPUT]".to_string();
        if has_newline {
            redacted.push('\n');
        }
        redacted
    } else {
        text.into_owned()
    }
}

fn contains_sensitive_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
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
}

trait EventStore: Send {
    fn insert_event(
        &mut self,
        session_id: &str,
        event_type: &str,
        payload: serde_json::Value,
    ) -> Result<(), String>;
}

struct SqliteEventStore {
    connection: rusqlite::Connection,
}

impl EventStore for SqliteEventStore {
    fn insert_event(
        &mut self,
        session_id: &str,
        event_type: &str,
        payload: serde_json::Value,
    ) -> Result<(), String> {
        insert_event(&self.connection, session_id, event_type, payload)
    }
}

struct UnavailableEventStore {
    error: String,
}

impl EventStore for UnavailableEventStore {
    fn insert_event(
        &mut self,
        _session_id: &str,
        _event_type: &str,
        _payload: serde_json::Value,
    ) -> Result<(), String> {
        Err(self.error.clone())
    }
}

enum PersistRequest {
    Stream {
        session_id: String,
        channel: String,
        sequence: usize,
        text: String,
        truncated: bool,
    },
    Shutdown,
}

#[derive(Clone)]
struct EventWriterHandle {
    sender: SyncSender<PersistRequest>,
    stats: Arc<Mutex<PersistenceStats>>,
}

struct EventWriter {
    handle: EventWriterHandle,
    join: Option<thread::JoinHandle<()>>,
}

impl EventWriter {
    fn spawn(database: &DatabaseState, stats: Arc<Mutex<PersistenceStats>>) -> Self {
        let store = database
            .open_connection()
            .map(|connection| Box::new(SqliteEventStore { connection }) as Box<dyn EventStore>)
            .unwrap_or_else(|error| Box::new(UnavailableEventStore { error }));
        Self::spawn_with_store(store, stats)
    }

    fn spawn_with_store(
        mut store: Box<dyn EventStore>,
        stats: Arc<Mutex<PersistenceStats>>,
    ) -> Self {
        let (sender, receiver) = sync_channel(PERSIST_QUEUE_CAPACITY);
        let writer_stats = stats.clone();
        let join = thread::spawn(move || event_writer_loop(&mut *store, receiver, writer_stats));
        Self {
            handle: EventWriterHandle { sender, stats },
            join: Some(join),
        }
    }

    fn handle(&self) -> EventWriterHandle {
        self.handle.clone()
    }

    fn finish(mut self) -> PersistenceStats {
        let _ = self.handle.sender.send(PersistRequest::Shutdown);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
        self.handle
            .stats
            .lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| PersistenceStats {
                degraded: true,
                diagnostic_code: Some("CODEX_PERSISTENCE_STATE_POISONED".into()),
                diagnostic_message: Some("persistence state could not be read".into()),
                ..PersistenceStats::default()
            })
    }
}

impl EventWriterHandle {
    fn stream(
        &self,
        session_id: &str,
        channel: &str,
        sequence: usize,
        text: String,
        truncated: bool,
    ) {
        let request = PersistRequest::Stream {
            session_id: session_id.to_string(),
            channel: channel.to_string(),
            sequence,
            text,
            truncated,
        };
        if self.sender.send(request).is_err() {
            mark_persistence_failure(
                &self.stats,
                "CODEX_PERSISTENCE_WRITER_UNAVAILABLE",
                "the bounded persistence writer stopped before the stream event was accepted",
            );
        }
    }
}

fn event_writer_loop(
    store: &mut dyn EventStore,
    receiver: Receiver<PersistRequest>,
    stats: Arc<Mutex<PersistenceStats>>,
) {
    while let Ok(request) = receiver.recv() {
        match request {
            PersistRequest::Stream {
                session_id,
                channel,
                sequence,
                text,
                truncated,
            } => {
                let payload = serde_json::json!({
                    "channel": channel,
                    "sequence": sequence,
                    "text": text,
                    "truncated": truncated
                });
                let persisted =
                    persist_with_bounded_retry(store, &session_id, "STREAM_OUTPUT", payload);
                if persisted.is_ok() {
                    record_persisted_output(&stats, &channel, text.len());
                } else if let Err(error) = persisted {
                    let first_failure = mark_persistence_failure(
                        &stats,
                        "CODEX_STREAM_OUTPUT_PERSISTENCE_FAILED",
                        &bounded_error(&error),
                    );
                    if first_failure {
                        let diagnostic = serde_json::json!({
                            "diagnosticCode":"CODEX_STREAM_OUTPUT_PERSISTENCE_FAILED",
                            "diagnosticMessage":bounded_error(&error),
                            "channel":channel,
                            "sequence":sequence,
                            "durable":false
                        });
                        let _ = persist_with_bounded_retry(
                            store,
                            &session_id,
                            "PERSISTENCE_DEGRADED",
                            diagnostic,
                        );
                    }
                }
            }
            PersistRequest::Shutdown => break,
        }
    }
}

fn persist_with_bounded_retry(
    store: &mut dyn EventStore,
    session_id: &str,
    event_type: &str,
    payload: serde_json::Value,
) -> Result<(), String> {
    let mut last_error = String::from("unknown persistence failure");
    for attempt in 0..PERSIST_RETRY_ATTEMPTS {
        match store.insert_event(session_id, event_type, payload.clone()) {
            Ok(()) => return Ok(()),
            Err(error) => {
                last_error = error;
                if attempt + 1 < PERSIST_RETRY_ATTEMPTS {
                    thread::sleep(PERSIST_RETRY_BACKOFF[attempt]);
                }
            }
        }
    }
    Err(last_error)
}

fn record_persisted_output(stats: &Arc<Mutex<PersistenceStats>>, channel: &str, bytes: usize) {
    if let Ok(mut state) = stats.lock() {
        let target = if channel == "STDERR" {
            &mut state.stderr
        } else {
            &mut state.stdout
        };
        target.bytes += bytes;
        target.events += 1;
    }
}

fn mark_persistence_failure(
    stats: &Arc<Mutex<PersistenceStats>>,
    code: &str,
    message: &str,
) -> bool {
    let Ok(mut state) = stats.lock() else {
        return false;
    };
    let first_failure = !state.degraded;
    state.degraded = true;
    state.failures = state.failures.saturating_add(1);
    if state.diagnostic_code.is_none() {
        state.diagnostic_code = Some(code.to_string());
        state.diagnostic_message = Some(message.to_string());
    }
    first_failure
}

fn bounded_error(error: &str) -> String {
    error.chars().take(512).collect()
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
    let resolution = resolve_codex_executable();
    let Some(executable) = resolution.selected.as_deref() else {
        let (code, message) = if resolution.skipped_candidates > 0 {
            (
                "CODEX_NATIVE_EXECUTABLE_NOT_FOUND",
                format!(
                    "No native codex.exe was available; skipped {} invalid candidate(s)",
                    resolution.skipped_candidates
                ),
            )
        } else {
            (
                "CODEX_EXECUTABLE_NOT_FOUND",
                "No codex.exe was found on PATH or the bounded native install fallback".into(),
            )
        };
        return AdapterReadiness {
            provider: PROVIDER.into(),
            available: false,
            version: None,
            readiness_state: "UNAVAILABLE".into(),
            diagnostic_code: Some(code.into()),
            diagnostic_message: Some(message),
            checked_at,
        };
    };
    match probe_version(&executable, READINESS_TIMEOUT) {
        Ok(version) => AdapterReadiness {
            provider: PROVIDER.into(),
            available: true,
            version: Some(version),
            readiness_state: "VERSION_VERIFIED_AUTH_UNKNOWN".into(),
            diagnostic_code: Some("AUTH_READINESS_UNVERIFIED".into()),
            diagnostic_message: Some(selected_executable_message(resolution.skipped_candidates)),
            checked_at,
        },
        Err(ProbeError::Timeout) => unavailable_readiness(
            "PROBE_TIMEOUT",
            "CODEX_VERSION_PROBE_TIMEOUT",
            "Codex version probe exceeded its bounded timeout",
            checked_at,
        ),
        Err(ProbeError::Malformed) => unavailable_readiness(
            "MALFORMED_VERSION",
            "CODEX_VERSION_MALFORMED",
            "The selected native codex.exe returned malformed bounded version output",
            checked_at,
        ),
        Err(ProbeError::Failed) => unavailable_readiness(
            "PROBE_FAILED",
            "CODEX_VERSION_PROBE_FAILED",
            "The selected native codex.exe failed its bounded version probe",
            checked_at,
        ),
    }
}

fn selected_executable_message(skipped_candidates: usize) -> String {
    if skipped_candidates == 0 {
        "Native codex.exe selected; account authentication is determined when a bounded operation starts".into()
    } else {
        format!(
            "Native codex.exe selected after skipping {} invalid candidate(s); account authentication is determined when a bounded operation starts",
            skipped_candidates
        )
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
    let resolution = resolve_codex_executable();
    let executable = resolution.selected.as_deref().ok_or_else(|| {
        if resolution.skipped_candidates > 0 {
            format!(
                "CODEX_NATIVE_EXECUTABLE_NOT_FOUND: no valid native codex.exe candidate (skipped {} invalid candidate(s))",
                resolution.skipped_candidates
            )
        } else {
            "CODEX_EXECUTABLE_NOT_FOUND: no codex.exe was found on PATH or the bounded native install fallback".to_string()
        }
    })?;
    let session_id = Uuid::new_v4().to_string();
    let started_at = utc_timestamp();
    let operation_kind = if request.task_id.is_some() {
        "TASK_OPERATION"
    } else {
        "FREEFORM_PROJECT_OPERATION"
    };
    let connection = database.open_connection()?;
    connection.execute("INSERT INTO agent_sessions (id,project_id,task_id,provider,state,started_at,created_at,prompt_body) VALUES (?1,?2,?3,?4,'STARTING',?5,?5,?6)", params![session_id, request.project_id, request.task_id, PROVIDER, started_at, request.prompt]).map_err(|error| format!("persist Codex session: {error}"))?;
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
        serde_json::json!({"executable":"codex.exe","argumentPolicy":"FIXED_ADAPTER_ARGS","model":"gpt-5.5","ignoreUserConfig":true,"cwd":cwd.to_string_lossy(),"shell":false,"promptTransport":"STDIN_BOUNDED"}),
    )?;
    let mut command = background_command(executable);
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
    let persistence_stats = Arc::new(Mutex::new(PersistenceStats::default()));
    let event_writer = EventWriter::spawn(database, persistence_stats);
    let event_writer_for_stdout = event_writer.handle();
    let event_writer_for_stderr = event_writer.handle();
    let stdout_capture_for_monitor = stdout_capture.clone();
    let stderr_capture_for_monitor = stderr_capture.clone();
    let session_for_stdout = session_id.clone();
    let session_for_stderr = session_id.clone();
    let stdout_thread = thread::spawn(move || {
        read_stream(
            stdout,
            stdout_capture,
            event_writer_for_stdout,
            session_for_stdout,
            "STDOUT",
        )
    });
    let stderr_thread = thread::spawn(move || {
        read_stream(
            stderr,
            stderr_capture,
            event_writer_for_stderr,
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
            event_writer,
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
    event_writer: EventWriter,
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
    let persistence = event_writer.finish();
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
        let lifecycle_result = connection.execute(
            "UPDATE agent_sessions SET state=?2,ended_at=?3 WHERE id=?1",
            params![session_id, state, utc_timestamp()],
        );
        let mut finish_payload = session_finished_payload(
            state,
            exit_code,
            termination,
            stdout_state,
            stderr_state,
            persistence,
        );
        if state == "FAILED" {
            finish_payload["diagnosticCode"] =
                serde_json::Value::String("CODEX_PROCESS_FAILED".into());
            finish_payload["diagnosticMessage"] = serde_json::Value::String(
                exit_code
                    .map(|code| format!("Codex exited with code {code}"))
                    .unwrap_or_else(|| "Codex exited without an exit code".into()),
            );
        } else if state == "CRASHED" {
            finish_payload["diagnosticCode"] =
                serde_json::Value::String("CODEX_PROCESS_CRASHED".into());
            finish_payload["diagnosticMessage"] =
                serde_json::Value::String("Codex process ended without a status".into());
        }
        if let Err(error) = lifecycle_result {
            finish_payload["lifecyclePersistenceError"] =
                serde_json::Value::String(bounded_error(&error.to_string()));
        }
        let _ = insert_event_with_bounded_retry(
            &connection,
            &session_id,
            "SESSION_FINISHED",
            finish_payload,
        );
    }
    if let Ok(mut owned) = processes.lock() {
        owned.remove(&session_id);
    }
}

fn read_stream<R: Read>(
    mut reader: R,
    capture: Arc<Mutex<Capture>>,
    writer: EventWriterHandle,
    session_id: String,
    channel: &str,
) {
    let mut buffer = [0u8; 4096];
    let mut redactor = StreamRedactor::default();
    loop {
        let Ok(size) = reader.read(&mut buffer) else {
            break;
        };
        if size == 0 {
            break;
        }
        persist_redacted_records(
            redactor.push(&buffer[..size]),
            &capture,
            &writer,
            &session_id,
            channel,
        );
    }
    persist_redacted_records(redactor.finish(), &capture, &writer, &session_id, channel);
}

fn persist_redacted_records(
    records: Vec<String>,
    capture: &Arc<Mutex<Capture>>,
    writer: &EventWriterHandle,
    session_id: &str,
    channel: &str,
) {
    for record in records {
        let event = capture
            .lock()
            .ok()
            .and_then(|mut target| target.append(&record));
        if let Some((text, sequence, truncated)) = event {
            writer.stream(session_id, channel, sequence, text, truncated);
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
        "--model".into(),
        "gpt-5.5".into(),
        "--sandbox".into(),
        "workspace-write".into(),
        "--cd".into(),
        cwd.to_string_lossy().into_owned(),
        "--ephemeral".into(),
        "--ignore-user-config".into(),
        "--skip-git-repo-check".into(),
    ]
}

const MAX_NATIVE_HEADER_OFFSET: u64 = 1024 * 1024;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct CodexExecutableResolution {
    selected: Option<PathBuf>,
    skipped_candidates: usize,
}

fn resolve_codex_executable() -> CodexExecutableResolution {
    let mut entries = std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .unwrap_or_default();
    entries.extend(known_codex_install_candidates());
    resolve_codex_executable_from_entries(entries)
}

fn resolve_codex_executable_from_entries<I>(entries: I) -> CodexExecutableResolution
where
    I: IntoIterator<Item = PathBuf>,
{
    let mut resolution = CodexExecutableResolution::default();
    let mut seen = std::collections::HashSet::new();
    for entry in entries {
        for name in codex_executable_names() {
            let candidate = entry.join(name);
            if !candidate.is_file() || !seen.insert(candidate.clone()) {
                continue;
            }
            if !is_direct_candidate_name(name) {
                resolution.skipped_candidates = resolution.skipped_candidates.saturating_add(1);
                continue;
            }
            if is_native_codex_candidate(&candidate) {
                resolution.selected = Some(candidate);
                return resolution;
            }
            resolution.skipped_candidates = resolution.skipped_candidates.saturating_add(1);
        }
    }
    resolution
}

#[cfg(windows)]
fn codex_executable_names() -> &'static [&'static str] {
    &["codex.exe", "codex"]
}

#[cfg(not(windows))]
fn codex_executable_names() -> &'static [&'static str] {
    &["codex", "codex.exe"]
}

#[cfg(windows)]
fn is_direct_candidate_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("codex.exe")
}

#[cfg(not(windows))]
fn is_direct_candidate_name(_name: &str) -> bool {
    true
}

#[cfg(windows)]
fn known_codex_install_candidates() -> Vec<PathBuf> {
    std::env::var_os("LOCALAPPDATA")
        .map(|root| vec![PathBuf::from(root).join(r"OpenAI\Codex\bin")])
        .unwrap_or_default()
}

#[cfg(not(windows))]
fn known_codex_install_candidates() -> Vec<PathBuf> {
    Vec::new()
}

#[cfg(windows)]
fn is_native_codex_candidate(path: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() || metadata.len() < 64 {
        return false;
    }
    let Ok(mut file) = std::fs::File::open(path) else {
        return false;
    };
    let mut dos_header = [0u8; 64];
    if file.read_exact(&mut dos_header).is_err() || &dos_header[..2] != b"MZ" {
        return false;
    }
    let pe_offset = u32::from_le_bytes([
        dos_header[0x3c],
        dos_header[0x3d],
        dos_header[0x3e],
        dos_header[0x3f],
    ]) as u64;
    if pe_offset > MAX_NATIVE_HEADER_OFFSET || pe_offset.saturating_add(26) > metadata.len() {
        return false;
    }
    if file.seek(SeekFrom::Start(pe_offset)).is_err() {
        return false;
    }
    let mut pe_header = [0u8; 26];
    if file.read_exact(&mut pe_header).is_err() || &pe_header[..4] != b"PE\0\0" {
        return false;
    }
    let machine = u16::from_le_bytes([pe_header[4], pe_header[5]]);
    let optional_magic = u16::from_le_bytes([pe_header[24], pe_header[25]]);
    matches!(machine, 0x014c | 0x8664 | 0xaa64) && matches!(optional_magic, 0x010b | 0x020b)
}

#[cfg(not(windows))]
fn is_native_codex_candidate(path: &Path) -> bool {
    path.is_file()
}

#[derive(Debug)]
enum ProbeError {
    Timeout,
    Malformed,
    Failed,
}

fn probe_version(path: &Path, timeout: Duration) -> Result<String, ProbeError> {
    let mut child = background_command(path)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| ProbeError::Failed)?;
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let output = child.wait_with_output().map_err(|_| ProbeError::Failed)?;
                if !status.success() {
                    return Err(ProbeError::Failed);
                }
                return parse_version(&output);
            }
            Ok(None) if Instant::now() < deadline => thread::sleep(PROCESS_POLL),
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ProbeError::Timeout);
            }
            Err(_) => return Err(ProbeError::Failed),
        }
    }
}

fn parse_version(output: &Output) -> Result<String, ProbeError> {
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let first = text.lines().next().unwrap_or_default().trim();
    if first.len() > 256 || !first.to_ascii_lowercase().contains("codex") {
        return Err(ProbeError::Malformed);
    }
    Ok(first.to_string())
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

fn insert_event_with_bounded_retry(
    connection: &rusqlite::Connection,
    session_id: &str,
    event_type: &str,
    payload: serde_json::Value,
) -> Result<(), String> {
    let mut last_error = String::from("unknown persistence failure");
    for attempt in 0..PERSIST_RETRY_ATTEMPTS {
        match insert_event(connection, session_id, event_type, payload.clone()) {
            Ok(()) => return Ok(()),
            Err(error) => {
                last_error = error;
                if attempt + 1 < PERSIST_RETRY_ATTEMPTS {
                    thread::sleep(PERSIST_RETRY_BACKOFF[attempt]);
                }
            }
        }
    }
    Err(last_error)
}

fn session_finished_payload(
    state: &str,
    exit_code: Option<i32>,
    termination: &str,
    stdout: (bool, usize, usize),
    stderr: (bool, usize, usize),
    persistence: PersistenceStats,
) -> serde_json::Value {
    serde_json::json!({
        "state": state,
        "exitCode": exit_code,
        "termination": termination,
        "stdoutTruncated": stdout.0,
        "stderrTruncated": stderr.0,
        "stdoutBytes": persistence.stdout.bytes,
        "stderrBytes": persistence.stderr.bytes,
        "stdoutEvents": persistence.stdout.events,
        "stderrEvents": persistence.stderr.events,
        "stdoutCapturedBytes": stdout.1,
        "stderrCapturedBytes": stderr.1,
        "stdoutCapturedEvents": stdout.2,
        "stderrCapturedEvents": stderr.2,
        "stdoutPersistedBytes": persistence.stdout.bytes,
        "stderrPersistedBytes": persistence.stderr.bytes,
        "stdoutPersistedEvents": persistence.stdout.events,
        "stderrPersistedEvents": persistence.stderr.events,
        "stdoutPersistenceDegraded": persistence.degraded && stdout.2 > persistence.stdout.events,
        "stderrPersistenceDegraded": persistence.degraded && stderr.2 > persistence.stderr.events,
        "outputDegraded": persistence.degraded,
        "persistenceFailures": persistence.failures,
        "persistenceDiagnosticCode": persistence.diagnostic_code,
        "persistenceDiagnosticMessage": persistence.diagnostic_message
    })
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
    let row = connection.query_row("SELECT s.id,s.project_id,s.task_id,s.state,s.started_at,s.ended_at,p.original_path,s.prompt_body FROM agent_sessions s LEFT JOIN projects p ON p.id=s.project_id WHERE s.id=?1", [session_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, String>(3)?, row.get::<_, Option<String>>(4)?, row.get::<_, Option<String>>(5)?, row.get::<_, Option<String>>(6)?, row.get::<_, Option<String>>(7)?))).map_err(|error| format!("read Codex session: {error}"))?;
    let mut stdout = String::new();
    let mut stderr = String::new();
    let mut stdout_truncated = false;
    let mut stderr_truncated = false;
    let mut diagnostic_code = None;
    let mut diagnostic_message = None;
    let mut exit_code = None;
    let mut stdout_stream = Vec::new();
    let mut stderr_stream = Vec::new();
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
                    .unwrap_or_default()
                    .to_string();
                let sequence = value
                    .get("sequence")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(usize::MAX as u64) as usize;
                let truncated = value
                    .get("truncated")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
                if value.get("channel").and_then(serde_json::Value::as_str) == Some("STDERR") {
                    stderr_stream.push((sequence, text, truncated));
                } else {
                    stdout_stream.push((sequence, text, truncated));
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
                if diagnostic_code.is_none() {
                    diagnostic_code = value
                        .get("persistenceDiagnosticCode")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                }
                if diagnostic_message.is_none() {
                    diagnostic_message = value
                        .get("persistenceDiagnosticMessage")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string);
                }
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
    stdout_stream.sort_by_key(|(sequence, _, _)| *sequence);
    stderr_stream.sort_by_key(|(sequence, _, _)| *sequence);
    for (_, text, truncated) in stdout_stream {
        stdout.push_str(&text);
        stdout_truncated |= truncated;
    }
    for (_, text, truncated) in stderr_stream {
        stderr.push_str(&text);
        stderr_truncated |= truncated;
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
        prompt_body: row.7,
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
    let output = background_command("taskkill.exe")
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
    use std::io;
    use std::process::Command;
    use tempfile::tempdir;

    struct TinyChunkReader {
        bytes: Vec<u8>,
        offset: usize,
        chunk_size: usize,
    }

    impl TinyChunkReader {
        fn new(value: &str, chunk_size: usize) -> Self {
            Self {
                bytes: value.as_bytes().to_vec(),
                offset: 0,
                chunk_size,
            }
        }
    }

    impl Read for TinyChunkReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if self.offset == self.bytes.len() {
                return Ok(0);
            }
            let size = self
                .chunk_size
                .min(buffer.len())
                .min(self.bytes.len() - self.offset);
            buffer[..size].copy_from_slice(&self.bytes[self.offset..self.offset + size]);
            self.offset += size;
            Ok(size)
        }
    }

    #[derive(Default)]
    struct TestStoreState {
        events: Vec<(String, serde_json::Value)>,
        stream_attempts: usize,
        transient_failures: usize,
        terminal_stream_failure: bool,
    }

    #[derive(Clone)]
    struct TestStore {
        state: Arc<Mutex<TestStoreState>>,
    }

    impl EventStore for TestStore {
        fn insert_event(
            &mut self,
            _session_id: &str,
            event_type: &str,
            payload: serde_json::Value,
        ) -> Result<(), String> {
            let mut state = self.state.lock().unwrap();
            if event_type == "STREAM_OUTPUT" {
                state.stream_attempts += 1;
                if state.terminal_stream_failure
                    || state.stream_attempts <= state.transient_failures
                {
                    return Err("database is locked".into());
                }
            }
            state.events.push((event_type.to_string(), payload));
            Ok(())
        }
    }

    fn test_store(
        transient_failures: usize,
        terminal_stream_failure: bool,
    ) -> (TestStore, Arc<Mutex<TestStoreState>>) {
        let state = Arc::new(Mutex::new(TestStoreState {
            transient_failures,
            terminal_stream_failure,
            ..TestStoreState::default()
        }));
        (
            TestStore {
                state: state.clone(),
            },
            state,
        )
    }

    fn seed_session(database: &DatabaseState, session_id: &str) {
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT OR IGNORE INTO projects (id,name,local_path,status,priority,created_at,updated_at,original_path,normalized_path,registered_at) VALUES ('p','Project','C:\\project','ACTIVE',0,'now','now','C:\\project','c:\\project','now')", []).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,created_at) VALUES (?1,'p','CODEX','RUNNING','now')", [session_id]).unwrap();
    }

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

    #[cfg(windows)]
    fn write_native_pe_fixture(path: &Path) {
        let mut bytes = vec![0u8; 0x80];
        bytes[..2].copy_from_slice(b"MZ");
        bytes[0x3c..0x40].copy_from_slice(&0x40u32.to_le_bytes());
        bytes[0x40..0x44].copy_from_slice(b"PE\0\0");
        bytes[0x44..0x46].copy_from_slice(&0x8664u16.to_le_bytes());
        bytes[0x56..0x58].copy_from_slice(&0x0002u16.to_le_bytes());
        bytes[0x58..0x5a].copy_from_slice(&0x020bu16.to_le_bytes());
        std::fs::write(path, bytes).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn resolver_skips_earlier_extensionless_shim_and_selects_later_exe() {
        let directory = tempdir().unwrap();
        let earlier = directory.path().join("earlier");
        let later = directory.path().join("later");
        std::fs::create_dir_all(&earlier).unwrap();
        std::fs::create_dir_all(&later).unwrap();
        std::fs::write(earlier.join("codex"), b"@echo off\necho shim").unwrap();
        let expected = later.join("codex.exe");
        write_native_pe_fixture(&expected);

        let resolution = resolve_codex_executable_from_entries(vec![earlier, later]);

        assert_eq!(resolution.selected, Some(expected));
        assert_eq!(resolution.skipped_candidates, 1);
    }

    #[cfg(windows)]
    #[test]
    fn resolver_skips_invalid_exe_candidate_and_continues() {
        let directory = tempdir().unwrap();
        let earlier = directory.path().join("invalid");
        let later = directory.path().join("valid");
        std::fs::create_dir_all(&earlier).unwrap();
        std::fs::create_dir_all(&later).unwrap();
        std::fs::write(earlier.join("codex.exe"), b"not a PE executable").unwrap();
        let expected = later.join("codex.exe");
        write_native_pe_fixture(&expected);

        let resolution = resolve_codex_executable_from_entries(vec![earlier, later]);

        assert_eq!(resolution.selected, Some(expected));
        assert_eq!(resolution.skipped_candidates, 1);
    }

    #[cfg(windows)]
    #[test]
    fn resolver_preserves_first_valid_exe_order_deterministically() {
        let directory = tempdir().unwrap();
        let first = directory.path().join("first");
        let second = directory.path().join("second");
        std::fs::create_dir_all(&first).unwrap();
        std::fs::create_dir_all(&second).unwrap();
        let expected = first.join("codex.exe");
        write_native_pe_fixture(&expected);
        write_native_pe_fixture(&second.join("codex.exe"));

        let resolution = resolve_codex_executable_from_entries(vec![first, second]);

        assert_eq!(resolution.selected, Some(expected));
        assert_eq!(resolution.skipped_candidates, 0);
    }

    #[cfg(windows)]
    #[test]
    fn resolver_reports_unavailable_when_no_valid_native_exe_exists() {
        let directory = tempdir().unwrap();
        let shim_dir = directory.path().join("shim");
        let invalid_dir = directory.path().join("invalid");
        std::fs::create_dir_all(&shim_dir).unwrap();
        std::fs::create_dir_all(&invalid_dir).unwrap();
        std::fs::write(shim_dir.join("codex"), b"shim").unwrap();
        std::fs::write(invalid_dir.join("codex.exe"), b"not a PE executable").unwrap();

        let resolution = resolve_codex_executable_from_entries(vec![shim_dir, invalid_dir]);

        assert_eq!(resolution.selected, None);
        assert_eq!(resolution.skipped_candidates, 2);
    }

    #[cfg(windows)]
    #[test]
    fn readiness_and_start_share_the_same_resolver_policy() {
        let directory = tempdir().unwrap();
        let earlier = directory.path().join("shim");
        let later = directory.path().join("native");
        std::fs::create_dir_all(&earlier).unwrap();
        std::fs::create_dir_all(&later).unwrap();
        std::fs::write(earlier.join("codex"), b"shim").unwrap();
        write_native_pe_fixture(&later.join("codex.exe"));
        let entries = vec![earlier, later];

        let readiness_resolution = resolve_codex_executable_from_entries(entries.clone());
        let start_resolution = resolve_codex_executable_from_entries(entries);

        assert_eq!(readiness_resolution, start_resolution);
        assert_eq!(
            readiness_resolution.selected.unwrap().file_name().unwrap(),
            "codex.exe"
        );
    }

    #[test]
    fn prompt_metacharacters_and_flags_are_one_data_argument() {
        let args = fixed_exec_args(Path::new("C:\\registered"));
        assert_eq!(
            args,
            vec![
                "exec",
                "--json",
                "--model",
                "gpt-5.5",
                "--sandbox",
                "workspace-write",
                "--cd",
                "C:\\registered",
                "--ephemeral",
                "--ignore-user-config",
                "--skip-git-repo-check",
            ]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
        );
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
        let writer =
            EventWriter::spawn(&database, Arc::new(Mutex::new(PersistenceStats::default())));
        let data = b"first\napi_key=secret\n";
        read_stream(
            std::io::Cursor::new(data),
            capture,
            writer.handle(),
            "s".into(),
            "STDOUT",
        );
        let _ = writer.finish();
        let connection = database.open_connection().unwrap();
        let payloads: Vec<String> = connection
            .prepare("SELECT payload_json FROM agent_events WHERE session_id='s' AND event_type='STREAM_OUTPUT'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(|row| row.unwrap())
            .collect();
        let payload = payloads.join("\n");
        assert!(payload.contains("first"));
        assert!(payload.contains("REDACTED"));
        assert!(!payload.contains("secret"));
    }

    #[test]
    fn stream_redaction_handles_every_protected_marker_across_tiny_chunks() {
        let cases = [
            ("api_key", "api_key=super-secret-value"),
            ("apikey", "apikey=super-secret-value"),
            ("token", "token=super-secret-value"),
            ("password", "password=super-secret-value"),
            ("secret", "secret=super-secret-value"),
            ("authorization", "authorization: Bearer super-secret-value"),
            ("sk-", "sk-super-secret-value"),
        ];
        for (index, (marker, line)) in cases.iter().enumerate() {
            let directory = tempdir().unwrap();
            let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
            let session_id = format!("s-{index}");
            seed_session(&database, &session_id);
            let stats = Arc::new(Mutex::new(PersistenceStats::default()));
            let writer = EventWriter::spawn(&database, stats);
            read_stream(
                TinyChunkReader::new(&format!("{line}\n"), 1),
                Arc::new(Mutex::new(Capture::default())),
                writer.handle(),
                session_id.clone(),
                "STDOUT",
            );
            let _ = writer.finish();
            let connection = database.open_connection().unwrap();
            let payloads: Vec<String> = connection
                .prepare("SELECT payload_json FROM agent_events WHERE session_id=?1 AND event_type='STREAM_OUTPUT'")
                .unwrap()
                .query_map([session_id], |row| row.get(0))
                .unwrap()
                .map(|row| row.unwrap())
                .collect();
            assert!(
                !payloads.is_empty(),
                "marker {marker} must produce evidence"
            );
            let combined = payloads.join("\n").to_ascii_lowercase();
            assert!(!combined.contains(&line.to_ascii_lowercase()));
            assert!(!combined.contains("super-secret-value"));
            assert!(!combined.contains(marker));
            assert!(combined.contains("redacted"));
        }
    }

    #[test]
    fn stream_redaction_flushes_unterminated_sensitive_lines_without_leaking() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        seed_session(&database, "unterminated");
        let writer =
            EventWriter::spawn(&database, Arc::new(Mutex::new(PersistenceStats::default())));
        read_stream(
            TinyChunkReader::new("password=super-secret-value", 1),
            Arc::new(Mutex::new(Capture::default())),
            writer.handle(),
            "unterminated".into(),
            "STDOUT",
        );
        let writer = writer.finish();
        assert!(!writer.degraded);
        let connection = database.open_connection().unwrap();
        let payload: String = connection
            .query_row("SELECT payload_json FROM agent_events WHERE session_id='unterminated' AND event_type='STREAM_OUTPUT'", [], |row| row.get(0))
            .unwrap();
        assert!(!payload.contains("password"));
        assert!(payload.contains("REDACTED"));
    }

    #[test]
    fn normal_utf8_content_crossing_chunks_reconstructs_from_durable_events() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        seed_session(&database, "utf8");
        let writer =
            EventWriter::spawn(&database, Arc::new(Mutex::new(PersistenceStats::default())));
        read_stream(
            TinyChunkReader::new("normal café output\nsecond line", 1),
            Arc::new(Mutex::new(Capture::default())),
            writer.handle(),
            "utf8".into(),
            "STDOUT",
        );
        let _ = writer.finish();
        let session = load_session(&database, "utf8").unwrap();
        assert_eq!(session.stdout, "normal café output\nsecond line");
    }

    #[test]
    fn durable_stream_rows_match_final_persisted_output_counts() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        seed_session(&database, "counted");
        let capture = Arc::new(Mutex::new(Capture::default()));
        let writer =
            EventWriter::spawn(&database, Arc::new(Mutex::new(PersistenceStats::default())));
        read_stream(
            TinyChunkReader::new("one\ntwo\n", 1),
            capture.clone(),
            writer.handle(),
            "counted".into(),
            "STDOUT",
        );
        let persistence = writer.finish();
        let captured = capture.lock().unwrap();
        let payload = session_finished_payload(
            "COMPLETED",
            Some(0),
            "NATURAL",
            (
                captured.truncated,
                captured.retained_bytes,
                captured.event_count,
            ),
            (false, 0, 0),
            persistence,
        );
        let connection = database.open_connection().unwrap();
        let durable_rows: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM agent_events WHERE session_id='counted' AND event_type='STREAM_OUTPUT'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(durable_rows, payload["stdoutPersistedEvents"]);
        assert_eq!(
            payload["stdoutCapturedEvents"],
            payload["stdoutPersistedEvents"]
        );
        assert_eq!(payload["outputDegraded"], false);
    }

    #[test]
    fn stateful_redaction_remains_bounded_before_capture_caps_are_applied() {
        let mut redactor = StreamRedactor::default();
        let output = redactor.push(&vec![b'a'; MAX_REDACTION_CARRY_BYTES + 1]);
        assert!(output.iter().any(|value| value.contains("REDACTED")));
        assert!(redactor.carry.len() <= MAX_REDACTION_CARRY_BYTES);
        let mut capture = Capture::default();
        for _ in 0..(MAX_OUTPUT_EVENTS + 4) {
            let _ = capture.append("safe");
        }
        assert_eq!(capture.event_count, MAX_OUTPUT_EVENTS);
        assert!(capture.truncated);
    }

    #[test]
    fn stateful_redacted_output_keeps_durable_event_and_byte_caps() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        seed_session(&database, "capped");
        let capture = Arc::new(Mutex::new(Capture::default()));
        let writer =
            EventWriter::spawn(&database, Arc::new(Mutex::new(PersistenceStats::default())));
        let input = (0..(MAX_OUTPUT_EVENTS + 8))
            .map(|index| format!("safe-line-{index}\n"))
            .collect::<String>();
        read_stream(
            TinyChunkReader::new(&input, 1),
            capture.clone(),
            writer.handle(),
            "capped".into(),
            "STDOUT",
        );
        let _ = writer.finish();
        let captured = capture.lock().unwrap();
        let connection = database.open_connection().unwrap();
        let (rows, bytes): (i64, i64) = connection
            .query_row(
                "SELECT COUNT(*), COALESCE(SUM(length(json_extract(payload_json, '$.text'))), 0) FROM agent_events WHERE session_id='capped' AND event_type='STREAM_OUTPUT'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(rows, MAX_OUTPUT_EVENTS as i64);
        assert!(bytes <= MAX_OUTPUT_BYTES as i64);
        assert!(captured.truncated);
    }

    #[test]
    fn concurrent_stdout_stderr_writes_use_one_bounded_writer_and_keep_channel_sequences() {
        let (store, state) = test_store(0, false);
        let writer = EventWriter::spawn_with_store(
            Box::new(store),
            Arc::new(Mutex::new(PersistenceStats::default())),
        );
        let stdout_writer = writer.handle();
        let stderr_writer = writer.handle();
        let stdout = thread::spawn(move || {
            read_stream(
                TinyChunkReader::new("out-1\nout-2\n", 1),
                Arc::new(Mutex::new(Capture::default())),
                stdout_writer,
                "s".into(),
                "STDOUT",
            )
        });
        let stderr = thread::spawn(move || {
            read_stream(
                TinyChunkReader::new("err-1\nerr-2\n", 1),
                Arc::new(Mutex::new(Capture::default())),
                stderr_writer,
                "s".into(),
                "STDERR",
            )
        });
        stdout.join().unwrap();
        stderr.join().unwrap();
        let persistence = writer.finish();
        assert_eq!(persistence.stdout.events, 2);
        assert_eq!(persistence.stderr.events, 2);
        let state = state.lock().unwrap();
        let stdout_sequences: Vec<i64> = state
            .events
            .iter()
            .filter(|(event_type, _)| event_type == "STREAM_OUTPUT")
            .filter(|(_, payload)| payload["channel"] == "STDOUT")
            .map(|(_, payload)| payload["sequence"].as_i64().unwrap())
            .collect();
        let stderr_sequences: Vec<i64> = state
            .events
            .iter()
            .filter(|(event_type, _)| event_type == "STREAM_OUTPUT")
            .filter(|(_, payload)| payload["channel"] == "STDERR")
            .map(|(_, payload)| payload["sequence"].as_i64().unwrap())
            .collect();
        assert_eq!(stdout_sequences, vec![1, 2]);
        assert_eq!(stderr_sequences, vec![1, 2]);
    }

    #[test]
    fn transient_persistence_failure_recovers_with_bounded_retries() {
        let (store, state) = test_store(2, false);
        let stats = Arc::new(Mutex::new(PersistenceStats::default()));
        let writer = EventWriter::spawn_with_store(Box::new(store), stats);
        read_stream(
            TinyChunkReader::new("recoverable\n", 1),
            Arc::new(Mutex::new(Capture::default())),
            writer.handle(),
            "s".into(),
            "STDOUT",
        );
        let persistence = writer.finish();
        assert!(!persistence.degraded);
        assert_eq!(persistence.stdout.events, 1);
        assert_eq!(state.lock().unwrap().stream_attempts, 3);
    }

    #[test]
    fn terminal_persistence_failure_is_explicit_and_never_claims_durable_output() {
        let (store, state) = test_store(0, true);
        let stats = Arc::new(Mutex::new(PersistenceStats::default()));
        let writer = EventWriter::spawn_with_store(Box::new(store), stats);
        let capture = Arc::new(Mutex::new(Capture::default()));
        read_stream(
            TinyChunkReader::new("lost-output\n", 1),
            capture.clone(),
            writer.handle(),
            "s".into(),
            "STDOUT",
        );
        let persistence = writer.finish();
        let captured = capture.lock().unwrap();
        assert_eq!(captured.event_count, 1);
        assert_eq!(persistence.stdout.events, 0);
        assert!(persistence.degraded);
        assert_eq!(
            state.lock().unwrap().stream_attempts,
            PERSIST_RETRY_ATTEMPTS
        );
        let events = state.lock().unwrap();
        assert!(events
            .events
            .iter()
            .any(|(event_type, _)| event_type == "PERSISTENCE_DEGRADED"));
        let payload = session_finished_payload(
            "FAILED",
            Some(9),
            "NATURAL",
            (false, captured.retained_bytes, captured.event_count),
            (false, 0, 0),
            persistence,
        );
        assert_eq!(payload["stdoutCapturedEvents"], 1);
        assert_eq!(payload["stdoutPersistedEvents"], 0);
        assert_eq!(payload["stdoutEvents"], 0);
        assert_eq!(payload["outputDegraded"], true);
    }

    #[test]
    fn final_evidence_preserves_truthful_terminal_states_and_counts() {
        for state in ["COMPLETED", "FAILED", "STOPPED", "CRASHED"] {
            let payload = session_finished_payload(
                state,
                None,
                "NATURAL",
                (true, 12, 3),
                (false, 4, 1),
                PersistenceStats {
                    stdout: ChannelPersistenceStats {
                        bytes: 8,
                        events: 2,
                    },
                    stderr: ChannelPersistenceStats {
                        bytes: 4,
                        events: 1,
                    },
                    failures: 1,
                    degraded: true,
                    diagnostic_code: Some("CODEX_STREAM_OUTPUT_PERSISTENCE_FAILED".into()),
                    diagnostic_message: Some("locked".into()),
                },
            );
            assert_eq!(payload["state"], state);
            assert_eq!(payload["stdoutCapturedEvents"], 3);
            assert_eq!(payload["stdoutPersistedEvents"], 2);
            assert_eq!(payload["stderrCapturedEvents"], 1);
            assert_eq!(payload["stderrPersistedEvents"], 1);
            assert_eq!(payload["outputDegraded"], true);
        }
    }

    #[test]
    fn stdout_stderr_and_final_exit_evidence_remain_distinct() {
        let directory = tempdir().unwrap();
        let database = DatabaseState::initialize(directory.path().to_path_buf()).unwrap();
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO projects (id,name,local_path,status,priority,created_at,updated_at,original_path,normalized_path,registered_at) VALUES ('p','Project','C:\\project','ACTIVE',0,'now','now','C:\\project','c:\\project','now')", []).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,started_at,created_at) VALUES ('s','p','CODEX','RUNNING','now','now')", []).unwrap();
        drop(connection);
        let writer =
            EventWriter::spawn(&database, Arc::new(Mutex::new(PersistenceStats::default())));
        read_stream(
            std::io::Cursor::new(b"stdout-line"),
            Arc::new(Mutex::new(Capture::default())),
            writer.handle(),
            "s".into(),
            "STDOUT",
        );
        read_stream(
            std::io::Cursor::new(b"stderr-line"),
            Arc::new(Mutex::new(Capture::default())),
            writer.handle(),
            "s".into(),
            "STDERR",
        );
        let _ = writer.finish();
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
            let _ = capture.append("x");
        }
        assert_eq!(capture.event_count, MAX_OUTPUT_EVENTS);
        assert!(capture.truncated);
        let mut bytes = Capture::default();
        let _ = bytes.append(&"a".repeat(MAX_OUTPUT_BYTES + 1));
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
            let writer =
                EventWriter::spawn(&database, Arc::new(Mutex::new(PersistenceStats::default())));
            let writer_handle = writer.handle();
            move || {
                read_stream(
                    stdout,
                    Arc::new(Mutex::new(Capture::default())),
                    writer_handle,
                    "s".into(),
                    "STDOUT",
                );
                let _ = writer.finish();
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
        let event_writer =
            EventWriter::spawn(&database, Arc::new(Mutex::new(PersistenceStats::default())));
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
                    event_writer,
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
