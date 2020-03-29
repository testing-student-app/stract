#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::env;
use std::process::Command;
use stract_admin::{shell, ServerReply};

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;

            match serde_json::from_str(arg) {
                Err(_) => {}
                Ok(command) => match command {
                    LoadServer {
                        // port,
                        callback,
                        error,
                    } => tauri::execute_promise(
                        _webview,
                        move || {
                            let target_exe = env::current_exe()?;
                            let target_dir = target_exe.parent().unwrap();

                            let start_server = |port: u16| {
                                Command::new(shell::go_server_execname())
                                    .arg("-addr")
                                    .arg(format!(":{}", port))
                                    .current_dir(target_dir)
                                    .spawn()
                                    .expect("error in running go server")
                            };

                            start_server(8081);

                            if let Err(_) = shell::pidof("go-server") {
                                // return Err(port.into());
                            };

                            let reply = ServerReply {
                                status: String::from("started"),
                            };

                            Ok(serde_json::to_string(&reply).unwrap())
                        },
                        callback,
                        error,
                    ),
                    DestroyServer => {
                        let pid = shell::pidof("go-server").unwrap();
                        shell::kill(pid).unwrap();
                    }
                },
            }
        })
        .build()
        .run();
}
