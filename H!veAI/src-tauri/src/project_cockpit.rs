use crate::command_center::{self, ProjectOperationSummary};
use crate::db::DatabaseState;
use crate::git_engine::{
    self, GitDiff, GitDiffRequest, GitDiffScope, GitSnapshot, GitSnapshotRequest,
};
use crate::project_dashboard::{self, ProjectDashboardResolution};
use crate::projects::{fetch_project, ProjectRecord};
use crate::task_intelligence::TaskIntelligenceSnapshot;
use crate::task_sources::{self, DiscoveredProjectSource};
use crate::workflow::{self, WorkflowEvent, WorkflowHistoryQuery, WorkflowProjectList};
use rusqlite::Connection;
use serde::Serialize;

const MAX_COCKPIT_TASKS: usize = 128;
const MAX_COCKPIT_HISTORY: usize = 200;
const MAX_COCKPIT_RECORDS: usize = 100;
const MAX_COCKPIT_ACTIVITY: usize = 150;
const MAX_COCKPIT_FILES: usize = 256;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitTestRun {
    pub id: String,
    pub task_id: Option<String>,
    pub command: String,
    pub result: String,
    pub output_metadata: Option<String>,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitAuditFinding {
    pub id: String,
    pub severity: String,
    pub title: String,
    pub detail: Option<String>,
    pub file_path: Option<String>,
    pub line_number: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitAudit {
    pub id: String,
    pub task_id: Option<String>,
    pub result: String,
    pub summary: Option<String>,
    pub confidence: Option<f64>,
    pub created_at: String,
    pub findings: Vec<CockpitAuditFinding>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitAgentSession {
    pub id: String,
    pub task_id: Option<String>,
    pub provider: String,
    pub state: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitPermission {
    pub id: String,
    pub session_id: Option<String>,
    pub permission_kind: String,
    pub requested_resource: Option<String>,
    pub state: String,
    pub decided_by: Option<String>,
    pub created_at: String,
    pub decided_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitActivity {
    pub id: String,
    pub kind: String,
    pub event: String,
    pub state: Option<String>,
    pub actor: Option<String>,
    pub occurred_at: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitFileEntry {
    pub path: String,
    pub role: String,
    pub status: String,
    pub source_kind: Option<String>,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCockpitSnapshot {
    pub project: ProjectRecord,
    pub project_summary: ProjectOperationSummary,
    pub dashboard: ProjectDashboardResolution,
    pub task_intelligence: Option<TaskIntelligenceSnapshot>,
    pub task_intelligence_error: Option<String>,
    pub workflow: WorkflowProjectList,
    pub workflow_history: Vec<WorkflowEvent>,
    pub git: Option<GitSnapshot>,
    pub git_error: Option<String>,
    pub git_diff: Option<GitDiff>,
    pub git_diff_error: Option<String>,
    pub sources: Vec<DiscoveredProjectSource>,
    pub sources_error: Option<String>,
    pub tests: Vec<CockpitTestRun>,
    pub audits: Vec<CockpitAudit>,
    pub agent_sessions: Vec<CockpitAgentSession>,
    pub permissions: Vec<CockpitPermission>,
    pub activity: Vec<CockpitActivity>,
    pub files: Vec<CockpitFileEntry>,
    pub warnings: Vec<String>,
    pub generated_at: String,
}

pub fn snapshot(
    database: &DatabaseState,
    project_id: &str,
) -> Result<ProjectCockpitSnapshot, String> {
    let project = fetch_project(database, project_id)?;
    let dashboard = project_dashboard::resolve(database, project_id)?;
    let project_summary = command_center::summarize_project_for_cockpit(database, &project)?;
    let (task_intelligence, task_intelligence_error) =
        match crate::task_intelligence::list(database, project_id) {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        };
    let workflow = workflow::project_list(
        database,
        workflow::WorkflowProjectListQuery {
            project_id: project_id.to_string(),
            limit: Some(MAX_COCKPIT_TASKS),
        },
    )?;
    let mut workflow_history = Vec::new();
    let mut warnings = dashboard.warnings.clone();
    for task in workflow.tasks.iter().take(MAX_COCKPIT_TASKS) {
        match workflow::history(
            database,
            WorkflowHistoryQuery {
                task_id: task.task_id.clone(),
                limit: Some(MAX_COCKPIT_HISTORY),
            },
        ) {
            Ok(events) => workflow_history.extend(events),
            Err(error) => push_warning(&mut warnings, &error),
        }
        if workflow_history.len() >= MAX_COCKPIT_HISTORY {
            workflow_history.truncate(MAX_COCKPIT_HISTORY);
            break;
        }
    }
    workflow_history.sort_by(|left, right| {
        left.occurred_at
            .cmp(&right.occurred_at)
            .then(left.id.cmp(&right.id))
    });

    let (git, git_error) = match git_engine::snapshot(
        database,
        GitSnapshotRequest {
            project_id: project_id.to_string(),
            persist: Some(false),
        },
    ) {
        Ok(value) => (Some(value), None),
        Err(error) => (None, Some(error)),
    };
    let (git_diff, git_diff_error) = if git.is_some() {
        match git_engine::diff(
            database,
            GitDiffRequest {
                project_id: project_id.to_string(),
                scope: GitDiffScope::WorkingTree,
            },
        ) {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        }
    } else {
        (None, None)
    };
    let (sources, sources_error) = match task_sources::list(database, project_id) {
        Ok(value) => (value, None),
        Err(error) => (Vec::new(), Some(error)),
    };
    let connection = database.open_connection()?;
    let tests = read_tests(&connection, project_id)?;
    let audits = read_audits(&connection, project_id)?;
    let agent_sessions = read_agent_sessions(&connection, project_id)?;
    let permissions = read_permissions(&connection, project_id)?;
    let activity = build_activity(
        &workflow_history,
        &tests,
        &audits,
        &agent_sessions,
        &dashboard,
    );
    let files = build_files(&sources, &dashboard);
    for error in [
        task_intelligence_error.as_deref(),
        git_error.as_deref(),
        git_diff_error.as_deref(),
        sources_error.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        push_warning(&mut warnings, error);
    }
    Ok(ProjectCockpitSnapshot {
        project,
        project_summary,
        dashboard,
        task_intelligence,
        task_intelligence_error,
        workflow,
        workflow_history,
        git,
        git_error,
        git_diff,
        git_diff_error,
        sources,
        sources_error,
        tests,
        audits,
        agent_sessions,
        permissions,
        activity,
        files,
        warnings,
        generated_at: crate::time::utc_timestamp(),
    })
}

fn read_tests(connection: &Connection, project_id: &str) -> Result<Vec<CockpitTestRun>, String> {
    let mut statement = connection.prepare("SELECT id, task_id, command, result, output_metadata_json, started_at, finished_at FROM test_runs WHERE project_id=?1 ORDER BY started_at DESC, id DESC LIMIT ?2").map_err(db_error)?;
    let rows = statement
        .query_map(
            rusqlite::params![project_id, MAX_COCKPIT_RECORDS as i64],
            |row| {
                Ok(CockpitTestRun {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    command: row.get(2)?,
                    result: row.get(3)?,
                    output_metadata: row.get(4)?,
                    started_at: row.get(5)?,
                    finished_at: row.get(6)?,
                })
            },
        )
        .map_err(db_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_error)
}

fn read_audits(connection: &Connection, project_id: &str) -> Result<Vec<CockpitAudit>, String> {
    let mut statement = connection.prepare("SELECT id, task_id, result, summary, confidence, created_at FROM audits WHERE project_id=?1 ORDER BY created_at DESC, id DESC LIMIT ?2").map_err(db_error)?;
    let rows = statement
        .query_map(
            rusqlite::params![project_id, MAX_COCKPIT_RECORDS as i64],
            |row| {
                Ok(CockpitAudit {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    result: row.get(2)?,
                    summary: row.get(3)?,
                    confidence: row.get(4)?,
                    created_at: row.get(5)?,
                    findings: Vec::new(),
                })
            },
        )
        .map_err(db_error)?;
    let mut audits = rows.collect::<Result<Vec<_>, _>>().map_err(db_error)?;
    let mut finding_statement = connection.prepare("SELECT id, audit_id, severity, title, detail, file_path, line_number, created_at FROM audit_findings WHERE audit_id=?1 ORDER BY created_at ASC, id ASC LIMIT ?2").map_err(db_error)?;
    for audit in &mut audits {
        let findings = finding_statement
            .query_map(
                rusqlite::params![audit.id, MAX_COCKPIT_RECORDS as i64],
                |row| {
                    Ok(CockpitAuditFinding {
                        id: row.get(0)?,
                        severity: row.get(2)?,
                        title: row.get(3)?,
                        detail: row.get(4)?,
                        file_path: row.get(5)?,
                        line_number: row.get(6)?,
                        created_at: row.get(7)?,
                    })
                },
            )
            .map_err(db_error)?;
        audit.findings = findings.collect::<Result<Vec<_>, _>>().map_err(db_error)?;
    }
    Ok(audits)
}

fn read_agent_sessions(
    connection: &Connection,
    project_id: &str,
) -> Result<Vec<CockpitAgentSession>, String> {
    let mut statement = connection.prepare("SELECT id, task_id, provider, state, started_at, ended_at, created_at FROM agent_sessions WHERE project_id=?1 ORDER BY COALESCE(started_at, created_at) DESC, id DESC LIMIT ?2").map_err(db_error)?;
    let rows = statement
        .query_map(
            rusqlite::params![project_id, MAX_COCKPIT_RECORDS as i64],
            |row| {
                Ok(CockpitAgentSession {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    provider: row.get(2)?,
                    state: row.get(3)?,
                    started_at: row.get(4)?,
                    ended_at: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
        .map_err(db_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_error)
}

fn read_permissions(
    connection: &Connection,
    project_id: &str,
) -> Result<Vec<CockpitPermission>, String> {
    let mut statement = connection.prepare("SELECT p.id, p.session_id, p.permission_kind, p.requested_resource, p.state, p.decided_by, p.created_at, p.decided_at FROM permission_requests p LEFT JOIN agent_sessions s ON s.id=p.session_id WHERE s.project_id=?1 ORDER BY p.created_at DESC, p.id DESC LIMIT ?2").map_err(db_error)?;
    let rows = statement
        .query_map(
            rusqlite::params![project_id, MAX_COCKPIT_RECORDS as i64],
            |row| {
                Ok(CockpitPermission {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    permission_kind: row.get(2)?,
                    requested_resource: row.get(3)?,
                    state: row.get(4)?,
                    decided_by: row.get(5)?,
                    created_at: row.get(6)?,
                    decided_at: row.get(7)?,
                })
            },
        )
        .map_err(db_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_error)
}

fn build_activity(
    workflow_history: &[WorkflowEvent],
    tests: &[CockpitTestRun],
    audits: &[CockpitAudit],
    sessions: &[CockpitAgentSession],
    dashboard: &ProjectDashboardResolution,
) -> Vec<CockpitActivity> {
    let mut activity = Vec::new();
    activity.extend(workflow_history.iter().map(|event| CockpitActivity {
        id: format!("workflow:{}", event.id),
        kind: "WORKFLOW".into(),
        event: event.summary.clone(),
        state: event.to_state.map(|state| state.to_string()),
        actor: event.actor_type.map(|actor| actor.to_string()),
        occurred_at: event.occurred_at.clone(),
        source: "M10 task_events".into(),
    }));
    activity.extend(tests.iter().map(|test| CockpitActivity {
        id: format!("test:{}", test.id),
        kind: "TEST_RUN".into(),
        event: format!("{}: {}", test.command, test.result),
        state: Some(test.result.clone()),
        actor: None,
        occurred_at: test.started_at.clone(),
        source: "test_runs".into(),
    }));
    activity.extend(audits.iter().map(|audit| {
        CockpitActivity {
            id: format!("audit:{}", audit.id),
            kind: "AUDIT".into(),
            event: audit
                .summary
                .clone()
                .unwrap_or_else(|| format!("Audit {}", audit.result)),
            state: Some(audit.result.clone()),
            actor: None,
            occurred_at: audit.created_at.clone(),
            source: "audits".into(),
        }
    }));
    activity.extend(sessions.iter().map(|session| {
        CockpitActivity {
            id: format!("agent:{}", session.id),
            kind: "AGENT".into(),
            event: format!("{} session {}", session.provider, session.state),
            state: Some(session.state.clone()),
            actor: Some(session.provider.clone()),
            occurred_at: session
                .started_at
                .clone()
                .unwrap_or_else(|| session.created_at.clone()),
            source: "agent_sessions".into(),
        }
    }));
    activity.extend(
        dashboard
            .materialized
            .recent_meaningful_activity
            .iter()
            .enumerate()
            .map(|(index, event)| CockpitActivity {
                id: format!("dashboard-activity:{index}"),
                kind: "PROJECT_DASHBOARD".into(),
                event: event.clone(),
                state: None,
                actor: None,
                occurred_at: "UNDATED".into(),
                source: project_dashboard::MANIFEST_RELATIVE_PATH.into(),
            }),
    );
    activity.sort_by(|left, right| {
        let left_undated = left.occurred_at == "UNDATED";
        let right_undated = right.occurred_at == "UNDATED";
        left_undated
            .cmp(&right_undated)
            .then(right.occurred_at.cmp(&left.occurred_at))
            .then(left.id.cmp(&right.id))
    });
    activity.truncate(MAX_COCKPIT_ACTIVITY);
    activity
}

fn build_files(
    sources: &[DiscoveredProjectSource],
    dashboard: &ProjectDashboardResolution,
) -> Vec<CockpitFileEntry> {
    let mut files = Vec::new();
    for source in sources {
        files.push(CockpitFileEntry {
            path: source.relative_path.clone(),
            role: source.authority_class.clone(),
            status: source.status.clone(),
            source_kind: Some(source.source_kind.clone()),
            evidence: format!("M08 source inventory; discovered {}", source.discovered_at),
        });
    }
    for (role, resolved) in &dashboard.roles {
        for source in resolved {
            let status = match source.status {
                project_dashboard::SourceStatus::Available => "AVAILABLE",
                project_dashboard::SourceStatus::Missing => "MISSING",
                project_dashboard::SourceStatus::Rejected => "REJECTED",
            };
            files.push(CockpitFileEntry {
                path: source.path.clone(),
                role: role.clone(),
                status: status.into(),
                source_kind: None,
                evidence: "Project Dashboard authority map".into(),
            });
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path).then(left.role.cmp(&right.role)));
    files.dedup_by(|left, right| left.path == right.path && left.role == right.role);
    files.truncate(MAX_COCKPIT_FILES);
    files
}

fn push_warning(warnings: &mut Vec<String>, message: &str) {
    if !warnings.iter().any(|existing| existing == message) {
        warnings.push(message.to_string());
    }
}

fn db_error(error: rusqlite::Error) -> String {
    format!("project cockpit database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use tempfile::tempdir;

    #[test]
    fn m12_snapshot_is_project_scoped_and_unknowns_are_explicit() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_a_dir = tempdir().unwrap();
        let project_b_dir = tempdir().unwrap();
        let project_a = register_project(
            &database,
            RegisterProjectRequest {
                path: project_a_dir.path().to_string_lossy().into_owned(),
                name: Some("Project A".into()),
            },
        )
        .unwrap();
        let project_b = register_project(
            &database,
            RegisterProjectRequest {
                path: project_b_dir.path().to_string_lossy().into_owned(),
                name: Some("Project B".into()),
            },
        )
        .unwrap();
        let snapshot = snapshot(&database, &project_a.id).unwrap();
        assert_eq!(snapshot.project.id, project_a.id);
        assert_ne!(snapshot.project.id, project_b.id);
        assert_eq!(snapshot.project_summary.project_id, project_a.id);
        assert!(snapshot.git.is_none());
        assert!(snapshot
            .git_error
            .as_deref()
            .unwrap_or_default()
            .contains("NON_GIT_PROJECT"));
        assert!(snapshot
            .activity
            .iter()
            .all(|item| !item.event.contains("Project B")));
    }

    #[test]
    fn m12_git_loading_does_not_persist_a_snapshot() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_dir = tempdir().unwrap();
        let output = std::process::Command::new("git")
            .args(["init"])
            .current_dir(project_dir.path())
            .output()
            .unwrap();
        assert!(output.status.success());
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Git Project".into()),
            },
        )
        .unwrap();
        let snapshot = snapshot(&database, &project.id).unwrap();
        assert!(snapshot.git.is_some());
        let connection = database.open_connection().unwrap();
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM git_snapshots", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn m12_project_dashboard_authority_maps_never_cross_projects() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_a_dir = tempdir().unwrap();
        let project_b_dir = tempdir().unwrap();
        for (directory, key, task) in [
            (&project_a_dir, "project-a", "Project A canonical task"),
            (&project_b_dir, "project-b", "Project B canonical task"),
        ] {
            std::fs::create_dir_all(directory.path().join(".hiveai")).unwrap();
            std::fs::write(
                directory.path().join(project_dashboard::MANIFEST_RELATIVE_PATH),
                format!(
                    "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\nprojectKey: {key}\n## Source authorities\n- Canonical task source: `TASKS.md`\n## H!veAI live status\n| Field | Value |\n| --- | --- |\n| Project status | ACTIVE |\n| Current milestone | M12 |\n| Current task | {task} |\n| Current task ID | {key}-task |\n| Current workflow state | IN_PROGRESS |\n| Progress | 25% |\n| Required actor | CODEX |\n| Next action | Continue {key} |\n| Last meaningful update | 2026-08-27 |\n## Recent meaningful activity\n- {key} activity\n"
                ),
            )
            .unwrap();
        }
        let project_a = register_project(
            &database,
            RegisterProjectRequest {
                path: project_a_dir.path().to_string_lossy().into_owned(),
                name: Some("Project A".into()),
            },
        )
        .unwrap();
        let _project_b = register_project(
            &database,
            RegisterProjectRequest {
                path: project_b_dir.path().to_string_lossy().into_owned(),
                name: Some("Project B".into()),
            },
        )
        .unwrap();
        let snapshot = snapshot(&database, &project_a.id).unwrap();
        assert_eq!(snapshot.dashboard.project_key.as_deref(), Some("project-a"));
        assert_eq!(
            snapshot
                .dashboard
                .materialized
                .current_task_title
                .as_deref(),
            Some("Project A canonical task")
        );
        assert!(snapshot
            .dashboard
            .materialized
            .recent_meaningful_activity
            .iter()
            .all(|event| !event.contains("project-b") && !event.contains("Project B")));
        assert!(snapshot
            .files
            .iter()
            .all(|file| !file.evidence.contains("Project B")));
    }

    #[test]
    fn m12_missing_and_archived_states_remain_explicit() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let missing_dir = tempdir().unwrap();
        let missing_project = register_project(
            &database,
            RegisterProjectRequest {
                path: missing_dir.path().to_string_lossy().into_owned(),
                name: Some("Missing Project".into()),
            },
        )
        .unwrap();
        let missing_path = missing_dir.path().to_path_buf();
        drop(missing_dir);
        let missing_snapshot = snapshot(&database, &missing_project.id).unwrap();
        assert_eq!(missing_snapshot.project.status, "MISSING");
        assert!(missing_snapshot.git.is_none());
        assert!(missing_snapshot
            .warnings
            .iter()
            .any(|warning| warning.contains("MISSING") || warning.contains("unavailable")));

        let archived_dir = tempdir().unwrap();
        let archived_project = register_project(
            &database,
            RegisterProjectRequest {
                path: archived_dir.path().to_string_lossy().into_owned(),
                name: Some("Archived Project".into()),
            },
        )
        .unwrap();
        crate::projects::archive_project(&database, &archived_project.id).unwrap();
        let archived_snapshot = snapshot(&database, &archived_project.id).unwrap();
        assert_eq!(archived_snapshot.project.status, "ARCHIVED");
        assert!(archived_snapshot.task_intelligence.is_none());
        assert!(archived_snapshot
            .task_intelligence_error
            .as_deref()
            .unwrap_or_default()
            .contains("not been parsed"));
        assert!(!archived_snapshot
            .warnings
            .iter()
            .any(|warning| warning.contains(missing_path.to_string_lossy().as_ref())));
    }
}
