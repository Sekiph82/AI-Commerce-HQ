use crate::db::DatabaseState;
use crate::time::utc_timestamp;
use rusqlite::{params, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::Path;
use std::str::FromStr;

pub const MAX_SUMMARY_BYTES: usize = 4096;
pub const MAX_REQUEST_ID_BYTES: usize = 128;
pub const MAX_EVIDENCE_REFS: usize = 32;
pub const MAX_SCALAR_BYTES: usize = 512;
pub const DEFAULT_HISTORY_LIMIT: usize = 100;
pub const MAX_HISTORY_LIMIT: usize = 500;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkflowState {
    Backlog,
    PlanningRequired,
    PromptRequired,
    PromptReady,
    ReadyForImplementation,
    BuilderRunning,
    ImplementationComplete,
    AuditRequired,
    AuditRunning,
    AuditPassed,
    VerifyRequired,
    VerifyRunning,
    TaskComplete,
    AuditFailed,
    FixRequired,
    ReAuditRequired,
    Blocked,
    WaitingHuman,
    WaitingExternal,
    DesignGate,
}

impl WorkflowState {
    pub fn is_running(self) -> bool {
        matches!(
            self,
            Self::BuilderRunning | Self::AuditRunning | Self::VerifyRunning
        )
    }

    pub fn is_suspension(self) -> bool {
        matches!(
            self,
            Self::Blocked | Self::WaitingHuman | Self::WaitingExternal | Self::DesignGate
        )
    }

    pub fn normal_next(self, had_failed_audit: bool) -> &'static [WorkflowState] {
        match self {
            Self::Backlog => &[Self::PlanningRequired],
            Self::PlanningRequired => &[Self::PromptRequired],
            Self::PromptRequired => &[Self::PromptReady],
            Self::PromptReady => &[Self::ReadyForImplementation],
            Self::ReadyForImplementation => &[Self::BuilderRunning],
            Self::BuilderRunning => &[Self::ImplementationComplete],
            Self::ImplementationComplete => {
                if had_failed_audit {
                    &[Self::ReAuditRequired]
                } else {
                    &[Self::AuditRequired]
                }
            }
            Self::AuditRequired | Self::ReAuditRequired => &[Self::AuditRunning],
            Self::AuditRunning => &[Self::AuditPassed, Self::AuditFailed],
            Self::AuditPassed => &[Self::VerifyRequired],
            Self::VerifyRequired => &[Self::VerifyRunning],
            Self::VerifyRunning => &[Self::TaskComplete],
            Self::AuditFailed => &[Self::FixRequired],
            Self::FixRequired => &[Self::ReadyForImplementation],
            Self::Blocked
            | Self::WaitingHuman
            | Self::WaitingExternal
            | Self::DesignGate
            | Self::TaskComplete => &[],
        }
    }
}

impl fmt::Display for WorkflowState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Backlog => "BACKLOG",
            Self::PlanningRequired => "PLANNING_REQUIRED",
            Self::PromptRequired => "PROMPT_REQUIRED",
            Self::PromptReady => "PROMPT_READY",
            Self::ReadyForImplementation => "READY_FOR_IMPLEMENTATION",
            Self::BuilderRunning => "BUILDER_RUNNING",
            Self::ImplementationComplete => "IMPLEMENTATION_COMPLETE",
            Self::AuditRequired => "AUDIT_REQUIRED",
            Self::AuditRunning => "AUDIT_RUNNING",
            Self::AuditPassed => "AUDIT_PASSED",
            Self::VerifyRequired => "VERIFY_REQUIRED",
            Self::VerifyRunning => "VERIFY_RUNNING",
            Self::TaskComplete => "TASK_COMPLETE",
            Self::AuditFailed => "AUDIT_FAILED",
            Self::FixRequired => "FIX_REQUIRED",
            Self::ReAuditRequired => "RE_AUDIT_REQUIRED",
            Self::Blocked => "BLOCKED",
            Self::WaitingHuman => "WAITING_HUMAN",
            Self::WaitingExternal => "WAITING_EXTERNAL",
            Self::DesignGate => "DESIGN_GATE",
        };
        f.write_str(value)
    }
}

impl FromStr for WorkflowState {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "BACKLOG" => Ok(Self::Backlog),
            "PLANNING_REQUIRED" => Ok(Self::PlanningRequired),
            "PROMPT_REQUIRED" => Ok(Self::PromptRequired),
            "PROMPT_READY" => Ok(Self::PromptReady),
            "READY_FOR_IMPLEMENTATION" => Ok(Self::ReadyForImplementation),
            "BUILDER_RUNNING" => Ok(Self::BuilderRunning),
            "IMPLEMENTATION_COMPLETE" => Ok(Self::ImplementationComplete),
            "AUDIT_REQUIRED" => Ok(Self::AuditRequired),
            "AUDIT_RUNNING" => Ok(Self::AuditRunning),
            "AUDIT_PASSED" => Ok(Self::AuditPassed),
            "VERIFY_REQUIRED" => Ok(Self::VerifyRequired),
            "VERIFY_RUNNING" => Ok(Self::VerifyRunning),
            "TASK_COMPLETE" => Ok(Self::TaskComplete),
            "AUDIT_FAILED" => Ok(Self::AuditFailed),
            "FIX_REQUIRED" => Ok(Self::FixRequired),
            "RE_AUDIT_REQUIRED" => Ok(Self::ReAuditRequired),
            "BLOCKED" => Ok(Self::Blocked),
            "WAITING_HUMAN" => Ok(Self::WaitingHuman),
            "WAITING_EXTERNAL" => Ok(Self::WaitingExternal),
            "DESIGN_GATE" => Ok(Self::DesignGate),
            other => Err(format!(
                "WORKFLOW_UNKNOWN_STATE: persisted state '{other}' is not canonical"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActorType {
    Human,
    Codex,
    Claude,
    GptAudit,
    Ci,
    External,
    System,
}

impl fmt::Display for ActorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Human => "HUMAN",
            Self::Codex => "CODEX",
            Self::Claude => "CLAUDE",
            Self::GptAudit => "GPT_AUDIT",
            Self::Ci => "CI",
            Self::External => "EXTERNAL",
            Self::System => "SYSTEM",
        })
    }
}

