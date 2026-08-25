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
    let chrome = chrome_candidates()
        .into_iter()
        .find(|candidate| candidate.is_file())
        .ok_or(BrowserError::ChromeUnavailable)
        .map_err(|error| error.to_string())?;
    Command::new(chrome)
        .arg("--new-window")
        .arg(AKILTA_URL)
        .creation_flags(0x08000000)
        .spawn()
        .map(|_| ())
        .map_err(|error| BrowserError::LaunchFailed(error.to_string()).to_string())
}

#[cfg(not(windows))]
pub fn open_akilta() -> Result<(), String> {
    for executable in ["google-chrome", "google-chrome-stable"] {
        let result = Command::new(executable)
            .arg("--new-window")
            .arg(AKILTA_URL)
            .spawn();
        if result.is_ok() {
            return Ok(());
        }
    }
    Err(BrowserError::ChromeUnavailable.to_string())
}

#[cfg(test)]
mod tests {
    use super::AKILTA_URL;

    #[test]
    fn akilta_url_is_fixed() {
        assert_eq!(AKILTA_URL, "https://www.akilta.com/");
    }
}
