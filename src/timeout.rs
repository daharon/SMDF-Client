//! Constants and helpers for `timeout` functionality which
//! varies depending on OS and toolchain.

/// The `timeout` command.
#[cfg(target_os = "macos")]
pub static CMD: &str = "/usr/local/bin/gtimeout";  // brew install coreutils
#[cfg(target_os = "linux")]
pub static CMD: &str = "/usr/bin/timeout";

/// The exit code `timeout` returns when the command times out.
#[cfg(any(target_env = "gnu", target_env = ""))]
pub static EXIT_CODE: i32 = 124;
#[cfg(target_env = "musl")]  // For Alpine Linux.
pub static EXIT_CODE: i32 = 143;

/// Generate the array of arguments for the `timeout` command.
pub fn opts(timeout_seconds: usize) -> Vec<String> {
    #[cfg(any(target_env = "gnu", target_env = ""))]
    return vec!["--signal".to_string(), "TERM".to_string(), format!("{}s", timeout_seconds)];
    #[cfg(target_env = "musl")]  // For Alpine Linux.
    return vec!["-s".to_string(), "TERM".to_string(), "-t".to_string(), format!("{}", timeout_seconds)];
}
