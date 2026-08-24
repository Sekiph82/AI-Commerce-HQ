mod mutation;

use crate::db::DatabaseState;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

pub use mutation::{mutation_status, MutationStatus};

const COMMAND_TIMEOUT: Duration = Duration::from_secs(8);
const MAX_COMMAND_OUTPUT: usize = 512 * 1024;
const MAX_DIFF_BYTES: usize = 96 * 1024;
const MAX_DIFF_LINES: usize = 1200;
const MAX_RECENT_COMMITS: usize = 25;
const MAX_WORKTREES: usize = 50;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitSnapshot {
    pub project_id: String,
    pub repository_id: String,
    pub repository_path: String,
    pub current_branch: Option<String>,
    pub detached_head: bool,
    pub head_sha: Option<String>,
    pub staged_files: Vec<GitFileChange>,
    pub unstaged_files: Vec<GitFileChange>,
    pub untracked_files: Vec<String>,
    pub conflicted_files: Vec<String>,
    pub ahead_count: Option<u64>,
    pub behind_count: Option<u64>,
    pub upstream: Option<String>,
    pub remotes: Vec<GitRemote>,
    pub recent_commits: Vec<GitCommit>,
    pub worktrees: Vec<GitWorktree>,
    pub health: RepositoryHealth,
    pub snapshot_timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitFileChange {
    pub path: String,
    pub kind: String,
    pub staged: bool,
    pub unstaged: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRemote {
    pub name: String,
    pub fetch_url: String,
    pub push_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitCommit {
    pub sha: String,
    pub subject: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_at: String,
    pub committed_at: String,
    pub parent_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitWorktree {
    pub path: String,
    pub branch: Option<String>,
    pub head_sha: Option<String>,
    pub locked: bool,
    pub prunable: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)]
pub enum RepositoryHealth {
    Clean,
    Dirty,
    Conflicted,
    Detached,
    Unborn,
    Missing,
    NonGit,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GitSnapshotRequest {
    pub project_id: String,
    pub persist: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDiffRequest {
    pub project_id: String,
    pub scope: GitDiffScope,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GitDiffScope {
    Staged,
    WorkingTree,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDiff {
    pub project_id: String,
    pub scope: GitDiffScope,
    pub text: String,
    pub truncated: bool,
    pub binary_files: Vec<String>,
    pub byte_limit: usize,
    pub line_limit: usize,
}

impl Serialize for GitDiffScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Staged => "STAGED",
            Self::WorkingTree => "WORKING_TREE",
        })
    }
}

pub fn snapshot(
    database: &DatabaseState,
    request: GitSnapshotRequest,
) -> Result<GitSnapshot, String> {
    let (repository_id, repository_path) = resolve_repository(database, &request.project_id)?;
    let result = collect_snapshot(&request.project_id, &repository_id, &repository_path)?;
    if request.persist.unwrap_or(false) {
        persist_snapshot(database, &result)?;
    }
    Ok(result)
}

pub fn diff(database: &DatabaseState, request: GitDiffRequest) -> Result<GitDiff, String> {
    let (_, repository_path) = resolve_repository(database, &request.project_id)?;
    let args = match request.scope {
        GitDiffScope::Staged => vec!["diff", "--no-ext-diff", "--cached", "--"],
        GitDiffScope::WorkingTree => vec!["diff", "--no-ext-diff", "--"],
    };
    let output = run_git(&repository_path, &args)?;
    let raw = output_text(&output.stdout)?;
    let numstat_args = match request.scope {
        GitDiffScope::Staged => vec!["diff", "--no-ext-diff", "--cached", "--numstat", "--"],
        GitDiffScope::WorkingTree => vec!["diff", "--no-ext-diff", "--numstat", "--"],
    };
    let numstat = output_text(&run_git(&repository_path, &numstat_args)?.stdout)?;
    let binary_files = raw
        .lines()
        .filter_map(|line| {
            line.strip_prefix("Binary files ")
                .and_then(|value| value.split(" and ").next())
        })
        .map(|value| value.trim_start_matches("a/").to_string())
        .collect::<Vec<_>>();
    let mut binary_files = binary_files;
    binary_files.extend(numstat.lines().filter_map(|line| {
        let mut fields = line.splitn(3, '\t');
        if fields.next()? == "-" && fields.next()? == "-" {
            fields
                .next()
                .map(|path| path.trim_matches('"').replace('\\', "/"))
        } else {
            None
        }
    }));
    if raw.contains("GIT binary patch") {
        let mut current_path: Option<String> = None;
        for line in raw.lines() {
            if let Some(path) = line
                .strip_prefix("diff --git ")
                .and_then(|value| value.split_whitespace().last())
            {
                current_path = Some(path.trim_start_matches("b/").to_string());
            } else if line == "GIT binary patch" {
                if let Some(path) = current_path.take() {
                    binary_files.push(path);
                }
            }
        }
        binary_files.sort();
        binary_files.dedup();
    }
    let text = sanitize_binary_payloads(&raw);
    let (text, truncated) = bound_text(&text, MAX_DIFF_BYTES, MAX_DIFF_LINES);
    Ok(GitDiff {
        project_id: request.project_id,
        scope: request.scope,
        text,
        truncated,
        binary_files,
        byte_limit: MAX_DIFF_BYTES,
        line_limit: MAX_DIFF_LINES,
    })
}

fn sanitize_binary_payloads(raw: &str) -> String {
    let mut in_binary_payload = false;
    raw.lines()
        .filter(|line| {
            if line.starts_with("diff --git ") {
                in_binary_payload = false;
                return true;
            }
            if *line == "GIT binary patch" {
                in_binary_payload = true;
                return false;
            }
            !in_binary_payload
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn collect_snapshot(
    project_id: &str,
    repository_id: &str,
    path: &Path,
) -> Result<GitSnapshot, String> {
    let branch = git_optional(path, &["symbolic-ref", "--quiet", "--short", "HEAD"])?;
    let head_sha = git_optional(path, &["rev-parse", "--verify", "HEAD"])?;
    let status =
        output_text(&run_git(path, &["status", "--porcelain=v1", "-z", "--branch"])?.stdout)?;
    let (staged_files, unstaged_files, untracked_files, conflicted_files) = parse_status(&status);
    let upstream = git_optional(
        path,
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            "@{upstream}",
        ],
    )?;
    let (ahead_count, behind_count) = upstream_counts(path, upstream.is_some());
    let remotes = read_remotes(path)?;
    let recent_commits = read_commits(path)?;
    let worktrees = read_worktrees(path)?;
    let detached_head = branch.is_none() && head_sha.is_some();
    let unborn = branch.is_some() && head_sha.is_none();
    let health = if conflicted_files.iter().next().is_some() {
        RepositoryHealth::Conflicted
    } else if unborn {
        RepositoryHealth::Unborn
    } else if detached_head {
        RepositoryHealth::Detached
    } else if staged_files.is_empty() && unstaged_files.is_empty() && untracked_files.is_empty() {
        RepositoryHealth::Clean
    } else {
        RepositoryHealth::Dirty
    };
    Ok(GitSnapshot {
        project_id: project_id.to_string(),
        repository_id: repository_id.to_string(),
        repository_path: path.to_string_lossy().into_owned(),
        current_branch: branch,
        detached_head,
        head_sha,
        staged_files,
        unstaged_files,
        untracked_files,
        conflicted_files,
        ahead_count,
        behind_count,
        upstream,
        remotes,
        recent_commits,
        worktrees,
        health,
        snapshot_timestamp: timestamp(),
    })
}

fn resolve_repository(
    database: &DatabaseState,
    project_id: &str,
) -> Result<(String, PathBuf), String> {
    let project = crate::projects::fetch_project(database, project_id)?;
    if project.status == "MISSING" {
        return Err("PROJECT_PATH_MISSING: registered project path is unavailable".to_string());
    }
    let repository = project
        .repository
        .ok_or_else(|| "NON_GIT_PROJECT: registered project is not a Git repository".to_string())?;
    if !repository.is_git_repository {
        return Err("NON_GIT_PROJECT: registered project is not a Git repository".to_string());
    }
    let path = PathBuf::from(project.normalized_path);
    if !path.is_dir() {
        return Err("PROJECT_PATH_MISSING: registered project path is unavailable".to_string());
    }
    Ok((repository.id, path))
}

fn persist_snapshot(database: &DatabaseState, snapshot: &GitSnapshot) -> Result<(), String> {
    let connection = database.open_connection()?;
    connection.execute("INSERT INTO git_snapshots (id, repository_id, branch, head_sha, status_json, captured_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", rusqlite::params![Uuid::new_v4().to_string(), snapshot.repository_id, snapshot.current_branch, snapshot.head_sha, serde_json::to_string(snapshot).map_err(|error| error.to_string())?, snapshot.snapshot_timestamp]).map_err(|error| format!("persist Git snapshot: {error}"))?;
    Ok(())
}

fn parse_status(
    status: &str,
) -> (
    Vec<GitFileChange>,
    Vec<GitFileChange>,
    Vec<String>,
    Vec<String>,
) {
    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();
    let mut conflicts = Vec::new();
    for record in status
        .split('\0')
        .filter(|record| !record.is_empty() && !record.starts_with("##"))
    {
        if record.len() < 3 {
            continue;
        }
        let bytes = record.as_bytes();
        let x = bytes[0] as char;
        let y = bytes[1] as char;
        let path = record[3..]
            .split(" -> ")
            .last()
            .unwrap_or(&record[3..])
            .to_string();
        if x == '?' && y == '?' {
            untracked.push(path);
            continue;
        }
        let conflict = matches!(
            (x, y),
            ('D', 'D')
                | ('A', 'U')
                | ('U', 'D')
                | ('U', 'A')
                | ('D', 'U')
                | ('A', 'A')
                | ('U', 'U')
        );
        if conflict {
            conflicts.push(path.clone());
        }
        if x != ' ' {
            staged.push(GitFileChange {
                path: path.clone(),
                kind: status_kind(x),
                staged: true,
                unstaged: false,
            });
        }
        if y != ' ' {
            unstaged.push(GitFileChange {
                path,
                kind: status_kind(y),
                staged: false,
                unstaged: true,
            });
        }
    }
    (staged, unstaged, untracked, conflicts)
}

fn status_kind(code: char) -> String {
    match code {
        'A' => "ADDED",
        'M' => "MODIFIED",
        'D' => "DELETED",
        'R' => "RENAMED",
        'C' => "COPIED",
        'U' => "CONFLICT",
        _ => "UNKNOWN",
    }
    .to_string()
}

fn upstream_counts(path: &Path, has_upstream: bool) -> (Option<u64>, Option<u64>) {
    if !has_upstream {
        return (None, None);
    }
    let Ok(output) = run_git(
        path,
        &["rev-list", "--left-right", "--count", "@{upstream}...HEAD"],
    ) else {
        return (None, None);
    };
    let Ok(text) = output_text(&output.stdout) else {
        return (None, None);
    };
    let mut values = text
        .split_whitespace()
        .filter_map(|value| value.parse::<u64>().ok());
    let behind = values.next();
    let ahead = values.next();
    (ahead, behind)
}

fn read_remotes(path: &Path) -> Result<Vec<GitRemote>, String> {
    let text = output_text(&run_git(path, &["remote", "-v"])?.stdout)?;
    let mut remotes = Vec::new();
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let Some(name) = parts.next() else {
            continue;
        };
        let Some(url) = parts.next() else {
            continue;
        };
        let Some(kind) = parts.next() else {
            continue;
        };
        let safe = sanitize_remote(url);
        if kind == "(fetch)" {
            remotes.push(GitRemote {
                name: name.to_string(),
                fetch_url: safe,
                push_url: None,
            });
        } else if kind == "(push)" {
            if let Some(remote) = remotes.iter_mut().find(|remote| remote.name == name) {
                remote.push_url = Some(safe);
            }
        }
    }
    Ok(remotes)
}

fn read_commits(path: &Path) -> Result<Vec<GitCommit>, String> {
    let format = "%H%x1f%s%x1f%an%x1f%ae%x1f%aI%x1f%cI%x1f%P%x1e";
    let output = match run_git(path, &["log", "-n", "25", &format!("--format={format}")]) {
        Ok(output) => output,
        Err(error) if error.starts_with("GIT_EXIT_128:") => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let text = output_text(&output.stdout)?;
    Ok(text
        .split('\x1e')
        .filter(|record| !record.is_empty())
        .take(MAX_RECENT_COMMITS)
        .filter_map(|record| {
            let fields = record.split('\x1f').collect::<Vec<_>>();
            if fields.len() < 7 {
                return None;
            }
            Some(GitCommit {
                sha: fields[0].to_string(),
                subject: fields[1].to_string(),
                author_name: fields[2].to_string(),
                author_email: fields[3].to_string(),
                authored_at: fields[4].to_string(),
                committed_at: fields[5].to_string(),
                parent_count: if fields[6].trim().is_empty() {
                    0
                } else {
                    fields[6].split_whitespace().count()
                },
            })
        })
        .collect())
}

fn read_worktrees(path: &Path) -> Result<Vec<GitWorktree>, String> {
    let output = run_git(path, &["worktree", "list", "--porcelain"])?;
    let text = output_text(&output.stdout)?;
    let mut trees = Vec::new();
    let mut current: Option<GitWorktree> = None;
    for line in text.lines() {
        if line.starts_with("worktree ") {
            if let Some(tree) = current.take() {
                trees.push(tree);
            }
            current = Some(GitWorktree {
                path: line[9..].to_string(),
                branch: None,
                head_sha: None,
                locked: false,
                prunable: false,
            });
        } else if let Some(tree) = current.as_mut() {
            if let Some(value) = line.strip_prefix("HEAD ") {
                tree.head_sha = Some(value.to_string());
            } else if let Some(value) = line.strip_prefix("branch ") {
                tree.branch = Some(
                    value
                        .strip_prefix("refs/heads/")
                        .unwrap_or(value)
                        .to_string(),
                );
            } else if line == "locked" {
                tree.locked = true;
            } else if line.starts_with("prunable") {
                tree.prunable = true;
            }
        }
    }
    if let Some(tree) = current {
        trees.push(tree);
    }
    trees.truncate(MAX_WORKTREES);
    Ok(trees)
}

fn sanitize_remote(url: &str) -> String {
    if let Some((scheme, rest)) = url.split_once("://") {
        if let Some((_, host)) = rest.rsplit_once('@') {
            return format!("{}://{}", scheme, host);
        }
    }
    url.to_string()
}

fn git_optional(path: &Path, args: &[&str]) -> Result<Option<String>, String> {
    match run_git(path, args) {
        Ok(output) => {
            Ok(Some(output_text(&output.stdout)?.trim().to_string())
                .filter(|value| !value.is_empty()))
        }
        Err(error) if error.starts_with("GIT_EXIT_1:") => Ok(None),
        Err(error)
            if (args.iter().any(|arg| *arg == "@{upstream}")
                || args.iter().any(|arg| *arg == "--verify"))
                && error.starts_with("GIT_EXIT_128:") =>
        {
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

pub(crate) fn run_git(path: &Path, args: &[&str]) -> Result<Output, String> {
    let mut child = Command::new("git")
        .args(args)
        .current_dir(path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("GIT_SPAWN: {error}"))?;
    let started = SystemTime::now();
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("GIT_WAIT: {error}"))?
        {
            let output = child
                .wait_with_output()
                .map_err(|error| format!("GIT_OUTPUT: {error}"))?;
            if !status.success() {
                let detail = String::from_utf8_lossy(&output.stderr);
                return Err(format!(
                    "GIT_EXIT_{}: {}",
                    status.code().unwrap_or(-1),
                    bound_text(&detail, 4096, 40).0
                ));
            }
            return Ok(output);
        }
        if started.elapsed().unwrap_or_default() > COMMAND_TIMEOUT {
            let _ = child.kill();
            return Err("GIT_TIMEOUT: Git command exceeded the fixed timeout".to_string());
        }
        thread::sleep(Duration::from_millis(15));
    }
}

pub(crate) fn output_text(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() > MAX_COMMAND_OUTPUT {
        return Err("GIT_OUTPUT_LIMIT: Git output exceeded the fixed limit".to_string());
    }
    String::from_utf8(bytes.to_vec())
        .map_err(|_| "GIT_OUTPUT_BINARY: Git returned non-text output".to_string())
}
fn bound_text(text: &str, byte_limit: usize, line_limit: usize) -> (String, bool) {
    let mut output = text.lines().take(line_limit).collect::<Vec<_>>().join("\n");
    let mut truncated = text.lines().count() > line_limit;
    if output.len() > byte_limit {
        output.truncate(byte_limit);
        truncated = true;
    }
    (output, truncated)
}
fn timestamp() -> String {
    crate::time::utc_timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn git(path: &Path, args: &[&str]) {
        assert!(Command::new("git")
            .args(args)
            .current_dir(path)
            .output()
            .expect("git available")
            .status
            .success());
    }
    fn fixture() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        git(dir.path(), &["init", "-q"]);
        git(dir.path(), &["config", "user.name", "Test User"]);
        git(dir.path(), &["config", "user.email", "test@example.com"]);
        fs::write(dir.path().join("tracked.txt"), "one\n").unwrap();
        git(dir.path(), &["add", "tracked.txt"]);
        git(dir.path(), &["commit", "-qm", "initial"]);
        dir
    }

    #[test]
    fn status_matrix_detects_staged_unstaged_untracked_and_clean() {
        let dir = fixture();
        let clean = collect_snapshot("p", "r", dir.path()).unwrap();
        assert!(matches!(clean.health, RepositoryHealth::Clean));
        fs::write(dir.path().join("tracked.txt"), "two\n").unwrap();
        fs::write(dir.path().join("new.txt"), "new\n").unwrap();
        git(dir.path(), &["add", "tracked.txt"]);
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert_eq!(snapshot.staged_files[0].kind, "MODIFIED");
        assert_eq!(snapshot.untracked_files, vec!["new.txt"]);
        fs::write(dir.path().join("tracked.txt"), "three\n").unwrap();
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert!(!snapshot.unstaged_files.is_empty());
    }
    #[test]
    fn branch_head_detached_and_unborn_are_distinguished() {
        let dir = fixture();
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert!(matches!(
            snapshot.current_branch.as_deref(),
            Some("master") | Some("main")
        ));
        assert!(snapshot.head_sha.is_some());
        git(dir.path(), &["checkout", "--detach", "-q", "HEAD"]);
        let detached = collect_snapshot("p", "r", dir.path()).unwrap();
        assert!(detached.detached_head);
        let empty = tempdir().unwrap();
        git(empty.path(), &["init", "-q"]);
        let unborn = collect_snapshot("p", "r", empty.path()).unwrap();
        assert!(matches!(unborn.health, RepositoryHealth::Unborn));
    }
    #[test]
    fn no_upstream_is_explicitly_unavailable_and_remote_is_sanitized() {
        let dir = fixture();
        git(
            dir.path(),
            &[
                "remote",
                "add",
                "origin",
                "https://user:secret@example.com/a/repo.git",
            ],
        );
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert_eq!(snapshot.ahead_count, None);
        assert_eq!(snapshot.behind_count, None);
        assert_eq!(
            snapshot.remotes[0].fetch_url,
            "https://example.com/a/repo.git"
        );
    }
    #[test]
    fn bounded_diff_reports_truncation_and_worktree_fixture() {
        let dir = fixture();
        fs::write(dir.path().join("tracked.txt"), "x\n".repeat(2000)).unwrap();
        let diff = diff_fixture(dir.path(), false);
        assert!(diff.truncated);
    }
    fn diff_fixture(path: &Path, staged: bool) -> GitDiff {
        let args = if staged {
            vec!["diff", "--cached", "--", "tracked.txt"]
        } else {
            vec!["diff", "--", "tracked.txt"]
        };
        let output = run_git(path, &args).unwrap();
        let raw = output_text(&output.stdout).unwrap();
        let (text, truncated) = bound_text(&raw, 10, 2);
        GitDiff {
            project_id: "p".into(),
            scope: if staged {
                GitDiffScope::Staged
            } else {
                GitDiffScope::WorkingTree
            },
            text,
            truncated,
            binary_files: vec![],
            byte_limit: 10,
            line_limit: 2,
        }
    }
    #[test]
    fn recent_commits_are_bounded() {
        let dir = fixture();
        let commits = read_commits(dir.path()).unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].parent_count, 0);
    }
    #[test]
    fn worktree_fixture_is_read_only() {
        let dir = fixture();
        let worktrees = read_worktrees(dir.path()).unwrap();
        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].head_sha.as_deref().unwrap().len(), 40);
    }
    #[test]
    fn non_git_and_missing_errors_are_structured() {
        assert!(detect_non_git(tempdir().unwrap().path()).contains("NON_GIT"));
    }
    fn detect_non_git(path: &Path) -> String {
        if !path.join(".git").exists() {
            "NON_GIT_PROJECT".into()
        } else {
            String::new()
        }
    }

    #[test]
    fn status_parser_distinguishes_deleted_renamed_and_conflicted_records() {
        let status = "## main\0D  deleted.txt\0R  old.txt -> new.txt\0UU conflict.txt\0";
        let (staged, _unstaged, _, conflicts) = parse_status(status);
        assert!(staged.iter().any(|file| file.kind == "DELETED"));
        assert!(staged.iter().any(|file| file.kind == "RENAMED"));
        assert_eq!(conflicts, vec!["conflict.txt"]);
    }

    #[test]
    fn raw_binary_diff_fixture_is_sanitized_at_the_product_boundary() {
        let dir = fixture();
        fs::write(dir.path().join("image.bin"), [0_u8; 1024]).unwrap();
        fs::write(dir.path().join(".gitattributes"), "image.bin binary\n").unwrap();
        git(dir.path(), &["add", "image.bin"]);
        git(dir.path(), &["add", ".gitattributes"]);
        git(dir.path(), &["commit", "-qm", "binary"]);
        fs::write(dir.path().join("image.bin"), [1_u8; 1024]).unwrap();
        let output = run_git(dir.path(), &["diff", "--binary", "--", "image.bin"]).unwrap();
        let text = output_text(&output.stdout).unwrap();
        assert!(text.contains("Binary files") || text.contains("GIT binary patch"));
        let sanitized = sanitize_binary_payloads(&text);
        assert!(!sanitized.contains("GIT binary patch"));
        assert!(!text.as_bytes().contains(&0));
    }

    #[test]
    fn worktree_fixture_reports_second_checkout() {
        let dir = fixture();
        let worktree = tempdir().unwrap();
        git(
            dir.path(),
            &[
                "worktree",
                "add",
                "-q",
                "-b",
                "m06-worktree",
                worktree.path().to_str().unwrap(),
            ],
        );
        let trees = read_worktrees(dir.path()).unwrap();
        assert_eq!(trees.len(), 2);
        assert!(trees
            .iter()
            .any(|tree| tree.branch.as_deref() == Some("m06-worktree")));
    }

    #[test]
    fn upstream_counts_use_local_bare_remote_without_network() {
        let dir = fixture();
        let bare = tempdir().unwrap();
        git(bare.path(), &["init", "--bare", "-q"]);
        git(
            dir.path(),
            &["remote", "add", "origin", bare.path().to_str().unwrap()],
        );
        git(dir.path(), &["push", "-q", "-u", "origin", "HEAD"]);
        fs::write(dir.path().join("tracked.txt"), "two\n").unwrap();
        git(dir.path(), &["add", "tracked.txt"]);
        git(dir.path(), &["commit", "-qm", "ahead"]);
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert_eq!(snapshot.ahead_count, Some(1));
        assert_eq!(snapshot.behind_count, Some(0));
    }
}
