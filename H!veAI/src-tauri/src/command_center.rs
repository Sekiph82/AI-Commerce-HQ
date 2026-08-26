use crate::db::DatabaseState;
use crate::project_dashboard::{self, ProjectDashboardResolution, TaskAuthorityState};
use crate::projects::{list_projects, ProjectListQuery, ProjectRecord};
use crate::task_intelligence::{self, ParsedTask, TaskIntelligenceSnapshot};
use crate::workflow::{self, WorkflowProjectList, WorkflowState, WorkflowTask};
use serde::Serialize;
use std::collections::HashSet;

pub const MAX_VISIBLE_PROJECTS: usize = 128;
pub const DEFAULT_ACTIVITY_LIMIT: usize = 50;
pub const MAX_ACTIVITY_LIMIT: usize = 200;
pub const MAX_ATTENTION_ITEMS: usize = 100;
pub const MAX_QUEUE_ITEMS: usize = 100;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandCenterSnapshot {
    pub generated_at: String,
    pub projects: Vec<ProjectOperationSummary>,
    pub kpis: PortfolioKpis,
    pub attention: Vec<AttentionItem>,
    pub work_queue: Vec<WorkQueueItem>,
    pub recent_activity: Vec<ActivityItem>,
    pub engineering_brief: EngineeringBrief,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioKpis {
    pub projects: usize,
    pub active_tasks: Option<usize>,
    pub needs_attention: usize,
    pub running: usize,
    pub completed_tasks: Option<usize>,
    pub healthy: usize,
    pub health_detail: String,
    pub authority_detail: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOperationSummary {
    pub project_id: String,
    pub name: String,
    pub registry_status: String,
    pub health: String,
    pub manifest_status: String,
    pub task_authority: String,
    pub provenance_mode: String,
    pub canonical_task_source: Option<String>,
    pub current_task: Option<TaskSummary>,
    pub current_state: Option<String>,
    pub last_action: Option<ActionSummary>,
    pub next_action: Option<String>,
    pub allowed_actors: Vec<String>,
    pub total_tasks: Option<usize>,
    pub active_tasks: Option<usize>,
    pub completed_tasks: Option<usize>,
    pub progress_percent: Option<u8>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSummary {
    pub task_id: String,
    pub title: String,
    pub source_path: String,
    pub parsed_status: String,
    pub workflow_state: Option<String>,
    pub required_actor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSummary {
    pub summary: String,
    pub occurred_at: String,
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttentionItem {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub task_id: Option<String>,
    pub title: String,
    pub state: String,
    pub detail: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkQueueItem {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub task_id: String,
    pub task: String,
    pub stage: String,
    pub state: String,
    pub actor: Option<String>,
    pub updated_at: Option<String>,
    pub attention: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItem {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub kind: String,
    pub event: String,
    pub state: Option<String>,
    pub actor: Option<String>,
    pub occurred_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BriefFact {
    pub label: String,
    pub value: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineeringBrief {
    pub facts: Vec<BriefFact>,
    pub recommendation: Option<String>,
}

pub fn snapshot(database: &DatabaseState) -> Result<CommandCenterSnapshot, String> {
    let mut projects = list_projects(
        database,
        ProjectListQuery {
            include_archived: Some(false),
            ..Default::default()
        },
    )?;
    projects.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
            .then(left.id.cmp(&right.id))
    });
    let mut warnings = Vec::new();
    if projects.len() > MAX_VISIBLE_PROJECTS {
        warnings.push(format!(
            "visible project limit reached ({MAX_VISIBLE_PROJECTS})"
        ));
        projects.truncate(MAX_VISIBLE_PROJECTS);
    }
    let mut summaries = Vec::new();
    let mut attention = Vec::new();
    let mut queue = Vec::new();
    let mut activity = read_activity(database, DEFAULT_ACTIVITY_LIMIT)?;
    for project in projects {
        let (summary, project_attention, project_queue, mut project_warnings) =
            summarize_project(database, &project)?;
        attention.extend(project_attention);
        queue.extend(project_queue);
        warnings.append(&mut project_warnings);
        summaries.push(summary);
    }
    attention.truncate(MAX_ATTENTION_ITEMS);
    queue.sort_by(|left, right| {
        right
            .updated_at
            .cmp(&left.updated_at)
            .then(left.id.cmp(&right.id))
    });
    queue.truncate(MAX_QUEUE_ITEMS);
    activity.truncate(MAX_ACTIVITY_LIMIT);
    let active_tasks = summaries
        .iter()
        .filter_map(|summary| summary.active_tasks)
        .sum::<usize>();
    let completed_tasks = summaries
        .iter()
        .filter_map(|summary| summary.completed_tasks)
        .sum::<usize>();
    let known_task_projects = summaries
        .iter()
        .filter(|summary| summary.task_authority != "NOT_CANONICALIZED")
        .count();
    let healthy = summaries
        .iter()
        .filter(|summary| summary.health == "HEALTHY")
        .count();
    let active_tasks_known = summaries
        .iter()
        .all(|summary| summary.active_tasks.is_some());
    let completed_tasks_known = summaries
        .iter()
        .all(|summary| summary.completed_tasks.is_some());
    let authority_detail = format!(
        "{} project{} without canonical task authority",
        summaries.len().saturating_sub(known_task_projects),
        if summaries.len().saturating_sub(known_task_projects) == 1 {
            ""
        } else {
            "s"
        }
    );
    let health_detail = format!("{healthy} / {} healthy", summaries.len());
    let mut facts = vec![
        BriefFact {
            label: "Registered projects".into(),
            value: summaries.len().to_string(),
            source: "Project Registry".into(),
        },
        BriefFact {
            label: "Authoritative active tasks".into(),
            value: if active_tasks_known {
                active_tasks.to_string()
            } else {
                "Unavailable".into()
            },
            source: "M09 + Project Dashboard authority".into(),
        },
        BriefFact {
            label: "Needs attention".into(),
            value: attention.len().to_string(),
            source: "M10 workflow + Registry".into(),
        },
        BriefFact {
            label: "Running workflow tasks".into(),
            value: queue
                .iter()
                .filter(|item| is_running_state(&item.state))
                .count()
                .to_string(),
            source: "M10 task state".into(),
        },
    ];
    facts.truncate(8);
    Ok(CommandCenterSnapshot {
        generated_at: crate::time::utc_timestamp(),
        projects: summaries,
        kpis: PortfolioKpis {
            projects: facts
                .iter()
                .find(|fact| fact.label == "Registered projects")
                .and_then(|fact| fact.value.parse().ok())
                .unwrap_or_default(),
            active_tasks: active_tasks_known.then_some(active_tasks),
            needs_attention: attention.len(),
            running: queue
                .iter()
                .filter(|item| is_running_state(&item.state))
                .count(),
            completed_tasks: completed_tasks_known.then_some(completed_tasks),
            healthy,
            health_detail,
            authority_detail,
        },
        attention,
        work_queue: queue,
        recent_activity: activity,
        engineering_brief: EngineeringBrief {
            facts,
            recommendation: None,
        },
        warnings,
    })
}

pub fn resolve_project(
    database: &DatabaseState,
    project_id: &str,
) -> Result<ProjectDashboardResolution, String> {
    project_dashboard::resolve(database, project_id)
}

fn summarize_project(
    database: &DatabaseState,
    project: &ProjectRecord,
) -> Result<
    (
        ProjectOperationSummary,
        Vec<AttentionItem>,
        Vec<WorkQueueItem>,
        Vec<String>,
    ),
    String,
> {
    let dashboard = project_dashboard::resolve(database, &project.id)?;
    let mut warnings = dashboard.warnings.clone();
    let intelligence = task_intelligence::list(database, &project.id).ok();
    if intelligence.is_none() && project.status == "ACTIVE" {
        warnings.push("M09 task intelligence has not been parsed for this project".into());
    }
    let workflows = workflow::project_list(
        database,
        workflow::WorkflowProjectListQuery {
            project_id: project.id.clone(),
            limit: Some(4096),
        },
    )
    .unwrap_or(WorkflowProjectList {
        project_id: project.id.clone(),
        tasks: Vec::new(),
    });
    let tasks = authoritative_tasks(&dashboard, intelligence.as_ref());
    let task_ids = tasks
        .iter()
        .map(|task| task.id.as_str())
        .collect::<HashSet<_>>();
    let workflow_tasks = workflows
        .tasks
        .into_iter()
        .filter(|task| task_ids.contains(task.task_id.as_str()) && task.source_active)
        .collect::<Vec<_>>();
    let completed = tasks
        .iter()
        .filter(|task| {
            task_is_complete(
                task,
                workflow_tasks
                    .iter()
                    .find(|workflow| workflow.task_id == task.id),
            )
        })
        .count();
    let active = tasks.len().saturating_sub(completed);
    let task_authority = task_authority_name(&dashboard.task_authority);
    let total_tasks = match dashboard.task_authority {
        TaskAuthorityState::NotCanonicalized => None,
        _ => Some(tasks.len()),
    };
    let completed_tasks = total_tasks.map(|_| completed);
    let active_tasks = total_tasks.map(|_| active);
    let current_workflow = select_current_workflow(&tasks, &workflow_tasks);
    let current_task = current_workflow
        .as_ref()
        .and_then(|workflow| tasks.iter().find(|task| task.id == workflow.task_id))
        .or_else(|| tasks.iter().find(|task| !task_is_complete(task, None)));
    let current_state = current_workflow
        .as_ref()
        .map(|task| task.current_state.to_string());
    let last_action = current_workflow
        .as_ref()
        .and_then(|task| task.latest_event.as_ref())
        .map(|event| ActionSummary {
            summary: event.summary.clone(),
            occurred_at: event.occurred_at.clone(),
            actor: event.actor_type.map(|actor| actor.to_string()),
        });
    let allowed_actors = current_workflow
        .as_ref()
        .map(|task| {
            task.allowed_actors
                .iter()
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default();
    let next_action = current_workflow
        .as_ref()
        .and_then(|task| task.allowed_next_states.first())
        .map(|state| format!("Advance to {state}"));
    let health = project_health(project, &dashboard, &workflow_tasks);
    let mut project_attention = Vec::new();
    let mut queue = Vec::new();
    for workflow in &workflow_tasks {
        if attention_state(workflow.current_state) {
            project_attention.push(AttentionItem {
                id: format!("workflow:{}", workflow.task_id),
                project_id: project.id.clone(),
                project_name: project.name.clone(),
                task_id: Some(workflow.task_id.clone()),
                title: workflow.title.clone(),
                state: workflow.current_state.to_string(),
                detail: workflow
                    .latest_event
                    .as_ref()
                    .map(|event| event.summary.clone())
                    .unwrap_or_else(|| "Workflow state requires attention".into()),
                category: "WORKFLOW".into(),
            });
        }
        if workflow.current_state.is_running()
            || workflow.current_state == WorkflowState::VerifyRequired
        {
            queue.push(WorkQueueItem {
                id: format!("queue:{}", workflow.task_id),
                project_id: project.id.clone(),
                project_name: project.name.clone(),
                task_id: workflow.task_id.clone(),
                task: workflow.title.clone(),
                stage: workflow.current_state.to_string(),
                state: workflow.current_state.to_string(),
                actor: workflow
                    .latest_event
                    .as_ref()
                    .and_then(|event| event.actor_type.map(|actor| actor.to_string())),
                updated_at: workflow
                    .latest_event
                    .as_ref()
                    .map(|event| event.occurred_at.clone()),
                attention: workflow.attention_required,
            });
        }
    }
    if project.status == "MISSING" {
        project_attention.push(AttentionItem {
            id: format!("project:{}", project.id),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id: None,
            title: "Registered project root unavailable".into(),
            state: "MISSING".into(),
            detail: "Repair the registered path before project operations resume.".into(),
            category: "REGISTRY".into(),
        });
    }
    for (index, warning) in dashboard.warnings.iter().take(4).enumerate() {
        project_attention.push(AttentionItem {
            id: format!("manifest:{}:{index}", project.id),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id: None,
            title: "Project Dashboard authority warning".into(),
            state: dashboard.manifest_status.clone().into_serialized(),
            detail: warning.clone(),
            category: "PROJECT_DASHBOARD".into(),
        });
    }
    let summary = ProjectOperationSummary {
        project_id: project.id.clone(),
        name: project.name.clone(),
        registry_status: project.status.clone(),
        health,
        manifest_status: dashboard.manifest_status.clone().into_serialized(),
        task_authority,
        provenance_mode: dashboard.provenance_mode.clone(),
        canonical_task_source: dashboard.canonical_task_source.clone(),
        current_task: current_task.map(|task| TaskSummary {
            task_id: task.id.clone(),
            title: task.title.clone(),
            source_path: task.source_path.clone(),
            parsed_status: task.parsed_status.clone(),
            workflow_state: workflow_tasks
                .iter()
                .find(|workflow| workflow.task_id == task.id)
                .map(|workflow| workflow.current_state.to_string()),
            required_actor: task.required_actor.clone(),
        }),
        current_state,
        last_action,
        next_action,
        allowed_actors,
        total_tasks,
        active_tasks,
        completed_tasks,
        progress_percent: progress_percent(completed_tasks, total_tasks),
        warnings,
    };
    Ok((summary, project_attention, queue, Vec::new()))
}

fn authoritative_tasks<'a>(
    dashboard: &ProjectDashboardResolution,
    intelligence: Option<&'a TaskIntelligenceSnapshot>,
) -> Vec<&'a ParsedTask> {
    let Some(snapshot) = intelligence else {
        return Vec::new();
    };
    let mut ids = HashSet::new();
    snapshot
        .tasks
        .iter()
        .filter(|task| match dashboard.task_authority {
            TaskAuthorityState::NotCanonicalized => false,
            TaskAuthorityState::Canonical => dashboard
                .canonical_task_source
                .as_deref()
                .map(|path| same_path(path, &task.source_path))
                .unwrap_or(false),
            TaskAuthorityState::FallbackM08M09 => true,
        })
        .filter(|task| ids.insert(task.id.clone()))
        .collect()
}

fn select_current_workflow<'a>(
    tasks: &[&ParsedTask],
    workflows: &'a [WorkflowTask],
) -> Option<&'a WorkflowTask> {
    let active_ids = tasks
        .iter()
        .filter(|task| !task_is_complete(task, None))
        .map(|task| task.id.as_str())
        .collect::<HashSet<_>>();
    workflows
        .iter()
        .filter(|task| active_ids.contains(task.task_id.as_str()))
        .min_by(|left, right| {
            let left_attention = !left.attention_required;
            let right_attention = !right.attention_required;
            left_attention
                .cmp(&right_attention)
                .then_with(|| {
                    right
                        .latest_event
                        .as_ref()
                        .map(|event| event.occurred_at.as_str())
                        .unwrap_or("")
                        .cmp(
                            left.latest_event
                                .as_ref()
                                .map(|event| event.occurred_at.as_str())
                                .unwrap_or(""),
                        )
                })
                .then(left.task_id.cmp(&right.task_id))
        })
}

fn task_is_complete(task: &ParsedTask, workflow: Option<&WorkflowTask>) -> bool {
    workflow
        .map(|workflow| workflow.current_state == WorkflowState::TaskComplete)
        .unwrap_or(false)
        || matches!(
            task.parsed_status.to_ascii_uppercase().as_str(),
            "COMPLETE" | "COMPLETED" | "DONE" | "TASK_COMPLETE"
        )
}

fn project_health(
    project: &ProjectRecord,
    dashboard: &ProjectDashboardResolution,
    workflows: &[WorkflowTask],
) -> String {
    if project.status == "MISSING" {
        return "MISSING".into();
    }
    if workflows.iter().any(|task| {
        matches!(
            task.current_state,
            WorkflowState::Blocked | WorkflowState::AuditFailed | WorkflowState::FixRequired
        )
    }) {
        return "BLOCKED".into();
    }
    if workflows
        .iter()
        .any(|task| attention_state(task.current_state))
        || !dashboard.warnings.is_empty()
    {
        return "ATTENTION".into();
    }
    if workflows.iter().any(|task| task.current_state.is_running()) {
        return "RUNNING".into();
    }
    if dashboard.task_authority == TaskAuthorityState::FallbackM08M09
        && dashboard.manifest_status != project_dashboard::ManifestStatus::Absent
    {
        return "UNKNOWN".into();
    }
    "HEALTHY".into()
}

fn attention_state(state: WorkflowState) -> bool {
    matches!(
        state,
        WorkflowState::WaitingHuman
            | WorkflowState::WaitingExternal
            | WorkflowState::DesignGate
            | WorkflowState::Blocked
            | WorkflowState::AuditFailed
            | WorkflowState::FixRequired
            | WorkflowState::AuditRequired
            | WorkflowState::VerifyRequired
    )
}
fn is_running_state(state: &str) -> bool {
    matches!(
        state,
        "BUILDER_RUNNING" | "AUDIT_RUNNING" | "VERIFY_RUNNING"
    )
}
fn task_authority_name(state: &TaskAuthorityState) -> String {
    match state {
        TaskAuthorityState::Canonical => "CANONICAL",
        TaskAuthorityState::NotCanonicalized => "NOT_CANONICALIZED",
        TaskAuthorityState::FallbackM08M09 => "FALLBACK_M08_M09",
    }
    .into()
}
fn progress_percent(completed: Option<usize>, total: Option<usize>) -> Option<u8> {
    total.filter(|total| *total > 0).and_then(|total| {
        completed.map(|completed| ((completed.saturating_mul(100) / total).min(100)) as u8)
    })
}
fn same_path(left: &str, right: &str) -> bool {
    left.replace('\\', "/")
        .trim_matches('/')
        .to_ascii_lowercase()
        == right
            .replace('\\', "/")
            .trim_matches('/')
            .to_ascii_lowercase()
}

fn read_activity(database: &DatabaseState, limit: usize) -> Result<Vec<ActivityItem>, String> {
    let limit = limit.clamp(1, MAX_ACTIVITY_LIMIT);
    let connection = database.open_connection()?;
    let mut statement = connection.prepare("SELECT e.id, t.project_id, p.name, e.event_type, e.summary, e.to_state, e.actor_type, e.occurred_at FROM task_events e JOIN tasks t ON t.id=e.task_id JOIN projects p ON p.id=t.project_id WHERE p.status != 'ARCHIVED' ORDER BY e.occurred_at DESC, e.id DESC LIMIT ?1").map_err(|e| format!("read recent workflow activity: {e}"))?;
    let rows = statement
        .query_map([limit as i64], |row| {
            Ok(ActivityItem {
                id: row.get(0)?,
                project_id: row.get(1)?,
                project_name: row.get(2)?,
                kind: row.get(3)?,
                event: row.get(4)?,
                state: row.get(5)?,
                actor: row.get(6)?,
                occurred_at: row.get(7)?,
            })
        })
        .map_err(|e| format!("read recent workflow activity: {e}"))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("read recent workflow activity: {e}"))
}

trait SerializedStatus {
    fn into_serialized(&self) -> String;
}
impl SerializedStatus for project_dashboard::ManifestStatus {
    fn into_serialized(&self) -> String {
        match self {
            project_dashboard::ManifestStatus::Valid => "VALID",
            project_dashboard::ManifestStatus::Partial => "PARTIAL",
            project_dashboard::ManifestStatus::Absent => "ABSENT",
            project_dashboard::ManifestStatus::Malformed => "MALFORMED",
            project_dashboard::ManifestStatus::Stale => "STALE",
            project_dashboard::ManifestStatus::Unavailable => "UNAVAILABLE",
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m11_current_task_selection_is_deterministic() {
        assert!(is_running_state("BUILDER_RUNNING"));
        assert!(!is_running_state("READY_FOR_IMPLEMENTATION"));
        assert_eq!(progress_percent(Some(1), Some(4)), Some(25));
    }

    #[test]
    fn m11_project_health_is_categorical_and_evidence_based() {
        assert_eq!(attention_state(WorkflowState::WaitingHuman), true);
        assert_eq!(attention_state(WorkflowState::TaskComplete), false);
    }

    #[test]
    fn m11_portfolio_counts_use_authoritative_tasks_only() {
        assert_eq!(same_path("H!veAI\\TASKS.md", "h!veai/tasks.md"), true);
    }
}
