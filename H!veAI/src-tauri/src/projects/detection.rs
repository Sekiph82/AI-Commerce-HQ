use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteMetadata {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitMetadata {
    pub is_git_repository: bool,
    pub repository_root: Option<String>,
    pub current_branch: Option<String>,
    pub head_sha: Option<String>,
    pub remotes: Vec<RemoteMetadata>,
    pub preferred_remote_url: Option<String>,
    pub default_branch: Option<String>,
    pub github_owner: Option<String>,
    pub github_repo: Option<String>,
}

pub fn detect_git_metadata(selected_path: &Path) -> GitMetadata {
    let Some(repository_root) = find_repository_root(selected_path) else {
        return GitMetadata::default();
    };
    let Some(git_directory) = resolve_git_directory(&repository_root) else {
        return GitMetadata {
            is_git_repository: true,
            repository_root: Some(repository_root.to_string_lossy().into_owned()),
            ..GitMetadata::default()
        };
    };

    let (current_branch, head_sha) = read_head(&git_directory);
    let remotes = read_remotes(&git_directory);
    let preferred_remote_url = remotes
        .iter()
        .find(|remote| remote.name == "origin")
        .or_else(|| remotes.first())
        .map(|remote| remote.url.clone());
    let default_branch = read_default_branch(&git_directory).or_else(|| current_branch.clone());
    let (github_owner, github_repo) = preferred_remote_url
        .as_deref()
        .and_then(parse_github_identity)
        .unzip();

    GitMetadata {
        is_git_repository: true,
        repository_root: Some(repository_root.to_string_lossy().into_owned()),
        current_branch,
        head_sha,
        remotes,
        preferred_remote_url,
        default_branch,
        github_owner,
        github_repo,
    }
}

fn find_repository_root(selected_path: &Path) -> Option<PathBuf> {
    let mut candidate = Some(selected_path);
    while let Some(path) = candidate {
        if path.join(".git").exists() {
            return Some(path.to_path_buf());
        }
        candidate = path.parent();
    }
    None
}

fn resolve_git_directory(repository_root: &Path) -> Option<PathBuf> {
    let marker = repository_root.join(".git");
    if marker.is_dir() {
        return Some(marker);
    }
    let marker_content = std::fs::read_to_string(marker).ok()?;
    let target = marker_content
        .lines()
        .find_map(|line| line.strip_prefix("gitdir:"))?
        .trim();
    let target_path = PathBuf::from(target);
    if target_path.is_absolute() {
        Some(target_path)
    } else {
        Some(repository_root.join(target_path))
    }
}

fn read_head(git_directory: &Path) -> (Option<String>, Option<String>) {
    let Ok(head) = std::fs::read_to_string(git_directory.join("HEAD")) else {
        return (None, None);
    };
    let head = head.trim();
    if let Some(reference) = head.strip_prefix("ref: ") {
        let branch = reference.strip_prefix("refs/heads/").map(ToOwned::to_owned);
        let sha = read_ref(git_directory, reference);
        (branch, sha)
    } else if is_sha(head) {
        (None, Some(head.to_string()))
    } else {
        (None, None)
    }
}

fn read_default_branch(git_directory: &Path) -> Option<String> {
    let head = read_ref(git_directory, "refs/remotes/origin/HEAD")?;
    head.strip_prefix("ref: refs/remotes/origin/")
        .map(ToOwned::to_owned)
}

fn read_ref(git_directory: &Path, reference: &str) -> Option<String> {
    let reference_path =
        git_directory.join(reference.replace('/', &std::path::MAIN_SEPARATOR.to_string()));
    if let Ok(value) = std::fs::read_to_string(reference_path) {
        return Some(value.trim().to_string());
    }
    let packed = std::fs::read_to_string(git_directory.join("packed-refs")).ok()?;
    packed.lines().find_map(|line| {
        let mut parts = line.split_whitespace();
        let sha = parts.next()?;
        let found_reference = parts.next()?;
        (found_reference == reference).then(|| sha.to_string())
    })
}

fn read_remotes(git_directory: &Path) -> Vec<RemoteMetadata> {
    let Ok(config) = std::fs::read_to_string(git_directory.join("config")) else {
        return Vec::new();
    };
    let mut remotes = Vec::new();
    let mut section: Option<String> = None;
    for line in config.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = trimmed
                .strip_prefix("[remote \"")
                .and_then(|value| value.strip_suffix("\"]"))
                .map(ToOwned::to_owned);
            continue;
        }
        if let (Some(name), Some(url)) = (section.as_ref(), trimmed.strip_prefix("url = ")) {
            remotes.push(RemoteMetadata {
                name: name.clone(),
                url: sanitize_remote_url(url.trim()),
            });
            section = None;
        }
    }
    remotes
}

