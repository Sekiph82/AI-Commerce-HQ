use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub const AKILTA_URL: &str = "https://www.akilta.com/";

#[derive(Debug)]
pub enum BrowserError {
    ChromeUnavailable,
    LaunchFailed(String),
}

impl std::fmt::Display for BrowserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChromeUnavailable => write!(
                formatter,
                "AKILTA_CHROME_UNAVAILABLE: Google Chrome is not installed"
            ),
            Self::LaunchFailed(message) => {
                write!(formatter, "AKILTA_CHROME_LAUNCH_FAILED: {message}")
            }
        }
    }
}

#[cfg(windows)]
fn chrome_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    for variable in ["ProgramFiles", "ProgramFiles(x86)", "LocalAppData"] {
        if let Some(root) = std::env::var_os(variable) {
            candidates.push(PathBuf::from(root).join("Google/Chrome/Application/chrome.exe"));
        }
    }
    candidates
}

#[cfg(windows)]
pub fn open_akilta() -> Result<(), String> {
    open_https_url(AKILTA_URL)
}

#[cfg(windows)]
pub fn open_external_url(url: &str) -> Result<(), String> {
    validate_https_url(url)?;
    open_https_url(url)
}

#[cfg(windows)]
fn validate_https_url(url: &str) -> Result<(), String> {
    if !url.starts_with("https://")
        || url
            .chars()
            .any(|character| character.is_whitespace() || character == '"')
    {
        return Err("EXTERNAL_URL_BLOCKED: only bounded HTTPS URLs may be opened".into());
    }
    Ok(())
}

#[cfg(windows)]
fn open_https_url(url: &str) -> Result<(), String> {
    let chrome = chrome_candidates()
        .into_iter()
        .find(|candidate| candidate.is_file())
        .ok_or(BrowserError::ChromeUnavailable)
        .map_err(|error| error.to_string())?;
    Command::new(chrome)
        .arg("--new-window")
        .arg(url)
        .creation_flags(0x08000000)
        .spawn()
        .map(|_| ())
        .map_err(|error| BrowserError::LaunchFailed(error.to_string()).to_string())
}

#[cfg(not(windows))]
pub fn open_akilta() -> Result<(), String> {
    open_external_url(AKILTA_URL)
}

#[cfg(not(windows))]
pub fn open_external_url(url: &str) -> Result<(), String> {
    if !url.starts_with("https://")
        || url
            .chars()
            .any(|character| character.is_whitespace() || character == '"')
    {
        return Err("EXTERNAL_URL_BLOCKED: only bounded HTTPS URLs may be opened".into());
    }
    for executable in ["google-chrome", "google-chrome-stable"] {
        let result = Command::new(executable)
            .arg("--new-window")
            .arg(url)
            .spawn();
        if result.is_ok() {
            return Ok(());
        }
    }
    Err(BrowserError::ChromeUnavailable.to_string())
}

#[cfg(test)]
mod tests {
    use super::{open_external_url, AKILTA_URL};

    #[test]
    fn akilta_url_is_fixed() {
        assert_eq!(AKILTA_URL, "https://www.akilta.com/");
    }

    #[test]
    fn external_url_policy_rejects_non_https_input() {
        assert!(open_external_url("http://example.com").is_err());
        assert!(open_external_url("javascript:alert(1)").is_err());
    }
}
