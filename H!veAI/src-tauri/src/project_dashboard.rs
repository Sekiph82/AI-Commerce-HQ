use crate::db::DatabaseState;
use crate::projects::fetch_project;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

pub const MANIFEST_RELATIVE_PATH: &str = ".hiveai/PROJECT_DASHBOARD.md";
pub const MAX_MANIFEST_BYTES: usize = 64 * 1024;
pub const MAX_MANIFEST_LINE_BYTES: usize = 4096;
pub const MAX_FRONT_MATTER_FIELDS: usize = 32;
pub const MAX_SOURCE_PATHS: usize = 128;
pub const MAX_SOURCE_PATHS_PER_ROLE: usize = 32;
pub const MAX_SOURCE_PATH_BYTES: usize = 512;
pub const MAX_MANIFEST_WARNINGS: usize = 64;
pub const MAX_WARNING_SCALAR_BYTES: usize = 1024;
pub const MAX_MATERIALIZED_ITEMS: usize = 10;
pub const MAX_MATERIALIZED_PROVENANCE: usize = 32;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ManifestStatus {
    Valid,
    Partial,
    Absent,
    Malformed,
    Stale,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskAuthorityState {
    Canonical,
    NotCanonicalized,
    FallbackM08M09,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SourceStatus {
    Available,
    Missing,
    Rejected,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedSource {
    pub path: String,
    pub role: String,
    pub status: SourceStatus,
    pub exists: bool,
    pub contained: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MaterializedFact {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MaterializedWorkRow {
    pub id: String,
    pub item: String,
    pub status: String,
    pub owner_actor: String,
    pub evidence_source: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MaterializedDashboardStatus {
    pub project_status: Option<String>,
    pub health: Option<String>,
    pub current_milestone: Option<String>,
    pub current_task_title: Option<String>,
    pub current_task_id: Option<String>,
    pub declared_workflow_state: Option<String>,
    pub progress_raw: Option<String>,
    pub progress_percent: Option<u32>,
    pub required_actor: Option<String>,
    pub next_action: Option<String>,
    pub waiting_on: Option<String>,
    pub last_meaningful_update: Option<String>,
    pub current_work: Vec<MaterializedWorkRow>,
    pub blockers_waiting: Vec<String>,
    pub milestone_summary: Vec<String>,
    pub quality_verification: Vec<MaterializedFact>,
    pub recent_meaningful_activity: Vec<String>,
    pub provenance: Vec<MaterializedFact>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDashboardResolution {
    pub project_id: String,
    pub manifest_status: ManifestStatus,
    pub manifest_path: String,
    pub schema: Option<String>,
    pub project_key: Option<String>,
    pub repository: Option<String>,
    pub branch_policy: Option<String>,
    pub dashboard_mode: Option<String>,
    pub tracking_mode: Option<String>,
    pub refresh_policy: Option<String>,
    pub task_authority: TaskAuthorityState,
    pub canonical_task_source: Option<String>,
    pub roles: BTreeMap<String, Vec<ResolvedSource>>,
    pub provenance_mode: String,
    pub materialized: MaterializedDashboardStatus,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct ParsedManifest {
    schema: Option<String>,
    project_key: Option<String>,
    repository: Option<String>,
    branch_policy: Option<String>,
    dashboard_mode: Option<String>,
    tracking_mode: Option<String>,
    refresh_policy: Option<String>,
    materialized: MaterializedDashboardStatus,
    materialized_warnings: Vec<String>,
    roles: BTreeMap<String, Vec<String>>,
    warnings: Vec<String>,
}

pub fn resolve(
    database: &DatabaseState,
    project_id: &str,
) -> Result<ProjectDashboardResolution, String> {
    let project = fetch_project(database, project_id)?;
    let root = PathBuf::from(&project.normalized_path);
    let manifest_path = root.join(MANIFEST_RELATIVE_PATH);
    let base = empty_resolution(project_id, manifest_path.to_string_lossy().into_owned());
    if project.status != "ACTIVE" || !root.is_dir() {
        return Ok(ProjectDashboardResolution {
            manifest_status: ManifestStatus::Unavailable,
            provenance_mode: "FALLBACK_M08_M09".into(),
            warnings: vec!["registered project root is unavailable".into()],
            ..base
        });
    }
    if !manifest_path.exists() {
        return Ok(ProjectDashboardResolution {
            manifest_status: ManifestStatus::Absent,
            provenance_mode: "FALLBACK_M08_M09".into(),
            warnings: vec![format!("{MANIFEST_RELATIVE_PATH} is absent")],
            ..base
        });
    }
    let canonical_root =
        fs::canonicalize(&root).map_err(|e| format!("resolve project root: {e}"))?;
    let canonical_manifest = match fs::canonicalize(&manifest_path) {
        Ok(path) if path.starts_with(&canonical_root) && path.is_file() => path,
        Ok(_) => {
            return Ok(rejected_manifest(
                base,
                "manifest is outside the registered project root",
            ))
        }
        Err(error) => {
            return Ok(rejected_manifest(
                base,
                &format!("manifest is unavailable: {error}"),
            ))
        }
    };
    let text = match read_manifest(&canonical_manifest) {
        Ok(text) => text,
        Err(error) => return Ok(rejected_manifest(base, &error)),
    };
    let parsed = match parse_manifest(&text) {
        Ok(parsed) => parsed,
        Err(error) => return Ok(rejected_manifest(base, &error)),
    };
    let repository_matches = if let Some(manifest_repository) = parsed.repository.as_deref() {
        project
            .repository
            .as_ref()
            .filter(|repository| {
                repository.github_owner.is_some() && repository.github_repo.is_some()
            })
            .map(|repository| {
                normalize_repository(manifest_repository)
                    == normalize_repository(&format!(
                        "{}/{}",
                        repository.github_owner.as_deref().unwrap_or_default(),
                        repository.github_repo.as_deref().unwrap_or_default()
                    ))
            })
    } else {
        None
    };
    if repository_matches == Some(false) {
        return Ok(ProjectDashboardResolution {
            manifest_status: ManifestStatus::Stale,
            task_authority: TaskAuthorityState::FallbackM08M09,
            provenance_mode: "FALLBACK_M08_M09".into(),
            warnings: vec![
                "manifest repository identity conflicts with the registered Git identity".into(),
            ],
            ..base
        });
    }
    let mut resolution = ProjectDashboardResolution {
        project_id: project_id.to_string(),
        manifest_status: ManifestStatus::Valid,
        manifest_path: canonical_manifest.to_string_lossy().into_owned(),
        schema: parsed.schema,
        project_key: parsed.project_key,
        repository: parsed.repository,
        branch_policy: parsed.branch_policy,
        dashboard_mode: parsed.dashboard_mode,
        tracking_mode: parsed.tracking_mode,
        refresh_policy: parsed.refresh_policy,
        task_authority: TaskAuthorityState::NotCanonicalized,
        canonical_task_source: None,
        roles: BTreeMap::new(),
        provenance_mode: "MANIFEST".into(),
        materialized: parsed.materialized,
        warnings: {
            let mut warnings = parsed.warnings;
            for warning in parsed.materialized_warnings {
                push_warning(&mut warnings, warning);
            }
            warnings
        },
    };
    let mut extracted = 0usize;
    for (role, paths) in parsed.roles {
        let mut resolved = Vec::new();
        for path in paths {
            extracted += 1;
            if extracted > MAX_SOURCE_PATHS {
                push_warning(
                    &mut resolution.warnings,
                    format!("source path limit reached ({MAX_SOURCE_PATHS})"),
                );
                break;
            }
            let (normalized, valid) = normalize_relative_path(&path);
            if !valid {
                push_warning(
                    &mut resolution.warnings,
                    format!("rejected authority path for {role}"),
                );
                resolved.push(ResolvedSource {
                    path,
                    role: role.clone(),
                    status: SourceStatus::Rejected,
                    exists: false,
                    contained: false,
                });
                continue;
            }
            let candidate = root.join(&normalized);
            let exists = candidate.exists();
            let directory_allowed = matches!(role.as_str(), "progressHistory" | "buildTest");
            let pointer_available =
                candidate.is_file() || (directory_allowed && candidate.is_dir());
            let contained = exists
                && fs::canonicalize(&candidate)
                    .map(|path| path.starts_with(&canonical_root))
                    .unwrap_or(false);
            let status = if !contained {
                if exists {
                    SourceStatus::Rejected
                } else {
                    SourceStatus::Missing
                }
            } else if !pointer_available {
                SourceStatus::Rejected
            } else {
                SourceStatus::Available
            };
            resolved.push(ResolvedSource {
                path: normalized.clone(),
                role: role.clone(),
                status: status.clone(),
                exists,
                contained,
            });
            if role == "canonicalTask" && resolution.canonical_task_source.is_none() {
                if status == SourceStatus::Available {
                    resolution.canonical_task_source = Some(normalized);
                }
            }
        }
        if role == "canonicalTask"
            && resolved.iter().any(|source| {
                source.status == SourceStatus::Missing || source.status == SourceStatus::Rejected
            })
        {
            resolution.manifest_status = ManifestStatus::Stale;
            push_warning(
                &mut resolution.warnings,
                "canonical task source is unavailable or rejected",
            );
        }
        resolution.roles.insert(role, resolved);
    }
    if resolution.canonical_task_source.is_some() {
        resolution.task_authority = TaskAuthorityState::Canonical;
        if resolution.manifest_status == ManifestStatus::Valid
            && resolution
                .roles
                .values()
                .flatten()
                .any(|source| source.status != SourceStatus::Available)
        {
            resolution.manifest_status = ManifestStatus::Partial;
        }
    } else {
        resolution.task_authority = TaskAuthorityState::NotCanonicalized;
    }
    if resolution.manifest_status == ManifestStatus::Stale {
        resolution.task_authority = TaskAuthorityState::FallbackM08M09;
        resolution.provenance_mode = "FALLBACK_M08_M09".into();
    }
    Ok(resolution)
}

fn empty_resolution(project_id: &str, manifest_path: String) -> ProjectDashboardResolution {
    ProjectDashboardResolution {
        project_id: project_id.into(),
        manifest_status: ManifestStatus::Absent,
        manifest_path,
        schema: None,
        project_key: None,
        repository: None,
        branch_policy: None,
        dashboard_mode: None,
        tracking_mode: None,
        refresh_policy: None,
        task_authority: TaskAuthorityState::FallbackM08M09,
        canonical_task_source: None,
        roles: BTreeMap::new(),
        provenance_mode: "FALLBACK_M08_M09".into(),
        materialized: MaterializedDashboardStatus::default(),
        warnings: Vec::new(),
    }
}

fn rejected_manifest(
    mut base: ProjectDashboardResolution,
    warning: &str,
) -> ProjectDashboardResolution {
    base.manifest_status = ManifestStatus::Malformed;
    push_warning(&mut base.warnings, warning);
    base
}

fn push_warning(warnings: &mut Vec<String>, message: impl AsRef<str>) {
    let mut bounded = message.as_ref().to_string();
    while bounded.len() > MAX_WARNING_SCALAR_BYTES {
        bounded.pop();
    }
    if warnings.iter().any(|existing| existing == &bounded) {
        return;
    }
    if warnings.len() < MAX_MANIFEST_WARNINGS.saturating_sub(1) {
        warnings.push(bounded);
        return;
    }
    warnings.truncate(MAX_MANIFEST_WARNINGS.saturating_sub(1));
    warnings.push(format!(
        "WARNING_LIMIT_REACHED: manifest warning limit reached ({MAX_MANIFEST_WARNINGS})"
    ));
}

fn read_manifest(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("open manifest: {e}"))?;
    if file
        .metadata()
        .map_err(|e| format!("inspect manifest: {e}"))?
        .len()
        > MAX_MANIFEST_BYTES as u64
    {
        return Err(format!("manifest exceeds {MAX_MANIFEST_BYTES} bytes"));
    }
    let mut bytes = Vec::new();
    file.by_ref()
        .take((MAX_MANIFEST_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read manifest: {e}"))?;
    String::from_utf8(bytes).map_err(|_| "manifest is not valid UTF-8".into())
}

fn parse_manifest(text: &str) -> Result<ParsedManifest, String> {
    let mut fields = BTreeMap::new();
    let mut roles: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut in_header = true;
    let mut in_authorities = false;
    let mut warnings = Vec::new();
    let mut front_matter_fields = 0usize;
    for line in text.lines() {
        if line.as_bytes().len() > MAX_MANIFEST_LINE_BYTES {
            return Err(format!(
                "manifest line exceeds {MAX_MANIFEST_LINE_BYTES} bytes"
            ));
        }
        let trimmed = line.trim();
        if trimmed.contains("[ ]")
            || trimmed.contains("[x]")
            || trimmed.contains("[~]")
            || trimmed.contains("[!]")
        {
            push_warning(
                &mut warnings,
                "manifest contains task checkbox syntax; pointer checkboxes are ignored",
            );
        }
        if trimmed.starts_with("## ") {
            in_header = false;
            in_authorities = trimmed.eq_ignore_ascii_case("## Source authorities");
            continue;
        }
        if in_authorities {
            let Some((label, value)) = trimmed.split_once(':') else {
                continue;
            };
            let role = match label
                .trim()
                .trim_start_matches('-')
                .trim()
                .to_ascii_lowercase()
                .as_str()
            {
                "canonical task source" => "canonicalTask",
                "handoff source" | "handoff sources" => "handoff",
                "roadmap source" | "roadmap/plan source" | "roadmap / plan source" => "roadmap",
                "progress/history source" | "progress/history sources" => "progressHistory",
                "architecture source" | "architecture/design source" => "architecture",
                "decision source" | "decision/governance source" => "decision",
                "agent instruction source" | "agent instruction sources" => "instructions",
                "security source" => "security",
                "build/test metadata" => "buildTest",
                _ => continue,
            };
            let paths = extract_paths(value)?;
            if paths.is_empty() {
                continue;
            }
            let entry = roles.entry(role.into()).or_default();
            if entry.len().saturating_add(paths.len()) > MAX_SOURCE_PATHS_PER_ROLE {
                return Err(format!("source path limit reached for {label}"));
            }
            entry.extend(paths);
            continue;
        }
        if !in_header {
            continue;
        }
        {
            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim();
                if !key.is_empty() {
                    front_matter_fields = front_matter_fields.saturating_add(1);
                    if front_matter_fields > MAX_FRONT_MATTER_FIELDS {
                        return Err(format!(
                            "front-matter field limit reached ({MAX_FRONT_MATTER_FIELDS})"
                        ));
                    }
                }
                if matches!(
                    key,
                    "hiveaiDashboardSchema"
                        | "projectKey"
                        | "repository"
                        | "branchPolicy"
                        | "dashboardMode"
                        | "trackingMode"
                        | "refreshPolicy"
                ) {
                    fields.insert(key.to_string(), clean_scalar(value));
                }
            }
            continue;
        }
    }
    let schema = fields.get("hiveaiDashboardSchema").cloned();
    if schema.as_deref() != Some("hiveai-project-dashboard/v1") {
        return Err("unsupported or missing hiveaiDashboardSchema".into());
    }
    if fields.get("dashboardMode").map(String::as_str) != Some("source-map") {
        return Err("dashboardMode must be source-map".into());
    }
    let (materialized, materialized_warnings) = parse_materialized_sections(text);
    Ok(ParsedManifest {
        schema,
        project_key: fields.remove("projectKey"),
        repository: fields.remove("repository"),
        branch_policy: fields.remove("branchPolicy"),
        dashboard_mode: fields.remove("dashboardMode"),
        tracking_mode: fields.remove("trackingMode"),
        refresh_policy: fields.remove("refreshPolicy"),
        materialized,
        materialized_warnings,
        roles,
        warnings,
    })
}

fn extract_paths(value: &str) -> Result<Vec<String>, String> {
    if value.to_ascii_lowercase().contains("none verified") {
        return Ok(Vec::new());
    }
    let mut result = Vec::new();
    let mut parts = value.split('`');
    while let Some(_before) = parts.next() {
        let Some(token) = parts.next() else {
            break;
        };
        if token.as_bytes().len() > MAX_SOURCE_PATH_BYTES {
            return Err(format!("source path exceeds {MAX_SOURCE_PATH_BYTES} bytes"));
        }
        if !token.trim().is_empty() {
            result.push(token.trim().to_string());
        }
        if result.len() > MAX_SOURCE_PATHS_PER_ROLE {
            return Err("source path limit exceeded".into());
        }
    }
    Ok(result)
}

fn parse_materialized_sections(text: &str) -> (MaterializedDashboardStatus, Vec<String>) {
    let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        if let Some(heading) = line.trim().strip_prefix("## ") {
            let name = heading.trim().to_ascii_lowercase();
            current = Some(name.clone());
            sections.entry(name).or_default();
        } else if let Some(name) = current.as_ref() {
            sections
                .entry(name.clone())
                .or_default()
                .push(line.to_string());
        }
    }
    let mut status = MaterializedDashboardStatus::default();
    let mut warnings = Vec::new();
    let live = sections
        .get("h!veai live status")
        .map(Vec::as_slice)
        .unwrap_or_default();
    for (label, value) in table_or_colon_facts(live) {
        match label.to_ascii_lowercase().as_str() {
            "project status" => status.project_status = Some(value),
            "health" => status.health = Some(value),
            "current milestone" => status.current_milestone = Some(value),
            "current task" => status.current_task_title = Some(value),
            "current task id" => status.current_task_id = Some(value),
            "current workflow state" => status.declared_workflow_state = Some(value),
            "progress" => {
                status.progress_percent = parse_progress_percent(&value);
                status.progress_raw = Some(value);
            }
            "required actor" => status.required_actor = Some(value),
            "next action" => status.next_action = Some(value),
            "waiting on" => status.waiting_on = Some(value),
            "last meaningful update" => status.last_meaningful_update = Some(value),
            _ => {}
        }
    }
    if let Some(value) = status.project_status.take() {
        status.project_status = Some(normalize_materialized_enum(
            &value,
            "Project status",
            &[
                "ACTIVE", "PAUSED", "WAITING", "BLOCKED", "COMPLETE", "UNKNOWN",
            ],
            &mut warnings,
        ));
    }
    if let Some(value) = status.health.take() {
        status.health = Some(normalize_materialized_enum(
            &value,
            "Health",
            &["HEALTHY", "ATTENTION", "BLOCKED", "UNKNOWN"],
            &mut warnings,
        ));
    }
    if let Some(value) = status.required_actor.take() {
        status.required_actor = Some(normalize_materialized_enum(
            &value,
            "Required actor",
            &[
                "HUMAN",
                "CODEX",
                "CLAUDE",
                "GPT_AUDIT",
                "CI",
                "EXTERNAL",
                "NONE",
                "UNKNOWN",
            ],
            &mut warnings,
        ));
    }
    if let Some(lines) = sections.get("current work") {
        status.current_work = parse_work_rows(lines, &mut warnings);
    }
    if let Some(lines) = sections.get("blockers and waiting") {
        status.blockers_waiting = parse_bounded_items(lines, MAX_MATERIALIZED_ITEMS);
    }
    if let Some(lines) = sections.get("milestone summary") {
        status.milestone_summary = parse_bounded_items(lines, MAX_MATERIALIZED_ITEMS);
    }
    if let Some(lines) = sections.get("quality and verification") {
        status.quality_verification = parse_bounded_facts(lines, MAX_MATERIALIZED_ITEMS);
    }
    if let Some(lines) = sections.get("recent meaningful activity") {
        status.recent_meaningful_activity = parse_bounded_items(lines, MAX_MATERIALIZED_ITEMS);
    }
    if let Some(lines) = sections.get("provenance") {
        status.provenance = parse_bounded_facts(lines, MAX_MATERIALIZED_PROVENANCE);
    }
    (status, warnings)
}

fn normalize_materialized_enum(
    value: &str,
    field: &str,
    allowed: &[&str],
    warnings: &mut Vec<String>,
) -> String {
    let normalized = value.trim().to_ascii_uppercase().replace([' ', '-'], "_");
    if allowed.iter().any(|candidate| *candidate == normalized) {
        return normalized;
    }
    push_warning(
        warnings,
        format!("invalid materialized {field} value; using UNKNOWN"),
    );
    "UNKNOWN".into()
}

fn table_or_colon_facts(lines: &[String]) -> Vec<(String, String)> {
    lines
        .iter()
        .filter_map(|line| {
            let cells = table_cells(line);
            if cells.len() >= 2 && !is_table_separator(&cells) {
                return Some((cells[0].clone(), cells[1].clone()));
            }
            let (label, value) = line.trim().split_once(':')?;
            let value = bounded_materialized(value);
            (!label.trim().is_empty() && !value.is_empty())
                .then(|| (label.trim().to_string(), value))
        })
        .collect()
}

fn table_cells(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') {
        return Vec::new();
    }
    trimmed
        .trim_matches('|')
        .split('|')
        .map(bounded_materialized)
        .collect()
}

fn is_table_separator(cells: &[String]) -> bool {
    !cells.is_empty()
        && cells
            .iter()
            .all(|cell| !cell.is_empty() && cell.chars().all(|character| character == '-'))
}

fn parse_work_rows(lines: &[String], warnings: &mut Vec<String>) -> Vec<MaterializedWorkRow> {
    let mut rows = Vec::new();
    for cells in lines.iter().map(|line| table_cells(line)) {
        if cells.len() < 5 || is_table_separator(&cells) {
            continue;
        }
        if cells[0].eq_ignore_ascii_case("id") {
            continue;
        }
        rows.push(MaterializedWorkRow {
            id: cells[0].clone(),
            item: cells[1].clone(),
            status: cells[2].clone(),
            owner_actor: cells[3].clone(),
            evidence_source: cells[4].clone(),
        });
        if rows.len() == MAX_MATERIALIZED_ITEMS {
            break;
        }
    }
    if lines.iter().any(|line| line.contains('|')) && rows.is_empty() {
        push_warning(
            warnings,
            "malformed materialized Current work table ignored",
        );
    }
    rows
}

fn parse_bounded_items(lines: &[String], limit: usize) -> Vec<String> {
    let mut items = Vec::new();
    for line in lines {
        let value = if let Some(cells) = (!table_cells(line).is_empty()).then(|| table_cells(line))
        {
            if cells.len() >= 2 && !is_table_separator(&cells) {
                format!("{}: {}", cells[0], cells[1])
            } else {
                String::new()
            }
        } else {
            line.trim()
                .strip_prefix('-')
                .unwrap_or(line.trim())
                .trim()
                .to_string()
        };
        let value = bounded_materialized(&value);
        if value.is_empty() || value.eq_ignore_ascii_case("none verified") {
            continue;
        }
        items.push(value);
        if items.len() == limit {
            break;
        }
    }
    items
}

fn parse_bounded_facts(lines: &[String], limit: usize) -> Vec<MaterializedFact> {
    let mut facts = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let cells = table_cells(line);
        if cells.len() >= 3
            && cells[0].eq_ignore_ascii_case("check")
            && cells[1].eq_ignore_ascii_case("result")
            && cells[2].eq_ignore_ascii_case("evidence")
        {
            continue;
        }
        let pair = if cells.len() >= 2 && !is_table_separator(&cells) {
            Some((cells[0].clone(), cells[1].clone()))
        } else {
            line.trim()
                .strip_prefix('-')
                .unwrap_or(line.trim())
                .split_once(':')
                .map(|(label, value)| (bounded_materialized(label), bounded_materialized(value)))
        };
        let Some((label, value)) = pair else { continue };
        if label.is_empty()
            || value.is_empty()
            || (label.eq_ignore_ascii_case("check") && value.eq_ignore_ascii_case("result"))
            || label.eq_ignore_ascii_case("field")
            || label.eq_ignore_ascii_case("role")
            || label.eq_ignore_ascii_case("source")
            || is_table_separator(&[label.clone(), value.clone()])
        {
            continue;
        }
        facts.push(MaterializedFact {
            label: if label.is_empty() {
                format!("item-{index}")
            } else {
                label
            },
            value,
        });
        if facts.len() == limit {
            break;
        }
    }
    facts
}

fn parse_progress_percent(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    if let Some(percent) = trimmed.strip_suffix('%') {
        return percent
            .trim()
            .parse::<u32>()
            .ok()
            .filter(|value| *value <= 100);
    }
    let mut parts = trimmed.split('/').map(str::trim);
    let numerator = parts.next()?.parse::<u32>().ok()?;
    let denominator = parts.next()?.parse::<u32>().ok()?;
    if denominator == 0 || parts.next().is_some() || numerator > denominator {
        return None;
    }
    Some(numerator.saturating_mul(100) / denominator)
}

fn bounded_materialized(value: &str) -> String {
    let mut bounded = value.trim().trim_matches('`').trim().to_string();
    while bounded.len() > MAX_WARNING_SCALAR_BYTES {
        bounded.pop();
    }
    bounded
}

fn clean_scalar(value: &str) -> String {
    value
        .trim()
        .trim_matches('`')
        .trim_matches('"')
        .trim_matches('\'')
        .trim()
        .to_string()
}

fn normalize_relative_path(value: &str) -> (String, bool) {
    let value = value.trim().replace('\\', "/");
    let invalid = value.is_empty()
        || value.as_bytes().len() > MAX_SOURCE_PATH_BYTES
        || value.starts_with('/')
        || value.starts_with("//")
        || value
            .as_bytes()
            .iter()
            .any(|byte| *byte == 0 || *byte < 0x20)
        || value.as_bytes().get(1) == Some(&b':');
    if invalid {
        return (value, false);
    }
    let mut parts = Vec::new();
    for component in Path::new(&value).components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return (value, false)
            }
        }
    }
    let normalized = parts.join("/");
    (normalized.clone(), !normalized.is_empty())
}

fn normalize_repository(value: &str) -> String {
    value
        .trim()
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .trim_start_matches("https://github.com/")
        .trim_start_matches("http://github.com/")
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use std::fs;
    use tempfile::tempdir;

    fn fixture() -> (tempfile::TempDir, tempfile::TempDir, DatabaseState, String) {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(
            project_dir.path().join("TASKS.md"),
            "# Work\n- [ ] canonical\n",
        )
        .unwrap();
        fs::create_dir_all(project_dir.path().join(".hiveai")).unwrap();
        let db = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project = register_project(
            &db,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Dashboard fixture".into()),
            },
        )
        .unwrap();
        (db_dir, project_dir, db, project.id)
    }

    #[test]
    fn manifest_parser_accepts_v1_roles_and_ignores_prose() {
        let parsed = parse_manifest("hiveaiDashboardSchema: hiveai-project-dashboard/v1\nprojectKey: demo\nrepository: owner/demo\ndashboardMode: source-map\n## Source authorities\nCanonical task source: `H!veAI/TASKS.md`\nRoadmap source: workspace manifests where present `ROADMAP.md`\n").unwrap();
        assert_eq!(parsed.roles["canonicalTask"], vec!["H!veAI/TASKS.md"]);
        assert_eq!(parsed.roles["roadmap"], vec!["ROADMAP.md"]);
    }

    #[test]
    fn manifest_parser_treats_none_verified_as_no_authority() {
        let parsed = parse_manifest("hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Source authorities\nCanonical task source: none verified at repository root\n").unwrap();
        assert!(!parsed.roles.contains_key("canonicalTask"));
    }

    #[test]
    fn materialized_colons_do_not_consume_front_matter_budget() {
        let mut manifest = String::from(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Source authorities\nCanonical task source: `TASKS.md`\n## Recent meaningful activity\n",
        );
        for index in 0..40 {
            manifest.push_str(&format!("- Activity {index}: detail with a colon\n"));
        }
        assert!(parse_manifest(&manifest).is_ok());
    }

    #[test]
    fn genuinely_excessive_header_fields_still_fail_closed() {
        let mut manifest = String::from(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n",
        );
        for index in 0..31 {
            manifest.push_str(&format!("header-{index}: value\n"));
        }
        let error = parse_manifest(&manifest).unwrap_err();
        assert!(error.contains("front-matter field limit reached"));
    }

    #[test]
    fn materialized_enum_values_normalize_and_invalid_values_become_unknown() {
        let parsed = parse_manifest(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## H!veAI live status\nProject status: waiting\nHealth: SUPER_HEALTHY\nRequired actor: human\n",
        )
        .unwrap();
        assert_eq!(
            parsed.materialized.project_status.as_deref(),
            Some("WAITING")
        );
        assert_eq!(parsed.materialized.health.as_deref(), Some("UNKNOWN"));
        assert_eq!(parsed.materialized.required_actor.as_deref(), Some("HUMAN"));
        assert!(parsed
            .materialized_warnings
            .iter()
            .any(|warning| warning.contains("invalid materialized Health")));
    }

    #[test]
    fn quality_table_header_is_not_a_materialized_fact() {
        let parsed = parse_manifest(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Quality and verification\n| Check | Result | Evidence |\n| --- | --- | --- |\n| Native tests | PASS | cargo test |\n",
        )
        .unwrap();
        assert_eq!(parsed.materialized.quality_verification.len(), 1);
        assert_eq!(
            parsed.materialized.quality_verification[0].label,
            "Native tests"
        );
        assert_eq!(parsed.materialized.quality_verification[0].value, "PASS");
    }

    #[test]
    fn paths_reject_traversal_absolute_and_drive_qualified_values() {
        for path in [
            "../TASKS.md",
            "/tmp/TASKS.md",
            "C:/TASKS.md",
            "\\\\server\\share",
        ] {
            assert!(!normalize_relative_path(path).1, "{path}");
        }
    }

    #[test]
    fn checkbox_syntax_is_only_a_warning() {
        let parsed = parse_manifest("hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n[ ] pointer note\n").unwrap();
        assert_eq!(parsed.warnings.len(), 1);
    }

    #[test]
    fn resolver_accepts_contained_canonical_source() {
        let (_db_dir, project_dir, db, project_id) = fixture();
        fs::write(project_dir.path().join(MANIFEST_RELATIVE_PATH), "hiveaiDashboardSchema: hiveai-project-dashboard/v1\nprojectKey: dashboard-fixture\ndashboardMode: source-map\n## Source authorities\nCanonical task source: `TASKS.md`\n").unwrap();
        let resolution = resolve(&db, &project_id).unwrap();
        assert_eq!(resolution.manifest_status, ManifestStatus::Valid);
        assert_eq!(resolution.task_authority, TaskAuthorityState::Canonical);
        assert_eq!(
            resolution.canonical_task_source.as_deref(),
            Some("TASKS.md")
        );
    }

    #[test]
    fn resolver_parses_tracking_mode_and_bounded_materialized_status() {
        let (_db_dir, project_dir, db, project_id) = fixture();
        fs::write(
            project_dir.path().join(MANIFEST_RELATIVE_PATH),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\nrefreshPolicy: project-agent-maintained; H!veAI watches only .hiveai/PROJECT_DASHBOARD.md\n## Source authorities\n- Canonical task source: `TASKS.md`\n## H!veAI live status\n| Field | Value |\n| --- | --- |\n| Project status | ACTIVE |\n| Health | UNKNOWN |\n| Current milestone | M11A REV3 |\n| Current task | Dashboard materialization |\n| Current task ID | M11A.REV3 |\n| Current workflow state | IMPLEMENTATION_COMPLETE_PENDING_AUDIT |\n| Progress | 11/20 |\n| Required actor | CODEX |\n| Next action | Run gates |\n| Waiting on | Audit |\n| Last meaningful update | UNKNOWN |\n## Current work\n| ID | Item | Status | Owner/actor | Evidence/source |\n| --- | --- | --- | --- | --- |\n| one | One bounded item | ACTIVE | CODEX | TASKS.md |\n## Blockers and waiting\n- None verified\n## Provenance\n- Task authority: `TASKS.md`\n",
        )
        .unwrap();
        let resolution = resolve(&db, &project_id).unwrap();
        assert_eq!(
            resolution.tracking_mode.as_deref(),
            Some("single-dashboard-watch")
        );
        assert_eq!(resolution.task_authority, TaskAuthorityState::Canonical);
        assert_eq!(
            resolution.materialized.project_status.as_deref(),
            Some("ACTIVE")
        );
        assert_eq!(resolution.materialized.progress_percent, Some(55));
        assert_eq!(resolution.materialized.current_work.len(), 1);
        assert_eq!(resolution.materialized.blockers_waiting.len(), 0);
    }

    #[test]
    fn hiveai_dogfood_dashboard_is_a_single_watch_contract() {
        let (_db_dir, project_dir, db, project_id) = fixture();
        fs::write(
            project_dir.path().join(MANIFEST_RELATIVE_PATH),
            include_str!("../../.hiveai/PROJECT_DASHBOARD.md"),
        )
        .unwrap();
        let resolution = resolve(&db, &project_id).unwrap();
        assert_eq!(resolution.manifest_status, ManifestStatus::Partial);
        assert_eq!(
            resolution.tracking_mode.as_deref(),
            Some("single-dashboard-watch")
        );
        assert_eq!(
            resolution.materialized.current_milestone.as_deref(),
            Some("M13")
        );
    }

    #[test]
    fn resolver_falls_back_when_manifest_is_absent_or_unverified() {
        let (_db_dir, project_dir, db, project_id) = fixture();
        let absent = resolve(&db, &project_id).unwrap();
        assert_eq!(absent.manifest_status, ManifestStatus::Absent);
        fs::write(project_dir.path().join(MANIFEST_RELATIVE_PATH), "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Source authorities\nCanonical task source: none verified\n").unwrap();
        let unverified = resolve(&db, &project_id).unwrap();
        assert_eq!(
            unverified.task_authority,
            TaskAuthorityState::NotCanonicalized
        );
        assert_eq!(unverified.canonical_task_source, None);
    }

    #[test]
    fn resolver_accepts_directory_backed_history_and_build_roles() {
        let (_db_dir, project_dir, db, project_id) = fixture();
        fs::create_dir_all(project_dir.path().join(".hiveai/history")).unwrap();
        fs::create_dir_all(project_dir.path().join(".hiveai/build")).unwrap();
        fs::write(
            project_dir.path().join(MANIFEST_RELATIVE_PATH),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Source authorities\nCanonical task source: `TASKS.md`\nProgress/history source: `.hiveai/history/`\nBuild/test metadata: `.hiveai/build/`\n",
        )
        .unwrap();
        let resolution = resolve(&db, &project_id).unwrap();
        assert_eq!(resolution.manifest_status, ManifestStatus::Valid);
        assert_eq!(resolution.task_authority, TaskAuthorityState::Canonical);
        assert_eq!(
            resolution.roles["progressHistory"][0].status,
            SourceStatus::Available
        );
        assert_eq!(
            resolution.roles["buildTest"][0].status,
            SourceStatus::Available
        );
    }

    #[test]
    fn resolver_preserves_canonical_authority_when_secondary_directory_is_missing() {
        let (_db_dir, project_dir, db, project_id) = fixture();
        fs::write(
            project_dir.path().join(MANIFEST_RELATIVE_PATH),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Source authorities\nCanonical task source: `TASKS.md`\nProgress/history source: `.hiveai/history/`\n",
        )
        .unwrap();
        let resolution = resolve(&db, &project_id).unwrap();
        assert_eq!(resolution.manifest_status, ManifestStatus::Partial);
        assert_eq!(resolution.task_authority, TaskAuthorityState::Canonical);
        assert_eq!(
            resolution.canonical_task_source.as_deref(),
            Some("TASKS.md")
        );
        assert_eq!(
            resolution.roles["progressHistory"][0].status,
            SourceStatus::Missing
        );
    }

    #[test]
    fn resolver_rejects_directory_as_canonical_task_source() {
        let (_db_dir, project_dir, db, project_id) = fixture();
        fs::remove_file(project_dir.path().join("TASKS.md")).unwrap();
        fs::create_dir_all(project_dir.path().join("TASKS.md")).unwrap();
        fs::write(
            project_dir.path().join(MANIFEST_RELATIVE_PATH),
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Source authorities\nCanonical task source: `TASKS.md`\n",
        )
        .unwrap();
        let resolution = resolve(&db, &project_id).unwrap();
        assert_eq!(resolution.manifest_status, ManifestStatus::Stale);
        assert_eq!(
            resolution.task_authority,
            TaskAuthorityState::FallbackM08M09
        );
        assert_eq!(resolution.canonical_task_source, None);
        assert_eq!(
            resolution.roles["canonicalTask"][0].status,
            SourceStatus::Rejected
        );
    }

    #[test]
    fn warning_scalar_bound_is_utf8_safe() {
        let mut warnings = Vec::new();
        push_warning(&mut warnings, "é".repeat(MAX_WARNING_SCALAR_BYTES));
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].len() <= MAX_WARNING_SCALAR_BYTES);
        assert!(warnings[0].is_char_boundary(warnings[0].len()));
    }
}
