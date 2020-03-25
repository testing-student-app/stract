#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::env;
use std::process::Command;
use stract_admin::{go_server_execname, kill, pidof, ServerReply};

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;

            match serde_json::from_str(arg) {
                Err(_) => {}
                Ok(command) => match command {
                    LoadServer { callback, error } => tauri::execute_promise(
                        _webview,
                        move || {
                            let target_exe = env::current_exe()?;
                            let target_dir = target_exe.parent().unwrap();
                            let port: u16 = 8081;

                            let output = Command::new(go_server_execname())
                                .arg("-addr")
                                .arg(format!(":{}", port))
                                .current_dir(target_dir)
                                .output()?;

                            if !output.status.success() {
                                return Err("failed".into());
                            };

                            let reply = ServerReply {
                                status: String::from("started"),
                                port: Some(port),
                            };

                            Ok(serde_json::to_string(&reply).unwrap())
                        },
                        callback,
                        error,
                    ),
                    DestroyServer => {
                        let pid = pidof("go-server");
                        kill(pid).unwrap();
                    }
                },
            }
        })
        .build()
        .run();
}
