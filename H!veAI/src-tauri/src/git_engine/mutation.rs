use super::{output_text, run_git};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationStatus {
    pub enabled: bool,
    pub reason: String,
}

pub fn mutation_status() -> MutationStatus {
    MutationStatus { enabled: false, reason: "Git writes require a future explicit permission decision; M06 UI exposes no mutation controls.".to_string() }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct MutationGate {
    pub(crate) approved: bool,
}

#[allow(dead_code)]
pub(crate) fn create_branch(path: &Path, branch: &str, gate: MutationGate) -> Result<(), String> {
    authorize(gate)?;
    validate_branch(branch)?;
    run_git(path, &["branch", branch]).map(|_| ())
}

#[allow(dead_code)]
pub(crate) fn stage_paths(path: &Path, paths: &[String], gate: MutationGate) -> Result<(), String> {
    authorize(gate)?;
    if paths.is_empty() {
        return Err(
            "MUTATION_INVALID: at least one explicit relative path is required".to_string(),
        );
    }
    let validated = paths
        .iter()
        .map(|value| validate_relative_path(value))
        .collect::<Result<Vec<_>, _>>()?;
    let mut args = vec!["add", "--"];
    let values = validated
        .iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    args.extend(values.iter().map(String::as_str));
    run_git(path, &args).map(|_| ())
}

#[allow(dead_code)]
pub(crate) fn commit(path: &Path, message: &str, gate: MutationGate) -> Result<String, String> {
    authorize(gate)?;
    if message.trim().is_empty() {
        return Err("MUTATION_INVALID: commit message is required".to_string());
    }
    run_git(path, &["commit", "-m", message])?;
    output_text(&run_git(path, &["rev-parse", "--verify", "HEAD"])?.stdout)
        .map(|value| value.trim().to_string())
}

#[allow(dead_code)]
pub(crate) fn push(
    path: &Path,
    remote: &str,
    branch: &str,
    gate: MutationGate,
) -> Result<(), String> {
    authorize(gate)?;
    if remote.is_empty()
        || remote.chars().any(|character| {
            character.is_whitespace() || matches!(character, '/' | '\\' | ';' | '&')
        })
    {
        return Err("MUTATION_INVALID: remote name is not allowlisted".to_string());
    }
    validate_branch(branch)?;
    run_git(path, &["push", remote, branch]).map(|_| ())
}

fn authorize(gate: MutationGate) -> Result<(), String> {
    if gate.approved {
        Ok(())
    } else {
        Err("MUTATION_DENIED: Git write permission is disabled by default".to_string())
    }
}

fn validate_branch(branch: &str) -> Result<(), String> {
    if branch.is_empty()
        || branch.starts_with('-')
        || branch.contains("..")
        || branch.contains("@{")
        || branch.ends_with('.')
        || branch.ends_with('/')
        || branch.contains(' ')
        || branch.chars().any(|character| {
            character.is_control() || matches!(character, '~' | '^' | ':' | '?' | '*' | '[' | '\\')
        })
    {
        return Err("MUTATION_INVALID: unsafe branch name".to_string());
    }
    Ok(())
}

fn validate_relative_path(value: &str) -> Result<PathBuf, String> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(
            "MUTATION_INVALID: paths must be explicit repository-relative paths".to_string(),
        );
    }
    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::tempdir;

    fn git(path: &Path, args: &[&str]) {
        assert!(Command::new("git")
            .args(args)
            .current_dir(path)
            .output()
            .unwrap()
            .status
            .success());
    }
    fn repo() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        git(dir.path(), &["init", "-q"]);
        git(dir.path(), &["config", "user.name", "Test User"]);
        git(dir.path(), &["config", "user.email", "test@example.com"]);
        fs::write(dir.path().join("tracked.txt"), "one\n").unwrap();
        stage_paths(
            dir.path(),
            &["tracked.txt".into()],
            MutationGate { approved: true },
        )
        .unwrap();
        commit(dir.path(), "initial", MutationGate { approved: true }).unwrap();
        dir
    }

    #[test]
    fn denied_writes_are_default() {
        let dir = repo();
        let error =
            create_branch(dir.path(), "safe-branch", MutationGate { approved: false }).unwrap_err();
        assert!(error.starts_with("MUTATION_DENIED"));
    }
    #[test]
    fn branch_stage_and_commit_are_isolated_to_temp_repo() {
        let dir = repo();
        create_branch(dir.path(), "feature/m06", MutationGate { approved: true }).unwrap();
        git(dir.path(), &["checkout", "-q", "feature/m06"]);
        fs::write(dir.path().join("tracked.txt"), "two\n").unwrap();
        stage_paths(
            dir.path(),
            &["tracked.txt".into()],
            MutationGate { approved: true },
        )
        .unwrap();
        let sha = commit(dir.path(), "update", MutationGate { approved: true }).unwrap();
        assert_eq!(sha.len(), 40);
    }
    #[test]
    fn explicit_paths_and_dangerous_names_are_rejected() {
        let dir = repo();
        assert!(stage_paths(
            dir.path(),
            &["../outside".into()],
            MutationGate { approved: true }
        )
        .is_err());
        assert!(create_branch(dir.path(), "bad name", MutationGate { approved: true }).is_err());
        assert!(push(
            dir.path(),
            "origin;rm",
            "main",
            MutationGate { approved: true }
        )
        .is_err());
    }

    #[test]
    fn push_uses_existing_local_bare_remote_only() {
        let dir = repo();
        let bare = tempdir().unwrap();
        git(bare.path(), &["init", "--bare", "-q"]);
        git(
            dir.path(),
            &["remote", "add", "origin", bare.path().to_str().unwrap()],
        );
        let branch = String::from_utf8(
            Command::new("git")
                .args(["branch", "--show-current"])
                .current_dir(dir.path())
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();
        push(
            dir.path(),
            "origin",
            &branch,
            MutationGate { approved: true },
        )
        .unwrap();
    }
}