impl FromStr for ActorType {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "HUMAN" => Ok(Self::Human),
            "CODEX" => Ok(Self::Codex),
            "CLAUDE" => Ok(Self::Claude),
            "GPT_AUDIT" => Ok(Self::GptAudit),
            "CI" => Ok(Self::Ci),
            "EXTERNAL" => Ok(Self::External),
            "SYSTEM" => Ok(Self::System),
            other => Err(format!(
                "WORKFLOW_UNKNOWN_ACTOR: actor '{other}' is not canonical"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvidenceKind {
    Prompt,
    AgentSession,
    Audit,
    TestRun,
    Decision,
    GitSnapshot,
    TaskSource,
    ExternalReference,
}

impl fmt::Display for EvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Prompt => "PROMPT",
            Self::AgentSession => "AGENT_SESSION",
            Self::Audit => "AUDIT",
            Self::TestRun => "TEST_RUN",
            Self::Decision => "DECISION",
            Self::GitSnapshot => "GIT_SNAPSHOT",
            Self::TaskSource => "TASK_SOURCE",
            Self::ExternalReference => "EXTERNAL_REFERENCE",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceRef {
    pub kind: EvidenceKind,
    pub id: String,
    pub locator: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowTransitionRequest {
    pub task_id: String,
    pub expected_from_state: WorkflowState,
    pub to_state: WorkflowState,
    pub actor_type: ActorType,
    pub request_id: String,
    pub summary: String,
    #[serde(default)]
    pub evidence_refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowOverrideRequest {
    pub task_id: String,
    pub expected_from_state: WorkflowState,
    pub to_state: WorkflowState,
    pub request_id: String,
    pub rationale: String,
    #[serde(default)]
    pub evidence_refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowEvent {
    pub id: String,
    pub task_id: String,
    pub event_type: String,
    pub from_state: Option<WorkflowState>,
    pub to_state: Option<WorkflowState>,
    pub actor_type: Option<ActorType>,
    pub summary: String,
    pub evidence_refs: Vec<EvidenceRef>,
    pub occurred_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowTask {
    pub task_id: String,
    pub project_id: String,
    pub title: String,
    pub current_state: WorkflowState,
    pub workflow_managed: bool,
    pub source_active: bool,
    pub source_retired: bool,
    pub allowed_next_states: Vec<WorkflowState>,
    pub allowed_actors: Vec<ActorType>,
    pub suspension_resume_state: Option<WorkflowState>,
    pub latest_event: Option<WorkflowEvent>,
    pub attention_required: bool,
    pub required_actor: Option<String>,
    pub milestone: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowHistoryQuery {
    pub task_id: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowProjectListQuery {
    pub project_id: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowProjectList {
    pub project_id: String,
    pub tasks: Vec<WorkflowTask>,
}

fn error(code: &str, message: impl AsRef<str>) -> String {
    format!("{code}: {}", message.as_ref())
}

fn validate_scalar(name: &str, value: &str, max: usize, required: bool) -> Result<(), String> {
    if required && value.trim().is_empty() {
        return Err(error(
            "WORKFLOW_INVALID_REQUEST",
            format!("{name} is required"),
        ));
    }
    if value.len() > max {
        return Err(error(
            "WORKFLOW_BOUNDS",
            format!("{name} exceeds {max} UTF-8 bytes"),
        ));
    }
    Ok(())
}

fn validate_refs(refs: &[EvidenceRef]) -> Result<(), String> {
    if refs.len() > MAX_EVIDENCE_REFS {
        return Err(error("WORKFLOW_BOUNDS", "evidence ref limit exceeded"));
    }
    for reference in refs {
        validate_scalar("evidence id", &reference.id, MAX_SCALAR_BYTES, true)?;
        if let Some(locator) = &reference.locator {
            validate_scalar("evidence locator", locator, MAX_SCALAR_BYTES, false)?;
        }
    }
    Ok(())
}

fn normalize_refs(refs: &[EvidenceRef]) -> Vec<EvidenceRef> {
    let mut refs = refs.to_vec();
    refs.sort_by(|a, b| {
        (a.kind.to_string(), &a.id, &a.locator).cmp(&(b.kind.to_string(), &b.id, &b.locator))
    });
    refs
}

fn event_id(task_id: &str, request_id: &str) -> String {
    format!("m10evt:{}", digest(&format!("{task_id}|{request_id}")))
}
fn recovery_event_id(task_id: &str, state: WorkflowState, latest: &str) -> String {
    format!(
        "m10recovery:{}",
        digest(&format!("{task_id}|{state}|{latest}"))
    )
}
fn decision_id(task_id: &str, request_id: &str) -> String {
    format!("m10decision:{}", digest(&format!("{task_id}|{request_id}")))
}
fn digest(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn state_value(value: &str) -> Result<WorkflowState, String> {
    WorkflowState::from_str(value)
}
fn actor_value(value: Option<String>) -> Result<Option<ActorType>, String> {
    value
        .map(|v| ActorType::from_str(&v).map_err(|e| e))
        .transpose()
}

fn event_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkflowEvent> {
    let evidence_json: Option<String> = row.get(7)?;
    let evidence_refs = evidence_json
        .and_then(|json| serde_json::from_str::<Value>(&json).ok())
        .and_then(|value| value.get("evidenceRefs").cloned())
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default();
    let from: Option<String> = row.get(3)?;
    let to: Option<String> = row.get(4)?;
    let actor: Option<String> = row.get(5)?;
    Ok(WorkflowEvent {
        id: row.get(0)?,
        task_id: row.get(1)?,
        event_type: row.get(2)?,
        from_state: from
            .as_deref()
            .map(state_value)
            .transpose()
            .map_err(|_| rusqlite::Error::InvalidQuery)?,
        to_state: to
            .as_deref()
            .map(state_value)
            .transpose()
            .map_err(|_| rusqlite::Error::InvalidQuery)?,
        actor_type: actor_value(actor).map_err(|_| rusqlite::Error::InvalidQuery)?,
        summary: row.get(6)?,
        evidence_refs,
        occurred_at: row.get(8)?,
    })
}

fn fetch_event(tx: &Transaction<'_>, id: &str) -> Result<Option<WorkflowEvent>, String> {
    tx.query_row("SELECT id, task_id, event_type, from_state, to_state, actor_type, summary, evidence_json, occurred_at FROM task_events WHERE id=?1", [id], event_from_row).optional().map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))
}

fn event_fingerprint(
    operation: &str,
    task_id: &str,
    expected: WorkflowState,
    to: WorkflowState,
    actor: Option<ActorType>,
    summary: &str,
    refs: &[EvidenceRef],
) -> Value {
    json!({"operation": operation, "taskId": task_id, "expectedFromState": expected.to_string(), "toState": to.to_string(), "actorType": actor.map(|v| v.to_string()), "summary": summary, "evidenceRefs": normalize_refs(refs)})
}

fn existing_fingerprint(evidence_json: &str) -> Option<Value> {
    serde_json::from_str::<Value>(evidence_json)
        .ok()?
        .get("request")
        .cloned()
}

fn ensure_project_mutable(tx: &Transaction<'_>, project_id: &str) -> Result<(), String> {
    let (status, path): (String, String) = tx
        .query_row(
            "SELECT status, COALESCE(normalized_path, local_path, '') FROM projects WHERE id=?1",
            [project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?
        .ok_or_else(|| error("WORKFLOW_PROJECT_NOT_FOUND", "project is not registered"))?;
    if status != "ACTIVE" {
        return Err(error(
            "WORKFLOW_PROJECT_NOT_MUTABLE",
            format!("project status is {status}"),
        ));
    }
    if !Path::new(&path).exists() {
        return Err(error(
            "WORKFLOW_PROJECT_NOT_MUTABLE",
            "project root is missing",
        ));
    }
    Ok(())
}

fn task_row(
    tx: &Transaction<'_>,
    task_id: &str,
) -> Result<
    (
        String,
        String,
        String,
        String,
        Option<String>,
        Option<String>,
        String,
    ),
    String,
> {
    tx.query_row("SELECT project_id, title, state, metadata_json, required_actor, milestone, updated_at FROM tasks WHERE id=?1", [task_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?))).optional().map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?.ok_or_else(|| error("WORKFLOW_TASK_NOT_FOUND", "task is not registered"))
}

fn has_workflow_history(tx: &Transaction<'_>, task_id: &str) -> Result<bool, String> {
    tx.query_row("SELECT EXISTS(SELECT 1 FROM task_events WHERE task_id=?1 AND event_type LIKE 'WORKFLOW_%')", [task_id], |row| row.get::<_, i64>(0)).map(|v| v != 0).map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))
}

fn had_failed_audit(tx: &Transaction<'_>, task_id: &str) -> Result<bool, String> {
    tx.query_row(
        "SELECT EXISTS(SELECT 1 FROM task_events WHERE task_id=?1 AND to_state='AUDIT_FAILED')",
        [task_id],
        |row| row.get::<_, i64>(0),
    )
    .map(|v| v != 0)
    .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))
}

fn resume_state(
    tx: &Transaction<'_>,
    task_id: &str,
    current: WorkflowState,
) -> Result<WorkflowState, String> {
    let latest: Option<String> = tx.query_row("SELECT evidence_json FROM task_events WHERE task_id=?1 AND to_state IN ('BLOCKED','WAITING_HUMAN','WAITING_EXTERNAL','DESIGN_GATE') ORDER BY occurred_at DESC, id DESC LIMIT 1", [task_id], |row| row.get(0)).optional().map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    if let Some(json) = latest {
        if let Ok(value) = serde_json::from_str::<Value>(&json) {
            if let Some(state) = value.get("resumeState").and_then(Value::as_str) {
                return state_value(state);
            }
        }
    }
    if current == WorkflowState::Blocked && !has_workflow_history(tx, task_id)? {
        return Ok(WorkflowState::Backlog);
    }
    Err(error(
        "WORKFLOW_RESUME_MISSING",
        "suspension has no deterministic resume state",
    ))
}

fn validate_evidence(
    tx: &Transaction<'_>,
    task_id: &str,
    project_id: &str,
    refs: &[EvidenceRef],
) -> Result<(), String> {
    validate_refs(refs)?;
    for reference in refs {
        let exists = match reference.kind {
            EvidenceKind::Prompt => tx.query_row("SELECT EXISTS(SELECT 1 FROM prompts WHERE id=?1 AND task_id=?2 AND project_id=?3)", params![reference.id, task_id, project_id], |r| r.get::<_, i64>(0)),
            EvidenceKind::AgentSession => tx.query_row("SELECT EXISTS(SELECT 1 FROM agent_sessions WHERE id=?1 AND task_id=?2 AND project_id=?3)", params![reference.id, task_id, project_id], |r| r.get::<_, i64>(0)),
            EvidenceKind::Audit => tx.query_row("SELECT EXISTS(SELECT 1 FROM audits WHERE id=?1 AND task_id=?2 AND project_id=?3)", params![reference.id, task_id, project_id], |r| r.get::<_, i64>(0)),
            EvidenceKind::TestRun => tx.query_row("SELECT EXISTS(SELECT 1 FROM test_runs WHERE id=?1 AND task_id=?2 AND project_id=?3)", params![reference.id, task_id, project_id], |r| r.get::<_, i64>(0)),
            EvidenceKind::Decision => tx.query_row("SELECT EXISTS(SELECT 1 FROM decisions WHERE id=?1 AND task_id=?2 AND project_id=?3)", params![reference.id, task_id, project_id], |r| r.get::<_, i64>(0)),
            EvidenceKind::TaskSource => tx.query_row("SELECT EXISTS(SELECT 1 FROM task_sources WHERE id=?1 AND project_id=?2)", params![reference.id, project_id], |r| r.get::<_, i64>(0)),
            EvidenceKind::GitSnapshot => tx.query_row("SELECT EXISTS(SELECT 1 FROM git_snapshots g JOIN repositories r ON r.id=g.repository_id WHERE g.id=?1 AND r.project_id=?2)", params![reference.id, project_id], |r| r.get::<_, i64>(0)),
            EvidenceKind::ExternalReference => { if reference.locator.as_deref().unwrap_or_default().trim().is_empty() { return Err(error("WORKFLOW_EVIDENCE_REQUIRED", "external reference requires a bounded locator")); } Ok(1) },
        }.map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
        if exists == 0 {
            return Err(error(
                "WORKFLOW_EVIDENCE_NOT_FOUND",
                format!(
                    "{} evidence '{}' is not owned by task/project",
                    reference.kind, reference.id
                ),
            ));
        }
    }
    Ok(())
}

fn has_kind(refs: &[EvidenceRef], kind: EvidenceKind) -> bool {
    refs.iter().any(|reference| reference.kind == kind)
}
fn evidence(refs: &[EvidenceRef], kind: EvidenceKind) -> Option<&EvidenceRef> {
    refs.iter().find(|reference| reference.kind == kind)
}

fn validate_gate(
    tx: &Transaction<'_>,
    task_id: &str,
    project_id: &str,
    from: WorkflowState,
    to: WorkflowState,
    refs: &[EvidenceRef],
) -> Result<(), String> {
    validate_evidence(tx, task_id, project_id, refs)?;
    let required = |kind: EvidenceKind| {
        if has_kind(refs, kind) {
            Ok(())
        } else {
            Err(error(
                "WORKFLOW_EVIDENCE_REQUIRED",
                format!("{kind} evidence is required"),
            ))
        }
    };
    match (from, to) {
        (WorkflowState::PromptRequired, WorkflowState::PromptReady) => {
            required(EvidenceKind::Prompt)
        }
        (WorkflowState::PromptReady, WorkflowState::ReadyForImplementation) => {
            required(EvidenceKind::Decision)?;
            let id = evidence(refs, EvidenceKind::Decision).unwrap().id.clone();
            let valid: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM decisions WHERE id=?1 AND UPPER(decision) IN ('APPROVED','APPROVE','PASS','ALLOW'))", [id], |r| r.get(0)).map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
            if valid == 0 {
                return Err(error(
                    "WORKFLOW_EVIDENCE_INCOMPATIBLE",
                    "decision is not explicit approval",
                ));
            }
            Ok(())
        }
        (WorkflowState::ReadyForImplementation, WorkflowState::BuilderRunning) => {
            required(EvidenceKind::AgentSession)?;
            let id = evidence(refs, EvidenceKind::AgentSession)
                .unwrap()
                .id
                .clone();
            let valid: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM agent_sessions WHERE id=?1 AND provider IN ('CODEX','CLAUDE') AND started_at IS NOT NULL AND ended_at IS NULL)", [id], |r| r.get(0)).map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
            if valid == 0 {
                return Err(error(
                    "WORKFLOW_EVIDENCE_INCOMPATIBLE",
                    "builder session is not live CODEX/CLAUDE evidence",
                ));
            }
            Ok(())
        }
        (WorkflowState::BuilderRunning, WorkflowState::ImplementationComplete) => {
            required(EvidenceKind::AgentSession)?;
            let id = evidence(refs, EvidenceKind::AgentSession)
                .unwrap()
                .id
                .clone();
            let valid: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM agent_sessions WHERE id=?1 AND provider IN ('CODEX','CLAUDE') AND started_at IS NOT NULL AND ended_at IS NOT NULL)", [id], |r| r.get(0)).map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
            if valid == 0 {
                return Err(error(
                    "WORKFLOW_EVIDENCE_INCOMPATIBLE",
                    "builder session is not completed evidence",
                ));
            }
            Ok(())
        }
        (
            WorkflowState::AuditRequired | WorkflowState::ReAuditRequired,
            WorkflowState::AuditRunning,
        ) => {
            required(EvidenceKind::AgentSession)?;
            let id = evidence(refs, EvidenceKind::AgentSession)
                .unwrap()
                .id
                .clone();
            let valid: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM agent_sessions WHERE id=?1 AND provider IN ('GPT_AUDIT','CI') AND started_at IS NOT NULL AND ended_at IS NULL)", [id], |r| r.get(0)).map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
            if valid == 0 {
                return Err(error(
                    "WORKFLOW_EVIDENCE_INCOMPATIBLE",
                    "audit execution is not live evidence",
                ));
            }
            Ok(())
        }
        (WorkflowState::AuditRunning, WorkflowState::AuditPassed)
        | (WorkflowState::AuditRunning, WorkflowState::AuditFailed) => {
            required(EvidenceKind::Audit)?;
            let id = evidence(refs, EvidenceKind::Audit).unwrap().id.clone();
            let result: String = tx
                .query_row("SELECT result FROM audits WHERE id=?1", [id], |r| r.get(0))
                .map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
            let pass = result.eq_ignore_ascii_case("PASS");
            if (to == WorkflowState::AuditPassed) != pass {
                return Err(error(
                    "WORKFLOW_EVIDENCE_INCOMPATIBLE",
                    "audit result does not match requested state",
                ));
            }
            Ok(())
        }
        (WorkflowState::VerifyRequired, WorkflowState::VerifyRunning) => {
            required(EvidenceKind::TestRun)?;
            let id = evidence(refs, EvidenceKind::TestRun).unwrap().id.clone();
            let valid: i64 = tx
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM test_runs WHERE id=?1 AND started_at IS NOT NULL)",
                    [id],
                    |r| r.get(0),
                )
                .map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
            if valid == 0 {
                return Err(error(
                    "WORKFLOW_EVIDENCE_INCOMPATIBLE",
                    "verification run has not started",
                ));
            }
            Ok(())
        }
        (WorkflowState::VerifyRunning, WorkflowState::TaskComplete) => {
            required(EvidenceKind::TestRun)?;
            let ids: Vec<String> = refs
                .iter()
                .filter(|r| r.kind == EvidenceKind::TestRun)
                .map(|r| r.id.clone())
                .collect();
            for id in ids {
                let valid: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM test_runs WHERE id=?1 AND result='PASS' AND finished_at IS NOT NULL)", [id], |r| r.get(0)).map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
                if valid == 0 {
                    return Err(error(
                        "WORKFLOW_EVIDENCE_INCOMPATIBLE",
                        "verification run is not finished PASS evidence",
                    ));
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_actor(actor: ActorType, from: WorkflowState, to: WorkflowState) -> Result<(), String> {
    if matches!(to, WorkflowState::BuilderRunning)
        && !matches!(actor, ActorType::Codex | ActorType::Claude)
    {
        return Err(error(
            "WORKFLOW_ACTOR_NOT_ALLOWED",
            "builder execution requires CODEX or CLAUDE",
        ));
    }
    if matches!(to, WorkflowState::AuditRunning)
        && !matches!(actor, ActorType::GptAudit | ActorType::Ci)
    {
        return Err(error(
            "WORKFLOW_ACTOR_NOT_ALLOWED",
            "audit execution requires GPT_AUDIT or CI",
        ));
    }
    if matches!(to, WorkflowState::VerifyRunning) && actor != ActorType::Ci {
        return Err(error(
            "WORKFLOW_ACTOR_NOT_ALLOWED",
            "verification execution requires CI",
        ));
    }
    if from.is_suspension()
        && matches!(
            from,
            WorkflowState::WaitingHuman | WorkflowState::DesignGate
        )
        && actor != ActorType::Human
    {
        return Err(error("WORKFLOW_ACTOR_NOT_ALLOWED", "resume requires HUMAN"));
    }
    if from == WorkflowState::WaitingExternal
        && !matches!(actor, ActorType::External | ActorType::Human)
    {
        return Err(error(
            "WORKFLOW_ACTOR_NOT_ALLOWED",
            "external wait resume requires EXTERNAL or HUMAN",
        ));
    }
    if from == WorkflowState::Blocked && !matches!(actor, ActorType::Human | ActorType::System) {
        return Err(error(
            "WORKFLOW_ACTOR_NOT_ALLOWED",
            "blocked resume actor is not allowed",
        ));
    }
    Ok(())
}

fn validate_override_running_evidence(
    tx: &Transaction<'_>,
    task_id: &str,
    project_id: &str,
    to: WorkflowState,
    refs: &[EvidenceRef],
) -> Result<(), String> {
    validate_evidence(tx, task_id, project_id, refs)?;
    let (kind, query) = match to {
        WorkflowState::BuilderRunning => (
            EvidenceKind::AgentSession,
            "SELECT EXISTS(SELECT 1 FROM agent_sessions WHERE id=?1 AND task_id=?2 AND project_id=?3 AND provider IN ('CODEX','CLAUDE') AND started_at IS NOT NULL AND ended_at IS NULL)",
        ),
        WorkflowState::AuditRunning => (
            EvidenceKind::AgentSession,
            "SELECT EXISTS(SELECT 1 FROM agent_sessions WHERE id=?1 AND task_id=?2 AND project_id=?3 AND provider IN ('GPT_AUDIT','CI') AND started_at IS NOT NULL AND ended_at IS NULL)",
        ),
        WorkflowState::VerifyRunning => (
            EvidenceKind::TestRun,
            "SELECT EXISTS(SELECT 1 FROM test_runs WHERE id=?1 AND task_id=?2 AND project_id=?3 AND started_at IS NOT NULL AND finished_at IS NULL)",
        ),
        _ => return Ok(()),
    };
    let reference = evidence(refs, kind).ok_or_else(|| {
        error(
            "WORKFLOW_EVIDENCE_REQUIRED",
            format!("{kind} evidence is required"),
        )
    })?;
    let valid: i64 = tx
        .query_row(query, params![reference.id, task_id, project_id], |row| {
            row.get(0)
        })
        .map_err(|e| error("WORKFLOW_EVIDENCE_DATABASE", e.to_string()))?;
    if valid == 0 {
        return Err(error(
            "WORKFLOW_EVIDENCE_INCOMPATIBLE",
            "override lacks compatible live execution evidence",
        ));
    }
    Ok(())
}

fn suspension_resume(current: WorkflowState, failed_audit: bool) -> WorkflowState {
    match current {
        WorkflowState::BuilderRunning => WorkflowState::ReadyForImplementation,
        WorkflowState::AuditRunning => {
            if failed_audit {
                WorkflowState::ReAuditRequired
            } else {
                WorkflowState::AuditRequired
            }
        }
        WorkflowState::VerifyRunning => WorkflowState::VerifyRequired,
        _ => current,
    }
}

fn insert_event(
    tx: &Transaction<'_>,
    id: &str,
    task_id: &str,
    event_type: &str,
    from: WorkflowState,
    to: WorkflowState,
    actor: ActorType,
    summary: &str,
    evidence_json: &str,
    occurred_at: &str,
) -> Result<(), String> {
    tx.execute("INSERT INTO task_events (id, task_id, event_type, from_state, to_state, actor_type, summary, evidence_json, occurred_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)", params![id, task_id, event_type, from.to_string(), to.to_string(), actor.to_string(), summary, evidence_json, occurred_at]).map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    let updated = tx
        .execute(
            "UPDATE tasks SET state=?2, updated_at=?3 WHERE id=?1 AND state=?4",
            params![task_id, to.to_string(), occurred_at, from.to_string()],
        )
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    if updated != 1 {
        return Err(error(
            "WORKFLOW_CONFLICT",
            "task state changed during transition",
        ));
    }
    Ok(())
}

fn transition_result(
    tx: &Transaction<'_>,
    event_id_value: &str,
    task_id: &str,
) -> Result<WorkflowEvent, String> {
    tx.query_row("SELECT id, task_id, event_type, from_state, to_state, actor_type, summary, evidence_json, occurred_at FROM task_events WHERE id=?1 AND task_id=?2", params![event_id_value, task_id], event_from_row).map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))
}

pub fn transition(
    database: &DatabaseState,
    request: WorkflowTransitionRequest,
) -> Result<WorkflowEvent, String> {
    validate_scalar("task id", &request.task_id, MAX_SCALAR_BYTES, true)?;
    validate_scalar(
        "request id",
        &request.request_id,
        MAX_REQUEST_ID_BYTES,
        true,
    )?;
    validate_scalar("summary", &request.summary, MAX_SUMMARY_BYTES, true)?;
    validate_refs(&request.evidence_refs)?;
    if request.to_state == request.expected_from_state {
        return Err(error(
            "WORKFLOW_INVALID_TRANSITION",
            "transition must change state",
        ));
    }
    let mut connection = database.open_connection()?;
    let tx = connection
        .transaction()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    let id = event_id(&request.task_id, &request.request_id);
    if let Some(existing) = fetch_event(&tx, &id)? {
        let stored: String = tx
            .query_row(
                "SELECT evidence_json FROM task_events WHERE id=?1",
                [&id],
                |row| row.get(0),
            )
            .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
        let expected = event_fingerprint(
            "TRANSITION",
            &request.task_id,
            request.expected_from_state,
            request.to_state,
            Some(request.actor_type),
            &request.summary,
            &request.evidence_refs,
        );
        if existing_fingerprint(&stored).as_ref() != Some(&expected) {
            return Err(error(
                "WORKFLOW_REQUEST_CONFLICT",
                "request id was reused with different semantics",
            ));
        }
        return Ok(existing);
    }
    let (project_id, _title, current_raw, _metadata, _actor, _milestone, _updated) =
        task_row(&tx, &request.task_id)?;
    let current = state_value(&current_raw)?;
    ensure_project_mutable(&tx, &project_id)?;
    if current != request.expected_from_state {
        return Err(error(
            "WORKFLOW_CONFLICT",
            format!("expected {}, found {current}", request.expected_from_state),
        ));
    }
    let failed = had_failed_audit(&tx, &request.task_id)?;
    let is_suspension = request.to_state.is_suspension();
    if current == WorkflowState::TaskComplete {
        return Err(error(
            "WORKFLOW_INVALID_TRANSITION",
            "TASK_COMPLETE is terminal for normal transitions",
        ));
    }
    if is_suspension {
        if current.is_suspension() {
            return Err(error(
                "WORKFLOW_INVALID_TRANSITION",
                "hold-to-hold chains are not allowed",
            ));
        }
        if request.summary.trim().is_empty() {
            return Err(error(
                "WORKFLOW_EVIDENCE_REQUIRED",
                "suspension reason is required",
            ));
        }
        if request.to_state == WorkflowState::WaitingExternal
            && !has_kind(&request.evidence_refs, EvidenceKind::ExternalReference)
        {
            return Err(error(
                "WORKFLOW_EVIDENCE_REQUIRED",
                "WAITING_EXTERNAL requires EXTERNAL_REFERENCE evidence",
            ));
        }
    } else if current.is_suspension() {
        let resume = resume_state(&tx, &request.task_id, current)?;
        if request.to_state != resume {
            return Err(error(
                "WORKFLOW_INVALID_TRANSITION",
                format!("suspension may only resume to {resume}"),
            ));
        }
    } else if !current.normal_next(failed).contains(&request.to_state) {
        return Err(error(
            "WORKFLOW_INVALID_TRANSITION",
            format!("{current} -> {} is not allowed", request.to_state),
        ));
    }
    validate_actor(request.actor_type, current, request.to_state)?;
    validate_gate(
        &tx,
        &request.task_id,
        &project_id,
        current,
        request.to_state,
        &request.evidence_refs,
    )?;
    let resume = if is_suspension {
        Some(suspension_resume(current, failed))
    } else {
        None
    };
    let fingerprint = event_fingerprint(
        "TRANSITION",
        &request.task_id,
        request.expected_from_state,
        request.to_state,
        Some(request.actor_type),
        &request.summary,
        &request.evidence_refs,
    );
    let evidence_json = json!({"request": fingerprint, "requestId": request.request_id, "evidenceRefs": normalize_refs(&request.evidence_refs), "suspendedState": if is_suspension { Some(current.to_string()) } else { None }, "resumeState": resume.map(|v| v.to_string())}).to_string();
    let occurred_at = utc_timestamp();
    insert_event(
        &tx,
        &id,
        &request.task_id,
        "WORKFLOW_TRANSITION",
        current,
        request.to_state,
        request.actor_type,
        &request.summary,
        &evidence_json,
        &occurred_at,
    )?;
    let result = transition_result(&tx, &id, &request.task_id)?;
    tx.commit()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    Ok(result)
}

pub fn override_state(
    database: &DatabaseState,
    request: WorkflowOverrideRequest,
) -> Result<WorkflowEvent, String> {
    validate_scalar("task id", &request.task_id, MAX_SCALAR_BYTES, true)?;
    validate_scalar(
        "request id",
        &request.request_id,
        MAX_REQUEST_ID_BYTES,
        true,
    )?;
    validate_scalar("rationale", &request.rationale, MAX_SUMMARY_BYTES, true)?;
    validate_refs(&request.evidence_refs)?;
    if request.to_state == request.expected_from_state {
        return Err(error(
            "WORKFLOW_INVALID_TRANSITION",
            "override must change state",
        ));
    }
    let mut connection = database.open_connection()?;
    let tx = connection
        .transaction()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    let id = event_id(&request.task_id, &request.request_id);
    if let Some(existing) = fetch_event(&tx, &id)? {
        let stored: String = tx
            .query_row(
                "SELECT evidence_json FROM task_events WHERE id=?1",
                [&id],
                |row| row.get(0),
            )
            .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
        let expected = event_fingerprint(
            "OVERRIDE",
            &request.task_id,
            request.expected_from_state,
            request.to_state,
            Some(ActorType::Human),
            &request.rationale,
            &request.evidence_refs,
        );
        if existing_fingerprint(&stored).as_ref() != Some(&expected) {
            return Err(error(
                "WORKFLOW_REQUEST_CONFLICT",
                "request id was reused with different semantics",
            ));
        }
        return Ok(existing);
    }
    let (project_id, _title, current_raw, _metadata, _actor, _milestone, _updated) =
        task_row(&tx, &request.task_id)?;
    let current = state_value(&current_raw)?;
    ensure_project_mutable(&tx, &project_id)?;
    if current != request.expected_from_state {
        return Err(error(
            "WORKFLOW_CONFLICT",
            format!("expected {}, found {current}", request.expected_from_state),
        ));
    }
    if request.to_state.is_running() {
        validate_override_running_evidence(
            &tx,
            &request.task_id,
            &project_id,
            request.to_state,
            &request.evidence_refs,
        )?;
        validate_actor(
            if request.to_state == WorkflowState::BuilderRunning {
                ActorType::Codex
            } else if request.to_state == WorkflowState::AuditRunning {
                ActorType::GptAudit
            } else {
                ActorType::Ci
            },
            current,
            request.to_state,
        )?;
    }
    let decision = decision_id(&request.task_id, &request.request_id);
    let now = utc_timestamp();
    tx.execute("INSERT INTO decisions (id, project_id, task_id, decision_kind, decision, rationale, created_at) VALUES (?1,?2,?3,'WORKFLOW_OVERRIDE',?4,?5,?6)", params![decision, project_id, request.task_id, request.to_state.to_string(), request.rationale, now]).map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    let fingerprint = event_fingerprint(
        "OVERRIDE",
        &request.task_id,
        request.expected_from_state,
        request.to_state,
        Some(ActorType::Human),
        &request.rationale,
        &request.evidence_refs,
    );
    let evidence_json = json!({"request": fingerprint, "requestId": request.request_id, "decisionId": decision, "rationale": request.rationale, "evidenceRefs": normalize_refs(&request.evidence_refs)}).to_string();
    insert_event(
        &tx,
        &id,
        &request.task_id,
        "WORKFLOW_OVERRIDE",
        current,
        request.to_state,
        ActorType::Human,
        &request.rationale,
        &evidence_json,
        &now,
    )?;
    let result = transition_result(&tx, &id, &request.task_id)?;
    tx.commit()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    Ok(result)
}

fn history_tx(
    tx: &Transaction<'_>,
    task_id: &str,
    limit: usize,
) -> Result<Vec<WorkflowEvent>, String> {
    let mut statement = tx.prepare("SELECT id, task_id, event_type, from_state, to_state, actor_type, summary, evidence_json, occurred_at FROM task_events WHERE task_id=?1 AND event_type LIKE 'WORKFLOW_%' ORDER BY occurred_at ASC, id ASC LIMIT ?2").map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    let rows = statement
        .query_map(params![task_id, limit as i64], event_from_row)
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))
}

