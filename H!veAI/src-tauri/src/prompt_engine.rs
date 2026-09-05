use crate::agent_session_center::{self, AgentSession, AgentSessionCenter, AgentStartRequest};
use crate::codex_adapter::CodexAdapter;
use crate::db::DatabaseState;
use crate::project_dashboard;
use crate::projects::fetch_project;
use crate::time::utc_timestamp;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use uuid::Uuid;

pub const MAX_CONTEXT_BYTES: usize = 64 * 1024;
pub const MAX_CONTEXT_ITEMS: usize = 64;
pub const MAX_CONTEXT_SOURCES: usize = 16;
pub const MAX_PROMPT_BODY_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PromptKind {
    Implementation,
    Remediation,
    AuditSupport,
}

impl PromptKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Implementation => "IMPLEMENTATION",
            Self::Remediation => "REMEDIATION",
            Self::AuditSupport => "AUDIT_SUPPORT",
        }
    }
    fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_ascii_uppercase().as_str() {
            "IMPLEMENTATION" => Ok(Self::Implementation),
            "REMEDIATION" => Ok(Self::Remediation),
            "AUDIT_SUPPORT" => Ok(Self::AuditSupport),
            _ => Err("PROMPT_KIND_UNSUPPORTED".into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PromptApprovalState {
    Draft,
    Approved,
    Dispatching,
    Dispatched,
    DispatchFailed,
    Superseded,
}

impl PromptApprovalState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "DRAFT",
            Self::Approved => "APPROVED",
            Self::Dispatching => "DISPATCHING",
            Self::Dispatched => "DISPATCHED",
            Self::DispatchFailed => "DISPATCH_FAILED",
            Self::Superseded => "SUPERSEDED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContextDisposition {
    Included,
    Omitted,
    Truncated,
    Stale,
    Unavailable,
    Excluded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContextItem {
    pub class: String,
    pub reference: String,
    pub disposition: ContextDisposition,
    pub bytes: usize,
    pub reason: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContextManifest {
    pub project_id: String,
    pub task_id: Option<String>,
    pub items: Vec<ContextItem>,
    pub included_bytes: usize,
    pub omitted_count: usize,
    pub source_count: usize,
    pub manifest_sha256: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptVersion {
    pub id: String,
    pub prompt_id: String,
    pub version: i64,
    pub kind: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: String,
    pub created_by: String,
    pub created_at: String,
    pub origin: String,
    pub context_manifest: Option<ContextManifest>,
    pub provenance: serde_json::Value,
    pub approval_state: String,
    pub approved_at: Option<String>,
    pub approved_body_sha256: Option<String>,
    pub used_at: Option<String>,
    pub selected_provider: Option<String>,
    pub dispatched_session_id: Option<String>,
    pub superseded_at: Option<String>,
    pub dispatch_state: String,
    pub dispatch_reservation_id: Option<String>,
    pub dispatch_reserved_at: Option<String>,
    pub dispatch_provenance: serde_json::Value,
    pub dispatch_error: Option<String>,
    pub body_sha256: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptRecord {
    pub id: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
    pub kind: String,
    pub current_version: i64,
    pub created_at: String,
    pub updated_at: String,
    pub current: Option<PromptVersion>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptContextRequest {
    pub project_id: String,
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptGenerateRequest {
    pub project_id: String,
    pub task_id: Option<String>,
    pub kind: PromptKind,
    pub title: String,
    pub summary: String,
    pub finding_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptEditRequest {
    pub project_id: String,
    pub prompt_id: String,
    pub version_id: String,
    pub content: String,
    pub title: Option<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptApproveRequest {
    pub project_id: String,
    pub prompt_id: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptDispatchRequest {
    pub project_id: String,
    pub prompt_id: String,
    pub version_id: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptDispatchResult {
    pub prompt: PromptVersion,
    pub session: AgentSession,
    pub prompt_id: String,
    pub prompt_version_id: String,
    pub prompt_version: i64,
    pub prompt_version_sha256: String,
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn bounded(value: &str, max: usize, code: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("{code}_EMPTY"));
    }
    if trimmed.as_bytes().len() > max {
        return Err(format!("{code}_TOO_LARGE"));
    }
    Ok(trimmed.to_string())
}

fn truncate_utf8(value: &str, max: usize) -> String {
    if value.len() <= max {
        return value.to_string();
    }
    let mut end = max;
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_string()
}

fn validate_active_project(
    database: &DatabaseState,
    project_id: &str,
) -> Result<crate::projects::ProjectRecord, String> {
    let project = fetch_project(database, project_id)?;
    if project.status != "ACTIVE" {
        return Err("PROMPT_PROJECT_NOT_ACTIVE".into());
    }
    if project.normalized_path.trim().is_empty() {
        return Err("PROMPT_PROJECT_PATH_UNAVAILABLE".into());
    }
    Ok(project)
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
    let owner: Option<String> = connection
        .query_row(
            "SELECT project_id FROM tasks WHERE id=?1",
            [task_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    match owner {
        Some(owner) if owner == project_id => Ok(()),
        Some(_) => Err("PROMPT_TASK_PROJECT_MISMATCH".into()),
        None => Err("PROMPT_TASK_NOT_FOUND".into()),
    }
}

fn context_item(
    class: &str,
    reference: String,
    value: Option<String>,
    disposition: ContextDisposition,
    reason: Option<String>,
) -> ContextItem {
    let bytes = value.as_deref().map(|v| v.len()).unwrap_or(0);
    ContextItem {
        class: class.into(),
        reference,
        disposition,
        bytes,
        reason,
        value,
    }
}

fn task_requirements(metadata: Option<&str>) -> Option<serde_json::Value> {
    let root: serde_json::Value = serde_json::from_str(metadata?).ok()?;
    let task = root.get("task")?;
    let mut selected = serde_json::Map::new();
    for key in [
        "nextStep",
        "ownerGate",
        "externalWait",
        "acceptanceCriteria",
        "blockers",
        "dependencyReferences",
    ] {
        if let Some(value) = task.get(key) {
            selected.insert(key.to_string(), value.clone());
        }
    }
    (!selected.is_empty()).then_some(serde_json::Value::Object(selected))
}

fn safe_source_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    !lower.contains("..")
        && !lower.starts_with('/')
        && !lower.contains(':')
        && !lower.contains(".env")
        && !lower.contains("hiveai.db")
        && !lower.contains("credential")
        && !lower.contains("cache")
        && !lower.contains("target")
}

pub fn collect_context(
    database: &DatabaseState,
    request: PromptContextRequest,
) -> Result<ContextManifest, String> {
    let project = validate_active_project(database, &request.project_id)?;
    validate_task(database, &request.project_id, request.task_id.as_deref())?;
    let connection = database.open_connection()?;
    let mut items = Vec::new();
    let project_value = serde_json::to_string(&json!({"id":project.id,"name":project.name,"status":project.status,"path":project.normalized_path,"branch":project.repository.as_ref().and_then(|r|r.current_branch.clone()),"headSha":project.repository.as_ref().and_then(|r|r.head_sha.clone())})).unwrap();
    items.push(context_item(
        "PROJECT_IDENTITY",
        format!("project:{}", project.id),
        Some(project_value),
        ContextDisposition::Included,
        None,
    ));
    if let Some(task_id) = request.task_id.as_deref() {
        let task: Option<(String, String, String, Option<String>, Option<String>, Option<String>)> = connection.query_row("SELECT title,state,required_actor,milestone,metadata_json,updated_at FROM tasks WHERE id=?1 AND project_id=?2", params![task_id, request.project_id], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?))).optional().map_err(|e| e.to_string())?;
        if let Some((title, state, actor, milestone, metadata, updated)) = task {
            let value = serde_json::to_string(&json!({"id":task_id,"title":title,"state":state,"requiredActor":actor,"milestone":milestone,"updatedAt":updated,"requirements":task_requirements(metadata.as_deref())})).unwrap();
            items.push(context_item(
                "TASK",
                format!("task:{}", task_id),
                Some(value),
                ContextDisposition::Included,
                None,
            ));
            let mut dependencies = connection.prepare("SELECT depends_on_task_id,dependency_kind FROM task_dependencies WHERE task_id=?1 ORDER BY depends_on_task_id,dependency_kind LIMIT 32").map_err(|e| e.to_string())?;
            let deps: Vec<String> = dependencies
                .query_map([task_id], |r| {
                    Ok(format!(
                        "{}:{}",
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?
                    ))
                })
                .map_err(|e| e.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;
            items.push(context_item(
                "TASK_DEPENDENCIES",
                format!("task-dependencies:{}", task_id),
                Some(serde_json::to_string(&deps).unwrap()),
                ContextDisposition::Included,
                None,
            ));
        }
    }
    let mut sources = connection.prepare("SELECT source_path,source_kind,content_hash,metadata_json FROM project_sources WHERE project_id=?1 ORDER BY source_path,source_kind LIMIT ?2").map_err(|e| e.to_string())?;
    let source_rows = sources
        .query_map(
            params![request.project_id, MAX_CONTEXT_SOURCES as i64],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, Option<String>>(3)?,
                ))
            },
        )
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let source_total: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM project_sources WHERE project_id=?1",
            [&request.project_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    for (path, kind, hash, _metadata) in source_rows {
        let disposition = if safe_source_path(&path) {
            ContextDisposition::Included
        } else {
            ContextDisposition::Excluded
        };
        let value = if matches!(disposition, ContextDisposition::Included) {
            Some(serde_json::to_string(&json!({"path":path,"kind":kind,"hash":hash})).unwrap())
        } else {
            None
        };
        let reason = if value.is_none() {
            Some("path is outside the bounded source allowlist or is sensitive/build data".into())
        } else {
            None
        };
        items.push(context_item(
            "M08_SOURCE_REFERENCE",
            format!("source:{path}"),
            value,
            disposition,
            reason,
        ));
    }
    if source_total > MAX_CONTEXT_SOURCES as i64 {
        items.push(context_item(
            "M08_SOURCE_REFERENCE",
            "source-bound".into(),
            None,
            ContextDisposition::Omitted,
            Some(format!(
                "{} source references omitted after the {}-source bound",
                source_total - MAX_CONTEXT_SOURCES as i64,
                MAX_CONTEXT_SOURCES
            )),
        ));
    }
    let dashboard = project_dashboard::resolve(database, &request.project_id)?;
    let dashboard_value = serde_json::to_string(&json!({"manifestStatus":dashboard.manifest_status,"taskAuthority":dashboard.task_authority,"provenanceMode":dashboard.provenance_mode,"roles":dashboard.roles,"warnings":dashboard.warnings})).unwrap();
    items.push(context_item(
        "PROJECT_DASHBOARD",
        format!("dashboard:{}", request.project_id),
        Some(dashboard_value),
        ContextDisposition::Included,
        None,
    ));
    let mut findings = connection.prepare("SELECT f.id,f.severity,f.title,f.detail,f.file_path,f.line_number FROM audit_findings f JOIN audits a ON a.id=f.audit_id WHERE a.project_id=?1 AND (?2 IS NULL OR a.task_id=?2) ORDER BY f.id LIMIT 32").map_err(|e| e.to_string())?;
    let finding_rows = findings
        .query_map(params![request.project_id, request.task_id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<String>>(3)?,
                r.get::<_, Option<String>>(4)?,
                r.get::<_, Option<i64>>(5)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    for (id, severity, title, detail, file, line) in finding_rows {
        let value = serde_json::to_string(&json!({"id":id,"severity":severity,"title":title,"detail":detail,"file":file,"line":line})).unwrap();
        items.push(context_item(
            "AUDIT_FINDING",
            format!("finding:{id}"),
            Some(value),
            ContextDisposition::Included,
            None,
        ));
    }
    let mut tests = connection.prepare("SELECT id,command,result,started_at,finished_at FROM test_runs WHERE project_id=?1 AND (?2 IS NULL OR task_id=?2) ORDER BY started_at DESC,id DESC LIMIT 8").map_err(|e| e.to_string())?;
    let test_rows = tests
        .query_map(params![request.project_id, request.task_id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, Option<String>>(4)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    for (id, command, result, started, finished) in test_rows {
        let value = serde_json::to_string(&json!({"id":id,"command":command,"result":result,"startedAt":started,"finishedAt":finished})).unwrap();
        items.push(context_item(
            "TEST_EVIDENCE",
            format!("test:{id}"),
            Some(value),
            ContextDisposition::Included,
            None,
        ));
    }
    items.push(context_item("EXCLUSION_POLICY", "filesystem-and-secrets".into(), None, ContextDisposition::Excluded, Some("arbitrary files, .env, credentials, local databases, build caches, and unapproved paths are never loaded".into())));
    items.sort_by(|left, right| {
        (&left.class, &left.reference).cmp(&(&right.class, &right.reference))
    });
    let mut included_bytes = 0;
    let mut omitted_count = 0;
    let mut included_items = 0;
    let mut unique_sources = BTreeSet::new();
    for item in &mut items {
        if matches!(item.disposition, ContextDisposition::Included) {
            if included_items >= MAX_CONTEXT_ITEMS
                || included_bytes + item.bytes > MAX_CONTEXT_BYTES
            {
                item.disposition = if item.value.is_some() {
                    ContextDisposition::Truncated
                } else {
                    ContextDisposition::Omitted
                };
                item.reason = Some("context item or byte bound reached".into());
                item.value = None;
                item.bytes = 0;
                omitted_count += 1;
            } else {
                included_bytes += item.bytes;
                included_items += 1;
                if item.class.contains("SOURCE") {
                    unique_sources.insert(item.reference.clone());
                }
            }
        } else {
            omitted_count += 1;
        }
    }
    let mut manifest = ContextManifest {
        project_id: request.project_id,
        task_id: request.task_id,
        items,
        included_bytes,
        omitted_count,
        source_count: unique_sources.len(),
        manifest_sha256: String::new(),
    };
    let canonical = serde_json::to_vec(&json!({"projectId":manifest.project_id,"taskId":manifest.task_id,"items":manifest.items,"includedBytes":manifest.included_bytes,"omittedCount":manifest.omitted_count,"sourceCount":manifest.source_count})).unwrap();
    manifest.manifest_sha256 = hash_bytes(&canonical);
    Ok(manifest)
}

fn render_context(context: &ContextManifest) -> String {
    let mut rendered = String::from("Materialized bounded context\n");
    for item in &context.items {
        match item.disposition {
            ContextDisposition::Included => {
                rendered.push_str(&format!("\n### {} [{}]\n", item.class, item.reference));
                if let Some(value) = &item.value {
                    rendered.push_str(value);
                    rendered.push('\n');
                }
            }
            ref disposition => {
                rendered.push_str(&format!(
                    "\n- {} [{}]: {}{}\n",
                    item.class,
                    item.reference,
                    serde_json::to_string(disposition).unwrap_or_else(|_| "UNKNOWN".into()),
                    item.reason
                        .as_deref()
                        .map(|reason| format!(" ({reason})"))
                        .unwrap_or_default()
                ));
            }
        }
    }
    rendered.push_str(&format!(
        "\nManifest: {}\nIncluded bytes: {}\nIncluded items: {}\nSource references: {}\n",
        context.manifest_sha256,
        context.included_bytes,
        context
            .items
            .iter()
            .filter(|item| matches!(item.disposition, ContextDisposition::Included))
            .count(),
        context.source_count
    ));
    rendered
}

fn bounded_body(body: String) -> String {
    if body.len() <= MAX_PROMPT_BODY_BYTES {
        return body;
    }
    const MARKER: &str =
        "\n\n[TRUNCATED: prompt body reached the bounded 65536-byte limit; see context manifest.]";
    let prefix = truncate_utf8(&body, MAX_PROMPT_BODY_BYTES - MARKER.len());
    format!("{prefix}{MARKER}")
}

fn implementation_body(request: &PromptGenerateRequest, context: &ContextManifest) -> String {
    bounded_body(format!("M15 IMPLEMENTATION PROMPT\n\nGoal\n{summary}\n\nProject and task\n- Project: {project}\n- Task: {task}\n\n{materialized}\n\nExecution contract\n1. Implement the requested behavior using the existing H!veAI architecture and authorities.\n2. Preserve exact project/task identity, local-first process policy, secret-safe persistence, and immutable evidence.\n3. Add focused tests for the requested behavior and keep the full regression green.\n4. Record truthful evidence and do not claim manual or audit acceptance.\n\nScope boundary\n{title}\n{summary}\n\nProhibited shortcuts\n- Do not scrape arbitrary files, load secrets, bypass the registry, expose shell/PID/argv control, redesign unrelated UI, or start a later milestone.\n", summary=request.summary, project=request.project_id, task=request.task_id.as_deref().unwrap_or("freeform project operation"), materialized=render_context(context), title=request.title))
}

fn remediation_body(
    request: &PromptGenerateRequest,
    context: &ContextManifest,
) -> Result<String, String> {
    let findings: Vec<&ContextItem> = context
        .items
        .iter()
        .filter(|item| {
            item.class == "AUDIT_FINDING"
                && matches!(item.disposition, ContextDisposition::Included)
        })
        .collect();
    if findings.is_empty() {
        return Err("PROMPT_REMEDIATION_FINDINGS_UNAVAILABLE".into());
    }
    let mut body = format!(
        "M15 REMEDIATION PROMPT\n\nScope\n{}\n{}\n\n{}\n\nSelected persisted findings\n",
        request.title,
        request.summary,
        render_context(context)
    );
    for finding in findings {
        body.push_str(&format!(
            "- {}\n",
            finding.value.as_deref().unwrap_or("unavailable")
        ));
    }
    body.push_str("\nRequired closure\n- Fix only the persisted findings above.\n- Add focused regression tests that reproduce the observed behavior.\n- Preserve M04-M14 security, process, lifecycle, and UI boundaries.\n- Do not invent findings or expand into another milestone.\n");
    Ok(bounded_body(body))
}

fn audit_support_body(request: &PromptGenerateRequest, context: &ContextManifest) -> String {
    bounded_body(format!(
        "M15 AUDIT-SUPPORT PROMPT\n\nAudit target and scope\n- Target: {}\n- Project: {}\n- Task: {}\n\n{}\n\nExpected verification output\n1. Identify the exact evidence references inspected.\n2. Report PASS, CONDITIONAL, or FAIL with bounded reasons.\n3. List reproduction and closure evidence for each finding.\n4. Mark missing or unavailable evidence explicitly.\n\nMutation boundary\n- Do not mutate files, projects, prompts, sessions, or provider state unless this prompt is explicitly authorized for that action.\n- Do not invent findings or treat builder claims as independent acceptance.\n",
        request.title,
        request.project_id,
        request.task_id.as_deref().unwrap_or("freeform project operation"),
        render_context(context)
    ))
}

pub fn generate(
    database: &DatabaseState,
    request: PromptGenerateRequest,
) -> Result<PromptVersion, String> {
    let title = bounded(&request.title, 512, "PROMPT_TITLE")?;
    let summary = bounded(&request.summary, 4096, "PROMPT_SUMMARY")?;
    validate_active_project(database, &request.project_id)?;
    validate_task(database, &request.project_id, request.task_id.as_deref())?;
    let context = collect_context(
        database,
        PromptContextRequest {
            project_id: request.project_id.clone(),
            task_id: request.task_id.clone(),
        },
    )?;
    let body_request = PromptGenerateRequest {
        title,
        summary,
        ..request
    };
    let selected_findings = body_request.finding_ids.clone();
    let mut context = context;
    if let Some(selected) = selected_findings.as_ref() {
        let available: BTreeSet<String> = context
            .items
            .iter()
            .filter(|item| item.class == "AUDIT_FINDING")
            .map(|item| item.reference.trim_start_matches("finding:").to_string())
            .collect();
        if selected.iter().any(|id| !available.contains(id)) {
            return Err("PROMPT_FINDING_NOT_FOUND_OR_OUT_OF_SCOPE".into());
        }
        context.items.retain(|item| {
            item.class != "AUDIT_FINDING"
                || selected
                    .iter()
                    .any(|id| item.reference == format!("finding:{id}"))
        });
        context.manifest_sha256 = hash_bytes(&serde_json::to_vec(&json!({"projectId":context.project_id,"taskId":context.task_id,"items":context.items,"includedBytes":context.included_bytes,"omittedCount":context.omitted_count,"sourceCount":context.source_count})).unwrap());
    }
    let body = match body_request.kind {
        PromptKind::Remediation => remediation_body(&body_request, &context)?,
        PromptKind::AuditSupport => audit_support_body(&body_request, &context),
        PromptKind::Implementation => implementation_body(&body_request, &context),
    };
    let now = utc_timestamp();
    let prompt_id = Uuid::new_v4().to_string();
    let version_id = Uuid::new_v4().to_string();
    let provenance = json!({"projectId":body_request.project_id,"taskId":body_request.task_id,"kind":body_request.kind,"origin":"M15_GENERATOR","contextManifestSha256":context.manifest_sha256,"findingIds":body_request.finding_ids.unwrap_or_default()});
    let connection = database.open_connection()?;
    connection.execute("INSERT INTO prompts (id,project_id,task_id,kind,current_version,created_at,updated_at) VALUES (?1,?2,?3,?4,1,?5,?5)", params![prompt_id, body_request.project_id, body_request.task_id, body_request.kind.as_str(), now]).map_err(|e| e.to_string())?;
    connection.execute("INSERT INTO prompt_versions (id,prompt_id,version,content,created_by,created_at,title,summary,origin,context_manifest_json,provenance_json,approval_state) VALUES (?1,?2,1,?3,'M15_PROMPT_ENGINE',?4,?5,?6,'M15_GENERATOR',?7,?8,'DRAFT')", params![version_id,prompt_id,body,now,body_request.title,body_request.summary,serde_json::to_string(&context).unwrap(),serde_json::to_string(&provenance).unwrap()]).map_err(|e| e.to_string())?;
    read_version(database, &prompt_id, &version_id)
}

fn parse_context(value: Option<String>) -> Option<ContextManifest> {
    value.and_then(|raw| serde_json::from_str(&raw).ok())
}
fn parse_json(value: Option<String>) -> serde_json::Value {
    value
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_else(|| json!({}))
}

fn read_version(
    database: &DatabaseState,
    prompt_id: &str,
    version_id: &str,
) -> Result<PromptVersion, String> {
    let connection = database.open_connection()?;
    let row = connection.query_row("SELECT v.id,v.prompt_id,v.version,p.kind,v.title,v.summary,v.content,v.created_by,v.created_at,v.origin,v.context_manifest_json,v.provenance_json,v.approval_state,v.approved_at,v.approved_body_sha256,v.used_at,v.selected_provider,v.dispatched_session_id,v.superseded_at,v.dispatch_state,v.dispatch_reservation_id,v.dispatch_reserved_at,v.dispatch_provenance_json,v.dispatch_error,p.current_version FROM prompt_versions v JOIN prompts p ON p.id=v.prompt_id WHERE v.prompt_id=?1 AND v.id=?2", params![prompt_id,version_id], |r| Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,i64>(2)?,r.get::<_,String>(3)?,r.get::<_,Option<String>>(4)?,r.get::<_,Option<String>>(5)?,r.get::<_,String>(6)?,r.get::<_,String>(7)?,r.get::<_,String>(8)?,r.get::<_,String>(9)?,r.get::<_,Option<String>>(10)?,r.get::<_,Option<String>>(11)?,r.get::<_,String>(12)?,r.get::<_,Option<String>>(13)?,r.get::<_,Option<String>>(14)?,r.get::<_,Option<String>>(15)?,r.get::<_,Option<String>>(16)?,r.get::<_,Option<String>>(17)?,r.get::<_,Option<String>>(18)?,r.get::<_,String>(19)?,r.get::<_,Option<String>>(20)?,r.get::<_,Option<String>>(21)?,r.get::<_,Option<String>>(22)?,r.get::<_,Option<String>>(23)?,r.get::<_,i64>(24)?))).optional().map_err(|e| e.to_string())?.ok_or("PROMPT_VERSION_NOT_FOUND")?;
    Ok(PromptVersion {
        id: row.0,
        prompt_id: row.1,
        version: row.2,
        kind: row.3,
        title: row.4,
        summary: row.5,
        body_sha256: hash_bytes(row.6.as_bytes()),
        content: row.6,
        created_by: row.7,
        created_at: row.8,
        origin: row.9,
        context_manifest: parse_context(row.10),
        provenance: parse_json(row.11),
        approval_state: row.12,
        approved_at: row.13,
        approved_body_sha256: row.14,
        used_at: row.15,
        selected_provider: row.16,
        dispatched_session_id: row.17,
        superseded_at: row.18,
        dispatch_state: row.19,
        dispatch_reservation_id: row.20,
        dispatch_reserved_at: row.21,
        dispatch_provenance: parse_json(row.22),
        dispatch_error: row.23,
        is_current: row.2 == row.24,
    })
}

pub fn list(database: &DatabaseState, project_id: String) -> Result<Vec<PromptRecord>, String> {
    validate_active_project(database, &project_id)?;
    let connection = database.open_connection()?;
    let mut statement = connection.prepare("SELECT id,project_id,task_id,kind,current_version,created_at,updated_at FROM prompts WHERE project_id=?1 ORDER BY updated_at DESC,id DESC LIMIT 100").map_err(|e| e.to_string())?;
    let rows = statement
        .query_map([project_id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, Option<String>>(1)?,
                r.get::<_, Option<String>>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, i64>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    rows.into_iter()
        .map(|row| {
            let current_id: String = connection
                .query_row(
                    "SELECT id FROM prompt_versions WHERE prompt_id=?1 AND version=?2",
                    params![row.0, row.4],
                    |r| r.get(0),
                )
                .map_err(|e| e.to_string())?;
            Ok(PromptRecord {
                id: row.0.clone(),
                project_id: row.1,
                task_id: row.2,
                kind: row.3,
                current_version: row.4,
                created_at: row.5,
                updated_at: row.6,
                current: Some(read_version(database, &row.0, &current_id)?),
            })
        })
        .collect()
}

pub fn versions(database: &DatabaseState, prompt_id: String) -> Result<Vec<PromptVersion>, String> {
    let connection = database.open_connection()?;
    let mut statement = connection
        .prepare(
            "SELECT id FROM prompt_versions WHERE prompt_id=?1 ORDER BY version DESC LIMIT 100",
        )
        .map_err(|e| e.to_string())?;
    let ids = statement
        .query_map([&prompt_id], |r| r.get::<_, String>(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    ids.into_iter()
        .map(|id| read_version(database, &prompt_id, &id))
        .collect()
}

pub fn versions_for_project(
    database: &DatabaseState,
    project_id: String,
    prompt_id: String,
) -> Result<Vec<PromptVersion>, String> {
    validate_prompt_owner(database, &project_id, &prompt_id)?;
    versions(database, prompt_id)
}

fn validate_prompt_owner(
    database: &DatabaseState,
    project_id: &str,
    prompt_id: &str,
) -> Result<(), String> {
    validate_active_project(database, project_id)?;
    let connection = database.open_connection()?;
    let owner: Option<String> = connection
        .query_row(
            "SELECT project_id FROM prompts WHERE id=?1",
            [prompt_id],
            |r| r.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    match owner {
        Some(owner) if owner == project_id => Ok(()),
        Some(_) => Err("PROMPT_PROJECT_MISMATCH".into()),
        None => Err("PROMPT_NOT_FOUND".into()),
    }
}

pub fn edit(database: &DatabaseState, request: PromptEditRequest) -> Result<PromptVersion, String> {
    validate_prompt_owner(database, &request.project_id, &request.prompt_id)?;
    let content = bounded(&request.content, MAX_PROMPT_BODY_BYTES, "PROMPT_BODY")?;
    let old = read_version(database, &request.prompt_id, &request.version_id)?;
    if old.approval_state != PromptApprovalState::Draft.as_str()
        || old.used_at.is_some()
        || old.dispatched_session_id.is_some()
    {
        let connection = database.open_connection()?;
        let next: i64 = connection
            .query_row(
                "SELECT COALESCE(MAX(version),0)+1 FROM prompt_versions WHERE prompt_id=?1",
                [&request.prompt_id],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;
        let id = Uuid::new_v4().to_string();
        let now = utc_timestamp();
        connection.execute("INSERT INTO prompt_versions (id,prompt_id,version,content,created_by,created_at,title,summary,origin,context_manifest_json,provenance_json,approval_state) SELECT ?1,prompt_id,?2,?3,'HUMAN_EDIT',?4,COALESCE(?5,title),COALESCE(?6,summary),'HUMAN_EDIT',context_manifest_json,json_set(COALESCE(provenance_json,'{}'),'$.userEdit',json(?7)),'DRAFT' FROM prompt_versions WHERE id=?8", params![id,next,content,now,request.title,request.summary,serde_json::to_string(&json!({"sourceVersionId":old.id})).unwrap(),request.version_id]).map_err(|e| e.to_string())?;
        connection
            .execute(
                "UPDATE prompts SET current_version=?2,updated_at=?3 WHERE id=?1",
                params![request.prompt_id, next, now],
            )
            .map_err(|e| e.to_string())?;
        return read_version(database, &request.prompt_id, &id);
    }
    let connection = database.open_connection()?;
    connection.execute("UPDATE prompt_versions SET content=?2,title=COALESCE(?3,title),summary=COALESCE(?4,summary) WHERE id=?1 AND approval_state='DRAFT' AND used_at IS NULL", params![request.version_id,content,request.title,request.summary]).map_err(|e| e.to_string())?;
    read_version(database, &request.prompt_id, &request.version_id)
}

pub fn approve(
    database: &DatabaseState,
    request: PromptApproveRequest,
) -> Result<PromptVersion, String> {
    validate_prompt_owner(database, &request.project_id, &request.prompt_id)?;
    let version = read_version(database, &request.prompt_id, &request.version_id)?;
    if version.approval_state != PromptApprovalState::Draft.as_str() {
        return Err("PROMPT_VERSION_NOT_EDITABLE".into());
    }
    let hash = hash_bytes(version.content.as_bytes());
    let now = utc_timestamp();
    let connection = database.open_connection()?;
    connection.execute("UPDATE prompt_versions SET approval_state='APPROVED',approved_at=?2,approved_body_sha256=?3 WHERE id=?1 AND approval_state='DRAFT'", params![request.version_id,now,hash]).map_err(|e| e.to_string())?;
    read_version(database, &request.prompt_id, &request.version_id)
}

#[derive(Debug, Clone)]
struct DispatchReservation {
    reservation_id: String,
    prompt_id: String,
    version_id: String,
    version: i64,
    content: String,
    hash: String,
    provider: String,
    project_id: String,
    task_id: Option<String>,
}

fn reserve_dispatch(
    database: &DatabaseState,
    request: &PromptDispatchRequest,
) -> Result<DispatchReservation, String> {
    let provider = request.provider.trim().to_ascii_uppercase();
    if provider != "CODEX" && provider != "CLAUDE" {
        return Err("PROMPT_PROVIDER_UNSUPPORTED".into());
    }
    validate_prompt_owner(database, &request.project_id, &request.prompt_id)?;
    let version = read_version(database, &request.prompt_id, &request.version_id)?;
    if version.approval_state != PromptApprovalState::Approved.as_str() {
        return Err("PROMPT_DISPATCH_REQUIRES_APPROVAL_OR_SINGLE_USE_CLAIM".into());
    }
    let hash = hash_bytes(version.content.as_bytes());
    if version.approved_body_sha256.as_deref() != Some(hash.as_str()) {
        return Err("PROMPT_APPROVAL_HASH_MISMATCH".into());
    }
    let connection = database.open_connection()?;
    let (project_id, task_id): (String, Option<String>) = connection
        .query_row(
            "SELECT project_id,task_id FROM prompts WHERE id=?1",
            [&request.prompt_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|e| e.to_string())?;
    if project_id != request.project_id {
        return Err("PROMPT_PROJECT_MISMATCH".into());
    }
    validate_active_project(database, &project_id)?;
    validate_task(database, &project_id, task_id.as_deref())?;

    let reservation_id = Uuid::new_v4().to_string();
    let now = utc_timestamp();
    let provenance = json!({
        "dispatchReservationId": reservation_id,
        "promptId": request.prompt_id,
        "promptVersionId": request.version_id,
        "promptVersion": version.version,
        "promptVersionSha256": hash,
        "provider": provider,
        "projectId": project_id,
        "taskId": task_id,
    });
    connection
        .execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| format!("PROMPT_DISPATCH_CLAIM_DATABASE_FAILURE: {e}"))?;
    let changed = connection.execute(
        "UPDATE prompt_versions SET approval_state='DISPATCHING',dispatch_state='RESERVED',dispatch_reservation_id=?1,dispatch_reserved_at=?2,dispatch_provenance_json=?3,dispatch_error=NULL WHERE id=?4 AND prompt_id=?5 AND approval_state='APPROVED' AND used_at IS NULL AND approved_body_sha256=?6 AND content=?7 AND COALESCE(dispatch_state,'AVAILABLE')='AVAILABLE'",
        params![reservation_id, now, provenance.to_string(), request.version_id, request.prompt_id, hash, version.content],
    );
    let changed = match changed {
        Ok(value) => value,
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK");
            return Err(format!("PROMPT_DISPATCH_CLAIM_DATABASE_FAILURE: {error}"));
        }
    };
    if changed != 1 {
        let _ = connection.execute_batch("ROLLBACK");
        return Err("PROMPT_DISPATCH_ALREADY_CLAIMED_OR_NOT_APPROVED".into());
    }
    connection
        .execute_batch("COMMIT")
        .map_err(|e| format!("PROMPT_DISPATCH_CLAIM_COMMIT_FAILURE: {e}"))?;
    Ok(DispatchReservation {
        reservation_id,
        prompt_id: request.prompt_id.clone(),
        version_id: request.version_id.clone(),
        version: version.version,
        content: version.content,
        hash,
        provider,
        project_id,
        task_id,
    })
}

fn mark_dispatch_failed(
    database: &DatabaseState,
    reservation: &DispatchReservation,
    error: &str,
) -> Result<(), String> {
    let connection = database.open_connection()?;
    let changed = connection
        .execute(
            "UPDATE prompt_versions SET approval_state='DISPATCH_FAILED',dispatch_state='FAILED',dispatch_error=?1 WHERE id=?2 AND dispatch_reservation_id=?3 AND dispatch_state='RESERVED'",
            params![truncate_utf8(error, 2048), reservation.version_id, reservation.reservation_id],
        )
        .map_err(|e| format!("persist dispatch failure: {e}"))?;
    if changed != 1 {
        return Err("PROMPT_DISPATCH_FAILURE_STATE_NOT_PERSISTED".into());
    }
    Ok(())
}

fn finalize_dispatch(
    database: &DatabaseState,
    reservation: &DispatchReservation,
    session_id: &str,
) -> Result<(), String> {
    let now = utc_timestamp();
    let provenance = json!({
        "dispatchReservationId": reservation.reservation_id,
        "promptId": reservation.prompt_id,
        "promptVersionId": reservation.version_id,
        "promptVersion": reservation.version,
        "promptVersionSha256": reservation.hash,
        "provider": reservation.provider,
        "projectId": reservation.project_id,
        "taskId": reservation.task_id,
        "sessionId": session_id,
    });
    let connection = database.open_connection()?;
    connection
        .execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| format!("PROMPT_DISPATCH_FINALIZE_DATABASE_FAILURE: {e}"))?;
    let session_changed = connection.execute(
        "UPDATE agent_sessions SET prompt_id=?1,prompt_version_id=?2,prompt_version=?3,prompt_version_sha256=?4 WHERE id=?5 AND project_id=?6 AND provider=?7",
        params![reservation.prompt_id, reservation.version_id, reservation.version, reservation.hash, session_id, reservation.project_id, reservation.provider],
    );
    let session_changed = match session_changed {
        Ok(value) => value,
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK");
            return Err(format!("persist session prompt provenance: {error}"));
        }
    };
    if session_changed != 1 {
        let _ = connection.execute_batch("ROLLBACK");
        return Err("PROMPT_DISPATCH_SESSION_PROVENANCE_NOT_PERSISTED".into());
    }
    let prompt_changed = connection.execute(
        "UPDATE prompt_versions SET approval_state='DISPATCHED',dispatch_state='DISPATCHED',used_at=?1,selected_provider=?2,dispatched_session_id=?3,dispatch_provenance_json=?4,dispatch_error=NULL WHERE id=?5 AND prompt_id=?6 AND dispatch_reservation_id=?7 AND dispatch_state='RESERVED' AND approved_body_sha256=?8 AND content=?9",
        params![now, reservation.provider, session_id, provenance.to_string(), reservation.version_id, reservation.prompt_id, reservation.reservation_id, reservation.hash, reservation.content],
    );
    let prompt_changed = match prompt_changed {
        Ok(value) => value,
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK");
            return Err(format!("persist prompt dispatch provenance: {error}"));
        }
    };
    if prompt_changed != 1 {
        let _ = connection.execute_batch("ROLLBACK");
        return Err("PROMPT_DISPATCH_RESERVATION_FINALIZE_NOT_PERSISTED".into());
    }
    connection
        .execute_batch("COMMIT")
        .map_err(|e| format!("PROMPT_DISPATCH_FINALIZE_COMMIT_FAILURE: {e}"))
}

pub fn dispatch(
    center: &AgentSessionCenter,
    codex: &CodexAdapter,
    database: &DatabaseState,
    request: PromptDispatchRequest,
) -> Result<PromptDispatchResult, String> {
    let reservation = reserve_dispatch(database, &request)?;
    let mut session = match agent_session_center::start(
        center,
        codex,
        database,
        AgentStartRequest {
            provider: reservation.provider.clone(),
            project_id: reservation.project_id.clone(),
            task_id: reservation.task_id.clone(),
            prompt: reservation.content.clone(),
        },
    ) {
        Ok(session) => session,
        Err(error) => {
            mark_dispatch_failed(database, &reservation, &error)?;
            return Err(format!("PROMPT_DISPATCH_PROVIDER_START_FAILED: {error}"));
        }
    };
    if let Err(error) = finalize_dispatch(database, &reservation, &session.id) {
        let _ = agent_session_center::stop(
            center,
            codex,
            database,
            &reservation.project_id,
            &session.id,
        );
        mark_dispatch_failed(database, &reservation, &error)?;
        return Err(format!("PROMPT_DISPATCH_PROVENANCE_FAILED: {error}"));
    }
    session.prompt_id = Some(reservation.prompt_id.clone());
    session.prompt_version_id = Some(reservation.version_id.clone());
    session.prompt_version = Some(reservation.version);
    session.prompt_version_sha256 = Some(reservation.hash.clone());
    let prompt = read_version(database, &reservation.prompt_id, &reservation.version_id)?;
    Ok(PromptDispatchResult {
        prompt,
        session,
        prompt_id: reservation.prompt_id,
        prompt_version_id: reservation.version_id,
        prompt_version: reservation.version,
        prompt_version_sha256: reservation.hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use std::sync::{Arc, Barrier};
    use std::thread;
    use tempfile::tempdir;

    #[test]
    fn prompt_kinds_are_explicit_and_hash_is_deterministic() {
        assert_eq!(
            PromptKind::parse("implementation").unwrap().as_str(),
            "IMPLEMENTATION"
        );
        assert_eq!(hash_bytes(b"same"), hash_bytes(b"same"));
        assert!(PromptKind::parse("SHELL").is_err());
    }

    #[test]
    fn context_path_filter_excludes_sensitive_and_build_paths() {
        assert!(safe_source_path("TASKS.md"));
        assert!(!safe_source_path(".env"));
        assert!(!safe_source_path("target/debug/app"));
        assert!(!safe_source_path("../outside.md"));
    }

    #[test]
    fn context_bounds_are_constants_and_body_hash_changes_on_edit() {
        assert!(MAX_CONTEXT_BYTES >= 64 * 1024);
        assert!(MAX_CONTEXT_ITEMS <= 64);
        assert_ne!(hash_bytes(b"one"), hash_bytes(b"two"));
        let _ = tempdir();
    }

    #[test]
    fn prompt_versions_are_project_scoped_and_approved_body_is_not_mutated() {
        let app_data = tempdir().unwrap();
        let project_a = tempdir().unwrap();
        let project_b = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let a = register_project(
            &database,
            RegisterProjectRequest {
                path: project_a.path().to_string_lossy().into(),
                name: Some("A".into()),
            },
        )
        .unwrap();
        let b = register_project(
            &database,
            RegisterProjectRequest {
                path: project_b.path().to_string_lossy().into(),
                name: Some("B".into()),
            },
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO prompts (id,project_id,kind,current_version,created_at,updated_at) VALUES ('prompt-a',?1,'IMPLEMENTATION',1,'now','now')", [&a.id]).unwrap();
        connection.execute("INSERT INTO prompt_versions (id,prompt_id,version,content,created_by,created_at,approval_state) VALUES ('version-a','prompt-a',1,'original','test','now','DRAFT')", []).unwrap();
        drop(connection);
        assert_eq!(
            versions_for_project(&database, b.id.clone(), "prompt-a".into()).unwrap_err(),
            "PROMPT_PROJECT_MISMATCH"
        );
        let approved = approve(
            &database,
            PromptApproveRequest {
                project_id: a.id.clone(),
                prompt_id: "prompt-a".into(),
                version_id: "version-a".into(),
            },
        )
        .unwrap();
        assert_eq!(approved.approval_state, "APPROVED");
        let edited = edit(
            &database,
            PromptEditRequest {
                project_id: a.id.clone(),
                prompt_id: "prompt-a".into(),
                version_id: "version-a".into(),
                content: "edited".into(),
                title: None,
                summary: None,
            },
        )
        .unwrap();
        assert_eq!(edited.version, 2);
        assert_eq!(
            read_version(&database, "prompt-a", "version-a")
                .unwrap()
                .content,
            "original"
        );
        assert_eq!(versions(&database, "prompt-a".into()).unwrap().len(), 2);
    }

    #[test]
    fn generation_covers_implementation_and_selected_remediation_findings() {
        let app_data = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Generation fixture".into()),
            },
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO audits (id,project_id,result,summary,created_at) VALUES ('audit-gen',?1,'FAIL','fixture','now')",
                [&project.id],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audit_findings (id,audit_id,severity,title,detail,created_at) VALUES ('finding-gen','audit-gen','MAJOR','Fixture defect','reproduce me','now')",
                [],
            )
            .unwrap();
        drop(connection);

        let implementation = generate(
            &database,
            PromptGenerateRequest {
                project_id: project.id.clone(),
                task_id: None,
                kind: PromptKind::Implementation,
                title: "Implement fixture".into(),
                summary: "Implement the bounded behavior".into(),
                finding_ids: None,
            },
        )
        .unwrap();
        assert_eq!(implementation.approval_state, "DRAFT");
        assert!(implementation
            .content
            .contains("Implement the bounded behavior"));

        let remediation = generate(
            &database,
            PromptGenerateRequest {
                project_id: project.id,
                task_id: None,
                kind: PromptKind::Remediation,
                title: "Remediate fixture".into(),
                summary: "Close the fixture defect".into(),
                finding_ids: Some(vec!["finding-gen".into()]),
            },
        )
        .unwrap();
        assert!(remediation.content.contains("reproduce me"));
        assert_eq!(
            remediation
                .provenance
                .get("findingIds")
                .and_then(|value| value.as_array())
                .and_then(|values| values.first())
                .and_then(|value| value.as_str()),
            Some("finding-gen")
        );
    }

    #[test]
    fn materialized_renderers_are_distinct_bounded_and_exclusion_safe() {
        let context = ContextManifest {
            project_id: "project-1".into(),
            task_id: Some("task-1".into()),
            items: vec![
                context_item(
                    "TASK",
                    "task:task-1".into(),
                    Some("acceptance and dependency evidence".into()),
                    ContextDisposition::Included,
                    None,
                ),
                context_item(
                    "M08_SOURCE_REFERENCE",
                    "source:TASKS.md".into(),
                    Some("safe source hash".into()),
                    ContextDisposition::Included,
                    None,
                ),
                context_item(
                    "SECRET",
                    "source:.env".into(),
                    Some("SHOULD_NOT_RENDER".into()),
                    ContextDisposition::Excluded,
                    Some("secret exclusion".into()),
                ),
                context_item(
                    "TEST_EVIDENCE",
                    "test:test-1".into(),
                    None,
                    ContextDisposition::Omitted,
                    Some("context bound".into()),
                ),
            ],
            included_bytes: 49,
            omitted_count: 2,
            source_count: 1,
            manifest_sha256: "manifest-1".into(),
        };
        let implementation = implementation_body(
            &PromptGenerateRequest {
                project_id: "project-1".into(),
                task_id: Some("task-1".into()),
                kind: PromptKind::Implementation,
                title: "Implement".into(),
                summary: "Goal".into(),
                finding_ids: None,
            },
            &context,
        );
        let audit = audit_support_body(
            &PromptGenerateRequest {
                project_id: "project-1".into(),
                task_id: Some("task-1".into()),
                kind: PromptKind::AuditSupport,
                title: "Audit".into(),
                summary: "Verify".into(),
                finding_ids: None,
            },
            &context,
        );
        assert!(implementation.contains("acceptance and dependency evidence"));
        assert!(implementation.contains("safe source hash"));
        assert!(implementation.contains("EXCLUDED"));
        assert!(!implementation.contains("SHOULD_NOT_RENDER"));
        assert!(implementation.contains("OMITTED"));
        assert!(audit.contains("AUDIT-SUPPORT PROMPT"));
        assert!(audit.contains("Mutation boundary"));
        assert_ne!(implementation, audit);
        let huge = ContextManifest {
            items: vec![context_item(
                "TASK",
                "task:huge".into(),
                Some("x".repeat(MAX_PROMPT_BODY_BYTES + 100)),
                ContextDisposition::Included,
                None,
            )],
            ..context
        };
        let bounded_prompt = implementation_body(
            &PromptGenerateRequest {
                project_id: "project-1".into(),
                task_id: None,
                kind: PromptKind::Implementation,
                title: "Bound".into(),
                summary: "Bound".into(),
                finding_ids: None,
            },
            &huge,
        );
        assert!(bounded_prompt.len() <= MAX_PROMPT_BODY_BYTES);
        assert!(bounded_prompt.contains("TRUNCATED"));
    }

    fn approved_dispatch_fixture() -> (
        tempfile::TempDir,
        tempfile::TempDir,
        DatabaseState,
        String,
        String,
    ) {
        let app_data = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Dispatch fixture".into()),
            },
        )
        .unwrap();
        let prompt_id = "prompt-dispatch".to_string();
        let version_id = "version-dispatch".to_string();
        let content = "approved dispatch fixture";
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO prompts (id,project_id,kind,current_version,created_at,updated_at) VALUES (?1,?2,'IMPLEMENTATION',1,'now','now')", params![prompt_id,project.id]).unwrap();
        connection.execute("INSERT INTO prompt_versions (id,prompt_id,version,content,created_by,created_at,approval_state,approved_body_sha256) VALUES (?1,?2,1,?3,'test','now','APPROVED',?4)", params![version_id, prompt_id, content, hash_bytes(content.as_bytes())]).unwrap();
        (app_data, project_dir, database, prompt_id, version_id)
    }

    #[test]
    fn durable_dispatch_claim_is_single_use_under_concurrency_and_replay() {
        let (_app_data, _project_dir, database, prompt_id, version_id) =
            approved_dispatch_fixture();
        let project_id: String = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT project_id FROM prompts WHERE id=?1",
                [&prompt_id],
                |row| row.get(0),
            )
            .unwrap();
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let database = database.clone();
            let barrier = barrier.clone();
            let prompt_id = prompt_id.clone();
            let version_id = version_id.clone();
            let project_id = project_id.clone();
            handles.push(thread::spawn(move || {
                barrier.wait();
                reserve_dispatch(
                    &database,
                    &PromptDispatchRequest {
                        project_id,
                        prompt_id,
                        version_id,
                        provider: "CODEX".into(),
                    },
                )
                .is_ok()
            }));
        }
        let successes = handles
            .into_iter()
            .filter_map(|handle| handle.join().ok())
            .filter(|success| *success)
            .count();
        assert_eq!(successes, 1);
        let state: String = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT dispatch_state FROM prompt_versions WHERE id=?1",
                [&version_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(state, "RESERVED");
        let replay = reserve_dispatch(
            &database,
            &PromptDispatchRequest {
                project_id,
                prompt_id,
                version_id,
                provider: "CODEX".into(),
            },
        )
        .unwrap_err();
        assert_eq!(
            replay,
            "PROMPT_DISPATCH_REQUIRES_APPROVAL_OR_SINGLE_USE_CLAIM"
        );
    }

    #[test]
    fn claimed_dispatch_failure_is_durable_and_not_replayable() {
        let (_app_data, _project_dir, database, prompt_id, version_id) =
            approved_dispatch_fixture();
        let project_id: String = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT project_id FROM prompts WHERE id=?1",
                [&prompt_id],
                |row| row.get(0),
            )
            .unwrap();
        let reservation = reserve_dispatch(
            &database,
            &PromptDispatchRequest {
                project_id,
                prompt_id: prompt_id.clone(),
                version_id: version_id.clone(),
                provider: "CLAUDE".into(),
            },
        )
        .unwrap();
        mark_dispatch_failed(&database, &reservation, "provider start fixture failed").unwrap();
        let version = read_version(&database, &prompt_id, &version_id).unwrap();
        assert_eq!(version.approval_state, "DISPATCH_FAILED");
        assert_eq!(version.dispatch_state, "FAILED");
        assert_eq!(
            version.dispatch_error.as_deref(),
            Some("provider start fixture failed")
        );
    }

    #[test]
    fn finalized_dispatch_persists_exact_session_provenance_atomically() {
        let (_app_data, _project_dir, database, prompt_id, version_id) =
            approved_dispatch_fixture();
        let project_id: String = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT project_id FROM prompts WHERE id=?1",
                [&prompt_id],
                |row| row.get(0),
            )
            .unwrap();
        let reservation = reserve_dispatch(
            &database,
            &PromptDispatchRequest {
                project_id: project_id.clone(),
                prompt_id: prompt_id.clone(),
                version_id: version_id.clone(),
                provider: "CODEX".into(),
            },
        )
        .unwrap();
        let session_id = "session-provenance";
        database
            .open_connection()
            .unwrap()
            .execute(
                "INSERT INTO agent_sessions (id,project_id,provider,state,created_at) VALUES (?1,?2,'CODEX','RUNNING','now')",
                params![session_id, project_id],
            )
            .unwrap();

        finalize_dispatch(&database, &reservation, session_id).unwrap();

        let connection = database.open_connection().unwrap();
        let session: (String, String, i64, String) = connection
            .query_row(
                "SELECT prompt_id,prompt_version_id,prompt_version,prompt_version_sha256 FROM agent_sessions WHERE id=?1",
                [session_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();
        assert_eq!(session.0, prompt_id);
        assert_eq!(session.1, version_id);
        assert_eq!(session.2, 1);
        assert_eq!(session.3, reservation.hash);

        let version = read_version(&database, &session.0, &session.1).unwrap();
        assert_eq!(version.approval_state, "DISPATCHED");
        assert_eq!(version.dispatch_state, "DISPATCHED");
        assert_eq!(version.dispatched_session_id.as_deref(), Some(session_id));
        assert_eq!(version.dispatch_provenance["sessionId"], session_id);
        assert_eq!(
            version.dispatch_provenance["promptVersionSha256"],
            reservation.hash
        );
    }

    #[test]
    fn tampered_approval_hash_is_denied_before_provider_launch() {
        let app_data = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Hash fixture".into()),
            },
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO prompts (id,project_id,kind,current_version,created_at,updated_at) VALUES ('prompt-hash',?1,'IMPLEMENTATION',1,'now','now')", [&project.id]).unwrap();
        connection.execute("INSERT INTO prompt_versions (id,prompt_id,version,content,created_by,created_at,approval_state,approved_body_sha256) VALUES ('version-hash','prompt-hash',1,'changed','test','now','APPROVED','not-the-body-hash')", []).unwrap();
        drop(connection);
        let result = dispatch(
            &AgentSessionCenter::default(),
            &CodexAdapter::default(),
            &database,
            PromptDispatchRequest {
                project_id: project.id,
                prompt_id: "prompt-hash".into(),
                version_id: "version-hash".into(),
                provider: "CODEX".into(),
            },
        );
        assert_eq!(result.unwrap_err(), "PROMPT_APPROVAL_HASH_MISMATCH");
    }
}
