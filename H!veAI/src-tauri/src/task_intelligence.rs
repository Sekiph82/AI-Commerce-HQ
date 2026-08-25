use crate::db::DatabaseState;
use crate::projects::fetch_project;
use crate::task_sources::{self, DiscoveredProjectSource, MAX_SOURCE_BYTES};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const MAX_TASKS: usize = 4096;
const MAX_FIELD_BYTES: usize = 4096;
const MAX_ENTRIES: usize = 128;
const MAX_WARNINGS: usize = 512;
const OWNER: &str = "M09_TASK_INTELLIGENCE_PARSER";
const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TaskIntelligenceSnapshot {
    pub project_id: String,
    pub parsed_at: String,
    pub adapter: ParserAdapterIdentity,
    pub tasks: Vec<ParsedTask>,
    pub handoff: Option<HandoffSummary>,
    pub warnings: Vec<ParserWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParsedTask {
    pub id: String,
    pub project_id: String,
    pub source_id: String,
    pub source_path: String,
    pub source_kind: String,
    pub title: String,
    pub parsed_status: String,
    pub storage_state: String,
    pub explicit_task_id: Option<String>,
    pub milestone: Option<String>,
    pub required_actor: Option<String>,
    pub blockers: Vec<String>,
    pub dependency_references: Vec<String>,
    pub next_step: Option<String>,
    pub owner_gate: Option<String>,
    pub external_wait: Option<String>,
    pub acceptance_criteria: Vec<String>,
    pub confidence: TaskConfidence,
    pub evidence: TaskEvidenceLocator,
    pub adapter_id: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TaskEvidenceLocator {
    pub source_path: String,
    pub content_hash: String,
    pub start_line: usize,
    pub end_line: usize,
    pub heading_path: Vec<String>,
    pub locator_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TaskConfidence {
    pub score: f32,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HandoffSummary {
    pub current: Vec<String>,
    pub next: Vec<String>,
    pub blockers: Vec<String>,
    pub waiting: Vec<String>,
    pub evidence: Vec<TaskEvidenceLocator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParserWarning {
    pub code: String,
    pub message: String,
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParserAdapterIdentity {
    pub id: String,
    pub evidence: String,
    pub convention_matched: bool,
}

#[derive(Debug, Clone)]
struct ParsedLineTask {
    line: usize,
    title: String,
    status: String,
    explicit_id: Option<String>,
    checklist: bool,
}

#[derive(Debug, Clone, Default)]
struct Fields {
    blockers: Vec<String>,
    dependencies: Vec<String>,
    next_step: Option<String>,
    owner_gate: Option<String>,
    external_wait: Option<String>,
    acceptance: Vec<String>,
    actor: Option<String>,
    end_line: usize,
}

pub fn parse(
    database: &DatabaseState,
    project_id: &str,
) -> Result<TaskIntelligenceSnapshot, String> {
    let project = fetch_project(database, project_id)?;
    if project.status == "ARCHIVED" {
        return Err("project is archived".into());
    }
    if project.status != "ACTIVE" {
        return Err("registered project root is unavailable".into());
    }
    let sources = task_sources::discover(database, project_id)?;
    let adapter = adapter_for(&project.name);
    let mut snapshot = TaskIntelligenceSnapshot {
        project_id: project_id.to_string(),
        parsed_at: crate::time::utc_timestamp(),
        adapter: adapter.clone(),
        tasks: Vec::new(),
        handoff: None,
        warnings: Vec::new(),
    };
    let mut parsed_sources = Vec::new();
    for source in sources {
        if !is_parser_source(&source) {
            continue;
        }
        match read_authoritative_source(database, project_id, &source) {
            Ok(Some((text, hash))) => {
                let (mut tasks, handoff, warnings) =
                    parse_document(&source, &text, &hash, &adapter);
                snapshot.tasks.append(&mut tasks);
                if snapshot.handoff.is_none() {
                    snapshot.handoff = handoff;
                }
                snapshot.warnings.extend(warnings);
                parsed_sources.push((source, hash));
            }
            Ok(None) => snapshot.warnings.push(ParserWarning {
                code: "SOURCE_CHANGED_DURING_PARSE".into(),
                message: "source changed after one bounded retry; source skipped".into(),
                source_path: Some(source.relative_path),
            }),
            Err(warning) => snapshot.warnings.push(warning),
        }
        trim_warnings(&mut snapshot.warnings);
    }
    snapshot.tasks.truncate(MAX_TASKS);
    resolve_dependencies(&mut snapshot);
    persist(database, project_id, &snapshot, &parsed_sources)?;
    Ok(snapshot)
}

pub fn list(
    database: &DatabaseState,
    project_id: &str,
) -> Result<TaskIntelligenceSnapshot, String> {
    fetch_project(database, project_id)?;
    let connection = database.open_connection()?;
    let json: String = connection
        .query_row(
            "SELECT value_json FROM settings WHERE key = ?1 AND scope = 'PROJECT'",
            [settings_key(project_id)],
            |row| row.get(0),
        )
        .optional()
        .map_err(db_error)?
        .ok_or_else(|| "task intelligence has not been parsed for this project".to_string())?;
    serde_json::from_str(&json).map_err(|error| format!("read task intelligence snapshot: {error}"))
}

fn is_parser_source(source: &DiscoveredProjectSource) -> bool {
    if source.owner != "M08_TASK_SOURCE_DISCOVERY"
        || source.schema_version != 1
        || source.status != "AVAILABLE"
    {
        return false;
    }
    if source.authority_class == "INSTRUCTION"
        || matches!(source.source_kind.as_str(), "AGENTS" | "CLAUDE")
    {
        return false;
    }
    matches!(
        source.source_kind.as_str(),
        "TASKS" | "PLAN" | "PROGRESS" | "ROADMAP" | "HANDOFF" | "CUSTOM" | "OTHER_TASK_SOURCE"
    ) && [".md", ".markdown", ".txt"]
        .iter()
        .any(|suffix| source.relative_path.to_ascii_lowercase().ends_with(suffix))
}

fn read_authoritative_source(
    database: &DatabaseState,
    project_id: &str,
    source: &DiscoveredProjectSource,
) -> Result<Option<(String, String)>, ParserWarning> {
    let project = fetch_project(database, project_id)
        .map_err(|error| warning("SOURCE_READ_FAILED", error, Some(&source.relative_path)))?;
    let root = fs::canonicalize(Path::new(&project.normalized_path)).map_err(|error| {
        warning(
            "SOURCE_READ_FAILED",
            error.to_string(),
            Some(&source.relative_path),
        )
    })?;
    let candidate = root.join(PathBuf::from(&source.relative_path));
    let physical = fs::canonicalize(&candidate).map_err(|error| {
        warning(
            "SOURCE_READ_FAILED",
            error.to_string(),
            Some(&source.relative_path),
        )
    })?;
    if !physical.starts_with(&root) {
        return Err(warning(
            "SOURCE_READ_FAILED",
            "source is outside registered root".into(),
            Some(&source.relative_path),
        ));
    }
    let read = || read_bounded_text(&physical);
    let (text, hash) =
        read().map_err(|(code, message)| warning(&code, message, Some(&source.relative_path)))?;
    if source.content_hash.as_deref() == Some(hash.as_str()) {
        return Ok(Some((text, hash)));
    }
    let refreshed = task_sources::discover(database, project_id)
        .map_err(|error| warning("SOURCE_READ_FAILED", error, Some(&source.relative_path)))?;
    let Some(current) = refreshed
        .into_iter()
        .find(|item| item.relative_path == source.relative_path && item.status == "AVAILABLE")
    else {
        return Ok(None);
    };
    if current.content_hash.as_deref() != source.content_hash.as_deref() {
        return Ok(None);
    }
    let (text, hash) =
        read().map_err(|(code, message)| warning(&code, message, Some(&source.relative_path)))?;
    if current.content_hash.as_deref() == Some(hash.as_str()) {
        Ok(Some((text, hash)))
    } else {
        Ok(None)
    }
}

fn read_bounded_text(path: &Path) -> Result<(String, String), (String, String)> {
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|error| ("SOURCE_READ_FAILED".into(), error.to_string()))?
        .take(MAX_SOURCE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| ("SOURCE_READ_FAILED".into(), error.to_string()))?;
    if bytes.len() as u64 > MAX_SOURCE_BYTES {
        return Err((
            "SOURCE_READ_FAILED".into(),
            "source exceeds M08 size bound".into(),
        ));
    }
    let hash = hash_bytes(&bytes);
    let text = String::from_utf8(bytes)
        .map_err(|_| ("INVALID_UTF8".into(), "source is not valid UTF-8".into()))?;
    Ok((text, hash))
}

fn parse_document(
    source: &DiscoveredProjectSource,
    text: &str,
    hash: &str,
    adapter: &ParserAdapterIdentity,
) -> (Vec<ParsedTask>, Option<HandoffSummary>, Vec<ParserWarning>) {
    let lines = text.lines().collect::<Vec<_>>();
    let mut headings: Vec<(usize, String)> = Vec::new();
    let mut candidates = Vec::new();
    let mut handoff = HandoffSummary {
        current: Vec::new(),
        next: Vec::new(),
        blockers: Vec::new(),
        waiting: Vec::new(),
        evidence: Vec::new(),
    };
    let mut has_handoff = source.source_kind.eq_ignore_ascii_case("HANDOFF");
    for (index, raw) in lines.iter().enumerate() {
        let line = index + 1;
        let trimmed = raw.trim();
        if let Some((level, title)) = heading(trimmed) {
            while headings.last().is_some_and(|(old, _)| *old >= level) {
                headings.pop();
            }
            headings.push((level, title.to_string()));
            has_handoff |= is_handoff_heading(title);
            continue;
        }
        if let Some(task) = task_line(trimmed) {
            candidates.push(ParsedLineTask { line, ..task });
        }
        if has_handoff && !trimmed.is_empty() && !is_task_line(trimmed) {
            let section = headings
                .last()
                .map(|(_, title)| title.to_ascii_lowercase())
                .unwrap_or_default();
            let value = clean_value(trimmed);
            if !value.is_empty() {
                let heading_path = headings
                    .iter()
                    .map(|(_, title)| title.clone())
                    .collect::<Vec<_>>();
                let locator = evidence(source, hash, line, line, &heading_path, None);
                if section.contains("current") || section == "now" {
                    handoff.current.push(value.clone());
                } else if section.contains("next") {
                    handoff.next.push(value.clone());
                } else if section.contains("block") {
                    handoff.blockers.push(value.clone());
                } else if section.contains("wait") {
                    handoff.waiting.push(value.clone());
                }
                if section.contains("current")
                    || section.contains("next")
                    || section.contains("block")
                    || section.contains("wait")
                {
                    handoff.evidence.push(locator);
                }
            }
        }
    }
    let mut tasks = Vec::new();
    let mut warnings = Vec::new();
    let mut duplicate_ordinals: HashMap<String, usize> = HashMap::new();
    for (position, candidate) in candidates.iter().enumerate() {
        if tasks.len() >= MAX_TASKS {
            warnings.push(warning(
                "TASK_LIMIT_REACHED",
                format!("maximum of {MAX_TASKS} tasks reached"),
                Some(&source.relative_path),
            ));
            break;
        }
        let mut context = heading_context(&lines, candidate.line);
        if context.is_empty() {
            context = Vec::new();
        }
        let fields = fields_for(
            &lines,
            candidate.line,
            candidates.get(position + 1).map(|t| t.line),
            &mut warnings,
            &source.relative_path,
        );
        let key = if let Some(explicit) = &candidate.explicit_id {
            format!("explicit|{}", explicit.to_ascii_lowercase())
        } else {
            format!("{}|{}", context.join("/"), normalize_text(&candidate.title))
        };
        let ordinal = duplicate_ordinals.entry(key.clone()).or_insert(0);
        let current_ordinal = *ordinal;
        *ordinal += 1;
        let id = task_id(
            &source.project_id,
            &source.relative_path,
            &context,
            candidate,
            current_ordinal,
        );
        let storage_state = match candidate.status.as_str() {
            "DONE" => "TASK_COMPLETE",
            "BLOCKED" => "BLOCKED",
            _ => "BACKLOG",
        }
        .to_string();
        let mut reasons = vec![if candidate.checklist {
            "checklist task base"
        } else {
            "explicit task row base"
        }
        .into()];
        let mut score: f32 = if candidate.checklist { 0.70 } else { 0.65 };
        if candidate.explicit_id.is_some() {
            score += 0.10;
            reasons.push("explicit task id".into());
        }
        if !matches!(
            candidate.status.as_str(),
            "CHECKLIST" | "OPEN" | "DONE" | "IN_PROGRESS" | "BLOCKED"
        ) {
            score += 0.05;
            reasons.push("explicit structured status".into());
        }
        if !context.is_empty() {
            score += 0.05;
            reasons.push("heading/milestone context".into());
        }
        if !fields.blockers.is_empty()
            || !fields.dependencies.is_empty()
            || fields.next_step.is_some()
            || fields.actor.is_some()
            || !fields.acceptance.is_empty()
        {
            score += 0.05;
            reasons.push("structured task metadata".into());
        }
        let adapter_bonus = adapter.convention_matched && candidate.explicit_id.is_some();
        if adapter_bonus {
            score += 0.05;
            reasons.push("evidenced repo-specific adapter convention".into());
        }
        let end_line = fields.end_line.max(candidate.line);
        tasks.push(ParsedTask {
            id,
            project_id: source.project_id.clone(),
            source_id: source_id(source),
            source_path: source.relative_path.clone(),
            source_kind: source.source_kind.clone(),
            title: truncate(&candidate.title),
            parsed_status: candidate.status.clone(),
            storage_state,
            explicit_task_id: candidate.explicit_id.clone(),
            milestone: context.last().cloned(),
            required_actor: fields.actor,
            blockers: cap_values(fields.blockers),
            dependency_references: cap_values(fields.dependencies),
            next_step: fields.next_step.map(|value| truncate(&value)),
            owner_gate: fields.owner_gate.map(|value| truncate(&value)),
            external_wait: fields.external_wait.map(|value| truncate(&value)),
            acceptance_criteria: cap_values(fields.acceptance),
            confidence: TaskConfidence {
                score: score.min(1.0),
                reasons,
            },
            evidence: evidence(
                source,
                hash,
                candidate.line,
                end_line,
                &context,
                candidate.explicit_id.clone(),
            ),
            adapter_id: adapter.id.clone(),
            warnings: Vec::new(),
        });
    }
    if has_handoff
        && (handoff.current.is_empty()
            && handoff.next.is_empty()
            && handoff.blockers.is_empty()
            && handoff.waiting.is_empty())
    {
        return (tasks, None, warnings);
    }
    let handoff = if has_handoff { Some(handoff) } else { None };
    (tasks, handoff, warnings)
}

fn fields_for(
    lines: &[&str],
    start: usize,
    next: Option<usize>,
    warnings: &mut Vec<ParserWarning>,
    path: &str,
) -> Fields {
    let mut fields = Fields {
        end_line: start,
        ..Default::default()
    };
    let stop = next.unwrap_or(lines.len() + 1);
    for (index, raw) in lines
        .iter()
        .enumerate()
        .skip(start)
        .take(stop.saturating_sub(start + 1))
    {
        let line = index + 1;
        let trimmed = raw.trim();
        if heading(trimmed).is_some() || task_line(trimmed).is_some() {
            break;
        }
        let value = trimmed.trim_start_matches('-').trim();
        if let Some((label, content)) = value.split_once(':') {
            let label = label.trim().to_ascii_lowercase();
            let content = clean_value(content);
            if content.is_empty() {
                continue;
            }
            fields.end_line = line;
            match label.as_str() {
                "blocker" | "blockers" | "blocked by" => {
                    push_limited(&mut fields.blockers, content, warnings, path)
                }
                "depends on" | "dependency" | "dependencies" => {
                    push_limited(&mut fields.dependencies, content, warnings, path)
                }
                "next" | "next step" => fields.next_step = Some(content),
                "owner" | "actor" | "required actor" => {
                    fields.actor = normalize_actor(&content).or_else(|| Some(String::new()))
                }
                "waiting for" => fields.external_wait = Some(content),
                "external" | "external wait" => fields.external_wait = Some(content),
                "acceptance" | "acceptance criteria" | "ac" | "definition of done" => {
                    push_limited(&mut fields.acceptance, content, warnings, path)
                }
                _ => {}
            }
        }
    }
    if fields.actor.as_deref() == Some("") {
        fields.actor = None;
    }
    fields
}

fn resolve_dependencies(snapshot: &mut TaskIntelligenceSnapshot) {
    let mut ids: HashMap<String, Vec<String>> = HashMap::new();
    for task in &snapshot.tasks {
        if let Some(explicit) = &task.explicit_task_id {
            ids.entry(explicit.to_ascii_lowercase())
                .or_default()
                .push(task.id.clone());
        }
    }
    for task in &mut snapshot.tasks {
        for reference in &task.dependency_references {
            let matches = ids
                .get(&reference.to_ascii_lowercase())
                .cloned()
                .unwrap_or_default();
            if matches.len() != 1 {
                snapshot.warnings.push(warning(
                    if matches.is_empty() {
                        "UNRESOLVED_DEPENDENCY"
                    } else {
                        "AMBIGUOUS_DEPENDENCY"
                    },
                    format!("dependency reference '{reference}' did not resolve uniquely"),
                    Some(&task.source_path),
                ));
            }
        }
    }
    trim_warnings(&mut snapshot.warnings);
}

fn persist(
    database: &DatabaseState,
    project_id: &str,
    snapshot: &TaskIntelligenceSnapshot,
    sources: &[(DiscoveredProjectSource, String)],
) -> Result<(), String> {
    let mut connection = database.open_connection()?;
    let tx = connection.transaction().map_err(db_error)?;
    tx.execute("DELETE FROM task_dependencies WHERE task_id IN (SELECT id FROM tasks WHERE project_id = ?1 AND json_extract(metadata_json, '$.owner') = ?2)", params![project_id, OWNER]).map_err(db_error)?;
    tx.execute(
        "DELETE FROM tasks WHERE project_id = ?1 AND json_extract(metadata_json, '$.owner') = ?2",
        params![project_id, OWNER],
    )
    .map_err(db_error)?;
    tx.execute(
        "DELETE FROM task_sources WHERE project_id = ?1 AND id LIKE 'm09src:%'",
        [project_id],
    )
    .map_err(db_error)?;
    let now = crate::time::utc_timestamp();
    let mut source_ids = HashMap::new();
    for (source, hash) in sources {
        let id = source_id(source);
        source_ids.insert(source.relative_path.clone(), id.clone());
        tx.execute("INSERT INTO task_sources (id, project_id, source_path, source_kind, locator, content_hash, discovered_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", params![id, project_id, source.relative_path, source.source_kind, "M09", hash, now]).map_err(db_error)?;
    }
    for task in &snapshot.tasks {
        let metadata =
            serde_json::json!({"owner": OWNER, "schemaVersion": SCHEMA_VERSION, "task": task});
        let source_id = source_ids.get(&task.source_path);
        tx.execute("INSERT INTO tasks (id, project_id, source_id, title, state, required_actor, milestone, metadata_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)", params![task.id, project_id, source_id, task.title, task.storage_state, task.required_actor, task.milestone, metadata.to_string(), now]).map_err(db_error)?;
    }
    let ids: HashMap<String, Vec<String>> = snapshot
        .tasks
        .iter()
        .filter_map(|task| {
            task.explicit_task_id
                .as_ref()
                .map(|id| (id.to_ascii_lowercase(), task.id.clone()))
        })
        .fold(HashMap::new(), |mut map, (key, value)| {
            map.entry(key).or_default().push(value);
            map
        });
    for task in &snapshot.tasks {
        for reference in &task.dependency_references {
            if let Some(targets) = ids.get(&reference.to_ascii_lowercase()) {
                if targets.len() == 1 {
                    tx.execute("INSERT INTO task_dependencies (task_id, depends_on_task_id, dependency_kind, created_at) VALUES (?1, ?2, 'SOURCE_EXPLICIT', ?3)", params![task.id, targets[0], now]).map_err(db_error)?;
                }
            }
        }
    }
    let json = serde_json::to_string(snapshot)
        .map_err(|error| format!("encode task intelligence snapshot: {error}"))?;
    tx.execute("INSERT INTO settings (key, value_json, scope, created_at, updated_at) VALUES (?1, ?2, 'PROJECT', ?3, ?3) ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json, updated_at = excluded.updated_at", params![settings_key(project_id), json, now]).map_err(db_error)?;
    tx.commit().map_err(db_error)
}

fn adapter_for(name: &str) -> ParserAdapterIdentity {
    let lower = name.to_ascii_lowercase();
    if lower == "formulab" {
        ParserAdapterIdentity {
            id: "formulab".into(),
            evidence: "FormuLab PROGRESS.md uses FVL-numbered work notation.".into(),
            convention_matched: true,
        }
    } else if lower == "scrubbots" {
        ParserAdapterIdentity {
            id: "scrubbots".into(),
            evidence: "ScrubBots TASKS.md uses TASK-XXX task identifiers.".into(),
            convention_matched: true,
        }
    } else if lower == "fmcg-erp-system" || lower == "fmcg erp" {
        ParserAdapterIdentity { id: "fmcg-erp-system".into(), evidence: "FMCG ERP TASKS.md/PLANS.md use module and phase headings with explicit checklist tasks.".into(), convention_matched: true }
    } else {
        ParserAdapterIdentity {
            id: "generic".into(),
            evidence: "Generic deterministic Markdown grammar.".into(),
            convention_matched: false,
        }
    }
}

fn task_line(line: &str) -> Option<ParsedLineTask> {
    let mut value = line.trim_start();
    if value.starts_with("- [") && value.len() >= 5 {
        let marker = value.as_bytes()[3] as char;
        let status = match marker {
            'x' | 'X' => "DONE",
            '~' => "IN_PROGRESS",
            '!' => "BLOCKED",
            _ => "OPEN",
        };
        if !matches!(marker, ' ' | 'x' | 'X' | '~' | '!') {
            return None;
        }
        value = value.get(5..).unwrap_or_default().trim();
        let explicit = explicit_id(value);
        return Some(ParsedLineTask {
            line: 0,
            title: clean_task_title(value),
            status: status.into(),
            explicit_id: explicit,
            checklist: true,
        });
    }
    if value.to_ascii_uppercase().starts_with("TASK:") || explicit_id(value).is_some() {
        if value.to_ascii_uppercase().starts_with("TASK:") {
            value = value.get(5..).unwrap_or_default().trim();
        }
        let mut parsed_status = "OPEN";
        for (tag, state) in [
            ("[DONE]", "DONE"),
            ("[BLOCKED]", "BLOCKED"),
            ("[WAITING]", "WAITING"),
            ("[READY]", "READY"),
            ("[IN PROGRESS]", "IN_PROGRESS"),
        ] {
            if value.to_ascii_uppercase().starts_with(tag) {
                value = value.get(tag.len()..).unwrap_or_default().trim();
                parsed_status = state;
                break;
            }
        }
        return Some(ParsedLineTask {
            line: 0,
            title: clean_task_title(value),
            status: parsed_status.into(),
            explicit_id: explicit_id(value),
            checklist: false,
        });
    }
    None
}

fn is_task_line(line: &str) -> bool {
    task_line(line).is_some()
}
fn heading(line: &str) -> Option<(usize, &str)> {
    let count = line.chars().take_while(|c| *c == '#').count();
    if (1..=6).contains(&count) && line.chars().nth(count) == Some(' ') {
        Some((count, line[count + 1..].trim()))
    } else {
        None
    }
}
fn is_handoff_heading(title: &str) -> bool {
    let lower = title.to_ascii_lowercase();
    [
        "handoff",
        "current",
        "next session",
        "next steps",
        "waiting",
        "blockers",
    ]
    .iter()
    .any(|key| lower.contains(key))
}
fn heading_context(lines: &[&str], target: usize) -> Vec<String> {
    let mut out = Vec::new();
    for line in lines.iter().take(target.saturating_sub(1)) {
        if let Some((level, title)) = heading(line.trim()) {
            while out.len() >= level {
                out.pop();
            }
            out.push(title.to_string());
        }
    }
    out
}
fn explicit_id(value: &str) -> Option<String> {
    let token = value
        .split_whitespace()
        .next()?
        .trim_end_matches(':')
        .trim_matches('[')
        .trim_matches(']');
    if token.to_ascii_uppercase().starts_with("TASK-")
        || token.to_ascii_uppercase().starts_with("FVL-")
        || token.to_ascii_uppercase().starts_with("FMCG-")
    {
        Some(token.to_string())
    } else {
        None
    }
}
fn clean_task_title(value: &str) -> String {
    let value = if let Some((_, rest)) = value.split_once(':') {
        if explicit_id(value).is_some() {
            rest
        } else {
            value
        }
    } else {
        value
    };
    truncate(value.trim())
}
fn clean_value(value: &str) -> String {
    truncate(value.trim().trim_start_matches('-').trim())
}
fn normalize_text(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}
fn truncate(value: &str) -> String {
    let mut out = value.to_string();
    while out.len() > MAX_FIELD_BYTES {
        out.pop();
    }
    out
}
fn cap_values(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .take(MAX_ENTRIES)
        .map(|value| truncate(&value))
        .collect()
}
fn push_limited(
    values: &mut Vec<String>,
    value: String,
    warnings: &mut Vec<ParserWarning>,
    path: &str,
) {
    if values.len() < MAX_ENTRIES {
        values.push(value);
    } else {
        warnings.push(warning(
            "TASK_LIMIT_REACHED",
            "metadata entry bound reached".into(),
            Some(path),
        ));
    }
}
fn normalize_actor(value: &str) -> Option<String> {
    ["Human", "Codex", "Claude", "GPT Audit", "CI", "External"]
        .iter()
        .find(|actor| actor.eq_ignore_ascii_case(value.trim()))
        .map(|actor| (*actor).into())
}
fn evidence(
    source: &DiscoveredProjectSource,
    hash: &str,
    start: usize,
    end: usize,
    headings: &[String],
    locator_text: Option<String>,
) -> TaskEvidenceLocator {
    TaskEvidenceLocator {
        source_path: source.relative_path.clone(),
        content_hash: hash.to_string(),
        start_line: start,
        end_line: end,
        heading_path: headings.to_vec(),
        locator_text,
    }
}
fn task_id(
    project: &str,
    path: &str,
    headings: &[String],
    candidate: &ParsedLineTask,
    ordinal: usize,
) -> String {
    let identity = if let Some(explicit) = &candidate.explicit_id {
        format!(
            "{project}|{path}|explicit|{}|{ordinal}",
            explicit.to_ascii_lowercase()
        )
    } else {
        format!(
            "{project}|{path}|{}|{}|{ordinal}",
            headings.join("/"),
            normalize_text(&candidate.title)
        )
    };
    format!("m09task:{}", hash_bytes(identity.as_bytes()))
}
fn source_id(source: &DiscoveredProjectSource) -> String {
    format!(
        "m09src:{}",
        hash_bytes(
            format!(
                "{}|{}|{}",
                source.project_id,
                source.relative_path,
                source.content_hash.clone().unwrap_or_default()
            )
            .as_bytes()
        )
    )
}
fn hash_bytes(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn settings_key(project_id: &str) -> String {
    format!("task_intelligence.snapshot.{project_id}")
}
fn warning(code: &str, message: String, path: Option<&str>) -> ParserWarning {
    ParserWarning {
        code: code.into(),
        message: truncate(&message),
        source_path: path.map(str::to_string),
    }
}
fn trim_warnings(warnings: &mut Vec<ParserWarning>) {
    if warnings.len() >= MAX_WARNINGS {
        warnings.truncate(MAX_WARNINGS - 1);
        warnings.push(warning(
            "WARNING_LIMIT_REACHED",
            format!("maximum of {MAX_WARNINGS} warnings retained"),
            None,
        ));
        warnings.truncate(MAX_WARNINGS);
    }
}
fn db_error(error: rusqlite::Error) -> String {
    format!("task intelligence database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use std::fs;
    use tempfile::tempdir;

    fn fixture(contents: &str) -> (tempfile::TempDir, tempfile::TempDir, DatabaseState, String) {
        fixture_named(contents, "Synthetic")
    }
    fn fixture_named(
        contents: &str,
        name: &str,
    ) -> (tempfile::TempDir, tempfile::TempDir, DatabaseState, String) {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(project_dir.path().join("TASKS.md"), contents).unwrap();
        let db = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project = register_project(
            &db,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some(name.into()),
            },
        )
        .unwrap();
        (db_dir, project_dir, db, project.id)
    }
    #[test]
    fn p01_inventory_boundary_excludes_instruction_bullets() {
        let (_db_dir, _dir, db, id) = fixture("# Work\n- [ ] real\n");
        let project = fetch_project(&db, &id).unwrap();
        fs::write(
            Path::new(&project.normalized_path).join("AGENTS.md"),
            "- [ ] instruction\n",
        )
        .unwrap();
        let snapshot = parse(&db, &id).unwrap();
        assert_eq!(snapshot.tasks.len(), 1);
        assert_eq!(snapshot.tasks[0].title, "real");
    }
    #[test]
    fn p01_outside_root_source_is_rejected_by_production_reader() {
        let (_db_dir, _dir, db, id) = fixture("- [ ] real\n");
        let outside = tempfile::tempdir().unwrap();
        let path = outside.path().join("outside.md");
        fs::write(&path, "- [ ] outside").unwrap();
        let source = DiscoveredProjectSource {
            id: "forged".into(),
            project_id: id.clone(),
            relative_path: "../outside.md".into(),
            absolute_path: path.to_string_lossy().into(),
            source_kind: "TASKS".into(),
            origin: "STANDARD".into(),
            status: "AVAILABLE".into(),
            authority_class: "TASKS".into(),
            priority: 10,
            size_bytes: None,
            modified_at: None,
            discovered_at: "now".into(),
            content_hash: Some(hash_bytes(b"- [ ] outside")),
            depth: 0,
            warnings: Vec::new(),
            schema_version: 1,
            owner: "M08_TASK_SOURCE_DISCOVERY".into(),
            source_order: None,
        };
        let result = read_authoritative_source(&db, &id, &source).unwrap_err();
        assert_eq!(result.code, "SOURCE_READ_FAILED");
    }
    #[test]
    fn p01_source_change_is_retried_once_then_warned() {
        let (_db_dir, dir, db, id) = fixture("- [ ] old\n");
        let target = dir.path().join("TASKS.md");
        let source = task_sources::discover(&db, &id)
            .unwrap()
            .into_iter()
            .find(|source| source.relative_path == "TASKS.md")
            .unwrap();
        fs::write(&target, "- [ ] changed\n").unwrap();
        assert!(read_authoritative_source(&db, &id, &source)
            .unwrap()
            .is_none());
    }
    #[test]
    fn p01_invalid_utf8_isolated_from_valid_source() {
        let (_db_dir, dir, db, id) = fixture("- [ ] valid\n");
        fs::write(dir.path().join("bad.md"), [0xff, 0xfe]).unwrap();
        task_sources::custom_path_add(
            &db,
            task_sources::CustomPathRequest {
                project_id: id.clone(),
                path: "bad.md".into(),
            },
        )
        .unwrap();
        let snapshot = parse(&db, &id).unwrap();
        assert!(snapshot.tasks.iter().any(|task| task.title == "valid"));
        assert!(snapshot
            .warnings
            .iter()
            .any(|warning| warning.code == "INVALID_UTF8"));
    }
    #[test]
    fn p02_ids_are_stable_and_project_scoped() {
        let (_db_dir, _dir, db, id) = fixture("# Work\n- [ ] one\n- [ ] one\n");
        let first = parse(&db, &id).unwrap();
        let second = parse(&db, &id).unwrap();
        assert_eq!(first.tasks, second.tasks);
        assert_ne!(first.tasks[0].id, first.tasks[1].id);
    }
    #[test]
    fn p02_same_source_text_in_two_projects_never_collides() {
        let (_db_dir, _dir, db, first_id) = fixture("- [ ] same\n");
        let second_dir = tempfile::tempdir().unwrap();
        fs::write(second_dir.path().join("TASKS.md"), "- [ ] same\n").unwrap();
        let second = register_project(
            &db,
            RegisterProjectRequest {
                path: second_dir.path().to_string_lossy().into(),
                name: Some("Second".into()),
            },
        )
        .unwrap();
        let first_task = parse(&db, &first_id).unwrap().tasks.remove(0);
        let second_task = parse(&db, &second.id).unwrap().tasks.remove(0);
        assert_ne!(first_task.id, second_task.id);
    }
    #[test]
    fn p03_checkbox_and_neutral_storage_mapping() {
        let (_db_dir, _dir, db, id) = fixture(
            "# Milestone\n- [ ] open\n- [x] done\n- [~] active\n- [!] blocked\n- ordinary prose\n",
        );
        let s = parse(&db, &id).unwrap();
        assert_eq!(
            s.tasks
                .iter()
                .map(|t| t.parsed_status.as_str())
                .collect::<Vec<_>>(),
            vec!["OPEN", "DONE", "IN_PROGRESS", "BLOCKED"]
        );
        assert_eq!(
            s.tasks
                .iter()
                .map(|t| t.storage_state.as_str())
                .collect::<Vec<_>>(),
            vec!["BACKLOG", "TASK_COMPLETE", "BACKLOG", "BLOCKED"]
        );
    }
    #[test]
    fn p04_structured_metadata_and_unknown_actor() {
        let (_db_dir, _dir, db, id) = fixture("# Work\n- [ ] build\n  Blocker: dependency\n  Depends on: TASK-2\n  Next step: verify\n  Owner: Mystery\n  Waiting for: vendor\n  Acceptance: test it\n");
        let s = parse(&db, &id).unwrap();
        let t = &s.tasks[0];
        assert_eq!(t.blockers, vec!["dependency"]);
        assert_eq!(t.next_step.as_deref(), Some("verify"));
        assert_eq!(t.required_actor, None);
        assert_eq!(t.evidence.start_line, 2);
        assert_eq!(t.evidence.end_line, 8);
    }
    #[test]
    fn p04_dependency_resolves_only_to_an_unambiguous_explicit_id() {
        let (_db_dir, _dir, db, id) =
            fixture("- [ ] TASK-0: base\n- [ ] TASK-1: child\n  Depends on: TASK-0\n");
        let snapshot = parse(&db, &id).unwrap();
        let connection = db.open_connection().unwrap();
        assert_eq!(
            connection
                .query_row("SELECT dependency_kind FROM task_dependencies", [], |row| {
                    row.get::<_, String>(0)
                })
                .unwrap(),
            "SOURCE_EXPLICIT"
        );
        assert!(!snapshot
            .warnings
            .iter()
            .any(|warning| warning.code == "UNRESOLVED_DEPENDENCY"));
    }
    #[test]
    fn p04_ambiguous_dependency_stays_metadata_only_with_warning() {
        let (_db_dir, _dir, db, id) = fixture("- [ ] TASK-0: first\n- [ ] TASK-0: second\n- [ ] TASK-1: child\n  Depends on: TASK-0\n");
        let snapshot = parse(&db, &id).unwrap();
        assert!(snapshot
            .warnings
            .iter()
            .any(|warning| warning.code == "AMBIGUOUS_DEPENDENCY"));
        assert_eq!(
            db.open_connection()
                .unwrap()
                .query_row("SELECT COUNT(*) FROM task_dependencies", [], |row| row
                    .get::<_, i64>(0))
                .unwrap(),
            0
        );
    }
    #[test]
    fn p05_handoff_narrative_and_checklist_are_separate() {
        let (_db_dir, _dir, db, id) = fixture("# Handoff\n## Current\nworking now\n## Next session\n- [ ] continue\n## Waiting for\nvendor\n");
        let s = parse(&db, &id).unwrap();
        assert_eq!(s.tasks.len(), 1);
        let h = s.handoff.unwrap();
        assert_eq!(h.current, vec!["working now"]);
        assert_eq!(h.next, Vec::<String>::new());
        assert_eq!(h.waiting, vec!["vendor"]);
    }
    #[test]
    fn p06_adapter_selection_is_explicit() {
        assert_eq!(adapter_for("FormuLab").id, "formulab");
        assert_eq!(adapter_for("ScrubBots").id, "scrubbots");
        assert_eq!(adapter_for("fmcg-erp-system").id, "fmcg-erp-system");
        assert_eq!(adapter_for("unrelated").id, "generic");
    }
    #[test]
    fn p06_registered_adapter_fixtures_use_evidenced_conventions() {
        for (name, marker, expected) in [
            ("FormuLab", "FVL-03.013-018: formula", "formulab"),
            ("ScrubBots", "TASK-101: board", "scrubbots"),
            ("fmcg-erp-system", "FMCG-001: utility", "fmcg-erp-system"),
        ] {
            let (_db_dir, _dir, db, id) = fixture_named(&format!("- [ ] {marker}\n"), name);
            let snapshot = parse(&db, &id).unwrap();
            assert_eq!(snapshot.adapter.id, expected);
            assert!(snapshot.adapter.convention_matched);
            assert_eq!(snapshot.tasks[0].adapter_id, expected);
        }
        let (_db_dir, _dir, db, id) =
            fixture_named("- [ ] TASK-101: unrelated\n", "FormuLab Clone");
        assert_eq!(parse(&db, &id).unwrap().adapter.id, "generic");
    }
    #[test]
    fn p07_owned_sql_reconciliation_preserves_events_and_is_idempotent() {
        let (_db_dir, _dir, db, id) = fixture("- [ ] one\n");
        let a = parse(&db, &id).unwrap();
        let b = parse(&db, &id).unwrap();
        assert_eq!(a.tasks[0].id, b.tasks[0].id);
        let c = db.open_connection().unwrap();
        assert_eq!(
            c.query_row("SELECT COUNT(*) FROM task_events", [], |r| r
                .get::<_, i64>(0))
                .unwrap(),
            0
        );
        assert_eq!(c.query_row("SELECT COUNT(*) FROM tasks WHERE project_id=?1 AND json_extract(metadata_json,'$.owner')=?2", params![id, OWNER], |r| r.get::<_,i64>(0)).unwrap(), 1);
    }
    #[test]
    fn p07_unrelated_rows_and_project_bytes_are_preserved() {
        let (_db_dir, dir, db, id) = fixture("- [ ] one\n");
        let before = fs::read(dir.path().join("TASKS.md")).unwrap();
        let connection = db.open_connection().unwrap();
        connection.execute("INSERT INTO task_sources (id, project_id, source_path, source_kind, locator, content_hash, discovered_at) VALUES ('legacy-source', ?1, 'legacy.md', 'LEGACY', 'legacy', 'legacy-hash', 'now')", [&id]).unwrap();
        connection.execute("INSERT INTO tasks (id, project_id, source_id, title, state, metadata_json, created_at, updated_at) VALUES ('legacy-task', ?1, 'legacy-source', 'Legacy', 'BACKLOG', '{\"legacy\":true}', 'now', 'now')", [&id]).unwrap();
        connection.execute("INSERT INTO settings (key, value_json, scope, created_at, updated_at) VALUES ('legacy-setting', '{\"keep\":true}', 'PROJECT', 'now', 'now')", []).unwrap();
        parse(&db, &id).unwrap();
        assert_eq!(before, fs::read(dir.path().join("TASKS.md")).unwrap());
        assert_eq!(
            connection
                .query_row(
                    "SELECT COUNT(*) FROM tasks WHERE id='legacy-task'",
                    [],
                    |row| row.get::<_, i64>(0)
                )
                .unwrap(),
            1
        );
        assert_eq!(
            connection
                .query_row(
                    "SELECT COUNT(*) FROM task_sources WHERE id='legacy-source'",
                    [],
                    |row| row.get::<_, i64>(0)
                )
                .unwrap(),
            1
        );
        assert_eq!(
            connection
                .query_row(
                    "SELECT value_json FROM settings WHERE key='legacy-setting'",
                    [],
                    |row| row.get::<_, String>(0)
                )
                .unwrap(),
            "{\"keep\":true}"
        );
    }
    #[test]
    fn p08_list_reads_persisted_snapshot_only() {
        let (_db_dir, _dir, db, id) = fixture("- [ ] one\n");
        parse(&db, &id).unwrap();
        let listed = list(&db, &id).unwrap();
        assert_eq!(listed.tasks.len(), 1);
    }
    #[test]
    fn p09_archived_project_is_rejected() {
        let (_db_dir, _dir, db, id) = fixture("- [ ] one\n");
        crate::projects::archive_project(&db, &id).unwrap();
        assert!(parse(&db, &id).unwrap_err().contains("archived"));
    }
    #[test]
    fn p09_missing_project_is_rejected() {
        let (_db_dir, _dir, db, id) = fixture("- [ ] one\n");
        db.open_connection()
            .unwrap()
            .execute("UPDATE projects SET status='MISSING' WHERE id=?1", [&id])
            .unwrap();
        assert!(parse(&db, &id).unwrap_err().contains("unavailable"));
    }
    #[test]
    fn p09_warning_bound_is_structured() {
        let mut warnings = (0..MAX_WARNINGS)
            .map(|_| warning("SOURCE_READ_FAILED", "bounded".into(), None))
            .collect::<Vec<_>>();
        trim_warnings(&mut warnings);
        assert_eq!(warnings.len(), MAX_WARNINGS);
        assert_eq!(warnings.last().unwrap().code, "WARNING_LIMIT_REACHED");
    }
    #[test]
    fn p10_locator_and_confidence_are_bounded_and_deterministic() {
        let (_db_dir, _dir, db, id) =
            fixture("# M\n- [ ] item\n  Acceptance: done\n- [ ] sibling\n");
        let s = parse(&db, &id).unwrap();
        assert_eq!(
            (s.tasks[0].evidence.start_line, s.tasks[0].evidence.end_line),
            (2, 3)
        );
        assert_eq!(s.tasks[0].confidence.score, 0.80);
        assert!(s.tasks[0].evidence.end_line < s.tasks[1].evidence.start_line);
    }
}