pub fn history(
    database: &DatabaseState,
    query: WorkflowHistoryQuery,
) -> Result<Vec<WorkflowEvent>, String> {
    validate_scalar("task id", &query.task_id, MAX_SCALAR_BYTES, true)?;
    let limit = bounded_limit(query.limit)?;
    let mut connection = database.open_connection()?;
    let tx = connection
        .transaction()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    task_row(&tx, &query.task_id)?;
    let result = history_tx(&tx, &query.task_id, limit)?;
    tx.commit()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    Ok(result)
}

fn bounded_limit(limit: Option<usize>) -> Result<usize, String> {
    let limit = limit.unwrap_or(DEFAULT_HISTORY_LIMIT);
    if limit == 0 || limit > MAX_HISTORY_LIMIT {
        return Err(error(
            "WORKFLOW_BOUNDS",
            format!("history/list limit must be 1..={MAX_HISTORY_LIMIT}"),
        ));
    }
    Ok(limit)
}

fn parse_metadata(metadata: &str) -> Value {
    serde_json::from_str(metadata).unwrap_or_else(|_| json!({}))
}
fn task_read(tx: &Transaction<'_>, task_id: &str, _limit: usize) -> Result<WorkflowTask, String> {
    let (project_id, title, state_raw, metadata_raw, required_actor, milestone, _updated) =
        task_row(tx, task_id)?;
    let state = state_value(&state_raw)?;
    let metadata = parse_metadata(&metadata_raw);
    let source_active = metadata
        .get("sourceActive")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    let workflow_managed = has_workflow_history(tx, task_id)?;
    let failed = had_failed_audit(tx, task_id)?;
    let resume = if state.is_suspension() {
        Some(resume_state(tx, task_id, state)?)
    } else {
        None
    };
    let events = history_tx(tx, task_id, 1)?;
    let allowed_next_states = if state.is_suspension() {
        resume.into_iter().collect()
    } else {
        state.normal_next(failed).to_vec()
    };
    let allowed_actors =
        if state == WorkflowState::WaitingHuman || state == WorkflowState::DesignGate {
            vec![ActorType::Human]
        } else if state == WorkflowState::WaitingExternal {
            vec![ActorType::External, ActorType::Human]
        } else if state == WorkflowState::Blocked {
            vec![ActorType::Human, ActorType::System]
        } else {
            vec![
                ActorType::Human,
                ActorType::Codex,
                ActorType::Claude,
                ActorType::GptAudit,
                ActorType::Ci,
                ActorType::External,
            ]
        };
    Ok(WorkflowTask {
        task_id: task_id.to_string(),
        project_id,
        title,
        current_state: state,
        workflow_managed,
        source_active,
        source_retired: !source_active,
        allowed_next_states,
        allowed_actors,
        suspension_resume_state: resume,
        latest_event: events.into_iter().next(),
        attention_required: state.is_suspension()
            || state == WorkflowState::AuditFailed
            || state == WorkflowState::FixRequired,
        required_actor,
        milestone,
    })
}

