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
pub struct ProjectDashboardResolution {
    pub project_id: String,
    pub manifest_status: ManifestStatus,
    pub manifest_path: String,
    pub schema: Option<String>,
    pub project_key: Option<String>,
    pub repository: Option<String>,
    pub branch_policy: Option<String>,
    pub dashboard_mode: Option<String>,
    pub refresh_policy: Option<String>,
    pub task_authority: TaskAuthorityState,
    pub canonical_task_source: Option<String>,
    pub roles: BTreeMap<String, Vec<ResolvedSource>>,
    pub provenance_mode: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct ParsedManifest {
    schema: Option<String>,
    project_key: Option<String>,
    repository: Option<String>,
    branch_policy: Option<String>,
    dashboard_mode: Option<String>,
    refresh_policy: Option<String>,
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
        refresh_policy: parsed.refresh_policy,
        task_authority: TaskAuthorityState::NotCanonicalized,
        canonical_task_source: None,
        roles: BTreeMap::new(),
        provenance_mode: "MANIFEST".into(),
        warnings: parsed.warnings,
    };
    let mut extracted = 0usize;
    for (role, paths) in parsed.roles {
        let mut resolved = Vec::new();
        for path in paths {
            extracted += 1;
            if extracted > MAX_SOURCE_PATHS {
                resolution
                    .warnings
                    .push(format!("source path limit reached ({MAX_SOURCE_PATHS})"));
                break;
            }
            let (normalized, valid) = normalize_relative_path(&path);
            if !valid {
                resolution
                    .warnings
                    .push(format!("rejected authority path for {role}"));
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
            let exists = candidate.is_file();
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
            } else {
                SourceStatus::Available
            };
            resolved.push(ResolvedSource {
                path: normalized.clone(),
                role: role.clone(),
                status,
                exists,
                contained,
            });
            if role == "canonicalTask" && resolution.canonical_task_source.is_none() {
                if contained {
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
            resolution
                .warnings
                .push("canonical task source is unavailable or rejected".into());
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
        refresh_policy: None,
        task_authority: TaskAuthorityState::FallbackM08M09,
        canonical_task_source: None,
        roles: BTreeMap::new(),
        provenance_mode: "FALLBACK_M08_M09".into(),
        warnings: Vec::new(),
    }
}

fn rejected_manifest(
    mut base: ProjectDashboardResolution,
    warning: &str,
) -> ProjectDashboardResolution {
    base.manifest_status = ManifestStatus::Malformed;
    base.warnings.push(warning.to_string());
    base
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
    let mut in_authorities = false;
    let mut warnings = Vec::new();
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
            warnings.push(
                "manifest contains task checkbox syntax; pointer checkboxes are ignored".into(),
            );
        }
        if trimmed.eq_ignore_ascii_case("## Source authorities") {
            in_authorities = true;
            continue;
        }
        if trimmed.starts_with("## ") && !trimmed.eq_ignore_ascii_case("## Source authorities") {
            in_authorities = false;
        }
        if !in_authorities {
            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim();
                if matches!(
                    key,
                    "hiveaiDashboardSchema"
                        | "projectKey"
                        | "repository"
                        | "branchPolicy"
                        | "dashboardMode"
                        | "refreshPolicy"
                ) {
                    if fields.len() >= MAX_FRONT_MATTER_FIELDS {
                        return Err(format!(
                            "front-matter field limit reached ({MAX_FRONT_MATTER_FIELDS})"
                        ));
                    }
                    fields.insert(key.to_string(), clean_scalar(value));
                }
            }
            continue;
        }
        let Some((label, value)) = trimmed.split_once(':') else {
            continue;
        };
        let role = match label.trim().to_ascii_lowercase().as_str() {
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
        let entry = roles.entry(role.into()).or_default();
        if entry.len().saturating_add(paths.len()) > MAX_SOURCE_PATHS_PER_ROLE {
            return Err(format!("source path limit reached for {label}"));
        }
        entry.extend(paths);
    }
    let schema = fields.get("hiveaiDashboardSchema").cloned();
    if schema.as_deref() != Some("hiveai-project-dashboard/v1") {
        return Err("unsupported or missing hiveaiDashboardSchema".into());
    }
    if fields.get("dashboardMode").map(String::as_str) != Some("source-map") {
        return Err("dashboardMode must be source-map".into());
    }
    Ok(ParsedManifest {
        schema,
        project_key: fields.remove("projectKey"),
        repository: fields.remove("repository"),
        branch_policy: fields.remove("branchPolicy"),
        dashboard_mode: fields.remove("dashboardMode"),
        refresh_policy: fields.remove("refreshPolicy"),
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
}
