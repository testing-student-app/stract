use std::ffi::OsStr;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
pub fn command_output<S: AsRef<OsStr>>(
  command: S,
  current_dir: &Path,
  args: Vec<&str>,
) -> std::io::Result<Output> {
  Command::new(command)
    .args(args)
    .current_dir(current_dir)
    .stdout(Stdio::piped())
    .creation_flags(CREATE_NO_WINDOW)
    .output()
}

#[cfg(not(target_os = "windows"))]
pub fn command_output<S: AsRef<OsStr>>(
  command: S,
  current_dir: &Path,
  args: Vec<&str>,
) -> std::io::Result<Output> {
  Command::new(command)
    .args(args)
    .current_dir(current_dir)
    .stdout(Stdio::piped())
    .output()
}

#[cfg(target_os = "windows")]
pub fn spawn_command<S: AsRef<OsStr>>(
  command: S,
  current_dir: &Path,
  args: Vec<&str>,
) -> std::io::Result<Child> {
  Command::new(command)
    .args(args)
    .current_dir(current_dir)
    .stdout(Stdio::piped())
    .creation_flags(CREATE_NO_WINDOW)
    .spawn()
}

#[cfg(not(target_os = "windows"))]
pub fn spawn_command<S: AsRef<OsStr>>(
  command: S,
  current_dir: &Path,
  args: Vec<&str>,
) -> std::io::Result<Child> {
  Command::new(command)
    .args(args)
    .current_dir(current_dir)
    .stdout(Stdio::piped())
    .spawn()
}
