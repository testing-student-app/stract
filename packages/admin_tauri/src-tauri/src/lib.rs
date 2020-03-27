use duct::cmd;
use serde::Serialize;
use std::io;

#[derive(Serialize)]
pub struct ServerReply {
    pub status: String,
    pub port: Option<u16>,
}

#[cfg(unix)]
pub fn go_server_execname() -> String {
    String::from("./go-server")
}

#[cfg(windows)]
pub fn go_server_execname() -> String {
    String::from("go-server.exe")
}

#[cfg(unix)]
pub fn pidof(process_name: &str) -> u16 {
    cmd!("pidof", process_name)
        .read()
        .unwrap()
        .parse::<u16>()
        .unwrap()
}

#[cfg(windows)]
pub fn pidof(process_name: &str) -> u16 {
    let script = format!(
        "Get-Process | where {{$_.ProcessName -eq '{}'}} | select Id | ForEach-Object {{$_.Id}} | Out-String -stream",
        process_name
    );
    cmd!("powershell", "-c", script)
        .read()
        .unwrap()
        .parse::<u16>()
        .unwrap()
}

#[cfg(unix)]
pub fn kill(pid: u16) -> io::Result<()> {
    cmd!("kill", pid.to_string()).run()?;
    Ok(())
}

#[cfg(windows)]
pub fn kill(pid: u16) {}
