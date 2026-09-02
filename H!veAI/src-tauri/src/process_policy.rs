use std::ffi::OsStr;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
pub const CREATE_NO_WINDOW: u32 = 0x08000000;

pub(crate) fn background_command(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

#[cfg(test)]
mod tests {
    #[cfg(windows)]
    #[test]
    fn background_process_policy_uses_create_no_window() {
        assert_eq!(super::CREATE_NO_WINDOW, 0x08000000);
    }
}
