use duct::cmd;
use std::io;

#[cfg(target_os = "linux")]
pub fn go_server_execname() -> String {
    String::from("./go-server")
}

#[cfg(target_os = "windows")]
pub fn go_server_execname() -> String {
    String::from("go-server.exe")
}

#[cfg(target_os = "linux")]
pub fn pidof(process_name: &str) -> Result<u16, std::error::Error> {
    cmd!("pidof", process_name).read().unwrap().parse::<u16>()
}

#[cfg(target_os = "windows")]
pub fn pidof(process_name: &str) -> Result<u16, std::error::Error> {
    let script = format!(
        "Get-Process | where {{$_.ProcessName -eq '{}'}} | select Id | ForEach-Object {{$_.Id}} | Out-String -stream",
        process_name
    );
    cmd!("powershell", "-c", script)
        .read()
        .unwrap()
        .parse::<u16>()
}

#[cfg(target_os = "linux")]
pub fn kill(pid: u16) -> io::Result<()> {
    cmd!("kill", pid.to_string()).run()?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn kill(pid: u16) -> io::Result<()> {
    cmd!(
        "powershell",
        "-c",
        format!("Stop-Process -ID {} -Force", pid.to_string())
    )
    .run()?;
    Ok(())
}