pub fn task_get(database: &DatabaseState, task_id: String) -> Result<WorkflowTask, String> {
    validate_scalar("task id", &task_id, MAX_SCALAR_BYTES, true)?;
    let mut connection = database.open_connection()?;
    let tx = connection
        .transaction()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    let result = task_read(&tx, &task_id, 1)?;
    tx.commit()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    Ok(result)
}

pub fn project_list(
    database: &DatabaseState,
    query: WorkflowProjectListQuery,
) -> Result<WorkflowProjectList, String> {
    validate_scalar("project id", &query.project_id, MAX_SCALAR_BYTES, true)?;
    let limit = bounded_limit(query.limit)?;
    let mut connection = database.open_connection()?;
    let tx = connection
        .transaction()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    tx.query_row(
        "SELECT id FROM projects WHERE id=?1",
        [&query.project_id],
        |r| r.get::<_, String>(0),
    )
    .optional()
    .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?
    .ok_or_else(|| error("WORKFLOW_PROJECT_NOT_FOUND", "project is not registered"))?;
    let mut statement = tx
        .prepare(
            "SELECT id FROM tasks WHERE project_id=?1 ORDER BY updated_at ASC, id ASC LIMIT ?2",
        )
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    let ids = statement
        .query_map(params![query.project_id, limit as i64], |r| {
            r.get::<_, String>(0)
        })
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    drop(statement);
    let tasks = ids
        .into_iter()
        .map(|id| task_read(&tx, &id, 1))
        .collect::<Result<Vec<_>, _>>()?;
    tx.commit()
        .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    Ok(WorkflowProjectList {
        project_id: query.project_id,
        tasks,
    })
}