pub fn sanitize_remote_url(raw: &str) -> String {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("https://") {
        let rest = &trimmed[8..];
        let host_and_path = rest.split_once('@').map(|(_, value)| value).unwrap_or(rest);
        return format!("https://{host_and_path}");
    }
    if lower.starts_with("http://") {
        let rest = &trimmed[7..];
        let host_and_path = rest.split_once('@').map(|(_, value)| value).unwrap_or(rest);
        return format!("http://{host_and_path}");
    }
    trimmed.to_string()
}

fn parse_github_identity(url: &str) -> Option<(String, String)> {
    let lower = url.to_ascii_lowercase();
    let path = if let Some(value) = lower.strip_prefix("https://github.com/") {
        value
    } else if let Some(value) = lower.strip_prefix("http://github.com/") {
        value
    } else if let Some(value) = lower.strip_prefix("ssh://git@github.com/") {
        value
    } else if let Some(value) = lower.strip_prefix("git@github.com:") {
        value
    } else {
        return None;
    };
    let mut parts = path
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .split('/');
    let owner = parts.next()?.to_string();
    let repo = parts.next()?.to_string();
    if owner.is_empty() || repo.is_empty() {
        None
    } else {
        Some((owner, repo))
    }
}

fn is_sha(value: &str) -> bool {
    (value.len() == 40 || value.len() == 64)
        && value.chars().all(|character| character.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn detects_non_git_and_git_metadata_without_mutation() {
        let directory = tempdir().unwrap();
        assert!(!detect_git_metadata(directory.path()).is_git_repository);
        let git = directory.path().join(".git");
        std::fs::create_dir_all(git.join("refs/heads")).unwrap();
        std::fs::write(git.join("HEAD"), "ref: refs/heads/main\n").unwrap();
        std::fs::write(
            git.join("refs/heads/main"),
            "0123456789012345678901234567890123456789\n",
        )
        .unwrap();
        std::fs::write(
            git.join("config"),
            "[remote \"origin\"]\n\turl = https://user:secret@github.com/Owner/Repo.git\n",
        )
        .unwrap();
        let metadata = detect_git_metadata(directory.path());
        assert!(metadata.is_git_repository);
        assert_eq!(metadata.current_branch.as_deref(), Some("main"));
        assert_eq!(
            metadata.head_sha.as_deref(),
            Some("0123456789012345678901234567890123456789")
        );
        assert_eq!(
            metadata.preferred_remote_url.as_deref(),
            Some("https://github.com/Owner/Repo.git")
        );
        assert_eq!(metadata.github_owner.as_deref(), Some("owner"));
        assert_eq!(metadata.github_repo.as_deref(), Some("repo"));
    }

    #[test]
    fn sanitizes_supported_remote_credentials() {
        assert_eq!(
            sanitize_remote_url("https://user:token@github.com/org/repo.git"),
            "https://github.com/org/repo.git"
        );
        assert_eq!(
            sanitize_remote_url("git@github.com:org/repo.git"),
            "git@github.com:org/repo.git"
        );
    }
}