pub fn recover_stale(database: &DatabaseState) -> Result<usize, String> {
    let mut connection = database.open_connection()?;
    let candidates: Vec<(String, String)> = {
        let mut statement = connection.prepare("SELECT t.id, t.state FROM tasks t JOIN projects p ON p.id=t.project_id WHERE p.status='ACTIVE' AND t.state IN ('BUILDER_RUNNING','AUDIT_RUNNING','VERIFY_RUNNING') AND EXISTS(SELECT 1 FROM task_events e WHERE e.task_id=t.id AND e.event_type LIKE 'WORKFLOW_%') ORDER BY t.id LIMIT 4096").map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
        let rows = statement
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
        rows
    };
    let mut recovered = 0;
    for (task_id, raw) in candidates {
        let current = state_value(&raw)?;
        let tx = connection
            .transaction()
            .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
        let (project_id, _, latest_id): (String, String, String) = tx.query_row("SELECT project_id, state, COALESCE((SELECT id FROM task_events WHERE task_id=?1 ORDER BY occurred_at DESC, id DESC LIMIT 1),'') FROM tasks WHERE id=?1", [&task_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?))).map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
        let target = match current {
            WorkflowState::BuilderRunning => WorkflowState::ReadyForImplementation,
            WorkflowState::AuditRunning => {
                if had_failed_audit(&tx, &task_id)? {
                    WorkflowState::ReAuditRequired
                } else {
                    WorkflowState::AuditRequired
                }
            }
            WorkflowState::VerifyRunning => WorkflowState::VerifyRequired,
            _ => continue,
        };
        let id = recovery_event_id(&task_id, current, &latest_id);
        let occurred = utc_timestamp();
        let json = json!({"reason":"native restart interrupted transient execution","interruptedState":current.to_string(),"resumeState":target.to_string()}).to_string();
        let updated = tx
            .execute(
                "UPDATE tasks SET state=?2, updated_at=?3 WHERE id=?1 AND state=?4",
                params![task_id, target.to_string(), occurred, current.to_string()],
            )
            .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
        if updated == 1 {
            tx.execute("INSERT INTO task_events (id, task_id, event_type, from_state, to_state, actor_type, summary, evidence_json, occurred_at) VALUES (?1,?2,'WORKFLOW_RECOVERY',?3,?4,'SYSTEM',?5,?6,?7)", params![id, task_id, current.to_string(), target.to_string(), "native restart recovered stale transient workflow state", json, occurred]).map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
            recovered += 1;
        }
        let _ = project_id;
        tx.commit()
            .map_err(|e| error("WORKFLOW_DATABASE", e.to_string()))?;
    }
    Ok(recovered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use crate::task_intelligence;
    use std::fs;
    use tempfile::tempdir;

    fn fixture(
        contents: &str,
    ) -> (
        tempfile::TempDir,
        tempfile::TempDir,
        DatabaseState,
        String,
        String,
    ) {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(project_dir.path().join("TASKS.md"), contents).unwrap();
        let db = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project = register_project(
            &db,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("M10 Fixture".into()),
            },
        )
        .unwrap();
        let task = task_intelligence::parse(&db, &project.id).unwrap().tasks[0]
            .id
            .clone();
        (db_dir, project_dir, db, project.id, task)
    }
    fn seed(db: &DatabaseState, sql: &str) {
        db.open_connection().unwrap().execute_batch(sql).unwrap();
    }
    fn req(
        task: &str,
        from: WorkflowState,
        to: WorkflowState,
        actor: ActorType,
        id: &str,
        refs: Vec<EvidenceRef>,
    ) -> WorkflowTransitionRequest {
        WorkflowTransitionRequest {
            task_id: task.into(),
            expected_from_state: from,
            to_state: to,
            actor_type: actor,
            request_id: id.into(),
            summary: format!("transition {to}"),
            evidence_refs: refs,
        }
    }
    fn ev(kind: EvidenceKind, id: &str) -> EvidenceRef {
        EvidenceRef {
            kind,
            id: id.into(),
            locator: None,
        }
    }
    fn session_sql(
        project: &str,
        task: &str,
        id: &str,
        provider: &str,
        ended: Option<&str>,
    ) -> String {
        let ended_sql = ended
            .map(|value| format!("'{value}'"))
            .unwrap_or_else(|| "NULL".to_string());
        format!("INSERT INTO agent_sessions (id,project_id,task_id,provider,state,started_at,ended_at,created_at) VALUES ('{id}','{project}','{task}','{provider}','COMPLETED','now',{ended_sql},'now');")
    }

    #[test]
    fn m10_happy_path_requires_each_canonical_step() {
        let (_d, _p, db, project, task) = fixture("- [ ] work\n");
        seed(&db, &format!("INSERT INTO prompts (id,project_id,task_id,kind,created_at,updated_at) VALUES ('prompt-1','{project}','{task}','IMPLEMENTATION','now','now'); INSERT INTO decisions (id,project_id,task_id,decision_kind,decision,rationale,created_at) VALUES ('decision-1','{project}','{task}','PROMPT_APPROVAL','APPROVED','go','now'); {}", session_sql(&project,&task,"builder-1","CODEX",None)));
        let mut s = WorkflowState::Backlog;
        for (i, next) in [
            WorkflowState::PlanningRequired,
            WorkflowState::PromptRequired,
            WorkflowState::PromptReady,
        ]
        .into_iter()
        .enumerate()
        {
            s = next;
            transition(
                &db,
                req(
                    &task,
                    if i == 0 {
                        WorkflowState::Backlog
                    } else if i == 1 {
                        WorkflowState::PlanningRequired
                    } else {
                        WorkflowState::PromptRequired
                    },
                    next,
                    ActorType::Human,
                    &format!("h{i}"),
                    if next == WorkflowState::PromptReady {
                        vec![ev(EvidenceKind::Prompt, "prompt-1")]
                    } else {
                        vec![]
                    },
                ),
            )
            .unwrap();
        }
        transition(
            &db,
            req(
                &task,
                s,
                WorkflowState::ReadyForImplementation,
                ActorType::Human,
                "approval",
                vec![ev(EvidenceKind::Decision, "decision-1")],
            ),
        )
        .unwrap();
        transition(
            &db,
            req(
                &task,
                WorkflowState::ReadyForImplementation,
                WorkflowState::BuilderRunning,
                ActorType::Codex,
                "builder",
                vec![ev(EvidenceKind::AgentSession, "builder-1")],
            ),
        )
        .unwrap();
        seed(&db, &format!("UPDATE agent_sessions SET ended_at='now', state='COMPLETED' WHERE id='builder-1'; INSERT INTO agent_sessions (id,project_id,task_id,provider,state,started_at,created_at) VALUES ('audit-1','{project}','{task}','GPT_AUDIT','RUNNING','now','now');"));
        transition(
            &db,
            req(
                &task,
                WorkflowState::BuilderRunning,
                WorkflowState::ImplementationComplete,
                ActorType::Codex,
                "complete",
                vec![ev(EvidenceKind::AgentSession, "builder-1")],
            ),
        )
        .unwrap();
        transition(
            &db,
            req(
                &task,
                WorkflowState::ImplementationComplete,
                WorkflowState::AuditRequired,
                ActorType::System,
                "audit required",
                vec![],
            ),
        )
        .unwrap();
        transition(
            &db,
            req(
                &task,
                WorkflowState::AuditRequired,
                WorkflowState::AuditRunning,
                ActorType::GptAudit,
                "audit",
                vec![ev(EvidenceKind::AgentSession, "audit-1")],
            ),
        )
        .unwrap();
        seed(&db, &format!("INSERT INTO audits (id,project_id,task_id,result,created_at) VALUES ('audit-result','{project}','{task}','PASS','now');"));
        transition(
            &db,
            req(
                &task,
                WorkflowState::AuditRunning,
                WorkflowState::AuditPassed,
                ActorType::GptAudit,
                "pass",
                vec![ev(EvidenceKind::Audit, "audit-result")],
            ),
        )
        .unwrap();
        transition(
            &db,
            req(
                &task,
                WorkflowState::AuditPassed,
                WorkflowState::VerifyRequired,
                ActorType::System,
                "verify",
                vec![],
            ),
        )
        .unwrap();
        seed(&db, &format!("INSERT INTO test_runs (id,project_id,task_id,command,result,started_at,finished_at) VALUES ('test-1','{project}','{task}','cargo test','PASS','now',NULL);"));
        transition(
            &db,
            req(
                &task,
                WorkflowState::VerifyRequired,
                WorkflowState::VerifyRunning,
                ActorType::Ci,
                "test",
                vec![ev(EvidenceKind::TestRun, "test-1")],
            ),
        )
        .unwrap();
        seed(
            &db,
            "UPDATE test_runs SET result='PASS', finished_at='now' WHERE id='test-1';",
        );
        assert_eq!(
            transition(
                &db,
                req(
                    &task,
                    WorkflowState::VerifyRunning,
                    WorkflowState::TaskComplete,
                    ActorType::Ci,
                    "done",
                    vec![ev(EvidenceKind::TestRun, "test-1")]
                )
            )
            .unwrap()
            .to_state,
            Some(WorkflowState::TaskComplete)
        );
    }

    #[test]
    fn m10_invalid_direct_jump_is_rejected() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::TaskComplete,
                ActorType::Human,
                "jump",
                vec![],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_INVALID_TRANSITION"));
    }
    #[test]
    fn m10_builder_running_requires_matching_live_builder_session() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        seed(
            &db,
            &format!("UPDATE tasks SET state='READY_FOR_IMPLEMENTATION' WHERE id='{task}';"),
        );
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::ReadyForImplementation,
                WorkflowState::BuilderRunning,
                ActorType::Codex,
                "builder",
                vec![],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_EVIDENCE_REQUIRED"));
    }
    #[test]
    fn m10_audit_pass_requires_matching_pass_audit() {
        let (_d, _p, db, project, task) = fixture("- [ ] work\n");
        seed(&db, &format!("UPDATE tasks SET state='AUDIT_RUNNING' WHERE id='{task}'; INSERT INTO audits (id,project_id,task_id,result,created_at) VALUES ('audit-fail','{project}','{task}','FAIL','now');"));
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::AuditRunning,
                WorkflowState::AuditPassed,
                ActorType::GptAudit,
                "audit-pass",
                vec![ev(EvidenceKind::Audit, "audit-fail")],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_EVIDENCE_INCOMPATIBLE"));
    }
    #[test]
    fn m10_verify_complete_requires_finished_pass_test_run() {
        let (_d, _p, db, project, task) = fixture("- [ ] work\n");
        seed(&db, &format!("UPDATE tasks SET state='VERIFY_RUNNING' WHERE id='{task}'; INSERT INTO test_runs (id,project_id,task_id,command,result,started_at,finished_at) VALUES ('test-fail','{project}','{task}','test','FAIL','now','now');"));
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::VerifyRunning,
                WorkflowState::TaskComplete,
                ActorType::Ci,
                "verify",
                vec![ev(EvidenceKind::TestRun, "test-fail")],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_EVIDENCE_INCOMPATIBLE"));
    }
    #[test]
    fn m10_cross_project_evidence_is_rejected() {
        let (_d, _dir, db, _project, task) = fixture("- [ ] work\n");
        let other_dir = tempdir().unwrap();
        fs::write(other_dir.path().join("TASKS.md"), "- [ ] other\n").unwrap();
        let other = register_project(
            &db,
            RegisterProjectRequest {
                path: other_dir.path().to_string_lossy().into(),
                name: Some("Other M10 Fixture".into()),
            },
        )
        .unwrap();
        let other_task = task_intelligence::parse(&db, &other.id).unwrap().tasks[0]
            .id
            .clone();
        seed(&db, &format!("UPDATE tasks SET state='PROMPT_REQUIRED' WHERE id='{task}'; INSERT INTO prompts (id,project_id,task_id,kind,created_at,updated_at) VALUES ('cross-prompt','{}','{other_task}','x','now','now');", other.id));
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::PromptRequired,
                WorkflowState::PromptReady,
                ActorType::Human,
                "cross-project",
                vec![ev(EvidenceKind::Prompt, "cross-prompt")],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_EVIDENCE_NOT_FOUND"));
    }
    #[test]
    fn m10_audit_failure_routes_to_reaudit_after_fix() {
        let (_d, _p, db, project, task) = fixture("- [ ] work\n");
        seed(&db, &format!("UPDATE tasks SET state='AUDIT_RUNNING' WHERE id='{task}'; INSERT INTO task_events (id,task_id,event_type,from_state,to_state,actor_type,summary,evidence_json,occurred_at) VALUES ('audit-start','{task}','WORKFLOW_TRANSITION','AUDIT_REQUIRED','AUDIT_RUNNING','GPT_AUDIT','audit','{{}}','now'); INSERT INTO audits (id,project_id,task_id,result,created_at) VALUES ('audit-fail','{project}','{task}','FAIL','now');"));
        transition(
            &db,
            req(
                &task,
                WorkflowState::AuditRunning,
                WorkflowState::AuditFailed,
                ActorType::GptAudit,
                "failed",
                vec![ev(EvidenceKind::Audit, "audit-fail")],
            ),
        )
        .unwrap();
        transition(
            &db,
            req(
                &task,
                WorkflowState::AuditFailed,
                WorkflowState::FixRequired,
                ActorType::Human,
                "fix",
                vec![],
            ),
        )
        .unwrap();
        transition(
            &db,
            req(
                &task,
                WorkflowState::FixRequired,
                WorkflowState::ReadyForImplementation,
                ActorType::Human,
                "ready",
                vec![],
            ),
        )
        .unwrap();
        seed(&db, &format!("INSERT INTO agent_sessions (id,project_id,task_id,provider,state,started_at,created_at) VALUES ('builder-retry','{project}','{task}','CODEX','RUNNING','now','now');"));
        transition(
            &db,
            req(
                &task,
                WorkflowState::ReadyForImplementation,
                WorkflowState::BuilderRunning,
                ActorType::Codex,
                "retry",
                vec![ev(EvidenceKind::AgentSession, "builder-retry")],
            ),
        )
        .unwrap();
        seed(
            &db,
            "UPDATE agent_sessions SET state='COMPLETED', ended_at='now' WHERE id='builder-retry';",
        );
        transition(
            &db,
            req(
                &task,
                WorkflowState::BuilderRunning,
                WorkflowState::ImplementationComplete,
                ActorType::Codex,
                "retry complete",
                vec![ev(EvidenceKind::AgentSession, "builder-retry")],
            ),
        )
        .unwrap();
        let result = transition(
            &db,
            req(
                &task,
                WorkflowState::ImplementationComplete,
                WorkflowState::ReAuditRequired,
                ActorType::System,
                "re-audit",
                vec![],
            ),
        )
        .unwrap();
        assert_eq!(result.to_state, Some(WorkflowState::ReAuditRequired));
    }
    #[test]
    fn m10_prompt_ready_requires_same_task_prompt() {
        let (_d, _p, db, project, task) = fixture("- [ ] work\n");
        seed(
            &db,
            &format!("UPDATE tasks SET state='PROMPT_REQUIRED' WHERE id='{task}';"),
        );
        seed(&db,&format!("INSERT INTO prompts (id,project_id,task_id,kind,created_at,updated_at) VALUES ('other','{project}',NULL,'x','now','now');"));
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::PromptRequired,
                WorkflowState::PromptReady,
                ActorType::Human,
                "prompt",
                vec![ev(EvidenceKind::Prompt, "other")],
            ),
        )
        .unwrap_err();
        assert!(e.contains("WORKFLOW_EVIDENCE"));
    }
    #[test]
    fn m10_request_id_is_idempotent() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        let a = req(
            &task,
            WorkflowState::Backlog,
            WorkflowState::PlanningRequired,
            ActorType::Human,
            "same",
            vec![],
        );
        let first = transition(&db, a.clone()).unwrap();
        let second = transition(&db, a).unwrap();
        assert_eq!(first.id, second.id);
        assert_eq!(
            db.open_connection()
                .unwrap()
                .query_row("SELECT COUNT(*) FROM task_events", [], |r| r
                    .get::<_, i64>(0))
                .unwrap(),
            1
        );
    }
    #[test]
    fn m10_request_id_conflicting_reuse_is_rejected() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "same",
                vec![],
            ),
        )
        .unwrap();
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PromptRequired,
                ActorType::Human,
                "same",
                vec![],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_REQUEST_CONFLICT"));
    }
    #[test]
    fn m10_expected_state_prevents_stale_double_transition() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "one",
                vec![],
            ),
        )
        .unwrap();
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "two",
                vec![],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_CONFLICT"));
    }
    #[test]
    fn m10_waiting_human_round_trip_resumes_exact_prior_state() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "one",
                vec![],
            ),
        )
        .unwrap();
        transition(
            &db,
            req(
                &task,
                WorkflowState::PlanningRequired,
                WorkflowState::WaitingHuman,
                ActorType::Human,
                "hold",
                vec![],
            ),
        )
        .unwrap();
        let event = transition(
            &db,
            req(
                &task,
                WorkflowState::WaitingHuman,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "resume",
                vec![],
            ),
        )
        .unwrap();
        assert_eq!(event.to_state, Some(WorkflowState::PlanningRequired));
    }
    #[test]
    fn m10_parser_seeded_blocked_defaults_resume_to_backlog() {
        let (_d, _p, db, _project, task) = fixture("- [!] blocked\n");
        task_intelligence::parse(&db, _project.as_str()).unwrap();
        let event = transition(
            &db,
            req(
                &task,
                WorkflowState::Blocked,
                WorkflowState::Backlog,
                ActorType::Human,
                "resume",
                vec![],
            ),
        )
        .unwrap();
        assert_eq!(event.to_state, Some(WorkflowState::Backlog));
    }
    #[test]
    fn m10_override_requires_nonempty_rationale() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        let request = WorkflowOverrideRequest {
            task_id: task,
            expected_from_state: WorkflowState::Backlog,
            to_state: WorkflowState::TaskComplete,
            request_id: "override".into(),
            rationale: "".into(),
            evidence_refs: vec![],
        };
        let e = override_state(&db, request).unwrap_err();
        assert!(e.starts_with("WORKFLOW_INVALID_REQUEST"));
    }
    #[test]
    fn m10_override_records_decision_and_event_atomically() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        let request = WorkflowOverrideRequest {
            task_id: task.clone(),
            expected_from_state: WorkflowState::Backlog,
            to_state: WorkflowState::PlanningRequired,
            request_id: "override".into(),
            rationale: "human correction".into(),
            evidence_refs: vec![],
        };
        override_state(&db, request).unwrap();
        let c = db.open_connection().unwrap();
        assert_eq!(
            c.query_row(
                "SELECT decision_kind FROM decisions WHERE task_id=?1",
                [task.clone()],
                |r| r.get::<_, String>(0)
            )
            .unwrap(),
            "WORKFLOW_OVERRIDE"
        );
        assert_eq!(
            c.query_row(
                "SELECT event_type FROM task_events WHERE task_id=?1",
                [task],
                |r| r.get::<_, String>(0)
            )
            .unwrap(),
            "WORKFLOW_OVERRIDE"
        );
    }
    #[test]
    fn m10_task_complete_reopen_requires_override() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        seed(
            &db,
            "UPDATE tasks SET state='TASK_COMPLETE' WHERE id LIKE 'm09task:%';",
        );
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::TaskComplete,
                WorkflowState::Backlog,
                ActorType::Human,
                "normal",
                vec![],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_INVALID_TRANSITION"));
        let request = WorkflowOverrideRequest {
            task_id: task,
            expected_from_state: WorkflowState::TaskComplete,
            to_state: WorkflowState::Backlog,
            request_id: "override".into(),
            rationale: "reopen".into(),
            evidence_refs: vec![],
        };
        assert_eq!(
            override_state(&db, request).unwrap().event_type,
            "WORKFLOW_OVERRIDE"
        );
    }
    #[test]
    fn m10_running_state_suspension_resumes_to_safe_prerequisite() {
        let (_d, _p, db, project, task) = fixture("- [ ] work\n");
        seed(&db,&format!("UPDATE tasks SET state='BUILDER_RUNNING' WHERE id='{task}'; INSERT INTO task_events (id,task_id,event_type,from_state,to_state,actor_type,summary,evidence_json,occurred_at) VALUES ('seed','{task}','WORKFLOW_TRANSITION','READY_FOR_IMPLEMENTATION','BUILDER_RUNNING','CODEX','run','{{}}','now');"));
        transition(
            &db,
            req(
                &task,
                WorkflowState::BuilderRunning,
                WorkflowState::WaitingHuman,
                ActorType::Human,
                "hold",
                vec![],
            ),
        )
        .unwrap();
        assert_eq!(
            transition(
                &db,
                req(
                    &task,
                    WorkflowState::WaitingHuman,
                    WorkflowState::ReadyForImplementation,
                    ActorType::Human,
                    "resume",
                    vec![]
                )
            )
            .unwrap()
            .to_state,
            Some(WorkflowState::ReadyForImplementation)
        );
        let _ = project;
    }
    #[test]
    fn m10_restart_recovery_demotes_stale_running_states() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        seed(&db,&format!("UPDATE tasks SET state='BUILDER_RUNNING' WHERE id='{task}'; INSERT INTO task_events (id,task_id,event_type,from_state,to_state,actor_type,summary,evidence_json,occurred_at) VALUES ('seed','{task}','WORKFLOW_TRANSITION','READY_FOR_IMPLEMENTATION','BUILDER_RUNNING','CODEX','run','{{}}','now');"));
        assert_eq!(recover_stale(&db).unwrap(), 1);
        assert_eq!(recover_stale(&db).unwrap(), 0);
        assert_eq!(
            task_get(&db, task).unwrap().current_state,
            WorkflowState::ReadyForImplementation
        );
    }
    #[test]
    fn m10_history_is_bounded_and_deterministically_ordered() {
        let (_d, _p, db, _project, task) = fixture("- [ ] work\n");
        for i in 0..3 {
            transition(
                &db,
                req(
                    &task,
                    if i == 0 {
                        WorkflowState::Backlog
                    } else if i == 1 {
                        WorkflowState::PlanningRequired
                    } else {
                        WorkflowState::PromptRequired
                    },
                    if i == 0 {
                        WorkflowState::PlanningRequired
                    } else if i == 1 {
                        WorkflowState::PromptRequired
                    } else {
                        WorkflowState::PromptReady
                    },
                    ActorType::Human,
                    &format!("{i}"),
                    if i == 2 {
                        vec![ev(EvidenceKind::Prompt, "missing")]
                    } else {
                        vec![]
                    },
                ),
            )
            .ok();
        }
        let e = history(
            &db,
            WorkflowHistoryQuery {
                task_id: task,
                limit: Some(1),
            },
        )
        .unwrap();
        assert_eq!(e.len(), 1);
    }
    #[test]
    fn m10_archived_or_missing_project_rejects_mutation_but_allows_history_read() {
        let (_d, _p, db, project, task) = fixture("- [ ] work\n");
        transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "one",
                vec![],
            ),
        )
        .unwrap();
        seed(
            &db,
            &format!("UPDATE projects SET status='ARCHIVED' WHERE id='{project}';"),
        );
        let e = transition(
            &db,
            req(
                &task,
                WorkflowState::PlanningRequired,
                WorkflowState::PromptRequired,
                ActorType::Human,
                "two",
                vec![],
            ),
        )
        .unwrap_err();
        assert!(e.starts_with("WORKFLOW_PROJECT_NOT_MUTABLE"));
        assert_eq!(
            history(
                &db,
                WorkflowHistoryQuery {
                    task_id: task,
                    limit: None
                }
            )
            .unwrap()
            .len(),
            1
        );
    }
    #[test]
    fn m10_m09_reparse_preserves_workflow_state_and_events() {
        let (_d, dir, db, project, task) = fixture("- [ ] TASK-1: old\n");
        transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "one",
                vec![],
            ),
        )
        .unwrap();
        fs::write(dir.path().join("TASKS.md"), "- [ ] TASK-1: changed\n").unwrap();
        task_intelligence::parse(&db, &project).unwrap();
        let c = db.open_connection().unwrap();
        assert_eq!(
            c.query_row("SELECT state FROM tasks WHERE id=?1", [task.clone()], |r| r
                .get::<_, String>(0))
                .unwrap(),
            "PLANNING_REQUIRED"
        );
        assert_eq!(
            c.query_row(
                "SELECT COUNT(*) FROM task_events WHERE task_id=?1",
                [task],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
            1
        );
    }
    #[test]
    fn m10_m09_stale_source_preserves_managed_history() {
        let (_d, dir, db, project, task) = fixture("- [ ] TASK-1: old\n");
        transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "one",
                vec![],
            ),
        )
        .unwrap();
        fs::remove_file(dir.path().join("TASKS.md")).unwrap();
        task_intelligence::parse(&db, &project).unwrap();
        let c = db.open_connection().unwrap();
        assert_eq!(
            c.query_row(
                "SELECT COUNT(*) FROM tasks WHERE id=?1",
                [task.clone()],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
            1
        );
        assert_eq!(
            c.query_row(
                "SELECT json_extract(metadata_json,'$.sourceActive') FROM tasks WHERE id=?1",
                [task.clone()],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
            0
        );
        assert_eq!(
            c.query_row(
                "SELECT COUNT(*) FROM task_events WHERE task_id=?1",
                [task],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
            1
        );
    }
    #[test]
    fn m10_m09_reappearance_reactivates_same_task_without_history_loss() {
        let (_d, dir, db, project, task) = fixture("- [ ] TASK-1: old\n");
        let created = db
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT created_at FROM tasks WHERE id=?1",
                [task.clone()],
                |r| r.get::<_, String>(0),
            )
            .unwrap();
        transition(
            &db,
            req(
                &task,
                WorkflowState::Backlog,
                WorkflowState::PlanningRequired,
                ActorType::Human,
                "one",
                vec![],
            ),
        )
        .unwrap();
        fs::remove_file(dir.path().join("TASKS.md")).unwrap();
        task_intelligence::parse(&db, &project).unwrap();
        fs::write(dir.path().join("TASKS.md"), "- [ ] TASK-1: new\n").unwrap();
        task_intelligence::parse(&db, &project).unwrap();
        let c = db.open_connection().unwrap();
        assert_eq!(
            c.query_row("SELECT title FROM tasks WHERE id=?1", [task.clone()], |r| r
                .get::<_, String>(0))
                .unwrap(),
            "new"
        );
        assert_eq!(
            c.query_row("SELECT state FROM tasks WHERE id=?1", [task.clone()], |r| r
                .get::<_, String>(0))
                .unwrap(),
            "PLANNING_REQUIRED"
        );
        assert_eq!(
            c.query_row(
                "SELECT created_at FROM tasks WHERE id=?1",
                [task.clone()],
                |r| r.get::<_, String>(0)
            )
            .unwrap(),
            created
        );
        assert_eq!(
            c.query_row(
                "SELECT COUNT(*) FROM task_events WHERE task_id=?1",
                [task],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
            1
        );
    }
    #[test]
    fn m10_no_history_stale_task_keeps_m09_cleanup() {
        let (_d, dir, db, project, task) = fixture("- [ ] old\n");
        fs::remove_file(dir.path().join("TASKS.md")).unwrap();
        task_intelligence::parse(&db, &project).unwrap();
        assert_eq!(
            db.open_connection()
                .unwrap()
                .query_row("SELECT COUNT(*) FROM tasks WHERE id=?1", [task], |r| r
                    .get::<_, i64>(0))
                .unwrap(),
            0
        );
    }
}
